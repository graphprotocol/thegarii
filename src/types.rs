//! arweave types

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
pub struct Block {
    // - block height < 269510
    nonce: String,
    previous_block: String,
    timestamp: u64,
    last_retarget: u64,
    // - `u64` if block height < 269510
    // - `String` if block height >= 269510
    diff: u64,
    height: u64,
    hash: String,
    indep_hash: String,
    txs: Vec<String>,
    wallet_list: String,
    reward_addr: String,
    tags: Vec<String>,
    reward_pool: u64,
    weave_size: u64,
    block_size: u64,

    // - 269510 <= block height < 422250
    cumulative_diff: String,
    hash_list_merkle: String,

    // - block height > 422250
    tx_root: String,
    tx_tree: Vec<String>,
    poa: Poa,
}

/// POA field of `Block`
pub struct Poa {
    option: String,
    tx_path: String,
    data_path: String,
    chunk: String,
}

/// Transaction type
pub struct Transaction {
    format: usize,
    id: String,
    last_tx: String,
    owner: String,
    tags: Vec<String>,
    target: String,
    quantity: String,
    data_root: String,
    data: String,
    data_Size: String,
    reward: String,
    signature: String,
}
