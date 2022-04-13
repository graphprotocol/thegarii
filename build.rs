// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

use std::io::Result;
fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=./proto");
    tonic_build::configure()
        .format(true)
        .compile(&["proto/type.proto"], &["proto"])
}
