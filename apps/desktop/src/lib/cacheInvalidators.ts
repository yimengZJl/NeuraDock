import { QueryClient } from '@tanstack/react-query';

/**
 * Centralized cache invalidation utilities to ensure consistent
 * query cache updates across the application.
 */
export const cacheInvalidators = {
  /**
   * Invalidate caches for a specific account
   */
  invalidateAccount: (queryClient: QueryClient, accountId: string) => {
    queryClient.invalidateQueries({ queryKey: ['account', accountId] });
    queryClient.invalidateQueries({ queryKey: ['balance', accountId] });
    queryClient.invalidateQueries({ queryKey: ['tokens', accountId] });
  },

  /**
   * Invalidate all accounts list and related aggregate data
   */
  invalidateAllAccounts: (queryClient: QueryClient) => {
    queryClient.invalidateQueries({ queryKey: ['accounts'], exact: false });
    queryClient.refetchQueries({ queryKey: ['accounts'], type: 'active' });
    queryClient.invalidateQueries({ queryKey: ['balance-statistics'], exact: false });
    queryClient.refetchQueries({ queryKey: ['balance-statistics'], type: 'active' });
  },

  /**
   * Invalidate caches after check-in operation
   */
  invalidateAfterCheckIn: (queryClient: QueryClient, accountId: string) => {
    cacheInvalidators.invalidateAccount(queryClient, accountId);
    cacheInvalidators.invalidateAllAccounts(queryClient);
    queryClient.invalidateQueries({ queryKey: ['check-in-streak'] });
  },

  /**
   * Invalidate provider-related caches
   */
  invalidateProvider: (queryClient: QueryClient, providerId?: string) => {
    queryClient.invalidateQueries({ queryKey: ['providers'] });
    if (providerId) {
      queryClient.invalidateQueries({ queryKey: ['provider', providerId] });
    }
  },

  /**
   * Invalidate notification channels
   */
  invalidateNotificationChannels: (queryClient: QueryClient) => {
    queryClient.invalidateQueries({ queryKey: ['notification-channels'] });
  },
};
