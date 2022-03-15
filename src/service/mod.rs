// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Env, Result};
use async_trait::async_trait;
use futures::{future::join_all, join};

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
        self.run().await?;
        Ok(())
    }
}

/// start services
pub async fn start() -> Result<()> {
    let env = Env::new()?;

    let (polling, checking) = join!(Polling::new(&env), Checking::new(&env));
    join_all(vec![polling?.start(), checking?.start()]).await;

    Ok(())
}
