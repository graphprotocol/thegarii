// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::{
    service::{Service, Shared},
    Client, Result,
};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;
use std::time::Duration;

/// Tracking service
pub struct Tracking {
    pub client: Arc<Client>,
    pub confirms: u64,
    pub next: Duration,
    pub latest: Arc<Mutex<u64>>,
}

impl Tracking {
    async fn track(&self) -> Result<()> {
        let head = self.client.get_current_block().await?;
        if head.height < self.confirms {
            return Ok(());
        }

        // reset the latest block number
        let latest = head.height - self.confirms;
        *self.latest.lock().await = latest;
        log::info!("updated latest block ptr: {}", latest);
        Ok(())
    }
}

#[async_trait]
impl Service for Tracking {
    const NAME: &'static str = "tracking";

    fn new(shared: Shared) -> Result<Self> {
        Ok(Tracking {
            next: Duration::from_millis(shared.env.block_time),
            confirms: shared.env.confirms,
            client: shared.client,
            latest: shared.latest,
        })
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            tokio::time::sleep(self.next).await;
            self.track().await?;
        }
    }
}
