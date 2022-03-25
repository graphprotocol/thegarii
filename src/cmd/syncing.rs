// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Client, Env, Result, Storage};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Syncing {
    /// database path
    #[structopt(short, long)]
    pub db_path: Option<PathBuf>,
}

/// sync status
#[derive(Debug)]
pub struct SyncingStatus {
    pub current: u64,
    pub syncing: u64,
}

impl Syncing {
    /// start services
    pub async fn exec(&self, mut env: Env) -> Result<()> {
        if let Some(db_path) = &self.db_path {
            env.with_db_path(db_path.into());
        }

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
