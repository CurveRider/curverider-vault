#!/bin/bash
# Setup script for Curverider Vault

set -e

echo "🚀 Curverider Vault Setup Script"
echo "=================================="

# Load environments
export PATH="/home/codespace/.local/share/solana/install/active_release/bin:$PATH"
source "$HOME/.cargo/env"

# Check installations
echo "✅ Checking installations..."
solana --version
cargo --version

# Install AVM if needed
if ! command -v avm &> /dev/null; then
    echo "📦 Installing Anchor Version Manager..."
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
fi

# Install and use Anchor 0.29.0
echo "📦 Installing Anchor CLI..."
avm install 0.29.0
avm use 0.29.0
anchor --version

# Configure Solana
echo "⚙️  Configuring Solana..."
solana config set --url localhost

# Generate a new keypair if needed
if [ ! -f ~/.config/solana/id.json ]; then
    echo "🔑 Generating new keypair..."
    solana-keygen new --no-bip39-passphrase
fi

echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Start local validator: solana-test-validator"
echo "2. Build program: anchor build"
echo "3. Run tests: anchor test"
