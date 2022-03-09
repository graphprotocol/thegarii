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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
    pub tags: Vec<Tag>,
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

impl Into<FirehoseBlock> for Block {
    fn into(self) -> FirehoseBlock {
        FirehoseBlock {
            ver: 1,
            indep_hash: self.indep_hash,
            nonce: self.nonce,
            previous_block: self.previous_block,
            timestamp: self.timestamp,
            last_retarget: self.last_retarget,
            diff: self.diff,
            height: self.height,
            hash: self.hash,
            tx_root: self.tx_root,
            txs: vec![],
            wallet_list: self.wallet_list,
            reward_addr: self.reward_addr,
            tags: self.tags,
            reward_pool: self.reward_pool,
            weave_size: self.weave_size,
            block_size: self.block_size,
            cumulative_diff: self.cumulative_diff,
            hash_list_merkle: self.hash_list_merkle,
            poa: self.poa,
        }
    }
}

/// POA field of `Block`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Poa {
    pub option: String,
    pub tx_path: String,
    pub data_path: String,
    pub chunk: String,
}

/// Transaction type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tag {
    name: String,
    value: String,
}

/// abstract firehose block which simply combines
/// `Block`, `Transaction` and `TransactionData`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirehoseBlock {
    /// Firehose block version (unrelated to Arweave block version)
    pub ver: u32,
    /// The block identifier
    pub indep_hash: String,
    /// The nonce chosen to solve the mining problem
    pub nonce: String,
    /// `indep_hash` of the previous block in the weave
    pub previous_block: String,
    /// POSIX time of block discovery
    pub timestamp: u64,
    /// POSIX time of the last difficulty retarget
    pub last_retarget: u64,
    /// Mining difficulty, the number `hash` must be greater than.
    pub diff: String,
    /// How many blocks have passed since the genesis block
    pub height: u64,
    /// Mining solution hash of the block, must satisfy the mining difficulty
    pub hash: String,
    /// Merkle root of the tree of Merkle roots of block's transactions' data.
    pub tx_root: Option<String>,
    /// Transactions contained within this block
    pub txs: Vec<Transaction>,
    /// The root hash of the Merkle Patricia Tree containing
    /// all wallet (account) balances and the identifiers
    /// of the last transactions posted by them, if any.
    pub wallet_list: String,
    /// Address of the account to receive the block rewards. Can also be unclaimed which is encoded as a null byte
    pub reward_addr: String,
    /// Tags that a block producer can add to a block
    pub tags: Vec<Tag>,
    /// Size of reward pool
    pub reward_pool: String,
    /// Size of the weave in bytes
    pub weave_size: String,
    /// Size of this block in bytes
    pub block_size: String,
    /// Required after the version 1.8 fork. Zero otherwise.
    /// The sum of the average number of hashes computed
    /// by the network to produce the past blocks including this one.
    pub cumulative_diff: Option<String>,
    // // The list of the block identifiers of the last
    // // STORE_BLOCKS_BEHIND_CURRENT blocks.
    // pub hash_list: Vec<String>,
    // Required after the version 1.8 fork. Null byte otherwise.
    // The Merkle root of the block index - the list of {`indep_hash`, `weave_size`, `tx_root`} triplets
    pub hash_list_merkle: Option<String>,
    // The proof of access, Used after v2.4 only, set as defaults otherwise
    pub poa: Option<Poa>,
}
