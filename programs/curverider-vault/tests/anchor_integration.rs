use anchor_lang::prelude::*;
use solana_sdk::signer::Signer;
use anchor_lang::solana_program::system_program;
use anchor_lang::prelude::Pubkey;
use anchor_lang::prelude::ToAccountInfo;
use anchor_lang::prelude::System;
use anchor_lang::prelude::AccountMeta;
use anchor_lang::prelude::Key;
use anchor_lang::prelude::Program;




use anchor_lang::prelude::*;
use solana_program_test::{processor, ProgramTest};
use std::str::FromStr;

#[tokio::test]
async fn test_vault_initialization() {
    // Setup test context
    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "curverider-vault",
        program_id,
        None, // Anchor handles the entrypoint automatically
    );
    use anchor_lang::InstructionData;
    use anchor_lang::ToAccountMetas;
    use solana_program_test::tokio;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction, system_program};

    // Set up test context
    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "curverider_vault",
        program_id,
        None,
    );

    // Create authority keypair
    let authority = Keypair::new();

    // Start test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Derive vault PDA
    let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault"], &program_id);

    // Build instruction
    let min_deposit = 1_000_000;
    let max_deposit = 10_000_000;
    let management_fee_bps = 100;
    let performance_fee_bps = 2000;
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::InitializeVault {
            vault: vault_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::InitializeVault {
            vault_bump,
            min_deposit,
            max_deposit,
            management_fee_bps,
            performance_fee_bps,
        }
        .data(),
    };

    // Fund authority
    let fund_ix = solana_sdk::system_instruction::transfer(&payer.pubkey(), &authority.pubkey(), 2_000_000_000);
    let fund_tx = Transaction::new_signed_with_payer(
        &[fund_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(fund_tx).await.unwrap();

    // Send initialize transaction
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // Fetch and assert vault state
    let vault_account = banks_client
        .get_account(vault_pda)
        .await
        .unwrap()
        .expect("vault account not found");
    let vault: curverider_vault::Vault = anchor_lang::AccountDeserialize::try_deserialize(&mut &vault_account.data[..]).unwrap();
    assert_eq!(vault.authority, authority.pubkey());
    assert_eq!(vault.min_deposit, min_deposit);
    assert_eq!(vault.max_deposit, max_deposit);
    assert_eq!(vault.management_fee_bps, management_fee_bps);
    assert_eq!(vault.performance_fee_bps, performance_fee_bps);
    assert!(vault.is_active);
}

#[tokio::test]
async fn test_deposit_withdraw() {
    use anchor_lang::InstructionData;
    use anchor_lang::ToAccountMetas;
    use solana_program_test::tokio;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction, system_program};

    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "curverider-vault",
        program_id,
        None,
    );

    // Create authority and user keypairs
    let authority = Keypair::new();
    let user = Keypair::new();

    // Start test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Derive vault PDA
    let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault"], &program_id);
    let (user_account_pda, _user_bump) = Pubkey::find_program_address(&[b"user", user.pubkey().as_ref()], &program_id);

    // Fund authority and user
    let fund_ixs = vec![
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &authority.pubkey(), 2_000_000_000),
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &user.pubkey(), 2_000_000_000),
    ];
    let fund_tx = Transaction::new_signed_with_payer(
        &fund_ixs,
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(fund_tx).await.unwrap();

    // Initialize vault
    let min_deposit = 1_000_000;
    let max_deposit = 10_000_000;
    let management_fee_bps = 100;
    let performance_fee_bps = 2000;
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::InitializeVault {
            vault: vault_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::InitializeVault {
            vault_bump,
            min_deposit,
            max_deposit,
            management_fee_bps,
            performance_fee_bps,
        }
        .data(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // Deposit
    let deposit_amount = 2_000_000;
    let deposit_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::Deposit {
            vault: vault_pda,
            user_account: user_account_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::Deposit {
            amount: deposit_amount,
        }
        .data(),
    };
    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    banks_client.process_transaction(deposit_tx).await.unwrap();

    // Fetch and assert vault and user state after deposit
    let vault_account = banks_client.get_account(vault_pda).await.unwrap().expect("vault not found");
    let vault: curverider_vault::Vault = anchor_lang::AccountDeserialize::try_deserialize(&mut &vault_account.data[..]).unwrap();
    assert_eq!(vault.total_deposited, deposit_amount);
    assert_eq!(vault.total_shares, deposit_amount);

    let user_account = banks_client.get_account(user_account_pda).await.unwrap().expect("user account not found");
    let user_acc: curverider_vault::UserAccount = anchor_lang::AccountDeserialize::try_deserialize(&mut &user_account.data[..]).unwrap();
    assert_eq!(user_acc.shares, deposit_amount);
    assert_eq!(user_acc.total_deposited, deposit_amount);

    // Withdraw
    let withdraw_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::Withdraw {
            vault: vault_pda,
            user_account: user_account_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::Withdraw {
            shares_to_burn: deposit_amount,
        }
        .data(),
    };
    let withdraw_tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    banks_client.process_transaction(withdraw_tx).await.unwrap();

    // Fetch and assert vault and user state after withdrawal
    let vault_account = banks_client.get_account(vault_pda).await.unwrap().expect("vault not found");
    let vault: curverider_vault::Vault = anchor_lang::AccountDeserialize::try_deserialize(&mut &vault_account.data[..]).unwrap();
    assert_eq!(vault.total_deposited, 0);
    assert_eq!(vault.total_shares, 0);

    let user_account = banks_client.get_account(user_account_pda).await.unwrap().expect("user account not found");
    let user_acc: curverider_vault::UserAccount = anchor_lang::AccountDeserialize::try_deserialize(&mut &user_account.data[..]).unwrap();
    assert_eq!(user_acc.shares, 0);
}

#[tokio::test]
async fn test_trading_logic() {
    use anchor_lang::InstructionData;
    use anchor_lang::ToAccountMetas;
    use solana_program_test::tokio;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction, system_program};

    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "curverider-vault",
        program_id,
        None,
    );

    let authority = Keypair::new();
    let user = Keypair::new();
    let bot = Keypair::new();

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault"], &program_id);
    let (user_account_pda, _user_bump) = Pubkey::find_program_address(&[b"user", user.pubkey().as_ref()], &program_id);
    let (position_pda, position_bump) = Pubkey::find_program_address(&[b"position", user.pubkey().as_ref(), &[0]], &program_id);

    // Fund authority, user, and bot
    let fund_ixs = vec![
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &authority.pubkey(), 2_000_000_000),
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &user.pubkey(), 2_000_000_000),
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &bot.pubkey(), 2_000_000_000),
    ];
    let fund_tx = Transaction::new_signed_with_payer(
        &fund_ixs,
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(fund_tx).await.unwrap();

    // Initialize vault
    let min_deposit = 1_000_000;
    let max_deposit = 10_000_000;
    let management_fee_bps = 100;
    let performance_fee_bps = 2000;
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::InitializeVault {
            vault: vault_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::InitializeVault {
            vault_bump,
            min_deposit,
            max_deposit,
            management_fee_bps,
            performance_fee_bps,
        }
        .data(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // Deposit by user
    let deposit_amount = 2_000_000;
    let deposit_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::Deposit {
            vault: vault_pda,
            user_account: user_account_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::Deposit {
            amount: deposit_amount,
        }
        .data(),
    };
    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    banks_client.process_transaction(deposit_tx).await.unwrap();

    // Open position (by authority/bot)
    let token_mint = Pubkey::new_unique();
    let entry_price = 100_000;
    let take_profit_price = 120_000;
    let stop_loss_price = 90_000;
    let open_position_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::OpenPosition {
            vault: vault_pda,
            position: position_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::OpenPosition {
            token_mint,
            amount_sol: deposit_amount,
            entry_price,
            take_profit_price,
            stop_loss_price,
        }
        .data(),
    };
    let open_tx = Transaction::new_signed_with_payer(
        &[open_position_ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    banks_client.process_transaction(open_tx).await.unwrap();

    // Fetch and assert position state
    let position_account = banks_client.get_account(position_pda).await.unwrap().expect("position not found");
    let position: curverider_vault::Position = anchor_lang::AccountDeserialize::try_deserialize(&mut &position_account.data[..]).unwrap();
    assert_eq!(position.amount_sol, deposit_amount);
    assert_eq!(position.entry_price, entry_price);
    assert_eq!(position.status, 0); // Open

    // Close position
    let exit_price = 110_000;
    let amount_received = deposit_amount + 100_000; // Simulate profit
    let close_position_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::ClosePosition {
            vault: vault_pda,
            position: position_pda,
            authority: authority.pubkey(),
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::ClosePosition {
            exit_price,
            amount_received,
        }
        .data(),
    };
    let close_tx = Transaction::new_signed_with_payer(
        &[close_position_ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    banks_client.process_transaction(close_tx).await.unwrap();

    // Fetch and assert vault stats
    let vault_account = banks_client.get_account(vault_pda).await.unwrap().expect("vault not found");
    let vault: curverider_vault::Vault = anchor_lang::AccountDeserialize::try_deserialize(&mut &vault_account.data[..]).unwrap();
    assert_eq!(vault.total_trades, 1);
    assert_eq!(vault.profitable_trades, 1);
    assert!(vault.total_pnl > 0);
    // Position should be closed
    let position_account = banks_client.get_account(position_pda).await.unwrap().expect("position not found");
    let position: curverider_vault::Position = anchor_lang::AccountDeserialize::try_deserialize(&mut &position_account.data[..]).unwrap();
    assert_eq!(position.status, 1); // Closed
    assert_eq!(position.pnl, 100_000);
}

#[tokio::test]
async fn test_error_cases() {
    use anchor_lang::InstructionData;
    use anchor_lang::ToAccountMetas;
    use solana_program_test::tokio;
    use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction, system_program};

    let program_id = Pubkey::from_str("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS").unwrap();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "curverider-vault",
        program_id,
        None,
    );

    let authority = Keypair::new();
    let user = Keypair::new();

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault"], &program_id);
    let (user_account_pda, _user_bump) = Pubkey::find_program_address(&[b"user", user.pubkey().as_ref()], &program_id);

    // Fund authority and user
    let fund_ixs = vec![
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &authority.pubkey(), 2_000_000_000),
        solana_sdk::system_instruction::transfer(&payer.pubkey(), &user.pubkey(), 2_000_000_000),
    ];
    let fund_tx = Transaction::new_signed_with_payer(
        &fund_ixs,
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    banks_client.process_transaction(fund_tx).await.unwrap();

    // Initialize vault
    let min_deposit = 1_000_000;
    let max_deposit = 10_000_000;
    let management_fee_bps = 100;
    let performance_fee_bps = 2000;
    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::InitializeVault {
            vault: vault_pda,
            authority: authority.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::InitializeVault {
            vault_bump,
            min_deposit,
            max_deposit,
            management_fee_bps,
            performance_fee_bps,
        }
        .data(),
    };
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&authority.pubkey()),
        &[&authority],
        recent_blockhash,
    );
    banks_client.process_transaction(tx).await.unwrap();

    // Try deposit below minimum
    let deposit_amount = 100; // Below min_deposit
    let deposit_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::Deposit {
            vault: vault_pda,
            user_account: user_account_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::Deposit {
            amount: deposit_amount,
        }
        .data(),
    };
    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    let result = banks_client.process_transaction(deposit_tx).await;
    assert!(result.is_err());

    // Now deposit a valid amount
    let valid_deposit = 2_000_000;
    let deposit_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::Deposit {
            vault: vault_pda,
            user_account: user_account_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::Deposit {
            amount: valid_deposit,
        }
        .data(),
    };
    let deposit_tx = Transaction::new_signed_with_payer(
        &[deposit_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    banks_client.process_transaction(deposit_tx).await.unwrap();

    // Try to withdraw more shares than owned
    let withdraw_ix = anchor_lang::solana_program::instruction::Instruction {
        program_id,
        accounts: curverider_vault::accounts::Withdraw {
            vault: vault_pda,
            user_account: user_account_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: curverider_vault::instruction::Withdraw {
            shares_to_burn: valid_deposit + 1_000_000, // More than owned
        }
        .data(),
    };
    let withdraw_tx = Transaction::new_signed_with_payer(
        &[withdraw_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );
    let result = banks_client.process_transaction(withdraw_tx).await;
    assert!(result.is_err());
}
