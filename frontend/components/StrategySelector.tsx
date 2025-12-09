'use client';

import { useState } from 'react';

export type StrategyType = 'conservative' | 'ultra_early_sniper' | 'momentum_scalper' | 'graduation_anticipator';

interface Strategy {
  id: StrategyType;
  name: string;
  emoji: string;
  description: string;
  risk: 'Low' | 'Medium' | 'High' | 'Very High';
  targetReturn: string;
  holdTime: string;
  winRate: string;
  color: string;
  gradient: string;
  features: string[];
  entryConditions: string[];
  exitParams: {
    takeProfit: string;
    stopLoss: string;
    timeout: string;
    trailingStop?: string;
  };
}

const strategies: Strategy[] = [
  {
    id: 'conservative',
    name: 'Conservative Multi-Factor',
    emoji: '⚖️',
    description: 'Balanced approach combining fundamental analysis with technical signals. Best for consistent wins.',
    risk: 'Medium',
    targetReturn: '2x',
    holdTime: '1 hour',
    winRate: '60-70%',
    color: '#0066FF',
    gradient: 'from-[#0066FF] to-[#00F0FF]',
    features: [
      'Multi-factor analysis',
      'Validated fundamentals',
      'Moderate returns',
      'Consistent performance',
    ],
    entryConditions: [
      'Bonding curve: 30-70%',
      'Liquidity: >5 SOL',
      'Holders: >50, <30% concentration',
      'Confidence: >80%',
    ],
    exitParams: {
      takeProfit: '2x (100% gain)',
      stopLoss: '50%',
      timeout: '1 hour',
    },
  },
  {
    id: 'ultra_early_sniper',
    name: 'Ultra-Early Sniper',
    emoji: '🎯',
    description: 'Catch tokens in the first 5 minutes before most scanners detect them. High risk, moonshot potential.',
    risk: 'Very High',
    targetReturn: '3-10x',
    holdTime: '10 minutes',
    winRate: '30-40%',
    color: '#FF0066',
    gradient: 'from-[#FF0066] to-[#FF3399]',
    features: [
      'Ultra-early entry (<5min)',
      'Moonshot hunting',
      '10-100x potential',
      'High turnover',
    ],
    entryConditions: [
      'Bonding curve: <10%',
      'Token age: <5 minutes',
      'Buy pressure: >5:1 ratio',
      'Confidence: >75%',
    ],
    exitParams: {
      takeProfit: '3x (200% gain)',
      stopLoss: '30%',
      timeout: '10 minutes',
    },
  },
  {
    id: 'momentum_scalper',
    name: 'Momentum Scalper',
    emoji: '⚡',
    description: 'Ride explosive momentum waves. Fast in, fast out on tokens showing parabolic price action.',
    risk: 'High',
    targetReturn: '1.5x',
    holdTime: '30 minutes',
    winRate: '50-60%',
    color: '#39FF14',
    gradient: 'from-[#39FF14] to-[#00FF00]',
    features: [
      'Quick flips',
      'Explosive momentum',
      'Trailing stop',
      'High frequency',
    ],
    entryConditions: [
      'Bonding curve: 40-80%',
      'Price change 1h: >50%',
      'Volume: >20 SOL',
      'Liquidity: >8 SOL',
    ],
    exitParams: {
      takeProfit: '1.5x (50% gain)',
      stopLoss: '25%',
      timeout: '30 minutes',
      trailingStop: 'Activate at +20%, trail by 10%',
    },
  },
  {
    id: 'graduation_anticipator',
    name: 'Graduation Anticipator',
    emoji: '🎓',
    description: 'Position before DEX graduation to capture the migration pump. Lower risk, consistent success.',
    risk: 'Low',
    targetReturn: '1.8x',
    holdTime: '2 hours',
    winRate: '70-80%',
    color: '#00F0FF',
    gradient: 'from-[#00F0FF] to-[#0080FF]',
    features: [
      'Pre-DEX positioning',
      'Graduation pumps',
      'Lower risk',
      'Patient approach',
    ],
    entryConditions: [
      'Bonding curve: 60-85%',
      'Liquidity: >15 SOL',
      'Holders: >100, <25% concentration',
      'Volume 24h: >50 SOL',
    ],
    exitParams: {
      takeProfit: '1.8x (80% gain)',
      stopLoss: '35%',
      timeout: '2 hours',
    },
  },
];

