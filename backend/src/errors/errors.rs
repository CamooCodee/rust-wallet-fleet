use base64::DecodeError;
use solana_sdk::{hash::ParseHashError, signer::SignerError};
use thiserror::Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    FundingJobNotStarted(String),

    #[error("{0}")]
    InsufficientFunding(String),

    #[error("{0}")]
    InsufficientSol(String),

    #[error("{0}")]
    Database(#[from] tokio_rusqlite::Error),

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

    #[error("{0}")]
    Decoding(#[from] DecodeError),

    #[error("{0}")]
    JoinError(#[from] JoinError),

    #[error("{0}")]
    SignerError(#[from] SignerError),
}
