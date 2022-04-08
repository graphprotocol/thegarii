// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{client::Client, pb::Block, Env, Error, Result};
use prost::Message;
use std::{fs, path::PathBuf, sync::Arc, time::Duration};

/// block pointer
pub struct Ptr {
    pub value: u64,
    path: PathBuf,
}

impl Ptr {
    /// new ptr host
    pub fn new(path: PathBuf) -> Result<Self> {
        let value: u64 = fs::read_to_string(&path)
            .ok()
            .and_then(|v: String| v.parse().ok())
            .unwrap_or(0);

        Ok(Ptr { value, path })
    }

    /// get mut ptr
    pub fn update(&self, value: u64) -> Result<()> {
        fs::write(&self.path, value.to_string())?;
        Ok(())
    }
}

/// console service
pub struct Console {
    batch: u16,
    block_time: u64,
    client: Arc<Client>,
    confirms: u64,
    ptr: Ptr,
}

impl Console {
    /// new console service
    pub fn new(env: Env) -> Result<Self> {
        let client = Arc::new(Client::new(
            env.endpoints.clone(),
            Duration::from_millis(env.timeout),
            env.retry,
        )?);

        let ptr = Ptr::new(env.ptr_path)?;

        Ok(Self {
            batch: env.batch_blocks,
            block_time: env.block_time,
            client,
            confirms: env.confirms,
            ptr,
        })
    }

    /// get latest block
    async fn get_latest(&self) -> Result<u64> {
        Ok(self
            .client
            .get_current_block()
            .await?
            .height
            .saturating_sub(self.confirms))
    }

    /// poll blocks and write to stdout
    pub async fn poll(&mut self) -> Result<()> {
        log::info!("start polling blocks...");
        log::info!(
            "\n\t-batch: {}\n\t-confirms: {}\n\t-ptr: {}\n\t-endpoints: {:?}",
            self.batch,
            self.confirms,
            self.ptr.value,
            self.client.endpoints
        );
        let latest = self.get_latest().await?;
        let mut blocks = (self.ptr.value..=latest).collect::<Vec<u64>>();

        if blocks.is_empty() {
            return Ok(());
        }

        while !blocks.is_empty() {
            let mut polling = blocks.clone();
            if polling.len() > self.batch as usize {
                blocks = polling.split_off(self.batch as usize);
            } else {
                blocks.drain(..);
            }

            let blocks = self.client.poll(polling.into_iter()).await?;
            for b in blocks {
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

                self.ptr.update(height)?;
            }
        }

        Ok(())
    }

    /// run as service
    pub async fn exec(&mut self) -> Result<()> {
        loop {
            tokio::time::sleep(Duration::from_millis(self.block_time)).await;
            self.poll().await?;
        }
    }
}
