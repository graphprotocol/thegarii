// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! start service
use crate::{
    service::{Grpc, Polling, Service, Shared},
    Env, Result, Storage,
};
use futures::{future::join_all, join, lock::Mutex};
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Start {}

impl Start {
    /// start services
    pub async fn exec(&self, env: Env) -> Result<()> {
        let storage = Storage::new(&env.db_path)?;
        let shared = Shared {
            env: Arc::new(env),
            latest: Arc::new(Mutex::new(0)),
            storage,
        };

        let (polling, grpc) = join!(Polling::new(shared.clone()), Grpc::new(shared));

        join_all(vec![polling?.start(), grpc?.start()])
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}
