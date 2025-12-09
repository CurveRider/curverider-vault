# Non-Custodial Vault Architecture

## Overview

Users maintain full custody of their funds while delegating trading permissions to the bot. No funds are transferred to the vault - instead, the bot operates on user wallets through delegated permissions.

## How It Works

### 1. User Onboarding Flow

```
User Wallet → Create Delegation Account → Set Strategy Preferences → Bot Monitors
     ↓
  Funds stay in user's wallet
  Bot has permission to execute trades only
```

### 2. Delegation System

```rust
// On-chain account structure
pub struct DelegationAccount {
    pub user: Pubkey,              // User's wallet
    pub bot_authority: Pubkey,      // Bot's public key
    pub strategy: StrategyType,     // Selected strategy
    pub max_position_size: u64,     // Max SOL per trade
    pub max_concurrent_trades: u8,  // Max open positions
    pub is_active: bool,            // Can bot trade?
    pub created_at: i64,
    pub total_trades: u64,
    pub total_pnl: i64,
}
```

### 3. Trading Flow

```
1. Bot scans pump.fun for opportunities
2. Bot finds signal matching user's strategy
3. Bot checks delegation permissions
4. Bot executes trade via CPI (Cross-Program Invocation)
   - Uses user's SOL from their wallet
   - Bot signs with delegated authority
   - Trade executes on user's behalf
5. Profits/losses stay in user's wallet
```

## Key Benefits

✅ **Non-Custodial**: Users keep full control of funds
✅ **Revocable**: Users can revoke delegation anytime
✅ **Transparent**: All trades on-chain and auditable
✅ **No License Needed**: Not holding customer funds
✅ **Granular Control**: Users set limits per strategy
✅ **No Withdrawal Risk**: Bot can't withdraw, only trade

## Smart Contract Updates Needed

### New Instructions

1. `create_delegation` - User creates delegation account
2. `update_delegation` - User modifies settings
3. `revoke_delegation` - User disables bot access
4. `execute_trade` - Bot executes trade on behalf of user
5. `close_trade` - Bot closes position

### Security Features

- **Position Limits**: Max SOL per trade, max concurrent trades
- **Time Locks**: Optional cooldown periods
- **Strategy Lock**: Bot can only use selected strategy
- **Emergency Stop**: User can pause at any time
- **Audit Trail**: All actions logged on-chain

## Frontend Changes

### User Dashboard
- Connect wallet
- View balance (stays in wallet)
- Create/modify delegation
- Select strategy (Conservative, Ultra-Early, etc.)
- Set position limits
- View active positions
- View trade history
- Revoke access button

### Real-time Updates
- WebSocket connection to bot
- Live position monitoring
- PnL updates
- Strategy performance metrics

## Bot Changes

### HTTP API Endpoints
- `GET /api/health` - Bot status
- `GET /api/strategies` - Available strategies
- `GET /api/users/:wallet/positions` - User's positions
- `GET /api/users/:wallet/stats` - Performance stats
- `POST /api/delegation` - Register delegation (webhook)
- `WS /api/stream` - Real-time updates

### Bot Architecture
```
┌─────────────────┐
│  Scanner        │ → Scans pump.fun
│  (unchanged)    │
└────────┬────────┘
         │ signals
         ▼
┌─────────────────┐
│  Strategy       │ → Analyzes opportunities
│  Analyzer       │
└────────┬────────┘
         │ trading signals
         ▼
┌─────────────────┐
│  Delegation     │ → Matches users by strategy
│  Manager (new)  │   Checks permissions/limits
└────────┬────────┘
         │ filtered signals
         ▼
┌─────────────────┐
│  Trade Executor │ → Executes on behalf of users
│  (modified)     │   Uses CPI with delegation
└─────────────────┘
         │
         ▼
   User's Wallet (funds stay here)
```

## Deployment Strategy

### Phase 1: Local Testing
1. Deploy updated smart contract to devnet
2. Run bot locally with test wallets
3. Test delegation + revocation
4. Test trade execution

### Phase 2: Railway Deployment
1. Deploy bot to Railway
2. Configure environment variables
3. Set up monitoring/logging
4. Test with small amounts

### Phase 3: Frontend Integration
1. Add delegation UI
2. Connect to bot API
3. Real-time position tracking
4. Deploy frontend (Vercel)

### Phase 4: Mainnet
1. Audit smart contracts
2. Gradual rollout (whitelist)
3. Monitor for issues
4. Public launch

## Legal Considerations

### Why This Avoids Licensing

✅ **No Custody**: Never hold user funds
✅ **User Authorization**: Each trade requires on-chain permission
✅ **Revocable**: Users maintain control
✅ **Software Service**: Providing automated trading software, not financial advice
✅ **Open Source**: Code is auditable

### Terms of Service Should Include

- "Software provided as-is"
- "Users maintain custody of funds"
- "No financial advice provided"
- "High risk of loss"
- "Use at your own risk"

## Cost Structure

Since funds stay in user wallets, you can charge:
- **Subscription fee**: Monthly access to bot (e.g., $50/mo)
- **Performance fee**: Small % of profitable trades (e.g., 5-10%)
- **Free tier**: Limited to 1 strategy or smaller position sizes

Performance fees can be:
1. Voluntary (honor system)
2. On-chain via smart contract (auto-deducted on profitable closes)
3. Platform fee address receives portion of trade

## Security Measures

1. **Rate Limiting**: Prevent abuse of delegation
2. **Position Sizing**: Hard caps per user tier
3. **Circuit Breakers**: Auto-pause on anomalies
4. **Audit Logging**: All actions recorded
5. **Open Source**: Community auditing

## Next Steps

1. Update smart contract with delegation system
2. Add HTTP API to Rust bot
3. Create delegation management UI
4. Deploy to Railway
5. Test end-to-end flow
6. Document deployment process
