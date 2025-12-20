import type { ReactNode } from 'react';

import { cn } from '@/lib/utils';

interface HeaderActionsProps {
  children: ReactNode;
  className?: string;
}

export function HeaderActions({ children, className }: HeaderActionsProps) {
  return <div className={cn('flex items-center gap-3', className)}>{children}</div>;
}

interface HeaderActionsSeparatorProps {
  className?: string;
}

export function HeaderActionsSeparator({ className }: HeaderActionsSeparatorProps) {
  return <div className={cn('w-px h-6 bg-border shrink-0', className)} />;
}

