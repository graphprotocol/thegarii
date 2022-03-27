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

/// checking service
pub struct Polling {
    batch: u16,
    client: Arc<Client>,
    latest: Arc<Mutex<u64>>,
    storage: Storage,
}

impl Polling {
    /// get latest block from threads
    ///
    /// # NOTE
    ///
    /// we can use this `latest` directly since it has already
    /// minus `confirms` in the tracking service
    async fn get_latest(&self) -> u64 {
        *self.latest.lock().await
    }

    /// returns the missing blocks
    pub async fn check(&self) -> Result<u64> {
        let count = self.storage.count()?;

        // if storage is not continuous
        log::info!("checking continuous...");
        let mut blocks = self.storage.map_keys(|k, _| {
            let mut b = [0; 8];
            b.copy_from_slice(k);

            u64::from_le_bytes(b)
        });

        blocks.sort_unstable();

        Ok(blocks
            .into_iter()
            .enumerate()
            .filter(|(idx, height)| (*idx as u64) != *height)
            .min()
            .map(|(_, v)| v)
            .unwrap_or(count))
    }

    /// check missed blocks and re-poll
    pub async fn poll(&self) -> Result<()> {
        let ptr = self.check().await?;
        let mut blocks = (ptr..=self.get_latest().await).collect::<Vec<u64>>();

        if blocks.is_empty() {
            return Ok(());
        }

        while !blocks.is_empty() {
            let latest = self.get_latest().await;
            let mut polling = blocks.clone();
            if polling.len() > self.batch as usize {
                blocks = polling.split_off(self.batch as usize);
            } else {
                blocks.drain(..);
            }

            polling = self.storage.missing(polling.into_iter());
            if polling.is_empty() {
                continue;
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
    fn new(shared: Shared) -> Result<Self> {
        Ok(Self {
            batch: shared.env.batch_blocks,
            client: shared.client,
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
