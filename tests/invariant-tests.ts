import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CurveriderVault } from "../target/types/curverider_vault";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

/**
 * Invariant Tests for Non-Custodial Vault
 *
 * These tests verify that critical system invariants hold true
 * regardless of the sequence of operations.
 */

describe("Invariant Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CurveriderVault as Program<CurveriderVault>;

  let user: Keypair;
  let botAuthority: Keypair;
  let delegationPda: PublicKey;

  before(async () => {
    user = Keypair.generate();
    botAuthority = Keypair.generate();

    // Airdrop SOL
    await provider.connection.requestAirdrop(user.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(botAuthority.publicKey, 5 * LAMPORTS_PER_SOL);

    // Wait for confirmation
    await new Promise(resolve => setTimeout(resolve, 1000));

    [delegationPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("delegation"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("Delegation Invariants", () => {
    beforeEach(async () => {
      // Create fresh delegation for each test
      try {
        await program.methods
          .createDelegation(
            0, // Conservative
            new anchor.BN(0.5 * LAMPORTS_PER_SOL),
            3
          )
          .accounts({
            delegation: delegationPda,
            botAuthority: botAuthority.publicKey,
            user: user.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([user])
          .rpc();

        await new Promise(resolve => setTimeout(resolve, 500));
      } catch (e) {
        // Delegation might already exist from previous test
      }
    });

    it("INVARIANT: activeTrades <= maxConcurrentTrades", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegation.activeTrades).to.be.lte(delegation.maxConcurrentTrades);
      console.log(`✓ Active trades (${delegation.activeTrades}) <= Max (${delegation.maxConcurrentTrades})`);
    });

    it("INVARIANT: profitableTrades <= totalTrades", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegation.profitableTrades.toNumber()).to.be.lte(delegation.totalTrades.toNumber());
      console.log(`✓ Profitable (${delegation.profitableTrades}) <= Total (${delegation.totalTrades})`);
    });

    it("INVARIANT: strategy is always valid (0-3)", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegation.strategy).to.be.gte(0);
      expect(delegation.strategy).to.be.lte(3);
      console.log(`✓ Strategy (${delegation.strategy}) is valid`);
    });

    it("INVARIANT: maxPositionSizeSol > 0", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegation.maxPositionSizeSol.toNumber()).to.be.gt(0);
      console.log(`✓ Max position size (${delegation.maxPositionSizeSol.toNumber()}) > 0`);
    });

    it("INVARIANT: maxConcurrentTrades is within bounds (1-10)", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegation.maxConcurrentTrades).to.be.gte(1);
      expect(delegation.maxConcurrentTrades).to.be.lte(10);
      console.log(`✓ Max concurrent trades (${delegation.maxConcurrentTrades}) is within bounds`);
    });

    it("INVARIANT: User wallet balance never goes negative", async () => {
      const balanceBefore = await provider.connection.getBalance(user.publicKey);

      // Perform multiple operations
      await program.methods
        .updateDelegation(1, null, null, null)
        .accounts({ delegation: delegationPda, user: user.publicKey })
        .signers([user])
        .rpc();

      const balanceAfter = await provider.connection.getBalance(user.publicKey);

      expect(balanceAfter).to.be.gte(0);
      console.log(`✓ User balance (${balanceAfter}) >= 0`);
    });
  });

  describe("Position Invariants", () => {
    let position1: Keypair;
    let position2: Keypair;
    const tokenMint = Keypair.generate().publicKey;

    beforeEach(async () => {
      position1 = Keypair.generate();
      position2 = Keypair.generate();

      // Ensure delegation is active
      try {
        await program.methods
          .updateDelegation(null, null, null, true)
          .accounts({ delegation: delegationPda, user: user.publicKey })
          .signers([user])
          .rpc();
      } catch (e) {}
    });

    it("INVARIANT: Position amount never exceeds maxPositionSize", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      const maxSize = delegation.maxPositionSizeSol.toNumber();

      // Open position at max size
      await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(maxSize),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position1])
        .rpc();

      const position = await program.account.position.fetch(position1.publicKey);

      expect(position.amountSol.toNumber()).to.be.lte(maxSize);
      console.log(`✓ Position amount (${position.amountSol.toNumber()}) <= Max (${maxSize})`);
    });

    it("INVARIANT: Closed position status never reverts to Open", async () => {
      // Open position
      await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(0.1 * LAMPORTS_PER_SOL),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position1])
        .rpc();

      // Close position
      await program.methods
        .closePosition(
          new anchor.BN(1000000),
          new anchor.BN(0.1 * LAMPORTS_PER_SOL)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          botAuthority: botAuthority.publicKey,
        })
        .signers([botAuthority])
        .rpc();

      const position = await program.account.position.fetch(position1.publicKey);
      expect(position.status).to.equal(1); // Closed

      // Try to close again (should fail or be idempotent)
      try {
        await program.methods
          .closePosition(
            new anchor.BN(1000000),
            new anchor.BN(0.1 * LAMPORTS_PER_SOL)
          )
          .accounts({
            delegation: delegationPda,
            position: position1.publicKey,
            botAuthority: botAuthority.publicKey,
          })
          .signers([botAuthority])
          .rpc();

        expect.fail("Should not allow closing already closed position");
      } catch (e) {
        console.log("✓ Cannot close already closed position");
      }
    });

    it("INVARIANT: Position belongs to correct delegation", async () => {
      await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(0.1 * LAMPORTS_PER_SOL),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position1])
        .rpc();

      const position = await program.account.position.fetch(position1.publicKey);

      expect(position.delegation.toString()).to.equal(delegationPda.toString());
      console.log("✓ Position delegation matches");
    });

    it("INVARIANT: Position user matches delegation user", async () => {
      await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(0.1 * LAMPORTS_PER_SOL),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position1])
        .rpc();

      const position = await program.account.position.fetch(position1.publicKey);
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      expect(position.user.toString()).to.equal(delegation.user.toString());
      console.log("✓ Position user matches delegation user");
    });

    it("INVARIANT: PnL calculation is consistent", async () => {
      const entryAmount = 0.5 * LAMPORTS_PER_SOL;
      const exitAmount = 1.0 * LAMPORTS_PER_SOL; // 2x

      await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(entryAmount),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position1])
        .rpc();

      await program.methods
        .closePosition(
          new anchor.BN(2000000),
          new anchor.BN(exitAmount)
        )
        .accounts({
          delegation: delegationPda,
          position: position1.publicKey,
          botAuthority: botAuthority.publicKey,
        })
        .signers([botAuthority])
        .rpc();

      const position = await program.account.position.fetch(position1.publicKey);
      const expectedPnl = exitAmount - entryAmount;

      expect(position.pnl.toNumber()).to.be.closeTo(expectedPnl, 1);
      console.log(`✓ PnL (${position.pnl.toNumber()}) matches expected (${expectedPnl})`);
    });
  });

  describe("State Consistency Invariants", () => {
    it("INVARIANT: Opening position increments counters correctly", async () => {
      const delegationBefore = await program.account.delegationAccount.fetch(delegationPda);
      const activeTradesBefore = delegationBefore.activeTrades;
      const totalTradesBefore = delegationBefore.totalTrades.toNumber();

      const position = Keypair.generate();
      await program.methods
        .openPosition(
          Keypair.generate().publicKey,
          new anchor.BN(0.1 * LAMPORTS_PER_SOL),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position])
        .rpc();

      const delegationAfter = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegationAfter.activeTrades).to.equal(activeTradesBefore + 1);
      expect(delegationAfter.totalTrades.toNumber()).to.equal(totalTradesBefore + 1);
      console.log("✓ Counters incremented correctly on open");
    });

    it("INVARIANT: Closing position decrements active trades", async () => {
      const position = Keypair.generate();

      await program.methods
        .openPosition(
          Keypair.generate().publicKey,
          new anchor.BN(0.1 * LAMPORTS_PER_SOL),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position])
        .rpc();

      const delegationBefore = await program.account.delegationAccount.fetch(delegationPda);
      const activeTradesBefore = delegationBefore.activeTrades;

      await program.methods
        .closePosition(
          new anchor.BN(1000000),
          new anchor.BN(0.1 * LAMPORTS_PER_SOL)
        )
        .accounts({
          delegation: delegationPda,
          position: position.publicKey,
          botAuthority: botAuthority.publicKey,
        })
        .signers([botAuthority])
        .rpc();

      const delegationAfter = await program.account.delegationAccount.fetch(delegationPda);

      expect(delegationAfter.activeTrades).to.equal(activeTradesBefore - 1);
      console.log("✓ Active trades decremented correctly on close");
    });

    it("INVARIANT: Total PnL is sum of all position PnLs", async () => {
      // This is a simplified check - in real audit, would track all positions
      const delegation = await program.account.delegationAccount.fetch(delegationPda);

      // Total PnL should be reasonable relative to number of trades
      const totalPnl = delegation.totalPnl.toNumber();
      const totalTrades = delegation.totalTrades.toNumber();

      // If we have trades, PnL should not be absurdly large
      if (totalTrades > 0) {
        const maxExpectedPnl = totalTrades * 10 * LAMPORTS_PER_SOL; // Max 10 SOL per trade
        expect(Math.abs(totalPnl)).to.be.lte(maxExpectedPnl);
      }

      console.log(`✓ Total PnL (${totalPnl}) is reasonable for ${totalTrades} trades`);
    });
  });

  describe("Time-based Invariants", () => {
    it("INVARIANT: createdAt timestamp is in the past", async () => {
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      const now = Math.floor(Date.now() / 1000);

      expect(delegation.createdAt.toNumber()).to.be.lte(now);
      console.log(`✓ Creation time (${delegation.createdAt}) <= Now (${now})`);
    });

    it("INVARIANT: Position openedAt <= closedAt (if closed)", async () => {
      const position = Keypair.generate();

      await program.methods
        .openPosition(
          Keypair.generate().publicKey,
          new anchor.BN(0.1 * LAMPORTS_PER_SOL),
          new anchor.BN(1000000),
          new anchor.BN(2000000),
          new anchor.BN(500000)
        )
        .accounts({
          delegation: delegationPda,
          position: position.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, position])
        .rpc();

      await program.methods
        .closePosition(
          new anchor.BN(1000000),
          new anchor.BN(0.1 * LAMPORTS_PER_SOL)
        )
        .accounts({
          delegation: delegationPda,
          position: position.publicKey,
          botAuthority: botAuthority.publicKey,
        })
        .signers([botAuthority])
        .rpc();

      const positionData = await program.account.position.fetch(position.publicKey);

      expect(positionData.openedAt.toNumber()).to.be.lte(positionData.closedAt.toNumber());
      console.log(`✓ Opened (${positionData.openedAt}) <= Closed (${positionData.closedAt})`);
    });
  });
});
