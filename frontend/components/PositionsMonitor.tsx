'use client';

import { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { botApi, Position, UserStats } from '@/lib/botApi';

export default function PositionsMonitor() {
  const { publicKey } = useWallet();
  const [positions, setPositions] = useState<Position[]>([]);
  const [stats, setStats] = useState<UserStats | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string>('');

  useEffect(() => {
    if (publicKey) {
      loadData();
      const interval = setInterval(loadData, 5000); // Refresh every 5 seconds
      return () => clearInterval(interval);
    }
  }, [publicKey]);

  const loadData = async () => {
    if (!publicKey) return;

    setLoading(true);
    try {
      const [positionsData, statsData] = await Promise.all([
        botApi.getUserPositions(publicKey.toString()),
        botApi.getUserStats(publicKey.toString()),
      ]);

      setPositions(positionsData);
      setStats(statsData);
      setError('');
    } catch (err: any) {
      // If user hasn't created delegation yet, don't show error
      if (err.message.includes('404')) {
        setError('');
      } else {
        setError('Failed to load data');
      }
      console.error('Failed to load positions:', err);
    } finally {
      setLoading(false);
    }
  };

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleString();
  };

  const formatSOL = (lamports: number) => {
    return (lamports / 1_000_000_000).toFixed(4);
  };

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'open':
        return 'text-[#00F0FF]';
      case 'closed':
        return 'text-gray-400';
      case 'liquidated':
        return 'text-red-400';
      default:
        return 'text-white';
    }
  };

  const getPnLColor = (pnl: number) => {
    if (pnl > 0) return 'text-[#39FF14]';
    if (pnl < 0) return 'text-red-400';
    return 'text-gray-400';
  };

  if (!publicKey) {
    return (
      <div className="glass-card rounded-2xl p-8 border-2 border-[#0066FF]/30 text-center">
        <p className="text-gray-400">Connect your wallet to view positions</p>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Stats Card */}
      {stats && (
        <div className="glass-card rounded-2xl p-6 border-2 border-[#39FF14]/30">
          <h3 className="text-xl font-bold text-white mb-4">📊 Your Trading Stats</h3>
          <div className="grid md:grid-cols-4 gap-4">
            <div>
              <p className="text-gray-400 text-sm">Strategy</p>
              <p className="text-white font-bold">{stats.strategy}</p>
            </div>
            <div>
              <p className="text-gray-400 text-sm">Total Trades</p>
              <p className="text-white font-bold">{stats.total_trades}</p>
            </div>
            <div>
              <p className="text-gray-400 text-sm">Win Rate</p>
              <p className="text-[#39FF14] font-bold">{stats.win_rate.toFixed(1)}%</p>
            </div>
            <div>
              <p className="text-gray-400 text-sm">Total P&L</p>
              <p className={`font-bold ${getPnLColor(stats.total_pnl_sol)}`}>
                {stats.total_pnl_sol >= 0 ? '+' : ''}{stats.total_pnl_sol.toFixed(4)} SOL
              </p>
            </div>
          </div>
        </div>
      )}

      {/* Positions Table */}
      <div className="glass-card rounded-2xl p-6 border-2 border-[#00F0FF]/30">
        <div className="flex justify-between items-center mb-4">
          <h3 className="text-xl font-bold text-white">📈 Your Positions</h3>
          {loading && <span className="text-gray-400 text-sm">Refreshing...</span>}
        </div>

        {error && (
          <div className="p-4 glass border-2 border-red-500/50 rounded-xl text-red-400 mb-4">
            {error}
          </div>
        )}

        {positions.length === 0 ? (
          <div className="text-center py-8 text-gray-400">
            <p>No active positions</p>
            <p className="text-sm mt-2">The bot will open positions when it finds good opportunities</p>
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-[#0066FF]/30">
                  <th className="text-left py-3 px-2 text-gray-400 font-semibold">Token</th>
                  <th className="text-right py-3 px-2 text-gray-400 font-semibold">Amount</th>
                  <th className="text-right py-3 px-2 text-gray-400 font-semibold">Entry</th>
                  <th className="text-right py-3 px-2 text-gray-400 font-semibold">Current</th>
                  <th className="text-right py-3 px-2 text-gray-400 font-semibold">P&L</th>
                  <th className="text-center py-3 px-2 text-gray-400 font-semibold">Status</th>
                  <th className="text-right py-3 px-2 text-gray-400 font-semibold">Opened</th>
                </tr>
              </thead>
              <tbody>
                {positions.map((position) => (
                  <tr key={position.position_id} className="border-b border-[#0066FF]/10 hover:bg-white/5 transition-colors">
                    <td className="py-4 px-2">
                      <div>
                        <p className="text-white font-semibold">{position.token_symbol}</p>
                        <p className="text-gray-400 text-xs font-mono">
                          {position.token_mint.slice(0, 4)}...{position.token_mint.slice(-4)}
                        </p>
                      </div>
                    </td>
                    <td className="text-right py-4 px-2 text-white">
                      {position.amount_sol.toFixed(2)} SOL
                    </td>
                    <td className="text-right py-4 px-2 text-gray-300">
                      ${(position.entry_price / 1_000_000).toFixed(6)}
                    </td>
                    <td className="text-right py-4 px-2 text-white">
                      ${(position.current_price / 1_000_000).toFixed(6)}
                    </td>
                    <td className="text-right py-4 px-2">
                      <div>
                        <p className={`font-bold ${getPnLColor(position.pnl)}`}>
                          {position.pnl >= 0 ? '+' : ''}{formatSOL(position.pnl)} SOL
                        </p>
                        <p className={`text-sm ${getPnLColor(position.pnl)}`}>
                          {position.pnl_percentage >= 0 ? '+' : ''}{position.pnl_percentage.toFixed(2)}%
                        </p>
                      </div>
                    </td>
                    <td className="text-center py-4 px-2">
                      <span className={`px-3 py-1 rounded-full text-xs font-bold ${getStatusColor(position.status)}`}>
                        {position.status.toUpperCase()}
                      </span>
                    </td>
                    <td className="text-right py-4 px-2 text-gray-400 text-sm">
                      {formatTimestamp(position.opened_at)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
