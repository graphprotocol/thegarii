// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
//! App envorionments
use crate::{Error, Result};
use std::{env, fs, path::PathBuf};

/// get DB_PATH from env or use `$DATA_DIR/thegarii/thegarii.db`
pub fn db_path() -> Result<PathBuf> {
    let path = match env::var("DB_PATH").map(PathBuf::from) {
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
