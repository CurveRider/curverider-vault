import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CurveriderVault } from "../target/types/curverider_vault";
import { PublicKey, Keypair, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { expect } from "chai";

/**
 * Fuzz Tests for Non-Custodial Vault
 *
 * These tests use randomized inputs to discover edge cases and vulnerabilities
 */

describe("Fuzz Tests", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CurveriderVault as Program<CurveriderVault>;

  const FUZZ_ITERATIONS = 50; // Number of random test iterations

  function randomInt(min: number, max: number): number {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }

  function randomBigInt(min: number, max: number): anchor.BN {
    return new anchor.BN(randomInt(min, max));
  }

  function randomStrategy(): number {
    return randomInt(0, 3);
  }

  function randomBoolean(): boolean {
    return Math.random() > 0.5;
  }

  describe("Delegation Creation Fuzz", () => {
    it("Handles random valid delegation parameters", async () => {
      let successCount = 0;
      let failCount = 0;

      for (let i = 0; i < FUZZ_ITERATIONS; i++) {
        try {
          const user = Keypair.generate();
          const botAuthority = Keypair.generate();

          // Airdrop
          await provider.connection.requestAirdrop(user.publicKey, 2 * LAMPORTS_PER_SOL);
          await new Promise(resolve => setTimeout(resolve, 500));

          const [delegationPda] = PublicKey.findProgramAddressSync(
            [Buffer.from("delegation"), user.publicKey.toBuffer()],
            program.programId
          );

          const strategy = randomStrategy();
          const maxPosition = randomInt(1000000, 5 * LAMPORTS_PER_SOL); // 0.001 to 5 SOL
          const maxTrades = randomInt(1, 10);

          await program.methods
            .createDelegation(
              strategy,
              new anchor.BN(maxPosition),
              maxTrades
            )
            .accounts({
              delegation: delegationPda,
              botAuthority: botAuthority.publicKey,
              user: user.publicKey,
              systemProgram: SystemProgram.programId,
            })
            .signers([user])
            .rpc();

          // Verify
          const delegation = await program.account.delegationAccount.fetch(delegationPda);
          expect(delegation.strategy).to.equal(strategy);
          expect(delegation.maxPositionSizeSol.toNumber()).to.equal(maxPosition);
          expect(delegation.maxConcurrentTrades).to.equal(maxTrades);

          successCount++;
        } catch (error) {
          failCount++;
          console.log(`Iteration ${i} failed:`, error.message);
        }
      }

      console.log(`Fuzz test results: ${successCount} successes, ${failCount} failures`);
      expect(successCount).to.be.gt(FUZZ_ITERATIONS * 0.9); // At least 90% success rate
    });

    it("Rejects invalid random parameters", async () => {
      const user = Keypair.generate();
      const botAuthority = Keypair.generate();

      await provider.connection.requestAirdrop(user.publicKey, 2 * LAMPORTS_PER_SOL);
      await new Promise(resolve => setTimeout(resolve, 500));

      const [delegationPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("delegation"), user.publicKey.toBuffer()],
        program.programId
      );

      let rejectedCount = 0;

      for (let i = 0; i < 20; i++) {
        try {
          // Generate intentionally invalid parameters
          const invalidStrategy = randomInt(10, 100); // Invalid range
          const maxPosition = randomInt(1000000, 5 * LAMPORTS_PER_SOL);
          const maxTrades = randomInt(1, 10);

          await program.methods
            .createDelegation(
              invalidStrategy,
              new anchor.BN(maxPosition),
              maxTrades
            )
            .accounts({
              delegation: delegationPda,
              botAuthority: botAuthority.publicKey,
              user: user.publicKey,
              systemProgram: SystemProgram.programId,
            })
            .signers([user])
            .rpc();

          console.log(`Expected rejection but succeeded with strategy ${invalidStrategy}`);
        } catch (error) {
          if (error.toString().includes("InvalidStrategy")) {
            rejectedCount++;
          }
        }
      }

      console.log(`Correctly rejected ${rejectedCount}/20 invalid strategies`);
      expect(rejectedCount).to.equal(20); // Should reject all
    });
  });

  describe("Position Management Fuzz", () => {
    let user: Keypair;
    let botAuthority: Keypair;
    let delegationPda: PublicKey;

    before(async () => {
      user = Keypair.generate();
      botAuthority = Keypair.generate();

      await provider.connection.requestAirdrop(user.publicKey, 10 * LAMPORTS_PER_SOL);
      await provider.connection.requestAirdrop(botAuthority.publicKey, 5 * LAMPORTS_PER_SOL);
      await new Promise(resolve => setTimeout(resolve, 1000));

      [delegationPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("delegation"), user.publicKey.toBuffer()],
        program.programId
      );

      // Create delegation
      await program.methods
        .createDelegation(
          0,
          new anchor.BN(5 * LAMPORTS_PER_SOL),
          10
        )
        .accounts({
          delegation: delegationPda,
          botAuthority: botAuthority.publicKey,
          user: user.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();
    });

    it("Handles random position parameters", async () => {
      let successCount = 0;

      for (let i = 0; i < 30; i++) {
        try {
          const position = Keypair.generate();
          const tokenMint = Keypair.generate().publicKey;

          const amountSol = randomInt(100000, 5 * LAMPORTS_PER_SOL);
          const entryPrice = randomInt(100, 10000000);
          const takeProfitPrice = entryPrice + randomInt(entryPrice, entryPrice * 10);
          const stopLossPrice = Math.max(1, entryPrice - randomInt(0, entryPrice));

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
              position: position.publicKey,
              user: user.publicKey,
              botAuthority: botAuthority.publicKey,
              systemProgram: SystemProgram.programId,
            })
            .signers([botAuthority, position])
            .rpc();

          // Immediately close to avoid hitting limits
          const exitPrice = randomInt(stopLossPrice, takeProfitPrice);
          const amountReceived = Math.floor(amountSol * (exitPrice / entryPrice));

          await program.methods
            .closePosition(
              new anchor.BN(exitPrice),
              new anchor.BN(amountReceived)
            )
            .accounts({
              delegation: delegationPda,
              position: position.publicKey,
              botAuthority: botAuthority.publicKey,
            })
            .signers([botAuthority])
            .rpc();

          successCount++;
        } catch (error) {
          console.log(`Position iteration ${i} error:`, error.message);
        }
      }

      console.log(`Successfully opened/closed ${successCount}/30 random positions`);
      expect(successCount).to.be.gt(20); // At least 20/30 should succeed
    });

    it("Tests extreme position values", async () => {
      const extremeValues = [
        { amount: 1, entry: 1, tp: 2, sl: 0 }, // Minimum values
        { amount: 1000, entry: 1, tp: 1000000, sl: 1 }, // Huge multiplier
        { amount: LAMPORTS_PER_SOL, entry: 1000000, tp: 2000000, sl: 500000 }, // Normal
        { amount: 100000, entry: 999999, tp: 1000000, sl: 999998 }, // Tiny margins
      ];

      for (const testCase of extremeValues) {
        try {
          const position = Keypair.generate();
          const tokenMint = Keypair.generate().publicKey;

          await program.methods
            .openPosition(
              tokenMint,
              new anchor.BN(testCase.amount),
              new anchor.BN(testCase.entry),
              new anchor.BN(testCase.tp),
              new anchor.BN(testCase.sl)
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

          const positionData = await program.account.position.fetch(position.publicKey);
          expect(positionData.status).to.equal(0); // Open

          // Clean up
          await program.methods
            .closePosition(
              new anchor.BN(testCase.entry),
              new anchor.BN(testCase.amount)
            )
            .accounts({
              delegation: delegationPda,
              position: position.publicKey,
              botAuthority: botAuthority.publicKey,
            })
            .signers([botAuthority])
            .rpc();

          console.log(`✓ Extreme case passed: amount=${testCase.amount}, entry=${testCase.entry}`);
        } catch (error) {
          console.log(`✗ Extreme case failed: ${error.message}`);
        }
      }
    });
  });

  describe("Update Operations Fuzz", () => {
    let user: Keypair;
    let botAuthority: Keypair;
    let delegationPda: PublicKey;

    before(async () => {
      user = Keypair.generate();
      botAuthority = Keypair.generate();

      await provider.connection.requestAirdrop(user.publicKey, 5 * LAMPORTS_PER_SOL);
      await new Promise(resolve => setTimeout(resolve, 1000));

      [delegationPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("delegation"), user.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .createDelegation(
          0,
          new anchor.BN(LAMPORTS_PER_SOL),
          5
        )
        .accounts({
          delegation: delegationPda,
          botAuthority: botAuthority.publicKey,
          user: user.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();
    });

    it("Handles rapid update sequences", async () => {
      let updateCount = 0;

      for (let i = 0; i < 50; i++) {
        try {
          const strategy = randomBoolean() ? randomStrategy() : null;
          const maxPosition = randomBoolean() ? randomInt(100000, 5 * LAMPORTS_PER_SOL) : null;
          const maxTrades = randomBoolean() ? randomInt(1, 10) : null;
          const isActive = randomBoolean() ? randomBoolean() : null;

          await program.methods
            .updateDelegation(
              strategy,
              maxPosition ? new anchor.BN(maxPosition) : null,
              maxTrades,
              isActive
            )
            .accounts({
              delegation: delegationPda,
              user: user.publicKey,
            })
            .signers([user])
            .rpc();

          updateCount++;
        } catch (error) {
          console.log(`Update ${i} failed:`, error.message);
        }
      }

      console.log(`Successfully performed ${updateCount}/50 rapid updates`);
      expect(updateCount).to.be.gt(40); // Most should succeed

      // Verify delegation is still valid
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.strategy).to.be.gte(0).and.lte(3);
      expect(delegation.maxPositionSizeSol.toNumber()).to.be.gt(0);
      expect(delegation.maxConcurrentTrades).to.be.gte(1).and.lte(10);
    });

    it("Tests state transitions with random operations", async () => {
      const operations = ["update", "revoke", "activate", "update"];
      let transitionCount = 0;

      for (let i = 0; i < 20; i++) {
        try {
          const operation = operations[randomInt(0, operations.length - 1)];

          switch (operation) {
            case "update":
              await program.methods
                .updateDelegation(
                  randomStrategy(),
                  new anchor.BN(randomInt(100000, 5 * LAMPORTS_PER_SOL)),
                  randomInt(1, 10),
                  null
                )
                .accounts({
                  delegation: delegationPda,
                  user: user.publicKey,
                })
                .signers([user])
                .rpc();
              break;

            case "revoke":
              await program.methods
                .revokeDelegation()
                .accounts({
                  delegation: delegationPda,
                  user: user.publicKey,
                })
                .signers([user])
                .rpc();
              break;

            case "activate":
              await program.methods
                .updateDelegation(null, null, null, true)
                .accounts({
                  delegation: delegationPda,
                  user: user.publicKey,
                })
                .signers([user])
                .rpc();
              break;
          }

          transitionCount++;
        } catch (error) {
          // Some transitions might fail, that's okay
        }
      }

      console.log(`Completed ${transitionCount} state transitions`);

      // Ensure delegation is still accessible
      const delegation = await program.account.delegationAccount.fetch(delegationPda);
      expect(delegation.user.toString()).to.equal(user.publicKey.toString());
    });
  });

  describe("PnL Calculation Fuzz", () => {
    let user: Keypair;
    let botAuthority: Keypair;
    let delegationPda: PublicKey;

    before(async () => {
      user = Keypair.generate();
      botAuthority = Keypair.generate();

      await provider.connection.requestAirdrop(user.publicKey, 10 * LAMPORTS_PER_SOL);
      await provider.connection.requestAirdrop(botAuthority.publicKey, 5 * LAMPORTS_PER_SOL);
      await new Promise(resolve => setTimeout(resolve, 1000));

      [delegationPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("delegation"), user.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .createDelegation(
          0,
          new anchor.BN(5 * LAMPORTS_PER_SOL),
          10
        )
        .accounts({
          delegation: delegationPda,
          botAuthority: botAuthority.publicKey,
          user: user.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([user])
        .rpc();
    });

    it("Verifies PnL with random profit/loss scenarios", async () => {
      let correctPnLCount = 0;

      for (let i = 0; i < 20; i++) {
        const position = Keypair.generate();
        const tokenMint = Keypair.generate().publicKey;

        const entryAmount = randomInt(100000, LAMPORTS_PER_SOL);
        const multiplier = 0.5 + Math.random() * 3; // 0.5x to 3.5x
        const exitAmount = Math.floor(entryAmount * multiplier);

        try {
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
              position: position.publicKey,
              user: user.publicKey,
              botAuthority: botAuthority.publicKey,
              systemProgram: SystemProgram.programId,
            })
            .signers([botAuthority, position])
            .rpc();

          await program.methods
            .closePosition(
              new anchor.BN(Math.floor(1000000 * multiplier)),
              new anchor.BN(exitAmount)
            )
            .accounts({
              delegation: delegationPda,
              position: position.publicKey,
              botAuthority: botAuthority.publicKey,
            })
            .signers([botAuthority])
            .rpc();

          const positionData = await program.account.position.fetch(position.publicKey);
          const expectedPnL = exitAmount - entryAmount;
          const actualPnL = positionData.pnl.toNumber();

          // Allow small rounding differences
          if (Math.abs(actualPnL - expectedPnL) <= 1) {
            correctPnLCount++;
          } else {
            console.log(`PnL mismatch: expected ${expectedPnL}, got ${actualPnL}`);
          }
        } catch (error) {
          console.log(`PnL test ${i} failed:`, error.message);
        }
      }

      console.log(`Correct PnL calculations: ${correctPnLCount}/20`);
      expect(correctPnLCount).to.equal(20); // All should be correct
    });
  });
});
