#!/bin/bash
# Quick test script for Curverider Vault

set -e

# Load environments
export PATH="/home/codespace/.local/share/solana/install/active_release/bin:$PATH"
export PATH="$HOME/.avm/bin:$PATH"
source "$HOME/.cargo/env"

echo "🧪 Running Curverider Vault Tests"
echo "=================================="

# Build the program
echo "📦 Building program..."
anchor build

# Update program ID in lib.rs and Anchor.toml if needed
PROGRAM_ID=$(solana address -k target/deploy/curverider_vault-keypair.json)
echo "📝 Program ID: $PROGRAM_ID"

# Run tests
echo "🧪 Running tests..."
anchor test --skip-local-validator

echo "✅ Tests complete!"
