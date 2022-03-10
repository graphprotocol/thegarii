// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{env, types::FirehoseBlock, Error, Result};
use rocksdb::{IteratorMode, DB};

/// firehose block storage
pub struct Storage(DB);

impl Storage {
    /// new storage
    pub fn new() -> Result<Self> {
        Ok(Self(DB::open_default(env::db_path()?)?))
    }

    /// count blocks
    ///
    /// see https://github.com/facebook/rocksdb/blob/08809f5e6cd9cc4bc3958dd4d59457ae78c76660/include/rocksdb/db.h#L654-L689
    pub fn count(&self) -> Result<u64> {
        Ok(self
            .0
            .property_int_value("rocksdb.estimate-num-keys")?
            .unwrap_or(0))
    }

    /// get the last block
    pub fn last(&self) -> Result<FirehoseBlock> {
        Ok(self
            .0
            .iterator(IteratorMode::End)
            .next()
            .ok_or(Error::NoBlockExists)?)
    }

    /// get block
    pub fn get(&self, height: u64) -> Result<FirehoseBlock> {
        let block_bytes = self
            .0
            .get(height.to_le_bytes())?
            .ok_or(Error::BlockNotFound(height))?;

        Ok(bincode::deserialize(&block_bytes)?)
    }

    /// set block
    pub fn put(&self, block: FirehoseBlock) -> Result<()> {
        let height = block.height;
        let bytes = bincode::serialize(&block)?;

        self.0.put(height.to_le_bytes(), &bytes)?;
        Ok(())
    }

    /// new read-only storage
    pub fn read_only() -> Result<Self> {
        Ok(Self(DB::open_for_read_only(
            &Default::default(),
            env::db_path()?,
            false,
        )?))
    }
}
