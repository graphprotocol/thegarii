// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::io::Result;
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=./proto/block.proto");
    tonic_build::configure()
        .out_dir("src/protobuf")
        .format(true)
        .compile(&["proto/block.proto"], &["proto"])
}
