#!/bin/bash

# Curverider Vault - Non-Custodial Setup Script
# This script helps prepare the project for non-custodial deployment

set -e

echo "🚀 Curverider Vault - Non-Custodial Setup"
echo "=========================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check prerequisites
echo "Checking prerequisites..."
echo ""

# Check for required tools
command -v node >/dev/null 2>&1 || { print_error "Node.js is required but not installed. Aborting."; exit 1; }
command -v cargo >/dev/null 2>&1 || { print_error "Rust is required but not installed. Aborting."; exit 1; }
command -v solana >/dev/null 2>&1 || { print_error "Solana CLI is required but not installed. Aborting."; exit 1; }
command -v anchor >/dev/null 2>&1 || { print_error "Anchor CLI is required but not installed. Aborting."; exit 1; }

print_success "All prerequisites found"
echo ""

# Step 1: Update Smart Contract
echo "Step 1: Updating Smart Contract for Non-Custodial"
echo "------------------------------------------------"

if [ -f "programs/curverider-vault/src/lib_noncustodial.rs" ]; then
    cp programs/curverider-vault/src/lib.rs programs/curverider-vault/src/lib_custodial_backup.rs
    print_success "Backed up original contract to lib_custodial_backup.rs"

    cp programs/curverider-vault/src/lib_noncustodial.rs programs/curverider-vault/src/lib.rs
    print_success "Switched to non-custodial contract"
else
    print_error "Non-custodial contract file not found!"
    exit 1
fi

echo ""

# Step 2: Install Bot Dependencies
echo "Step 2: Installing Bot Dependencies"
echo "-----------------------------------"

cd bot-rust
if cargo check; then
    print_success "Bot dependencies installed"
else
    print_error "Failed to install bot dependencies"
    exit 1
fi
cd ..

echo ""

# Step 3: Install Frontend Dependencies
echo "Step 3: Installing Frontend Dependencies"
echo "---------------------------------------"

cd frontend
if npm install; then
    print_success "Frontend dependencies installed"
else
    print_error "Failed to install frontend dependencies"
    exit 1
fi
cd ..

echo ""

# Step 4: Create Environment Files
echo "Step 4: Creating Environment Configuration Files"
echo "-----------------------------------------------"

# Bot .env
if [ ! -f "bot-rust/.env" ]; then
    if [ -f "bot-rust/.env.example" ]; then
        cp bot-rust/.env.example bot-rust/.env
        print_success "Created bot-rust/.env from example"
        print_warning "⚠️  IMPORTANT: Edit bot-rust/.env with your configuration"
    else
        print_error "bot-rust/.env.example not found"
    fi
else
    print_warning "bot-rust/.env already exists, skipping"
fi

# Frontend .env.local
if [ ! -f "frontend/.env.local" ]; then
    cat > frontend/.env.local << 'EOF'
# Bot API URL (update after Railway deployment)
NEXT_PUBLIC_BOT_API_URL=http://localhost:8080

# Solana Configuration
NEXT_PUBLIC_SOLANA_NETWORK=devnet
NEXT_PUBLIC_SOLANA_RPC_URL=https://api.devnet.solana.com

# Program ID (update after contract deployment)
NEXT_PUBLIC_PROGRAM_ID=Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
EOF
    print_success "Created frontend/.env.local"
    print_warning "Update NEXT_PUBLIC_BOT_API_URL after Railway deployment"
else
    print_warning "frontend/.env.local already exists, skipping"
fi

echo ""

# Step 5: Build Smart Contract
echo "Step 5: Building Smart Contract"
echo "-------------------------------"

if anchor build; then
    print_success "Smart contract built successfully"

    # Get the program ID
    PROGRAM_ID=$(solana address -k target/deploy/curverider_vault-keypair.json)
    echo ""
    print_success "Program ID: $PROGRAM_ID"
    echo ""
    print_warning "⚠️  IMPORTANT: Update the program ID in:"
    echo "   - programs/curverider-vault/src/lib.rs (line 3)"
    echo "   - Anchor.toml"
    echo "   - bot-rust/.env (PROGRAM_ID)"
    echo "   - frontend/.env.local (NEXT_PUBLIC_PROGRAM_ID)"
else
    print_error "Failed to build smart contract"
    exit 1
fi

echo ""

# Step 6: Create Deployment Checklist
echo "Step 6: Creating Deployment Checklist"
echo "------------------------------------"

cat > DEPLOYMENT_CHECKLIST.md << 'EOF'
# Deployment Checklist

## Pre-Deployment
- [ ] Update program ID in all files
- [ ] Configure bot-rust/.env with your settings
- [ ] Configure frontend/.env.local
- [ ] Test locally on devnet
- [ ] Audit smart contract (recommended)

## Smart Contract Deployment
- [ ] Build: `anchor build`
- [ ] Deploy to devnet: `anchor deploy`
- [ ] Test on devnet
- [ ] Deploy to mainnet (when ready)
- [ ] Verify deployment

## Bot Deployment (Railway)
- [ ] Create Railway project
- [ ] Connect GitHub repo
- [ ] Set environment variables
- [ ] Deploy
- [ ] Test health endpoint
- [ ] Note down Railway URL

## Frontend Deployment (Vercel)
- [ ] Update NEXT_PUBLIC_BOT_API_URL with Railway URL
- [ ] Create Vercel project
- [ ] Connect GitHub repo
- [ ] Set environment variables
- [ ] Deploy
- [ ] Test end-to-end flow

## Post-Deployment
- [ ] Create test delegation
- [ ] Monitor bot logs
- [ ] Check first trade execution
- [ ] Set up monitoring alerts
- [ ] Announce launch

## Documentation
See:
- DEPLOYMENT_GUIDE.md - Complete deployment instructions
- USER_GUIDE.md - User documentation
- IMPLEMENTATION_SUMMARY.md - Technical overview
EOF

print_success "Created DEPLOYMENT_CHECKLIST.md"

echo ""
echo "=========================================="
echo "✅ Setup Complete!"
echo "=========================================="
echo ""
echo "Next Steps:"
echo ""
echo "1. Update Program ID:"
echo "   Your program ID: $PROGRAM_ID"
echo "   Update in: lib.rs, Anchor.toml, .env files"
echo ""
echo "2. Configure Environment:"
echo "   - Edit bot-rust/.env with your wallet and settings"
echo "   - Edit frontend/.env.local with your configuration"
echo ""
echo "3. Deploy:"
echo "   - Follow DEPLOYMENT_GUIDE.md for complete instructions"
echo "   - Use DEPLOYMENT_CHECKLIST.md to track progress"
echo ""
echo "4. Test:"
echo "   - Deploy to devnet first"
echo "   - Test all functionality"
echo "   - Only then deploy to mainnet"
echo ""
echo "📚 Documentation:"
echo "   - DEPLOYMENT_GUIDE.md - Deployment instructions"
echo "   - USER_GUIDE.md - User documentation"
echo "   - NON_CUSTODIAL_ARCHITECTURE.md - System design"
echo "   - IMPLEMENTATION_SUMMARY.md - What was built"
echo ""
print_success "Ready to deploy! Good luck! 🚀"
