// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
#![allow(missing_docs)]
use crate::{
    result::{Error, Result},
    types::{self, FirehoseBlock, Poa, U256},
};
use core::convert::{TryFrom, TryInto};

pub mod sf {
    pub mod arweave {
        pub mod r#type {
            pub mod v1 {
                use crate::{types::U256, Error, Result};

                include!(concat!(env!("OUT_DIR"), "/sf.arweave.r#type.v1.rs"));

                impl TryFrom<String> for BigInt {
                    type Error = Error;

                    fn try_from(s: String) -> Result<Self> {
                        Ok(Self {
                            bytes: U256::from_dec_str(&s)?.to_be(),
                        })
                    }
                }

                impl TryFrom<Option<String>> for BigInt {
                    type Error = Error;

                    fn try_from(os: Option<String>) -> Result<Self> {
                        if let Some(s) = os {
                            s.try_into()
                        } else {
                            Ok(Self {
                                bytes: Default::default(),
                            })
                        }
                    }
                }
            }
        }
    }
}

pub use self::sf::arweave::r#type::v1::*;

/// decode string to bytes with base64url
fn bd(s: &str) -> Result<Vec<u8>> {
    // parse empty reward addr to vec![]
    if s == "unclaimed" {
        return Ok(vec![]);
    }

    base64_url::decode(s).map_err(Into::into)
}

impl TryFrom<FirehoseBlock> for Block {
    type Error = Error;

    fn try_from(block: FirehoseBlock) -> Result<Self> {
        Ok(Self {
            ver: 1,
            indep_hash: bd(&block.indep_hash)?,
            nonce: bd(&block.nonce)?,
            previous_block: bd(&block.previous_block)?,
            timestamp: block.timestamp,
            last_retarget: block.last_retarget,
            diff: Some(block.diff.try_into()?),
            height: block.height,
            hash: bd(&block.hash)?,
            tx_root: block.tx_root.unwrap_or_default().try_into()?,
            txs: block
                .txs
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            wallet_list: bd(&block.wallet_list)?,
            reward_addr: bd(&block.reward_addr)?,
            tags: block
                .tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            reward_pool: Some(BigInt {
                bytes: U256::from_dec_str(&block.reward_pool)?.to_be(),
            }),
            weave_size: Some(block.weave_size.try_into()?),
            block_size: Some(block.block_size.try_into()?),
            cumulative_diff: Some(block.cumulative_diff.try_into()?),
            hash_list_merkle: block.hash_list_merkle.unwrap_or_default().try_into()?,
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
            format: tx.format.unwrap_or_default(),
            id: bd(&tx.id)?,
            last_tx: bd(&tx.last_tx)?,
            owner: bd(&tx.owner)?,
            tags: tx
                .tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>>>()?,
            target: bd(&tx.target)?,
            quantity: Some(tx.quantity.try_into()?),
            data: bd(&tx.data)?,
            data_size: Some(tx.data_size.try_into()?),
            data_root: bd(&tx.data_root)?,
            signature: bd(&tx.signature)?,
            reward: Some(tx.reward.try_into()?),
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