const riskColors = {
  'Low': 'text-[#00F0FF]',
  'Medium': 'text-[#39FF14]',
  'High': 'text-[#FFB800]',
  'Very High': 'text-[#FF0066]',
};

interface StrategySelectorProps {
  selectedStrategy: StrategyType;
  onStrategyChange: (strategy: StrategyType) => void;
}

export default function StrategySelector({ selectedStrategy, onStrategyChange }: StrategySelectorProps) {
  const [expandedStrategy, setExpandedStrategy] = useState<StrategyType | null>(null);

  const selectedStrategyData = strategies.find(s => s.id === selectedStrategy);

  return (
    <div className="space-y-6">
      {/* Current Strategy Display */}
      <div className="glass-card rounded-2xl p-8 border-2 border-[#00F0FF]/30">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-2xl font-bold bg-gradient-to-r from-[#0066FF] to-[#00F0FF] bg-clip-text text-transparent">
            🎲 Active Strategy
          </h2>
          <span className={`px-4 py-2 rounded-full glass border ${riskColors[selectedStrategyData?.risk || 'Medium']} border-current font-bold`}>
            {selectedStrategyData?.risk} Risk
          </span>
        </div>

        {selectedStrategyData && (
          <div className="grid md:grid-cols-2 gap-6">
            <div>
              <div className="flex items-center gap-3 mb-4">
                <span className="text-4xl">{selectedStrategyData.emoji}</span>
                <div>
                  <h3 className={`text-xl font-bold bg-gradient-to-r ${selectedStrategyData.gradient} bg-clip-text text-transparent`}>
                    {selectedStrategyData.name}
                  </h3>
                  <p className="text-gray-400 text-sm">{selectedStrategyData.description}</p>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-3">
              <div className="glass p-3 rounded-lg border border-[#0066FF]/20">
                <div className="text-gray-400 text-xs mb-1">Target Return</div>
                <div className="text-xl font-bold text-[#39FF14]">{selectedStrategyData.targetReturn}</div>
              </div>
              <div className="glass p-3 rounded-lg border border-[#00F0FF]/20">
                <div className="text-gray-400 text-xs mb-1">Hold Time</div>
                <div className="text-xl font-bold text-white">{selectedStrategyData.holdTime}</div>
              </div>
              <div className="glass p-3 rounded-lg border border-[#39FF14]/20">
                <div className="text-gray-400 text-xs mb-1">Win Rate</div>
                <div className="text-xl font-bold text-[#00F0FF]">{selectedStrategyData.winRate}</div>
              </div>
              <div className="glass p-3 rounded-lg border border-[#FF0066]/20">
                <div className="text-gray-400 text-xs mb-1">Take Profit</div>
                <div className="text-lg font-bold text-white">{selectedStrategyData.exitParams.takeProfit.split(' ')[0]}</div>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Strategy Selection Grid */}
      <div className="glass-card rounded-2xl p-8">
        <h3 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#39FF14] to-[#00F0FF] bg-clip-text text-transparent">
          Choose Your Strategy
        </h3>

        <div className="grid md:grid-cols-2 gap-4">
          {strategies.map((strategy) => (
            <div
              key={strategy.id}
              className={`glass rounded-xl p-6 cursor-pointer transition-all border-2 hover:scale-[1.02] active:scale-[0.98] ${
                selectedStrategy === strategy.id
                  ? `border-[${strategy.color}] shadow-[0_0_30px_${strategy.color}40]`
                  : 'border-white/10 hover:border-white/30'
              }`}
              onClick={() => {
                onStrategyChange(strategy.id);
                setExpandedStrategy(expandedStrategy === strategy.id ? null : strategy.id);
              }}
            >
              <div className="flex items-start justify-between mb-3">
                <div className="flex items-center gap-3">
                  <span className="text-3xl">{strategy.emoji}</span>
                  <div>
                    <h4 className={`font-bold bg-gradient-to-r ${strategy.gradient} bg-clip-text text-transparent`}>
                      {strategy.name}
                    </h4>
                    <span className={`text-xs ${riskColors[strategy.risk]}`}>{strategy.risk} Risk</span>
                  </div>
                </div>
                {selectedStrategy === strategy.id && (
                  <span className="text-[#39FF14] text-2xl">✓</span>
                )}
              </div>

              <p className="text-gray-400 text-sm mb-4">{strategy.description}</p>

              <div className="grid grid-cols-3 gap-2 text-center mb-4">
                <div className="glass p-2 rounded-lg">
                  <div className="text-xs text-gray-400">Return</div>
                  <div className="text-sm font-bold text-[#39FF14]">{strategy.targetReturn}</div>
                </div>
                <div className="glass p-2 rounded-lg">
                  <div className="text-xs text-gray-400">Time</div>
                  <div className="text-sm font-bold text-white">{strategy.holdTime}</div>
                </div>
                <div className="glass p-2 rounded-lg">
                  <div className="text-xs text-gray-400">Win Rate</div>
                  <div className="text-sm font-bold text-[#00F0FF]">{strategy.winRate}</div>
                </div>
              </div>

              {expandedStrategy === strategy.id && (
                <div className="border-t border-white/10 pt-4 mt-4 space-y-3 animate-fade-in">

                  <div>
                    <h5 className="text-xs font-bold text-gray-400 mb-2">KEY FEATURES</h5>
                    <div className="space-y-1">
                      {strategy.features.map((feature, idx) => (
                        <div key={idx} className="text-xs text-gray-300 flex items-center gap-2">
                          <span className="text-[#39FF14]">•</span>
                          {feature}
                        </div>
                      ))}
                    </div>
                  </div>

                  <div>
                    <h5 className="text-xs font-bold text-gray-400 mb-2">ENTRY CONDITIONS</h5>
                    <div className="space-y-1">
                      {strategy.entryConditions.map((condition, idx) => (
                        <div key={idx} className="text-xs text-gray-300 flex items-center gap-2">
                          <span className="text-[#0066FF]">▸</span>
                          {condition}
                        </div>
                      ))}
                    </div>
                  </div>

                  <div>
                    <h5 className="text-xs font-bold text-gray-400 mb-2">EXIT PARAMETERS</h5>
                    <div className="space-y-1">
                      <div className="text-xs text-gray-300 flex items-center gap-2">
                        <span className="text-[#39FF14]">✓</span>
                        TP: {strategy.exitParams.takeProfit}
                      </div>
                      <div className="text-xs text-gray-300 flex items-center gap-2">
                        <span className="text-[#FF0066]">✗</span>
                        SL: {strategy.exitParams.stopLoss}
                      </div>
                      <div className="text-xs text-gray-300 flex items-center gap-2">
                        <span className="text-[#00F0FF]">⏱</span>
                        Timeout: {strategy.exitParams.timeout}
                      </div>
                      {strategy.exitParams.trailingStop && (
                        <div className="text-xs text-gray-300 flex items-center gap-2">
                          <span className="text-[#FFB800]">📉</span>
                          {strategy.exitParams.trailingStop}
                        </div>
                      )}
                    </div>
                  </div>
                </div>
              )}

              <button
                className={`w-full mt-3 py-2 rounded-lg font-semibold text-sm transition-all ${
                  selectedStrategy === strategy.id
                    ? `bg-gradient-to-r ${strategy.gradient} text-black`
                    : 'glass border border-white/20 text-white hover:border-white/40'
                }`}
              >
                {selectedStrategy === strategy.id ? 'Active Strategy' : 'Select Strategy'}
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* Strategy Comparison */}
      <div className="glass-card rounded-2xl p-8">
        <h3 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#0066FF] to-[#39FF14] bg-clip-text text-transparent">
          📊 Strategy Comparison
        </h3>

        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-white/10">
                <th className="text-left py-3 text-gray-400 font-semibold">Strategy</th>
                <th className="text-center py-3 text-gray-400 font-semibold">Risk</th>
                <th className="text-center py-3 text-gray-400 font-semibold">Return</th>
                <th className="text-center py-3 text-gray-400 font-semibold">Time</th>
                <th className="text-center py-3 text-gray-400 font-semibold">Win Rate</th>
              </tr>
            </thead>
            <tbody>
              {strategies.map((strategy) => (
                <tr
                  key={strategy.id}
                  className={`border-b border-white/5 hover:bg-white/5 cursor-pointer transition-colors ${
                    selectedStrategy === strategy.id ? 'bg-white/10' : ''
                  }`}
                  onClick={() => onStrategyChange(strategy.id)}
                >
                  <td className="py-4">
                    <div className="flex items-center gap-2">
                      <span className="text-xl">{strategy.emoji}</span>
                      <span className={`font-semibold bg-gradient-to-r ${strategy.gradient} bg-clip-text text-transparent`}>
                        {strategy.name}
                      </span>
                    </div>
                  </td>
                  <td className="text-center">
                    <span className={`px-3 py-1 rounded-full glass text-xs font-bold ${riskColors[strategy.risk]}`}>
                      {strategy.risk}
                    </span>
                  </td>
                  <td className="text-center font-bold text-[#39FF14]">{strategy.targetReturn}</td>
                  <td className="text-center text-white">{strategy.holdTime}</td>
                  <td className="text-center font-bold text-[#00F0FF]">{strategy.winRate}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Portfolio Allocation Recommendation */}
      <div className="glass-card rounded-2xl p-8">
        <h3 className="text-2xl font-bold mb-6 bg-gradient-to-r from-[#39FF14] to-[#FF0066] bg-clip-text text-transparent">
          💼 Recommended Capital Allocation
        </h3>
        <p className="text-gray-300 mb-6">For a diversified meme coin portfolio with 10 SOL:</p>

        <div className="space-y-3">
          {[
            { name: 'Conservative Multi-Factor', allocation: '40%', amount: '4 SOL', color: '#0066FF', description: 'Core holdings' },
            { name: 'Graduation Anticipator', allocation: '30%', amount: '3 SOL', color: '#00F0FF', description: 'Stable income' },
            { name: 'Momentum Scalper', allocation: '20%', amount: '2 SOL', color: '#39FF14', description: 'Active trading' },
            { name: 'Ultra-Early Sniper', allocation: '10%', amount: '1 SOL', color: '#FF0066', description: 'Moonshot tickets' },
          ].map((item, idx) => (
            <div key={idx} className="glass p-4 rounded-xl border border-white/10">
              <div className="flex items-center justify-between mb-2">
                <span className="font-semibold text-white">{item.name}</span>
                <span className="text-[#39FF14] font-bold">{item.allocation}</span>
              </div>
              <div className="flex items-center justify-between text-sm">
                <span className="text-gray-400">{item.description}</span>
                <span className="text-white font-mono">{item.amount}</span>
              </div>
              <div className="mt-2 h-2 glass rounded-full overflow-hidden">
                <div
                  className="h-full rounded-full"
                  style={{
                    width: item.allocation,
                    background: `linear-gradient(90deg, ${item.color}80, ${item.color})`
                  }}
                />
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
