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

impl Client {
    /// http get
    async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let mut url = self.endpoint.to_string();
        url.push_str(path);

        Ok(reqwest::get(url).await?.json().await?)
    }

    /// get block by height
    pub async fn get_block_by_height(&self, height: u64) -> Result<Block> {
        self.get(&format!("block/height/{}", height)).await
    }

    /// get transaction by id
    pub async fn get_tx_by_id(&self, id: &str) -> Result<Transaction> {
        self.get(&format!("tx/{}", id)).await
    }

    /// get transaction data by id
    pub async fn get_tx_data_by_id(&self, id: &str) -> Result<String> {
        self.get(&format!("tx/{}/data", id)).await
    }
}
