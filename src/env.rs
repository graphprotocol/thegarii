// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! App envorionments
use crate::{Error, Result};
use std::{env, fs, path::PathBuf};

const DB_PATH: &str = "DB_PATH";
const ENDPOINTS: &str = "ENDPOINTS";

/// environments
pub struct Env {
    pub db_path: PathBuf,
    pub endpoints: Vec<String>,
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

    /// new environments
    pub fn new() -> Result<Self> {
        Ok(Self {
            db_path: Self::db_path()?,
            endpoints: Self::endpoints()?,
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
