pub use anyhow::Result;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Bytes could not be converted to Fr")]
    BadFrBytes,
    #[error("invalid input size")]
    InvalidInputSize,
}