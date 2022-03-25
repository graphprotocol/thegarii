// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! thegarii commands
use crate::{Env, EnvArguments, Result};
use structopt::StructOpt;

mod backup;
mod get;
mod restore;
mod start;
mod stream;
mod syncing;

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Backup blocks to path
    Backup(backup::Backup),
    /// Get a block from database or fetch it
    Get(get::Get),
    /// Restore blocks from path
    Restore(restore::Restore),
    /// Start thegarii service
    Start(start::Start),
    /// Stream blocks from gRPC service
    Stream(stream::Stream),
    /// Show the syncing status
    Syncing(syncing::Syncing),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "thegaril", author = "info@chainsafe.io")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(flatten)]
    pub env: EnvArguments,

    /// commands
    #[structopt(subcommand)]
    pub command: Command,
}

impl Opt {
    /// exec commands
    pub async fn exec() -> Result<()> {
        let opt = Opt::from_args();

        if opt.debug {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("thegarii"))
                .init();
        } else {
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
                .init();
        }

        let env = Env::from_args(opt.env)?;
        match opt.command {
            Command::Backup(backup) => backup.exec(env).await?,
            Command::Get(get) => get.exec(env).await?,
            Command::Restore(restore) => restore.exec(env).await?,
            Command::Start(start) => start.exec(env).await?,
            Command::Stream(stream) => stream.exec(env).await?,
            Command::Syncing(syncing) => syncing.exec(env).await?,
        }

        Ok(())
    }
}
