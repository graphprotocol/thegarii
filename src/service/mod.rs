// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Service {
    const NAME: &'static str;

    /// run service
    async fn run(&self) -> Result<()>;
}
