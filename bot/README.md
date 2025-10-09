# Curverider Vault Bot

Off-chain TypeScript trading bot for the Curverider Vault system.

## Setup

1. Install dependencies:
```bash
npm install
```

2. Copy `.env.example` to `.env` and configure:
```bash
cp .env.example .env
```

3. Edit `.env` with your settings:
- `RPC_URL`: Your Solana RPC endpoint
- `PRIVATE_KEY`: Your wallet private key (base58 encoded)
- `PROGRAM_ID`: The deployed program ID

## Development

Build the bot:
```bash
npm run build
```

Run in development mode:
```bash
npm run dev
```

Run the built version:
```bash
npm start
```

## Features

- Token discovery and monitoring
- Automated sniping on bonding curves
- DEX trading automation
- Integration with Curverider Vault program
