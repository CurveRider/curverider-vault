use crate::types::{TokenMetrics, TradingSignal, SignalType, StrategyType, StrategyExitParams};
use crate::error::Result;
use tracing::{info, warn, debug};

/// Trading Strategy Trait - All strategies must implement this
pub trait TradingStrategy: Send + Sync {
    fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal>;
    fn get_exit_params(&self) -> StrategyExitParams;
    fn name(&self) -> &str;
}

/// Advanced Multi-Factor Token Analysis (Conservative Strategy)
/// Based on 7 years of DeFi trading expertise
pub struct TokenAnalyzer {
    // Configurable thresholds
    min_liquidity: f64,
    min_volume_5m: f64,
    min_holder_count: u32,
    max_holder_concentration: f64,
}

impl TokenAnalyzer {
    pub fn new(
        min_liquidity: f64,
        min_volume_5m: f64,
        min_holder_count: u32,
        max_holder_concentration: f64,
    ) -> Self {
        Self {
            min_liquidity,
            min_volume_5m,
            min_holder_count,
            max_holder_concentration,
        }
    }

    /// Comprehensive token analysis with multiple factors
    pub fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        let mut score = 0.0;
        let mut max_score = 0.0;
        let mut reasoning = Vec::new();

        // Factor 1: Volume Analysis (Weight: 25%)
        let (volume_score, volume_reason) = self.analyze_volume(metrics);
        score += volume_score * 0.25;
        max_score += 0.25;
        reasoning.extend(volume_reason);

        // Factor 2: Liquidity Analysis (Weight: 20%)
        let (liquidity_score, liquidity_reason) = self.analyze_liquidity(metrics);
        score += liquidity_score * 0.20;
        max_score += 0.20;
        reasoning.extend(liquidity_reason);

        // Factor 3: Holder Distribution (Weight: 15%)
        let (holder_score, holder_reason) = self.analyze_holders(metrics);
        score += holder_score * 0.15;
        max_score += 0.15;
        reasoning.extend(holder_reason);

        // Factor 4: Price Momentum (Weight: 20%)
        let (momentum_score, momentum_reason) = self.analyze_momentum(metrics);
        score += momentum_score * 0.20;
        max_score += 0.20;
        reasoning.extend(momentum_reason);

        // Factor 5: Buy/Sell Pressure (Weight: 10%)
        let (pressure_score, pressure_reason) = self.analyze_pressure(metrics);
        score += pressure_score * 0.10;
        max_score += 0.10;
        reasoning.extend(pressure_reason);

        // Factor 6: Bonding Curve Progress (Weight: 10%)
        let (curve_score, curve_reason) = self.analyze_bonding_curve(metrics);
        score += curve_score * 0.10;
        max_score += 0.10;
        reasoning.extend(curve_reason);

        // Normalize confidence score
        let confidence = score / max_score;

        // Determine signal type based on confidence
        let signal_type = self.determine_signal_type(confidence, metrics);

        info!(
            "Token {} analyzed: confidence={:.2}%, signal={:?}",
            metrics.symbol,
            confidence * 100.0,
            signal_type
        );

