use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Non-Custodial Trading Vault Program - MAINNET VERSION
///
/// KEY CHANGES FROM DEV VERSION:
/// 1. Position PDAs with deterministic seeds
/// 2. Bump seeds stored for efficiency
/// 3. Account closing for rent recovery
/// 4. Emergency pause functionality
/// 5. Bot authority rotation support
/// 6. Events for indexing
/// 7. Additional safety checks
///
/// NOTE: This version still tracks positions only.
/// Actual DEX integration (Jupiter/Raydium) should be done
/// via CPI or off-chain by the bot, with this contract
/// serving as the permission/tracking layer.
#[program]
pub mod curverider_vault {
    use super::*;

    /// Initialize the global config (one-time setup)
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        emergency_authority: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.emergency_authority = emergency_authority;
        config.is_paused = false;
        config.total_delegations = 0;
        config.total_positions = 0;
        config.bump = ctx.bumps.config;

        emit!(ConfigInitialized {
            authority: config.authority,
            emergency_authority,
        });

        Ok(())
    }

    /// Emergency pause - stops all new positions
    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.is_paused = true;

        emit!(EmergencyPaused {
            paused_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Resume from emergency pause
    pub fn emergency_resume(ctx: Context<EmergencyPause>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.is_paused = false;

        emit!(EmergencyResumed {
            resumed_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Create a delegation account allowing bot to trade on user's behalf
    pub fn create_delegation(
        ctx: Context<CreateDelegation>,
        strategy: u8,
        max_position_size_sol: u64,
        max_concurrent_trades: u8,
    ) -> Result<()> {
        // Check global pause
        require!(!ctx.accounts.config.is_paused, VaultError::SystemPaused);

        let delegation = &mut ctx.accounts.delegation;

        // Validate inputs
        require!(max_position_size_sol > 0, VaultError::InvalidAmount);
        require!(
            max_position_size_sol <= 100 * LAMPORTS_PER_SOL, // Max 100 SOL per position
            VaultError::PositionTooLarge
        );
        require!(
            max_concurrent_trades > 0 && max_concurrent_trades <= 10,
            VaultError::InvalidAmount
        );
        require!(strategy <= 3, VaultError::InvalidStrategy);

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
        delegation.total_volume = 0;
        delegation.created_at = Clock::get()?.unix_timestamp;
        delegation.last_trade_at = 0;
        delegation.bump = ctx.bumps.delegation;
        delegation.position_counter = 0;

        // Update global stats
        let config = &mut ctx.accounts.config;
        config.total_delegations = config.total_delegations.checked_add(1).unwrap();

        emit!(DelegationCreated {
            user: delegation.user,
            bot_authority: delegation.bot_authority,
            strategy,
            max_position_size_sol,
            max_concurrent_trades,
            timestamp: delegation.created_at,
        });

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
            require!(strat <= 3, VaultError::InvalidStrategy);
            delegation.strategy = strat;
        }

        if let Some(max_pos) = max_position_size_sol {
            require!(max_pos > 0, VaultError::InvalidAmount);
            require!(max_pos <= 100 * LAMPORTS_PER_SOL, VaultError::PositionTooLarge);
            delegation.max_position_size_sol = max_pos;
        }

        if let Some(max_trades) = max_concurrent_trades {
            require!(max_trades > 0 && max_trades <= 10, VaultError::InvalidAmount);
            // Don't allow reducing below current active trades
            require!(
                max_trades >= delegation.active_trades,
                VaultError::CannotReduceBelowActive
            );
            delegation.max_concurrent_trades = max_trades;
        }

        if let Some(active) = is_active {
            delegation.is_active = active;
        }

        emit!(DelegationUpdated {
            user: delegation.user,
            strategy: delegation.strategy,
            max_position_size_sol: delegation.max_position_size_sol,
            max_concurrent_trades: delegation.max_concurrent_trades,
            is_active: delegation.is_active,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Change bot authority (for key rotation)
    pub fn change_bot_authority(
        ctx: Context<ChangeBotAuthority>,
        new_bot_authority: Pubkey,
    ) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;

        // Can only change if no active trades
        require!(
            delegation.active_trades == 0,
            VaultError::HasActiveTrades
        );

        let old_authority = delegation.bot_authority;
        delegation.bot_authority = new_bot_authority;

        emit!(BotAuthorityChanged {
            user: delegation.user,
            old_authority,
            new_authority: new_bot_authority,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Revoke delegation - immediately stops bot from trading
    pub fn revoke_delegation(ctx: Context<RevokeDelegation>) -> Result<()> {
        let delegation = &mut ctx.accounts.delegation;

        delegation.is_active = false;

        emit!(DelegationRevoked {
            user: delegation.user,
            active_trades_remaining: delegation.active_trades,
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Close delegation account and recover rent (only if no active trades)
    pub fn close_delegation(ctx: Context<CloseDelegation>) -> Result<()> {
        let delegation = &ctx.accounts.delegation;

        require!(
            delegation.active_trades == 0,
            VaultError::HasActiveTrades
        );

        // Update global stats
        let config = &mut ctx.accounts.config;
        config.total_delegations = config.total_delegations.saturating_sub(1);

        emit!(DelegationClosed {
            user: delegation.user,
            total_trades: delegation.total_trades,
            total_pnl: delegation.total_pnl,
            timestamp: Clock::get()?.unix_timestamp,
        });

        // Account will be closed automatically by Anchor's close constraint

        Ok(())
    }

    /// Bot opens a trading position on behalf of user
    ///
    /// NOTE: This creates a position record. The actual token swap
    /// should be done by the bot via Jupiter/Raydium CPI or separately.
    pub fn open_position(
        ctx: Context<OpenPosition>,
        token_mint: Pubkey,
        amount_sol: u64,
        entry_price: u64,
        take_profit_price: u64,
        stop_loss_price: u64,
    ) -> Result<()> {
        // Check global pause
        require!(!ctx.accounts.config.is_paused, VaultError::SystemPaused);

        let delegation = &mut ctx.accounts.delegation;
        let position = &mut ctx.accounts.position;

        // Validate delegation
        require!(delegation.is_active, VaultError::DelegationNotActive);
        require!(
            delegation.active_trades < delegation.max_concurrent_trades,
            VaultError::MaxTradesReached
        );
        require!(
            amount_sol <= delegation.max_position_size_sol,
            VaultError::PositionTooLarge
        );
        require!(amount_sol > 0, VaultError::InvalidAmount);

        // Validate prices
        require!(entry_price > 0, VaultError::InvalidPrice);
        require!(take_profit_price > entry_price, VaultError::InvalidPrice);
        require!(stop_loss_price < entry_price, VaultError::InvalidPrice);
        require!(stop_loss_price > 0, VaultError::InvalidPrice);

        // Validate user has enough SOL (basic check)
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
        position.position_id = delegation.position_counter;
        position.bump = ctx.bumps.position;

        // Update delegation stats
        delegation.active_trades = delegation.active_trades.checked_add(1).unwrap();
        delegation.total_trades = delegation.total_trades.checked_add(1).unwrap();
        delegation.total_volume = delegation.total_volume.checked_add(amount_sol).unwrap();
        delegation.position_counter = delegation.position_counter.checked_add(1).unwrap();
        delegation.last_trade_at = Clock::get()?.unix_timestamp;

        // Update global stats
        let config = &mut ctx.accounts.config;
        config.total_positions = config.total_positions.checked_add(1).unwrap();

        emit!(PositionOpened {
            user: delegation.user,
            position_id: position.position_id,
            token_mint,
            amount_sol,
            entry_price,
            take_profit_price,
            stop_loss_price,
            timestamp: position.opened_at,
        });

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

        // Calculate PnL (can be negative)
        let pnl = (amount_received as i64)
            .checked_sub(position.amount_sol as i64)
            .ok_or(VaultError::MathOverflow)?;

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

        emit!(PositionClosed {
            user: delegation.user,
            position_id: position.position_id,
            token_mint: position.token_mint,
            entry_price: position.entry_price,
            exit_price,
            pnl,
            timestamp: position.closed_at,
        });

        Ok(())
    }

    /// Close position account and recover rent
    pub fn close_position_account(ctx: Context<ClosePositionAccount>) -> Result<()> {
        let position = &ctx.accounts.position;

        require!(
            position.status != PositionStatus::Open as u8,
            VaultError::PositionNotOpen
        );

        emit!(PositionAccountClosed {
            user: position.user,
            position_id: position.position_id,
            timestamp: Clock::get()?.unix_timestamp,
        });

        // Account will be closed by Anchor's close constraint

        Ok(())
    }
}

// ============================================================================
// Constants
// ============================================================================

const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct GlobalConfig {
    /// Protocol authority
    pub authority: Pubkey,
    /// Emergency authority (can pause)
    pub emergency_authority: Pubkey,
    /// Whether system is paused
    pub is_paused: bool,
    /// Total delegations created
    pub total_delegations: u64,
    /// Total positions created
    pub total_positions: u64,
    /// PDA bump
    pub bump: u8,
}

#[account]
pub struct DelegationAccount {
    /// User's wallet public key
    pub user: Pubkey,
    /// Bot's authority public key
    pub bot_authority: Pubkey,
    /// Selected strategy (0-3)
    pub strategy: u8,
    /// Maximum SOL per position (in lamports)
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
    /// Total volume traded in lamports
    pub total_volume: u64,
    /// Timestamp of delegation creation
    pub created_at: i64,
    /// Timestamp of last trade
    pub last_trade_at: i64,
    /// PDA bump seed
    pub bump: u8,
    /// Counter for position IDs
    pub position_counter: u64,
}

#[account]
pub struct Position {
    /// Delegation account that owns this position
    pub delegation: Pubkey,
    /// User's wallet
    pub user: Pubkey,
    /// Token mint address
    pub token_mint: Pubkey,
    /// Amount of SOL invested (lamports)
    pub amount_sol: u64,
    /// Entry price (scaled by 1e6)
    pub entry_price: u64,
    /// Current/exit price (scaled by 1e6)
    pub current_price: u64,
    /// Take profit target
    pub take_profit_price: u64,
    /// Stop loss target
    pub stop_loss_price: u64,
    /// Position status
    pub status: u8,
    /// When position was opened
    pub opened_at: i64,
    /// When position was closed
    pub closed_at: i64,
    /// Profit/loss in lamports
    pub pnl: i64,
    /// Unique position ID within delegation
    pub position_id: u64,
    /// PDA bump seed
    pub bump: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum PositionStatus {
    Open = 0,
    Closed = 1,
    Liquidated = 2,
}

// ============================================================================
// Context Structures
// ============================================================================

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<GlobalConfig>(),
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, GlobalConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        constraint = config.authority == authority.key() ||
                     config.emergency_authority == authority.key()
    )]
    pub config: Account<'info, GlobalConfig>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreateDelegation<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, GlobalConfig>,

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
        bump = delegation.bump,
        has_one = user
    )]
    pub delegation: Account<'info, DelegationAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct ChangeBotAuthority<'info> {
    #[account(
        mut,
        seeds = [b"delegation", user.key().as_ref()],
        bump = delegation.bump,
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
        bump = delegation.bump,
        has_one = user
    )]
    pub delegation: Account<'info, DelegationAccount>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseDelegation<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [b"delegation", user.key().as_ref()],
        bump = delegation.bump,
        has_one = user,
        close = user
    )]
    pub delegation: Account<'info, DelegationAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, GlobalConfig>,

    #[account(
        mut,
        seeds = [b"delegation", delegation.user.as_ref()],
        bump = delegation.bump,
        has_one = bot_authority
    )]
    pub delegation: Account<'info, DelegationAccount>,

    #[account(
        init,
        payer = bot_authority,
        space = 8 + std::mem::size_of::<Position>(),
        seeds = [
            b"position",
            delegation.key().as_ref(),
            &delegation.position_counter.to_le_bytes()
        ],
        bump
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
        bump = delegation.bump,
        has_one = bot_authority
    )]
    pub delegation: Account<'info, DelegationAccount>,

    #[account(
        mut,
        seeds = [
            b"position",
            delegation.key().as_ref(),
            &position.position_id.to_le_bytes()
        ],
        bump = position.bump,
        constraint = position.delegation == delegation.key()
    )]
    pub position: Account<'info, Position>,

    pub bot_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePositionAccount<'info> {
    #[account(
        mut,
        seeds = [
            b"position",
            position.delegation.as_ref(),
            &position.position_id.to_le_bytes()
        ],
        bump = position.bump,
        constraint = position.user == user.key(),
        close = user
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub user: Signer<'info>,
}

