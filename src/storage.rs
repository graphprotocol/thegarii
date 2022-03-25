// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

#![allow(unused)]
use crate::{env, types::FirehoseBlock, Error, Result};
use futures::lock::Mutex;
use rocksdb::{IteratorMode, WriteBatch, DB};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

const LATEST_BLOCK_STROAGE_KEY: [u8; 13] = *b"_LATEST_BLOCK";

/// firehose block storage
#[derive(Clone)]
pub struct Storage {
    pub read: Arc<DB>,
    pub write: Option<Arc<Mutex<DB>>>,
}

impl Storage {
    /// new storage
    pub fn new(db_path: &dyn AsRef<Path>) -> Result<Self> {
        Ok(Self {
            write: Some(Arc::new(Mutex::new(DB::open_default(db_path)?))),
            read: Self::load_read_db(db_path),
        })
    }

    /// new read-only storage
    pub fn read_only(db_path: &dyn AsRef<Path>) -> Result<Self> {
        Ok(Self {
            read: Self::load_read_db(db_path),
            write: None,
        })
    }

    fn load_read_db(db_path: &dyn AsRef<Path>) -> Arc<DB> {
        Arc::new(
            DB::open_for_read_only(&Default::default(), db_path, false)
                .expect("cannot load read only db, make sure db exists"),
        )
    }

    //
    // read APIs
    //

    /// map storage keys
    pub fn map_keys<T>(&self, map: fn(&[u8], &[u8]) -> T) -> Vec<T> {
        self.read
            .iterator(IteratorMode::Start)
            .map(|(k, v)| map(&k, &v))
            .collect::<Vec<T>>()
    }

    /// get missing block numbers
    pub fn missing<Blocks>(&self, keys: Blocks) -> Vec<u64>
    where
        Blocks: Iterator<Item = u64> + Sized,
    {
        keys.into_iter()
            .filter(|key| self.get(*key).is_err())
            .collect()
    }

    /// count blocks
    ///
    /// see https://github.com/facebook/rocksdb/blob/08809f5e6cd9cc4bc3958dd4d59457ae78c76660/include/rocksdb/db.h#L654-L689
    pub fn count(&self) -> Result<u64> {
        Ok(self
            .read
            .property_int_value("rocksdb.estimate-num-keys")?
            .unwrap_or(0))
    }

    /// get the last block
    pub fn last(&self) -> Result<FirehoseBlock> {
        let (_, value) = self
            .read
            .iterator(IteratorMode::End)
            .next()
            .ok_or(Error::NoBlockExists)?;

        Ok(bincode::deserialize(&value)?)
    }

    /// get block
    pub fn get(&self, height: u64) -> Result<FirehoseBlock> {
        let block_bytes = self
            .read
            .get(height.to_le_bytes())?
            .ok_or(Error::BlockNotFound(height))?;

        Ok(bincode::deserialize(&block_bytes)?)
    }

    //
    // write APIs
    //

    /// query lastest block number from db
    pub fn latest(&self) -> Result<u64> {
        let mut buf = [0; 8];
        let mut latest = self
            .read
            .get(&LATEST_BLOCK_STROAGE_KEY)?
            .ok_or(Error::NoLatestBlockRecord)?;
        buf.copy_from_slice(&mut latest);

        Ok(u64::from_le_bytes(buf))
    }

    /// get mut latest block
    pub async fn latest_mut(&self, maybe_latest: u64) -> Result<u64> {
        let mut should_update = false;
        let next = if let Ok(latest) = self.latest() {
            if maybe_latest > latest {
                should_update = true;
                maybe_latest
            } else {
                latest
            }
        } else {
            maybe_latest
        };

        if should_update {
            let db = self.write.as_ref().ok_or(Error::ReadOnlyDatabase)?;
            db.lock()
                .await
                .put(&LATEST_BLOCK_STROAGE_KEY, next.to_le_bytes());
        }

        Ok(next)
    }

    /// set block
    pub async fn put(&self, block: FirehoseBlock) -> Result<()> {
        let db = self.write.as_ref().ok_or(Error::ReadOnlyDatabase)?;

        db.lock()
            .await
            .put(block.height.to_le_bytes(), &bincode::serialize(&block)?)?;

        Ok(())
    }

    /// batch write blocks into db
    pub async fn write(&self, blocks: Vec<FirehoseBlock>) -> Result<()> {
        let db = self.write.as_ref().ok_or(Error::ReadOnlyDatabase)?;

        let mut batch = WriteBatch::default();
        for b in blocks {
            batch.put(b.height.to_le_bytes(), bincode::serialize(&b)?);
        }

        db.lock().await.write(batch)?;
        Ok(())
    }

    /// flush data to disk
    pub async fn flush(&self) -> Result<()> {
        let db = self.write.as_ref().ok_or(Error::ReadOnlyDatabase)?;

        db.lock().await.flush()?;

        Ok(())
    }
}
