// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! gRPC service

use crate::{pb::stream_server::StreamServer, service::Service, Env, Result, Storage};
use async_trait::async_trait;
use handler::StreamHandler;

pub mod handler;
pub mod types;

/// gRPC service
pub struct Grpc {
    _server: StreamServer<handler::StreamHandler>,
}

#[async_trait]
impl Service for Grpc {
    const NAME: &'static str = "grpc";

    /// new gRPC service
    async fn new(_env: &Env, _storage: Storage) -> Result<Self> {
        Ok(Self {
            _server: StreamServer::new(StreamHandler),
        })
    }

    /// run gRPC service
    async fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
