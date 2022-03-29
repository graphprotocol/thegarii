// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
//! checking service

use crate::{
    service::{Service, Shared},
    Client, Result,
};
use async_trait::async_trait;
use futures::lock::Mutex;
use std::env;
use std::sync::Arc;

const PTR_PATH: &str = "thegarii/ptr";
const PTR_KEY: &str = "PTR_PATH";

/// checking service
pub struct Polling {
    batch: u16,
    client: Arc<Client>,
    latest: Arc<Mutex<u64>>,
    ptr_path: String,
    ptr: Option<u64>,
}

impl Polling {
    /// new polling service
    fn new_with_path(shared: Shared, ptr_path: String) -> Result<Self> {
        Ok(Self {
            batch: shared.env.batch_blocks,
            client: shared.client,
            latest: shared.latest,
            ptr_path,
            ptr: None,
        })
    }
    /// get latest block from threads
    ///
    /// # NOTE
    ///
    /// we can use this `latest` directly since it has already
    /// minus `confirms` in the tracking service
    async fn get_latest(&self) -> u64 {
        *self.latest.lock().await
    }

    async fn load_ptr(&self) -> u64 {
        match self.ptr {
            Some(v) => v,
            None => tokio::fs::read_to_string(&self.ptr_path)
                .await
                .map(|f| f.parse::<u64>().expect("invalid ptr str, should be u64"))
                .unwrap_or(0u64),
        }
    }

    async fn save_ptr(&mut self, ptr: u64) {
        self.ptr = Some(ptr);
        tokio::fs::write(&self.ptr_path, ptr.to_string().as_bytes())
            .await
            .expect("cannot save ptr");
    }

    /// check missing blocks and re-poll
    pub async fn poll(&mut self) -> Result<()> {
        let mut ptr = self.load_ptr().await;
        let mut blocks = (ptr..=self.get_latest().await).collect::<Vec<u64>>();

        if blocks.is_empty() {
            return Ok(());
        }

        while !blocks.is_empty() {
            let mut polling = blocks.clone();
            if polling.len() > self.batch as usize {
                blocks = polling.split_off(self.batch as usize);
            } else {
                blocks.drain(..);
            }

            let blocks = self.client.poll(polling.into_iter()).await?;

            // We print to console as requested by the project client in sequential order
            for b in blocks.iter() {
                // If fails, let the program crash
                let block_proto: crate::pb::Block = b.try_into()?;
                let encoding = block_proto.encode_to_vec();
                println!("DMLOG BLOCK {:?} {:}", b.height, hex::encode(bytes));
            }

            ptr += blocks.len() as u64;
            self.save_ptr(ptr).await;
        }

        Ok(())
    }
}

#[async_trait]
impl Service for Polling {
    const NAME: &'static str = "polling";

    /// new polling service
    fn new(shared: Shared) -> Result<Self> {
        let path = env::var(PTR_KEY).unwrap_or_else(|_| PTR_PATH.parse().unwrap());
        Self::new_with_path(shared, path)
    }

    /// run polling service
    async fn run(&mut self) -> Result<()> {
        loop {
            self.poll().await?
        }
    }
}
