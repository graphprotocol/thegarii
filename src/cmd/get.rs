// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Client, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Get {
    /// block number
    pub height: u64,
}

impl Get {
    pub async fn exec(&self) -> Result<()> {
        let client = Client::from_env()?;
        let block = client.get_firehose_block_by_height(self.height).await?;

        println!("{}", serde_json::to_string_pretty(&block)?);
        Ok(())
    }
}
