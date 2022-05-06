// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::process::exit;

use thegarii::Opt;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tokio::spawn(async {
        if let Err(err) = signal::ctrl_c().await {
            log::error!("error waiting for SIGINT signal: {:?}", err);
            exit(1);
        }

        log::info!("received SIGINT signal, terminating");
        exit(0);
    });

    match Opt::exec().await {
        Ok(_) => {
            log::info!("completed");
            Ok(())
        }
        Err(err) => {
            log::error!("unexpected error occurred: {:?}", err);
            exit(1);
        }
    }
}
