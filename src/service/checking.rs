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
    /// check missed blocks and re-poll
    pub async fn check(&self) -> Result<()> {
        let missed = self.storage.continuous()?;
        if !missed.is_empty() {
            self.storage
                .write(self.client.poll(missed.into_iter()).await?)?;
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
    const NAME: &'static str = "polling";

    /// new checking service
    async fn new(env: &Env) -> Result<Self> {
        let client = Client::new(
            env.endpoints.clone(),
            Duration::from_millis(env.polling_timeout),
            env.polling_retry_times,
        )?;
        let storage = Storage::new(&env.db_path)?;

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
