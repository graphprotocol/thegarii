//! the graii results

/// the graii errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

/// result type
pub type Result<T> = std::result::Result<T, Error>;
