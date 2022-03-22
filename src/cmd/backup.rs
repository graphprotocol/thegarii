// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Env, Result, Storage};
use rocksdb::backup::{BackupEngine, BackupEngineOptions};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Backup {
    /// database path
    #[structopt(short, long)]
    pub db_path: Option<PathBuf>,

    /// backup database to this path
    pub backup_path: PathBuf,
}

impl Backup {
    /// backup database to path
    pub async fn exec(&self, mut env: Env) -> Result<()> {
        if let Some(db_path) = &self.db_path {
            env.with_db_path(db_path.into());
        }

        let storage = Storage::new(&env.db_path)?;
        let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &self.backup_path)?;
        engine.create_new_backup_flush(&storage.read, true)?;

        Ok(())
    }
}
