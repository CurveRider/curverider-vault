'use client';

import { useEffect, useRef } from 'react';
import { gsap } from 'gsap';

export default function Hero3D() {
  const containerRef = useRef<HTMLDivElement>(null);
  const orb1Ref = useRef<HTMLDivElement>(null);
  const orb2Ref = useRef<HTMLDivElement>(null);
  const orb3Ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      // Floating orbs animation
      if (orb1Ref.current) {
        gsap.to(orb1Ref.current, {
          y: -30,
          x: 20,
          duration: 3,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
        });
      }

      if (orb2Ref.current) {
        gsap.to(orb2Ref.current, {
          y: 40,
          x: -30,
          duration: 4,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
        });
      }

      if (orb3Ref.current) {
        gsap.to(orb3Ref.current, {
          y: -20,
          x: 30,
          duration: 3.5,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
        });
      }
    }, containerRef);

    return () => ctx.revert();
  }, []);

  return (
    <div ref={containerRef} className="absolute inset-0 overflow-hidden pointer-events-none">
      {/* Electric Blue Orb */}
      <div
        ref={orb1Ref}
        className="absolute top-1/4 left-1/4 w-64 h-64 rounded-full"
        style={{
          background: 'radial-gradient(circle, rgba(0, 102, 255, 0.6) 0%, rgba(0, 102, 255, 0) 70%)',
          filter: 'blur(60px)',
        }}
      />

      {/* Neon Green Orb */}
      <div
        ref={orb2Ref}
        className="absolute top-1/2 right-1/4 w-80 h-80 rounded-full"
        style={{
          background: 'radial-gradient(circle, rgba(57, 255, 20, 0.4) 0%, rgba(57, 255, 20, 0) 70%)',
          filter: 'blur(80px)',
        }}
      />

      {/* Cyan Orb */}
      <div
        ref={orb3Ref}
        className="absolute bottom-1/4 left-1/2 w-72 h-72 rounded-full"
        style={{
          background: 'radial-gradient(circle, rgba(0, 240, 255, 0.5) 0%, rgba(0, 240, 255, 0) 70%)',
          filter: 'blur(70px)',
        }}
      />

      {/* Grid overlay */}
      <div 
        className="absolute inset-0 opacity-10"
        style={{
          backgroundImage: `
            linear-gradient(rgba(0, 240, 255, 0.3) 1px, transparent 1px),
            linear-gradient(90deg, rgba(0, 240, 255, 0.3) 1px, transparent 1px)
          `,
          backgroundSize: '50px 50px',
        }}
      />
    </div>
  );
}
