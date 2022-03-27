// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(missing_docs)]
use crate::{
    result::{Error, Result},
    types::{self, FirehoseBlock, Poa},
};
use core::convert::{TryFrom, TryInto};
use prost_types::Timestamp;
use std::time::{Duration, SystemTime};

pub mod cs {
    pub mod arweave {
        pub mod codec {
            pub mod v1 {
                tonic::include_proto!("cs.arweave.codec.v1");
            }
        }
    }
}

pub mod sf {
    pub mod firehose {
        pub mod v1 {
            tonic::include_proto!("sf.firehose.v1");
        }
    }
}

pub use self::{cs::arweave::codec::v1::*, sf::firehose::v1::*};

/// decode string to bytes with base64url
fn bd(s: &str) -> Result<Vec<u8>> {
    base64_url::decode(s).map_err(Into::into)
}

fn convert_timestamp(t: u64) -> Option<Timestamp> {
    SystemTime::UNIX_EPOCH
        .checked_add(Duration::from_secs(t))
        .map(Into::into)
}

impl TryFrom<FirehoseBlock> for Block {
    type Error = Error;

    fn try_from(block: FirehoseBlock) -> Result<Self> {
        Ok(Self {
            ver: 1,
            indep_hash: bd(&block.indep_hash)?,
            nonce: bd(&block.nonce)?,
            previous_block: bd(&block.previous_block)?,
            timestamp: convert_timestamp(block.timestamp),
            last_retarget: convert_timestamp(block.last_retarget),
            diff: block.diff,
            height: block.height,
            hash: bd(&block.hash)?,
            tx_root: block.tx_root.and_then(|r| bd(&r).ok()),
            txs: block
                .txs
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            wallet_list: bd(&block.wallet_list)?,
            reward_addr: block.reward_addr,
            tags: block
                .tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            reward_pool: block.reward_pool,
            weave_size: block.weave_size,
            block_size: block.block_size,
            cumulative_diff: block.cumulative_diff,
            // # TODO
            //
            // handle this field which is not a member of arweave block
            hash_list: vec![],
            hash_list_merkle: block.hash_list_merkle.and_then(|h| bd(&h).ok()),
            poa: block.poa.and_then(|p| p.try_into().ok()),
        })
    }
}

impl TryFrom<Poa> for ProofOfAccess {
    type Error = Error;

    fn try_from(poa: Poa) -> Result<Self> {
        Ok(Self {
            option: poa.option,
            tx_path: bd(&poa.tx_path)?,
            data_path: bd(&poa.data_path)?,
            chunk: bd(&poa.chunk)?,
        })
    }
}

impl TryFrom<types::Transaction> for Transaction {
    type Error = Error;

    fn try_from(tx: types::Transaction) -> Result<Self> {
        Ok(Self {
            format: tx.format,
            id: tx.id,
            last_tx: bd(&tx.last_tx)?,
            owner: bd(&tx.owner)?,
            tags: tx
                .tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            target: bd(&tx.target)?,
            quantity: tx.quantity,
            data: bd(&tx.data)?,
            data_size: tx.data_size,
            data_root: tx.data_root,
            signature: bd(&tx.signature)?,
            reward: tx.reward,
        })
    }
}

impl TryFrom<types::Tag> for Tag {
    type Error = Error;

    fn try_from(tag: types::Tag) -> Result<Self> {
        Ok(Self {
            name: bd(&tag.name)?,
            value: bd(&tag.value)?,
        })
    }
}
