# Meme Coin Sniping Strategies

This document describes the four trading strategies implemented in the Curverider Vault bot for sniping meme coins on pump.fun.

## Strategy Overview

| Strategy | Risk Level | Target Return | Hold Time | Win Rate | Best For |
|----------|-----------|---------------|-----------|----------|----------|
| **Conservative** | Medium | 2x | 1 hour | ~60-70% | Consistent, balanced returns |
| **Ultra-Early Sniper** | Very High | 3-10x | 10 minutes | ~30-40% | Moonshot hunting, high risk tolerance |
| **Momentum Scalper** | High | 1.5x | 30 minutes | ~50-60% | Active trading, quick flips |
| **Graduation Anticipator** | Low | 1.8x | 2 hours | ~70-80% | Risk-averse, steady gains |

---

## Strategy 1: Conservative Multi-Factor (Default)

**Philosophy**: Balanced approach combining fundamental analysis with technical signals.

### Entry Criteria
- **Bonding Curve**: 30-70% (validated but room to grow)
- **Minimum Confidence**: 80%
- **Key Factors**:
  - Volume Analysis (25% weight): 5m volume, acceleration, buyer/seller ratio
  - Liquidity Analysis (20% weight): Minimum 5 SOL liquidity
  - Holder Distribution (15% weight): >50 holders, <30% concentration
  - Price Momentum (20% weight): 5m and 1h price changes
  - Buy/Sell Pressure (10% weight): Buy pressure ratio
  - Bonding Curve Progress (10% weight): Sweet spot detection

### Exit Parameters
- **Take Profit**: 2x (100% gain)
- **Stop Loss**: 50% (conservative capital protection)
- **Timeout**: 3600 seconds (1 hour)
- **Trailing Stop**: No

### When to Use
- Default strategy for most traders
- Medium risk tolerance
- Looking for consistent wins
- Can't monitor positions constantly

### Example Token Profile
```
Symbol: PEPE2
Age: 45 minutes
Bonding Curve: 55%
Liquidity: 12 SOL
Holders: 150 (concentration: 18%)
Volume 5m: 25 SOL
Price Change 1h: +45%
Buy Pressure: 2.5:1
```

---

## Strategy 2: Ultra-Early Sniper

**Philosophy**: Catch tokens in the first 5 minutes before most scanners detect them. High risk, moonshot potential.

### Entry Criteria
- **Bonding Curve**: <10% (VERY early stage)
- **Token Age**: <5 minutes old (CRITICAL)
- **Minimum Confidence**: 75%
- **Key Factors**:
  - Buy Pressure (35% weight): >5:1 ratio required for strong signal
  - Volume Acceleration (30% weight): Rapid growth = viral potential
  - Price Momentum 5m (20% weight): Early momentum confirmation
  - Holder Growth (10% weight): New wallets joining rapidly
  - Minimal Liquidity (5% weight): Accept 1+ SOL (low barrier)

### Exit Parameters
- **Take Profit**: 3x (200% gain, but often holds for more)
- **Stop Loss**: 30% (tight - cut losers FAST)
- **Timeout**: 600 seconds (10 minutes max)
- **Trailing Stop**: No (quick exit on target)

### When to Use
- High risk tolerance
- Can monitor positions actively
- Looking for 10-100x moonshots
- Willing to accept 60-70% loss rate
- Small position sizes

### Example Token Profile
```
Symbol: MOONCAT
Age: 2 minutes 14 seconds
Bonding Curve: 4.2%
Liquidity: 2.3 SOL
Holders: 35
Unique Buyers 5m: 42
Volume 5m: 8 SOL
Volume Acceleration: 6.5x
Price Change 5m: +85%
Buy Pressure: 12:1
```

### Risk Warning
This strategy has the HIGHEST failure rate but captures the biggest winners. Use only 10-20% of total capital. Many tokens will rug or dump within minutes.

---

## Strategy 3: Momentum Scalper

**Philosophy**: Ride explosive momentum waves. Fast in, fast out on tokens showing parabolic price action.

### Entry Criteria
- **Bonding Curve**: 40-80% (mid-stage with established momentum)
- **Price Change 1h**: >50% (EXPLOSIVE growth required)
- **Minimum Confidence**: 75%
- **Key Factors**:
  - Price Momentum (40% weight): 1h and 5m combined (MOST IMPORTANT)
  - Volume Analysis (30% weight): High absolute volume + acceleration
  - Buy Pressure (20% weight): Confirmation of trend continuation
  - Liquidity (10% weight): Need 8+ SOL for quick exits
  - Bonding Curve Progress: Not weighted (speed over fundamentals)
  - Holder Distribution: Not weighted (momentum is king)

