// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! App envorionments
use crate::{Error, Result};
use std::{env, fs, path::PathBuf};

const DB_PATH: &str = "DB_PATH";
const ENDPOINTS: &str = "ENDPOINTS";
const POLLING_INTERVAL: &str = "POLLING_INTERVAL";
const POLLING_TIMEOUT: &str = "POLLING_TIMEOUT";
const POLLING_RETRY_TIMES: &str = "POLLING_RETRY_TIMES";

/// environments
#[derive(Debug)]
pub struct Env {
    /// storage db path
    pub db_path: PathBuf,
    /// client endpoints
    pub endpoints: Vec<String>,
    /// how many blocks polling at one time
    pub polling_interval: u64,
    /// timeout of polling service
    pub polling_timeout: u64,
    /// retry times when failed on http requests
    pub polling_retry_times: u8,
}

impl Env {
    /// get $DB_PATH from env or use `$DATA_DIR/thegarii/thegarii.db`
    pub fn db_path() -> Result<PathBuf> {
        let path = match env::var(DB_PATH).map(PathBuf::from) {
            Ok(p) => p,
            Err(e) => {
                log::error!("path not exists: {:?}", e);
                dirs::data_dir()
                    .map(|p| p.join("thegarii/thegarii.db"))
                    .ok_or(Error::NoDataDirectory)?
            }
        };

        fs::create_dir_all(&path)?;
        Ok(path)
    }

    /// get $ENDPOINTS from env or use `"https://arweave.net"`
    pub fn endpoints() -> Result<Vec<String>> {
        let raw_endpoints = match env::var(ENDPOINTS) {
            Ok(endpoints) => endpoints,
            Err(_) => "https://arweave.net".to_string(),
        };

        Ok(raw_endpoints.split(',').map(|e| e.to_string()).collect())
    }

    /// get $POLLING_INTERVAL from env or use `50`
    pub fn polling_interval() -> Result<u64> {
        Ok(match env::var(POLLING_INTERVAL) {
            Ok(interval) => interval.parse()?,
            Err(_) => 50,
        })
    }

    /// get $POLLING_TIMEOUT from env or use `10_000`
    pub fn polling_timeout() -> Result<u64> {
        Ok(match env::var(POLLING_TIMEOUT) {
            Ok(timeout) => timeout.parse()?,
            Err(_) => 10_000,
        })
    }

    /// get $POLLING_RETRY_TIMES from env or use `3`
    pub fn polling_retry_times() -> Result<u8> {
        Ok(match env::var(POLLING_RETRY_TIMES) {
            Ok(times) => times.parse()?,
            Err(_) => 3,
        })
    }

    /// new environments
    pub fn new() -> Result<Self> {
        Ok(Self {
            db_path: Self::db_path()?,
            endpoints: Self::endpoints()?,
            polling_interval: Self::polling_interval()?,
            polling_timeout: Self::polling_timeout()?,
            polling_retry_times: Self::polling_retry_times()?,
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
