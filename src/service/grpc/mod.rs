// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! gRPC service

use crate::{
    pb::stream_server::StreamServer,
    service::{Service, Shared},
    Client, Result,
};
use async_trait::async_trait;
use handler::StreamHandler;
use std::{net::SocketAddr, time::Duration};
use tonic::transport::Server;

pub mod handler;
pub mod types;

/// gRPC service
pub struct Grpc {
    addr: SocketAddr,
    server: StreamServer<handler::StreamHandler>,
}

#[async_trait]
impl Service for Grpc {
    const NAME: &'static str = "grpc";

    /// new gRPC service
    fn new(shared: Shared) -> Result<Self> {
        let client = Client::new(
            shared.env.endpoints.clone(),
            Duration::from_millis(shared.env.timeout),
            shared.env.retry,
        )?;

        Ok(Self {
            addr: shared.env.grpc_addr,
            server: StreamServer::new(StreamHandler::new(client, shared.storage)),
        })
    }

    /// run gRPC service
    async fn run(&mut self) -> Result<()> {
        Server::builder()
            .add_service(tonic_web::enable(self.server.clone()))
            .serve(self.addr)
            .await?;
        Ok(())
    }
}