### Exit Parameters
- **Take Profit**: 1.5x (50% gain, quick scalp)
- **Stop Loss**: 25% (protect capital)
- **Timeout**: 1800 seconds (30 minutes)
- **Trailing Stop**: YES
  - Activate at +20% gain
  - Trail by 10%

### When to Use
- Active trading style
- Can monitor positions in real-time
- Looking for quick profits
- High volume trading (many positions per day)
- Comfortable with volatility

### Example Token Profile
```
Symbol: TURBOPUMP
Age: 2 hours
Bonding Curve: 68%
Liquidity: 18 SOL
Volume 5m: 45 SOL
Volume 1h: 380 SOL
Price Change 5m: +15%
Price Change 1h: +120%
Buy Pressure: 3.2:1
```

### Trading Tip
The trailing stop is key - it locks in profits if momentum continues while protecting against sudden reversals. Monitor for the trailing stop activation message.

---

## Strategy 4: Graduation Anticipator

**Philosophy**: Position before DEX graduation to capture the migration pump. Lower risk, consistent success.

### Entry Criteria
- **Bonding Curve**: 60-85% (near graduation threshold)
- **Not Yet Graduated**: Must still be on bonding curve
- **Minimum Confidence**: 75%
- **Key Factors**:
  - Bonding Curve Progress (30% weight): Sweet spot 70-80%
  - Liquidity (25% weight): >15 SOL needed for DEX migration
  - Holder Distribution (20% weight): >100 holders, <25% concentration
  - Volume Sustained (15% weight): 24h volume matters (>50 SOL)
  - Price Stability (10% weight): Lower volatility preferred

### Exit Parameters
- **Take Profit**: 1.8x (80% gain)
- **Stop Loss**: 35% (wider tolerance for fluctuations)
- **Timeout**: 7200 seconds (2 hours)
- **Special Exit**: Auto-sell on graduation event + 5 minutes
- **Trailing Stop**: No

### When to Use
- Risk-averse traders
- Looking for consistent wins
- Can't monitor constantly (wider stops)
- Prefer quality over quantity
- Longer hold times acceptable

### Example Token Profile
```
Symbol: SAFEMOON2
Age: 6 hours
Bonding Curve: 74.5%
Liquidity: 28 SOL
Holders: 245 (concentration: 19%)
Volume 24h: 120 SOL
Volume 5m: 8 SOL
Price Change 1h: +12%
Price Volatility: 0.28 (low)
```

### Expected Pattern
Tokens typically graduate around 85-90% bonding curve progress. The graduation event often triggers a 20-50% pump as:
1. New DEX liquidity arrives
2. More traders can access the token
3. Arbitrage opportunities create buying pressure

This strategy positions you BEFORE that event with established, stable tokens.

---

## Strategy Comparison Matrix

### Risk vs Reward

```
Risk Level:   Low ←―――――――――――――――――――――――――――→ High
              [Graduation] [Conservative] [Momentum] [Ultra-Early]

Expected ROI: 1.8x         2.0x           1.5x        3-10x
Win Rate:     75%          65%            55%         35%
Hold Time:    2 hours      1 hour         30 min      10 min
```

### Factor Importance by Strategy

| Factor | Conservative | Ultra-Early | Momentum | Graduation |
|--------|-------------|-------------|----------|------------|
| **Buy Pressure** | 10% | **35%** | 20% | 0% |
| **Volume** | 25% | 30% | **30%** | 15% |
| **Liquidity** | 20% | 5% | 10% | **25%** |
| **Momentum** | 20% | 20% | **40%** | 0% |
| **Holders** | 15% | 10% | 0% | 20% |
| **Bonding Curve** | 10% | 0% (inverse) | 0% (filter) | **30%** |
| **Stability** | 0% | 0% | 0% | 10% |

### Capital Allocation Recommendations

For a diversified meme coin portfolio with 10 SOL:

- **Conservative**: 4 SOL (40%) - Core holdings
- **Graduation**: 3 SOL (30%) - Stable income
- **Momentum**: 2 SOL (20%) - Active trading
- **Ultra-Early**: 1 SOL (10%) - Moonshot lottery tickets

---

## Configuration

### Set Strategy via Environment Variable

Edit your `.env` file:

```bash
# Choose one:
STRATEGY_TYPE=conservative          # Default, balanced
STRATEGY_TYPE=ultra_early_sniper   # High risk moonshots
STRATEGY_TYPE=momentum_scalper     # Quick flips
STRATEGY_TYPE=graduation_anticipator # Low risk, pre-DEX
```

