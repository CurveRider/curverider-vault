# 🚀 Curverider Vault - Complete DeFi Trading Platform

A full-stack Solana-based decentralized trading platform with automated bot strategies and stunning UI.

## 📦 Project Structure

```
curverider-vault/
├── frontend/                  # Next.js 15 frontend with Electric Blue/Neon Green theme
│   ├── app/                   # Pages and layouts
│   │   ├── page.tsx          # Homepage with 3D animations
│   │   ├── dapp/             # dApp interface
│   │   └── globals.css       # Glassmorphism design system
│   └── components/           # Reusable React components
│       ├── Hero3D.tsx        # Three.js floating orbs
│       ├── AnimatedCard.tsx  # 3D card animations
│       ├── GlowButton.tsx    # Interactive buttons
│       └── ...               # 9+ components
│
├── bot-rust/                 # Rust trading bot
│   ├── src/
│   │   ├── main.rs          # Trading loop orchestrator
│   │   ├── analyzer.rs      # 6-factor token analysis
│   │   ├── scanner.rs       # pump.fun API integration
│   │   ├── trader.rs        # Trade execution
│   │   └── types.rs         # Data structures
│   └── README.md            # Bot documentation
│
├── programs/                 # Solana smart contracts
│   └── curverider-vault/
│       └── src/
│           └── lib.rs       # Anchor program (8 instructions)
│
└── tests/                    # Anchor test suite
    └── curverider-vault.ts  # Comprehensive tests
```

## 🎨 Design System

**Color Palette:**
- Electric Blue: `#0066FF` - Primary actions, links
- Neon Green: `#39FF14` - Success, highlights
- Cyber Cyan: `#00F0FF` - Accents, gradients
- Midnight Black: `#0A0A0F` - Backgrounds
- Deep Space: `#1A1A2E` - Cards, surfaces

**Features:**
- Glassmorphism effects with backdrop blur
- 3D GSAP animations
- Three.js floating orbs
- Neon glow effects
- Responsive design
- Dark mode optimized

## 🤖 Trading Bot

**Strategy:** Multi-factor token analysis
- Volume Analysis (25%) - Trading activity trends
- Liquidity Score (20%) - Market depth evaluation
- Holder Distribution (15%) - Ownership patterns
- Momentum Score (20%) - Price movement velocity
- Buy Pressure (10%) - Buy/sell ratio
- Bonding Curve (10%) - Token graduation tracking

**Performance:**
- Built in Rust for maximum speed
- Async/await with Tokio
- Real-time scanning of pump.fun tokens
- Automatic position management
- Stop-loss and take-profit automation

## 🔐 Smart Contract

**Anchor Program Features:**
- Share-based vault system
- User deposit/withdrawal
- Trading position tracking
- PnL calculation
- Fee management (mgmt + performance)
- Authority-based access control

**Instructions:**
1. `initialize_vault` - Setup vault with config
2. `deposit` - Users deposit SOL for shares
3. `withdraw` - Users redeem shares for SOL
4. `open_position` - Bot opens trade
5. `close_position` - Bot closes trade
6. `update_vault_config` - Admin updates settings
7. `claim_fees` - Admin claims accrued fees

## 🚀 Quick Start

### 1. Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

Open http://localhost:3000

### 2. Bot Setup

```bash
cd bot-rust
cp .env.example .env
# Configure your wallet and RPC
cargo build --release
./target/release/bot-rust
```

### 3. Smart Contract Setup

```bash
# Install dependencies
./setup.sh

# Start local validator (separate terminal)
solana-test-validator

# Build and test
anchor build
anchor test
```

## 📚 Documentation

- **Frontend**
  - [Design System](frontend/DESIGN_SYSTEM.md)
  - [Quick Start](frontend/QUICKSTART.md)
  - [Visual Preview](frontend/VISUAL_PREVIEW.md)
  - [Component Library](frontend/COMPONENT_LIBRARY.md)

- **Bot**
  - [Bot README](bot-rust/README.md)
  - [Quick Start](bot-rust/QUICKSTART.md)
  - [Trading Strategy](bot-rust/STRATEGY.md)

- **Smart Contract**
  - [Testing Guide](TESTING_GUIDE.md)
  - [Anchor Documentation](https://book.anchor-lang.com/)

## 🔧 Technology Stack

### Frontend
- Next.js 15.5.4
- React 19.1.0
- Tailwind CSS 4
- GSAP 3.x
- Three.js & @react-three/fiber
- @solana/wallet-adapter

### Backend (Bot)
- Rust 1.70+
- Tokio async runtime
- Solana SDK 1.18
- Anchor Client 0.29
- reqwest for HTTP

### Smart Contract
- Anchor Framework 0.29.0
- Solana 1.18
- SPL Token
- Rust 1.70+

## 🧪 Testing

### Frontend Tests
```bash
cd frontend
npm run lint
npm run build
```

### Bot Tests
```bash
cd bot-rust
cargo test
cargo clippy
```

### Smart Contract Tests
```bash
anchor test
# or
./test.sh
```

## 📊 Test Coverage

**Smart Contract Tests:**
- ✅ Vault initialization
- ✅ User deposits (1:1 and proportional shares)
- ✅ Deposit validation (min/max)
- ✅ Position opening/closing
- ✅ PnL tracking
- ✅ Withdrawals (partial and full)
- ✅ Configuration updates
- ✅ Fee validation
- ✅ Error cases

**Expected Results:**
- All tests pass ✅
- Vault statistics accurate
- Share calculations correct
- PnL tracking functional

## 🎯 Roadmap

### Phase 1: MVP (Current) ✅
- [x] Frontend design system
- [x] Trading bot implementation
- [x] Smart contract development
- [x] Test suite

### Phase 2: Integration 🔄
- [ ] Connect bot to on-chain program
- [ ] Frontend wallet integration
- [ ] Real-time stats display
- [ ] Position monitoring UI

### Phase 3: Deployment 📋
- [ ] Deploy to devnet
- [ ] Beta testing
- [ ] Audit smart contracts
- [ ] Mainnet deployment

### Phase 4: Enhancement 🚀
- [ ] Advanced trading strategies
- [ ] Multi-vault support
- [ ] Social features
- [ ] Mobile app

## 🤝 Contributing

This is a hackathon demo project. For the full version:
1. Audit smart contracts
2. Add comprehensive error handling
3. Implement rate limiting
4. Add monitoring/logging
5. Security hardening

## 📄 License

MIT License - See LICENSE file for details

## 🎉 Acknowledgments

- Solana Foundation
- Anchor Framework
- pump.fun API
- GSAP Animation Library
- Three.js Community

## 📞 Support

For questions or issues:
- GitHub Issues
- Discord: [your-discord]
- Twitter: [your-twitter]

---

Built with ⚡ by the Curverider team for [Hackathon Name]
