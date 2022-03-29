#[cfg(feature = "console")]
mod console;
#[cfg(feature = "storage")]
mod storage;

#[cfg(feature = "console")]
pub use console::Polling;

#[cfg(feature = "storage")]
pub use storage::Polling;
