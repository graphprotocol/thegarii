// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Env, Result};
use rocksdb::backup::{BackupEngine, BackupEngineOptions, RestoreOptions};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Restore {
    /// database path
    #[structopt(short, long)]
    pub db_path: Option<PathBuf>,

    /// restore database from this path
    pub restore_path: PathBuf,
}

impl Restore {
    /// backup database to path
    pub async fn exec(&self, mut env: Env) -> Result<()> {
        if let Some(db_path) = &self.db_path {
            env.with_db_path(db_path.into());
        }

        let mut engine = BackupEngine::open(&BackupEngineOptions::default(), &self.restore_path)?;
        engine.restore_from_latest_backup(
            &env.db_path,
            &env.db_path,
            &RestoreOptions::default(),
        )?;

        Ok(())
    }
}
