// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Env, Result};
use rocksdb::backup::{BackupEngine, BackupEngineOptions, RestoreOptions};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Restore {
    /// restore database from this path
    pub restore_path: PathBuf,
}

impl Restore {
    /// backup database to path
    pub async fn exec(&self, env: Env) -> Result<()> {
        let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &self.restore_path)?;
        engine.restore_from_latest_backup(
            &env.db_path,
            &env.db_path,
            &RestoreOptions::default(),
        )?;

        Ok(())
    }
}
