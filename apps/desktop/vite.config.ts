import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  base: './',  // 使用相对路径，确保 Tauri 生产构建正常
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      output: {
        manualChunks(id) {
          const normalizedId = id.split(path.sep).join('/');
          if (!normalizedId.includes('node_modules')) return;

          const reactPackages = [
            '/node_modules/react/',
            '/node_modules/react-dom/',
            '/node_modules/scheduler/',
            '/node_modules/use-sync-external-store/',
          ];

          if (reactPackages.some((pkg) => normalizedId.includes(pkg))) return 'react';
          if (normalizedId.includes('react-router')) return 'router';
          if (normalizedId.includes('@tanstack')) return 'tanstack';
          if (normalizedId.includes('@radix-ui')) return 'radix';
          if (normalizedId.includes('framer-motion')) return 'motion';
          if (normalizedId.includes('recharts') || normalizedId.includes('/d3-')) return 'charts';
          if (normalizedId.includes('lucide-react') || normalizedId.includes('@tabler/icons-react')) return 'icons';

          return 'vendor';
        },
      },
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
});
