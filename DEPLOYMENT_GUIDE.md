# 🚀 Curverider Vault - Complete Deployment Guide

## Overview

This guide walks you through deploying the complete non-custodial Curverider Vault system:
1. **Smart Contract** (Solana)
2. **Rust Trading Bot** (Railway)
3. **Frontend** (Vercel)

---

## Prerequisites

### Required Tools
- Node.js 18+ and npm
- Rust 1.75+
- Solana CLI tools
- Anchor CLI 0.30.0+
- Git
- Railway CLI (for bot deployment)
- Vercel CLI (for frontend deployment)

### Required Accounts
- Solana wallet with SOL (devnet or mainnet)
- Railway account (free tier works)
- Vercel account (free tier works)
- Helius or QuickNode RPC account (recommended for production)

---

## Part 1: Smart Contract Deployment

### Step 1: Build the Smart Contract

```bash
cd programs/curverider-vault

# Switch to the non-custodial version
cp src/lib_noncustodial.rs src/lib.rs

# Build
anchor build

# Get the program ID
solana address -k target/deploy/curverider_vault-keypair.json
```

### Step 2: Update Program ID

Update the program ID in:
- `programs/curverider-vault/src/lib.rs` (line 3)
- `Anchor.toml`

```rust
// In lib.rs
declare_id!("YOUR_PROGRAM_ID_HERE");
```

```toml
# In Anchor.toml
[programs.devnet]
curverider_vault = "YOUR_PROGRAM_ID_HERE"

[programs.mainnet]
curverider_vault = "YOUR_PROGRAM_ID_HERE"
```

### Step 3: Deploy to Devnet (Testing)

```bash
# Set Solana to devnet
solana config set --url devnet

# Airdrop SOL for deployment (devnet only)
solana airdrop 2

# Deploy
anchor deploy

# Verify deployment
solana program show YOUR_PROGRAM_ID
```

### Step 4: Deploy to Mainnet (Production)

```bash
# Set Solana to mainnet
solana config set --url mainnet-beta

# Ensure you have enough SOL (~5 SOL for deployment)
solana balance

# Deploy
anchor deploy

# Verify
solana program show YOUR_PROGRAM_ID
```

**⚠️ IMPORTANT:** After mainnet deployment:
- Run comprehensive tests
- Start with small position limits
- Monitor closely for the first week
- Consider a security audit

---

## Part 2: Rust Bot Deployment to Railway

### Step 1: Prepare the Bot

```bash
cd bot-rust

# Ensure Dockerfile exists
ls Dockerfile railway.json

# Test build locally (optional)
docker build -t curverider-bot .
docker run --env-file .env curverider-bot
```

### Step 2: Create Railway Project

