// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::time::Instant;
use thegarii::Extractor;

async fn run(nodes: &Vec<&'static str>, loops: usize) -> (u128, u128) {
    let start = Instant::now();
    let mut extractor = Extractor::new(nodes);

    let mut total_time = 0;
    let mut round = 0;
    for _ in 0..loops {
        match extractor.pull_latest().await {
            Ok(_) => {
                let elapsed = start.elapsed();
                log::info!("done, used: {:?}", elapsed);
                total_time += elapsed.as_millis();
                round += 1;
            }
            Err(e) => log::error!("failed with reason: {:?}", e),
        }
    }

    (total_time, round)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let nodes = vec![
        "https://arweave.net/",
        // "http://51.75.206.225:1984/",
        // "http://51.195.254.19:1984/",
        // "http://51.178.38.52:1984/",
        // "http://178.62.222.154:1984/",
        // "http://188.166.200.45:1984/",
        // "http://178.170.48.167:1984/",
    ];

    let loops = 10usize;
    let (total, success_rounds) = run(&nodes, loops).await;
    println!(
        "finished running success loops: {:?} in {:?} millis, avg: {:?} mills",
        success_rounds,
        total,
        total / success_rounds
    );
}
