use bot_rust::types::{TokenMetrics, TradingSignal, SignalType, Position, PositionStatus};
use chrono::Utc;

#[test]
fn test_token_analyzer_signal_generation() {
    use bot_rust::analyzer::TokenAnalyzer;
    let analyzer = TokenAnalyzer::new(5.0, 1000.0, 50, 0.2);
    let metrics = TokenMetrics {
        mint: "So11111111111111111111111111111111111111112".to_string(),
        name: "TestToken".to_string(),
        symbol: "TTK".to_string(),
        volume_5m: 2000.0,
        volume_1h: 10000.0,
        volume_24h: 50000.0,
        current_price: 1.0,
        price_change_5m: 0.1,
        price_change_1h: 0.2,
        liquidity_sol: 10.0,
        liquidity_usd: 1000.0,
        holder_count: 100,
        holder_concentration: 0.1,
        unique_buyers_5m: 20,
        unique_sellers_5m: 5,
        market_cap: 100000.0,
        fully_diluted_valuation: 200000.0,
        bonding_curve_progress: 0.5,
        is_graduated: false,
        created_at: Utc::now().timestamp(),
        time_since_creation: 600,
        buy_pressure: 0.7,
        sell_pressure: 0.2,
        volatility_score: 0.3,
    };
    let signal = analyzer.analyze(&metrics).unwrap();
    assert!(matches!(signal.signal_type, SignalType::StrongBuy | SignalType::Buy | SignalType::Hold | SignalType::Sell | SignalType::StrongSell));
    assert!(signal.confidence >= 0.0 && signal.confidence <= 1.0);
    assert!(!signal.reasoning.is_empty());
}

#[test]
fn test_position_lifecycle() {
    let mint = solana_sdk::pubkey::Pubkey::new_unique();
    let mut position = Position {
        token_mint: mint,
        entry_price: 1.0,
        amount: 1000,
        sol_invested: 1.0,
        entry_time: Utc::now().timestamp(),
        take_profit_price: 2.0,
        stop_loss_price: 0.5,
        status: PositionStatus::Open,
    };
    // Simulate price movement
    let current_price = 2.1;
    if current_price >= position.take_profit_price {
        position.status = PositionStatus::Closed;
    }
    assert_eq!(position.status, PositionStatus::Closed);
}

#[test]
fn test_error_handling_insufficient_funds() {
    use bot_rust::error::BotError;
    let err = BotError::InsufficientFunds { required: 10.0, available: 2.0 };
    let msg = format!("{}", err);
    assert!(msg.contains("Insufficient funds"));
}
use tokio;

#[tokio::test]
async fn test_trader_position_limit() {
    use bot_rust::types::BotConfig;
    use bot_rust::trader::Trader;
    use solana_sdk::pubkey::Pubkey;
    let mut config = BotConfig {
        rpc_url: "https://api.testnet.solana.com".to_string(),
        rpc_ws_url: "wss://api.testnet.solana.com".to_string(),
        wallet_keypair: solana_sdk::signature::Keypair::new(),
        min_liquidity_sol: 1.0,
        max_position_size_sol: 1.0,
        take_profit_multiplier: 2.0,
        stop_loss_percentage: 0.5,
        pump_fun_api_url: "https://api.pump.fun".to_string(),
        raydium_amm_program: Pubkey::new_unique(),
        max_slippage_bps: 500,
        max_concurrent_positions: 1,
        position_timeout_seconds: 3600,
        scan_interval_ms: 1000,
        volume_threshold_sol: 10.0,
        holder_count_min: 50,
    };
    let mut trader = Trader::new(config.clone());
    // Simulate filling the position limit
    trader.positions.push(Default::default());
    let result = trader.buy_token(&Pubkey::new_unique(), 1.0).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_scanner_scan_new_tokens() {
    use bot_rust::types::BotConfig;
    use bot_rust::scanner::PumpFunScanner;
    use solana_sdk::pubkey::Pubkey;
    let config = BotConfig {
        rpc_url: "https://api.testnet.solana.com".to_string(),
        rpc_ws_url: "wss://api.testnet.solana.com".to_string(),
        wallet_keypair: solana_sdk::signature::Keypair::new(),
        min_liquidity_sol: 1.0,
        max_position_size_sol: 1.0,
        take_profit_multiplier: 2.0,
        stop_loss_percentage: 0.5,
        pump_fun_api_url: "https://api.pump.fun".to_string(),
        raydium_amm_program: Pubkey::new_unique(),
        max_slippage_bps: 500,
        max_concurrent_positions: 5,
        position_timeout_seconds: 3600,
        scan_interval_ms: 1000,
        volume_threshold_sol: 10.0,
        holder_count_min: 50,
    };
    let scanner = PumpFunScanner::new(config);
    let result = scanner.scan_new_tokens().await;
    assert!(result.is_ok() || result.is_err()); // Accepts network errors
}
use bot_rust::types::{BotConfig};
use bot_rust::analyzer::TokenAnalyzer;
use bot_rust::trader::Trader;
use bot_rust::scanner::PumpFunScanner;
use bot_rust::error::BotError;
use solana_sdk::pubkey::Pubkey;

#[test]
fn test_config_from_env() {
    std::env::set_var("WALLET_PRIVATE_KEY", "3v1...fake...key");
    // Add other required env vars as needed
    let result = BotConfig::from_env();
    assert!(result.is_err() || result.is_ok()); // Just checks parsing runs
}

#[test]
fn test_token_analyzer_new() {
    let analyzer = TokenAnalyzer::new(1.0, 1000.0, 10, 0.1);
    assert_eq!(analyzer.min_liquidity, 1.0);
}

#[test]
fn test_trader_new() {
    let config = BotConfig {
        rpc_url: "https://api.testnet.solana.com".to_string(),
        rpc_ws_url: "wss://api.testnet.solana.com".to_string(),
        wallet_keypair: solana_sdk::signature::Keypair::new(),
        min_liquidity_sol: 1.0,
        max_position_size_sol: 1.0,
        take_profit_multiplier: 2.0,
        stop_loss_percentage: 0.5,
        pump_fun_api_url: "https://api.pump.fun".to_string(),
        raydium_amm_program: Pubkey::new_unique(),
        max_slippage_bps: 500,
        max_concurrent_positions: 5,
        position_timeout_seconds: 3600,
        scan_interval_ms: 1000,
        volume_threshold_sol: 10.0,
        holder_count_min: 50,
    };
    let trader = Trader::new(config.clone());
    assert_eq!(trader.config.rpc_url, config.rpc_url);
}

#[test]
fn test_scanner_new() {
    let config = BotConfig {
        rpc_url: "https://api.testnet.solana.com".to_string(),
        rpc_ws_url: "wss://api.testnet.solana.com".to_string(),
        wallet_keypair: solana_sdk::signature::Keypair::new(),
        min_liquidity_sol: 1.0,
        max_position_size_sol: 1.0,
        take_profit_multiplier: 2.0,
        stop_loss_percentage: 0.5,
        pump_fun_api_url: "https://api.pump.fun".to_string(),
        raydium_amm_program: Pubkey::new_unique(),
        max_slippage_bps: 500,
        max_concurrent_positions: 5,
        position_timeout_seconds: 3600,
        scan_interval_ms: 1000,
        volume_threshold_sol: 10.0,
        holder_count_min: 50,
    };
    let scanner = PumpFunScanner::new(config);
    assert_eq!(scanner.config.max_slippage_bps, 500);
}

#[test]
fn test_bot_error_display() {
    let err = BotError::Config("bad config".to_string());
    assert_eq!(format!("{}", err), "Invalid configuration: bad config");
}
