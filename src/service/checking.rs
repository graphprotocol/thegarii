// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
//! checking service

use crate::{service::Service, Client, Env, Result, Storage};
use async_trait::async_trait;
use std::time::Duration;

/// checking service
pub struct Checking {
    batch: u16,
    client: Client,
    storage: Storage,
    interval: u64,
}

impl Checking {
    /// check block continuous
    ///
    /// returns the missed block heights
    pub async fn missing(storage: &Storage) -> Result<Vec<u64>> {
        let last = storage.last()?;
        let total = storage.count()?;

        if total == last.height + 1 {
            return Ok(vec![]);
        }

        let in_db = storage.map_keys(|key, _| {
            let mut height = [0; 8];
            height.copy_from_slice(key);
            u64::from_le_bytes(height)
        });

        let latest = in_db.iter().max().unwrap_or(&0);
        Ok((0..*latest).filter(|h| !in_db.contains(h)).collect())
    }

    /// check missed blocks and re-poll
    pub async fn check(&self) -> Result<()> {
        let mut missing = Self::missing(&self.storage).await?;
        if missing.is_empty() {
            return Ok(());
        }

        log::info!("checking blocks, missing {:?} blocks", missing.len());
        while !missing.is_empty() {
            let mut next = missing.clone();
            if missing.len() > self.batch as usize {
                missing = next.split_off(self.batch as usize);
            } else {
                missing.drain(..);
            }

            log::info!(
                "refetching blocks {:?}..{:?}...",
                next.first().unwrap_or(&0),
                next.last().unwrap_or(&0),
            );
            self.storage
                .write(self.client.poll(next.into_iter()).await?)
                .await?;
        }

        Ok(())
    }

    /// trigger checking service
    pub async fn checking(&mut self) -> Result<()> {
        loop {
            tokio::time::sleep(Duration::from_millis(self.interval)).await;
            self.check().await?;
        }
    }
}

#[async_trait]
impl Service for Checking {
    const NAME: &'static str = "checking";

    /// new checking service
    async fn new(env: &Env, storage: Storage) -> Result<Self> {
        let client = Client::new(
            env.endpoints.clone(),
            Duration::from_millis(env.timeout),
            env.retry,
        )?;

        Ok(Self {
            batch: env.polling_batch_blocks,
            client,
            storage,
            interval: env.checking_interval,
        })
    }

    /// run checking service
    async fn run(&mut self) -> Result<()> {
        return self.checking().await;
    }
}
