use thiserror::Error;

pub type Result<T> = std::result::Result<T, WalletError>;

#[derive(Error, Debug)]
pub enum WalletError {
    #[error("Solana RPC error: {0}")]
    RpcError(String),

    #[error("Invalid keypair format: {0}")]
    InvalidKeypair(String),

    #[error("Insufficient balance for transaction")]
    InsufficientBalance,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}
