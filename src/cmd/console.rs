// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::{cmd::CommandT, Console as ConsoleService, Env, Result};
use async_trait::async_trait;
use std::time::Duration;
use structopt::StructOpt;

/// polling blocks and write to stdout
#[derive(StructOpt, Debug)]
pub struct Console {}

#[async_trait]
impl CommandT for Console {
    async fn exec(&self, env: Env) -> Result<()> {
        let block_time = env.block_time;
        let mut service = ConsoleService::new(env)?;

        loop {
            tokio::time::sleep(Duration::from_millis(block_time)).await;
            service.poll().await?;
        }
    }
}
