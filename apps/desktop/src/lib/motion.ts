import type { Variants } from 'framer-motion';

interface StaggerContainerOptions {
  staggerChildren?: number;
  delayChildren?: number;
}

export function createStaggerContainer({
  staggerChildren = 0.05,
  delayChildren = 0,
}: StaggerContainerOptions = {}): Variants {
  return {
    hidden: { opacity: 0 },
    show: {
      opacity: 1,
      transition: {
        staggerChildren,
        delayChildren,
      },
    },
  };
}

interface FadeUpItemOptions {
  y?: number;
  scale?: number;
  stiffness?: number;
  damping?: number;
}

export function createFadeUpItem({
  y = 10,
  scale = 0.98,
  stiffness = 260,
  damping = 20,
}: FadeUpItemOptions = {}): Variants {
  return {
    hidden: { opacity: 0, y, scale },
    show: {
      opacity: 1,
      y: 0,
      scale: 1,
      transition: {
        type: 'spring',
        stiffness,
        damping,
      },
    },
  };
}

