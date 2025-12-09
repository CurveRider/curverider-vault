import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CurveriderVault } from "../target/types/curverider_vault";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

describe("Non-Custodial Curverider Vault", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CurveriderVault as Program<CurveriderVault>;

  // Test accounts
  let user: Keypair;
  let botAuthority: Keypair;
  let delegationPda: PublicKey;
  let delegationBump: number;
  let positionKeypair: Keypair;

  // Test constants
  const STRATEGY_CONSERVATIVE = 0;
  const STRATEGY_ULTRA_EARLY = 1;
  const STRATEGY_MOMENTUM = 2;
  const STRATEGY_GRADUATION = 3;
  const MAX_POSITION_SIZE = 0.5 * LAMPORTS_PER_SOL;
  const MAX_CONCURRENT_TRADES = 3;

  before(async () => {
    // Create test accounts
    user = Keypair.generate();
    botAuthority = Keypair.generate();
    positionKeypair = Keypair.generate();

    // Airdrop SOL to user
    const signature = await provider.connection.requestAirdrop(
      user.publicKey,
      5 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);

    // Airdrop SOL to bot authority for creating positions
    const botSig = await provider.connection.requestAirdrop(
      botAuthority.publicKey,
      2 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(botSig);

    // Derive delegation PDA
    [delegationPda, delegationBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("delegation"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("Delegation Management", () => {
    it("Creates a delegation account", async () => {
      const tx = await program.methods
        .createDelegation(
          STRATEGY_CONSERVATIVE,
          new anchor.BN(MAX_POSITION_SIZE),
          MAX_CONCURRENT_TRADES
        )
        .accounts({
          delegation: delegationPda,
          botAuthority: botAuthority.publicKey,
          user: user.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();

      console.log("Create delegation tx:", tx);

      // Fetch and verify delegation account
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.user.toString()).to.equal(user.publicKey.toString());
      expect(delegation.botAuthority.toString()).to.equal(botAuthority.publicKey.toString());
      expect(delegation.strategy).to.equal(STRATEGY_CONSERVATIVE);
      expect(delegation.maxPositionSizeSol.toNumber()).to.equal(MAX_POSITION_SIZE);
      expect(delegation.maxConcurrentTrades).to.equal(MAX_CONCURRENT_TRADES);
      expect(delegation.isActive).to.be.true;
      expect(delegation.activeTrades).to.equal(0);
      expect(delegation.totalTrades.toNumber()).to.equal(0);
      expect(delegation.profitableTrades.toNumber()).to.equal(0);
      expect(delegation.totalPnl.toNumber()).to.equal(0);
    });

    it("Updates delegation settings", async () => {
      const newMaxPosition = 1.0 * LAMPORTS_PER_SOL;
      const newMaxTrades = 5;
      const newStrategy = STRATEGY_MOMENTUM;

      const tx = await program.methods
        .updateDelegation(
          newStrategy,
          new anchor.BN(newMaxPosition),
          newMaxTrades,
          null // Keep active state unchanged
        )
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();

      console.log("Update delegation tx:", tx);

      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.strategy).to.equal(newStrategy);
      expect(delegation.maxPositionSizeSol.toNumber()).to.equal(newMaxPosition);
      expect(delegation.maxConcurrentTrades).to.equal(newMaxTrades);
    });

    it("Prevents non-owner from updating delegation", async () => {
      const attacker = Keypair.generate();

      // Airdrop to attacker
      const sig = await provider.connection.requestAirdrop(
        attacker.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      try {
        await program.methods
          .updateDelegation(
            STRATEGY_ULTRA_EARLY,
            null,
            null,
            null
          )
          .accounts({
            delegation: delegationPda,
            user: attacker.publicKey, // Wrong user!
          })
          .signers([attacker])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("ConstraintHasOne");
      }
    });

    it("Revokes delegation", async () => {
      const tx = await program.methods
        .revokeDelegation()
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();

      console.log("Revoke delegation tx:", tx);

      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.isActive).to.be.false;
    });

    it("Re-activates delegation", async () => {
      const tx = await program.methods
        .updateDelegation(
          null,
          null,
          null,
          true // Reactivate
        )
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();

      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.isActive).to.be.true;
    });
  });

  describe("Position Management", () => {
    const tokenMint = Keypair.generate().publicKey;
    const amountSol = 0.3 * LAMPORTS_PER_SOL;
    const entryPrice = 1000000; // Simulated price
    const takeProfitPrice = 2000000; // 2x
    const stopLossPrice = 500000; // 50% loss

    it("Opens a position", async () => {
      const tx = await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(amountSol),
          new anchor.BN(entryPrice),
          new anchor.BN(takeProfitPrice),
          new anchor.BN(stopLossPrice)
        )
        .accounts({
          delegation: delegationPda,
          position: positionKeypair.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, positionKeypair])
        .rpc();

      console.log("Open position tx:", tx);

      // Verify position
      const position = await program.account.position.fetch(positionKeypair.publicKey);
      expect(position.delegation.toString()).to.equal(delegationPda.toString());
      expect(position.user.toString()).to.equal(user.publicKey.toString());
      expect(position.tokenMint.toString()).to.equal(tokenMint.toString());
      expect(position.amountSol.toNumber()).to.equal(amountSol);
      expect(position.entryPrice.toNumber()).to.equal(entryPrice);
      expect(position.status).to.equal(0); // Open

      // Verify delegation updated
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.activeTrades).to.equal(1);
      expect(delegation.totalTrades.toNumber()).to.equal(1);
    });

    it("Prevents opening position when delegation inactive", async () => {
      // Deactivate delegation
      await program.methods
        .revokeDelegation()
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();

      try {
        const newPosition = Keypair.generate();
        await program.methods
          .openPosition(
            tokenMint,
            new anchor.BN(amountSol),
            new anchor.BN(entryPrice),
            new anchor.BN(takeProfitPrice),
            new anchor.BN(stopLossPrice)
          )
          .accounts({
            delegation: delegationPda,
            position: newPosition.publicKey,
            user: user.publicKey,
            botAuthority: botAuthority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([botAuthority, newPosition])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("DelegationNotActive");
      }

      // Reactivate for next tests
      await program.methods
        .updateDelegation(null, null, null, true)
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
    });

    it("Prevents opening position exceeding max position size", async () => {
      try {
        const tooLargeAmount = 2 * LAMPORTS_PER_SOL; // Exceeds 0.5 SOL limit
        const newPosition = Keypair.generate();

        await program.methods
          .openPosition(
            tokenMint,
            new anchor.BN(tooLargeAmount),
            new anchor.BN(entryPrice),
            new anchor.BN(takeProfitPrice),
            new anchor.BN(stopLossPrice)
          )
          .accounts({
            delegation: delegationPda,
            position: newPosition.publicKey,
            user: user.publicKey,
            botAuthority: botAuthority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([botAuthority, newPosition])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("PositionTooLarge");
      }
    });

    it("Closes position with profit", async () => {
      const exitPrice = 2500000; // 2.5x profit
      const amountReceived = (amountSol * 2.5);

      const tx = await program.methods
        .closePosition(
          new anchor.BN(exitPrice),
          new anchor.BN(amountReceived)
        )
        .accounts({
          delegation: delegationPda,
          position: positionKeypair.publicKey,
          botAuthority: botAuthority.publicKey,
        })
        .signers([botAuthority])
        .rpc();

      console.log("Close position tx:", tx);

      // Verify position closed
      const position = await program.account.position.fetch(positionKeypair.publicKey);
      expect(position.status).to.equal(1); // Closed
      expect(position.currentPrice.toNumber()).to.equal(exitPrice);

      const pnl = amountReceived - amountSol;
      expect(position.pnl.toNumber()).to.be.closeTo(pnl, 1);

      // Verify delegation stats updated
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.activeTrades).to.equal(0);
      expect(delegation.profitableTrades.toNumber()).to.equal(1);
      expect(delegation.totalPnl.toNumber()).to.be.closeTo(pnl, 1);
    });

    it("Opens and closes position with loss", async () => {
      const newPosition = Keypair.generate();

      // Open position
      await program.methods
        .openPosition(
          tokenMint,
          new anchor.BN(amountSol),
          new anchor.BN(entryPrice),
          new anchor.BN(takeProfitPrice),
          new anchor.BN(stopLossPrice)
        )
        .accounts({
          delegation: delegationPda,
          position: newPosition.publicKey,
          user: user.publicKey,
          botAuthority: botAuthority.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([botAuthority, newPosition])
        .rpc();

      // Close with loss
      const exitPrice = 600000; // 40% loss
      const amountReceived = (amountSol * 0.6);

      await program.methods
        .closePosition(
          new anchor.BN(exitPrice),
          new anchor.BN(amountReceived)
        )
        .accounts({
          delegation: delegationPda,
          position: newPosition.publicKey,
          botAuthority: botAuthority.publicKey,
        })
        .signers([botAuthority])
        .rpc();

      const position = await program.account.position.fetch(newPosition.publicKey);
      const pnl = amountReceived - amountSol;
      expect(position.pnl.toNumber()).to.be.lessThan(0);
      expect(position.pnl.toNumber()).to.be.closeTo(pnl, 1);

      // Verify profitable trades didn't increase
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.profitableTrades.toNumber()).to.equal(1); // Still 1 from before
    });
  });

  describe("Security Tests", () => {
    it("Prevents unauthorized bot from opening position", async () => {
      const fakeBotAuthority = Keypair.generate();

      // Airdrop to fake bot
      const sig = await provider.connection.requestAirdrop(
        fakeBotAuthority.publicKey,
        LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      try {
        const newPosition = Keypair.generate();
        const tokenMint = Keypair.generate().publicKey;

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
            position: newPosition.publicKey,
            user: user.publicKey,
            botAuthority: fakeBotAuthority.publicKey, // Wrong bot!
            systemProgram: SystemProgram.programId,
          })
          .signers([fakeBotAuthority, newPosition])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("ConstraintHasOne");
      }
    });

    it("Enforces max concurrent trades limit", async () => {
      // Set max to 1
      await program.methods
        .updateDelegation(null, null, 1, null)
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();

      // Open first position (should succeed)
      const position1 = Keypair.generate();
      const tokenMint = Keypair.generate().publicKey;

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

      // Try to open second position (should fail)
      try {
        const position2 = Keypair.generate();

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
            position: position2.publicKey,
            user: user.publicKey,
            botAuthority: botAuthority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([botAuthority, position2])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("MaxTradesReached");
      }

      // Clean up - close position
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

      // Reset max trades
      await program.methods
        .updateDelegation(null, null, 5, null)
        .accounts({
          delegation: delegationPda,
          user: user.publicKey,
        })
        .signers([user])
        .rpc();
    });

    it("Validates strategy selection", async () => {
      try {
        const invalidStrategy = 99; // Invalid

        await program.methods
          .updateDelegation(
            invalidStrategy,
            null,
            null,
            null
          )
          .accounts({
            delegation: delegationPda,
            user: user.publicKey,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("InvalidStrategy");
      }
    });
  });

  describe("Edge Cases", () => {
    it("Handles zero position size rejection", async () => {
      try {
        await program.methods
          .updateDelegation(
            null,
            new anchor.BN(0), // Zero!
            null,
            null
          )
          .accounts({
            delegation: delegationPda,
            user: user.publicKey,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("InvalidAmount");
      }
    });

    it("Handles max concurrent trades limits", async () => {
      try {
        await program.methods
          .updateDelegation(
            null,
            null,
            0, // Zero!
            null
          )
          .accounts({
            delegation: delegationPda,
            user: user.publicKey,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("InvalidAmount");
      }
    });

    it("Handles excessive concurrent trades limit", async () => {
      try {
        await program.methods
          .updateDelegation(
            null,
            null,
            100, // Too high!
            null
          )
          .accounts({
            delegation: delegationPda,
            user: user.publicKey,
          })
          .signers([user])
          .rpc();

        expect.fail("Should have thrown an error");
      } catch (error) {
        expect(error.toString()).to.include("InvalidAmount");
      }
    });

    it("Gets delegation stats", async () => {
      const tx = await program.methods
        .getDelegationStats()
        .accounts({
          delegation: delegationPda,
        })
        .rpc();

      console.log("Get delegation stats tx:", tx);

      // Transaction should succeed
      expect(tx).to.be.a("string");
    });
  });
});
