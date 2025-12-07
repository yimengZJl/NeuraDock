import { Link, useLocation } from 'react-router-dom';
import {
  Home,
  UserCircle,
  Flame,
  Key,
  Settings2
} from 'lucide-react';
import { cn } from '@/lib/utils';
import { useTranslation } from 'react-i18next';
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/components/ui/tooltip';

export function Sidebar() {
  const location = useLocation();
  const { t } = useTranslation();
  
  // Keep width fixed for icon-only sidebar for now
  const navigation = [
    { name: t('nav.dashboard'), href: '/', icon: Home },
    { name: t('nav.accounts'), href: '/accounts', icon: UserCircle },
    { name: t('nav.streaks'), href: '/streaks', icon: Flame },
    { name: t('nav.tokens'), href: '/tokens', icon: Key },
    { name: t('nav.settings'), href: '/settings', icon: Settings2 },
  ];

  return (
    <aside 
      className="flex flex-col h-full w-[72px] items-center pt-10 pb-4 transition-all duration-300 select-none bg-sidebar/50 backdrop-blur-sm"
      data-tauri-drag-region
    >
      {/* App Logo or Drag Region */}
      <div className="h-4 w-full mb-6" data-tauri-drag-region />

      {/* Navigation */}
      <nav className="flex-1 w-full px-3 space-y-3 flex flex-col items-center">
        {navigation.map((item) => {
          const isActive = location.pathname === item.href;
          const Icon = item.icon;

          return (
            <Tooltip key={item.name}>
              <TooltipTrigger asChild>
                <Link key={item.name} to={item.href} className="w-full flex justify-center">
                  <div
                    className={cn(
                      'relative group flex items-center justify-center w-10 h-10 rounded-xl transition-all duration-200 ease-out',
                      isActive
                        ? 'bg-white text-black shadow-md scale-105 dark:bg-primary dark:text-primary-foreground'
                        : 'text-muted-foreground hover:bg-black/5 dark:hover:bg-white/10 hover:text-foreground'
                    )}
                  >
                    <Icon className={cn("h-5 w-5", isActive && "stroke-[2.5px]")} />
                  </div>
                </Link>
              </TooltipTrigger>
              <TooltipContent side="right" className="ml-2 font-medium bg-popover text-popover-foreground border-border/50 shadow-macos">
                <p>{item.name}</p>
              </TooltipContent>
            </Tooltip>
          );
        })}
      </nav>
      
      {/* User or Bottom Actions can go here */}
    </aside>
  );
}
