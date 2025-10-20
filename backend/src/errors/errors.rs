use solana_sdk::hash::ParseHashError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    FundingJobNotStarted(String),

    #[error("{0}")]
    InsufficientFunding(String),

    #[error("{0}")]
    RpcError(String),

    #[error("{0}")]
    Transport(#[from] reqwest::Error),

    #[error("{0}")]
    Parse(#[from] serde_json::Error),

    #[error("{0}")]
    HashParse(#[from] ParseHashError),

    #[error("{0}")]
    Bincode(#[from] bincode::Error),
}
