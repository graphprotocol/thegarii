// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{cmd::CommandT, Client, Env, Result, Storage};
use async_trait::async_trait;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Get {
    /// block number
    pub height: u64,
}

#[async_trait]
impl CommandT for Get {
    async fn exec(&self, env: Env) -> Result<()> {
        let storage = Storage::read_only(&env.db_path)?;

        let block = if let Ok(block) = storage.get(self.height) {
            block
        } else {
            log::warn!("block not exists, fetching from endpoints...");
            let client = Client::from_env()?;
            client.get_firehose_block_by_height(self.height).await?
        };

        println!("{}", serde_json::to_string_pretty(&block)?);
        Ok(())
    }
}
