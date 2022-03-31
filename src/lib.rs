// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the garii project
//!
//! this library fetches blocks from Arweave and generates firehose blocks for the Graph
pub mod client;
pub mod cmd;
mod console;
mod encoding;
pub mod env;
pub mod pb;
pub mod result;
pub mod service;
mod storage;
pub mod types;

pub use self::{
    client::Client,
    cmd::Opt,
    console::Console,
    env::{Env, EnvArguments},
    result::{Error, Result},
    storage::Storage,
};
