'use client';

import { useEffect, useRef } from 'react';
import { gsap } from 'gsap';

interface AnimatedCardProps {
  icon: string;
  title: string;
  description: string;
  delay?: number;
  highlight?: string;
}

export default function AnimatedCard({ icon, title, description, delay = 0, highlight }: AnimatedCardProps) {
  const cardRef = useRef<HTMLDivElement>(null);
  const iconRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      // Initial state
      gsap.set(cardRef.current, {
        opacity: 0,
        y: 50,
        rotationX: -15,
      });

      // Entrance animation
      gsap.to(cardRef.current, {
        opacity: 1,
        y: 0,
        rotationX: 0,
        duration: 0.8,
        delay: delay,
        ease: 'power3.out',
      });

      // Icon pulse animation
      gsap.to(iconRef.current, {
        scale: 1.1,
        duration: 1.5,
        repeat: -1,
        yoyo: true,
        ease: 'power1.inOut',
      });
    }, cardRef);

    return () => ctx.revert();
  }, [delay]);

  const handleMouseEnter = () => {
    gsap.to(cardRef.current, {
      scale: 1.05,
      rotationY: 5,
      duration: 0.3,
      ease: 'power2.out',
    });
  };

  const handleMouseLeave = () => {
    gsap.to(cardRef.current, {
      scale: 1,
      rotationY: 0,
      duration: 0.3,
      ease: 'power2.out',
    });
  };

  const getRiskColor = (risk?: string) => {
    if (!risk) return '';
    if (risk.includes('Very High')) return 'border-[#FF0066] text-[#FF0066]';
    if (risk.includes('High')) return 'border-[#FFB800] text-[#FFB800]';
    if (risk.includes('Medium')) return 'border-[#39FF14] text-[#39FF14]';
    if (risk.includes('Low')) return 'border-[#00F0FF] text-[#00F0FF]';
    return 'border-white text-white';
  };

  return (
    <div
      ref={cardRef}
      className="glass-card rounded-2xl p-8 hover:border-[#39FF14] transition-all cursor-pointer relative"
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      style={{ transformStyle: 'preserve-3d' }}
    >
      {highlight && (
        <div className={`absolute top-4 right-4 px-3 py-1 rounded-full glass border text-xs font-bold ${getRiskColor(highlight)}`}>
          {highlight}
        </div>
      )}
      <div ref={iconRef} className="text-6xl mb-6 filter drop-shadow-[0_0_20px_rgba(57,255,20,0.6)]">
        {icon}
      </div>
      <h3 className="text-2xl font-bold mb-3 bg-gradient-to-r from-[#0066FF] to-[#00F0FF] bg-clip-text text-transparent">
        {title}
      </h3>
      <p className="text-gray-300 leading-relaxed">
        {description}
      </p>
    </div>
  );
}
