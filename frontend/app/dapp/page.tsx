'use client';

import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import { useEffect, useState } from 'react';
import Link from 'next/link';

export default function DAppPage() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState<number | null>(null);

  useEffect(() => {
    if (publicKey) {
      connection.getBalance(publicKey).then((bal) => {
        setBalance(bal / LAMPORTS_PER_SOL);
      });
    } else {
      setBalance(null);
    }
  }, [publicKey, connection]);

  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-black">
      <div className="container mx-auto px-4 py-8">
        <nav className="flex justify-between items-center mb-12">
          <Link href="/" className="text-2xl font-bold text-white hover:text-gray-300">
            ← Curverider Vault
          </Link>
          <WalletMultiButton className="!bg-gradient-to-r from-purple-500 to-blue-500 hover:from-purple-600 hover:to-blue-600" />
        </nav>

        <div className="max-w-6xl mx-auto">
          <h1 className="text-4xl md:text-5xl font-bold text-white mb-8 text-center">
            Curverider Vault dApp
          </h1>

          {publicKey ? (
            <div className="space-y-6">
              <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6">
                <h2 className="text-2xl font-semibold text-white mb-4">Wallet Info</h2>
                <div className="space-y-2 text-gray-300">
                  <p>
                    <span className="font-semibold">Address:</span>{' '}
                    <span className="font-mono text-sm break-all">{publicKey.toString()}</span>
                  </p>
                  {balance !== null && (
                    <p>
                      <span className="font-semibold">Balance:</span> {balance.toFixed(4)} SOL
                    </p>
                  )}
                </div>
              </div>

              <div className="grid md:grid-cols-2 gap-6">
                <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6">
                  <h3 className="text-xl font-semibold text-white mb-4">Vault Operations</h3>
                  <div className="space-y-4">
                    <button className="w-full px-6 py-3 bg-gradient-to-r from-green-500 to-emerald-500 text-white rounded-lg hover:from-green-600 hover:to-emerald-600 transition-all font-semibold">
                      Deposit to Vault
                    </button>
                    <button className="w-full px-6 py-3 bg-gradient-to-r from-orange-500 to-red-500 text-white rounded-lg hover:from-orange-600 hover:to-red-600 transition-all font-semibold">
                      Withdraw from Vault
                    </button>
                  </div>
                </div>

                <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6">
                  <h3 className="text-xl font-semibold text-white mb-4">Vault Stats</h3>
                  <div className="space-y-3 text-gray-300">
                    <div className="flex justify-between">
                      <span>Total Value Locked:</span>
                      <span className="font-semibold text-white">-- SOL</span>
                    </div>
                    <div className="flex justify-between">
                      <span>Your Position:</span>
                      <span className="font-semibold text-white">-- SOL</span>
                    </div>
                    <div className="flex justify-between">
                      <span>APY:</span>
                      <span className="font-semibold text-green-400">--%</span>
                    </div>
                  </div>
                </div>
              </div>

              <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6">
                <h3 className="text-xl font-semibold text-white mb-4">Solana Actions & Blinks</h3>
                <p className="text-gray-300 mb-4">
                  Integrate Solana Actions and Blinks for seamless transactions and notifications.
                </p>
                <div className="grid md:grid-cols-3 gap-4">
                  <button className="px-4 py-2 bg-purple-500/50 text-white rounded-lg hover:bg-purple-500/70 transition-all">
                    Create Action
                  </button>
                  <button className="px-4 py-2 bg-blue-500/50 text-white rounded-lg hover:bg-blue-500/70 transition-all">
                    View Blinks
                  </button>
                  <button className="px-4 py-2 bg-indigo-500/50 text-white rounded-lg hover:bg-indigo-500/70 transition-all">
                    Configure
                  </button>
                </div>
              </div>

              <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6">
                <h3 className="text-xl font-semibold text-white mb-4">Recent Activity</h3>
                <p className="text-gray-400 text-center py-8">No recent activity</p>
              </div>
            </div>
          ) : (
            <div className="bg-white/10 backdrop-blur-lg rounded-xl p-12 text-center">
              <h2 className="text-2xl font-semibold text-white mb-4">
                Connect Your Wallet to Get Started
              </h2>
              <p className="text-gray-300 mb-8">
                Connect your Solana wallet to interact with the Curverider Vault
              </p>
              <WalletMultiButton className="!bg-gradient-to-r from-purple-500 to-blue-500 hover:from-purple-600 hover:to-blue-600 !mx-auto" />
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
