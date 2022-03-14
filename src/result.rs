// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! the graii results

/// the graii errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no endpoints provided")]
    EmptyEndpoints,
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

/// result type
pub type Result<T> = std::result::Result<T, Error>;
