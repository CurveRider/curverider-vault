use crate::types::{BotConfig, Position, PositionStatus};
use crate::error::{Result, BotError};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
    instruction::Instruction,
    system_instruction,
    commitment_config::CommitmentConfig,
};
use spl_token::instruction as token_instruction;
use spl_associated_token_account::instruction as associated_token_instruction;
use tracing::{info, warn, error, debug};
use std::str::FromStr;

pub struct Trader {
    rpc_client: RpcClient,
    config: BotConfig,
    positions: Vec<Position>,
}

impl Trader {
    pub fn new(config: &BotConfig) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            config.rpc_url.clone(),
            CommitmentConfig::confirmed(),
        );

        Self {
            rpc_client,
            config: BotConfig {
                rpc_url: config.rpc_url.clone(),
                rpc_ws_url: config.rpc_ws_url.clone(),
                wallet_keypair: solana_sdk::signature::Keypair::from_bytes(&config.wallet_keypair.to_bytes()).unwrap(),
                min_liquidity_sol: config.min_liquidity_sol,
                max_position_size_sol: config.max_position_size_sol,
                take_profit_multiplier: config.take_profit_multiplier,
                stop_loss_percentage: config.stop_loss_percentage,
                pump_fun_api_url: config.pump_fun_api_url.clone(),
                raydium_amm_program: config.raydium_amm_program,
                max_slippage_bps: config.max_slippage_bps,
                max_concurrent_positions: config.max_concurrent_positions,
                position_timeout_seconds: config.position_timeout_seconds,
                scan_interval_ms: config.scan_interval_ms,
                volume_threshold_sol: config.volume_threshold_sol,
                holder_count_min: config.holder_count_min,
            },
            positions: Vec::new(),
        }
    }

    /// Buy token on pump.fun bonding curve
    pub async fn buy_token(
        &mut self,
        token_mint: &Pubkey,
        sol_amount: f64,
    ) -> Result<Position> {
        info!("🚀 Attempting to buy {} SOL of token {}", sol_amount, token_mint);

        // Check position limit
        if self.positions.len() >= self.config.max_concurrent_positions {
            return Err(BotError::PositionLimitReached(
                self.positions.len(),
                self.config.max_concurrent_positions,
            ));
        }

        // Check wallet balance
        let wallet_balance = self.get_wallet_balance()?;
        if wallet_balance < sol_amount {
            return Err(BotError::InsufficientFunds {
                required: sol_amount,
                available: wallet_balance,
            });
        }

        // Get or create associated token account
        let token_account = self.get_or_create_token_account(token_mint).await?;

        // Build buy transaction
        let transaction = self.build_buy_transaction(
            token_mint,
            &token_account,
            sol_amount,
        ).await?;

        // Send and confirm transaction
        let signature = self.send_and_confirm_transaction(transaction).await?;

        info!("✅ Buy transaction confirmed: {}", signature);

        // Get entry price and create position
        let entry_price = self.get_token_price(token_mint).await?;
        let amount = self.get_token_balance(&token_account)?;

        let position = Position {
            token_mint: *token_mint,
            entry_price,
            amount,
            sol_invested: sol_amount,
            entry_time: chrono::Utc::now().timestamp(),
            take_profit_price: entry_price * self.config.take_profit_multiplier,
            stop_loss_price: entry_price * (1.0 - self.config.stop_loss_percentage),
            status: PositionStatus::Open,
        };

        self.positions.push(position.clone());

        info!(
            "📊 Position opened: entry=${:.6}, TP=${:.6}, SL=${:.6}",
            entry_price,
            position.take_profit_price,
            position.stop_loss_price
        );

        Ok(position)
    }

    /// Sell token (either on bonding curve or DEX after graduation)
    pub async fn sell_token(
        &mut self,
        token_mint: &Pubkey,
        amount: Option<u64>,
    ) -> Result<f64> {
        info!("💰 Attempting to sell token {}", token_mint);

        // Find position index first to avoid borrow checker issues
        let pos_index = self.positions.iter().position(|p| &p.token_mint == token_mint && p.status == PositionStatus::Open)
            .ok_or_else(|| BotError::TokenNotFound(token_mint.to_string()))?;

        // Get sell_amount before mut borrow
        let sell_amount = {
            let position = &self.positions[pos_index];
            amount.unwrap_or(position.amount)
        };

        // Get token account and graduation status before mut borrow
        let token_account = self.get_token_account(token_mint)?;
        let is_graduated = self.check_if_graduated(token_mint).await?;

        let transaction = if is_graduated {
            info!("Token graduated - selling on Raydium");
            self.build_raydium_sell_transaction(token_mint, &token_account, sell_amount).await?
        } else {
            info!("Selling on pump.fun bonding curve");
            self.build_sell_transaction(token_mint, &token_account, sell_amount).await?
        };

        let signature = self.send_and_confirm_transaction(transaction).await?;
        let exit_price = self.get_token_price(token_mint).await?;
        let sol_received = (sell_amount as f64 * exit_price) / 1e9;

        // Now update position
        let position = &mut self.positions[pos_index];
        let pnl = sol_received - position.sol_invested;
        let pnl_percentage = (pnl / position.sol_invested) * 100.0;
        position.status = PositionStatus::Closed;

        info!(
            "✅ Sell transaction confirmed: {}\n\
             💵 SOL received: {:.4}\n\
             📈 PnL: {:.4} SOL ({:+.2}%)",
            signature, sol_received, pnl, pnl_percentage
        );

        Ok(pnl)
    }

    /// Monitor open positions and execute exit strategies
    pub async fn monitor_positions(&mut self) -> Result<()> {
        // Collect open positions' indices to avoid borrow checker issues
        let open_indices: Vec<_> = self.positions.iter().enumerate()
            .filter(|(_, p)| p.status == PositionStatus::Open)
            .map(|(i, _)| i)
            .collect();

        for i in open_indices {
            let (token_mint, take_profit_price, stop_loss_price, entry_time) = {
                let p = &self.positions[i];
                (p.token_mint, p.take_profit_price, p.stop_loss_price, p.entry_time)
            };
            let current_price = self.get_token_price(&token_mint).await?;
            let time_elapsed = chrono::Utc::now().timestamp() - entry_time;

            if current_price >= take_profit_price {
                info!("🎯 Take profit triggered for {}: ${:.6} >= ${:.6}", token_mint, current_price, take_profit_price);
                self.sell_token(&token_mint, None).await?;
                continue;
            }
            if current_price <= stop_loss_price {
                warn!("🛑 Stop loss triggered for {}: ${:.6} <= ${:.6}", token_mint, current_price, stop_loss_price);
                self.sell_token(&token_mint, None).await?;
                continue;
            }
            if time_elapsed > self.config.position_timeout_seconds as i64 {
                warn!("⏰ Position timeout for {}: {} seconds elapsed", token_mint, time_elapsed);
                self.sell_token(&token_mint, None).await?;
                continue;
            }
            let is_graduated = self.check_if_graduated(&token_mint).await?;
            if is_graduated {
                info!("🎓 Token {} graduated to DEX - considering exit", token_mint);
                // Could implement additional logic here
            }
        }
        Ok(())
    }

    /// Build buy transaction for pump.fun
    async fn build_buy_transaction(
        &self,
        token_mint: &Pubkey,
        token_account: &Pubkey,
        sol_amount: f64,
    ) -> Result<Transaction> {
        // TODO: Implement actual pump.fun buy instruction
        // This is a placeholder - actual implementation would need:
        // 1. Get bonding curve PDA
        // 2. Calculate expected token amount
        // 3. Build swap instruction with slippage protection
        
        let lamports = (sol_amount * 1e9) as u64;
        
        let instruction = system_instruction::transfer(
            &self.config.wallet_keypair.pubkey(),
            token_account,
            lamports,
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.config.wallet_keypair.pubkey()),
            &[&self.config.wallet_keypair],
            recent_blockhash,
        );

        Ok(transaction)
    }

    /// Build sell transaction for pump.fun
    async fn build_sell_transaction(
        &self,
        token_mint: &Pubkey,
        token_account: &Pubkey,
        amount: u64,
    ) -> Result<Transaction> {
        // TODO: Implement actual pump.fun sell instruction
        // Similar to buy but in reverse

        let instruction = system_instruction::transfer(
            &self.config.wallet_keypair.pubkey(),
            token_account,
            amount,
        );

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&self.config.wallet_keypair.pubkey()),
            &[&self.config.wallet_keypair],
            recent_blockhash,
        );

        Ok(transaction)
    }

    /// Build sell transaction for Raydium DEX
    async fn build_raydium_sell_transaction(
        &self,
        token_mint: &Pubkey,
        token_account: &Pubkey,
        amount: u64,
    ) -> Result<Transaction> {
        // TODO: Implement Raydium swap instruction
        // Would use Raydium SDK to build swap through liquidity pool

        warn!("Raydium sell not yet implemented - using placeholder");
        self.build_sell_transaction(token_mint, token_account, amount).await
    }

    /// Send and confirm transaction with retries
    async fn send_and_confirm_transaction(&self, transaction: Transaction) -> Result<String> {
        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        Ok(signature.to_string())
    }

    /// Get wallet SOL balance
    fn get_wallet_balance(&self) -> Result<f64> {
        let balance = self.rpc_client.get_balance(&self.config.wallet_keypair.pubkey())?;
        Ok(balance as f64 / 1e9)
    }

    /// Get or create associated token account
    async fn get_or_create_token_account(&self, token_mint: &Pubkey) -> Result<Pubkey> {
        let associated_token_address = spl_associated_token_account::get_associated_token_address(
            &self.config.wallet_keypair.pubkey(),
            token_mint,
        );

        // Check if account exists
        if self.rpc_client.get_account(&associated_token_address).is_ok() {
            return Ok(associated_token_address);
        }

        // Create account
        info!("Creating associated token account for {}", token_mint);
        // TODO: Implement account creation

        Ok(associated_token_address)
    }

    /// Get existing token account
    fn get_token_account(&self, token_mint: &Pubkey) -> Result<Pubkey> {
        Ok(spl_associated_token_account::get_associated_token_address(
            &self.config.wallet_keypair.pubkey(),
            token_mint,
        ))
    }

    /// Get token balance
    fn get_token_balance(&self, token_account: &Pubkey) -> Result<u64> {
        // TODO: Implement actual token balance check
        Ok(0)
    }

    /// Get current token price
    async fn get_token_price(&self, token_mint: &Pubkey) -> Result<f64> {
        // TODO: Implement actual price fetch from bonding curve or DEX
        Ok(0.001)
    }

    /// Check if token graduated to DEX
    async fn check_if_graduated(&self, token_mint: &Pubkey) -> Result<bool> {
        // TODO: Check if bonding curve is complete and token moved to Raydium
        Ok(false)
    }

    /// Get active positions
    pub fn get_active_positions(&self) -> Vec<&Position> {
        self.positions.iter()
            .filter(|p| p.status == PositionStatus::Open)
            .collect()
    }

    /// Get position count
    pub fn position_count(&self) -> usize {
        self.positions.iter()
            .filter(|p| p.status == PositionStatus::Open)
            .count()
    }
}
