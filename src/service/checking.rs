// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
//! checking service

use crate::{service::Service, Client, Env, Result, Storage};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::{sync::Arc, time::Duration};

/// checking service
pub struct Checking {
    client: Client,
    storage: Arc<Mutex<Storage>>,
    interval: u64,
}

impl Checking {
    /// check missed blocks and re-poll
    pub async fn check(&self) -> Result<()> {
        let storage = self.storage.lock().await;
        let missed = storage.continuous()?;
        if !missed.is_empty() {
            storage.write(self.client.poll(missed.into_iter()).await?)?;
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
    async fn new(env: &Env, storage: Arc<Mutex<Storage>>) -> Result<Self> {
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
