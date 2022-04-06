// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use crate::{cmd::CommandT, Console as ConsoleService, Env, Result};
use async_trait::async_trait;
use structopt::StructOpt;

/// polling blocks and write to stdout
#[derive(StructOpt, Debug)]
pub struct Console {}

#[async_trait]
impl CommandT for Console {
    async fn exec(&self, env: Env) -> Result<()> {
        let mut service = ConsoleService::new(env)?;

        service.exec().await?;
        Ok(())
    }
}
