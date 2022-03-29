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
        let value: u64 = if let Ok(v) = fs::read_to_string(&path) {
            v.parse()?
        } else {
            0
        };

        Ok(Ptr { value, path })
    }

    /// get mut ptr
    pub fn get_mut(&mut self) -> &mut u64 {
        &mut self.value
    }
}

impl Drop for Ptr {
    fn drop(&mut self) {
        if let Err(e) = fs::write(&self.path, self.value.to_le_bytes()) {
            // PANIC if writing ptr failed
            panic!("failed to update ptr to ptr_path, {:?}", e);
        }
    }
}

/// console service
pub struct Console {
    batch: u16,
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
            .max(self.confirms)
            - self.confirms)
    }

    /// poll blocks and write to stdout
    pub async fn poll(&mut self) -> Result<()> {
        log::info!("start polling blocks...");
        log::info!(
            "\n\t-batch: {}\n\t-confirms: {}\n\t-ptr: {}",
            self.batch,
            self.confirms,
            self.ptr.value
        );
        let latest = self.get_latest().await?;
        let ptr = self.ptr.get_mut();
        let mut blocks = (*ptr..=latest).collect::<Vec<u64>>();

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
                    "DM_BLOCK {} {}",
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

                *ptr = height;
            }
        }

        Ok(())
    }
}
