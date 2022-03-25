// Copyright 2021 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

//! App envorionments
use crate::{Error, Result};
use std::{env, fs, net::SocketAddr, path::PathBuf};
use structopt::StructOpt;

const BLOCK_TIME: &str = "BLOCK_TIME";
const DEFAULT_BLOCK_TIME: u64 = 10_000;
const CHECKING_INTERVAL: &str = "CHECKING_INTERVAL";
const DEFAULT_CHECKING_INTERVAL: u64 = 3_000;
const DB_PATH: &str = "DB_PATH";
const DEFAULT_DB_PATH: &str = "thegarii/thegarii.db";
const ENDPOINTS: &str = "ENDPOINTS";
const DEFAULT_ENDPOINTS: &str = "https://arweave.net";
const GRPC_ADDR: &str = "GRPC_ADDR";
const DEFAULT_GRPC_ADDR: &str = "[::1]:50051";
const BATCH_BLOCKS: &str = "BATCH_BLOCKS";
const DEFAULT_BATCH_BLOCKS: u16 = 50;
const RETRY: &str = "RETRY";
const DEFAULT_RETRY: u8 = 10;
const CONFIRMS: &str = "CONFIRMS";
const DEFAULT_CONFIRMS: u64 = 20;
const TIMEOUT: &str = "TIMEOUT";
const DEFAULT_TIMEOUT: u64 = 120_000;

/// env arguments for CLI
#[derive(Debug, StructOpt)]
pub struct EnvArguments {
    /// time cost for producing a new block in arweave
    #[structopt(short, long)]
    pub block_time: Option<u64>,
    /// inverval for checking missing blocks
    #[structopt(short, long)]
    pub checking_interval: Option<u64>,
    /// storage db path
    #[structopt(short = "D", long)]
    pub db_path: Option<PathBuf>,
    /// client endpoints
    #[structopt(short, long)]
    pub endpoints: Vec<String>,
    /// grpc address
    #[structopt(short, long)]
    pub grpc_addr: Option<String>,
    /// how many blocks polling at one time
    #[structopt(short = "B", long)]
    pub batch_blocks: Option<u16>,
    /// safe blocks against to reorg in polling
    #[structopt(short, long)]
    pub confirms: Option<u64>,
    /// timeout of http requests
    #[structopt(short, long)]
    pub timeout: Option<u64>,
    /// retry times when failed on http requests
    #[structopt(short, long)]
    pub retry: Option<u8>,
}

/// environments
#[derive(Debug)]
pub struct Env {
    /// time cost for producing a new block in arweave
    pub block_time: u64,
    /// inverval for checking missed blocks
    pub checking_interval: u64,
    /// storage db path
    pub db_path: PathBuf,
    /// client endpoints
    pub endpoints: Vec<String>,
    /// grpc address
    pub grpc_addr: SocketAddr,
    /// how many blocks polling at one time
    pub batch_blocks: u16,
    /// safe blocks against to reorg in polling
    pub confirms: u64,
    /// timeout of http requests
    pub timeout: u64,
    /// retry times when failed on http requests
    pub retry: u8,
}

impl Env {
    /// get $BLOCK_TIME from env or use DEFAULT_BLOCK_TIME
    pub fn block_time() -> Result<u64> {
        Ok(match env::var(BLOCK_TIME) {
            Ok(time) => time.parse()?,
            Err(_) => DEFAULT_BLOCK_TIME,
        })
    }

    /// get $CHECKING_INTERVAL from env or use $DEFAULT_CHECKING_INTERVAL
    pub fn checking_interval() -> Result<u64> {
        Ok(match env::var(CHECKING_INTERVAL) {
            Ok(interval) => interval.parse()?,
            Err(_) => DEFAULT_CHECKING_INTERVAL,
        })
    }

    /// get $DB_PATH from env or use `$DATA_DIR/$DEFAULT_DB_PATH`
    pub fn db_path() -> Result<PathBuf> {
        let path = match env::var(DB_PATH).map(PathBuf::from) {
            Ok(p) => p,
            Err(_) => dirs::data_dir()
                .map(|p| p.join(DEFAULT_DB_PATH))
                .ok_or(Error::NoDataDirectory)?,
        };

        fs::create_dir_all(&path)?;
        Ok(path)
    }

    /// get $ENDPOINTS from env or use $DEFAULT_ENDPOINTS
    pub fn endpoints() -> Result<Vec<String>> {
        let raw_endpoints = match env::var(ENDPOINTS) {
            Ok(endpoints) => endpoints,
            Err(_) => DEFAULT_ENDPOINTS.to_string(),
        };

        Ok(raw_endpoints.split(',').map(|e| e.to_string()).collect())
    }

    /// get $GRPC_ADDR from env or use $DEFAULT_GRPC_ADDR
    pub fn grpc_addr() -> Result<SocketAddr> {
        let addr = match env::var(GRPC_ADDR) {
            Ok(addr) => addr.parse()?,
            Err(_) => DEFAULT_GRPC_ADDR.parse()?,
        };

        Ok(addr)
    }

    /// get $BATCH_BLOCKS from env or use $DEFAULT_BATCH_BLOCKS
    pub fn batch_blocks() -> Result<u16> {
        Ok(match env::var(BATCH_BLOCKS) {
            Ok(blocks) => blocks.parse()?,
            Err(_) => DEFAULT_BATCH_BLOCKS,
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
            block_time: Self::block_time()?,
            checking_interval: Self::checking_interval()?,
            db_path: Self::db_path()?,
            endpoints: Self::endpoints()?,
            grpc_addr: Self::grpc_addr()?,
            batch_blocks: Self::batch_blocks()?,
            retry: Self::retry()?,
            confirms: Self::confirms()?,
            timeout: Self::timeout()?,
        })
    }

    /// derive env from arguments
    pub fn from_args(args: EnvArguments) -> Result<Self> {
        Ok(Self {
            block_time: args.block_time.unwrap_or(Self::block_time()?),
            checking_interval: args.checking_interval.unwrap_or(Self::checking_interval()?),
            db_path: args.db_path.unwrap_or(Self::db_path()?),
            endpoints: if args.endpoints.is_empty() {
                Self::endpoints()?
            } else {
                args.endpoints
            },
            grpc_addr: if let Some(addr) = args.grpc_addr {
                addr.parse()?
            } else {
                Self::grpc_addr()?
            },
            batch_blocks: args.batch_blocks.unwrap_or(Self::batch_blocks()?),
            retry: args.retry.unwrap_or(Self::retry()?),
            confirms: args.confirms.unwrap_or(Self::confirms()?),
            timeout: args.timeout.unwrap_or(Self::timeout()?),
        })
    }

    /// set block time
    pub fn with_block_time(&mut self, block_time: u64) -> &mut Self {
        self.block_time = block_time;
        self
    }

    /// set checking interval
    pub fn with_checking_interval(&mut self, checking_interval: u64) -> &mut Self {
        self.checking_interval = checking_interval;
        self
    }

    /// set db path
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
