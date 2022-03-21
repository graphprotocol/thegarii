// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Client, Env, Result, Storage};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Get {
    /// database path
    #[structopt(short, long)]
    pub db_path: Option<PathBuf>,
    /// block number
    pub height: u64,
}

impl Get {
    pub async fn exec(&self, mut env: Env) -> Result<()> {
        if let Some(db_path) = &self.db_path {
            env.with_db_path(db_path.into());
        }

        let storage = Storage::new(&env.db_path)?;
        let block = if let Ok(block) = storage.get(self.height) {
            block
        } else {
            let client = Client::from_env()?;
            client.get_firehose_block_by_height(self.height).await?
        };

        println!("{:#?}", block);
        Ok(())
    }
}
