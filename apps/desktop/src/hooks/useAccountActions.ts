import { useState } from 'react';
import { toast } from 'sonner';
import { useTranslation } from 'react-i18next';
import { Account, AccountDetail, accountCommands } from '@/lib/tauri-commands';
import { useAccountMutation } from './useMutationFactory';

export function useAccountActions() {
  const { t } = useTranslation();
  const [editingAccount, setEditingAccount] = useState<AccountDetail | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  // Fetch account detail
  const fetchAccountDetail = async (accountId: string): Promise<AccountDetail> => {
    return await accountCommands.getDetail(accountId);
  };

  // Edit account
  const handleEdit = async (account: Account) => {
    try {
      const accountDetail = await fetchAccountDetail(account.id);
      setEditingAccount(accountDetail);
      setDialogOpen(true);
    } catch (error) {
      console.error('Failed to fetch account details:', error);
      toast.error(t('common.error'));
    }
  };

  // Create new account
  const handleCreate = () => {
    setEditingAccount(null);
    setDialogOpen(true);
  };

  // Close dialog
  const handleDialogClose = () => {
    setDialogOpen(false);
    setEditingAccount(null);
  };

  // Delete account mutation
  const deleteAccountMutation = useAccountMutation({
    mutationFn: async (accountId: string) => {
      await accountCommands.delete(accountId);
    },
    successMessage: 'accounts.deleteSuccess',
    logPrefix: 'Delete account',
  });

  // Toggle account enabled status
  const toggleAccountMutation = useAccountMutation({
    mutationFn: async ({ accountId, enabled }: { accountId: string; enabled: boolean }) => {
      await accountCommands.toggle(accountId, enabled);
    },
    logPrefix: 'Toggle account',
  });

  // Batch enable accounts
  const batchEnableMutation = useAccountMutation({
    mutationFn: async (accountIds: string[]) => {
      await Promise.all(
        accountIds.map((id) => accountCommands.toggle(id, true))
      );
    },
    successMessage: 'accounts.batchEnableSuccess',
    logPrefix: 'Batch enable accounts',
  });

  // Batch disable accounts
  const batchDisableMutation = useAccountMutation({
    mutationFn: async (accountIds: string[]) => {
      await Promise.all(
        accountIds.map((id) => accountCommands.toggle(id, false))
      );
    },
    successMessage: 'accounts.batchDisableSuccess',
    logPrefix: 'Batch disable accounts',
  });

  return {
    // State
    editingAccount,
    dialogOpen,

    // Actions
    handleEdit,
    handleCreate,
    handleDialogClose,
    deleteAccount: deleteAccountMutation.mutate,
    toggleAccount: toggleAccountMutation.mutate,
    batchEnable: batchEnableMutation.mutate,
    batchDisable: batchDisableMutation.mutate,

    // Loading states
    isDeleting: deleteAccountMutation.isPending,
    isToggling: toggleAccountMutation.isPending,
    isBatchEnabling: batchEnableMutation.isPending,
    isBatchDisabling: batchDisableMutation.isPending,
  };
}
