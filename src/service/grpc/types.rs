// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::pb::Response;
use tonic::Status;

/// BlocksStream instance
///
/// # NOTE
///
/// use `Vec` since it's lighter than VecDedeup
pub struct BlocksStream(Vec<Result<Response, Status>>);

impl Iterator for BlocksStream {
    type Item = Result<Response, Status>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