// ============================================================================
// Events
// ============================================================================

#[event]
pub struct ConfigInitialized {
    pub authority: Pubkey,
    pub emergency_authority: Pubkey,
}

#[event]
pub struct EmergencyPaused {
    pub paused_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EmergencyResumed {
    pub resumed_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DelegationCreated {
    pub user: Pubkey,
    pub bot_authority: Pubkey,
    pub strategy: u8,
    pub max_position_size_sol: u64,
    pub max_concurrent_trades: u8,
    pub timestamp: i64,
}

#[event]
pub struct DelegationUpdated {
    pub user: Pubkey,
    pub strategy: u8,
    pub max_position_size_sol: u64,
    pub max_concurrent_trades: u8,
    pub is_active: bool,
    pub timestamp: i64,
}

#[event]
pub struct BotAuthorityChanged {
    pub user: Pubkey,
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DelegationRevoked {
    pub user: Pubkey,
    pub active_trades_remaining: u8,
    pub timestamp: i64,
}

#[event]
pub struct DelegationClosed {
    pub user: Pubkey,
    pub total_trades: u64,
    pub total_pnl: i64,
    pub timestamp: i64,
}

#[event]
pub struct PositionOpened {
    pub user: Pubkey,
    pub position_id: u64,
    pub token_mint: Pubkey,
    pub amount_sol: u64,
    pub entry_price: u64,
    pub take_profit_price: u64,
    pub stop_loss_price: u64,
    pub timestamp: i64,
}

#[event]
pub struct PositionClosed {
    pub user: Pubkey,
    pub position_id: u64,
    pub token_mint: Pubkey,
    pub entry_price: u64,
    pub exit_price: u64,
    pub pnl: i64,
    pub timestamp: i64,
}

#[event]
pub struct PositionAccountClosed {
    pub user: Pubkey,
    pub position_id: u64,
    pub timestamp: i64,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum VaultError {
    #[msg("System is paused for emergency")]
    SystemPaused,
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
    #[msg("Invalid price specified")]
    InvalidPrice,
    #[msg("Position is not open")]
    PositionNotOpen,
    #[msg("Invalid position for this delegation")]
    InvalidPosition,
    #[msg("Invalid strategy selected")]
    InvalidStrategy,
    #[msg("Cannot reduce max trades below active trades")]
    CannotReduceBelowActive,
    #[msg("Cannot change authority while trades are active")]
    HasActiveTrades,
    #[msg("Math overflow")]
    MathOverflow,
}