1. Go to [railway.app](https://railway.app)
2. Click "New Project"
3. Select "Deploy from GitHub repo"
4. Connect your GitHub and select `curverider-vault` repo
5. Choose the `bot-rust` directory as the root

### Step 3: Configure Environment Variables

In Railway project settings, add these variables:

```bash
# Solana Configuration
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
# Or use paid RPC:
# SOLANA_RPC_URL=https://mainnet.helius-rpc.com/?api-key=YOUR_KEY

# Bot Wallet (KEEP SECRET!)
WALLET_PRIVATE_KEY=your_bot_wallet_private_key_base58

# Program Configuration
PROGRAM_ID=YOUR_DEPLOYED_PROGRAM_ID

# Strategy
STRATEGY_TYPE=conservative

# Limits
MAX_POSITION_SIZE_SOL=0.5
MAX_CONCURRENT_POSITIONS=3

# Risk Management
VOLUME_THRESHOLD_SOL=10.0
MIN_LIQUIDITY_SOL=5.0
HOLDER_COUNT_MIN=50
MIN_CONFIDENCE_SCORE=0.80

# API
API_PORT=8080
API_ENABLE=true

# Logging
RUST_LOG=info

# Scanning
SCAN_INTERVAL_MS=1000
PUMP_FUN_API_URL=https://frontend-api.pump.fun
```

### Step 4: Deploy

```bash
# Using Railway CLI
railway login
railway link
railway up

# Or deploy via Railway dashboard
# Push to GitHub and Railway will auto-deploy
```

### Step 5: Get Bot URL

```bash
# Get the public URL
railway domain

# Your bot will be available at:
# https://your-app.railway.app
```

### Step 6: Verify Bot is Running

```bash
# Check health endpoint
curl https://your-app.railway.app/api/health

# Expected response:
# {"status":"healthy","version":"0.1.0","uptime_seconds":123}
```

---

## Part 3: Frontend Deployment to Vercel

### Step 1: Configure Environment Variables

Create `frontend/.env.local`:

```bash
# Bot API URL (from Railway)
NEXT_PUBLIC_BOT_API_URL=https://your-bot.railway.app

# Solana Network
NEXT_PUBLIC_SOLANA_NETWORK=mainnet-beta
# Or for testing:
# NEXT_PUBLIC_SOLANA_NETWORK=devnet

# Program ID
NEXT_PUBLIC_PROGRAM_ID=YOUR_DEPLOYED_PROGRAM_ID

# RPC URL (optional, uses default if not set)
NEXT_PUBLIC_SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
```

### Step 2: Test Locally

```bash
cd frontend

# Install dependencies
npm install

# Run dev server
npm run dev

# Visit http://localhost:3000
# Test wallet connection and delegation creation
```

### Step 3: Deploy to Vercel

```bash
# Install Vercel CLI
npm install -g vercel

# Login
vercel login

# Deploy
vercel

# Follow prompts:
# - Link to existing project or create new
# - Confirm settings
# - Deploy

# For production deployment
vercel --prod
```

### Step 4: Configure Vercel Environment Variables

In Vercel dashboard:
1. Go to Project Settings
2. Environment Variables
3. Add the same variables from `.env.local`
4. Redeploy

---

## Part 4: Post-Deployment Configuration

### Set Up Bot Authority

The bot needs to be registered as an authority in your system:

```bash
# In your wallet, note the bot's public key
# This is derived from WALLET_PRIVATE_KEY

# Users will create delegations pointing to this bot authority
```

### Test End-to-End Flow

1. **Connect Wallet** to frontend
2. **Create Delegation**:
   - Select strategy
   - Set position limits
   - Confirm transaction
3. **Monitor Bot**:
   - Check Railway logs
   - Verify bot detects your delegation
4. **Wait for Trades**:
   - Bot scans pump.fun
   - Opens positions when signals match
5. **View Positions**:
   - Real-time updates on frontend
   - P&L tracking

---

## Part 5: Monitoring & Maintenance

### Monitor Bot Logs (Railway)

```bash
# View live logs
railway logs

# Or in Railway dashboard
# Project > Deployments > View Logs
```

### Monitor Bot Performance

```bash
# Check bot stats
curl https://your-bot.railway.app/api/stats

# Monitor positions
curl https://your-bot.railway.app/api/positions
```

### Set Up Alerts

**Railway:**
- Enable deployment notifications
- Set up usage alerts
- Monitor resource usage

**Wallet Monitoring:**
```bash
# Check bot wallet balance regularly
solana balance YOUR_BOT_PUBLIC_KEY

# Set up alerts if balance drops below threshold
```

### Update Bot Configuration

To update bot settings without redeployment:

1. Go to Railway dashboard
2. Environment Variables
3. Update values (e.g., `MAX_POSITION_SIZE_SOL`)
4. Restart the service

---

## Part 6: Security Best Practices

### Bot Wallet Security

```bash
# Create dedicated bot wallet
solana-keygen new --outfile ~/bot-wallet.json

# Fund with limited amount (e.g., 10-50 SOL)
solana transfer YOUR_BOT_PUBKEY 10 --allow-unfunded-recipient

# Store private key securely in Railway secrets
# NEVER commit to git
```

### Smart Contract Security

- ✅ Audit contract before mainnet deployment
- ✅ Start with conservative limits
- ✅ Test thoroughly on devnet
- ✅ Monitor all transactions
- ✅ Have emergency pause mechanism

### API Security

- ✅ Enable CORS only for your frontend domain
- ✅ Rate limit API endpoints
- ✅ Monitor for abuse
- ✅ Use HTTPS only
- ✅ Validate all inputs

---

## Part 7: Scaling & Optimization

### Bot Performance Tuning

```bash
# Reduce scan interval for faster detection
SCAN_INTERVAL_MS=500

# Use paid RPC for better performance
SOLANA_RPC_URL=https://your-premium-rpc.com

# Increase parallel analysis
MAX_PARALLEL_ANALYSIS=20
```

### Multiple Strategy Bots

Deploy separate Railway instances for different strategies:

```bash
# Bot 1: Conservative (main capital)
STRATEGY_TYPE=conservative
MAX_POSITION_SIZE_SOL=1.0

# Bot 2: Ultra-Early Sniper (moonshot hunting)
STRATEGY_TYPE=ultra_early_sniper
MAX_POSITION_SIZE_SOL=0.1

# Bot 3: Graduation (stable gains)
STRATEGY_TYPE=graduation_anticipator
MAX_POSITION_SIZE_SOL=0.5
```

### Frontend Performance

```bash
# Enable caching in Vercel
# Add to vercel.json
{
  "headers": [
    {
      "source": "/api/:path*",
      "headers": [
        { "key": "Cache-Control", "value": "s-maxage=10, stale-while-revalidate" }
      ]
    }
  ]
}
```

---

## Part 8: Troubleshooting

### Bot Not Starting

```bash
# Check Railway logs
railway logs

# Common issues:
# - Invalid private key format
# - RPC connection failed
# - Program ID mismatch
```

### No Trades Executing

```bash
# Check bot is finding signals
curl https://your-bot.railway.app/api/stats

# Verify:
# - Bot is running (is_running: true)
# - Tokens are being analyzed
# - Signals are being generated

# Check pump.fun API is accessible
curl https://frontend-api.pump.fun/coins
```

### Frontend Not Connecting

```bash
# Check browser console
# Verify:
# - NEXT_PUBLIC_BOT_API_URL is correct
# - Bot API is accessible
# - CORS is configured

# Test API directly
curl https://your-bot.railway.app/api/health
```

### Transaction Failures

```bash
# Check wallet has enough SOL
solana balance

# Check program is deployed
solana program show YOUR_PROGRAM_ID

# Verify delegation account exists
solana account YOUR_DELEGATION_ADDRESS
```

---

## Part 9: Cost Estimation

### Railway Costs
- **Hobby Plan**: $5/month (512MB RAM, 1GB storage)
- **Pro Plan**: $20/month (8GB RAM, 100GB storage)
- Estimated: **$5-20/month** depending on usage

### Solana Costs
- **Program Deployment**: ~5 SOL one-time (~$500)
- **Transaction Fees**: ~0.00001 SOL per transaction (~$0.001)
- **Rent**: ~0.002 SOL per account (~$0.20)
- Estimated: **$10-50/month** for transaction fees

### RPC Costs
- **Free Tier**: Rate limited, OK for testing
- **Paid (Helius/QuickNode)**: $50-200/month
- Estimated: **$0-200/month**

### Total: **$15-270/month**

---

## Part 10: Going Live Checklist

### Pre-Launch
- [ ] Smart contract audited
- [ ] Deployed to devnet and tested
- [ ] Bot running on Railway with test wallet
- [ ] Frontend deployed to Vercel
- [ ] End-to-end flow tested
- [ ] Documentation complete
- [ ] Terms of service ready
- [ ] Support channels set up

### Launch Day
- [ ] Deploy contract to mainnet
- [ ] Update bot with mainnet program ID
- [ ] Update frontend with mainnet settings
- [ ] Fund bot wallet with initial capital
- [ ] Announce launch
- [ ] Monitor closely

### Post-Launch (First Week)
- [ ] Check logs daily
- [ ] Monitor all transactions
- [ ] Respond to user feedback
- [ ] Adjust parameters as needed
- [ ] Track performance metrics
- [ ] Document issues and fixes

---

## Support & Resources

### Documentation
- [Solana Docs](https://docs.solana.com/)
- [Anchor Book](https://book.anchor-lang.com/)
- [Railway Docs](https://docs.railway.app/)
- [Vercel Docs](https://vercel.com/docs)

### Community
- [Solana Discord](https://discord.gg/solana)
- [Anchor Discord](https://discord.gg/anchor)

### Monitoring Tools
- [Solana Explorer](https://explorer.solana.com/)
- [Solscan](https://solscan.io/)
- [Railway Dashboard](https://railway.app/dashboard)

---

## Conclusion

You now have a complete non-custodial trading vault system deployed!

**Remember:**
- Start small and scale gradually
- Monitor everything closely
- Users maintain custody of their funds
- No financial licenses required (software service only)
- Always include risk disclosures

**Good luck and trade responsibly! 🚀💰**
