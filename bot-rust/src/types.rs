use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[derive(Debug)]
pub struct BotConfig {
    // Solana
    pub rpc_url: String,
    pub rpc_ws_url: String,
    pub wallet_keypair: solana_sdk::signature::Keypair,

    // Trading Parameters
    pub min_liquidity_sol: f64,
    pub max_position_size_sol: f64,
    pub take_profit_multiplier: f64,
    pub stop_loss_percentage: f64,

    // API Endpoints
    pub pump_fun_api_url: String,
    pub raydium_amm_program: Pubkey,

    // Risk Management
    pub max_slippage_bps: u16,
    pub max_concurrent_positions: usize,
    pub position_timeout_seconds: u64,

    // Monitoring
    pub scan_interval_ms: u64,
    pub volume_threshold_sol: f64,
    pub holder_count_min: u32,

    // Strategy Selection
    pub strategy_type: StrategyType,
}

impl BotConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::from_filename("bot-rust/.env").ok();
        println!("[DEBUG] Environment variables loaded:");
        for (key, value) in std::env::vars() {
            println!("[DEBUG] {}={}", key, value);
        }

        let keypair_path = std::env::var("WALLET_KEYPAIR")?;
        let wallet_keypair = solana_sdk::signature::read_keypair_file(&keypair_path)
            .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;

        let raydium_program_str = std::env::var("RAYDIUM_AMM_PROGRAM")?;
        let raydium_amm_program = Pubkey::from_str(&raydium_program_str)?;

        Ok(Self {
            rpc_url: std::env::var("RPC_URL")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string()),
            rpc_ws_url: std::env::var("RPC_WS_URL")
                .unwrap_or_else(|_| "wss://api.mainnet-beta.solana.com".to_string()),
            wallet_keypair,

            min_liquidity_sol: std::env::var("MIN_LIQUIDITY_SOL")
                .unwrap_or_else(|_| "5.0".to_string())
                .parse()?,
            max_position_size_sol: std::env::var("MAX_POSITION_SIZE_SOL")
                .unwrap_or_else(|_| "1.0".to_string())
                .parse()?,
            take_profit_multiplier: std::env::var("TAKE_PROFIT_MULTIPLIER")
                .unwrap_or_else(|_| "2.0".to_string())
                .parse()?,
            stop_loss_percentage: std::env::var("STOP_LOSS_PERCENTAGE")
                .unwrap_or_else(|_| "0.5".to_string())
                .parse()?,

            pump_fun_api_url: std::env::var("PUMP_FUN_API_URL")
                .unwrap_or_else(|_| "https://frontend-api.pump.fun".to_string()),
            raydium_amm_program,

            max_slippage_bps: std::env::var("MAX_SLIPPAGE_BPS")
                .unwrap_or_else(|_| "500".to_string())
                .parse()?,
            max_concurrent_positions: std::env::var("MAX_CONCURRENT_POSITIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            position_timeout_seconds: std::env::var("POSITION_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()?,

            scan_interval_ms: std::env::var("SCAN_INTERVAL_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            volume_threshold_sol: std::env::var("VOLUME_THRESHOLD_SOL")
                .unwrap_or_else(|_| "10.0".to_string())
                .parse()?,
            holder_count_min: std::env::var("HOLDER_COUNT_MIN")
                .unwrap_or_else(|_| "50".to_string())
                .parse()?,

            strategy_type: std::env::var("STRATEGY_TYPE")
                .unwrap_or_else(|_| "conservative".to_string())
                .parse()?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenMetrics {
    pub mint: String,
    pub name: String,
    pub symbol: String,
    
    // Volume Metrics
    pub volume_5m: f64,
    pub volume_1h: f64,
    pub volume_24h: f64,
    
    // Price Metrics
    pub current_price: f64,
    pub price_change_5m: f64,
    pub price_change_1h: f64,
    
    // Liquidity
    pub liquidity_sol: f64,
    pub liquidity_usd: f64,
    
    // Social Metrics
    pub holder_count: u32,
    pub holder_concentration: f64, // Top 10 holders percentage
    pub unique_buyers_5m: u32,
    pub unique_sellers_5m: u32,
    
    // Market Cap
    pub market_cap: f64,
    pub fully_diluted_valuation: f64,
    
    // Bonding Curve
    pub bonding_curve_progress: f64, // 0-100%
    pub is_graduated: bool,
    
    // Timing
    pub created_at: i64,
    pub time_since_creation: u64, // seconds
    
    // Risk Factors
    pub buy_pressure: f64,
    pub sell_pressure: f64,
    pub volatility_score: f64,
}

#[derive(Debug, Clone)]
pub struct TradingSignal {
    pub token_mint: Pubkey,
    pub signal_type: SignalType,
    pub confidence: f64, // 0-1
    pub reasoning: Vec<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignalType {
    StrongBuy,
    Buy,
    Hold,
    Sell,
    StrongSell,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub token_mint: Pubkey,
    pub entry_price: f64,
    pub amount: u64,
    pub sol_invested: f64,
    pub entry_time: i64,
    pub take_profit_price: f64,
    pub stop_loss_price: f64,
    pub status: PositionStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PositionStatus {
    Open,
    Closed,
    Monitoring,
}

/// Strategy configuration for multi-strategy support
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StrategyType {
    Conservative,      // Original multi-factor strategy (default)
    UltraEarlySniper, // High risk, first 5 minutes, 10-100x targets
    MomentumScalper,  // Quick flips on explosive momentum
    GraduationAnticipator, // Pre-DEX positioning, lower risk
}

impl Default for StrategyType {
    fn default() -> Self {
        StrategyType::Conservative
    }
}

impl std::str::FromStr for StrategyType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "conservative" => Ok(StrategyType::Conservative),
            "ultra_early_sniper" | "ultra-early-sniper" | "early" => Ok(StrategyType::UltraEarlySniper),
            "momentum_scalper" | "momentum-scalper" | "momentum" => Ok(StrategyType::MomentumScalper),
            "graduation_anticipator" | "graduation-anticipator" | "graduation" => Ok(StrategyType::GraduationAnticipator),
            _ => Err(anyhow::anyhow!("Unknown strategy type: {}", s)),
        }
    }
}

/// Strategy-specific exit parameters
#[derive(Debug, Clone)]
pub struct StrategyExitParams {
    pub take_profit_multiplier: f64,
    pub stop_loss_percentage: f64,
    pub position_timeout_seconds: u64,
    pub use_trailing_stop: bool,
    pub trailing_activation_pct: f64,
    pub trailing_distance_pct: f64,
}
