// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(unused)]

use crate::{
    pb::{stream_server::Stream, Request},
    service::grpc::types::BlocksStream,
    Client, Storage,
};
use async_trait::async_trait;
use futures::stream::Iter;
use tonic::Status;

/// BlocksStream handler
pub struct StreamHandler {
    client: Client,
    storage: Storage,
}

impl StreamHandler {
    pub fn new(client: Client, storage: Storage) -> Self {
        StreamHandler { client, storage }
    }
}

#[async_trait]
impl Stream for StreamHandler {
    type BlocksStream = Iter<BlocksStream>;

    /// # TODO
    ///
    /// add logic of getting blocks from storage
    async fn blocks(
        &self,
        request: tonic::Request<Request>,
    ) -> Result<tonic::Response<Self::BlocksStream>, Status> {
        let req = request.into_inner();

        // # TODO
        //
        // filter with `fork_steps`

        Err(Status::aborted(""))
    }
}
