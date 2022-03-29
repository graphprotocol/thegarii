// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{
    cmd::CommandT,
    pb::{stream_client::StreamClient, Request},
    Env, Result,
};
use async_trait::async_trait;
use futures::StreamExt;
use structopt::StructOpt;

/// stream blocks from firehose service
#[derive(StructOpt, Debug)]
pub struct Stream {
    /// Controls where the stream of blocks will start.
    #[structopt(short = "s", long, default_value = "0")]
    pub start_block_num: i64,
    /// Controls where the stream of blocks will start which will be immediately after
    /// the Block pointed by this opaque cursor.
    #[structopt(short = "c", long, default_value = "")]
    pub start_crusor: String,
    // When non-zero, controls where the stream of blocks will stop.
    #[structopt(short = "e", long, default_value = "0")]
    pub stop_block_number: u64,
    /// "confirms:20"
    #[structopt(short, long, default_value = "confirms:20")]
    pub irreversibility_condition: String,
}

#[async_trait]
impl CommandT for Stream {
    async fn exec(&self, env: Env) -> Result<()> {
        let addr = format!("http://{}:{}", env.grpc_addr.ip(), env.grpc_addr.port());
        let mut client = StreamClient::connect(addr.clone()).await?;
        log::info!("connected {:?}", addr);

        // construct request
        let req = tonic::Request::new(Request {
            start_block_num: self.start_block_num,
            start_cursor: "".into(),
            stop_block_num: self.stop_block_number,
            fork_steps: vec![],
            include_filter_expr: "".into(),
            exclude_filter_expr: "".into(),
            irreversibility_condition: "".into(),
            transforms: vec![],
        });

        // fetch metadata
        log::info!("fetching metadata...");
        let res = client.blocks(req).await?;
        log::info!("got response {:?}", res);

        let mut streaming = res.into_inner();
        log::info!("message {:?}", streaming.message().await);

        while let Some(stream) = streaming.next().await {
            log::info!("message {:?}", stream);
        }

        Ok(())
    }
}
