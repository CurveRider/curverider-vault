'use client';

import { useEffect, useRef, useState } from 'react';
import { gsap } from 'gsap';

interface AnimatedStatProps {
  value: string;
  label: string;
  gradient: string;
  delay?: number;
}

export default function AnimatedStat({ value, label, gradient, delay = 0 }: AnimatedStatProps) {
  const statRef = useRef<HTMLDivElement>(null);
  const valueRef = useRef<HTMLDivElement>(null);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting) {
          setIsVisible(true);
        }
      },
      { threshold: 0.1 }
    );

    if (statRef.current) {
      observer.observe(statRef.current);
    }

    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    if (isVisible) {
      const ctx = gsap.context(() => {
        // Entrance animation
        gsap.from(statRef.current, {
          opacity: 0,
          scale: 0.8,
          y: 30,
          duration: 0.8,
          delay: delay,
          ease: 'back.out(1.7)',
        });

        // Value pop-in
        gsap.from(valueRef.current, {
          scale: 0,
          duration: 0.6,
          delay: delay + 0.2,
          ease: 'elastic.out(1, 0.5)',
        });

        // Continuous glow pulse
        gsap.to(valueRef.current, {
          textShadow: `
            0 0 20px rgba(0, 240, 255, 0.8),
            0 0 40px rgba(0, 240, 255, 0.4),
            0 0 60px rgba(57, 255, 20, 0.2)
          `,
          duration: 2,
          repeat: -1,
          yoyo: true,
          ease: 'power1.inOut',
        });
      }, statRef);

      return () => ctx.revert();
    }
  }, [isVisible, delay]);

  return (
    <div ref={statRef} className="glass-card rounded-xl p-6 text-center">
      <div
        ref={valueRef}
        className={`text-4xl font-black bg-gradient-to-r ${gradient} bg-clip-text text-transparent mb-2`}
      >
        {value}
      </div>
      <p className="text-gray-400">{label}</p>
    </div>
  );
}
