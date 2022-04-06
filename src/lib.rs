// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the garii project
//!
//! this library fetches blocks from Arweave and generates firehose blocks for the Graph
pub mod client;
mod console;
mod encoding;
pub mod env;
pub mod pb;
pub mod result;
pub mod types;

#[cfg(feature = "full")]
pub mod cmd;
#[cfg(feature = "full")]
pub mod service;
#[cfg(feature = "full")]
mod storage;

#[cfg(feature = "full")]
pub use storage::Storage;

#[cfg(not(feature = "firehose"))]
pub use cmd::Opt;

#[cfg(feature = "firehose")]
pub use console::cmd::Opt;

pub use self::{
    client::Client,
    console::Console,
    env::{Env, EnvArguments},
    result::{Error, Result},
};
