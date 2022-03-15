// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! polling service

use crate::{service::Service, Client, Env, Result, Storage};
use async_trait::async_trait;
use std::sync::atomic::AtomicU64;

/// polling service
pub struct Polling {
    client: Client,
    storage: Storage,
    ptr: AtomicU64,
}

#[async_trait]
impl Service for Polling {
    const NAME: &'static str = "polling";

    /// new polling service
    fn new(env: &Env) -> Result<Self> {
        let client = Client::new(env.endpoints.clone())?;
        let storage = Storage::new(&env.db_path)?;
        let ptr = AtomicU64::new(storage.last()?.height);

        Ok(Self {
            client,
            storage,
            ptr,
        })
    }

    async fn run(&self) -> Result<()> {
        todo!()
    }
}
