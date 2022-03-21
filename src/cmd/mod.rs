// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! thegarii commands
use crate::{Env, EnvArguments, Result};
use structopt::StructOpt;

mod start;

#[derive(StructOpt, Debug)]
pub enum Command {
    /// start thegarii service
    Start(start::Start),
    // #[structopt(subcommand)]
    // start: start::Start,
    // },
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

        let env = Env::from_args(opt.env)?;
        match opt.command {
            Command::Start(start) => start.exec(env).await?,
        }

        Ok(())
    }
}
