// Centralized Query Keys for TanStack Query

export const accountKeys = {
  all: ['accounts'] as const,
  lists: () => [...accountKeys.all, 'list'] as const,
  list: (enabledOnly: boolean) => [...accountKeys.lists(), { enabledOnly }] as const,
  details: () => [...accountKeys.all, 'detail'] as const,
  detail: (id: string) => [...accountKeys.details(), id] as const,
  stats: () => [...accountKeys.all, 'stats'] as const,
  balance: (accountId: string) => [...accountKeys.detail(accountId), 'balance'] as const,
  tokens: (accountId: string) => [...accountKeys.detail(accountId), 'tokens'] as const,
  balanceStatistics: () => ['balance-statistics'] as const,
};

export const providerKeys = {
  all: ['providers'] as const,
  lists: () => [...providerKeys.all, 'list'] as const,
  list: () => [...providerKeys.lists()] as const,
  detail: (id: string) => [...providerKeys.all, 'detail', id] as const,
};

export const checkInKeys = {
  all: ['check-in'] as const,
  streak: (accountId: string) => [...checkInKeys.all, 'streak', accountId] as const,
  streaks: () => [...checkInKeys.all, 'streaks'] as const,
  calendar: (accountId: string, year: number, month: number) => 
    [...checkInKeys.all, 'calendar', accountId, { year, month }] as const,
  trend: (accountId: string, days: number) => 
    [...checkInKeys.all, 'trend', accountId, { days }] as const,
};

export const notificationKeys = {
  all: ['notifications'] as const,
  channels: () => [...notificationKeys.all, 'channels'] as const,
};
