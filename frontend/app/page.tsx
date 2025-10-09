import Link from "next/link";

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-purple-900 via-blue-900 to-black">
      <div className="container mx-auto px-4 py-16">
        <nav className="flex justify-between items-center mb-20">
          <h1 className="text-3xl font-bold text-white">Curverider Vault</h1>
          <Link
            href="/dapp"
            className="px-6 py-3 bg-gradient-to-r from-purple-500 to-blue-500 text-white rounded-lg hover:from-purple-600 hover:to-blue-600 transition-all"
          >
            Launch dApp
          </Link>
        </nav>

        <main className="flex flex-col items-center text-center space-y-8 max-w-4xl mx-auto">
          <h2 className="text-5xl md:text-6xl font-bold text-white mb-6">
            Autonomous DeFi Strategy Vault on Solana
          </h2>
          <p className="text-xl md:text-2xl text-gray-300 mb-8">
            Profit from pump.fun meta with fully autonomous token discovery, sniping, and trading
          </p>

          <div className="grid md:grid-cols-3 gap-8 mt-12 w-full">
            <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 hover:bg-white/20 transition-all">
              <div className="text-4xl mb-4">🔍</div>
              <h3 className="text-xl font-semibold text-white mb-2">Token Discovery</h3>
              <p className="text-gray-300">
                Automatically discover trending tokens on pump.fun
              </p>
            </div>

            <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 hover:bg-white/20 transition-all">
              <div className="text-4xl mb-4">⚡</div>
              <h3 className="text-xl font-semibold text-white mb-2">Smart Sniping</h3>
              <p className="text-gray-300">
                Snipe tokens early on the bonding curve
              </p>
            </div>

            <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 hover:bg-white/20 transition-all">
              <div className="text-4xl mb-4">📈</div>
              <h3 className="text-xl font-semibold text-white mb-2">Auto Trading</h3>
              <p className="text-gray-300">
                Automated trading when tokens hit DEX
              </p>
            </div>
          </div>

          <div className="mt-16 flex gap-4 flex-col sm:flex-row">
            <Link
              href="/dapp"
              className="px-8 py-4 bg-gradient-to-r from-purple-500 to-blue-500 text-white text-lg rounded-lg hover:from-purple-600 hover:to-blue-600 transition-all font-semibold"
            >
              Get Started
            </Link>
            <a
              href="https://github.com/lggg123/curverider-vault"
              target="_blank"
              rel="noopener noreferrer"
              className="px-8 py-4 bg-white/10 backdrop-blur-lg text-white text-lg rounded-lg hover:bg-white/20 transition-all font-semibold"
            >
              View on GitHub
            </a>
          </div>
        </main>
      </div>
    </div>
  );
}
