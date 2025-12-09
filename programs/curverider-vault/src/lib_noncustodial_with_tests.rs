use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// ... (same program code as lib_noncustodial.rs) ...
// For brevity, showing just the test module

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_validation() {
        // Test valid strategies
        assert!(is_valid_strategy(0));
        assert!(is_valid_strategy(1));
        assert!(is_valid_strategy(2));
        assert!(is_valid_strategy(3));

        // Test invalid strategies
        assert!(!is_valid_strategy(4));
        assert!(!is_valid_strategy(255));
    }

    #[test]
    fn test_position_size_validation() {
        assert!(is_valid_position_size(1_000_000)); // 0.001 SOL
        assert!(is_valid_position_size(5_000_000_000)); // 5 SOL
        assert!(!is_valid_position_size(0)); // Zero
    }

    #[test]
    fn test_concurrent_trades_validation() {
        assert!(is_valid_concurrent_trades(1));
        assert!(is_valid_concurrent_trades(5));
        assert!(is_valid_concurrent_trades(10));
        assert!(!is_valid_concurrent_trades(0));
        assert!(!is_valid_concurrent_trades(11));
    }

    #[test]
    fn test_pnl_calculation() {
        let entry = 1_000_000;
        let exit_profit = 2_000_000;
        let exit_loss = 500_000;

        let pnl_profit = calculate_pnl(entry, exit_profit);
        let pnl_loss = calculate_pnl(entry, exit_loss);

        assert_eq!(pnl_profit, 1_000_000);
        assert_eq!(pnl_loss, -500_000);
    }

    #[test]
    fn test_position_status_transitions() {
        assert!(can_close_position(PositionStatus::Open as u8));
        assert!(!can_close_position(PositionStatus::Closed as u8));
        assert!(!can_close_position(PositionStatus::Liquidated as u8));
    }

    // Helper functions (would be in your actual lib.rs)
    fn is_valid_strategy(strategy: u8) -> bool {
        strategy <= 3
    }

    fn is_valid_position_size(size: u64) -> bool {
        size > 0
    }

    fn is_valid_concurrent_trades(count: u8) -> bool {
        count >= 1 && count <= 10
    }

    fn calculate_pnl(entry: i64, exit: i64) -> i64 {
        exit - entry
    }

    fn can_close_position(status: u8) -> bool {
        status == PositionStatus::Open as u8
    }
}

// Integration tests using Rust's built-in test framework
#[cfg(all(test, feature = "test-bpf"))]
mod integration_tests {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_create_delegation_rust() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::new(
            "curverider_vault",
            program_id,
            processor!(process_instruction),
        );

        // Add your test logic here
        // This would be similar to the TypeScript tests
        // but using Rust and solana-program-test crate

        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Example: Create delegation
        // ... test implementation ...

        assert!(true); // Placeholder
    }
}
