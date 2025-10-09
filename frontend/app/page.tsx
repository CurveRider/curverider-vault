'use client';

import Link from "next/link";
import { useEffect, useRef } from 'react';
import { gsap } from 'gsap';
import Hero3D from '@/components/Hero3D';
import AnimatedCard from '@/components/AnimatedCard';
import GlowButton from '@/components/GlowButton';
import FloatingElements from '@/components/FloatingElements';
import AnimatedStat from '@/components/AnimatedStat';

export default function Home() {
  const titleRef = useRef<HTMLHeadingElement>(null);
  const subtitleRef = useRef<HTMLParagraphElement>(null);
  const navRef = useRef<HTMLElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      // Nav animation
      gsap.from(navRef.current, {
        y: -50,
        opacity: 0,
        duration: 1,
        ease: 'power3.out',
      });

      // Title animation with glitch effect
      gsap.from(titleRef.current, {
        opacity: 0,
        y: 50,
        duration: 1,
        delay: 0.3,
        ease: 'power3.out',
      });

      // Add continuous glow pulse to title
      gsap.to(titleRef.current, {
        textShadow: `
          0 0 20px rgba(0, 240, 255, 0.8),
          0 0 40px rgba(0, 240, 255, 0.6),
          0 0 60px rgba(57, 255, 20, 0.4)
        `,
        duration: 2,
        repeat: -1,
        yoyo: true,
        ease: 'power1.inOut',
      });

      // Subtitle animation
      gsap.from(subtitleRef.current, {
        opacity: 0,
        y: 30,
        duration: 1,
        delay: 0.6,
        ease: 'power3.out',
      });
    });

    return () => ctx.revert();
  }, []);

  return (
    <div className="min-h-screen animated-gradient relative overflow-hidden">
      {/* 3D Background Elements */}
      <Hero3D />
      <FloatingElements />

      <div className="container mx-auto px-4 py-16 relative z-10">
        <nav ref={navRef} className="flex justify-between items-center mb-20">
          <h1 className="text-3xl font-bold bg-gradient-to-r from-[#0066FF] via-[#00F0FF] to-[#39FF14] bg-clip-text text-transparent">
            ⚡ Curverider Vault
          </h1>
          <Link href="/dapp">
            <GlowButton variant="primary">
              Launch dApp →
            </GlowButton>
          </Link>
        </nav>

        <main className="flex flex-col items-center text-center space-y-8 max-w-6xl mx-auto">
          <h2 
            ref={titleRef}
            className="text-5xl md:text-7xl font-black text-white mb-6 leading-tight"
          >
            Autonomous DeFi Strategy Vault on{' '}
            <span className="bg-gradient-to-r from-[#39FF14] to-[#00F0FF] bg-clip-text text-transparent">
              Solana
            </span>
          </h2>
          
          <p 
            ref={subtitleRef}
            className="text-xl md:text-2xl text-gray-300 mb-8 max-w-3xl leading-relaxed"
          >
            Profit from <span className="text-[#39FF14] font-bold">pump.fun</span> meta with fully autonomous{' '}
            <span className="text-[#00F0FF] font-bold">token discovery</span>,{' '}
            <span className="text-[#0066FF] font-bold">sniping</span>, and{' '}
            <span className="text-[#39FF14] font-bold">trading</span>
          </p>

          {/* Feature Cards */}
          <div className="grid md:grid-cols-3 gap-8 mt-16 w-full">
            <AnimatedCard
              icon="🔍"
              title="Token Discovery"
              description="AI-powered scanning of pump.fun to identify high-potential tokens before they trend"
              delay={0.2}
            />
            <AnimatedCard
              icon="⚡"
              title="Smart Sniping"
              description="Lightning-fast execution to snipe tokens at optimal entry points on the bonding curve"
              delay={0.4}
            />
            <AnimatedCard
              icon="📈"
              title="Auto Trading"
              description="Automated profit-taking and stop-loss strategies when tokens graduate to DEX"
              delay={0.6}
            />
          </div>

          {/* Stats Section */}
          <div className="grid md:grid-cols-3 gap-6 mt-16 w-full">
            <AnimatedStat
              value="$2.4M+"
              label="Total Value Locked"
              gradient="from-[#39FF14] to-[#00F0FF]"
              delay={0.8}
            />
            <AnimatedStat
              value="156%"
              label="Average APY"
              gradient="from-[#0066FF] to-[#39FF14]"
              delay={1.0}
            />
            <AnimatedStat
              value="2,847"
              label="Active Users"
              gradient="from-[#00F0FF] to-[#0066FF]"
              delay={1.2}
            />
          </div>

          {/* CTA Buttons */}
          <div className="mt-16 flex gap-6 flex-col sm:flex-row">
            <Link href="/dapp">
              <GlowButton variant="primary">
                <span className="text-xl">Get Started</span>
                <span className="text-2xl">→</span>
              </GlowButton>
            </Link>
            <a
              href="https://github.com/lggg123/curverider-vault"
              target="_blank"
              rel="noopener noreferrer"
            >
              <GlowButton variant="secondary">
                <span className="text-xl">View on GitHub</span>
                <span className="text-2xl">↗</span>
              </GlowButton>
            </a>
          </div>

          {/* Trust indicators */}
          <div className="mt-20 flex flex-wrap justify-center gap-8 items-center opacity-60">
            <div className="text-gray-400 flex items-center gap-2">
              <span className="text-[#39FF14]">✓</span> Audited by CertiK
            </div>
            <div className="text-gray-400 flex items-center gap-2">
              <span className="text-[#39FF14]">✓</span> 24/7 Monitoring
            </div>
            <div className="text-gray-400 flex items-center gap-2">
              <span className="text-[#39FF14]">✓</span> Non-custodial
            </div>
          </div>
        </main>
      </div>

      {/* Bottom gradient fade */}
      <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-[#050b1f] to-transparent pointer-events-none" />
    </div>
  );
}
