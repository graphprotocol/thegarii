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
    /// If never processed block before, start from -s if defined (use 'live' to start from "head" block)
    #[structopt(short = "s", long)]
    start: Option<String>,
    /// data directory where to store latest block fully processed
    #[structopt(short = "d", long, default_value = "./thegarii")]
    data_directory: String,
    /// reduce deep mind block output by just showing the length (not good for production!)
    #[structopt(short = "q", long)]
    quiet: bool,
}

impl Console {
    /// run as service
    pub async fn exec(&self, env: Env) -> Result<()> {
        log::debug!("\n{:?}", self);
        log::info!("start polling blocks...");

        let mut polling = Polling::new(
            self.data_directory.to_string(),
            self.end,
            env,
            self.forever,
            self.start.clone(),
            self.quiet,
        )
        .await?;

        if let Err(e) = polling.start().await {
            log::error!("{:?}", e);
            return Err(e);
        }

        Ok(())
    }
}
