import { useQuery } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';

export interface BalanceDto {
  current_balance: number;
  total_consumed: number;
  total_income: number;
}

export interface ProviderBalanceDto {
  provider_id: string;
  provider_name: string;
  current_balance: number;
  total_consumed: number;
  total_income: number;
  account_count: number;
}

export interface BalanceStatisticsDto {
  providers: ProviderBalanceDto[];
  total_current_balance: number;
  total_consumed: number;
  total_income: number;
}

// Query: Fetch account balance
export function useFetchAccountBalance(accountId: string, enabled = true) {
  return useQuery({
    queryKey: ['balance', accountId],
    queryFn: () => invoke<BalanceDto>('fetch_account_balance', { accountId }),
    enabled: enabled && !!accountId,
  });
}

// Query: Fetch multiple account balances
export function useFetchAccountsBalances(accountIds: string[]) {
  return useQuery({
    queryKey: ['balances', accountIds],
    queryFn: () => invoke<Record<string, BalanceDto | null>>('fetch_accounts_balances', { accountIds }),
    enabled: accountIds.length > 0,
  });
}

// Query: Get balance statistics
export function useBalanceStatistics() {
  return useQuery({
    queryKey: ['balance-statistics'],
    queryFn: () => invoke<BalanceStatisticsDto>('get_balance_statistics'),
  });
}
