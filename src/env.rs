// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! App envorionments
use crate::{Error, Result};
use std::{env, fs, path::PathBuf};

const BLOCK_TIME: &str = "BLOCK_TIME";
const DEFAULT_BLOCK_TIME: u64 = 10_000;
const CHECKING_INTERVAL: &str = "CHECKING_INTERVAL";
const DEFAULT_CHECKING_INTERVAL: u64 = 7_200_000;
const DB_PATH: &str = "DB_PATH";
const DEFAULT_DB_PATH: &str = "thegarii/thegarii.db";
const ENDPOINTS: &str = "ENDPOINTS";
const DEFAULT_ENDPOINTS: &str = "https://arweave.net";
const POLLING_BATCH_BLOCKS: &str = "POLLING_BATCH_BLOCKS";
const DEFAULT_POLLING_BATCH_BLOCKS: u16 = 50;
const POLLING_RETRY_TIMES: &str = "POLLING_RETRY_TIMES";
const DEFAULT_POLLING_RETRY_TIMES: u8 = 10;
const POLLING_SAFE_BLOCKS: &str = "POLLING_SAFE_BLOCKS";
const DEFAULT_POLLING_SAFE_BLOCKS: u64 = 20;
const POLLING_TIMEOUT: &str = "POLLING_TIMEOUT";
const DEFAULT_POLLING_TIMEOUT: u64 = 120_000;

/// environments
#[derive(Debug)]
pub struct Env {
    /// time cost for producing a new block in arweave
    pub block_time: u64,
    /// inverval for checking missed blocks
    pub checking_interval: u64,
    /// storage db path
    pub db_path: PathBuf,
    /// client endpoints
    pub endpoints: Vec<String>,
    /// how many blocks polling at one time
    pub polling_batch_blocks: u16,
    /// safe blocks against to reorg in polling
    pub polling_safe_blocks: u64,
    /// timeout of polling service
    pub polling_timeout: u64,
    /// retry times when failed on http requests
    pub polling_retry_times: u8,
}

impl Env {
    /// get $BLOCK_TIME from env or use DEFAULT_BLOCK_TIME
    pub fn block_time() -> Result<u64> {
        Ok(match env::var(BLOCK_TIME) {
            Ok(time) => time.parse()?,
            Err(_) => DEFAULT_BLOCK_TIME,
        })
    }

    /// get $CHECKING_INTERVAL from env or use $DEFAULT_CHECKING_INTERVAL
    pub fn checking_interval() -> Result<u64> {
        Ok(match env::var(CHECKING_INTERVAL) {
            Ok(interval) => interval.parse()?,
            Err(_) => DEFAULT_CHECKING_INTERVAL,
        })
    }

    /// get $DB_PATH from env or use `$DATA_DIR/$DEFAULT_DB_PATH`
    pub fn db_path() -> Result<PathBuf> {
        let path = match env::var(DB_PATH).map(PathBuf::from) {
            Ok(p) => p,
            Err(_) => dirs::data_dir()
                .map(|p| p.join(DEFAULT_DB_PATH))
                .ok_or(Error::NoDataDirectory)?,
        };

        fs::create_dir_all(&path)?;
        Ok(path)
    }

    /// get $ENDPOINTS from env or use $DEFAULT_ENDPOINTS
    pub fn endpoints() -> Result<Vec<String>> {
        let raw_endpoints = match env::var(ENDPOINTS) {
            Ok(endpoints) => endpoints,
            Err(_) => DEFAULT_ENDPOINTS.to_string(),
        };

        Ok(raw_endpoints.split(',').map(|e| e.to_string()).collect())
    }

    /// get $POLLING_BATCH_BLOCKS from env or use $DEFAULT_POLLING_BATCH_BLOCKS
    pub fn polling_batch_blocks() -> Result<u16> {
        Ok(match env::var(POLLING_BATCH_BLOCKS) {
            Ok(blocks) => blocks.parse()?,
            Err(_) => DEFAULT_POLLING_BATCH_BLOCKS,
        })
    }

    /// get $POLLING_RETRY_TIMES from env or use $DEFAULT_POLLING_RETRY_TIMES
    pub fn polling_retry_times() -> Result<u8> {
        Ok(match env::var(POLLING_RETRY_TIMES) {
            Ok(times) => times.parse()?,
            Err(_) => DEFAULT_POLLING_RETRY_TIMES,
        })
    }

    /// get $POLLING_SAFE_BLOCKS from env or use $DEFAULT_POLLING_SAFE_BLOCKS
    pub fn polling_safe_blocks() -> Result<u64> {
        Ok(match env::var(POLLING_SAFE_BLOCKS) {
            Ok(interval) => interval.parse()?,
            Err(_) => DEFAULT_POLLING_SAFE_BLOCKS,
        })
    }

    /// get $POLLING_TIMEOUT from env or use $DEFAULT_POLLING_TIMEOUT
    pub fn polling_timeout() -> Result<u64> {
        Ok(match env::var(POLLING_TIMEOUT) {
            Ok(timeout) => timeout.parse()?,
            Err(_) => DEFAULT_POLLING_TIMEOUT,
        })
    }

    /// new environments
    pub fn new() -> Result<Self> {
        Ok(Self {
            block_time: Self::block_time()?,
            checking_interval: Self::checking_interval()?,
            db_path: Self::db_path()?,
            endpoints: Self::endpoints()?,
            polling_batch_blocks: Self::polling_batch_blocks()?,
            polling_retry_times: Self::polling_retry_times()?,
            polling_safe_blocks: Self::polling_safe_blocks()?,
            polling_timeout: Self::polling_timeout()?,
        })
    }

    /// set db path
    pub fn with_db_path(&mut self, db_path: PathBuf) -> &mut Self {
        self.db_path = db_path;
        self
    }

    /// set endpoints
    pub fn with_endpoints(&mut self, endpoints: Vec<String>) -> &mut Self {
        self.endpoints = endpoints;
        self
    }
}
