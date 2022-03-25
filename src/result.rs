// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the graii results

use std::{env::VarError, io, num::ParseIntError};

/// the graii errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no endpoints provided")]
    EmptyEndpoints,
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error("block {0} not found")]
    BlockNotFound(u64),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("invalid block range")]
    InvalidRange,
    #[error("no block exists")]
    NoBlockExists,
    #[error("could not find data directory on this machine")]
    NoDataDirectory,
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
    #[error("can not write data in read-only mode")]
    ReadOnlyDatabase,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    RocksDB(#[from] rocksdb::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Var(#[from] VarError),
}

/// result type
pub type Result<T> = std::result::Result<T, Error>;
