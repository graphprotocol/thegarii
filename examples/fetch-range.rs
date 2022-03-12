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

    // Note: if the blocks are really old, the nodes might not have
    // those blocks, it would throw error. In this case, we need to
    // handle those cases. But for now, testing, we can just use
    // more recent nodes.

    let start_block = 890825u64;
    let end_block = 890830u64;
    match extractor.pull(start_block, end_block).await {
        Ok(_) => {
            let elapsed = start.elapsed();
            log::info!("done, used: {:?}", elapsed);
        }
        Err(e) => log::error!("failed with reason: {:?}", e),
    }
}
