'use client';

import { useState, useEffect } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { PublicKey } from '@solana/web3.js';
import { botApi, Strategy } from '@/lib/botApi';
import GlowButton from './GlowButton';

interface DelegationManagerProps {
  onDelegationCreated?: () => void;
}

export default function DelegationManager({ onDelegationCreated }: DelegationManagerProps) {
  const { publicKey, sendTransaction } = useWallet();
  const { connection } = useConnection();

  const [strategies, setStrategies] = useState<Strategy[]>([]);
  const [selectedStrategy, setSelectedStrategy] = useState<string>('conservative');
  const [maxPositionSize, setMaxPositionSize] = useState<number>(0.5);
  const [maxConcurrentTrades, setMaxConcurrentTrades] = useState<number>(3);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');
  const [success, setSuccess] = useState<string>('');

  useEffect(() => {
    loadStrategies();
  }, []);

  const loadStrategies = async () => {
    try {
      const strats = await botApi.getStrategies();
      setStrategies(strats);
    } catch (err) {
      console.error('Failed to load strategies:', err);
    }
  };

  const getStrategyId = (strategyName: string): number => {
    const mapping: { [key: string]: number } = {
      'conservative': 0,
      'ultra_early_sniper': 1,
      'momentum_scalper': 2,
      'graduation_anticipator': 3,
    };
    return mapping[strategyName] || 0;
  };

  const createDelegation = async () => {
    if (!publicKey) {
      setError('Please connect your wallet first');
      return;
    }

    setLoading(true);
    setError('');
    setSuccess('');

    try {
      // TODO: Replace with actual program interaction
      // This is a placeholder for the smart contract interaction

      // For now, we'll simulate the creation
      await new Promise(resolve => setTimeout(resolve, 2000));

      setSuccess('Delegation created successfully! You can now use the selected strategy.');

      if (onDelegationCreated) {
        onDelegationCreated();
      }
    } catch (err: any) {
      setError(err.message || 'Failed to create delegation');
      console.error('Delegation creation error:', err);
    } finally {
      setLoading(false);
    }
  };

  const selectedStrategyInfo = strategies.find(s => s.id === selectedStrategy);

  return (
    <div className="glass-card rounded-2xl p-8 border-2 border-[#00F0FF]/30">
      <h2 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#0066FF] to-[#00F0FF] bg-clip-text text-transparent">
        🤖 Create Trading Delegation
      </h2>

      <div className="space-y-6">
        {/* Strategy Selection */}
        <div>
          <label className="block text-[#00F0FF] font-semibold mb-3">
            Select Strategy
          </label>
          <select
            value={selectedStrategy}
            onChange={(e) => setSelectedStrategy(e.target.value)}
            className="w-full px-4 py-3 glass border-2 border-[#0066FF]/30 rounded-xl text-white focus:border-[#00F0FF] focus:outline-none transition-colors"
          >
            {strategies.map((strategy) => (
              <option key={strategy.id} value={strategy.id} className="bg-[#1A1A2E] text-white">
                {strategy.name} - {strategy.risk_level} Risk - {strategy.target_return}
              </option>
            ))}
          </select>

          {selectedStrategyInfo && (
            <div className="mt-3 p-4 glass rounded-lg border border-[#0066FF]/20">
              <p className="text-gray-300 text-sm mb-2">{selectedStrategyInfo.description}</p>
              <div className="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <span className="text-gray-400">Win Rate:</span>
                  <span className="text-[#39FF14] ml-2">{selectedStrategyInfo.win_rate}</span>
                </div>
                <div>
                  <span className="text-gray-400">Hold Time:</span>
                  <span className="text-[#00F0FF] ml-2">{selectedStrategyInfo.hold_time}</span>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Max Position Size */}
        <div>
          <label className="block text-[#00F0FF] font-semibold mb-3">
            Max Position Size (SOL)
          </label>
          <input
            type="number"
            value={maxPositionSize}
            onChange={(e) => setMaxPositionSize(parseFloat(e.target.value))}
            min="0.01"
            max="10"
            step="0.1"
            className="w-full px-4 py-3 glass border-2 border-[#0066FF]/30 rounded-xl text-white focus:border-[#00F0FF] focus:outline-none transition-colors"
          />
          <p className="text-gray-400 text-sm mt-2">
            Maximum SOL the bot can use per trade
          </p>
        </div>

        {/* Max Concurrent Trades */}
        <div>
          <label className="block text-[#00F0FF] font-semibold mb-3">
            Max Concurrent Trades
          </label>
          <input
            type="number"
            value={maxConcurrentTrades}
            onChange={(e) => setMaxConcurrentTrades(parseInt(e.target.value))}
            min="1"
            max="10"
            step="1"
            className="w-full px-4 py-3 glass border-2 border-[#0066FF]/30 rounded-xl text-white focus:border-[#00F0FF] focus:outline-none transition-colors"
          />
          <p className="text-gray-400 text-sm mt-2">
            Maximum number of positions open at once
          </p>
        </div>

        {/* Risk Summary */}
        <div className="glass p-4 rounded-xl border border-[#39FF14]/20">
          <h3 className="text-[#39FF14] font-bold mb-2">Risk Summary</h3>
          <div className="space-y-1 text-sm text-gray-300">
            <p>Max capital at risk: <span className="text-white font-bold">{(maxPositionSize * maxConcurrentTrades).toFixed(2)} SOL</span></p>
            <p>Strategy: <span className="text-white font-bold">{selectedStrategyInfo?.name}</span></p>
            <p>Risk Level: <span className="text-white font-bold">{selectedStrategyInfo?.risk_level}</span></p>
          </div>
        </div>

        {/* Action Button */}
        <GlowButton
          variant="primary"
          onClick={createDelegation}
          disabled={!publicKey || loading}
          className="w-full"
        >
          {loading ? 'Creating Delegation...' : 'Create Delegation'}
        </GlowButton>

        {/* Messages */}
        {error && (
          <div className="p-4 glass border-2 border-red-500/50 rounded-xl text-red-400">
            ⚠️ {error}
          </div>
        )}
        {success && (
          <div className="p-4 glass border-2 border-[#39FF14]/50 rounded-xl text-[#39FF14]">
            ✅ {success}
          </div>
        )}

        {/* Info Box */}
        <div className="p-4 glass rounded-xl border border-[#0066FF]/20">
          <h4 className="text-white font-bold mb-2">ℹ️ Non-Custodial Trading</h4>
          <ul className="text-gray-300 text-sm space-y-1">
            <li>✅ Funds stay in your wallet</li>
            <li>✅ Bot only has permission to trade</li>
            <li>✅ You can revoke access anytime</li>
            <li>✅ All trades are on-chain and auditable</li>
            <li>⚠️ Only risk what you can afford to lose</li>
          </ul>
        </div>
      </div>
    </div>
  );
}
