use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Token, TokenAccount, Transfer, Mint};
// use anchor_spl::associated_token::AssociatedToken;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Main program module for Curverider Vault
/// Manages autonomous DeFi trading strategies on Solana
#[program]
pub mod curverider_vault {
    use super::*;

    /// Initialize the vault with configuration parameters
    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_bump: u8,
        min_deposit: u64,
        max_deposit: u64,
        management_fee_bps: u16,
        performance_fee_bps: u16,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        vault.authority = ctx.accounts.authority.key();
        vault.vault_bump = vault_bump;
        vault.total_deposited = 0;
        vault.total_shares = 0;
        vault.min_deposit = min_deposit;
        vault.max_deposit = max_deposit;
        vault.management_fee_bps = management_fee_bps;
        vault.performance_fee_bps = performance_fee_bps;
        vault.is_active = true;
        vault.total_trades = 0;
        vault.profitable_trades = 0;
        vault.total_pnl = 0;
        vault.created_at = Clock::get()?.unix_timestamp;
        
        msg!("✅ Vault initialized!");
        msg!("Authority: {}", vault.authority);
        msg!("Min deposit: {} lamports", min_deposit);
        msg!("Max deposit: {} lamports", max_deposit);
        
        Ok(())
    }

    /// Deposit SOL into the vault and receive vault shares
    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
    ) -> Result<()> {
        // Avoid double mutable/immutable borrow by not holding vault as a mutable reference during CPI
        require!(ctx.accounts.vault.is_active, VaultError::VaultNotActive);
        require!(amount >= ctx.accounts.vault.min_deposit, VaultError::BelowMinDeposit);
        require!(amount <= ctx.accounts.vault.max_deposit, VaultError::AboveMaxDeposit);

        // Calculate shares to mint
        let shares_to_mint = if ctx.accounts.vault.total_shares == 0 {
            amount
        } else {
            amount
                .checked_mul(ctx.accounts.vault.total_shares)
                .unwrap()
                .checked_div(ctx.accounts.vault.total_deposited)
                .unwrap()
        };

        // Transfer SOL from user to vault
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;

        // Now get mutable references
        let vault = &mut ctx.accounts.vault;
        let user_account = &mut ctx.accounts.user_account;

        // Update vault state
        vault.total_deposited = vault.total_deposited.checked_add(amount).unwrap();
        vault.total_shares = vault.total_shares.checked_add(shares_to_mint).unwrap();

        // Initialize or update user account
        if user_account.shares == 0 {
            user_account.owner = ctx.accounts.user.key();
            user_account.vault = vault.key();
            user_account.deposited_at = Clock::get()?.unix_timestamp;
        }
        user_account.shares = user_account.shares.checked_add(shares_to_mint).unwrap();
        user_account.total_deposited = user_account.total_deposited.checked_add(amount).unwrap();

        msg!("💰 Deposit successful!");
        msg!("Amount: {} lamports", amount);
        msg!("Shares minted: {}", shares_to_mint);
        msg!("User total shares: {}", user_account.shares);

        Ok(())
    }

    /// Withdraw SOL from the vault by burning shares
    pub fn withdraw(
        ctx: Context<Withdraw>,
        shares_to_burn: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let user_account = &mut ctx.accounts.user_account;
        
        require!(shares_to_burn > 0, VaultError::InvalidAmount);
        require!(user_account.shares >= shares_to_burn, VaultError::InsufficientShares);
        
        // Calculate SOL to return
        // amount = (shares_to_burn * total_deposited) / total_shares
        let amount_to_return = shares_to_burn
            .checked_mul(vault.total_deposited)
            .unwrap()
            .checked_div(vault.total_shares)
            .unwrap();
        
        // Transfer SOL from vault to user
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount_to_return;
        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount_to_return;
        
        // Update vault state
        vault.total_deposited = vault.total_deposited.checked_sub(amount_to_return).unwrap();
        vault.total_shares = vault.total_shares.checked_sub(shares_to_burn).unwrap();
        
        // Update user account
        user_account.shares = user_account.shares.checked_sub(shares_to_burn).unwrap();
        
        msg!("💵 Withdrawal successful!");
        msg!("Shares burned: {}", shares_to_burn);
        msg!("SOL returned: {} lamports", amount_to_return);
        msg!("User remaining shares: {}", user_account.shares);
        
        Ok(())
    }

    /// Open a new trading position (called by bot/authority)
    pub fn open_position(
        ctx: Context<OpenPosition>,
        token_mint: Pubkey,
        amount_sol: u64,
        entry_price: u64,
        take_profit_price: u64,
        stop_loss_price: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let position = &mut ctx.accounts.position;
        
        require!(vault.is_active, VaultError::VaultNotActive);
        require!(amount_sol <= vault.total_deposited, VaultError::InsufficientFunds);
        
        position.vault = vault.key();
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
        
        vault.total_trades = vault.total_trades.checked_add(1).unwrap();
        
        msg!("📈 Position opened!");
        msg!("Token: {}", token_mint);
        msg!("Entry price: {}", entry_price);
        msg!("TP: {}, SL: {}", take_profit_price, stop_loss_price);
        
        Ok(())
    }

    /// Close a trading position and record PnL
    pub fn close_position(
        ctx: Context<ClosePosition>,
        exit_price: u64,
        amount_received: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        let position = &mut ctx.accounts.position;
        
        require!(position.status == PositionStatus::Open as u8, VaultError::PositionNotOpen);
        require!(position.vault == vault.key(), VaultError::InvalidPosition);
        
        // Calculate PnL (can be negative)
        let pnl = (amount_received as i64)
            .checked_sub(position.amount_sol as i64)
            .unwrap();
        
        position.current_price = exit_price;
        position.status = PositionStatus::Closed as u8;
        position.closed_at = Clock::get()?.unix_timestamp;
        position.pnl = pnl;
        
        // Update vault statistics
        vault.total_pnl = vault.total_pnl.checked_add(pnl).unwrap();
        
        if pnl > 0 {
            vault.profitable_trades = vault.profitable_trades.checked_add(1).unwrap();
            vault.total_deposited = vault.total_deposited
                .checked_add(pnl as u64)
                .unwrap();
        } else {
            vault.total_deposited = vault.total_deposited
                .checked_sub((-pnl) as u64)
                .unwrap();
        }
        
        msg!("📊 Position closed!");
        msg!("Exit price: {}", exit_price);
        msg!("PnL: {} lamports", pnl);
        msg!("Vault total PnL: {}", vault.total_pnl);
        
        Ok(())
    }

    /// Update vault configuration (authority only)
    pub fn update_vault_config(
        ctx: Context<UpdateVaultConfig>,
        min_deposit: Option<u64>,
        max_deposit: Option<u64>,
        management_fee_bps: Option<u16>,
        performance_fee_bps: Option<u16>,
        is_active: Option<bool>,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        if let Some(min) = min_deposit {
            vault.min_deposit = min;
        }
        if let Some(max) = max_deposit {
            vault.max_deposit = max;
        }
        if let Some(mgmt_fee) = management_fee_bps {
            require!(mgmt_fee <= 1000, VaultError::FeeTooHigh); // Max 10%
            vault.management_fee_bps = mgmt_fee;
        }
        if let Some(perf_fee) = performance_fee_bps {
            require!(perf_fee <= 3000, VaultError::FeeTooHigh); // Max 30%
            vault.performance_fee_bps = perf_fee;
        }
        if let Some(active) = is_active {
            vault.is_active = active;
        }
        
        msg!("⚙️ Vault configuration updated!");
        
        Ok(())
    }

    /// Claim accumulated fees (authority only)
    pub fn claim_fees(
        ctx: Context<ClaimFees>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        require!(amount <= vault.total_deposited, VaultError::InsufficientFunds);
        
        // Transfer SOL from vault to authority
        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.to_account_info().try_borrow_mut_lamports()? += amount;
        
        msg!("💰 Fees claimed: {} lamports", amount);
        
        Ok(())
    }
}

