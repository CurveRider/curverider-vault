use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Non-Custodial Trading Vault Program
/// Users maintain custody of funds while delegating trading permissions to bot
#[program]
pub mod curverider_vault {
    use super::*;

    /// Create a delegation account allowing bot to trade on user's behalf
    pub fn create_delegation(
        ctx: Context<CreateDelegation>,
        strategy: u8,
        max_position_size_sol: u64,
        max_concurrent_trades: u8,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;

        require!(max_position_size_sol > 0, VaultError::InvalidAmount);
        require!(max_concurrent_trades > 0 && max_concurrent_trades <= 10, VaultError::InvalidAmount);
        require!(strategy < 4, VaultError::InvalidStrategy); // 0-3 for 4 strategies

        delegation.user = ctx.accounts.user.key();
        delegation.bot_authority = ctx.accounts.bot_authority.key();
        delegation.strategy = strategy;
        delegation.max_position_size_sol = max_position_size_sol;
        delegation.max_concurrent_trades = max_concurrent_trades;
        delegation.is_active = true;
        delegation.active_trades = 0;
        delegation.total_trades = 0;
        delegation.profitable_trades = 0;
        delegation.total_pnl = 0;
        delegation.created_at = Clock::get()?.unix_timestamp;
        delegation.last_trade_at = 0;

        msg!("✅ Delegation created!");
        msg!("User: {}", delegation.user);
        msg!("Strategy: {}", strategy_name(strategy));
        msg!("Max position: {} SOL", max_position_size_sol);
        msg!("Max concurrent: {}", max_concurrent_trades);

        Ok(())
    }

    /// Update delegation settings
    pub fn update_delegation(
        ctx: Context<UpdateDelegation>,
        strategy: Option<u8>,
        max_position_size_sol: Option<u64>,
        max_concurrent_trades: Option<u8>,
        is_active: Option<bool>,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;

        if let Some(strat) = strategy {
            require!(strat < 4, VaultError::InvalidStrategy);
            delegation.strategy = strat;
            msg!("Strategy updated to: {}", strategy_name(strat));
        }

        if let Some(max_pos) = max_position_size_sol {
            require!(max_pos > 0, VaultError::InvalidAmount);
            delegation.max_position_size_sol = max_pos;
            msg!("Max position updated to: {} SOL", max_pos);
        }

        if let Some(max_trades) = max_concurrent_trades {
            require!(max_trades > 0 && max_trades <= 10, VaultError::InvalidAmount);
            delegation.max_concurrent_trades = max_trades;
            msg!("Max concurrent updated to: {}", max_trades);
        }

        if let Some(active) = is_active {
            delegation.is_active = active;
            msg!("Delegation active: {}", active);
        }

        Ok(())
    }

    /// Revoke delegation - immediately stops bot from trading
    pub fn revoke_delegation(
        ctx: Context<RevokeDelegation>,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;

        delegation.is_active = false;

        msg!("🛑 Delegation revoked!");
        msg!("User: {}", delegation.user);
        msg!("Active trades remaining: {}", delegation.active_trades);

        Ok(())
    }

    /// Bot opens a trading position on behalf of user
    pub fn open_position(
        ctx: Context<OpenPosition>,
        token_mint: Pubkey,
        amount_sol: u64,
        entry_price: u64,
        take_profit_price: u64,
        stop_loss_price: u64,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        let position = &mut ctx.accounts.position;

        // Validate delegation is active
        require!(delegation.is_active, VaultError::DelegationNotActive);

        // Check position limits
        require!(
            delegation.active_trades < delegation.max_concurrent_trades,
            VaultError::MaxTradesReached
        );
        require!(
            amount_sol <= delegation.max_position_size_sol,
            VaultError::PositionTooLarge
        );

        // Validate user has enough SOL
        let user_balance = ctx.accounts.user.lamports();
        require!(user_balance >= amount_sol, VaultError::InsufficientFunds);

        // Initialize position
        position.delegation = delegation.key();
        position.user = delegation.user;
        position.token_mint = token_mint;
        position.amount_sol = amount_sol;
        position.entry_price = entry_price;
        position.current_price = entry_price;
        position.take_profit_price = take_profit_price;
        position.stop_loss_price = stop_loss_price;
        position.status = PositionStatus::Open as u8;
        position.opened_at = Clock::get()?.unix_timestamp;
        position.closed_at = 0;
        position.pnl = 0;

        // Update delegation stats
        delegation.active_trades = delegation.active_trades.checked_add(1).unwrap();
        delegation.total_trades = delegation.total_trades.checked_add(1).unwrap();
        delegation.last_trade_at = Clock::get()?.unix_timestamp;

        msg!("📈 Position opened!");
        msg!("User: {}", delegation.user);
        msg!("Token: {}", token_mint);
        msg!("Amount: {} SOL", amount_sol);
        msg!("Entry: {}, TP: {}, SL: {}", entry_price, take_profit_price, stop_loss_price);

        Ok(())
    }

    /// Bot closes a trading position
    pub fn close_position(
        ctx: Context<ClosePosition>,
        exit_price: u64,
        amount_received: u64,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;
        let position = &mut ctx.accounts.position;

        // Validate position state
        require!(
            position.status == PositionStatus::Open as u8,
            VaultError::PositionNotOpen
        );
        require!(
            position.delegation == delegation.key(),
            VaultError::InvalidPosition
        );

        // Calculate PnL
        let pnl = (amount_received as i64)
            .checked_sub(position.amount_sol as i64)
            .unwrap();

        // Update position
        position.current_price = exit_price;
        position.status = PositionStatus::Closed as u8;
        position.closed_at = Clock::get()?.unix_timestamp;
        position.pnl = pnl;

        // Update delegation stats
        delegation.active_trades = delegation.active_trades.checked_sub(1).unwrap();
        delegation.total_pnl = delegation.total_pnl.checked_add(pnl).unwrap();

        if pnl > 0 {
            delegation.profitable_trades = delegation.profitable_trades.checked_add(1).unwrap();
        }

        msg!("📊 Position closed!");
        msg!("Exit price: {}", exit_price);
        msg!("PnL: {} lamports", pnl);
        msg!("User total PnL: {}", delegation.total_pnl);

        Ok(())
    }

