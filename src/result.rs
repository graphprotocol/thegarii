// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the graii results

use std::{env::VarError, io};

/// the graii errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error("block {0} not found")]
    BlockNotFound(u64),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("could not find data directory on this machine")]
    NoDataDirectory,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    RocksDB(#[from] rocksdb::Error),
    #[error(transparent)]
    Var(#[from] VarError),
}

/// result type
pub type Result<T> = std::result::Result<T, Error>;
