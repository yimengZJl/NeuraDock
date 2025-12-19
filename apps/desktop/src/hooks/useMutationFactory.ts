import { useMutation, useQueryClient, UseMutationOptions } from '@tanstack/react-query';
import { toast } from 'sonner';
import { useTranslation } from 'react-i18next';

interface MutationFactoryOptions<TData = unknown, TVariables = unknown> {
  /**
   * The mutation function to execute
   */
  mutationFn: (variables: TVariables) => Promise<TData>;

  /**
   * Query keys to invalidate on success
   * @default ['accounts']
   */
  invalidateKeys?: string[][];

  /**
   * Success toast message (i18n key or string)
   */
  successMessage?: string;

  /**
   * Error toast message (i18n key or string)
   * @default 'common.error'
   */
  errorMessage?: string;

  /**
   * Log prefix for console errors
   */
  logPrefix?: string;

  /**
   * Additional mutation options to merge
   */
  options?: Omit<UseMutationOptions<TData, Error, TVariables>, 'mutationFn' | 'onSuccess' | 'onError'>;
}

/**
 * Factory hook for creating standardized mutations with automatic
 * query invalidation, toast notifications, and error logging
 *
 * @example
 * ```ts
 * const deleteAccountMutation = useAccountMutation({
 *   mutationFn: async (accountId: string) => {
 *     await invoke('delete_account', { accountId });
 *   },
 *   successMessage: 'accounts.deleteSuccess',
 *   logPrefix: 'Delete account',
 * });
 * ```
 */
export function useAccountMutation<TData = unknown, TVariables = unknown>(
  config: MutationFactoryOptions<TData, TVariables>
) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  const {
    mutationFn,
    invalidateKeys = [['accounts']],
    successMessage,
    errorMessage = 'common.error',
    logPrefix,
    options = {},
  } = config;

  return useMutation<TData, Error, TVariables>({
    mutationFn,
    onSuccess: (data, variables, context) => {
      // Invalidate specified query keys
      invalidateKeys.forEach(queryKey => {
        queryClient.invalidateQueries({ queryKey });
      });

      // Show success toast if message provided
      if (successMessage) {
        const message = t(successMessage) || successMessage;
        toast.success(message);
      }

      // Call custom onSuccess if provided
      options.onSuccess?.(data, variables, context);
    },
    onError: (error, variables, context) => {
      // Log error to console
      const prefix = logPrefix ? `${logPrefix}:` : 'Mutation error:';
      console.error(prefix, error);

      // Show error toast
      const message = t(errorMessage) || errorMessage;
      toast.error(message);

      // Call custom onError if provided
      options.onError?.(error, variables, context);
    },
    ...options,
  });
}
