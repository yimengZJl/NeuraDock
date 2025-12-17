import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import { ProviderDto } from './useProviders';

export function useProviderActions() {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const [dialogOpen, setDialogOpen] = useState(false);
  const [editingProvider, setEditingProvider] = useState<ProviderDto | null>(null);

  const createMutation = useMutation({
    mutationFn: async (input: {
      name: string;
      domain: string;
      needs_waf_bypass: boolean;
      supports_check_in?: boolean;
      check_in_bugged?: boolean;
      login_path?: string;
      sign_in_path?: string;
      user_info_path?: string;
      token_api_path?: string;
      models_path?: string;
      api_user_key?: string;
    }) => {
      return await invoke<string>('create_provider', { input: input });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['providers'] });
      toast.success(t('providers.toast.createSuccess'));
    },
    onError: (error: any) => {
      console.error('Failed to create provider:', error);
      toast.error(t('providers.toast.createError', {
        error: error.message || t('providers.toast.unknownError')
      }));
    },
  });

  const updateMutation = useMutation({
    mutationFn: async (input: {
      provider_id: string;
      name?: string;
      domain?: string;
      needs_waf_bypass?: boolean;
      supports_check_in?: boolean;
      check_in_bugged?: boolean;
      login_path?: string;
      sign_in_path?: string;
      user_info_path?: string;
      token_api_path?: string;
      models_path?: string;
      api_user_key?: string;
    }) => {
      return await invoke<boolean>('update_provider', { input: input });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['providers'] });
      toast.success(t('providers.toast.updateSuccess'));
    },
    onError: (error: any) => {
      console.error('Failed to update provider:', error);
      toast.error(t('providers.toast.updateError', {
        error: error.message || t('providers.toast.unknownError')
      }));
    },
  });

  const deleteMutation = useMutation({
    mutationFn: async (providerId: string) => {
      return await invoke<boolean>('delete_provider', {
        input: { provider_id: providerId }
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['providers'] });
      toast.success(t('providers.toast.deleteSuccess'));
    },
    onError: (error: any) => {
      console.error('Failed to delete provider:', error);
      toast.error(t('providers.toast.deleteError', {
        error: error.message || t('providers.toast.unknownError')
      }));
    },
  });

  const handleCreate = () => {
    setEditingProvider(null);
    setDialogOpen(true);
  };

  const handleEdit = (provider: ProviderDto) => {
    setEditingProvider(provider);
    setDialogOpen(true);
  };

  const handleDialogClose = () => {
    setDialogOpen(false);
    setEditingProvider(null);
  };

  const handleDelete = async (providerId: string) => {
    await deleteMutation.mutateAsync(providerId);
  };

  return {
    dialogOpen,
    editingProvider,
    handleCreate,
    handleEdit,
    handleDialogClose,
    handleDelete,
    createMutation,
    updateMutation,
    deleteMutation,
  };
}
