import { useEffect } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { QueryClientProvider } from '@tanstack/react-query';
import { queryClient } from './lib/query-client';
import { ThemeProvider } from './hooks/useTheme';
import { MainLayout } from './components/layout/MainLayout';
import { Toaster } from './components/ui/toaster';
import { HomePage } from './pages/HomePage';
import { AccountsPage } from './pages/AccountsPage';
import { AccountOverviewPage } from './pages/AccountOverviewPage';
import { AccountActivityPage } from './pages/AccountActivityPage';
import { TokensPage } from './pages/TokensPage';
import { ProvidersPage } from './pages/ProvidersPage';
import { PreferencesPage } from './pages/PreferencesPage';
import { TooltipProvider } from '@/components/ui/tooltip';
import { getCurrentWindow } from '@tauri-apps/api/window';

function App() {
  useEffect(() => {
    const setupWindowSizePersistence = async () => {
      const window = getCurrentWindow();

      // Restore saved window size on startup
      try {
        const savedSize = localStorage.getItem('neuradock-window-size');
        if (savedSize) {
          const { width, height } = JSON.parse(savedSize);
          await window.setSize({ width, height });
        }
      } catch (error) {
        console.error('Failed to restore window size:', error);
      }

      // Listen for window resize events and save size
      let resizeTimeout: number | undefined;
      const unlisten = await window.onResized(async ({ payload: size }) => {
        // Debounce: only save after user stops resizing for 500ms
        if (resizeTimeout) clearTimeout(resizeTimeout);
        resizeTimeout = window.setTimeout(() => {
          localStorage.setItem('neuradock-window-size', JSON.stringify(size));
        }, 500);
      });

      // Cleanup
      return () => {
        unlisten();
        if (resizeTimeout) clearTimeout(resizeTimeout);
      };
    };

    setupWindowSizePersistence();
  }, []);

  return (
    <QueryClientProvider client={queryClient}>
      <ThemeProvider>
        <TooltipProvider delayDuration={0}>
          <BrowserRouter>
            <MainLayout>
              <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/accounts" element={<AccountsPage />} />
                <Route path="/accounts/:accountId" element={<AccountOverviewPage />} />
                <Route path="/account/:accountId/records" element={<AccountActivityPage />} />
                <Route path="/tokens" element={<TokensPage />} />
                <Route path="/providers" element={<ProvidersPage />} />
                <Route path="/settings" element={<PreferencesPage />} />
              </Routes>
            </MainLayout>
            <Toaster />
          </BrowserRouter>
        </TooltipProvider>
      </ThemeProvider>
    </QueryClientProvider>
  );
}

export default App;
