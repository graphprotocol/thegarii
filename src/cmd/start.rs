// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! start service
use crate::{
    cmd::CommandT,
    service::{Grpc, Polling, Service, Shared, Tracking},
    Client, Env, Result, Storage,
};
use async_trait::async_trait;
use futures::{future::join_all, lock::Mutex};
use std::{sync::Arc, time::Duration};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Start {}

#[async_trait]
impl CommandT for Start {
    /// start services
    async fn exec(&self, env: Env) -> Result<()> {
        let client = Client::new(
            env.endpoints.clone(),
            Duration::from_millis(env.timeout),
            env.retry,
        )?;

        let latest = client.get_current_block().await?.height.max(env.confirms) - env.confirms;
        let storage = Storage::new(&env.db_path)?;
        let shared = Shared {
            client: Arc::new(client),
            env: Arc::new(env),
            latest: Arc::new(Mutex::new(latest)),
            storage,
        };

        join_all(vec![
            Tracking::new(shared.clone())?.start(),
            Polling::new(shared.clone())?.start(),
            Grpc::new(shared)?.start(),
        ])
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}
