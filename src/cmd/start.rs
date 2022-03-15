//! start service
use crate::{
    service::{Checking, Polling, Service},
    Env, Result,
};
use futures::{future::join_all, join};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Start {
    /// database path
    #[structopt(short, long)]
    pub db_path: Option<PathBuf>,
}

impl Start {
    /// start services
    pub async fn exec(&self, mut env: Env) -> Result<()> {
        if let Some(db_path) = &self.db_path {
            env.with_db_path(db_path.into());
        }

        let (polling, checking) = join!(Polling::new(&env), Checking::new(&env));

        join_all(vec![polling?.start(), checking?.start()])
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        Ok(())
    }
}
