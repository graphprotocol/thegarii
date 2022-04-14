// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    client::Client,
    env::Env,
    pb::Block,
    types::{FirehoseBlock, U256},
    Error, Result,
};
use prost::Message;
use std::{collections::BTreeMap, time::Duration};

#[derive(Debug, Clone)]
struct BlockInfo {
    pub indep_hash: String,
    pub cumulative_diff: U256,
}

/// polling service
pub struct Polling {
    batch: usize,
    block_time: u64,
    client: Client,
    confirms: u64,
    end: Option<u64>,
    latest: u64,
    live_blocks: BTreeMap<u64, BlockInfo>,
    ptr: u64,
}

impl Polling {
    /// new polling service
    pub async fn new(ptr: u64, end: Option<u64>, env: Env) -> Result<Self> {
        let client = Client::new(env.endpoints, Duration::from_millis(env.timeout), env.retry)?;
        let batch = env.batch_blocks as usize;

        Ok(Self {
            batch,
            block_time: env.block_time,
            confirms: env.confirms,
            client,
            end,
            latest: 0,
            live_blocks: Default::default(),
            ptr,
        })
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

            // poll blocks
            let mut blocks = self.client.poll(polling.into_iter()).await?;
            self.cmp_live_blocks(&mut blocks)?;
            for b in blocks {
                let cur = b.height;
                Self::dm_log(b)?;
                // # Safty
                //
                // only update ptr after dm_log
                self.ptr = cur + 1;
            }
        }

        Ok(())
    }

    /// start polling service
    pub async fn start(&mut self) -> Result<()> {
        if let Some(end) = self.end {
            self.poll(self.ptr..=end).await?;

            return Ok(());
        }

        loop {
            let r: Result<()> = {
                self.latest = self.client.get_current_block().await?.height;
                self.poll(self.ptr..=self.latest).await?;
                Ok(())
            };

            // restart when network error occurs
            if let Err(e) = r {
                log::error!("{:?}, restarting...", e);
                continue;
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
