// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! arweave client
use crate::{
    result::{Error, Result},
    types::{Block, FirehoseBlock, Transaction},
    Env,
};
use futures::future::join_all;
use rand::Rng;
use reqwest::{Client as ReqwestClient, ClientBuilder, StatusCode};
use serde::de::DeserializeOwned;
use std::time::Duration;

/// Arweave client
pub struct Client {
    client: ReqwestClient,
    /// arweave endpoints
    pub endpoints: Vec<String>,
    retry: u8,
}

impl Client {
    /// get next endpoint
    fn next_endpoint(&self, already_used_endpoints: &[String]) -> String {
        let mut endpoints = self.endpoints.clone();

        // if all endpoints are already used, return random endpoint
        if endpoints.len() == already_used_endpoints.len() {
            return already_used_endpoints
                [rand::thread_rng().gen_range(0..already_used_endpoints.len())]
            .to_string();
        }

        // round robin endpoints that are not already used
        let mut endpoint = endpoints.remove(0);
        while already_used_endpoints.contains(&endpoint) {
            endpoint = endpoints.remove(0);
        }
        endpoint
    }

    /// new arweave client
    pub fn new(endpoints: Vec<String>, timeout: Duration, retry: u8) -> Result<Self> {
        if endpoints.is_empty() {
            return Err(Error::EmptyEndpoints);
        }

        let client = ClientBuilder::new().gzip(true).timeout(timeout).build()?;

        Ok(Self {
            client,
            endpoints,
            retry,
        })
    }

    /// new client from environments
    pub fn from_env() -> Result<Self> {
        let env = Env::new()?;
        let client = ClientBuilder::new()
            .gzip(true)
            .timeout(Duration::from_millis(env.timeout))
            .build()?;

        Ok(Self {
            client,
            endpoints: env.endpoints,
            retry: env.retry,
        })
    }

    /// http get request with base url
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let mut retried = 0;
        let mut ms_between_retries = 10000;
        let mut already_used_endpoints: Vec<String> = Vec::new();

        loop {
            let endpoint = self.next_endpoint(&already_used_endpoints);
            let url = format!("{}/{}", endpoint, path);

            match self.client.get(&url).send().await {
                Ok(r) => match r.status() {
                    StatusCode::OK => match r.json::<T>().await {
                        Ok(r) => return Ok(r),
                        Err(e) => return Err(e.into()),
                    },
                    _ => {
                        // If there's still endpoints not used yet, retry now with different endpoint
                        if self.endpoints.len() != already_used_endpoints.len() {
                            already_used_endpoints.push(endpoint.clone());
                            continue;
                        }

                        // If all endpoints are used, wait then retry with random endpoint
                        if retried < self.retry {
                            let duration = Duration::from_millis(ms_between_retries);
                            tokio::time::sleep(duration).await;
                            retried += 1;
                            ms_between_retries *= 2;
                            log::warn!(
                                "{{ \"endpoint\": \"{}\", \"path\": \"{}\", \"status\": \"{}\", \"retry_in\": {}, \"attempts\": {}, \"attempts_left\": {} }}",
                                endpoint,
                                path,
                                r.status(),
                                Duration::as_secs(&duration),
                                retried,
                                self.retry - retried
                            );
                            continue;
                        }

                        // If all endpoints used and all retries done, return error
                        return Err(Error::RetriesReached);
                    }
                },
                Err(e) => {
                    return Err(e.into());
                }
            }
        }
    }

    /// get arweave block by height
    ///
    /// ```rust
    /// use thegarii::types::Block;
    ///
    /// let client = thegarii::Client::from_env().unwrap();
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
    /// let client = thegarii::Client::from_env().unwrap();
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
    /// let client = thegarii::Client::from_env().unwrap();
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
    /// let client = thegarii::Client::from_env().unwrap();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// { // tx BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ - https://arweave.net/tx/BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ/data
    ///   let json = include_str!("../res/data.json");
    ///   let tx = rt.block_on(client.get_tx_data_by_id("BNttzDav3jHVnNiV7nYbQv-GY0HQ-4XXsdkE5K9ylHQ")).unwrap();
    ///   assert_eq!(tx, json);
    /// }
    /// ```
    ///
    /// # NOTE
    ///
    /// timeout and retry don't work for this reqeust since we're not using
    /// this api in the polling service.
    pub async fn get_tx_data_by_id(&self, id: &str) -> Result<String> {
        Ok(self
            .client
            .get(&format!(
                "{}/tx/{}/data",
                self.next_endpoint(&self.endpoints),
                id
            ))
            .send()
            .await?
            .text()
            .await?)
    }

    /// get and parse firehose blocks by height
    ///
    /// ```rust
    /// let client = thegarii::Client::from_env().unwrap();
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
        log::info!("resolving firehose block {}", height);

        let block = self.get_block_by_height(height).await?;
        let txs: Vec<Transaction> = join_all(block.txs.iter().map(|tx| self.get_tx_by_id(tx)))
            .await
            .into_iter()
            .collect::<Result<Vec<Transaction>>>()?;

        let mut firehose_block: FirehoseBlock = block.into();
        firehose_block.txs = txs;
        Ok(firehose_block)
    }

    /// poll blocks from iterator
    ///
    /// ```rust
    /// let client = thegarii::Client::from_env().unwrap();
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    ///
    /// rt.block_on(client.poll(269512..269515)).unwrap();
    /// ```
    pub async fn poll<Blocks>(&self, blocks: Blocks) -> Result<Vec<FirehoseBlock>>
    where
        Blocks: Iterator<Item = u64> + Sized,
    {
        join_all(blocks.map(|block| self.get_firehose_block_by_height(block)))
            .await
            .into_iter()
            .collect::<Result<Vec<FirehoseBlock>>>()
    }
}
