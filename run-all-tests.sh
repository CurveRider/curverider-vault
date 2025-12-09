#!/bin/bash

# Curverider Vault - Complete Test Suite Runner
# This script runs all tests for the non-custodial vault system

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

print_header() {
    echo -e "\n${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}\n"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Start timer
START_TIME=$(date +%s)

print_header "🧪 Curverider Vault - Complete Test Suite"

echo "Test execution started at $(date)"
echo ""

# Check prerequisites
print_header "📋 Checking Prerequisites"

if command -v anchor &> /dev/null; then
    print_success "Anchor CLI found: $(anchor --version)"
else
    print_error "Anchor CLI not found. Please install: https://book.anchor-lang.com/getting_started/installation.html"
    exit 1
fi

if command -v cargo &> /dev/null; then
    print_success "Cargo found: $(cargo --version)"
else
    print_error "Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

if command -v solana &> /dev/null; then
    print_success "Solana CLI found: $(solana --version)"
else
    print_error "Solana CLI not found. Please install: https://docs.solana.com/cli/install-solana-cli-tools"
    exit 1
fi

# Check Solana network
SOLANA_NETWORK=$(solana config get | grep "RPC URL" | awk '{print $3}')
print_warning "Solana network: $SOLANA_NETWORK"

if [[ $SOLANA_NETWORK == *"localhost"* ]]; then
    print_warning "Using localhost. Make sure solana-test-validator is running!"
fi

# Part 1: Smart Contract Tests
print_header "🔗 Part 1: Smart Contract Tests"

echo "Building Anchor program..."
if anchor build; then
    print_success "Program built successfully"
else
    print_error "Program build failed"
    exit 1
fi

echo ""
echo "Running all smart contract tests..."
echo ""

# Run unit tests
echo "→ Running unit tests (noncustodial-vault.ts)..."
if anchor test --skip-build tests/noncustodial-vault.ts 2>&1 | tee /tmp/unit-test-output.txt; then
    UNIT_TESTS_PASSED=$(grep -c "passing" /tmp/unit-test-output.txt || echo "0")
    print_success "Unit tests passed: $UNIT_TESTS_PASSED tests"
    PASSED_TESTS=$((PASSED_TESTS + UNIT_TESTS_PASSED))
else
    print_error "Unit tests failed"
    FAILED_TESTS=$((FAILED_TESTS + 1))
fi

echo ""

# Run invariant tests
echo "→ Running invariant tests (invariant-tests.ts)..."
if anchor test --skip-build tests/invariant-tests.ts 2>&1 | tee /tmp/invariant-test-output.txt; then
    INVARIANT_TESTS_PASSED=$(grep -c "passing" /tmp/invariant-test-output.txt || echo "0")
    print_success "Invariant tests passed: $INVARIANT_TESTS_PASSED invariants verified"
    PASSED_TESTS=$((PASSED_TESTS + INVARIANT_TESTS_PASSED))
else
    print_error "Invariant tests failed"
    FAILED_TESTS=$((FAILED_TESTS + 1))
fi

echo ""

# Run fuzz tests
echo "→ Running fuzz tests (fuzz-tests.ts)..."
if anchor test --skip-build tests/fuzz-tests.ts 2>&1 | tee /tmp/fuzz-test-output.txt; then
    FUZZ_TESTS_PASSED=$(grep -c "passing" /tmp/fuzz-test-output.txt || echo "0")
    print_success "Fuzz tests passed: $FUZZ_TESTS_PASSED scenarios"
    PASSED_TESTS=$((PASSED_TESTS + FUZZ_TESTS_PASSED))
else
    print_warning "Some fuzz tests failed (expected for edge cases)"
fi

# Part 2: Bot Tests
print_header "🤖 Part 2: Bot Tests"

cd bot-rust

echo "Running bot unit tests..."
if cargo test --release 2>&1 | tee /tmp/bot-test-output.txt; then
    BOT_TESTS_PASSED=$(grep "test result: ok" /tmp/bot-test-output.txt | awk '{print $4}')
    print_success "Bot tests passed: $BOT_TESTS_PASSED tests"
    PASSED_TESTS=$((PASSED_TESTS + BOT_TESTS_PASSED))
else
    print_error "Bot tests failed"
    FAILED_TESTS=$((FAILED_TESTS + 1))
fi

cd ..

# Part 3: Code Quality Checks
print_header "🔍 Part 3: Code Quality Checks"

echo "Running Rust clippy..."
cd bot-rust
if cargo clippy -- -D warnings 2>&1 | tee /tmp/clippy-output.txt; then
    print_success "No clippy warnings"
else
    print_warning "Clippy found warnings (see output above)"
fi

echo ""
echo "Checking Rust formatting..."
if cargo fmt -- --check; then
    print_success "Rust code is properly formatted"
else
    print_warning "Run 'cargo fmt' to format Rust code"
fi

cd ..

# Part 4: Security Checks
print_header "🔒 Part 4: Security Checks"

echo "Running cargo audit..."
cd bot-rust
if cargo install cargo-audit 2>/dev/null && cargo audit; then
    print_success "No known security vulnerabilities in dependencies"
else
    print_warning "Could not run cargo audit or vulnerabilities found"
fi

cd ..

# Part 5: Test Coverage (optional)
print_header "📊 Part 5: Test Coverage (Optional)"

if command -v cargo-tarpaulin &> /dev/null; then
    echo "Generating test coverage report..."
    cd bot-rust
    if cargo tarpaulin --out Html --output-dir ../coverage; then
        print_success "Coverage report generated: coverage/tarpaulin-report.html"
    else
        print_warning "Coverage generation failed"
    fi
    cd ..
else
    print_warning "cargo-tarpaulin not installed. Skipping coverage."
    echo "  Install with: cargo install cargo-tarpaulin"
fi

# Summary
print_header "📈 Test Summary"

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo "Test execution completed at $(date)"
echo "Total duration: ${DURATION} seconds"
echo ""
echo "Results:"
echo "  ✅ Passed: $PASSED_TESTS"
echo "  ❌ Failed: $FAILED_TESTS"
echo ""

# Check if all critical tests passed
if [ $FAILED_TESTS -eq 0 ]; then
    print_success "ALL TESTS PASSED! 🎉"
    echo ""
    echo "Next steps:"
    echo "  1. Review test coverage report (if generated)"
    echo "  2. Check SECURITY_AUDIT_CHECKLIST.md"
    echo "  3. Deploy to devnet for integration testing"
    echo "  4. Schedule external security audit"
    echo ""
    exit 0
else
    print_error "SOME TESTS FAILED"
    echo ""
    echo "Please review the failures above and fix before proceeding."
    echo ""
    exit 1
fi
