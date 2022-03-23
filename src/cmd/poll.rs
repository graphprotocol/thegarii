// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only
use crate::{Client, Env, Error, Result};
use rand::Rng;
use std::time::Instant;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Poll {
    /// how many blocks gonna poll
    #[structopt(short, long, default_value = "100")]
    pub blocks: u64,
    /// poll blocks from
    #[structopt(short, long, default_value = "0")]
    pub start: u64,
    /// poll blocks to, ( 0 will be reset to the latest block height )
    #[structopt(short, long, default_value = "0")]
    pub end: u64,
}

impl Poll {
    fn time(mut secs: u64) -> String {
        let days = secs / 86_400;
        secs -= 86_400 * days;
        let hours = secs / 3_600;
        secs -= 36_00 * hours;
        let minutes = secs / 60;
        secs -= 60 * minutes;

        let mut elapsed = String::new();

        let pad = |elapsed: &mut String| {
            if !elapsed.is_empty() {
                elapsed.push_str(" ");
            }
        };

        if days != 0 {
            elapsed.push_str(&format!("{} days", days));
        }
        if hours != 0 {
            pad(&mut elapsed);
            elapsed.push_str(&format!("{} hours", hours));
        }
        if minutes != 0 {
            pad(&mut elapsed);
            elapsed.push_str(&format!("{} minutes", days));
        }
        if secs != 0 {
            pad(&mut elapsed);
            elapsed.push_str(&format!("{} seconds", secs));
        }

        elapsed
    }

    pub async fn exec(&self, env: Env) -> Result<()> {
        let client = Client::from_env()?;
        let current = client.get_current_block().await?.height;

        // reset set end if it's zero
        let mut end = self.end;
        if end == 0 {
            end = current
        }

        if self.start >= end {
            return Err(Error::InvalidRange);
        }

        // prepare blocks
        let mut rng = rand::thread_rng();
        let mut blocks = (0..self.blocks)
            .map(|_| rng.gen_range(self.start..=end))
            .collect::<Vec<u64>>();

        // start polling
        let now = Instant::now();
        while !blocks.is_empty() {
            let mut _blocks = blocks.clone();
            if blocks.len() > env.polling_batch_blocks as usize {
                blocks = _blocks.split_off(env.polling_batch_blocks as usize);
            } else {
                blocks.drain(..);
            }

            _blocks.sort();
            log::info!("polling blocks {:?}...", _blocks);
            client.poll(_blocks.into_iter()).await?;
        }

        // log result
        log::info!("\n{:#?}", env);
        log::info!("time cost: {}", Self::time(now.elapsed().as_secs()));
        log::info!(
            "estimate fully sync will cost: {}",
            Self::time(now.elapsed().as_secs() * current / self.blocks)
        );
        Ok(())
    }
}
