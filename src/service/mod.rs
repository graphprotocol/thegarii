// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Env, Result};
use async_trait::async_trait;

mod checking;
mod polling;

pub use self::{checking::Checking, polling::Polling};

#[async_trait]
pub trait Service: Sized {
    const NAME: &'static str;

    /// new service instance
    async fn new(env: &Env) -> Result<Self>;

    /// run service
    async fn run(&mut self) -> Result<()>;

    /// start service
    async fn start(&mut self) -> Result<()> {
        log::info!("start {} service...", Self::NAME);

        if let Err(e) = self.run().await {
            log::error!("{} service is down {}, restarting...", Self::NAME, e);
            self.start().await?;
        }

        Ok(())
    }
}
