// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    client::Client,
    env::Env,
    pb::Block,
    types::{FirehoseBlock, U256},
    Error, Result,
};
use anyhow::Context;
use prost::Message;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Debug, Clone)]
struct BlockInfo {
    pub indep_hash: String,
    pub cumulative_diff: U256,
}

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
    live_blocks: BTreeMap<u64, BlockInfo>,
    ptr: u64,
    ptr_file: PathBuf,
}

impl Polling {
    /// new polling service
    pub async fn new(
        data_directory: String,
        end: Option<u64>,
        env: Env,
        forever: bool,
        ptr: Option<u64>,
    ) -> Result<Self> {
        let client = Client::new(env.endpoints, Duration::from_millis(env.timeout), env.retry)?;
        let batch = env.batch_blocks as usize;

        fs::create_dir_all(&data_directory).context(
            format_args!("unable to create data directory {}", &data_directory).to_string(),
        )?;

        let last_processed_block_path =
            Path::new(&data_directory).join("latest_block_processed.txt");

        let ptr = Self::determine_start_ptr(&last_processed_block_path, ptr)?;

        Ok(Self {
            last_processed_block_path: Box::new(last_processed_block_path),
            batch,
            block_time: env.block_time,
            confirms: env.confirms,
            client,
            end,
            forever,
            latest: 0,
            live_blocks: Default::default(),
            ptr,
            ptr_file: env.ptr_file,
        })
    }

    fn determine_start_ptr(
        last_processed_block_path: &PathBuf,
        start_block_flag: Option<u64>,
    ) -> Result<u64> {
        match last_processed_block_path.exists() {
            true => {
                let content: String = fs::read_to_string(last_processed_block_path).context(
                    format_args!(
                        "unable to read content of last block processsed file {:?}",
                        last_processed_block_path,
                    )
                    .to_string(),
                )?;

                let value = content.parse::<u64>().context(
                    format_args!("content {} is not a valid u64 string value", &content)
                        .to_string(),
                )?;

                log::info!(
                    "start block retrieved from last processed block state file, starting from block {}",
                    value
                );

                Ok(value)
            }
            false => {
                if let Some(start) = start_block_flag {
                    log::info!(
                        "start block explicitely provided, starting from block {}",
                        start
                    );

                    Ok(start)
                } else {
                    log::info!(
                        "no previous latest block processed file {:?} exists, starting from block 0",
                        &last_processed_block_path
                    );

                    Ok(0)
                }
            }
        }
    }

    /// dm log to stdout
    ///
    /// DMLOG BLOCK <HEIGHT> <ENCODED>
    fn dm_log(b: FirehoseBlock) -> Result<()> {
        let height = b.height;
        let proto: Block = b.try_into()?;

        println!(
            "DMLOG BLOCK {} {}",
            height,
            proto
                .encode_to_vec()
                .into_iter()
                .map(|b| format!("{:02x}", b))
                .reduce(|mut r, c| {
                    r.push_str(&c);
                    r
                })
                .ok_or(Error::ParseBlockFailed)?
        );

        Ok(())
    }

    /// compare blocks with current live blocks
    ///
    /// # TODO
    ///
    /// - return the height of fork block if exists
    /// - replace live_blocks field with a sorted stack
    fn cmp_live_blocks(&mut self, blocks: &mut [FirehoseBlock]) -> Result<()> {
        if blocks.is_empty() {
            return Ok(());
        }

        // # Safty
        //
        // this will never happen since we have an empty check above
        let last = blocks.last().ok_or(Error::ParseBlockPtrFailed)?.clone();
        if last.height + self.confirms < self.latest {
            return Ok(());
        }

        // - detect if have fork
        // - add new live blocks
        let mut dup_blocks = vec![];
        for b in blocks.iter() {
            let cumulative_diff =
                U256::from_dec_str(&b.cumulative_diff.clone().unwrap_or_else(|| "0".into()))?;

            let block_info = BlockInfo {
                indep_hash: b.indep_hash.clone(),
                cumulative_diff,
            };

            // detect fork
            if let Some(value) = self.live_blocks.get(&b.height) {
                // - comparing if have different `indep_hash`
                // - comparing if the block belongs to a longer chain
                if *value.indep_hash != b.indep_hash && cumulative_diff > value.cumulative_diff {
                    // TODO
                    //
                    // return fork number
                } else {
                    dup_blocks.push(b.height);
                    continue;
                }
            }

            // update live blocks
            if b.height + self.confirms > self.latest {
                self.live_blocks.insert(b.height, block_info);
            }
        }

        // remove emitted live blocks
        // blocks.retain(|b| !dup_blocks.contains(&b.height));

        // trim irreversible blocks
        self.live_blocks = self
            .live_blocks
            .clone()
            .into_iter()
            .filter(|(h, _)| *h + self.confirms > self.latest)
            .collect();

        log::trace!(
            "live blocks: {:?}",
            self.live_blocks.keys().into_iter().collect::<Vec<&u64>>()
        );
        Ok(())
    }

    /// poll blocks and write to stdout
    async fn poll(&mut self, blocks: impl IntoIterator<Item = u64>) -> Result<()> {
        let mut blocks = blocks.into_iter().collect::<Vec<u64>>();

        if blocks.is_empty() {
            return Ok(());
        }

        while !blocks.is_empty() {
            let mut polling = blocks.clone();
            if polling.len() > self.batch {
                blocks = polling.split_off(self.batch);
            } else {
                blocks.drain(..);
            }

            // poll blocks and dm logging
            let mut blocks = self.client.poll(polling.into_iter()).await?;
            self.cmp_live_blocks(&mut blocks)?;
            for b in blocks {
                let cur = b.height;
                Self::dm_log(b)?;

                // # Safty
                //
                // only update ptr after dm_log
                //
                // # NOTE
                //
                // Stores string for easy debugging
                self.ptr = cur + 1;

                self.write_ptr().await?;
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

    /// poll to head
    async fn track_head(&mut self) -> Result<()> {
        self.latest = self.client.get_current_block().await?.height;
        self.poll(self.ptr..=self.latest).await?;
        Ok(())
    }

    /// start polling service
    pub async fn start(&mut self) -> Result<()> {
        if let Some(end) = self.end {
            self.poll(self.ptr..=end).await?;

            return Ok(());
        }

        loop {
            // restart when network error occurs
            if let Err(e) = self.track_head().await {
                log::error!("{:?}", e);

                if self.forever {
                    log::info!("restarting...");
                    continue;
                } else {
                    return Err(e);
                }
            }

            // sleep and waiting for new blocks
            log::info!(
                "waiting for new blocks for {}ms... current height: {}",
                self.block_time,
                self.latest,
            );
            tokio::time::sleep(Duration::from_millis(self.block_time)).await;
        }
    }
}
