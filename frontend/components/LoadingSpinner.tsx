'use client';

import { useEffect, useRef } from 'react';
import { gsap } from 'gsap';

export default function LoadingSpinner() {
  const spinnerRef = useRef<HTMLDivElement>(null);
  const ringsRef = useRef<(HTMLDivElement | null)[]>([]);

  useEffect(() => {
    const ctx = gsap.context(() => {
      // Rotate each ring at different speeds
      ringsRef.current.forEach((ring, index) => {
        if (ring) {
          gsap.to(ring, {
            rotation: 360,
            duration: 2 - index * 0.3,
            repeat: -1,
            ease: 'none',
          });
        }
      });

      // Pulse the center
      gsap.to('.spinner-center', {
        scale: 1.2,
        opacity: 0.5,
        duration: 1,
        repeat: -1,
        yoyo: true,
        ease: 'power1.inOut',
      });
    }, spinnerRef);

    return () => ctx.revert();
  }, []);

  return (
    <div ref={spinnerRef} className="relative w-24 h-24">
      {/* Outer ring - Electric Blue */}
      <div
        ref={(el) => { ringsRef.current[0] = el; }}
        className="absolute inset-0 rounded-full border-4 border-transparent border-t-[#0066FF] border-r-[#0066FF]"
      />
      
      {/* Middle ring - Cyan */}
      <div
        ref={(el) => { ringsRef.current[1] = el; }}
        className="absolute inset-2 rounded-full border-4 border-transparent border-b-[#00F0FF] border-l-[#00F0FF]"
      />
      
      {/* Inner ring - Neon Green */}
      <div
        ref={(el) => { ringsRef.current[2] = el; }}
        className="absolute inset-4 rounded-full border-4 border-transparent border-t-[#39FF14] border-r-[#39FF14]"
      />

      {/* Center dot */}
      <div className="spinner-center absolute inset-0 m-auto w-4 h-4 rounded-full bg-gradient-to-r from-[#0066FF] to-[#39FF14]" />
    </div>
  );
}
