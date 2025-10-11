use std::env;
use solana_sdk::signature::{Keypair, read_keypair_file};
use anyhow::{Result, Context};

pub struct BotConfig {
    pub wallet_keypair: Keypair,
    pub rpc_url: String,
}

impl BotConfig {
    pub fn from_env() -> Result<Self> {
        let keypair_path = env::var("WALLET_KEYPAIR").context("WALLET_KEYPAIR env var not set")?;
        let wallet_keypair = read_keypair_file(&keypair_path)
            .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
        Ok(Self { wallet_keypair, rpc_url })
    }
}
