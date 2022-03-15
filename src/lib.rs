// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the garii project
//!
//! this library fetches blocks from Arweave and generates firehose blocks for the Graph
pub mod client;
mod encoding;
mod env;
pub mod result;
mod service;
mod storage;
pub mod types;

pub use self::{
    client::Client,
    result::{Error, Result},
};
