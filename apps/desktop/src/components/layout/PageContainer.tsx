import { cn } from '@/lib/utils';
import { ReactNode } from 'react';
import { Bell, Moon, Sun } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useTheme } from '@/hooks/useTheme';
import { LanguageSwitcher } from '@/components/LanguageSwitcher';
import { useTranslation } from 'react-i18next';

interface PageContainerProps {
  children: ReactNode;
  className?: string;
  title?: ReactNode;
  actions?: ReactNode;
}

export function PageContainer({ children, className, title, actions }: PageContainerProps) {
  const { theme, setTheme } = useTheme();
  const { t } = useTranslation();

  return (
    <div className="flex flex-col h-full">
      {/* Header */}
      <div className="flex items-center justify-between px-6 py-4 shrink-0 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 z-10">
        <div className="flex items-center gap-4 min-w-0">
          {typeof title === 'string' ? (
            <h1 className="text-2xl font-bold tracking-tight truncate">{title}</h1>
          ) : (
            title
          )}
        </div>

        <div className="flex items-center gap-2 shrink-0">
          {/* Page Actions */}
          {actions}

          {/* Separator */}
          <div className="w-px h-6 bg-border mx-2" />

          {/* Global Actions */}
          <LanguageSwitcher />
          
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8 rounded-full hover:bg-muted"
            onClick={() => setTheme(theme === 'light' ? 'dark' : 'light')}
            title={t('settings.theme')}
          >
            {theme === 'light' ? <Moon className="h-4 w-4" /> : <Sun className="h-4 w-4" />}
            <span className="sr-only">Toggle theme</span>
          </Button>

          <Button variant="ghost" size="icon" className="h-8 w-8 rounded-full hover:bg-muted">
            <Bell className="h-4 w-4" />
            <span className="sr-only">Notifications</span>
          </Button>
        </div>
      </div>

      {/* Content */}
      <div className={cn("flex-1 overflow-auto p-6", className)}>
        {children}
      </div>
    </div>
  );
}
