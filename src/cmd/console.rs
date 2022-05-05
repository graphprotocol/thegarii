// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{polling::Polling, Env, Result};
use std::fs;
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
    /// polling start from, if `None`, thegarii will poll from the block height
    /// stored in $PTR_FILE or 0
    #[structopt(short = "s", long)]
    start: Option<u64>,
}

impl Console {
    /// run as service
    pub async fn exec(&self, env: Env) -> Result<()> {
        log::debug!("\n{:#?}", self);
        let ptr = if let Some(start) = self.start {
            start
        } else {
            fs::read_to_string(&env.ptr_file)
                .ok()
                .map(|s| s.parse().ok())
                .flatten()
                .unwrap_or(0)
        };

        log::info!("start polling blocks from {}...", ptr);
        let mut polling = Polling::new(self.end, env, self.forever, ptr).await?;

        if let Err(e) = polling.start().await {
            log::error!("{:?}", e);
            return Err(e);
        }

        Ok(())
    }
}
