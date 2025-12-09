# Testing Approaches for Solana Programs

## Why TypeScript Tests for Solana Programs?

### Anchor's Default: TypeScript

**Advantages:**
1. **Client-Side Perspective**: Tests how real users/frontends interact with your program
2. **Better RPC Testing**: Easy to test transaction simulation, account fetching, etc.
3. **Anchor Framework Integration**: `anchor test` command handles everything
4. **Web3.js Ecosystem**: Access to full Solana JavaScript ecosystem
5. **Easier Mocking**: Can mock external APIs, wallets, etc.
6. **Cross-Program Tests**: Test interactions between multiple programs easily

**Our TypeScript Tests:**
- `tests/noncustodial-vault.ts` - 25+ comprehensive unit tests
- `tests/invariant-tests.ts` - 15+ system invariants
- `tests/fuzz-tests.ts` - 150+ randomized scenarios

**Example:**
```typescript
it("Creates a delegation account", async () => {
  const tx = await program.methods
    .createDelegation(strategy, maxPosition, maxTrades)
    .accounts({...})
    .signers([user])
    .rpc();

  const delegation = await program.account.delegationAccount.fetch(delegationPda);
  expect(delegation.user.toString()).to.equal(user.publicKey.toString());
});
```

---

## Rust Tests for Solana Programs

### Option 1: Unit Tests in lib.rs

**Best For:**
- Testing pure logic functions
- Validation functions
- Math calculations
- Enum/struct behaviors

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_validation() {
        assert!(is_valid_strategy(0));
        assert!(is_valid_strategy(3));
        assert!(!is_valid_strategy(4));
    }

    #[test]
    fn test_pnl_calculation() {
        let pnl = calculate_pnl(1_000_000, 2_000_000);
        assert_eq!(pnl, 1_000_000);
    }
}
```

**Run with:**
```bash
cd programs/curverider-vault
cargo test
```

---

### Option 2: Integration Tests with solana-program-test

**Best For:**
- Testing full program instructions
- Account state transitions
- CPI (Cross-Program Invocation)
- Complex instruction sequences

**Setup:**
```toml
# Cargo.toml
[dev-dependencies]
solana-program-test = "1.16"
solana-sdk = "1.16"
tokio = "1.0"
```

**Example:**
```rust
#[cfg(test)]
mod integration_tests {
    use solana_program_test::*;
    use solana_sdk::signature::{Keypair, Signer};

    #[tokio::test]
    async fn test_create_delegation() {
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new(
            "curverider_vault",
            program_id,
            processor!(process_instruction),
        );

        let (mut banks_client, payer, recent_blockhash) =
            program_test.start().await;

        // Create and send transaction
        let mut transaction = Transaction::new_with_payer(
            &[create_delegation_ix],
            Some(&payer.pubkey()),
        );

        transaction.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
    }
}
```

---

## Comparison Table

| Feature | TypeScript Tests | Rust Unit Tests | Rust Integration Tests |
|---------|-----------------|-----------------|----------------------|
| **Setup** | ✅ Easy (anchor test) | ✅ Easy (cargo test) | ⚠️ Medium (solana-program-test) |
| **Speed** | ⚠️ Slower (spins up validator) | ✅ Very fast | ⚠️ Slower |
| **RPC Testing** | ✅ Excellent | ❌ No | ✅ Good |
| **Pure Logic** | ⚠️ Harder | ✅ Perfect | ✅ Good |
| **Client Simulation** | ✅ Perfect | ❌ No | ⚠️ Limited |
| **IDE Support** | ✅ Good | ✅ Excellent | ✅ Good |
| **Debugging** | ⚠️ Harder | ✅ Easier | ⚠️ Harder |

---

## Recommended Hybrid Approach

### Use TypeScript for:
1. **End-to-End Tests**: Full user workflows
2. **Integration Tests**: Multi-program interactions
3. **RPC Behavior**: Account fetching, simulations
4. **Client-Side Logic**: How frontends will use your program

### Use Rust for:
1. **Unit Tests**: Pure functions, calculations
2. **Validation Logic**: Input checking, constraints
3. **Business Logic**: Core algorithm testing
4. **Performance**: Benchmarking critical paths

---

## Our Testing Strategy

### 1. TypeScript Tests (Primary)
```
tests/
├── noncustodial-vault.ts    # 25+ functional tests
├── invariant-tests.ts        # 15+ system invariants
└── fuzz-tests.ts             # 150+ random scenarios
```

**Coverage:**
- ✅ Full delegation lifecycle
- ✅ Position management
- ✅ Security & authorization
- ✅ Edge cases & errors
- ✅ State consistency
- ✅ PnL calculations

### 2. Rust Tests (Supplementary)
```
programs/curverider-vault/src/
└── lib.rs
    └── #[cfg(test)] mod tests { ... }
