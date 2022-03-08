// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! arweave types
use crate::encoding::number_or_string;
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    // - block height < 269510
    pub nonce: String,
    pub previous_block: String,
    pub timestamp: u64,
    pub last_retarget: u64,
    // - `u64` if block height < 269510
    // - `String` if block height >= 269510
    #[serde(deserialize_with = "number_or_string")]
    pub diff: String,
    pub height: u64,
    pub hash: String,
    pub indep_hash: String,
    pub txs: Vec<String>,
    pub wallet_list: String,
    pub reward_addr: String,
    pub tags: Vec<String>,
    #[serde(deserialize_with = "number_or_string")]
    pub reward_pool: String,
    #[serde(deserialize_with = "number_or_string")]
    pub weave_size: String,
    #[serde(deserialize_with = "number_or_string")]
    pub block_size: String,
    // - 269510 <= block height < 422250
    pub cumulative_diff: Option<String>,
    pub hash_list_merkle: Option<String>,
    // - block height > 422250
    pub tx_root: Option<String>,
    pub tx_tree: Option<Vec<String>>,
    pub poa: Option<Poa>,
}

/// POA field of `Block`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Poa {
    pub option: String,
    pub tx_path: String,
    pub data_path: String,
    pub chunk: String,
}

/// Transaction type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub format: usize,
    pub id: String,
    pub last_tx: String,
    pub owner: String,
    pub tags: Vec<Tag>,
    pub target: String,
    pub quantity: String,
    pub data_root: String,
    pub data: String,
    pub data_size: String,
    pub reward: String,
    pub signature: String,
}

/// Transaction type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tag {
    name: String,
    value: String,
}

/// abstract firehose block which simply combines
/// `Block`, `Transaction` and `TransactionData`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirehoseBlock {
    pub block: Block,
    pub txs: Vec<Transaction>,
    pub txs_data: Vec<String>,
}
