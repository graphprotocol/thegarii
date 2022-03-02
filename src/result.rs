//! the graii results

/// the graii errors
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// result type
pub type Result<T> = std::result::Result<T, Error>;
