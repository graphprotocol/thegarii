// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::time::Instant;
use thegarii::Extractor;

#[tokio::main]
async fn main() {
    env_logger::init();

    let start = Instant::now();
    let nodes = vec![
        "https://arweave.net/",
        "http://51.75.206.225:1984/",
        "http://51.195.254.19:1984/",
        "http://51.178.38.52:1984/",
        "http://178.62.222.154:1984/",
        "http://188.166.200.45:1984/",
        "http://178.170.48.167:1984/",
    ];
    let mut extractor = Extractor::new(nodes);
    match extractor.pull_latest().await {
        Ok(_) => {
            let elapsed = start.elapsed();
            log::info!("done, used: {:?}", elapsed);
        }
        Err(e) => log::error!("failed with reason: {:?}", e),
    }
}
