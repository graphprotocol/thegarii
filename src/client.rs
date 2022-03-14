// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! arweave client
use crate::{
    result::{Error, Result},
    types::{Block, FirehoseBlock, Transaction},
};
use futures::future::join_all;
use rand::Rng;
use serde::de::DeserializeOwned;

/// Arweave client
pub struct Client {
    endpoints: Vec<&'static str>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            endpoints: vec!["https://arweave.net/"],
        }
    }
}

impl Client {
    fn next_endpoint(&self) -> String {
        self.endpoints[rand::thread_rng().gen_range(0..self.endpoints.len())].to_string()
    }

    pub fn new(endpoints: Vec<&'static str>) -> Result<Self> {
        if endpoints.is_empty() {
            return Err(Error::EmptyEndpoints);
        }

        Ok(Self { endpoints })
    }

    /// http get request with base url
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let mut url = self.next_endpoint();
        url.push_str(path);

        Ok(reqwest::get(url).await?.json().await?)
    }

    /// get arweave block by height
    ///
    /// ```rust
    /// use thegarii::types::Block;
    ///
    /// let client = thegarii::Client::default();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// { // block height 100 - https://arweave.net/block/height/100
    ///   let json = include_str!("../res/block_height_100.json");
    ///   let block = rt.block_on(client.get_block_by_height(100)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    ///
    /// { // block height 269512 - https://arweave.net/block/height/269512
    ///   let json = include_str!("../res/block_height_269512.json");
    ///   let block = rt.block_on(client.get_block_by_height(269512)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    ///
    /// { // block height 422250 - https://arweave.net/block/height/422250
    ///   let json = include_str!("../res/block_height_422250.json");
    ///   let block = rt.block_on(client.get_block_by_height(422250)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    /// ```
    pub async fn get_block_by_height(&self, height: u64) -> Result<Block> {
        self.get(&format!("block/height/{}", height)).await
    }

    /// ```rust
    /// use thegarii::types::Block;
    ///
    /// let client = thegarii::Client::default();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// { //  using indep_hash of block_height_100
    ///   let json = include_str!("../res/block_height_100.json");
    ///   let hash = "ngFDAB2KRhJgJRysuhpp1u65FjBf5WZk99_NyoMx8w6uP0IVjzb93EVkYxmcErdZ";
    ///   let block = rt.block_on(client.get_block_by_hash(hash)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    /// { //  using indep_hash of block_height_269512
    ///   let json = include_str!("../res/block_height_269512.json");
    ///   let hash = "5H-hJycMS_PnPOpobXu2CNobRlgqmw4yEMQSc5LeBfS7We63l8HjS-Ek3QaxK8ug";
    ///   let block = rt.block_on(client.get_block_by_hash(hash)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    /// { //  using indep_hash of block_height_422250
    ///   let json = include_str!("../res/block_height_422250.json");
    ///   let hash = "5VTARz7bwDO4GqviCSI9JXm8_JOtoQwF-QCZm0Gt2gVgwdzSY3brOtOD46bjMz09";
    ///   let block = rt.block_on(client.get_block_by_hash(hash)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    pub async fn get_block_by_hash(&self, hash: &str) -> Result<Block> {
        self.get(&format!("block/hash/{}", hash)).await
    }

    /// get latest block
    pub async fn get_current_block(&self) -> Result<Block> {
        self.get("current_block").await
    }

    /// get arweave transaction by id
    ///
    /// ```rust
    /// use thegarii::types::Transaction;
    ///
    /// let client = thegarii::Client::default();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// { // tx BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ - https://arweave.net/tx/BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ
    ///   let json = include_str!("../res/tx.json");
    ///   let tx = rt.block_on(client.get_tx_by_id("BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ")).unwrap();
    ///   assert_eq!(tx, serde_json::from_str::<Transaction>(&json).unwrap());
    /// }
    /// ```
    pub async fn get_tx_by_id(&self, id: &str) -> Result<Transaction> {
        self.get(&format!("tx/{}", id)).await
    }

    /// get arweave transaction data by id
    ///
    /// ```rust
    /// let client = thegarii::Client::default();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// { // tx BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ - https://arweave.net/tx/BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ/data
    ///   let json = include_str!("../res/data.json");
    ///   let tx = rt.block_on(client.get_tx_data_by_id("BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ")).unwrap();
    ///   assert_eq!(tx, json);
    /// }
    /// ```
    pub async fn get_tx_data_by_id(&self, id: &str) -> Result<String> {
        Ok(
            reqwest::get(&format!("{}tx/{}/data", self.next_endpoint(), id))
                .await?
                .text()
                .await?,
        )
    }

    /// get and parse firehose blocks by height
    ///
    /// ```rust
    /// let client = thegarii::Client::default();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// { // block height 269512 - https://arweave.net/block/height/269512
    ///   let firehose_block = rt.block_on(client.get_firehose_block_by_height(269512)).unwrap();
    ///
    ///   let mut block_without_txs = firehose_block.clone();
    ///   block_without_txs.txs = vec![];
    ///
    ///   assert_eq!(block_without_txs, rt.block_on(client.get_block_by_height(269512)).unwrap().into());
    ///   for (idx, tx) in firehose_block.txs.iter().map(|tx| tx.id.clone()).enumerate() {
    ///     assert_eq!(firehose_block.txs[idx], rt.block_on(client.get_tx_by_id(&tx)).unwrap());
    ///   }
    /// }
    /// ```
    pub async fn get_firehose_block_by_height(&self, height: u64) -> Result<FirehoseBlock> {
        let block = self.get_block_by_height(height).await?;
        let txs: Vec<Transaction> = join_all(block.txs.iter().map(|tx| self.get_tx_by_id(tx)))
            .await
            .into_iter()
            .collect::<Result<Vec<Transaction>>>()?;

        let mut firehose_block: FirehoseBlock = block.into();
        firehose_block.txs = txs;
        Ok(firehose_block)
    }
}