        Ok(TradingSignal {
            token_mint: metrics.mint.parse().unwrap(),
            signal_type,
            confidence,
            reasoning,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }

    /// Factor 1: Volume Analysis
    /// Strong volume indicates real interest and reduces price impact
    fn analyze_volume(&self, metrics: &TokenMetrics) -> (f64, Vec<String>) {
        let mut score = 0.0;
        let mut reasons = Vec::new();

        // 5-minute volume (most important for sniping)
        if metrics.volume_5m > self.min_volume_5m * 2.0 {
            score += 0.4;
            reasons.push(format!("Exceptional 5m volume: {:.2} SOL", metrics.volume_5m));
        } else if metrics.volume_5m > self.min_volume_5m {
            score += 0.2;
            reasons.push(format!("Good 5m volume: {:.2} SOL", metrics.volume_5m));
        } else {
            reasons.push(format!("Low 5m volume: {:.2} SOL", metrics.volume_5m));
        }

        // Volume acceleration (1h vs 5m)
        let volume_acceleration = if metrics.volume_5m > 0.0 {
            (metrics.volume_1h / 12.0) / metrics.volume_5m
        } else {
            0.0
        };

        if volume_acceleration > 1.5 {
            score += 0.3;
            reasons.push(format!("Volume accelerating: {:.2}x", volume_acceleration));
        } else if volume_acceleration > 1.0 {
            score += 0.15;
            reasons.push(format!("Volume stable: {:.2}x", volume_acceleration));
        }

        // Unique buyers vs sellers ratio
        let buyer_seller_ratio = if metrics.unique_sellers_5m > 0 {
            metrics.unique_buyers_5m as f64 / metrics.unique_sellers_5m as f64
        } else {
            metrics.unique_buyers_5m as f64
        };

        if buyer_seller_ratio > 2.0 {
            score += 0.3;
            reasons.push(format!("Strong buyer interest: {:.2}:1 ratio", buyer_seller_ratio));
        } else if buyer_seller_ratio > 1.2 {
            score += 0.15;
            reasons.push(format!("Positive buyer/seller ratio: {:.2}:1", buyer_seller_ratio));
        }

        (score, reasons)
    }

    /// Factor 2: Liquidity Analysis
    /// Higher liquidity = lower slippage and easier exits
    fn analyze_liquidity(&self, metrics: &TokenMetrics) -> (f64, Vec<String>) {
        let mut score = 0.0;
        let mut reasons = Vec::new();

        if metrics.liquidity_sol > self.min_liquidity * 3.0 {
            score += 1.0;
            reasons.push(format!("Excellent liquidity: {:.2} SOL", metrics.liquidity_sol));
        } else if metrics.liquidity_sol > self.min_liquidity * 1.5 {
            score += 0.7;
            reasons.push(format!("Good liquidity: {:.2} SOL", metrics.liquidity_sol));
        } else if metrics.liquidity_sol > self.min_liquidity {
            score += 0.4;
            reasons.push(format!("Adequate liquidity: {:.2} SOL", metrics.liquidity_sol));
        } else {
            reasons.push(format!("Low liquidity: {:.2} SOL (risky)", metrics.liquidity_sol));
        }

        (score, reasons)
    }

    /// Factor 3: Holder Distribution Analysis
    /// Well-distributed = less rug risk
    fn analyze_holders(&self, metrics: &TokenMetrics) -> (f64, Vec<String>) {
        let mut score = 0.0;
        let mut reasons = Vec::new();

        // Holder count
        if metrics.holder_count > self.min_holder_count * 3 {
            score += 0.5;
            reasons.push(format!("Strong holder base: {} holders", metrics.holder_count));
        } else if metrics.holder_count > self.min_holder_count {
            score += 0.3;
            reasons.push(format!("Good holder count: {}", metrics.holder_count));
        } else {
            reasons.push(format!("Low holder count: {} (risky)", metrics.holder_count));
        }

        // Concentration (lower is better - more distributed)
        if metrics.holder_concentration < self.max_holder_concentration * 0.5 {
            score += 0.5;
            reasons.push(format!(
                "Well distributed: {:.1}% top holder concentration",
                metrics.holder_concentration * 100.0
            ));
        } else if metrics.holder_concentration < self.max_holder_concentration {
            score += 0.25;
            reasons.push(format!(
                "Acceptable distribution: {:.1}% concentration",
                metrics.holder_concentration * 100.0
            ));
        } else {
            reasons.push(format!(
                "High concentration risk: {:.1}% (whale risk)",
                metrics.holder_concentration * 100.0
            ));
        }

        (score, reasons)
    }

    /// Factor 4: Price Momentum Analysis
    /// Positive momentum indicates trend strength
    fn analyze_momentum(&self, metrics: &TokenMetrics) -> (f64, Vec<String>) {
        let mut score = 0.0;
        let mut reasons = Vec::new();

        // 5-minute momentum (most important for entry timing)
        if metrics.price_change_5m > 0.20 {
            score += 0.5;
            reasons.push(format!("Strong 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        } else if metrics.price_change_5m > 0.10 {
            score += 0.3;
            reasons.push(format!("Good 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        } else if metrics.price_change_5m > 0.0 {
            score += 0.1;
            reasons.push(format!("Positive 5m: +{:.1}%", metrics.price_change_5m * 100.0));
        } else {
            reasons.push(format!("Negative 5m: {:.1}%", metrics.price_change_5m * 100.0));
        }

        // 1-hour momentum
        if metrics.price_change_1h > 0.50 {
            score += 0.5;
            reasons.push(format!("Explosive 1h growth: +{:.1}%", metrics.price_change_1h * 100.0));
        } else if metrics.price_change_1h > 0.25 {
            score += 0.3;
            reasons.push(format!("Strong 1h growth: +{:.1}%", metrics.price_change_1h * 100.0));
        } else if metrics.price_change_1h > 0.0 {
            score += 0.1;
        }

        (score, reasons)
    }

    /// Factor 5: Buy/Sell Pressure Analysis
    fn analyze_pressure(&self, metrics: &TokenMetrics) -> (f64, Vec<String>) {
        let mut score = 0.0;
        let mut reasons = Vec::new();

        let pressure_ratio = if metrics.sell_pressure > 0.0 {
            metrics.buy_pressure / metrics.sell_pressure
        } else {
            metrics.buy_pressure
        };

        if pressure_ratio > 3.0 {
            score += 1.0;
            reasons.push(format!("Dominant buy pressure: {:.2}:1", pressure_ratio));
        } else if pressure_ratio > 1.5 {
            score += 0.7;
            reasons.push(format!("Strong buy pressure: {:.2}:1", pressure_ratio));
        } else if pressure_ratio > 1.0 {
            score += 0.4;
            reasons.push(format!("Positive buy pressure: {:.2}:1", pressure_ratio));
        } else {
            reasons.push(format!("Sell pressure dominant: {:.2}:1", pressure_ratio));
        }

        (score, reasons)
    }

    /// Factor 6: Bonding Curve Analysis
    /// Sweet spot: 30-70% - enough validation, but room to grow
    fn analyze_bonding_curve(&self, metrics: &TokenMetrics) -> (f64, Vec<String>) {
        let mut score = 0.0;
        let mut reasons = Vec::new();

        if metrics.is_graduated {
            score += 0.5;
            reasons.push("Token graduated to DEX".to_string());
        } else if metrics.bonding_curve_progress > 70.0 {
            score += 0.8;
            reasons.push(format!(
                "Near graduation: {:.1}% bonding curve",
                metrics.bonding_curve_progress
            ));
        } else if metrics.bonding_curve_progress > 30.0 {
            score += 1.0;
            reasons.push(format!(
                "Sweet spot: {:.1}% bonding curve (validated + room to grow)",
                metrics.bonding_curve_progress
            ));
        } else if metrics.bonding_curve_progress > 10.0 {
            score += 0.6;
            reasons.push(format!(
                "Early stage: {:.1}% bonding curve",
                metrics.bonding_curve_progress
            ));
        } else {
            score += 0.2;
            reasons.push(format!(
                "Very early: {:.1}% bonding curve (high risk)",
                metrics.bonding_curve_progress
            ));
        }

        (score, reasons)
    }

    /// Determine signal type based on confidence and other factors
    fn determine_signal_type(&self, confidence: f64, metrics: &TokenMetrics) -> SignalType {
        // Check for deal-breakers
        if metrics.liquidity_sol < self.min_liquidity {
            return SignalType::Hold;
        }

        if metrics.holder_concentration > self.max_holder_concentration * 1.5 {
            warn!("Token {} rejected: too concentrated", metrics.symbol);
            return SignalType::Hold;
        }

        // Signal based on confidence
        if confidence >= 0.80 {
            SignalType::StrongBuy
        } else if confidence >= 0.65 {
            SignalType::Buy
        } else if confidence >= 0.45 {
            SignalType::Hold
        } else if confidence >= 0.30 {
            SignalType::Sell
        } else {
            SignalType::StrongSell
        }
    }

    /// Calculate volatility score (0-1, higher = more volatile)
    pub fn calculate_volatility(&self, metrics: &TokenMetrics) -> f64 {
        let price_volatility = (metrics.price_change_5m.abs() + metrics.price_change_1h.abs()) / 2.0;
        let volume_volatility = if metrics.volume_1h > 0.0 {
            (metrics.volume_5m * 12.0 / metrics.volume_1h - 1.0).abs()
        } else {
            1.0
        };

        (price_volatility + volume_volatility) / 2.0
    }
}

/// Implement TradingStrategy trait for TokenAnalyzer (Conservative Strategy)
impl TradingStrategy for TokenAnalyzer {
    fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        TokenAnalyzer::analyze(self, metrics)
    }

    fn get_exit_params(&self) -> StrategyExitParams {
        StrategyExitParams {
            take_profit_multiplier: 2.0,
            stop_loss_percentage: 0.5,
            position_timeout_seconds: 3600,
            use_trailing_stop: false,
            trailing_activation_pct: 0.0,
            trailing_distance_pct: 0.0,
        }
    }

    fn name(&self) -> &str {
        "Conservative Multi-Factor"
    }
}

// ============================================================================
// STRATEGY 1: ULTRA-EARLY SNIPER
// High Risk, High Reward - First 5 Minutes
// ============================================================================

pub struct UltraEarlySniper {
    min_liquidity: f64,
}

impl UltraEarlySniper {
    pub fn new() -> Self {
        Self {
            min_liquidity: 1.0, // Accept low liquidity for ultra-early
        }
    }

    fn analyze_impl(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        let mut score = 0.0;
        let mut max_score = 0.0;
        let mut reasoning = Vec::new();

        // CRITICAL: Must be ultra-early (< 5 minutes old)
        if metrics.time_since_creation > 300 {
            return Ok(TradingSignal {
                token_mint: metrics.mint.parse().unwrap(),
                signal_type: SignalType::Hold,
                confidence: 0.0,
                reasoning: vec!["Too old for ultra-early strategy (>5min)".to_string()],
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        // CRITICAL: Must be very early bonding curve (< 10%)
        if metrics.bonding_curve_progress > 10.0 {
            return Ok(TradingSignal {
                token_mint: metrics.mint.parse().unwrap(),
                signal_type: SignalType::Hold,
                confidence: 0.0,
                reasoning: vec!["Bonding curve too advanced for ultra-early (>10%)".to_string()],
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        // Factor 1: Buy Pressure (35% weight) - MOST IMPORTANT
        let pressure_ratio = if metrics.sell_pressure > 0.0 {
            metrics.buy_pressure / metrics.sell_pressure
        } else {
            metrics.buy_pressure
        };

        if pressure_ratio > 10.0 {
            score += 1.0 * 0.35;
            reasoning.push(format!("EXCEPTIONAL buy pressure: {:.1}:1 ratio", pressure_ratio));
        } else if pressure_ratio > 5.0 {
            score += 0.8 * 0.35;
            reasoning.push(format!("Dominant buy pressure: {:.1}:1 ratio", pressure_ratio));
        } else if pressure_ratio > 3.0 {
            score += 0.5 * 0.35;
            reasoning.push(format!("Strong buy pressure: {:.1}:1 ratio", pressure_ratio));
        } else {
            reasoning.push(format!("Weak buy pressure: {:.1}:1 (risky)", pressure_ratio));
        }
        max_score += 0.35;

        // Factor 2: Volume Acceleration (30% weight)
        let volume_acceleration = if metrics.volume_5m > 0.0 && metrics.volume_1h > 0.0 {
            (metrics.volume_5m * 12.0) / metrics.volume_1h
        } else {
            1.0
        };

        if volume_acceleration > 5.0 {
            score += 1.0 * 0.30;
            reasoning.push(format!("EXPLOSIVE volume acceleration: {:.1}x", volume_acceleration));
        } else if volume_acceleration > 3.0 {
            score += 0.8 * 0.30;
            reasoning.push(format!("Strong volume acceleration: {:.1}x", volume_acceleration));
        } else if volume_acceleration > 1.5 {
            score += 0.5 * 0.30;
            reasoning.push(format!("Good volume acceleration: {:.1}x", volume_acceleration));
        } else {
            reasoning.push(format!("Low volume acceleration: {:.1}x", volume_acceleration));
        }
        max_score += 0.30;

        // Factor 3: Price Momentum 5m (20% weight)
        if metrics.price_change_5m > 0.50 {
            score += 1.0 * 0.20;
            reasoning.push(format!("EXPLOSIVE 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        } else if metrics.price_change_5m > 0.30 {
            score += 0.8 * 0.20;
            reasoning.push(format!("Strong 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        } else if metrics.price_change_5m > 0.15 {
            score += 0.5 * 0.20;
            reasoning.push(format!("Good 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        } else {
            reasoning.push(format!("Weak 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        }
        max_score += 0.20;

        // Factor 4: Holder Growth (10% weight)
        if metrics.unique_buyers_5m > 50 {
            score += 1.0 * 0.10;
            reasoning.push(format!("Viral growth: {} new buyers in 5m", metrics.unique_buyers_5m));
        } else if metrics.unique_buyers_5m > 30 {
            score += 0.7 * 0.10;
            reasoning.push(format!("Strong growth: {} new buyers", metrics.unique_buyers_5m));
        } else if metrics.unique_buyers_5m > 20 {
            score += 0.4 * 0.10;
            reasoning.push(format!("Good growth: {} new buyers", metrics.unique_buyers_5m));
        }
        max_score += 0.10;

        // Factor 5: Minimal Liquidity Check (5% weight)
        if metrics.liquidity_sol > self.min_liquidity * 3.0 {
            score += 1.0 * 0.05;
            reasoning.push(format!("Good early liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else if metrics.liquidity_sol > self.min_liquidity {
            score += 0.5 * 0.05;
            reasoning.push(format!("Adequate liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else {
            reasoning.push(format!("Very low liquidity: {:.1} SOL (high risk)", metrics.liquidity_sol));
        }
        max_score += 0.05;

        // Normalize confidence
        let confidence = score / max_score;

        // Determine signal type - AGGRESSIVE thresholds
        let signal_type = if confidence >= 0.75 {
            SignalType::StrongBuy
        } else if confidence >= 0.60 {
            SignalType::Buy
        } else if confidence >= 0.40 {
            SignalType::Hold
        } else {
            SignalType::Sell
        };

        info!(
            "[ULTRA-EARLY SNIPER] {} analyzed: confidence={:.1}%, age={}s, curve={:.1}%, signal={:?}",
            metrics.symbol,
            confidence * 100.0,
            metrics.time_since_creation,
            metrics.bonding_curve_progress,
            signal_type
        );

        Ok(TradingSignal {
            token_mint: metrics.mint.parse().unwrap(),
            signal_type,
            confidence,
            reasoning,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }
}

impl TradingStrategy for UltraEarlySniper {
    fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        self.analyze_impl(metrics)
    }

    fn get_exit_params(&self) -> StrategyExitParams {
        StrategyExitParams {
            take_profit_multiplier: 3.0,  // Aggressive 3x target
            stop_loss_percentage: 0.30,    // Tight 30% SL
            position_timeout_seconds: 600, // 10 minutes max
            use_trailing_stop: false,
            trailing_activation_pct: 0.0,
            trailing_distance_pct: 0.0,
        }
    }

    fn name(&self) -> &str {
        "Ultra-Early Sniper (High Risk)"
    }
}

// ============================================================================
// STRATEGY 2: MOMENTUM SCALPER
// Quick Flips - 15-30 Minute Holds
// ============================================================================

pub struct MomentumScalper {
    min_liquidity: f64,
    min_volume_5m: f64,
}

impl MomentumScalper {
    pub fn new() -> Self {
        Self {
            min_liquidity: 8.0,  // Need exit liquidity
            min_volume_5m: 20.0, // Need strong volume
        }
    }

    fn analyze_impl(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        let mut score = 0.0;
        let mut max_score = 0.0;
        let mut reasoning = Vec::new();

        // Must be in sweet spot for momentum (40-80% bonding curve)
        if metrics.bonding_curve_progress < 40.0 || metrics.bonding_curve_progress > 80.0 {
            return Ok(TradingSignal {
                token_mint: metrics.mint.parse().unwrap(),
                signal_type: SignalType::Hold,
                confidence: 0.0,
                reasoning: vec![format!("Bonding curve {:.1}% outside momentum zone (40-80%)", metrics.bonding_curve_progress)],
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        // Factor 1: Price Momentum (40% weight) - MOST IMPORTANT
        let momentum_score = if metrics.price_change_1h > 1.0 {
            1.0
        } else if metrics.price_change_1h > 0.75 {
            0.9
        } else if metrics.price_change_1h > 0.50 {
            0.7
        } else if metrics.price_change_1h > 0.30 {
            0.4
        } else {
            0.0
        };

        score += momentum_score * 0.40;
        max_score += 0.40;

        if metrics.price_change_1h > 0.50 {
            reasoning.push(format!("EXPLOSIVE 1h growth: +{:.1}%", metrics.price_change_1h * 100.0));
        } else {
            reasoning.push(format!("Weak 1h momentum: +{:.1}%", metrics.price_change_1h * 100.0));
        }

        // 5m momentum continuation
        if metrics.price_change_5m > 0.20 {
            score += 0.5 * 0.40;
            reasoning.push(format!("Strong 5m continuation: +{:.1}%", metrics.price_change_5m * 100.0));
        } else if metrics.price_change_5m > 0.10 {
            score += 0.3 * 0.40;
            reasoning.push(format!("Good 5m momentum: +{:.1}%", metrics.price_change_5m * 100.0));
        }

        // Factor 2: Volume Analysis (30% weight)
        let volume_score = if metrics.volume_5m > self.min_volume_5m * 3.0 {
            1.0
        } else if metrics.volume_5m > self.min_volume_5m * 2.0 {
            0.7
        } else if metrics.volume_5m > self.min_volume_5m {
            0.4
        } else {
            0.0
        };

        score += volume_score * 0.30;
        max_score += 0.30;
        reasoning.push(format!("5m volume: {:.1} SOL", metrics.volume_5m));

        // Factor 3: Buy Pressure (20% weight)
        let pressure_ratio = if metrics.sell_pressure > 0.0 {
            metrics.buy_pressure / metrics.sell_pressure
        } else {
            metrics.buy_pressure
        };

        if pressure_ratio > 3.0 {
            score += 1.0 * 0.20;
            reasoning.push(format!("Dominant buy pressure: {:.1}:1", pressure_ratio));
        } else if pressure_ratio > 2.0 {
            score += 0.7 * 0.20;
            reasoning.push(format!("Strong buy pressure: {:.1}:1", pressure_ratio));
        } else if pressure_ratio > 1.5 {
            score += 0.4 * 0.20;
            reasoning.push(format!("Positive pressure: {:.1}:1", pressure_ratio));
        }
        max_score += 0.20;

        // Factor 4: Liquidity (10% weight)
        if metrics.liquidity_sol > self.min_liquidity * 2.0 {
            score += 1.0 * 0.10;
            reasoning.push(format!("Excellent liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else if metrics.liquidity_sol > self.min_liquidity {
            score += 0.5 * 0.10;
            reasoning.push(format!("Good liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else {
            reasoning.push(format!("Low liquidity: {:.1} SOL (risky exit)", metrics.liquidity_sol));
        }
        max_score += 0.10;

        // Normalize confidence
        let confidence = score / max_score;

        // Determine signal type
        let signal_type = if confidence >= 0.75 {
            SignalType::StrongBuy
        } else if confidence >= 0.60 {
            SignalType::Buy
        } else if confidence >= 0.45 {
            SignalType::Hold
        } else {
            SignalType::Sell
        };

        info!(
            "[MOMENTUM SCALPER] {} analyzed: confidence={:.1}%, 1h_change=+{:.1}%, signal={:?}",
            metrics.symbol,
            confidence * 100.0,
            metrics.price_change_1h * 100.0,
            signal_type
        );

        Ok(TradingSignal {
            token_mint: metrics.mint.parse().unwrap(),
            signal_type,
            confidence,
            reasoning,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }
}

impl TradingStrategy for MomentumScalper {
    fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        self.analyze_impl(metrics)
    }

    fn get_exit_params(&self) -> StrategyExitParams {
        StrategyExitParams {
            take_profit_multiplier: 1.5,   // Quick 1.5x scalp
            stop_loss_percentage: 0.25,     // 25% SL
            position_timeout_seconds: 1800, // 30 minutes
            use_trailing_stop: true,        // Use trailing stop
            trailing_activation_pct: 0.20,  // Activate at +20%
            trailing_distance_pct: 0.10,    // Trail by 10%
        }
    }

    fn name(&self) -> &str {
        "Momentum Scalper (Quick Flips)"
    }
}

// ============================================================================
// STRATEGY 3: GRADUATION ANTICIPATOR
// Pre-DEX Positioning - Low Risk, High Success Rate
// ============================================================================

pub struct GraduationAnticipator {
    min_liquidity: f64,
    min_holder_count: u32,
    max_holder_concentration: f64,
}

impl GraduationAnticipator {
    pub fn new() -> Self {
        Self {
            min_liquidity: 15.0,            // Need strong DEX migration liquidity
            min_holder_count: 100,          // Established community
            max_holder_concentration: 0.25, // Well distributed
        }
    }

    fn analyze_impl(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        let mut score = 0.0;
        let mut max_score = 0.0;
        let mut reasoning = Vec::new();

        // Must be in graduation zone (60-85% bonding curve)
        if metrics.bonding_curve_progress < 60.0 || metrics.bonding_curve_progress > 85.0 {
            return Ok(TradingSignal {
                token_mint: metrics.mint.parse().unwrap(),
                signal_type: SignalType::Hold,
                confidence: 0.0,
                reasoning: vec![format!("Bonding curve {:.1}% outside graduation zone (60-85%)", metrics.bonding_curve_progress)],
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        // Already graduated? Skip
        if metrics.is_graduated {
            return Ok(TradingSignal {
                token_mint: metrics.mint.parse().unwrap(),
                signal_type: SignalType::Hold,
                confidence: 0.0,
                reasoning: vec!["Already graduated to DEX".to_string()],
                timestamp: chrono::Utc::now().timestamp(),
            });
        }

        // Factor 1: Bonding Curve Progress (30% weight)
        let curve_score = if metrics.bonding_curve_progress >= 70.0 && metrics.bonding_curve_progress <= 80.0 {
            1.0 // Sweet spot
        } else if metrics.bonding_curve_progress >= 65.0 && metrics.bonding_curve_progress < 85.0 {
            0.8
        } else {
            0.5
        };

        score += curve_score * 0.30;
        max_score += 0.30;
        reasoning.push(format!("Near graduation: {:.1}% bonding curve", metrics.bonding_curve_progress));

        // Factor 2: Liquidity (25% weight)
        if metrics.liquidity_sol > self.min_liquidity * 2.0 {
            score += 1.0 * 0.25;
            reasoning.push(format!("Excellent DEX-ready liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else if metrics.liquidity_sol > self.min_liquidity * 1.5 {
            score += 0.7 * 0.25;
            reasoning.push(format!("Strong liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else if metrics.liquidity_sol > self.min_liquidity {
            score += 0.4 * 0.25;
            reasoning.push(format!("Adequate liquidity: {:.1} SOL", metrics.liquidity_sol));
        } else {
            reasoning.push(format!("Low liquidity: {:.1} SOL (risky)", metrics.liquidity_sol));
        }
        max_score += 0.25;

        // Factor 3: Holder Distribution (20% weight)
        if metrics.holder_count > self.min_holder_count * 2 {
            score += 0.5 * 0.20;
            reasoning.push(format!("Strong community: {} holders", metrics.holder_count));
        } else if metrics.holder_count > self.min_holder_count {
            score += 0.3 * 0.20;
            reasoning.push(format!("Good holder base: {} holders", metrics.holder_count));
        } else {
            reasoning.push(format!("Weak holder count: {}", metrics.holder_count));
        }

        if metrics.holder_concentration < self.max_holder_concentration * 0.6 {
            score += 0.5 * 0.20;
            reasoning.push(format!("Well distributed: {:.1}% concentration", metrics.holder_concentration * 100.0));
        } else if metrics.holder_concentration < self.max_holder_concentration {
            score += 0.3 * 0.20;
            reasoning.push(format!("Acceptable distribution: {:.1}%", metrics.holder_concentration * 100.0));
        } else {
            reasoning.push(format!("High concentration risk: {:.1}%", metrics.holder_concentration * 100.0));
        }
        max_score += 0.20;

        // Factor 4: Volume Sustained (15% weight)
        if metrics.volume_24h > 100.0 {
            score += 1.0 * 0.15;
            reasoning.push(format!("Exceptional 24h volume: {:.1} SOL", metrics.volume_24h));
        } else if metrics.volume_24h > 50.0 {
            score += 0.7 * 0.15;
            reasoning.push(format!("Strong 24h volume: {:.1} SOL", metrics.volume_24h));
        } else if metrics.volume_24h > 25.0 {
            score += 0.4 * 0.15;
            reasoning.push(format!("Good sustained volume: {:.1} SOL", metrics.volume_24h));
        }
        max_score += 0.15;

        // Factor 5: Price Stability (10% weight)
        let volatility = (metrics.price_change_5m.abs() + metrics.price_change_1h.abs()) / 2.0;

        if volatility < 0.20 {
            score += 1.0 * 0.10;
            reasoning.push("Stable price action (low volatility)".to_string());
        } else if volatility < 0.40 {
            score += 0.6 * 0.10;
            reasoning.push("Moderate volatility".to_string());
        } else {
            reasoning.push("High volatility (risky)".to_string());
        }
        max_score += 0.10;

        // Normalize confidence
        let confidence = score / max_score;

        // Determine signal type - Conservative thresholds
        let signal_type = if confidence >= 0.75 {
            SignalType::StrongBuy
        } else if confidence >= 0.60 {
            SignalType::Buy
        } else if confidence >= 0.45 {
            SignalType::Hold
        } else {
            SignalType::Sell
        };

        info!(
            "[GRADUATION ANTICIPATOR] {} analyzed: confidence={:.1}%, curve={:.1}%, holders={}, signal={:?}",
            metrics.symbol,
            confidence * 100.0,
            metrics.bonding_curve_progress,
            metrics.holder_count,
            signal_type
        );

        Ok(TradingSignal {
            token_mint: metrics.mint.parse().unwrap(),
            signal_type,
            confidence,
            reasoning,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }
}

impl TradingStrategy for GraduationAnticipator {
    fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        self.analyze_impl(metrics)
    }

    fn get_exit_params(&self) -> StrategyExitParams {
        StrategyExitParams {
            take_profit_multiplier: 1.8,    // Conservative 1.8x
            stop_loss_percentage: 0.35,      // Wider 35% SL
            position_timeout_seconds: 7200,  // 2 hours
            use_trailing_stop: false,
            trailing_activation_pct: 0.0,
            trailing_distance_pct: 0.0,
        }
    }

    fn name(&self) -> &str {
        "Graduation Anticipator (Low Risk)"
    }
}

/// Factory function to create strategy based on type
pub fn create_strategy(strategy_type: StrategyType) -> Box<dyn TradingStrategy> {
    match strategy_type {
        StrategyType::Conservative => Box::new(TokenAnalyzer::new(5.0, 10.0, 50, 0.3)),
        StrategyType::UltraEarlySniper => Box::new(UltraEarlySniper::new()),
        StrategyType::MomentumScalper => Box::new(MomentumScalper::new()),
        StrategyType::GraduationAnticipator => Box::new(GraduationAnticipator::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_confidence_token() {
        let analyzer = TokenAnalyzer::new(5.0, 10.0, 50, 0.3);

        let metrics = TokenMetrics {
            mint: "test123".to_string(),
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            volume_5m: 25.0,
            volume_1h: 200.0,
            volume_24h: 1000.0,
            current_price: 0.001,
            price_change_5m: 0.15,
            price_change_1h: 0.40,
            liquidity_sol: 20.0,
            liquidity_usd: 2000.0,
            holder_count: 200,
            holder_concentration: 0.15,
            unique_buyers_5m: 50,
            unique_sellers_5m: 20,
            market_cap: 100000.0,
            fully_diluted_valuation: 100000.0,
            bonding_curve_progress: 50.0,
            is_graduated: false,
            created_at: 0,
            time_since_creation: 3600,
            buy_pressure: 3.0,
            sell_pressure: 1.0,
            volatility_score: 0.3,
        };

        let signal = analyzer.analyze(&metrics).unwrap();
        assert!(signal.confidence > 0.7);
        assert!(matches!(
            signal.signal_type,
            SignalType::StrongBuy | SignalType::Buy
        ));
    }
}
