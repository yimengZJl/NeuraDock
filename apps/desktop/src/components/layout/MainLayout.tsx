import { ReactNode } from 'react';
import { Sidebar } from './Sidebar';

interface MainLayoutProps {
  children: ReactNode;
}

export function MainLayout({ children }: MainLayoutProps) {
  return (
    <div className="flex h-screen overflow-hidden bg-sidebar text-foreground">
      {/* Sidebar sits directly on the base background */}
      <Sidebar />
      
      {/* Main Content Area - Floating Canvas Style */}
      <main className="flex-1 min-w-0 p-2 pl-0 h-screen overflow-hidden relative">
        <div className="h-full w-full bg-background rounded-2xl shadow-sm border border-border/50 overflow-hidden flex flex-col relative">
          <div className="flex-1 overflow-hidden p-0 relative z-0">
            {children}
          </div>
        </div>
      </main>
    </div>
  );
}
