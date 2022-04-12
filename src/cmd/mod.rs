// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! thegarii commands
use crate::{Env, EnvArguments, Result};
use structopt::StructOpt;

mod console;
mod get;
mod poll;

#[cfg(feature = "stream")]
mod stream;

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Get a block from database or fetch it
    Get(get::Get),
    /// Poll blocks and print to stdout
    Console(console::Console),
    /// Dry-run random polling with time estimating
    Poll(poll::Poll),
    /// Stream blocks from gRPC service
    #[cfg(feature = "stream")]
    Stream(stream::Stream),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "thegarii", author = "info@chainsafe.io")]
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

        // extract env
        let env = Env::from_args(opt.env)?;
        log::debug!("\n{:#?}", env);

        // process commmands
        match opt.command {
            Command::Get(get) => get.exec().await?,
            Command::Poll(poll) => poll.exec(env).await?,
            Command::Console(console) => console.exec(env).await?,
            #[cfg(feature = "stream")]
            Command::Stream(stream) => stream.exec().await?,
        }

        Ok(())
    }
}
