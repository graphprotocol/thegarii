// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{polling::Polling, Env, Result};
use structopt::StructOpt;

/// console service
#[derive(Debug, StructOpt)]
pub struct Console {
    /// polling end to, if `None`, polling to the latest
    #[structopt(short = "e", long)]
    end: Option<u64>,
    /// if restarting service on failing automatically
    #[structopt(short = "f", long)]
    forever: bool,
    /// polling start from, if `None`, polling from 0
    #[structopt(short = "s", long)]
    start: Option<u64>,
    /// data directory where to store latest block fully processed
    #[structopt(short = "d", long, default_value = "./thegarii")]
    data_directory: String,
}

impl Console {
    /// run as service
    pub async fn exec(&self, env: Env) -> Result<()> {
        let mut polling = Polling::new(
            self.data_directory.to_string(),
            self.end,
            env,
            self.forever,
            self.start,
        )
        .await?;

        if let Err(e) = polling.start().await {
            log::error!("{:?}", e);
            return Err(e);
        }

        Ok(())
    }
}
