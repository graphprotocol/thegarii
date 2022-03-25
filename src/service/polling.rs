// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! polling service

use crate::{service::Service, Client, Env, Result, Storage};
use async_trait::async_trait;
use std::time::Duration;

/// polling service
pub struct Polling {
    batch: u16,
    block_time: u64,
    client: Client,
    current: u64,
    ptr: u64,
    storage: Storage,
    confirms: u64,
}

impl Polling {
    /// trigger polling blocks
    async fn polling(&mut self) -> Result<()> {
        loop {
            let end = (self.ptr + self.batch as u64).min((self.current - self.confirms).max(0));
            log::info!("fetching blocks {}..{}/{}...", self.ptr, end, self.current);

            let blocks = self
                .client
                .poll(self.storage.missing(self.ptr..end).into_iter())
                .await?;
            self.storage.write(blocks).await?;

            self.ptr = end;
            if self.ptr + self.batch as u64 > self.current {
                tokio::time::sleep(Duration::from_millis(self.confirms * self.block_time)).await;
                self.current = self.client.get_current_block().await?.height;
            }
        }
    }
}

#[async_trait]
impl Service for Polling {
    const NAME: &'static str = "polling";

    /// new polling service
    async fn new(env: &Env, storage: Storage) -> Result<Self> {
        let client = Client::new(
            env.endpoints.clone(),
            Duration::from_millis(env.timeout),
            env.retry,
        )?;
        let ptr = if let Ok(last) = storage.last() {
            last.height
        } else {
            0
        };
        let current = client.get_current_block().await?.height;

        Ok(Self {
            batch: env.batch_blocks,
            block_time: env.block_time,
            client,
            current,
            ptr,
            confirms: env.confirms,
            storage,
        })
    }

    /// run polling service
    async fn run(&mut self) -> Result<()> {
        return self.polling().await;
    }
}
