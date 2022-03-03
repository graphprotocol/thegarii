//! arweave client
use crate::{
    result::Result,
    types::{Block, Transaction},
};
use serde::de::DeserializeOwned;

/// Arweave client
pub struct Client {
    // TODO
    //
    // use `endpoints` when supporting multiple endpoints
    endpoint: &'static str,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            endpoint: "https://arweave.net/",
        }
    }
}

impl Client {
    pub fn new(endpoint: &'static str) -> Self {
        Self { endpoint }
    }

    /// http get request with base url
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let mut url = self.endpoint.to_string();
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
    /// { // block height 100
    ///   let json = include_str!("../res/block_height_100.json");
    ///   let block = rt.block_on(client.get_block_by_height(100)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    ///
    /// { // block height 269512
    ///   let json = include_str!("../res/block_height_269512.json");
    ///   let block = rt.block_on(client.get_block_by_height(269512)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    ///
    /// { // block height 422250
    ///   let json = include_str!("../res/block_height_422250.json");
    ///   let block = rt.block_on(client.get_block_by_height(422250)).unwrap();
    ///   assert_eq!(block, serde_json::from_str::<Block>(&json).unwrap());
    /// }
    /// ```
    pub async fn get_block_by_height(&self, height: u64) -> Result<Block> {
        self.get(&format!("block/height/{}", height)).await
    }

    /// get arweave transaction by id
    pub async fn get_tx_by_id(&self, id: &str) -> Result<Transaction> {
        self.get(&format!("tx/{}", id)).await
    }

    /// get arweave transaction data by id
    pub async fn get_tx_data_by_id(&self, id: &str) -> Result<String> {
        self.get(&format!("tx/{}/data", id)).await
    }
}
