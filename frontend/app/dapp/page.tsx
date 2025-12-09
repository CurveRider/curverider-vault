'use client';

import { useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import { useEffect, useState, useRef } from 'react';
import Link from 'next/link';
import { gsap } from 'gsap';
import Hero3D from '@/components/Hero3D';
import GlowButton from '@/components/GlowButton';
import StrategySelector, { StrategyType } from '@/components/StrategySelector';

export default function DAppPage() {
  const { connection } = useConnection();
  const { publicKey } = useWallet();
  const [balance, setBalance] = useState<number | null>(null);
  const [selectedStrategy, setSelectedStrategy] = useState<StrategyType>('conservative');
  const pageRef = useRef<HTMLDivElement>(null);
  const cardsRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (publicKey) {
      connection.getBalance(publicKey).then((bal) => {
        setBalance(bal / LAMPORTS_PER_SOL);
      });
    } else {
      setBalance(null);
    }
  }, [publicKey, connection]);

  useEffect(() => {
    if (publicKey && cardsRef.current) {
      const cards = cardsRef.current.querySelectorAll('.animate-card');
      gsap.from(cards, {
        opacity: 0,
        y: 30,
        stagger: 0.1,
        duration: 0.8,
        ease: 'power3.out',
      });
    }
  }, [publicKey]);

  return (
    <div ref={pageRef} className="min-h-screen animated-gradient relative overflow-hidden">
      {/* 3D Background */}
      <Hero3D />

      <div className="container mx-auto px-4 py-8 relative z-10">
        <nav className="flex justify-between items-center mb-12">
          <Link href="/" className="text-2xl font-bold bg-gradient-to-r from-[#0066FF] to-[#00F0FF] bg-clip-text text-transparent hover:opacity-80 transition-opacity">
            ← Curverider Vault
          </Link>
          <WalletMultiButton className="!bg-gradient-to-r from-[#0066FF] to-[#00F0FF] hover:shadow-[0_0_30px_rgba(0,240,255,0.6)] !transition-all" />
        </nav>

        <div className="max-w-6xl mx-auto">
          <h1 className="text-4xl md:text-6xl font-black text-white mb-8 text-center neon-glow-blue">
            Curverider Vault <span className="text-[#39FF14]">dApp</span>
          </h1>

          {publicKey ? (
            <div ref={cardsRef} className="space-y-6">
              {/* Wallet Info Card */}
              <div className="glass-card rounded-2xl p-8 animate-card border-2 border-[#00F0FF]/30">
                <h2 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#0066FF] to-[#00F0FF] bg-clip-text text-transparent">
                  💎 Wallet Info
                </h2>
                <div className="space-y-4 text-gray-300">
                  <div className="flex flex-col gap-2">
                    <span className="font-semibold text-[#00F0FF]">Address:</span>
                    <span className="font-mono text-sm break-all glass p-3 rounded-lg border border-[#0066FF]/20">
                      {publicKey.toString()}
                    </span>
                  </div>
                  {balance !== null && (
                    <div className="flex justify-between items-center glass p-4 rounded-xl border border-[#39FF14]/20">
                      <span className="font-semibold text-[#39FF14]">Balance:</span>
                      <span className="text-2xl font-bold text-white">{balance.toFixed(4)} SOL</span>
                    </div>
                  )}
                </div>
              </div>

              {/* Operations & Stats Grid */}
              <div className="grid md:grid-cols-2 gap-6">
                {/* Vault Operations */}
                <div className="glass-card rounded-2xl p-8 animate-card">
                  <h3 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#39FF14] to-[#00F0FF] bg-clip-text text-transparent">
                    ⚡ Vault Operations
                  </h3>
                  <div className="space-y-4">
                    <button className="w-full px-6 py-4 bg-gradient-to-r from-[#39FF14] to-[#00F0FF] text-black rounded-xl hover:shadow-[0_0_30px_rgba(57,255,20,0.6)] transition-all font-bold text-lg neon-border">
                      Deposit to Vault
                    </button>
                    <button className="w-full px-6 py-4 glass border-2 border-[#0066FF] text-white rounded-xl hover:shadow-[0_0_30px_rgba(0,102,255,0.6)] transition-all font-bold text-lg">
                      Withdraw from Vault
                    </button>
                  </div>
                </div>

                {/* Vault Stats */}
                <div className="glass-card rounded-2xl p-8 animate-card">
                  <h3 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#0066FF] to-[#39FF14] bg-clip-text text-transparent">
                    📊 Vault Stats
                  </h3>
                  <div className="space-y-4">
                    <div className="flex justify-between items-center glass p-4 rounded-xl border border-[#00F0FF]/20">
                      <span className="text-gray-400">Total Value Locked:</span>
                      <span className="font-bold text-white text-xl">2.4M SOL</span>
                    </div>
                    <div className="flex justify-between items-center glass p-4 rounded-xl border border-[#0066FF]/20">
                      <span className="text-gray-400">Your Position:</span>
                      <span className="font-bold text-white text-xl">-- SOL</span>
                    </div>
                    <div className="flex justify-between items-center glass p-4 rounded-xl border border-[#39FF14]/20 pulse-glow">
                      <span className="text-gray-400">APY:</span>
                      <span className="font-bold text-[#39FF14] text-2xl">156%</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Solana Actions & Blinks */}
              <div className="glass-card rounded-2xl p-8 animate-card">
                <h3 className="text-2xl font-bold mb-4 bg-gradient-to-r from-[#00F0FF] to-[#0066FF] bg-clip-text text-transparent">
                  🔗 Solana Actions & Blinks
                </h3>
                <p className="text-gray-300 mb-6 leading-relaxed">
                  Integrate Solana Actions and Blinks for seamless transactions and real-time notifications.
                </p>
                <div className="grid md:grid-cols-3 gap-4">
                  <button className="px-6 py-3 glass border border-[#0066FF]/50 text-white rounded-xl hover:border-[#0066FF] hover:shadow-[0_0_20px_rgba(0,102,255,0.4)] transition-all font-semibold">
                    Create Action
                  </button>
                  <button className="px-6 py-3 glass border border-[#00F0FF]/50 text-white rounded-xl hover:border-[#00F0FF] hover:shadow-[0_0_20px_rgba(0,240,255,0.4)] transition-all font-semibold">
                    View Blinks
                  </button>
                  <button className="px-6 py-3 glass border border-[#39FF14]/50 text-white rounded-xl hover:border-[#39FF14] hover:shadow-[0_0_20px_rgba(57,255,20,0.4)] transition-all font-semibold">
                    Configure
                  </button>
                </div>
              </div>

              {/* Strategy Selector */}
              <StrategySelector
                selectedStrategy={selectedStrategy}
                onStrategyChange={setSelectedStrategy}
              />

              {/* Recent Activity */}
              <div className="glass-card rounded-2xl p-8 animate-card">
                <h3 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#39FF14] to-[#0066FF] bg-clip-text text-transparent">
                  📈 Recent Activity
                </h3>
                <div className="text-center py-12">
                  <div className="text-6xl mb-4 opacity-50">⚡</div>
                  <p className="text-gray-400 text-lg">No recent activity</p>
                  <p className="text-gray-500 text-sm mt-2">Your transactions will appear here</p>
                </div>
              </div>
            </div>
          ) : (
            <div className="glass-card rounded-3xl p-16 text-center border-2 border-[#00F0FF]/30">
              <div className="text-8xl mb-8 animate-pulse">🔐</div>
              <h2 className="text-3xl font-bold text-white mb-6 neon-glow-blue">
                Connect Your Wallet to Get Started
              </h2>
              <p className="text-xl text-gray-300 mb-10 max-w-2xl mx-auto leading-relaxed">
                Connect your Solana wallet to access the <span className="text-[#39FF14] font-bold">Curverider Vault</span> and start earning with autonomous DeFi strategies
              </p>
              <WalletMultiButton className="!bg-gradient-to-r from-[#0066FF] to-[#00F0FF] hover:shadow-[0_0_40px_rgba(0,240,255,0.8)] !transition-all !text-lg !py-4 !px-8 !rounded-xl !font-bold !mx-auto" />
              
              {/* Features Preview */}
              <div className="grid md:grid-cols-3 gap-6 mt-16">
                <div className="glass p-6 rounded-xl border border-[#0066FF]/20">
                  <div className="text-3xl mb-3">🚀</div>
                  <h4 className="text-[#00F0FF] font-bold mb-2">Fast Execution</h4>
                  <p className="text-gray-400 text-sm">Lightning-fast transactions on Solana</p>
                </div>
                <div className="glass p-6 rounded-xl border border-[#39FF14]/20">
                  <div className="text-3xl mb-3">🔒</div>
                  <h4 className="text-[#39FF14] font-bold mb-2">Secure & Safe</h4>
                  <p className="text-gray-400 text-sm">Non-custodial, audited contracts</p>
                </div>
                <div className="glass p-6 rounded-xl border border-[#00F0FF]/20">
                  <div className="text-3xl mb-3">💰</div>
                  <h4 className="text-[#0066FF] font-bold mb-2">High Yields</h4>
                  <p className="text-gray-400 text-sm">Automated profit optimization</p>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
