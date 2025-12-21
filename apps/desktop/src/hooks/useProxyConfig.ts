import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import { useTranslation } from 'react-i18next';

export interface ProxyConfig {
  enabled: boolean;
  proxy_type: string; // "http" or "socks5"
  host: string;
  port: number;
}

export function useProxyConfig() {
  const { t } = useTranslation();
  const [config, setConfig] = useState<ProxyConfig>({
    enabled: false,
    proxy_type: 'http',
    host: '',
    port: 0,
  });
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  // Load proxy config from backend
  const loadConfig = async () => {
    try {
      setIsLoading(true);
      const data = await invoke<ProxyConfig>('get_proxy_config');
      setConfig(data);
    } catch (error) {
      console.error('Failed to load proxy config:', error);
      toast.error(t('settings.proxyLoadFailed'));
    } finally {
      setIsLoading(false);
    }
  };

  // Save proxy config to backend
  const saveConfig = async (newConfig: ProxyConfig) => {
    try {
      setIsSaving(true);
      const result = await invoke<ProxyConfig>('update_proxy_config', {
        input: newConfig,
      });
      setConfig(result);
      toast.success(t('settings.proxySaveSuccess'));
      return true;
    } catch (error) {
      console.error('Failed to save proxy config:', error);
      // Better error message handling
      const errorMessage = error instanceof Error
        ? error.message
        : typeof error === 'string'
          ? error
          : JSON.stringify(error);
      toast.error(t('settings.proxySaveFailed', { message: errorMessage }));
      return false;
    } finally {
      setIsSaving(false);
    }
  };

  // Update a single field
  const updateField = <K extends keyof ProxyConfig>(
    field: K,
    value: ProxyConfig[K]
  ) => {
    setConfig((prev) => ({ ...prev, [field]: value }));
  };

  // Initial load
  useEffect(() => {
    loadConfig();
  }, []);

  return {
    config,
    isLoading,
    isSaving,
    updateField,
    saveConfig,
    reload: loadConfig,
  };
}
