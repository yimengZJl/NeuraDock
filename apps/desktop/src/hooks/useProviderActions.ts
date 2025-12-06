import { useMutation, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import { useTranslation } from 'react-i18next';

export interface RefreshModelsParams {
  providerId: string;
  useWaf?: boolean;
}

export function useProviderActions() {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  // Refresh provider models
  const refreshModelsMutation = useMutation({
    mutationFn: async ({ providerId, useWaf = true }: RefreshModelsParams) => {
      if (useWaf) {
        return await invoke('refresh_provider_models_with_waf', { providerId });
      } else {
        return await invoke('refresh_provider_models', { providerId });
      }
    },
    onSuccess: (_, variables) => {
      toast.success(t('providers.modelsRefreshed') || 'Models refreshed successfully');
      queryClient.invalidateQueries({ queryKey: ['providers'] });
      queryClient.invalidateQueries({ queryKey: ['provider-models', variables.providerId] });
    },
    onError: (error) => {
      console.error('Failed to refresh models:', error);
      toast.error(t('common.error'));
    },
  });

  // Sync provider (add to database)
  const syncProviderMutation = useMutation({
    mutationFn: async (providerId: string) => {
      await invoke('sync_provider', { providerId });
    },
    onSuccess: () => {
      toast.success(t('providers.syncSuccess') || 'Provider synced successfully');
      queryClient.invalidateQueries({ queryKey: ['providers'] });
    },
    onError: (error) => {
      console.error('Failed to sync provider:', error);
      toast.error(t('common.error'));
    },
  });

  // Delete provider
  const deleteProviderMutation = useMutation({
    mutationFn: async (providerId: string) => {
      await invoke('delete_provider', { providerId });
    },
    onSuccess: () => {
      toast.success(t('providers.deleteSuccess') || 'Provider deleted successfully');
      queryClient.invalidateQueries({ queryKey: ['providers'] });
    },
    onError: (error) => {
      console.error('Failed to delete provider:', error);
      toast.error(t('common.error'));
    },
  });

  return {
    // Actions
    refreshModels: refreshModelsMutation.mutate,
    refreshModelsAsync: refreshModelsMutation.mutateAsync,
    syncProvider: syncProviderMutation.mutate,
    deleteProvider: deleteProviderMutation.mutate,
    
    // Loading states
    isRefreshingModels: refreshModelsMutation.isPending,
    isSyncing: syncProviderMutation.isPending,
    isDeleting: deleteProviderMutation.isPending,
  };
}
