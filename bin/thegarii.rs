// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use thegarii::Opt;

#[tokio::main]
async fn main() {
    Opt::exec().await.expect("thegraii crashed")
}
