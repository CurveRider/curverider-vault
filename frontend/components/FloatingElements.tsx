'use client';

import { useEffect, useRef, useState } from 'react';
import { gsap } from 'gsap';

// Generate consistent particle positions (same on server and client)
const particlePositions = Array.from({ length: 20 }, (_, i) => ({
  left: ((i * 37 + 23) % 100), // Deterministic pseudo-random
  top: ((i * 47 + 17) % 100),
}));

export default function FloatingElements() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
    
    const ctx = gsap.context(() => {
      // Animate all floating elements
      const elements = containerRef.current?.querySelectorAll('.float-element');
      
      elements?.forEach((element, index) => {
        gsap.to(element, {
          y: `random(-30, 30)`,
          x: `random(-20, 20)`,
          rotation: `random(-15, 15)`,
          duration: `random(2, 4)`,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
          delay: index * 0.2,
        });

        // Add glow pulse
        gsap.to(element, {
          opacity: 0.3,
          duration: 2,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
        });
      });
    }, containerRef);

    return () => ctx.revert();
  }, []);

  return (
    <div ref={containerRef} className="absolute inset-0 pointer-events-none overflow-hidden">
      {/* Geometric shapes floating around */}
      <div 
        className="float-element absolute top-[10%] left-[5%] w-20 h-20 border-2 border-[#0066FF] rounded-lg opacity-20"
        style={{ transform: 'rotate(45deg)' }}
      />
      <div 
        className="float-element absolute top-[30%] right-[10%] w-16 h-16 border-2 border-[#39FF14] rounded-full opacity-20"
      />
      <div 
        className="float-element absolute bottom-[20%] left-[15%] w-24 h-24 border-2 border-[#00F0FF] opacity-20"
        style={{ clipPath: 'polygon(50% 0%, 0% 100%, 100% 100%)' }}
      />
      <div 
        className="float-element absolute top-[50%] right-[20%] w-20 h-20 border-2 border-[#0066FF] rounded-full opacity-20"
      />
      <div 
        className="float-element absolute bottom-[40%] right-[5%] w-16 h-16 border-2 border-[#39FF14] rounded-lg opacity-20"
        style={{ transform: 'rotate(30deg)' }}
      />
      <div 
        className="float-element absolute top-[70%] left-[25%] w-12 h-12 border-2 border-[#00F0FF] rounded-lg opacity-20"
      />

      {/* Small particles */}
      {mounted && particlePositions.map((pos, i) => (
        <div
          key={i}
          className="float-element absolute w-1 h-1 bg-[#00F0FF] rounded-full opacity-40"
          style={{
            left: `${pos.left}%`,
            top: `${pos.top}%`,
          }}
        />
      ))}
    </div>
  );
}
