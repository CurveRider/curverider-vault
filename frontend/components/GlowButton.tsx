'use client';

import { useEffect, useRef } from 'react';
import { gsap } from 'gsap';

interface GlowButtonProps {
  children: React.ReactNode;
  href?: string;
  onClick?: () => void;
  variant?: 'primary' | 'secondary';
  className?: string;
  disabled?: boolean;
}

export default function GlowButton({
  children,
  href,
  onClick,
  variant = 'primary',
  className = '',
  disabled = false
}: GlowButtonProps) {
  const buttonRef = useRef<HTMLAnchorElement | HTMLButtonElement>(null);
  const glowRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const ctx = gsap.context(() => {
      // Continuous glow pulse
      gsap.to(glowRef.current, {
        scale: 1.2,
        opacity: 0.6,
        duration: 1.5,
        repeat: -1,
        yoyo: true,
        ease: 'power1.inOut',
      });
    }, buttonRef);

    return () => ctx.revert();
  }, []);

  const handleMouseMove = (e: React.MouseEvent) => {
    const rect = buttonRef.current?.getBoundingClientRect();
    if (!rect) return;

    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    gsap.to(buttonRef.current, {
      '--x': `${x}px`,
      '--y': `${y}px`,
      duration: 0.3,
    });
  };

  const baseClasses = `
    relative px-8 py-4 rounded-xl font-bold text-lg overflow-hidden
    transition-all duration-300 transform
    ${!disabled && 'hover:scale-105 cursor-pointer'}
    ${disabled && 'opacity-50 cursor-not-allowed'}
    ${className}
  `;

  const variantClasses = variant === 'primary'
    ? 'bg-gradient-to-r from-[#0066FF] to-[#00F0FF] text-white shadow-[0_0_30px_rgba(0,240,255,0.5)]'
    : 'glass border-2 border-[#00F0FF] text-white shadow-[0_0_20px_rgba(0,240,255,0.3)]';

  const commonProps = {
    ref: buttonRef as any,
    className: `${baseClasses} ${variantClasses}`,
    onMouseMove: handleMouseMove,
  };

  const content = (
    <>
      {/* Glow effect */}
      <div
        ref={glowRef}
        className="absolute inset-0 opacity-0 pointer-events-none"
        style={{
          background: variant === 'primary' 
            ? 'radial-gradient(circle at var(--x, 50%) var(--y, 50%), rgba(57, 255, 20, 0.3), transparent 70%)'
            : 'radial-gradient(circle at var(--x, 50%) var(--y, 50%), rgba(0, 240, 255, 0.3), transparent 70%)',
        }}
      />
      
      {/* Button content */}
      <span className="relative z-10 flex items-center justify-center gap-2">
        {children}
      </span>

      {/* Shine effect */}
      <div 
        className="absolute inset-0 opacity-0 hover:opacity-100 transition-opacity duration-500"
        style={{
          background: 'linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent)',
          transform: 'translateX(-100%)',
        }}
      />
    </>
  );

  if (href) {
    return (
      <a {...commonProps} href={href}>
        {content}
      </a>
    );
  }

  return (
    <button {...commonProps} onClick={onClick} disabled={disabled}>
      {content}
    </button>
  );
}
