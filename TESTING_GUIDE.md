# Curverider Vault - Smart Contract Testing Guide

## 🎯 Overview

This guide will help you build, deploy, and test the Curverider Vault Solana program.

## 📋 Prerequisites

The setup script will install:
- ✅ Rust and Cargo
- ✅ Solana CLI (v1.18+)
- ✅ Anchor Framework (v0.29.0)

## 🚀 Quick Start

### 1. Setup Environment

```bash
chmod +x setup.sh
./setup.sh
```

This will:
- Install Anchor CLI via AVM
- Configure Solana for localhost
- Generate keypair if needed

### 2. Start Local Validator

In a separate terminal:

```bash
solana-test-validator
```

Keep this running during testing.

### 3. Build & Test

```bash
chmod +x test.sh
./test.sh
```

Or manually:

```bash
anchor build
anchor test
```

## 🧪 Test Suite

The test suite covers:

### 1. **Vault Initialization** ✅
- Creates vault with configuration
- Sets min/max deposits
- Configures management and performance fees

### 2. **User Deposits** 💰
- First deposit (1:1 share ratio)
- Proportional share allocation
- Minimum deposit validation

### 3. **Trading Positions** 📈
- Opens positions with token mint
- Tracks entry/exit prices
- Calculates PnL
- Updates vault statistics

### 4. **Withdrawals** 💵
- Partial share withdrawals
- Proportional SOL redemption
- Insufficient shares validation

### 5. **Vault Configuration** ⚙️
- Updates deposit limits
- Modifies fee structure
- Fee validation (max 10% mgmt, 30% perf)

### 6. **Final Statistics** 📊
- Total deposited amount
- Share distribution
- Trade count and win rate
- Total PnL

## 📊 Expected Test Output

```
🔑 Test Setup Complete
Vault PDA: <address>
User 1: <address>
User 2: <address>

✅ Vault initialized
✅ User 1 deposited 2 SOL
✅ User 2 deposited 3 SOL
✅ Position opened
✅ Position closed with profit
✅ User 1 withdrew 50% shares
✅ Vault config updated

═══════════════════════════════════════
📊 FINAL VAULT STATISTICS
═══════════════════════════════════════
Total Deposited: 5.5 SOL
Total Shares: 2750000000
Total Trades: 1
Profitable Trades: 1
Total PnL: 1.5 SOL
Win Rate: 100.00%
═══════════════════════════════════════
```

## 🔧 Program Instructions

### Initialize Vault
```typescript
await program.methods
  .initializeVault(
    vaultBump,
    minDeposit,
    maxDeposit,
    managementFeeBps,
    performanceFeeBps
  )
  .accounts({
    vault: vaultPda,
    authority: authority.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Deposit
```typescript
await program.methods
  .deposit(amount)
  .accounts({
    vault: vaultPda,
    userAccount: userAccountPda,
    user: user.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([user])
  .rpc();
```

### Open Position
```typescript
await program.methods
  .openPosition(
    tokenMint,
    amountSol,
    entryPrice,
    takeProfitPrice,
    stopLossPrice
  )
  .accounts({
    vault: vaultPda,
    position: positionPda,
    authority: authority.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Close Position
```typescript
await program.methods
  .closePosition(exitPrice, amountReceived)
  .accounts({
    vault: vaultPda,
    position: positionPda,
    authority: authority.publicKey,
  })
  .rpc();
```

### Withdraw
```typescript
await program.methods
  .withdraw(sharesToBurn)
  .accounts({
    vault: vaultPda,
    userAccount: userAccountPda,
    user: user.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .signers([user])
  .rpc();
```

## 🔍 Account Structure

### Vault Account
```rust
pub struct Vault {
    pub authority: Pubkey,           // Vault manager
    pub vault_bump: u8,              // PDA bump
    pub total_deposited: u64,        // Total SOL in vault
    pub total_shares: u64,           // Total shares issued
    pub min_deposit: u64,            // Minimum deposit amount
    pub max_deposit: u64,            // Maximum deposit amount
    pub management_fee_bps: u16,     // Management fee (basis points)
    pub performance_fee_bps: u16,    // Performance fee (basis points)
    pub is_active: bool,             // Vault status
    pub accrued_fees: u64,           // Unclaimed fees
    pub total_trades: u64,           // Number of trades
    pub profitable_trades: u64,      // Winning trades
    pub total_pnl: i64,              // Total profit/loss
}
```

### UserAccount
```rust
pub struct UserAccount {
    pub owner: Pubkey,               // User wallet
    pub vault: Pubkey,               // Associated vault
    pub shares: u64,                 // User's shares
    pub total_deposited: u64,        // Lifetime deposits
    pub deposited_at: i64,           // First deposit timestamp
}
```

### Position
```rust
pub struct Position {
    pub vault: Pubkey,               // Associated vault
    pub token_mint: Pubkey,          // Token being traded
    pub amount_sol: u64,             // SOL invested
    pub entry_price: u64,            // Entry price
    pub current_price: u64,          // Current/exit price
    pub take_profit_price: u64,      // Take profit target
    pub stop_loss_price: u64,        // Stop loss level
    pub status: PositionStatus,      // Open/Closed
    pub opened_at: i64,              // Open timestamp
    pub closed_at: i64,              // Close timestamp
    pub pnl: i64,                    // Profit/loss
}
```

## 🚨 Error Codes

- `InsufficientShares` (6000): User doesn't have enough shares
- `BelowMinDeposit` (6001): Deposit below minimum
- `AboveMaxDeposit` (6002): Deposit exceeds maximum
- `VaultInactive` (6003): Vault is not active
- `Unauthorized` (6004): Not vault authority
- `FeeTooHigh` (6005): Fee exceeds maximum
- `InvalidPrice` (6006): Price is zero
- `PositionClosed` (6007): Position already closed
- `MathOverflow` (6008): Calculation overflow

## 🎯 Next Steps

1. **Run the tests** - Verify all instructions work
2. **Deploy to devnet** - `solana config set --url devnet`
3. **Integrate with bot** - Connect Rust bot to on-chain program
4. **Frontend integration** - Add wallet interactions
5. **Production deployment** - Deploy to mainnet

## 📚 Resources

- [Anchor Book](https://book.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)
- [Anchor Examples](https://github.com/coral-xyz/anchor/tree/master/tests)

## 🐛 Troubleshooting

### Test validator not starting?
```bash
solana-test-validator --reset
```

### Build errors?
```bash
cargo clean
anchor clean
anchor build
```

### Program ID mismatch?
Update `lib.rs` and `Anchor.toml` with:
```bash
solana address -k target/deploy/curverider_vault-keypair.json
```

## 🎉 Success!

Once tests pass, you'll have a fully functional Solana vault program ready for integration with your trading bot and frontend!
