//! firehose client
use crate::result::Result;
use async_trait::async_trait;

/// custom client
#[async_trait]
trait Client {
    const Endpoint: &'static str;

    type Block;
    type Transaction;
    type Data;

    fn get_block_by_height(height: u64) -> Result<Self::Block>;
    fn get_tx_by_hash(hash: [u8; 32]) -> Result<Self::Transaction>;
    fn get_tx_data_by_hash(hash: [u8; 32]) -> Result<Self::Data>;
}
