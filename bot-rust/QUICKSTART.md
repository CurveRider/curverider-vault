# 🚀 Quick Start - Curverider Bot

## 5-Minute Setup Guide

### Step 1: Install Rust (if not installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version  # Verify installation
```

### Step 2: Configure
```bash
cd bot-rust
cp .env.example .env
```

Edit `.env` and add your private key:
```bash
WALLET_PRIVATE_KEY=your_base58_key_here
```

**Get your private key:**
```bash
# If you have Solana CLI:
solana-keygen show ~/.config/solana/id.json

# Or export from Phantom wallet (Settings -> Show Private Key)
```

### Step 3: Fund Your Wallet
```bash
# Check balance
solana balance YOUR_WALLET_ADDRESS

# Need at least 2-3 SOL for trading + fees
```

### Step 4: Build & Run
```bash
# Build (first time takes ~2-5 minutes)
cargo build --release

# Run the bot
RUST_LOG=info cargo run --release
```

## What You'll See

```
🚀 Starting Curverider Vault Bot
⚡ High-Performance Rust Trading Bot for pump.fun
═══════════════════════════════════════════════
✅ Configuration loaded
📊 Wallet: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
💰 Max position size: 1 SOL
✅ Bot initialized successfully
🔍 Starting main trading loop...

[Bot starts scanning for tokens...]
```

## ⚙️ Configuration Quick Reference

### Conservative (Recommended for First Run)
```bash
MAX_POSITION_SIZE_SOL=0.1          # Only risk 0.1 SOL per trade
MAX_CONCURRENT_POSITIONS=2         # Max 2 positions at once
TAKE_PROFIT_MULTIPLIER=2.0         # Exit at 2x (100% profit)
STOP_LOSS_PERCENTAGE=0.3           # Exit at 30% loss
```

### Moderate
```bash
MAX_POSITION_SIZE_SOL=0.5
MAX_CONCURRENT_POSITIONS=3
TAKE_PROFIT_MULTIPLIER=2.5
STOP_LOSS_PERCENTAGE=0.4
```

### Aggressive (High Risk!)
```bash
MAX_POSITION_SIZE_SOL=1.0
MAX_CONCURRENT_POSITIONS=5
TAKE_PROFIT_MULTIPLIER=3.0
STOP_LOSS_PERCENTAGE=0.5
```

## 🎯 First Trade Checklist

- [ ] Wallet has 2+ SOL balance
- [ ] RPC_URL is set (consider paid RPC for better speed)
- [ ] Position size is small (0.1 SOL recommended)
- [ ] Stop loss is configured
- [ ] Watching logs actively
- [ ] Ready to emergency stop (Ctrl+C)

## 📊 Monitoring

### Watch for These Signals
```
🎯 STRONG BUY SIGNAL DETECTED!     # Bot found a good token
🚀 Attempting to buy...            # Executing trade
✅ Position opened successfully!    # Trade complete
🎯 Take profit triggered           # Selling for profit
🛑 Stop loss triggered             # Cutting losses
```

### Check Status
Bot displays status every 10 iterations:
```
═══════════════════════════════════════════════
📊 BOT STATUS
═══════════════════════════════════════════════
🔓 Active Positions: 2/5
  1. EPjF...xyz - Entry: $0.001234, Held: 450s
  2. 7Np4...abc - Entry: $0.002468, Held: 125s
═══════════════════════════════════════════════
```

## 🛑 Emergency Stop

To stop the bot safely:
```bash
# Press Ctrl+C
# Bot will finish current operation and exit
```

## ⚠️ Common Issues

### "Failed to load wallet"
- Check your `WALLET_PRIVATE_KEY` is base58 encoded
- Ensure no spaces or newlines in the key

### "Insufficient funds"
- Add more SOL to your wallet
- Reduce `MAX_POSITION_SIZE_SOL`

### "RPC connection failed"
- Check your RPC_URL
- Consider using a paid RPC (Helius, QuickNode)
- Verify internet connection

### "No tokens found"
- Normal - bot is waiting for opportunities
- pump.fun might be slow
- Try adjusting `VOLUME_THRESHOLD_SOL`

## 📈 Performance Tips

### 1. Use Paid RPC
```bash
# Helius
RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_KEY

# QuickNode
RPC_URL=https://YOUR-ENDPOINT.solana-mainnet.quiknode.pro/YOUR_TOKEN/
```

### 2. Optimize Scan Interval
```bash
# Faster scanning (more aggressive)
SCAN_INTERVAL_MS=500

# Slower scanning (less resource intensive)
SCAN_INTERVAL_MS=2000
```

### 3. Adjust Thresholds
```bash
# More selective (fewer trades, higher quality)
VOLUME_THRESHOLD_SOL=20.0
HOLDER_COUNT_MIN=100

# Less selective (more trades, higher risk)
VOLUME_THRESHOLD_SOL=5.0
HOLDER_COUNT_MIN=30
```

## 🎓 Learning Mode

### Start Here
1. **Day 1-2**: Run with `MAX_POSITION_SIZE_SOL=0.05`
2. **Day 3-5**: Increase to `0.1` if comfortable
3. **Week 2**: Adjust thresholds based on results
4. **Week 3+**: Optimize for your risk tolerance

### Track Results
```bash
# Keep a simple log
echo "$(date) - Position opened - Token: XXX - Entry: $X.XX" >> my-trades.log
```

## 🚀 Next Steps

Once comfortable:
1. Review [README.md](README.md) for full documentation
2. Experiment with configuration parameters
3. Monitor performance over multiple days
4. Consider setting up monitoring/alerts
5. Optimize based on your results

---

## 💡 Pro Tips

- **Start small**: Better to miss gains than lose capital
- **Monitor actively**: Don't leave bot unattended initially
- **Review positions**: Check your open positions regularly
- **Learn patterns**: Note which signals work best
- **Be patient**: Not every scan finds a trade

---

**Happy Trading!** 🎯

*Remember: This is experimental software. Never risk more than you can afford to lose.*