    /// Get delegation statistics
    pub fn get_delegation_stats(
        ctx: Context<GetDelegationStats>,
    ) -> Result<()> {
        let delegation = &ctx.accounts.delegation;

        msg!("📊 Delegation Stats");
        msg!("User: {}", delegation.user);
        msg!("Strategy: {}", strategy_name(delegation.strategy));
        msg!("Active: {}", delegation.is_active);
        msg!("Active trades: {}/{}", delegation.active_trades, delegation.max_concurrent_trades);
        msg!("Total trades: {}", delegation.total_trades);
        msg!("Profitable: {}", delegation.profitable_trades);
        msg!("Total PnL: {} lamports", delegation.total_pnl);

        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct DelegationAccount {
    /// User's wallet public key
    pub user: Pubkey,
    /// Bot's authority public key
    pub bot_authority: Pubkey,
    /// Selected strategy (0=Conservative, 1=UltraEarly, 2=Momentum, 3=Graduation)
    pub strategy: u8,
    /// Maximum SOL per position
    pub max_position_size_sol: u64,
    /// Maximum concurrent open trades
    pub max_concurrent_trades: u8,
    /// Whether bot can currently trade
    pub is_active: bool,
    /// Current number of open positions
    pub active_trades: u8,
    /// Total number of trades executed
    pub total_trades: u64,
    /// Number of profitable trades
    pub profitable_trades: u64,
    /// Total profit/loss in lamports
    pub total_pnl: i64,
    /// Timestamp of delegation creation
    pub created_at: i64,
    /// Timestamp of last trade
    pub last_trade_at: i64,
}

#[account]
pub struct Position {
    /// Delegation account that owns this position
    pub delegation: Pubkey,
    /// User's wallet
    pub user: Pubkey,
    /// Token mint address
    pub token_mint: Pubkey,
    /// Amount of SOL invested
    pub amount_sol: u64,
    /// Entry price
    pub entry_price: u64,
    /// Current/exit price
    pub current_price: u64,
    /// Take profit target
    pub take_profit_price: u64,
    /// Stop loss target
    pub stop_loss_price: u64,
    /// Position status (0=Open, 1=Closed, 2=Liquidated)
    pub status: u8,
    /// When position was opened
    pub opened_at: i64,
    /// When position was closed
    pub closed_at: i64,
    /// Profit/loss in lamports
    pub pnl: i64,
}

#[repr(u8)]
pub enum PositionStatus {
    Open = 0,
    Closed = 1,
    Liquidated = 2,
}

// ============================================================================
// Context Structures
// ============================================================================

#[derive(Accounts)]
pub struct CreateDelegation<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + std::mem::size_of::<DelegationAccount>(),
        seeds = [b"delegation", user.key().as_ref()],
        bump
    )]
    pub delegation: Account<'info, DelegationAccount>,

    /// CHECK: Bot's public key for validation
    pub bot_authority: AccountInfo<'info>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDelegation<'info> {
    #[account(
        mut,
        seeds = [b"delegation", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub delegation: Account<'info, DelegationAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct RevokeDelegation<'info> {
    #[account(
        mut,
        seeds = [b"delegation", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub delegation: Account<'info, DelegationAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(
        mut,
        seeds = [b"delegation", delegation.user.as_ref()],
        bump,
        has_one = bot_authority
    )]
    pub delegation: Account<'info, DelegationAccount>,

    #[account(
        init,
        payer = bot_authority,
        space = 8 + std::mem::size_of::<Position>()
    )]
    pub position: Account<'info, Position>,

    /// CHECK: User's wallet (not signing, just for balance check)
    pub user: AccountInfo<'info>,

    #[account(mut)]
    pub bot_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(
        mut,
        seeds = [b"delegation", delegation.user.as_ref()],
        bump,
        has_one = bot_authority
    )]
    pub delegation: Account<'info, DelegationAccount>,

    #[account(mut)]
    pub position: Account<'info, Position>,

    pub bot_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetDelegationStats<'info> {
    #[account(
        seeds = [b"delegation", delegation.user.as_ref()],
        bump
    )]
    pub delegation: Account<'info, DelegationAccount>,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum VaultError {
    #[msg("Delegation is not active")]
    DelegationNotActive,
    #[msg("Maximum concurrent trades reached")]
    MaxTradesReached,
    #[msg("Position size exceeds maximum allowed")]
    PositionTooLarge,
    #[msg("Insufficient funds in user wallet")]
    InsufficientFunds,
    #[msg("Invalid amount specified")]
    InvalidAmount,
    #[msg("Position is not open")]
    PositionNotOpen,
    #[msg("Invalid position for this delegation")]
    InvalidPosition,
    #[msg("Invalid strategy selected")]
    InvalidStrategy,
}

// ============================================================================
// Helper Functions
// ============================================================================

fn strategy_name(strategy: u8) -> &'static str {
    match strategy {
        0 => "Conservative",
        1 => "Ultra-Early Sniper",
        2 => "Momentum Scalper",
        3 => "Graduation Anticipator",
        _ => "Unknown",
    }
}
