// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(unused)]

use crate::{
    pb::{stream_server::Stream, ForkStep, Request, Response},
    service::grpc::{
        result::{Error, Result},
        types::BlocksStream,
    },
    Client, Storage,
};
use async_trait::async_trait;
use futures::{future::join_all, lock::Mutex, stream::iter, stream::Iter};
use std::sync::Arc;
use tonic::{Request as RequestT, Response as ResponseT, Status};

// irreversibility_condition for arweave
//
// - "confirm:20"
pub const IRREVERSIBILITY_CONDITION: &str = "confirms";

/// BlocksStream handler
pub struct StreamHandler {
    pub client: Arc<Client>,
    pub confirms: u64,
    pub storage: Storage,
    pub latest: Arc<Mutex<u64>>,
}

impl StreamHandler {
    /// construct response from block number
    pub async fn response(&self, num: u64, latest: u64) -> Result<Response> {
        let block = if let Ok(block) = self.storage.get(num) {
            block
        } else {
            self.client
                .get_firehose_block_by_height(num)
                .await
                .map_err(|_| Error::BlockNotFound)?
        };

        let step = if num < latest {
            ForkStep::StepIrreversible
        } else {
            ForkStep::StepNew
        };

        Ok(Response {
            block: block.try_into().ok(),
            step: step as i32,
            cursor: num.to_string(),
        })
    }
}

#[async_trait]
impl Stream for StreamHandler {
    type BlocksStream = Iter<BlocksStream>;

    /// # TODO
    ///
    /// add logic of getting blocks from storage
    async fn blocks(&self, request: RequestT<Request>) -> Result<ResponseT<Self::BlocksStream>> {
        let req = request.into_inner();

        // parse start block num
        let start = if let Ok(start) = req.start_cursor.parse() {
            start
        } else if req.start_block_num < 0 {
            let neg = req.start_block_num.abs() as u64;
            (*self.latest.lock().await).max(neg) - neg
        } else {
            req.start_block_num as u64
        };

        // parse `irreversibility_condition`
        let confirms: u64 = req
            .irreversibility_condition
            .trim_start_matches("confirms")
            .parse()
            .unwrap_or(self.confirms);

        // parse end block num
        let mut latest = *self.latest.lock().await;
        latest = if confirms > self.confirms {
            latest + (confirms - self.confirms)
        } else {
            latest - (self.confirms - confirms)
        };

        let end = if req.stop_block_num != 0 {
            req.stop_block_num
        } else {
            latest
        };

        // get blocks stream
        let mut stream: Vec<Result<Response>> = join_all(
            (start..end)
                .map(|num| self.response(num, latest))
                .collect::<Vec<_>>(),
        )
        .await;

        // filter with fork step
        if !req.fork_steps.is_empty() {
            stream = stream
                .into_iter()
                .filter(|r| {
                    if let Ok(resp) = r {
                        req.fork_steps.contains(&resp.step)
                    } else {
                        false
                    }
                })
                .collect();
        }

        // # TODO
        //
        // filter with
        // - include_filter_expr
        // - exclude_filter_expr

        Ok(tonic::Response::new(iter(BlocksStream(stream))))
    }
}
