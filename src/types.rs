//! arweave types
use serde::{Deserialize, Serialize};

/// Arweave Block
///
/// - block height < 269510
/// - 269510 <= block height < 422250
/// - 422250 < block height
///
/// # NOTE
///
/// Arweave encoding their data with Base64URL, see
/// https://docs.arweave.org/developers/server/http-api#transaction-format,
/// here we simply parse `String` wrt the golang arweave client implementation
/// https://github.com/everFinance/goar/blob/main/types/block.go
///
/// ## TODO
///
/// Convert `String` to `Vec<u8>` for more effcient
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    // - block height < 269510
    pub nonce: String,
    pub previous_block: String,
    pub timestamp: u64,
    pub last_retarget: u64,
    // - `u64` if block height < 269510
    // - `String` if block height >= 269510
    pub diff: u64,
    pub height: u64,
    pub hash: String,
    pub indep_hash: String,
    pub txs: Vec<String>,
    pub wallet_list: String,
    pub reward_addr: String,
    pub tags: Vec<String>,
    pub reward_pool: u64,
    pub weave_size: u64,
    pub block_size: u64,

    // - 269510 <= block height < 422250
    pub cumulative_diff: String,
    pub hash_list_merkle: String,

    // - block height > 422250
    pub tx_root: String,
    pub tx_tree: Vec<String>,
    pub poa: Poa,
}

/// POA field of `Block`
#[derive(Debug, Serialize, Deserialize)]
pub struct Poa {
    pub option: String,
    pub tx_path: String,
    pub data_path: String,
    pub chunk: String,
}

/// Transaction type
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub format: usize,
    pub id: String,
    pub last_tx: String,
    pub owner: String,
    pub tags: Vec<String>,
    pub target: String,
    pub quantity: String,
    pub data_root: String,
    pub data: String,
    pub data_size: String,
    pub reward: String,
    pub signature: String,
}
