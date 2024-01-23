use crate::types::FirehoseBlock;
// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{client::Client, env::Env, pb::Block, Error, Result};
use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use futures::stream;
use futures::StreamExt;
use prost::Message;

use std::path::{Path, PathBuf};
use std::{fs, time::Duration};

/// polling service
pub struct Polling {
    last_processed_block_path: Box<PathBuf>,
    batch: usize,
    block_time: u64,
    client: Client,
    confirms: u64,
    end: Option<u64>,
    forever: bool,
    latest: u64,
    ptr: u64,
    quiet: bool,
}

impl Polling {
    /// new polling service
    pub async fn new(
        data_directory: String,
        end: Option<u64>,
        env: Env,
        forever: bool,
        ptr: Option<String>,
        quiet: bool,
    ) -> Result<Self> {
        let client = Client::new(
            env.endpoints.clone(),
            Duration::from_millis(env.timeout),
            env.retry,
        )?;
        let batch = env.batch_blocks as usize;

        fs::create_dir_all(&data_directory).context(
            format_args!("unable to create data directory {}", &data_directory).to_string(),
        )?;

        let last_processed_block_path =
            Path::new(&data_directory).join("latest_block_processed.txt");

        let mut poller = Self {
            last_processed_block_path: Box::new(last_processed_block_path),
            batch,
            block_time: env.block_time,
            confirms: env.confirms,
            client,
            end,
            forever,
            latest: 0,
            ptr: 0,
            quiet: quiet,
        };

        poller.initialize_start_ptr(ptr).await?;

        Ok(poller)
    }

    async fn initialize_start_ptr(&mut self, start_block_flag: Option<String>) -> Result<()> {
        self.ptr = match self.last_processed_block_path.exists() {
            true => self.start_ptr_from_state().await?,
            false => match start_block_flag {
                Some(value) if value == "live" => self.start_ptr_from_last_irreversible().await?,
                Some(start) => self.start_ptr_from_flag_value(&start).await?,
                _ => {
                    log::info!(
                            "no previous latest block processed file {:?} exists, starting from block 0",
                            self.last_processed_block_path
                        );

                    0
                }
            },
        };

        Ok(())
    }

    async fn start_ptr_from_state(&self) -> Result<u64> {
        let content: String = tokio::fs::read_to_string(self.last_processed_block_path.as_ref())
            .await
            .context(
                format_args!(
                    "unable to read content of last block processsed file {:?}",
                    self.last_processed_block_path,
                )
                .to_string(),
            )?;

        content.parse::<u64>()
            .context(format_args!("content {} is not a valid u64 string value", &content).to_string(),
        ).map(|value|  {
            log::info!(
                "start block retrieved from last processed block state file, starting from block {}",
                value
            );

            value
        }).map_err(Into::into)
    }

    async fn start_ptr_from_last_irreversible(&self) -> Result<u64> {
        log::info!("user requested 'live' block, retrieving it from endpoint");

        self.latest_irreversible_block_num()
            .await
            .and_then(|live_block| {
                log::info!(
                    "start block explicitely provided, starting from live block {}",
                    live_block
                );

                Ok(live_block)
            })
    }

    async fn start_ptr_from_flag_value(&self, value: &String) -> Result<u64> {
        value
            .parse::<u64>()
            .and_then(|value| {
                log::info!(
                    "start block explicitely provided, starting from block {}",
                    value
                );

                Ok(value)
            })
            .context(format_args!("start {} is not a valid u64 string value", value).to_string())
            .map_err(Into::into)
    }

    /// Firehose init log to stdout
    ///
    /// FIRE INIT <VERSION> <BLOCK_TYPE_URL>
    fn firehose_init(&self) {
        println!("FIRE INIT 1.0 sf.arweave.type.v1");
    }

