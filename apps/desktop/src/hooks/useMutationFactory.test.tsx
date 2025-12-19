import React from 'react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { renderHook, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useAccountMutation } from './useMutationFactory';
import { invoke } from '@tauri-apps/api/core';

// Mock dependencies
vi.mock('@tauri-apps/api/core');
vi.mock('sonner', () => ({
  toast: {
    success: vi.fn(),
    error: vi.fn(),
  },
}));

describe('useAccountMutation', () => {
  let queryClient: QueryClient;
  let wrapper: any;

  beforeEach(() => {
    queryClient = new QueryClient({
      defaultOptions: {
        queries: { retry: false },
        mutations: { retry: false },
      },
    });

    wrapper = ({ children }: { children: React.ReactNode }) => (
      <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
    );

    vi.clearAllMocks();
  });

  it('should execute mutation successfully', async () => {
    const mockInvoke = vi.mocked(invoke);
    mockInvoke.mockResolvedValueOnce(undefined);

    const { result } = renderHook(
      () =>
        useAccountMutation({
          mutationFn: async (id: string) => {
            await invoke('test_command', { id });
          },
        }),
      { wrapper }
    );

    result.current.mutate('test-id');

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(mockInvoke).toHaveBeenCalledWith('test_command', { id: 'test-id' });
  });

  it('should invalidate queries on success', async () => {
    const mockInvoke = vi.mocked(invoke);
    mockInvoke.mockResolvedValueOnce(undefined);

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries');

    const { result } = renderHook(
      () =>
        useAccountMutation({
          mutationFn: async () => {
            await invoke('test_command');
          },
          invalidateKeys: [['accounts'], ['providers']],
        }),
      { wrapper }
    );

    result.current.mutate(undefined);

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(invalidateSpy).toHaveBeenCalledTimes(2);
    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['accounts'] });
    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['providers'] });
  });

  it('should handle errors correctly', async () => {
    const mockInvoke = vi.mocked(invoke);
    const testError = new Error('Test error');
    mockInvoke.mockRejectedValueOnce(testError);

    const { result } = renderHook(
      () =>
        useAccountMutation({
          mutationFn: async () => {
            await invoke('failing_command');
          },
        }),
      { wrapper }
    );

    result.current.mutate(undefined);

    await waitFor(() => {
      expect(result.current.isError).toBe(true);
    });

    expect(result.current.error).toEqual(testError);
  });

  it('should use default invalidateKeys when not provided', async () => {
    const mockInvoke = vi.mocked(invoke);
    mockInvoke.mockResolvedValueOnce(undefined);

    const invalidateSpy = vi.spyOn(queryClient, 'invalidateQueries');

    const { result } = renderHook(
      () =>
        useAccountMutation({
          mutationFn: async () => {
            await invoke('test_command');
          },
          // No invalidateKeys provided, should use default ['accounts']
        }),
      { wrapper }
    );

    result.current.mutate(undefined);

    await waitFor(() => {
      expect(result.current.isSuccess).toBe(true);
    });

    expect(invalidateSpy).toHaveBeenCalledWith({ queryKey: ['accounts'] });
  });
});
