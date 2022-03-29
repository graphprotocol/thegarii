// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;

pub use self::{grpc::Grpc, tracking::Tracking};
use crate::{Client, Env, Result, Storage};

pub mod grpc;
mod polling;
mod tracking;

pub use polling::Polling;

/// shared data
#[derive(Clone)]
pub struct Shared {
    pub client: Arc<Client>,
    pub env: Arc<Env>,
    pub latest: Arc<Mutex<u64>>,
    pub storage: Storage,
}

#[async_trait]
pub trait Service: Sized {
    const NAME: &'static str;

    /// new service instance
    fn new(shared: Shared) -> Result<Self>;

    /// run service
    async fn run(&mut self) -> Result<()>;

    /// start service
    async fn start(&mut self) -> Result<()> {
        log::info!("start {} service...", Self::NAME);

        loop {
            if let Err(e) = self.run().await {
                log::warn!("{} service is down {}, restarting...", Self::NAME, e);
            }
        }
    }
}
