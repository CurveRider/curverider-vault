# Curverider Vault Frontend

Next.js TypeScript frontend for the Curverider Vault project, featuring a landing page and dApp interface with Solana wallet integration.

## Features

- **Landing Page**: Showcases the Curverider Vault project with key features
- **dApp Interface**: Interactive interface for vault operations with Solana wallet integration
- **Solana Actions & Blinks**: Integration points for Solana Actions and Blinks functionality
- **Wallet Integration**: Support for Phantom, Solflare, and other Solana wallets

## Getting Started

### Prerequisites

- Node.js 18+ and npm

### Installation

1. Install dependencies:
```bash
npm install
```

2. Run the development server:
```bash
npm run dev
```

3. Open [http://localhost:3000](http://localhost:3000) in your browser

### Build for Production

```bash
npm run build
npm start
```

## Project Structure

```
frontend/
├── app/
│   ├── page.tsx           # Landing page
│   ├── dapp/
│   │   └── page.tsx       # dApp interface
│   ├── layout.tsx         # Root layout with wallet provider
│   └── globals.css        # Global styles
├── components/
│   └── WalletProvider.tsx # Solana wallet provider setup
└── public/                # Static assets
```

## Key Technologies

- **Next.js 15**: React framework with App Router
- **TypeScript**: Type-safe development
- **Tailwind CSS**: Utility-first styling
- **Solana Web3.js**: Solana blockchain interaction
- **Wallet Adapter**: Solana wallet integration

## Development

The frontend integrates with the Curverider Vault program and bot:
- Connect to Solana devnet by default (configurable in WalletProvider.tsx)
- Interact with vault smart contracts
- Display real-time wallet and vault information
- Support for Solana Actions and Blinks

## Solana Actions & Blinks

The dApp includes integration points for:
- **Solana Actions**: Executable blockchain actions
- **Blinks**: Blockchain links for social sharing and notifications

These features are ready for implementation based on the Solana Actions specification.

## Learn More

- [Next.js Documentation](https://nextjs.org/docs)
- [Solana Web3.js Documentation](https://solana-labs.github.io/solana-web3.js/)
- [Solana Wallet Adapter](https://github.com/solana-labs/wallet-adapter)
