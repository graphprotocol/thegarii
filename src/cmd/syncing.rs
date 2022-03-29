// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{cmd::CommandT, Client, Env, Result, Storage};
use async_trait::async_trait;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Syncing {}

/// sync status
#[derive(Debug)]
pub struct SyncingStatus {
    pub current: u64,
    pub syncing: u64,
}

#[async_trait]
impl CommandT for Syncing {
    /// start services
    async fn exec(&self, env: Env) -> Result<()> {
        let client = Client::from_env()?;
        let storage = Storage::read_only(&env.db_path)?;

        println!(
            "{:#?}",
            SyncingStatus {
                current: client.get_current_block().await?.height,
                syncing: storage.count()?
            }
        );

        Ok(())
    }
}
