import { useDeleteAccount, useToggleAccount } from '@/hooks/useAccounts';
import { useRefreshAccountBalance } from '@/hooks/useBalance';
import { Account } from '@/lib/tauri-commands';

/**
 * Hook to encapsulate common account actions.
 * Provides simplified handlers that directly use the mutation hooks.
 */
export function useAccountOperations(account: Account) {
  const deleteMutation = useDeleteAccount();
  const toggleMutation = useToggleAccount();
  const refreshBalanceMutation = useRefreshAccountBalance();

  const handleToggle = () => {
    toggleMutation.mutate({
      accountId: account.id,
      enabled: !account.enabled,
    });
  };

  const handleRefreshBalance = () => {
    refreshBalanceMutation.mutate(account.id);
  };

  const handleDelete = () => {
    deleteMutation.mutate(account.id);
  };

  return {
    handleToggle,
    handleRefreshBalance,
    handleDelete,
    isDeleting: deleteMutation.isPending,
    isToggling: toggleMutation.isPending,
    isRefreshingBalance: refreshBalanceMutation.isPending,
  };
}
