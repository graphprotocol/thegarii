// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::pb::Response;
use crate::service::grpc::result::Result;

/// BlocksStream instance
///
/// # NOTE
///
/// use `Vec` since it's lighter than VecDedeup
pub struct BlocksStream(pub Vec<Result<Response>>);

impl From<Vec<Result<Response>>> for BlocksStream {
    fn from(v: Vec<Result<Response>>) -> Self {
        Self(v)
    }
}

impl Iterator for BlocksStream {
    type Item = Result<Response>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
