//! the garii project
//!
//! this library fetches blocks from Arweave and generates firehose blocks for the Graph
pub mod client;
mod encoding;
pub mod result;
pub mod types;

pub use self::{
    client::Client,
    result::{Error, Result},
};
