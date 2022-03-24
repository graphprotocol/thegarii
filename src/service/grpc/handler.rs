// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    pb::{stream_server::Stream, Request},
    service::grpc::types::BlocksStream,
};
use async_trait::async_trait;
use futures::stream::Iter;
use tonic::Status;

/// BlocksStream handler
pub struct StreamHandler;

#[async_trait]
impl Stream for StreamHandler {
    type BlocksStream = Iter<BlocksStream>;

    async fn blocks(
        &self,
        _request: tonic::Request<Request>,
    ) -> Result<tonic::Response<Self::BlocksStream>, Status> {
        Err(Status::aborted(""))
    }
}
