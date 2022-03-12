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

    /// Start pulling from the clients of the most recent node
    pub async fn pull_latest(&mut self) -> Result<()> {
        let client = self.select_random_client();
        let block = client.get_current_block().await?;

        log::info!(
            "pull block at height: {:?}, # of txns is {:}",
            block.height,
            block.txs.len()
        );
        self.pull_txs(block.txs).await?;
        log::info!("finished pulling block {:?}", block.height);

        Ok(())
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

        log::info!(
            "pull block at height: {:?}, # of txns is {:}",
            height,
            block.txs.len()
        );
        self.pull_txs(block.txs).await?;
        log::info!("finished pulling block {:?}", height);

        Ok(())
    }

    async fn pull_txs(&mut self, txns: Vec<String>) -> Result<()> {
        // This is just a simple count down latch implementation using mutex.
        // There should be better solutions in actual prod implementation.
        // For testing, this should be enough.
        let simple_countdown = Arc::new(Mutex::new(txns.len()));
        for t in txns {
            let c = Arc::clone(&self.clients[self.next_node]);
            let l = Arc::clone(&simple_countdown);

            // increment the next node idx
            self.next_node = (self.next_node + 1) % self.clients.len();

            tokio::spawn(async move {
                match c.get_tx_by_id(&t).await {
                    Ok(tx) => {
                        // we should store to rocks db
                        log::debug!(
                            "fetched transaction: {:?} with last: {:?}",
                            tx.id,
                            tx.last_tx
                        );
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

        Ok(())
    }

    fn select_random_client(&self) -> &Client {
        let index = rand::thread_rng().gen_range(0..self.clients.len());
        &self.clients[index]
    }
}
