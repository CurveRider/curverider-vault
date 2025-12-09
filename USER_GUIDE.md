# 👤 Curverider Vault - User Guide

## What is Curverider Vault?

Curverider Vault is a **non-custodial automated trading system** for Solana meme coins on pump.fun.

### Key Features
- ✅ **Non-Custodial**: You keep full control of your funds
- ✅ **Automated**: Bot trades 24/7 on your behalf
- ✅ **Multiple Strategies**: Choose your risk level
- ✅ **Transparent**: All trades on-chain and auditable
- ✅ **Revocable**: Stop the bot anytime

---

## Getting Started

### Step 1: Connect Your Wallet

1. Visit [your-frontend-url.vercel.app](https://your-frontend-url.vercel.app)
2. Click "Connect Wallet"
3. Select your Solana wallet (Phantom, Solflare, etc.)
4. Approve the connection

**Requirements:**
- Minimum 1 SOL in wallet (for trading + fees)
- Solana wallet installed (Phantom recommended)

### Step 2: Choose Your Strategy

We offer 4 trading strategies with different risk/reward profiles:

#### 🛡️ Conservative (Recommended for Beginners)
- **Risk Level**: Medium
- **Target Return**: 2x (100% gain)
- **Win Rate**: 60-70%
- **Hold Time**: ~1 hour
- **Best For**: Consistent, balanced returns

#### 🎯 Ultra-Early Sniper (High Risk/Reward)
- **Risk Level**: Very High
- **Target Return**: 3-10x (200-900% gain)
- **Win Rate**: 30-40%
- **Hold Time**: ~10 minutes
- **Best For**: Moonshot hunting, high risk tolerance

#### ⚡ Momentum Scalper (Active Trading)
- **Risk Level**: High
- **Target Return**: 1.5x (50% gain)
- **Win Rate**: 50-60%
- **Hold Time**: ~30 minutes
- **Best For**: Quick flips, active trading

#### 🎓 Graduation Anticipator (Low Risk)
- **Risk Level**: Low
- **Target Return**: 1.8x (80% gain)
- **Win Rate**: 70-80%
- **Hold Time**: ~2 hours
- **Best For**: Risk-averse, steady gains

### Step 3: Set Your Limits

Configure your risk parameters:

**Max Position Size:**
- How much SOL the bot can use per trade
- Recommended: 0.1 - 0.5 SOL for beginners
- Maximum: 10 SOL

**Max Concurrent Trades:**
- How many positions can be open at once
- Recommended: 2-3 for beginners
- Maximum: 10

**Example Setup:**
```
Strategy: Conservative
Max Position Size: 0.5 SOL
Max Concurrent Trades: 3

Total Capital at Risk: 1.5 SOL
```

### Step 4: Create Delegation

1. Review your settings
2. Click "Create Delegation"
3. Approve the transaction in your wallet
4. Wait for confirmation

**What Happens:**
- An on-chain delegation account is created
- The bot gains permission to trade on your behalf
- Your funds stay in YOUR wallet
- You can revoke access anytime

### Step 5: Monitor Your Trades

The bot will now:
1. Scan pump.fun for opportunities
2. Analyze tokens using your selected strategy
3. Open positions when it finds good signals
4. Close positions at take-profit or stop-loss
5. All profits/losses go directly to your wallet

---

## Understanding Your Dashboard

### Stats Overview
- **Active Positions**: Current open trades
- **Total Trades**: All trades executed
- **Win Rate**: Percentage of profitable trades
- **Total P&L**: Your profit/loss in SOL

### Position Table
Each position shows:
- **Token**: Name and address
- **Amount**: SOL invested
- **Entry/Current Price**: Buy price vs current price
- **P&L**: Profit/Loss in SOL and %
- **Status**: Open, Closed, or Liquidated
- **Opened**: When the trade started

### Real-Time Updates
- Dashboard updates every 5 seconds
- Live P&L calculations
- Instant notifications on new trades

---

## How It Works (Technical)

### 1. Token Discovery
Bot scans pump.fun API every second for new tokens and trending coins.

### 2. Analysis
Each token is analyzed using multiple factors:
- Volume trends and acceleration
- Liquidity depth
- Holder distribution
- Price momentum
- Buy/sell pressure
- Bonding curve progress

### 3. Signal Generation
If a token scores high enough (based on your strategy), a trading signal is generated.

### 4. Trade Execution
Bot checks:
- ✅ Do you have an active delegation?
- ✅ Is your position limit reached?
- ✅ Do you have enough SOL?

If all checks pass → Trade executes from your wallet

### 5. Position Management
Bot monitors open positions:
- Takes profit when target hit (e.g., 2x for Conservative)
- Stops loss if price drops too much
- Times out if position held too long
- Momentum Scalper uses trailing stops

---

## Managing Risk

### Start Small
```
Beginner Setup:
- Strategy: Conservative or Graduation
- Position Size: 0.1 - 0.2 SOL
- Concurrent Trades: 2-3
- Total Risk: ~0.5 SOL
```

### Diversify Strategies
Advanced users can run multiple strategies:
- 50% Conservative (stable gains)
- 30% Graduation (lower risk)
- 20% Ultra-Early (moonshots)

### Set Limits
**Never risk more than you can afford to lose**
- Meme coins are extremely volatile
- Even good strategies have losing trades
- The bot cannot predict rugs or scams

### Monitor Regularly
- Check dashboard daily
- Review trade history
- Adjust limits based on performance

---

## Common Questions

### Q: Can the bot withdraw my SOL?
**A:** NO. The bot only has permission to trade. Your funds stay in your wallet at all times.

### Q: How do I stop the bot?
**A:** Click "Revoke Delegation" in your dashboard. The bot will immediately stop opening new positions.

### Q: What happens to open positions when I revoke?
**A:** The bot will continue to manage existing positions until they hit take-profit, stop-loss, or timeout. You can also manually close them.

### Q: How much can I make?
**A:** It depends on:
- Your chosen strategy
- Market conditions
- Your risk settings
- Luck

Past performance doesn't guarantee future results.

### Q: What are the fees?
**A:**
- Solana transaction fees: ~0.00001 SOL per transaction
- Optional performance fee: [Your fee structure]
- No withdrawal fees (funds stay in your wallet)

### Q: Is this safe?
**A:**
- ✅ Non-custodial (you keep funds)
- ✅ Open source (code is auditable)
- ✅ On-chain (all trades transparent)
- ⚠️ Meme coins are inherently risky
- ⚠️ Software may have bugs
- ⚠️ Market conditions can change rapidly

**Use at your own risk. Never invest more than you can afford to lose.**

### Q: Which strategy should I choose?
**A:**
- **New to trading?** → Conservative or Graduation
- **High risk tolerance?** → Ultra-Early Sniper
- **Active trader?** → Momentum Scalper
- **Risk-averse?** → Graduation Anticipator

You can change strategies anytime.

### Q: Why didn't the bot execute a trade?
**A:**
- No signals matched your strategy criteria
- You reached position limit
- Insufficient SOL in wallet
- Token didn't meet minimum requirements

### Q: Can I manually close a position?
**A:** Not through the UI currently, but you can revoke delegation and the bot will close positions at the next trigger point.

---

## Tips for Success

### 1. Understand Your Strategy
Read the full strategy descriptions. Know what bonding curve range and risk level you're targeting.

### 2. Start Conservative
Begin with lower risk strategies and small position sizes. Scale up as you gain confidence.

### 3. Be Patient
The bot may not trade for hours if no good opportunities exist. This is normal and good - quality over quantity.

### 4. Check Market Conditions
Meme coin markets have cycles:
- **Bull market**: More opportunities, higher success rate
- **Bear market**: Fewer signals, more caution needed

### 5. Review Performance
After 10-20 trades, review your stats:
- Is win rate meeting expectations?
- Are losses being limited by stop-losses?
- Should you adjust position sizes?

### 6. Don't Overtrade
More trades ≠ more profit. Trust the strategy.

### 7. Keep SOL in Wallet
Maintain extra SOL for:
- New opportunities
- Transaction fees
- Market volatility

---

## Troubleshooting

### "Insufficient funds" error
- Check wallet SOL balance
- Ensure you have more than max position size
- Account for transaction fees (~0.001 SOL buffer)

### "Delegation not found" error
- Create a delegation first
- Check you're connected with correct wallet
- Refresh the page

### Positions not showing
- Wait 5 seconds for refresh
- Check you have active delegation
- Verify bot is running (check API health)

### Bot not trading
- No good signals found (normal)
- At position limit (adjust settings)
- Check market activity on pump.fun

---

## Safety & Security

### Protect Your Wallet
- Never share your seed phrase
- Use hardware wallet for large amounts
- Create a dedicated trading wallet
- Keep backup of recovery phrase

### Verify Before Signing
- Always check transaction details
- Confirm you're on the official site
- Verify contract addresses

### Red Flags
If you see any of these, DO NOT proceed:
- ❌ Site asking for seed phrase
- ❌ Unusual transaction permissions
- ❌ Requests to send SOL directly
- ❌ Promises of guaranteed returns

---

## Getting Help

### Resources
- Documentation: [Link to docs]
- Discord: [Your Discord]
- Twitter: [Your Twitter]
- Email: support@your-domain.com

### Reporting Issues
Include:
- Your wallet address
- Strategy being used
- Description of issue
- Screenshots if relevant

---

## Terms & Disclaimers

### Use at Your Own Risk
- This is experimental software
- Meme coins are extremely high risk
- You can lose all invested capital
- No guarantees of profit
- Past performance ≠ future results

### Not Financial Advice
- This is a software tool
- We don't provide investment advice
- Do your own research
- Consult a financial advisor

### Your Responsibility
- You maintain custody of funds
- You set all risk parameters
- You monitor your positions
- You decide when to stop

---

## Ready to Start?

1. ✅ Connect Wallet
2. ✅ Choose Strategy
3. ✅ Set Limits
4. ✅ Create Delegation
5. ✅ Monitor & Adjust

**Trade smart. Trade safe. Good luck! 🚀**
