use crate::types::{TokenMetrics, BotConfig};
use crate::error::{Result, BotError};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug, error};
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct PumpFunToken {
    mint: String,
    name: String,
    symbol: String,
    uri: String,
    #[serde(default)]
    usd_market_cap: f64,
    #[serde(default)]
    total_supply: u64,
    #[serde(default)]
    bonding_curve: Option<String>,
    #[serde(default)]
    associated_bonding_curve: Option<String>,
    #[serde(default)]
    creator: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PumpFunResponse {
    #[serde(default)]
    tokens: Vec<PumpFunToken>,
}

pub struct PumpFunScanner {
    client: Client,
    api_url: String,
    dry_run: bool,
}

impl PumpFunScanner {
    pub fn new(config: &BotConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_url: config.pump_fun_api_url.clone(),
            dry_run: config.dry_run,
        }
    }

    /// Generate mock tokens for dry run mode
    fn generate_mock_tokens(&self) -> Vec<String> {
        vec![
            "MockToken1111111111111111111111111111111111".to_string(),
            "MockToken2222222222222222222222222222222222".to_string(),
            "MockToken3333333333333333333333333333333333".to_string(),
        ]
    }

    /// Generate mock metrics for dry run mode
    fn generate_mock_metrics(&self, mint: &str) -> TokenMetrics {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        TokenMetrics {
            mint: mint.to_string(),
            name: format!("Mock Token {}", &mint[..8]),
            symbol: format!("MOCK{}", &mint[..4]),
            volume_5m: rng.gen_range(1.0..50.0),
            volume_1h: rng.gen_range(10.0..200.0),
            volume_24h: rng.gen_range(100.0..1000.0),
            current_price: rng.gen_range(0.0001..0.01),
            price_change_5m: rng.gen_range(-10.0..20.0),
            price_change_1h: rng.gen_range(-20.0..50.0),
            liquidity_sol: rng.gen_range(5.0..50.0),
            liquidity_usd: rng.gen_range(500.0..5000.0),
            holder_count: rng.gen_range(20..200),
            holder_concentration: rng.gen_range(0.1..0.5),
            unique_buyers_5m: rng.gen_range(5..50),
            unique_sellers_5m: rng.gen_range(2..20),
            market_cap: rng.gen_range(10000.0..100000.0),
            fully_diluted_valuation: rng.gen_range(50000.0..500000.0),
            bonding_curve_progress: rng.gen_range(10.0..90.0),
            is_graduated: false,
            created_at: chrono::Utc::now().timestamp() - rng.gen_range(60..3600),
            time_since_creation: rng.gen_range(60..3600),
            buy_pressure: rng.gen_range(0.5..2.0),
            sell_pressure: rng.gen_range(0.3..1.5),
            volatility_score: rng.gen_range(0.1..0.8),
        }
    }

    /// Scan for new tokens on pump.fun
    pub async fn scan_new_tokens(&self) -> Result<Vec<String>> {
        if self.dry_run {
            debug!("[DRY RUN] Returning mock tokens");
            let mints = self.generate_mock_tokens();
            info!("[DRY RUN] Found {} mock tokens", mints.len());
            return Ok(mints);
        }

        let url = format!("{}/tokens/latest", self.api_url);

        debug!("Scanning pump.fun for new tokens...");

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<PumpFunResponse>()
            .await?;

        let mints: Vec<String> = response.tokens.iter().map(|t| t.mint.clone()).collect();

        info!("Found {} new tokens on pump.fun", mints.len());
        Ok(mints)
    }

    /// Scan for trending/popular tokens
    pub async fn scan_trending_tokens(&self, limit: usize) -> Result<Vec<String>> {
        if self.dry_run {
            debug!("[DRY RUN] Returning mock trending tokens");
            let mints = self.generate_mock_tokens();
            info!("[DRY RUN] Found {} mock trending tokens", mints.len());
            return Ok(mints);
        }

        let url = format!("{}/tokens/trending?limit={}", self.api_url, limit);

        debug!("Scanning trending tokens on pump.fun...");

        let response = self.client
            .get(&url)
            .send()
            .await?
            .json::<PumpFunResponse>()
            .await?;

        let mints: Vec<String> = response.tokens.iter().map(|t| t.mint.clone()).collect();

        info!("Found {} trending tokens", mints.len());
        Ok(mints)
    }

    /// Get detailed metrics for a specific token
    pub async fn get_token_metrics(&self, mint: &str) -> Result<TokenMetrics> {
        if self.dry_run {
            debug!("[DRY RUN] Returning mock metrics for {}", mint);
            return Ok(self.generate_mock_metrics(mint));
        }

        let url = format!("{}/tokens/{}", self.api_url, mint);

        debug!("Fetching metrics for token {}", mint);

        // Fetch basic token data
        let token_data = self.client
            .get(&url)
            .send()
            .await?
            .json::<PumpFunToken>()
            .await?;

        // Fetch additional metrics (trades, holders, etc.)
        let trades_data = self.fetch_trade_data(mint).await?;
        let holder_data = self.fetch_holder_data(mint).await?;

        // Calculate metrics
        let metrics = self.calculate_metrics(token_data, trades_data, holder_data)?;

        debug!("Metrics calculated for {}: confidence_indicators={}", 
            metrics.symbol, 
            metrics.volume_5m
        );

        Ok(metrics)
    }

    /// Fetch recent trade data
    async fn fetch_trade_data(&self, mint: &str) -> Result<TradeData> {
        let url = format!("{}/trades/{}?limit=100", self.api_url, mint);
        
        let trades: Vec<Trade> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .unwrap_or_default();

        Ok(self.aggregate_trade_data(trades))
    }

    /// Fetch holder distribution data
    async fn fetch_holder_data(&self, mint: &str) -> Result<HolderData> {
        let url = format!("{}/holders/{}?limit=100", self.api_url, mint);
        
        let holders: Vec<Holder> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .unwrap_or_default();

        Ok(self.aggregate_holder_data(holders))
    }

    /// Aggregate trade data into metrics
    fn aggregate_trade_data(&self, trades: Vec<Trade>) -> TradeData {
        let now = chrono::Utc::now().timestamp();
        let five_min_ago = now - 300;
        let one_hour_ago = now - 3600;

        let mut volume_5m = 0.0;
        let mut volume_1h = 0.0;
        let mut volume_24h = 0.0;
        let mut unique_buyers_5m = std::collections::HashSet::new();
        let mut unique_sellers_5m = std::collections::HashSet::new();
        let mut buy_volume = 0.0;
        let mut sell_volume = 0.0;

        for trade in trades {
            volume_24h += trade.amount_sol;

            if trade.timestamp > one_hour_ago {
                volume_1h += trade.amount_sol;
            }

            if trade.timestamp > five_min_ago {
                volume_5m += trade.amount_sol;
                
                if trade.is_buy {
                    unique_buyers_5m.insert(trade.user.clone());
                    buy_volume += trade.amount_sol;
                } else {
                    unique_sellers_5m.insert(trade.user.clone());
                    sell_volume += trade.amount_sol;
                }
            }
        }

        let buy_pressure = if sell_volume > 0.0 {
            buy_volume / sell_volume
        } else {
            buy_volume
        };

        let sell_pressure = if buy_volume > 0.0 {
            sell_volume / buy_volume
        } else {
            1.0
        };

        TradeData {
            volume_5m,
            volume_1h,
            volume_24h,
            unique_buyers_5m: unique_buyers_5m.len() as u32,
            unique_sellers_5m: unique_sellers_5m.len() as u32,
            buy_pressure,
            sell_pressure,
        }
    }

    /// Aggregate holder data
    fn aggregate_holder_data(&self, holders: Vec<Holder>) -> HolderData {
        let holder_count = holders.len() as u32;
        
        let total_supply: u64 = holders.iter().map(|h| h.amount).sum();
        let top_10_amount: u64 = holders.iter().take(10).map(|h| h.amount).sum();

        let holder_concentration = if total_supply > 0 {
            top_10_amount as f64 / total_supply as f64
        } else {
            1.0
        };

        HolderData {
            holder_count,
            holder_concentration,
        }
    }

    /// Calculate comprehensive token metrics
    fn calculate_metrics(
        &self,
        token: PumpFunToken,
        trades: TradeData,
        holders: HolderData,
    ) -> Result<TokenMetrics> {
        // Fetch current price and liquidity from bonding curve
        let (current_price, liquidity_sol, bonding_progress) = (0.001, 10.0, 50.0); // TODO: actual calc

        let price_change_5m = 0.0; // TODO: calculate from trade history
        let price_change_1h = 0.0;

        Ok(TokenMetrics {
            mint: token.mint,
            name: token.name,
            symbol: token.symbol,
            volume_5m: trades.volume_5m,
            volume_1h: trades.volume_1h,
            volume_24h: trades.volume_24h,
            current_price,
            price_change_5m,
            price_change_1h,
            liquidity_sol,
            liquidity_usd: liquidity_sol * 100.0, // Assuming SOL price
            holder_count: holders.holder_count,
            holder_concentration: holders.holder_concentration,
            unique_buyers_5m: trades.unique_buyers_5m,
            unique_sellers_5m: trades.unique_sellers_5m,
            market_cap: token.usd_market_cap,
            fully_diluted_valuation: token.usd_market_cap,
            bonding_curve_progress: bonding_progress,
            is_graduated: false,
            created_at: chrono::Utc::now().timestamp(),
            time_since_creation: 0,
            buy_pressure: trades.buy_pressure,
            sell_pressure: trades.sell_pressure,
            volatility_score: 0.0,
        })
    }
}

#[derive(Debug, Deserialize)]
struct Trade {
    #[serde(default)]
    user: String,
    #[serde(default)]
    amount_sol: f64,
    #[serde(default)]
    is_buy: bool,
    #[serde(default)]
    timestamp: i64,
}

struct TradeData {
    volume_5m: f64,
    volume_1h: f64,
    volume_24h: f64,
    unique_buyers_5m: u32,
    unique_sellers_5m: u32,
    buy_pressure: f64,
    sell_pressure: f64,
}

#[derive(Debug, Deserialize)]
struct Holder {
    #[serde(default)]
    address: String,
    #[serde(default)]
    amount: u64,
}

struct HolderData {
    holder_count: u32,
    holder_concentration: f64,
}