```

**Coverage:**
- ✅ Input validation
- ✅ Strategy enum checks
- ✅ Math operations
- ✅ Status transitions
- ✅ Helper functions

### 3. Bot Tests (Rust)
```
bot-rust/src/tests/
├── analyzer_tests.rs         # Strategy logic
├── integration_tests.rs      # End-to-end flows
└── mod.rs                    # Test modules
```

**Coverage:**
- ✅ All 4 trading strategies
- ✅ Signal generation
- ✅ Exit parameters
- ✅ Performance testing

---

## Running All Tests

### Quick Test (TypeScript only)
```bash
anchor test
```

### Complete Suite (TypeScript + Rust)
```bash
# Run our comprehensive test script
./run-all-tests.sh
```

### Individual Test Suites
```bash
# TypeScript tests
anchor test tests/noncustodial-vault.ts
anchor test tests/invariant-tests.ts
anchor test tests/fuzz-tests.ts

# Rust program tests
cd programs/curverider-vault && cargo test

# Rust bot tests
cd bot-rust && cargo test
```

---

## Best Practices

### TypeScript Tests
```typescript
// ✅ Good: Comprehensive test with verification
it("Opens position with all checks", async () => {
  const delegationBefore = await program.account.delegationAccount.fetch(pda);

  await program.methods.openPosition(...).rpc();

  const delegationAfter = await program.account.delegationAccount.fetch(pda);
  const position = await program.account.position.fetch(positionPda);

  expect(delegationAfter.activeTrades).to.equal(delegationBefore.activeTrades + 1);
  expect(position.status).to.equal(0); // Open
});

// ❌ Bad: No verification
it("Opens position", async () => {
  await program.methods.openPosition(...).rpc();
  // No assertions!
});
```

### Rust Tests
```rust
// ✅ Good: Test pure logic
#[test]
fn test_pnl_with_edge_cases() {
    assert_eq!(calculate_pnl(100, 200), 100);
    assert_eq!(calculate_pnl(100, 50), -50);
    assert_eq!(calculate_pnl(100, 100), 0);
    assert_eq!(calculate_pnl(0, 100), 100); // Edge case
}

// ❌ Bad: Testing RPC behavior in Rust
#[test]
fn test_fetch_account() {
    // Can't easily test this in Rust unit tests
    // Use TypeScript or solana-program-test instead
}
```

---

## For Auditors

When reviewing this codebase for security audit:

### Start Here:
1. **Run full test suite**: `./run-all-tests.sh`
2. **Review TypeScript tests**: See `tests/*.ts` for complete coverage
3. **Check invariants**: `tests/invariant-tests.ts` - critical system properties
4. **Examine fuzz tests**: `tests/fuzz-tests.ts` - edge case discovery

### Key Test Files:
- `tests/noncustodial-vault.ts` - Security & authorization tests
- `tests/invariant-tests.ts` - Mathematical & logical invariants
- `tests/fuzz-tests.ts` - Randomized input testing
- `bot-rust/src/tests/*` - Bot logic verification

### Test Coverage:
- Smart Contract: >90% (see test output)
- Bot: >85% (run `cargo tarpaulin`)
- Integration: End-to-end flows covered

---

## Continuous Integration

Our CI runs all tests automatically:

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]

jobs:
  anchor-tests:
    - name: Run Anchor Tests
      run: anchor test

  rust-tests:
    - name: Run Rust Tests
      run: cd bot-rust && cargo test --release
```

---

## Summary

**Why TypeScript Tests?**
→ Better for integration, RPC, and client-side testing
→ Anchor's default and recommended approach
→ Easier to simulate real-world usage

**Why Rust Tests?**
→ Perfect for unit testing pure logic
→ Faster execution for specific functions
→ Better debugging with Rust tools

**Our Approach:**
→ **Primary**: TypeScript tests (75+ tests, comprehensive)
→ **Secondary**: Rust tests (unit tests for validation logic)
→ **Result**: Complete coverage from all angles

**For Mainnet:**
→ All TypeScript tests must pass (100%)
→ All Rust tests must pass (100%)
→ All invariants must hold (100%)
→ Fuzz tests must have >90% success rate

---

*See also:*
- [TESTING_DOCUMENTATION.md](TESTING_DOCUMENTATION.md) - How to run tests
- [SECURITY_AUDIT_CHECKLIST.md](SECURITY_AUDIT_CHECKLIST.md) - Security audit guide
- [Anchor Testing Guide](https://book.anchor-lang.com/anchor_in_depth/the_test_module.html)
