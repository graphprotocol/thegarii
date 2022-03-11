// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::time::Instant;
use thegarii::Extractor;

#[tokio::main]
async fn main() {
    env_logger::init();

    let start = Instant::now();
    let nodes = vec!["https://arweave.net/"];
    let mut extractor = Extractor::new(nodes);

    match extractor.pull(269512u64, 269513u64).await {
        Ok(_) => {
            let elapsed = start.elapsed();
            log::info!("done, used: {:?}", elapsed);
        }
        Err(e) => log::error!("failed with reason: {:?}", e),
    }
}