    /// Firehose log to stdout
    ///
    /// FIRE BLOCK <BLOCK_NUM> <BLOCK_HASH> <PARENT_NUM> <PARENT_HASH> <LIB> <TIMESTAMP> <ENCODED>
    fn firehose_log(&self, b: FirehoseBlock) -> Result<()> {
        let block_num = b.height;
        let block_hash = base64_url::decode(&b.indep_hash)
            .with_context(|| format!("invalid base64url indep_hash on block {}", block_num))?;
        let parent_hash = base64_url::decode(&b.previous_block)
            .with_context(|| format!("invalid base64url previous_block on block {}", block_num))?;
        let timestamp = b.timestamp;

        let parent_num = if b.previous_block.is_empty() {
            0
        } else {
            block_num - 1
        };

        let lib = if block_num > self.confirms {
            block_num - self.confirms
        } else {
            0
        };

        let encoded: Block = b.try_into()?;

        if self.quiet {
            println!(
                "FIRE BLOCK {} {} {} {} {} {}",
                block_num,
                hex::encode(block_hash),
                parent_num,
                hex::encode(parent_hash),
                lib,
                timestamp
            );
        } else {
            println!(
                "FIRE BLOCK {} {} {} {} {} {} {}",
                block_num,
                hex::encode(block_hash),
                parent_num,
                hex::encode(parent_hash),
                lib,
                timestamp,
                general_purpose::STANDARD.encode(encoded.encode_to_vec())
            );
        }

        Ok(())
    }

    /// poll blocks and write to stdout
    async fn poll(&mut self, blocks: Vec<u64>) -> Result<()> {
        if blocks.is_empty() {
            log::info!("nothing to poll, blocks are empty");
            return Ok(());
        }

        log::info!(
            "polling from {} to {}",
            blocks.first().expect("non-empty"),
            blocks.last().expect("non-empty")
        );

        let mut tasks = stream::iter(
            blocks
                .into_iter()
                .map(|block| self.client.get_firehose_block_by_height(block)),
        )
        .buffered(self.batch);

        while let Some(item) = tasks.next().await {
            let block = item?;
            let height = block.height;

            self.firehose_log(block)?;
            // # Safty
            //
            // only update ptr after firehose_log has been emitted
            self.ptr = height + 1;

            self.write_ptr().await?;

            if let Some(end) = self.end {
                if height == end {
                    return Err(Error::StopBlockReached);
                }
            }
        }

        Ok(())
    }

    async fn write_ptr(&self) -> Result<()> {
        let ptr_string = self.ptr.to_string();

        tokio::fs::write(self.last_processed_block_path.as_ref(), &ptr_string)
            .await
            .context(
                format_args!(
                    "unable to write last processed block ptr to {:?}",
                    &self.last_processed_block_path,
                )
                .to_string(),
            )?;

        Ok(())
    }

    async fn latest_irreversible_block_num(&self) -> Result<u64> {
        let head_block = self.client.get_current_block().await?.height;
        if head_block < self.confirms {
            return Ok(head_block);
        }

        Ok(head_block - self.confirms)
    }

    /// poll to head
    async fn track_head(&mut self) -> Result<()> {
        log::info!("fetching last irreversible block");
        self.latest = self.latest_irreversible_block_num().await?;

        log::info!("tracking head from {} to {}", self.ptr, self.latest);
        self.poll((self.ptr..=self.latest).collect()).await?;
        Ok(())
    }

    /// start polling service
    pub async fn start(&mut self) -> Result<()> {
        self.firehose_init();

        loop {
            // restart when network error occurs
            let result = self.track_head().await;
            match result {
                Err(Error::StopBlockReached) => {
                    log::info!(
                        "stop block {} reached, stopping poller",
                        self.end.expect("stop block reached, must be set")
                    );
                    return Ok(());
                }

                Err(e) => {
                    log::error!("{:?}", e);

                    if self.forever {
                        log::info!("restarting...");
                        continue;
                    } else {
                        return Err(e);
                    }
                }

                _ => {
                    log::info!(
                        "sleeping {}ms before checking for new blocks (last irrerversible block {})",
                        self.block_time,
                        self.latest,
                    );
                    tokio::time::sleep(Duration::from_millis(self.block_time)).await;
                }
            };
        }
    }
}
