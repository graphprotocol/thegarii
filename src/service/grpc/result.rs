// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use tonic::Status;

/// gRPC errors
pub enum Error {
    BlockNotFound,
}

impl From<Error> for Status {
    fn from(e: Error) -> Self {
        match e {
            Error::BlockNotFound => Status::not_found("block not found"),
        }
    }
}

pub type Result<T> = core::result::Result<T, Status>;
