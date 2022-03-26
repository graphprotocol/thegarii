// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! start service
use crate::{
    service::{Grpc, Polling, Service, Shared, Tracking},
    Client, Env, Result, Storage,
};
use futures::{future::join_all, lock::Mutex};
use std::sync::Arc;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Start {}

impl Start {
    /// start services
    pub async fn exec(&self, env: Env) -> Result<()> {
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
