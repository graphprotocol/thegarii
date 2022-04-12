// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{client::Client, env::Env, pb::Block, types::FirehoseBlock, Error, Result};
use prost::Message;
use std::{collections::BTreeMap, time::Duration};

/// polling service
pub struct Polling {
    batch: usize,
    client: Client,
    confirms: u64,
    end: Option<u64>,
    live_blocks: BTreeMap<u64, String>,
    ptr: u64,
}

impl Polling {
    /// new polling service
    pub fn new(ptr: u64, end: Option<u64>, env: Env) -> Result<Self> {
        let client = Client::new(env.endpoints, Duration::from_millis(env.timeout), env.retry)?;
        let batch = env.batch_blocks as usize;

        Ok(Self {
            batch,
            confirms: env.confirms,
            client,
            end,
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

    /// comparing which block have the potential to be irreversible
    ///
    /// if true, rhs wins
    fn challenge_block(lhs: &FirehoseBlock, rhs: &FirehoseBlock) -> bool {
        false
    }

    /// compare blocks with current live blocks
    ///
    /// return the height of fork block if exists
    fn cmp_live_blocks(&mut self, blocks: &[FirehoseBlock], end: u64) -> Result<Option<u64>> {
        if blocks.is_empty() {
            return Ok(None);
        }

        // # Safty
        //
        // this will never happen since we have an empty check above
        let last = blocks.last().ok_or(Error::ParseBlockPtrFailed)?.clone();
        if last.height + self.confirms < end {
            return Ok(None);
        }

        // - check if have fork
        // - add new live blocks
        for b in blocks {
            // check fork
            if let Some(value) = self.live_blocks.get(&b.height) {
                if *value != b.indep_hash {
                    self.live_blocks.insert(b.height, b.indep_hash.clone());
                }
            }
        }

        // trim irreversible blocks
        self.live_blocks = self
            .live_blocks
            .clone()
            .into_iter()
            .filter(|(h, _)| *h > end - self.confirms)
            .collect();

        Ok(None)
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

            // # Safty
            //
            // this will never happen since we have an empty check above
            let new_ptr = polling.last().ok_or(Error::ParseBlockPtrFailed)?.clone();
            let blocks = self.client.poll(polling.into_iter()).await?;
            for b in blocks {
                Self::dm_log(b)?;
            }

            // update ptr
            self.ptr = new_ptr;
        }

        Ok(())
    }

    /// start polling service
    pub async fn start(&mut self) -> Result<()> {
        if let Some(end) = self.end {
            self.poll(self.ptr..end).await?;

            return Ok(());
        }

        Ok(())
    }
}
