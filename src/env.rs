// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! App envorionments
use crate::Result;
use std::{env, path::PathBuf};
use structopt::StructOpt;

const BLOCK_TIME: &str = "BLOCK_TIME";
const DEFAULT_BLOCK_TIME: u64 = 20_000;
const ENDPOINTS: &str = "ENDPOINTS";
const DEFAULT_ENDPOINTS: &str = "https://arweave.net";
const BATCH_BLOCKS: &str = "BATCH_BLOCKS";
const DEFAULT_BATCH_BLOCKS: u16 = 50;
const RETRY: &str = "RETRY";
const DEFAULT_RETRY: u8 = 10;
const CONFIRMS: &str = "CONFIRMS";
const DEFAULT_CONFIRMS: u64 = 20;
const TIMEOUT: &str = "TIMEOUT";
const DEFAULT_TIMEOUT: u64 = 120_000;
const PTR_FILE: &str = "PTR_FILE";
const DEFAULT_PTR_FILE: &str = "./arweave.ptr";

/// env arguments for CLI
#[derive(Debug, StructOpt)]
pub struct EnvArguments {
    /// how many blocks polling at one time
    #[structopt(short = "B", long, default_value = "20")]
    pub batch_blocks: u16,
    /// time cost for producing a new block in arweave
    #[structopt(short, long, default_value = "60000")]
    pub block_time: u64,
    /// safe blocks against to reorg in polling
    #[structopt(short, long, default_value = "20")]
    pub confirms: u64,
    /// client endpoints
    #[structopt(short, long, default_value = "https://arweave.net/")]
    pub endpoints: Vec<String>,
    /// block pointer path
    #[structopt(short, long, default_value = "./arweave.ptr")]
    pub ptr_file: PathBuf,
    /// retry times when failed on http requests
    #[structopt(short, long, default_value = "10")]
    pub retry: u8,
    /// timeout of http requests
    #[structopt(short, long, default_value = "120000")]
    pub timeout: u64,
}

/// environments
#[derive(Clone, Debug)]
pub struct Env {
    /// how many blocks polling at one time
    pub batch_blocks: u16,
    /// time cost for producing a new block in arweave
    pub block_time: u64,
    /// safe blocks against to reorg in polling
    pub confirms: u64,
    /// client endpoints
    pub endpoints: Vec<String>,
    /// block pointer path
    pub ptr_file: PathBuf,
    /// retry times when failed on http requests
    pub retry: u8,
    /// timeout of http requests
    pub timeout: u64,
}

impl Env {
    /// get $BLOCK_TIME from env or use DEFAULT_BLOCK_TIME
    pub fn block_time() -> Result<u64> {
        Ok(match env::var(BLOCK_TIME) {
            Ok(time) => time.parse()?,
            Err(_) => DEFAULT_BLOCK_TIME,
        })
    }

    /// get $ENDPOINTS from env or use $DEFAULT_ENDPOINTS
    pub fn endpoints() -> Result<Vec<String>> {
        let raw_endpoints = match env::var(ENDPOINTS) {
            Ok(endpoints) => endpoints,
            Err(_) => DEFAULT_ENDPOINTS.to_string(),
        };

        Ok(raw_endpoints.split(',').map(|e| e.to_string()).collect())
    }

    /// get $BATCH_BLOCKS from env or use $DEFAULT_BATCH_BLOCKS
    pub fn batch_blocks() -> Result<u16> {
        Ok(match env::var(BATCH_BLOCKS) {
            Ok(blocks) => blocks.parse()?,
            Err(_) => DEFAULT_BATCH_BLOCKS,
        })
    }

    /// get $PTR_FILE from env or use DEFAULT_PTR_FILE
    pub fn ptr_file() -> Result<PathBuf> {
        Ok(match env::var(PTR_FILE) {
            Ok(path) => path.into(),
            Err(_) => DEFAULT_PTR_FILE.into(),
        })
    }

    /// get $RETRY from env or use $DEFAULT_RETRY
    pub fn retry() -> Result<u8> {
        Ok(match env::var(RETRY) {
            Ok(times) => times.parse()?,
            Err(_) => DEFAULT_RETRY,
        })
    }

    /// get $CONFIRMS from env or use $DEFAULT_CONFIRMS
    pub fn confirms() -> Result<u64> {
        Ok(match env::var(CONFIRMS) {
            Ok(interval) => interval.parse()?,
            Err(_) => DEFAULT_CONFIRMS,
        })
    }

    /// get $TIMEOUT from env or use $DEFAULT_TIMEOUT
    pub fn timeout() -> Result<u64> {
        Ok(match env::var(TIMEOUT) {
            Ok(timeout) => timeout.parse()?,
            Err(_) => DEFAULT_TIMEOUT,
        })
    }

    /// new environments
    pub fn new() -> Result<Self> {
        Ok(Self {
            batch_blocks: Self::batch_blocks()?,
            block_time: Self::block_time()?,
            confirms: Self::confirms()?,
            endpoints: Self::endpoints()?,
            ptr_file: Self::ptr_file()?,
            retry: Self::retry()?,
            timeout: Self::timeout()?,
        })
    }

    /// derive env from arguments
    pub fn from_args(args: EnvArguments) -> Result<Self> {
        Ok(Self {
            batch_blocks: args.batch_blocks,
            block_time: args.block_time,
            confirms: args.confirms,
            endpoints: if args.endpoints.is_empty() {
                Self::endpoints()?
            } else {
                args.endpoints
            },
            ptr_file: args.ptr_file,
            retry: args.retry,
            timeout: args.timeout,
        })
    }

    /// set block time
    pub fn with_block_time(&mut self, block_time: u64) -> &mut Self {
        self.block_time = block_time;
        self
    }

    /// set db path
    #[cfg(feature = "full")]
    pub fn with_db_path(&mut self, db_path: PathBuf) -> &mut Self {
        self.db_path = db_path;
        self
    }

    /// set endpoints
    pub fn with_endpoints(&mut self, endpoints: Vec<String>) -> &mut Self {
        self.endpoints = endpoints;
        self
    }

    /// set polling batch blocks
    pub fn with_batch_blocks(&mut self, batch_blocks: u16) -> &mut Self {
        self.batch_blocks = batch_blocks;
        self
    }

    /// set polling safe blocks
    pub fn with_confirms(&mut self, confirms: u64) -> &mut Self {
        self.confirms = confirms;
        self
    }

    /// set polling timeout
    pub fn with_timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// set polling retry times
    pub fn with_retry(&mut self, retry: u8) -> &mut Self {
        self.retry = retry;
        self
    }
}
