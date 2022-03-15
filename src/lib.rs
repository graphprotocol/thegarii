// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the garii project
//!
//! this library fetches blocks from Arweave and generates firehose blocks for the Graph
pub mod client;
pub mod cmd;
mod encoding;
pub mod env;
pub mod result;
pub mod service;
mod storage;
pub mod types;

pub use self::{
    client::Client,
    cmd::Opt,
    env::Env,
    result::{Error, Result},
    storage::Storage,
};
