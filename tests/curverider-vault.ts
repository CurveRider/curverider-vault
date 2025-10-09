import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CurveriderVault } from "../target/types/curverider_vault";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { assert } from "chai";

describe("curverider-vault", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CurveriderVault as Program<CurveriderVault>;
  
  // Test accounts
  const authority = provider.wallet;
  let vaultPda: PublicKey;
  let vaultBump: number;
  let userAccountPda: PublicKey;
  let user2AccountPda: PublicKey;

  // Test user wallets
  const user1 = anchor.web3.Keypair.generate();
  const user2 = anchor.web3.Keypair.generate();

  before(async () => {
    // Airdrop SOL to test users
    const airdrop1 = await provider.connection.requestAirdrop(
      user1.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop1);

    const airdrop2 = await provider.connection.requestAirdrop(
      user2.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdrop2);

    // Derive PDAs
    [vaultPda, vaultBump] = await PublicKey.findProgramAddress(
      [Buffer.from("vault")],
      program.programId
    );

    [userAccountPda] = await PublicKey.findProgramAddress(
      [Buffer.from("user"), user1.publicKey.toBuffer()],
      program.programId
    );

    [user2AccountPda] = await PublicKey.findProgramAddress(
      [Buffer.from("user"), user2.publicKey.toBuffer()],
      program.programId
    );

    console.log("🔑 Test Setup Complete");
    console.log("Vault PDA:", vaultPda.toString());
    console.log("User 1:", user1.publicKey.toString());
    console.log("User 2:", user2.publicKey.toString());
  });

  describe("Vault Initialization", () => {
    it("Initializes the vault with correct parameters", async () => {
      const minDeposit = new anchor.BN(0.1 * LAMPORTS_PER_SOL); // 0.1 SOL
      const maxDeposit = new anchor.BN(100 * LAMPORTS_PER_SOL); // 100 SOL
      const managementFeeBps = 100; // 1%
      const performanceFeeBps = 2000; // 20%

      const tx = await program.methods
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

      console.log("✅ Vault initialized:", tx);

      // Fetch and verify vault account
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      
      assert.equal(
        vaultAccount.authority.toString(),
        authority.publicKey.toString()
      );
      assert.equal(vaultAccount.vaultBump, vaultBump);
      assert.equal(vaultAccount.totalDeposited.toNumber(), 0);
      assert.equal(vaultAccount.totalShares.toNumber(), 0);
      assert.equal(vaultAccount.minDeposit.toNumber(), minDeposit.toNumber());
      assert.equal(vaultAccount.maxDeposit.toNumber(), maxDeposit.toNumber());
      assert.equal(vaultAccount.managementFeeBps, managementFeeBps);
      assert.equal(vaultAccount.performanceFeeBps, performanceFeeBps);
      assert.equal(vaultAccount.isActive, true);
      assert.equal(vaultAccount.totalTrades.toNumber(), 0);
      assert.equal(vaultAccount.profitableTrades.toNumber(), 0);

      console.log("📊 Vault State:", vaultAccount);
    });
  });

  describe("Deposits", () => {
    it("User 1 makes first deposit (1:1 shares)", async () => {
      const depositAmount = new anchor.BN(2 * LAMPORTS_PER_SOL); // 2 SOL

      const tx = await program.methods
        .deposit(depositAmount)
        .accounts({
          vault: vaultPda,
          userAccount: userAccountPda,
          user: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      console.log("✅ User 1 deposited 2 SOL:", tx);

      // Verify vault state
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      assert.equal(
        vaultAccount.totalDeposited.toNumber(),
        depositAmount.toNumber()
      );
      assert.equal(
        vaultAccount.totalShares.toNumber(),
        depositAmount.toNumber()
      );

      // Verify user account
      const userAccount = await program.account.userAccount.fetch(
        userAccountPda
      );
      assert.equal(userAccount.owner.toString(), user1.publicKey.toString());
      assert.equal(userAccount.shares.toNumber(), depositAmount.toNumber());
      assert.equal(
        userAccount.totalDeposited.toNumber(),
        depositAmount.toNumber()
      );

      console.log("📊 User 1 shares:", userAccount.shares.toNumber());
    });

    it("User 2 makes deposit (proportional shares)", async () => {
      const depositAmount = new anchor.BN(3 * LAMPORTS_PER_SOL); // 3 SOL

      const tx = await program.methods
        .deposit(depositAmount)
        .accounts({
          vault: vaultPda,
          userAccount: user2AccountPda,
          user: user2.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user2])
        .rpc();

      console.log("✅ User 2 deposited 3 SOL:", tx);

      // Verify vault state
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      console.log("📊 Total deposited:", vaultAccount.totalDeposited.toNumber());
      console.log("📊 Total shares:", vaultAccount.totalShares.toNumber());

      // Verify user account
      const userAccount = await program.account.userAccount.fetch(
        user2AccountPda
      );
      assert.equal(userAccount.owner.toString(), user2.publicKey.toString());
      console.log("📊 User 2 shares:", userAccount.shares.toNumber());
    });

    it("Rejects deposit below minimum", async () => {
      const tinyAmount = new anchor.BN(0.05 * LAMPORTS_PER_SOL); // 0.05 SOL

      try {
        await program.methods
          .deposit(tinyAmount)
          .accounts({
            vault: vaultPda,
            userAccount: userAccountPda,
            user: user1.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([user1])
          .rpc();
        
        assert.fail("Should have rejected deposit below minimum");
      } catch (error) {
        assert.include(error.toString(), "BelowMinDeposit");
        console.log("✅ Correctly rejected deposit below minimum");
      }
    });
  });

  describe("Trading Positions", () => {
    let positionKeypair: anchor.web3.Keypair;

    it("Opens a trading position", async () => {
      positionKeypair = anchor.web3.Keypair.generate();
      
      const tokenMint = anchor.web3.Keypair.generate().publicKey;
      const amountSol = new anchor.BN(1 * LAMPORTS_PER_SOL);
      const entryPrice = new anchor.BN(1000000); // 0.001 SOL
      const takeProfitPrice = new anchor.BN(2000000); // 0.002 SOL (2x)
      const stopLossPrice = new anchor.BN(500000); // 0.0005 SOL (50% loss)

      const tx = await program.methods
        .openPosition(
          tokenMint,
          amountSol,
          entryPrice,
          takeProfitPrice,
          stopLossPrice
        )
        .accounts({
          vault: vaultPda,
          position: positionKeypair.publicKey,
          authority: authority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([positionKeypair])
        .rpc();

      console.log("✅ Position opened:", tx);

      // Verify position
      const position = await program.account.position.fetch(
        positionKeypair.publicKey
      );
      assert.equal(position.vault.toString(), vaultPda.toString());
      assert.equal(position.tokenMint.toString(), tokenMint.toString());
      assert.equal(position.amountSol.toNumber(), amountSol.toNumber());
      assert.equal(position.entryPrice.toNumber(), entryPrice.toNumber());
      assert.equal(position.status, 0); // Open

      // Verify vault stats
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      assert.equal(vaultAccount.totalTrades.toNumber(), 1);

      console.log("📊 Position opened:", position);
    });

    it("Closes a position with profit", async () => {
      const exitPrice = new anchor.BN(2500000); // 0.0025 SOL (2.5x)
      const amountReceived = new anchor.BN(2.5 * LAMPORTS_PER_SOL);

      const vaultBefore = await program.account.vault.fetch(vaultPda);
      const totalDepositedBefore = vaultBefore.totalDeposited.toNumber();

      const tx = await program.methods
        .closePosition(exitPrice, amountReceived)
        .accounts({
          vault: vaultPda,
          position: positionKeypair.publicKey,
          authority: authority.publicKey,
        })
        .rpc();

      console.log("✅ Position closed with profit:", tx);

      // Verify position
      const position = await program.account.position.fetch(
        positionKeypair.publicKey
      );
      assert.equal(position.status, 1); // Closed
      assert.equal(position.currentPrice.toNumber(), exitPrice.toNumber());
      assert.isTrue(position.pnl.toNumber() > 0);

      // Verify vault stats
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      assert.equal(vaultAccount.profitableTrades.toNumber(), 1);
      assert.isTrue(vaultAccount.totalPnl.toNumber() > 0);
      assert.isTrue(
        vaultAccount.totalDeposited.toNumber() > totalDepositedBefore
      );

      console.log("📊 Position PnL:", position.pnl.toNumber());
      console.log("📊 Vault total PnL:", vaultAccount.totalPnl.toNumber());
    });
  });

  describe("Withdrawals", () => {
    it("User 1 withdraws partial shares", async () => {
      const userAccountBefore = await program.account.userAccount.fetch(
        userAccountPda
      );
      const sharesToBurn = userAccountBefore.shares.divn(2); // Withdraw 50%

      const user1BalanceBefore = await provider.connection.getBalance(
        user1.publicKey
      );

      const tx = await program.methods
        .withdraw(sharesToBurn)
        .accounts({
          vault: vaultPda,
          userAccount: userAccountPda,
          user: user1.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();

      console.log("✅ User 1 withdrew 50% shares:", tx);

      const user1BalanceAfter = await provider.connection.getBalance(
        user1.publicKey
      );
      const balanceIncrease = user1BalanceAfter - user1BalanceBefore;

      console.log("💵 SOL received:", balanceIncrease / LAMPORTS_PER_SOL);

      // Verify user account
      const userAccountAfter = await program.account.userAccount.fetch(
        userAccountPda
      );
      assert.equal(
        userAccountAfter.shares.toNumber(),
        userAccountBefore.shares.sub(sharesToBurn).toNumber()
      );

      console.log("📊 User 1 remaining shares:", userAccountAfter.shares.toNumber());
    });

    it("Rejects withdrawal of more shares than owned", async () => {
      const userAccount = await program.account.userAccount.fetch(
        userAccountPda
      );
      const tooManyShares = userAccount.shares.addn(1000000);

      try {
        await program.methods
          .withdraw(tooManyShares)
          .accounts({
            vault: vaultPda,
            userAccount: userAccountPda,
            user: user1.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([user1])
          .rpc();
        
        assert.fail("Should have rejected excessive withdrawal");
      } catch (error) {
        assert.include(error.toString(), "InsufficientShares");
        console.log("✅ Correctly rejected excessive withdrawal");
      }
    });
  });

  describe("Vault Configuration", () => {
    it("Updates vault configuration", async () => {
      const newMinDeposit = new anchor.BN(0.5 * LAMPORTS_PER_SOL);
      const newMaxDeposit = new anchor.BN(50 * LAMPORTS_PER_SOL);

      const tx = await program.methods
        .updateVaultConfig(
          newMinDeposit,
          newMaxDeposit,
          200, // 2% management fee
          2500, // 25% performance fee
          true
        )
        .accounts({
          vault: vaultPda,
          authority: authority.publicKey,
        })
        .rpc();

      console.log("✅ Vault config updated:", tx);

      // Verify updates
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      assert.equal(vaultAccount.minDeposit.toNumber(), newMinDeposit.toNumber());
      assert.equal(vaultAccount.maxDeposit.toNumber(), newMaxDeposit.toNumber());
      assert.equal(vaultAccount.managementFeeBps, 200);
      assert.equal(vaultAccount.performanceFeeBps, 2500);
    });

    it("Rejects excessive fees", async () => {
      try {
        await program.methods
          .updateVaultConfig(
            null,
            null,
            5000, // 50% - too high!
            null,
            null
          )
          .accounts({
            vault: vaultPda,
            authority: authority.publicKey,
          })
          .rpc();
        
        assert.fail("Should have rejected excessive fee");
      } catch (error) {
        assert.include(error.toString(), "FeeTooHigh");
        console.log("✅ Correctly rejected excessive fee");
      }
    });
  });

  describe("Final Stats", () => {
    it("Displays final vault statistics", async () => {
      const vaultAccount = await program.account.vault.fetch(vaultPda);
      
      console.log("\n═══════════════════════════════════════");
      console.log("📊 FINAL VAULT STATISTICS");
      console.log("═══════════════════════════════════════");
      console.log("Total Deposited:", vaultAccount.totalDeposited.toNumber() / LAMPORTS_PER_SOL, "SOL");
      console.log("Total Shares:", vaultAccount.totalShares.toNumber());
      console.log("Total Trades:", vaultAccount.totalTrades.toNumber());
      console.log("Profitable Trades:", vaultAccount.profitableTrades.toNumber());
      console.log("Total PnL:", vaultAccount.totalPnl.toNumber() / LAMPORTS_PER_SOL, "SOL");
      console.log("Win Rate:", 
        vaultAccount.totalTrades.toNumber() > 0
          ? ((vaultAccount.profitableTrades.toNumber() / vaultAccount.totalTrades.toNumber()) * 100).toFixed(2) + "%"
          : "N/A"
      );
      console.log("═══════════════════════════════════════\n");
    });
  });
});
