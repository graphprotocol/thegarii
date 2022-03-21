// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
//! checking service

use crate::{service::Service, Client, Env, Result, Storage};
use async_trait::async_trait;
use std::time::Duration;

/// checking service
pub struct Checking {
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

        if total == last.height {
            return Ok(vec![]);
        }

        let in_db = storage.map_keys(|key, _| {
            let mut height = [0; 8];
            height.copy_from_slice(key);
            u64::from_le_bytes(height)
        });

        Ok((0..total).filter(|h| !in_db.contains(h)).collect())
    }

    /// check missed blocks and re-poll
    pub async fn check(&self) -> Result<()> {
        let missing = Self::missing(&self.storage).await?;

        log::info!("checking blocks, missing: {:?}.", missing,);
        if !missing.is_empty() {
            self.storage
                .write(self.client.poll(missing.into_iter()).await?)
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
            Duration::from_millis(env.polling_timeout),
            env.polling_retry_times,
        )?;

        Ok(Self {
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
