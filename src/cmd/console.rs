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
    #[structopt(short = "s", long, default_value = "0")]
    start: u64,
}

impl Console {
    /// run as service
    pub async fn exec(&self, env: Env) -> Result<()> {
        log::debug!("\n{:#?}", self);
        log::info!("start polling blocks...");

        let mut polling = Polling::new(self.end, env, self.forever, self.start).await?;

        if let Err(e) = polling.start().await {
            log::error!("{:?}", e);
            return Err(e);
        }

        Ok(())
    }
}