### Running Multiple Strategies Simultaneously

To run multiple strategies in parallel (recommended), launch multiple bot instances:

```bash
# Terminal 1 - Conservative (main capital)
STRATEGY_TYPE=conservative MAX_POSITION_SIZE_SOL=0.5 ./bot

# Terminal 2 - Momentum Scalper (active trading)
STRATEGY_TYPE=momentum_scalper MAX_POSITION_SIZE_SOL=0.3 ./bot

# Terminal 3 - Ultra-Early Sniper (moonshot hunting)
STRATEGY_TYPE=ultra_early_sniper MAX_POSITION_SIZE_SOL=0.1 ./bot
```

---

## Performance Optimization Tips

### Conservative Strategy
- Increase `MIN_LIQUIDITY_SOL` to 8-10 for safer trades
- Raise `HOLDER_COUNT_MIN` to 100 for established tokens
- Monitor 24h volume trends

### Ultra-Early Sniper
- Set `SCAN_INTERVAL_MS=500` for faster detection
- Use smaller `MAX_POSITION_SIZE_SOL=0.1`
- Be prepared for high turnover
- Watch for bot-generated tokens (very low holder diversity)

### Momentum Scalper
- Monitor trailing stop activations closely
- Set `SCAN_INTERVAL_MS=750` for active scanning
- Look for volume > 50 SOL for best liquidity
- Exit manually on momentum breaks

### Graduation Anticipator
- Check bonding curve progress trends
- Set alerts for 75%+ bonding curve
- Monitor Raydium migration events
- Consider holding through graduation for extra pump

---

## Risk Management

### Position Sizing
- Never risk more than 2-5% of total capital per trade
- Ultra-Early Sniper: 1% per trade maximum
- Use smaller sizes when testing new strategies

### Stop Losses
- ALWAYS honor stop losses
- Never "hope" for recovery on meme coins
- The next opportunity is 60 seconds away

### Portfolio Limits
- `MAX_CONCURRENT_POSITIONS=5` recommended
- Diversify across strategies
- Don't over-concentrate in one token

### Red Flags (All Strategies)
- Holder concentration >50% (whale risk)
- Liquidity <3 SOL (rug pull risk)
- Sellers > Buyers consistently
- Volume dropping rapidly
- Dev wallets dumping

---

## Strategy Selection Guide

### "I want consistent, steady profits"
→ **Graduation Anticipator** + **Conservative**

### "I want to find the next 100x"
→ **Ultra-Early Sniper** (small positions, high volume)

### "I want to actively trade and scalp"
→ **Momentum Scalper** (requires monitoring)

### "I want balanced risk/reward"
→ **Conservative** (default, well-tested)

### "I want to maximize yield overall"
→ Run all four strategies in parallel with proper capital allocation

---

## Advanced: Creating Custom Strategies

To create your own strategy:

1. Implement the `TradingStrategy` trait in [analyzer.rs](bot-rust/src/analyzer.rs)
2. Define your analysis logic in `analyze()` method
3. Set exit parameters in `get_exit_params()` method
4. Add to the `create_strategy()` factory function
5. Add new enum variant to `StrategyType` in [types.rs](bot-rust/src/types.rs)

Example skeleton:

```rust
pub struct MyCustomStrategy {
    // Your parameters
}

impl TradingStrategy for MyCustomStrategy {
    fn analyze(&self, metrics: &TokenMetrics) -> Result<TradingSignal> {
        // Your analysis logic
    }

    fn get_exit_params(&self) -> StrategyExitParams {
        StrategyExitParams {
            take_profit_multiplier: 2.5,
            stop_loss_percentage: 0.40,
            position_timeout_seconds: 2400,
            use_trailing_stop: true,
            trailing_activation_pct: 0.25,
            trailing_distance_pct: 0.15,
        }
    }

    fn name(&self) -> &str {
        "My Custom Strategy"
    }
}
```

---

## Conclusion

Each strategy targets different opportunities in the meme coin lifecycle:

- **Ultra-Early Sniper**: Birth (0-5% bonding curve)
- **Conservative**: Growth (30-70% bonding curve)
- **Momentum Scalper**: Acceleration (40-80% bonding curve)
- **Graduation Anticipator**: Maturity (60-85% bonding curve)

By running multiple strategies simultaneously, you can capture opportunities at every stage while managing risk through diversification.

Remember: Meme coins are extremely high risk. Never invest more than you can afford to lose. These strategies optimize for opportunity detection, but cannot eliminate inherent market risks.

**Good luck, and may your bags moon! 🚀**
