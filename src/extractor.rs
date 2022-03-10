// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! Arweave block pulling orchestrator. It utilizes the Client to pull
//! data from different blocks based on the input configuration.

use crate::result::Result;
use crate::types::Transaction;
use crate::Client;
use rand::Rng;
use std::sync::Arc;

/// The Extractor struct that handles the pulling of different nodes
pub struct Extractor {
    /// The list of node clients to pull data from
    clients: Vec<Arc<Client>>,
    /// The next node to query data from
    next_node: usize,
    /// The sender to the storage thread
    storage_sender: Arc<tokio::sync::mpsc::Sender<Transaction>>,
}

impl Extractor {
    pub fn new(nodes: Vec<&'static str>, sender: tokio::sync::mpsc::Sender<Transaction>) -> Self {
        Self {
            clients: nodes.iter().map(|n| Arc::new(Client::new(n))).collect(),
            next_node: 0,
            storage_sender: Arc::new(sender),
        }
    }

    /// Start pulling from the clients from the start to the
    /// end blocks, exclusive of the end block
    pub async fn pull(&mut self, start: u64, end: u64) -> Result<()> {
        for block in start..end {
            self.pull_block(block).await?;
        }
        Ok(())
    }

    pub async fn pull_block(&mut self, height: u64) -> Result<()> {
        let client = self.select_random_client();
        let block = client.get_block_by_height(height).await?;

        for t in block.txs {
            let c = Arc::clone(&self.clients[self.next_node]);
            let s = Arc::clone(&self.storage_sender);
            self.next_node = (self.next_node + 1) % self.clients.len();
            tokio::spawn(async move {
                match c.get_tx_by_id(&t).await {
                    Ok(tx) => s.send(tx).await,
                    Err(_) => {
                        todo!()
                    }
                }
            });
        }
        Ok(())
    }

    fn select_random_client(&self) -> &Client {
        let index = rand::thread_rng().gen_range(0..self.clients.len());
        &self.clients[index]
    }
}
