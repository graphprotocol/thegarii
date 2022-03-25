// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
//! checking service

use crate::{
    service::{Service, Shared},
    Client, Result, Storage,
};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;
use std::time::Duration;

/// checking service
pub struct Polling {
    batch: u16,
    client: Client,
    confirms: u64,
    latest: Arc<Mutex<u64>>,
    storage: Storage,
}

impl Polling {
    /// get latest block from threads
    async fn get_latest(&self) -> u64 {
        let mut l = self.latest.lock().await.clone();
        if l > self.confirms {
            l -= self.confirms as u64;
        }

        l
    }

    /// returns the missing blocks
    pub async fn check(&self) -> Result<Vec<u64>> {
        let latest = self.get_latest().await;

        let in_db = self.storage.map_keys(|key, _| {
            let mut height = [0; 8];
            height.copy_from_slice(key);
            u64::from_le_bytes(height)
        });

        Ok((0..latest).filter(|h| !in_db.contains(h)).collect())
    }

    /// check missed blocks and re-poll
    pub async fn poll(&self) -> Result<()> {
        let mut blocks = self.check().await?;
        if blocks.is_empty() {
            return Ok(());
        }

        log::info!("pending {:?} blocks..", blocks.len());
        while !blocks.is_empty() {
            let latest = self.get_latest().await;
            let mut polling = blocks.clone();
            if polling.len() > self.batch as usize {
                blocks = polling.split_off(self.batch as usize);
            } else {
                blocks.drain(..);
            }

            log::info!(
                "polling blocks {}..{}/{}...",
                polling.first().unwrap_or(&0),
                polling.last().unwrap_or(&0),
                latest
            );

            let blocks = self.client.poll(polling.into_iter()).await?;
            self.storage.write(blocks).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl Service for Polling {
    const NAME: &'static str = "polling";

    /// new checking service
    async fn new(shared: Shared) -> Result<Self> {
        let client = Client::new(
            shared.env.endpoints.clone(),
            Duration::from_millis(shared.env.timeout),
            shared.env.retry,
        )?;

        Ok(Self {
            batch: shared.env.batch_blocks,
            client,
            confirms: shared.env.confirms,
            latest: shared.latest,
            storage: shared.storage,
        })
    }

    /// run polling service
    async fn run(&mut self) -> Result<()> {
        loop {
            self.poll().await?
        }
    }
}
