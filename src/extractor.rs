// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Arweave block pulling orchestrator. It utilizes the Client to pull
//! data from different blocks based on the input configuration.

use crate::result::Result;
use crate::Client;
use rand::Rng;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;

/// The Extractor struct that handles the pulling of different nodes
pub struct Extractor {
    /// The list of node clients to pull data from
    clients: Vec<Arc<Client>>,
    /// The next node to query data from
    next_node: usize,
}

impl Extractor {
    pub fn new(nodes: Vec<&'static str>) -> Self {
        Self {
            clients: nodes.iter().map(|n| Arc::new(Client::new(n))).collect(),
            next_node: 0,
        }
    }

    /// Start pulling from the clients from the start to the
    /// end blocks, exclusive of the end block
    pub async fn pull(&mut self, start: u64, end: u64) -> Result<()> {
        log::info!("pulling block from {:?} to {:?}", start, end - 1);
        for block in start..end {
            self.pull_block(block).await?;
        }
        Ok(())
    }

    pub async fn pull_block(&mut self, height: u64) -> Result<()> {
        let client = self.select_random_client();
        let block = client.get_block_by_height(height).await?;

        log::debug!(
            "pulled block at height: {:?}, # txns is {:}",
            height,
            block.txs.len()
        );

        // This is just a simple count down latch implementation using mutex.
        // There should be better solutions in actual prod implementation.
        // For testing, this should be enough.
        let simple_countdown = Arc::new(Mutex::new(block.txs.len()));
        for t in block.txs {
            let c = Arc::clone(&self.clients[self.next_node]);
            let l = Arc::clone(&simple_countdown);

            // increment the next node idx
            self.next_node = (self.next_node + 1) % self.clients.len();

            tokio::spawn(async move {
                match c.get_tx_by_id(&t).await {
                    Ok(tx) => {
                        // we should store to rocks db
                        log::debug!("fetched transaction: {:?}", tx.id);
                    }
                    Err(e) => {
                        log::error!("todo, handle this error: {:?}", e);
                    }
                }
                let l_ref = &(&*l);
                let mut r = l_ref.lock().await;
                *r -= 1;
            });
        }

        loop {
            let countdown = simple_countdown.lock().await;
            log::debug!("countdown: {:?}", countdown);
            if countdown.eq(&0) {
                break;
            }
            thread::sleep(Duration::from_millis(1000));
        }

        log::info!("finished pulling block {:?}", height);

        Ok(())
    }

    fn select_random_client(&self) -> &Client {
        let index = rand::thread_rng().gen_range(0..self.clients.len());
        &self.clients[index]
    }
}
