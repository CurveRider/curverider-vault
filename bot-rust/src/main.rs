use solana_sdk::signature::Signer;
mod error;
mod types;
mod config;
mod analyzer;
mod scanner;
mod trader;

use error::Result;
use types::{BotConfig, SignalType};
use analyzer::{TradingStrategy, create_strategy};
use scanner::PumpFunScanner;
use trader::Trader;

use tracing::{info, warn, error, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting Curverider Vault Bot");
    info!("⚡ High-Performance Rust Trading Bot for pump.fun");
    info!("═══════════════════════════════════════════════");

    // Load configuration
    let config = BotConfig::from_env()?;
    info!("✅ Configuration loaded");
    info!("📊 Wallet: {}", config.wallet_keypair.pubkey());
    info!("💰 Max position size: {} SOL", config.max_position_size_sol);

    // Initialize strategy
    let strategy = create_strategy(config.strategy_type);
    let exit_params = strategy.get_exit_params();

    info!("🎲 Strategy: {}", strategy.name());
    info!("🎯 Take profit: {}x", exit_params.take_profit_multiplier);
    info!("🛑 Stop loss: {:.0}%", exit_params.stop_loss_percentage * 100.0);
    info!("⏱️  Position timeout: {}s", exit_params.position_timeout_seconds);
    if exit_params.use_trailing_stop {
        info!("📉 Trailing stop: Activate at +{:.0}%, trail by {:.0}%",
            exit_params.trailing_activation_pct * 100.0,
            exit_params.trailing_distance_pct * 100.0);
    }

    // Initialize components
    let scanner = PumpFunScanner::new(&config);
    let mut trader = Trader::new(&config);

    info!("✅ Bot initialized successfully");
    info!("🔍 Starting main trading loop...\n");

    // Main trading loop
    let mut iteration = 0;
    loop {
        iteration += 1;

        match run_trading_cycle(&scanner, strategy.as_ref(), &mut trader, &config).await {
            Ok(_) => {
                debug!("Iteration {} completed successfully", iteration);
            }
            Err(e) => {
                error!("Error in trading cycle {}: {}", iteration, e);
            }
        }

        // Monitor existing positions
        if let Err(e) = trader.monitor_positions().await {
            error!("Error monitoring positions: {}", e);
        }

        // Display status
        if iteration % 10 == 0 {
            display_status(&trader, &config);
        }

        // Wait before next cycle
        time::sleep(Duration::from_millis(config.scan_interval_ms)).await;
    }
}

/// Run a single trading cycle
async fn run_trading_cycle(
    scanner: &PumpFunScanner,
    strategy: &dyn TradingStrategy,
    trader: &mut Trader,
    config: &BotConfig,
) -> Result<()> {
    // Skip if at position limit
    if trader.position_count() >= config.max_concurrent_positions {
        debug!("At position limit ({}/{}), skipping scan", 
            trader.position_count(), config.max_concurrent_positions);
        return Ok(());
    }

    // Scan for tokens
    let token_mints = scanner.scan_trending_tokens(20).await?;

    if token_mints.is_empty() {
        debug!("No tokens found in scan");
        return Ok(());
    }

    // Analyze each token
    for mint in token_mints {
        // Get metrics
        let metrics = match scanner.get_token_metrics(&mint).await {
            Ok(m) => m,
            Err(e) => {
                warn!("Failed to get metrics for {}: {}", mint, e);
                continue;
            }
        };

        // Analyze using selected strategy
        let signal = match strategy.analyze(&metrics) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to analyze {}: {}", mint, e);
                continue;
            }
        };

        // Log signal
        info!(
            "📊 {} ({}): {:?} - {:.1}% confidence",
            metrics.symbol,
            metrics.mint,
            signal.signal_type,
            signal.confidence * 100.0
        );

        if !signal.reasoning.is_empty() {
            for reason in &signal.reasoning {
                debug!("   └─ {}", reason);
            }
        }

        // Execute trade if strong buy signal
        if matches!(signal.signal_type, SignalType::StrongBuy) 
            && signal.confidence >= 0.75 {
            
            info!("🎯 STRONG BUY SIGNAL DETECTED!");
            info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            info!("Token: {} ({})", metrics.symbol, metrics.name);
            info!("Confidence: {:.1}%", signal.confidence * 100.0);
            info!("Reasons:");
            for reason in &signal.reasoning {
                info!("  • {}", reason);
            }
            info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

            // Execute buy
            match trader.buy_token(&signal.token_mint, config.max_position_size_sol).await {
                Ok(position) => {
                    info!("✅ Position opened successfully!");
                    info!("📍 Entry: ${:.6}", position.entry_price);
                    info!("🎯 Take Profit: ${:.6}", position.take_profit_price);
                    info!("🛑 Stop Loss: ${:.6}\n", position.stop_loss_price);
                }
                Err(e) => {
                    error!("❌ Failed to open position: {}\n", e);
                }
            }
        } else if matches!(signal.signal_type, SignalType::Buy) 
            && signal.confidence >= 0.65 {
            
            info!("📈 Buy signal detected (moderate confidence)");
            // Could implement smaller position sizing for lower confidence
        }

        // Small delay between token analyses
        time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

/// Display bot status
fn display_status(trader: &Trader, config: &BotConfig) {
    let active_positions = trader.get_active_positions();
    
    info!("═══════════════════════════════════════════════");
    info!("📊 BOT STATUS");
    info!("═══════════════════════════════════════════════");
    info!("🔓 Active Positions: {}/{}", 
        active_positions.len(), 
        config.max_concurrent_positions
    );

    if !active_positions.is_empty() {
        info!("Positions:");
        for (i, pos) in active_positions.iter().enumerate() {
            let time_held = chrono::Utc::now().timestamp() - pos.entry_time;
            info!(
                "  {}. {} - Entry: ${:.6}, Held: {}s",
                i + 1,
                pos.token_mint,
                pos.entry_price,
                time_held
            );
        }
    }

    info!("═══════════════════════════════════════════════\n");
}
