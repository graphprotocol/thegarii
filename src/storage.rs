// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{env, types::FirehoseBlock, Error, Result};
use rocksdb::DB;

/// firehose block storage
pub struct Storage(DB);

impl Storage {
    /// new storage
    pub fn new() -> Result<Self> {
        Ok(Self(DB::open_default(env::db_path()?)?))
    }

    /// new read-only storage
    pub fn read_only() -> Result<Self> {
        Ok(Self(DB::open_for_read_only(
            &Default::default(),
            env::db_path()?,
            false,
        )?))
    }

    /// set block
    pub fn put(&self, block: FirehoseBlock) -> Result<()> {
        let height = block.height;
        let bytes = bincode::serialize(&block)?;

        self.0.put(height.to_le_bytes(), &bytes)?;
        Ok(())
    }

    /// get block
    pub fn get(&self, block: u64) -> Result<Vec<u8>> {
        Ok(self
            .0
            .get(block.to_le_bytes())?
            .ok_or(Error::BlockNotFound(block))?)
    }

    // pub fn count(&self) -> usize {
    //     self.0.cou
    // }
}
