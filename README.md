# curverider-vault
A fully autonomous DeFi strategy vault on Solana that profits from pump.fun meta. The system discovers trending tokens, snipes them on the bonding curve, and automatically trades them when they hit a DEX. Built for the Cypherpunk hackathon.

## Project Structure

This project consists of two main components:

### 1. Program (`programs/curverider-vault/`)
The on-chain Anchor smart contract that manages the vault logic.

**Key files:**
- `programs/curverider-vault/src/lib.rs` - Main program logic
- `programs/curverider-vault/Cargo.toml` - Rust dependencies
- `Anchor.toml` - Anchor configuration

### 2. Bot (`bot/`)
The off-chain TypeScript trading bot that interacts with the program.

**Key files:**
- `bot/src/index.ts` - Main bot entry point
- `bot/package.json` - Node.js dependencies
- `bot/tsconfig.json` - TypeScript configuration

## Getting Started

### Prerequisites
- Rust and Cargo
- Solana CLI tools
- Anchor CLI
- Node.js and npm/yarn

### Installation

1. Install dependencies:
```bash
# Install root dependencies
npm install

# Install bot dependencies
cd bot && npm install
```

2. Build the program:
```bash
anchor build
```

3. Run tests:
```bash
anchor test
```

4. Configure and run the bot:
```bash
cd bot
cp .env.example .env
# Edit .env with your configuration
npm run dev
```

## Development

- `anchor build` - Build the Solana program
- `anchor test` - Run program tests
- `anchor deploy` - Deploy to configured cluster
- `cd bot && npm run dev` - Run the trading bot in development mode

## License
ISC