// ============================================================================
// Account Structures
// ============================================================================

#[account]
pub struct Vault {
    /// Vault authority (can update config and claim fees)
    pub authority: Pubkey,
    /// PDA bump seed
    pub vault_bump: u8,
    /// Total SOL deposited by all users
    pub total_deposited: u64,
    /// Total shares issued
    pub total_shares: u64,
    /// Minimum deposit amount
    pub min_deposit: u64,
    /// Maximum deposit amount
    pub max_deposit: u64,
    /// Management fee in basis points (e.g., 100 = 1%)
    pub management_fee_bps: u16,
    /// Performance fee in basis points (e.g., 2000 = 20%)
    pub performance_fee_bps: u16,
    /// Whether vault is accepting deposits
    pub is_active: bool,
    /// Total number of trades executed
    pub total_trades: u64,
    /// Number of profitable trades
    pub profitable_trades: u64,
    /// Total PnL (can be negative)
    pub total_pnl: i64,
    /// Timestamp when vault was created
    pub created_at: i64,
}

#[account]
pub struct UserAccount {
    /// User's public key
    pub owner: Pubkey,
    /// Vault this account belongs to
    pub vault: Pubkey,
    /// Number of shares owned
    pub shares: u64,
    /// Total amount deposited (for tracking)
    pub total_deposited: u64,
    /// Timestamp of first deposit
    pub deposited_at: i64,
}

#[account]
pub struct Position {
    /// Vault that owns this position
    pub vault: Pubkey,
    /// Token mint address
    pub token_mint: Pubkey,
    /// Amount of SOL invested
    pub amount_sol: u64,
    /// Entry price (in smallest unit)
    pub entry_price: u64,
    /// Current price (updated as needed)
    pub current_price: u64,
    /// Take profit price target
    pub take_profit_price: u64,
    /// Stop loss price target
    pub stop_loss_price: u64,
    /// Position status (0=Open, 1=Closed, 2=Liquidated)
    pub status: u8,
    /// Timestamp when position was opened
    pub opened_at: i64,
    /// Timestamp when position was closed
    pub closed_at: i64,
    /// Profit/Loss in lamports (can be negative)
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
#[instruction(vault_bump: u8)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Vault>(),
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.vault_bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + std::mem::size_of::<UserAccount>(),
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.vault_bump
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.vault_bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<Position>()
    )]
    pub position: Account<'info, Position>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.vault_bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub position: Account<'info, Position>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateVaultConfig<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.vault_bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimFees<'info> {
    #[account(
        mut,
        seeds = [b"vault"],
        bump = vault.vault_bump,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
}

// ============================================================================
// Errors
// ============================================================================

#[error_code]
pub enum VaultError {
    #[msg("Vault is not active")]
    VaultNotActive,
    #[msg("Deposit amount below minimum")]
    BelowMinDeposit,
    #[msg("Deposit amount above maximum")]
    AboveMaxDeposit,
    #[msg("Insufficient funds in vault")]
    InsufficientFunds,
    #[msg("Insufficient shares to withdraw")]
    InsufficientShares,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Position is not open")]
    PositionNotOpen,
    #[msg("Invalid position")]
    InvalidPosition,
    #[msg("Fee too high (max 10% mgmt, 30% performance)")]
    FeeTooHigh,
}
