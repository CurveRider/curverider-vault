use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Solana client error: {0}")]
    SolanaClient(#[from] solana_client::client_error::ClientError),

    #[error("Anchor error: {0}")]
    Anchor(#[from] anchor_client::ClientError),

    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Insufficient funds: required {required} SOL, available {available} SOL")]
    InsufficientFunds { required: f64, available: f64 },

    #[error("Token not found: {0}")]
    TokenNotFound(String),

    #[error("High slippage detected: {0}%")]
    HighSlippage(f64),

    #[error("Position limit reached: {0}/{1}")]
    PositionLimitReached(usize, usize),

    #[error("Trade timeout")]
    TradeTimeout,

    #[error("Invalid keypair")]
    InvalidKeypair,

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Analysis error: {0}")]
    Analysis(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, BotError>;
