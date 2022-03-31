// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{cmd::CommandT, Env, Result, Storage};
use async_trait::async_trait;
use rocksdb::backup::{BackupEngine, BackupEngineOptions};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Backup {
    /// backup database to this path
    pub backup_path: PathBuf,
}

#[async_trait]
impl CommandT for Backup {
    /// backup database to path
    async fn exec(&self, env: Env) -> Result<()> {
        let storage = Storage::new(&env.db_path)?;

        let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &self.backup_path)?;
        engine.create_new_backup(&storage.read)?;

        Ok(())
    }
}
