import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { RefreshCw, Package } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { toast } from 'sonner';

interface ProviderModelsCardProps {
  providerId: string;
  providerName: string;
  accountId: string; // Used only to get cookies when fetching from API
}

export function ProviderModelsCard({ providerId, providerName, accountId }: ProviderModelsCardProps) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();

  // Fetch models - cache key is only provider_id (shared across all accounts of same provider)
  const { data: models = [], isLoading, isError, error } = useQuery<string[]>({
    queryKey: ['provider-models', providerId],
    queryFn: () => invoke<string[]>('fetch_provider_models', {
      providerId,
      accountId,
      forceRefresh: false,
    }),
    staleTime: 24 * 60 * 60 * 1000, // 24 hours
    gcTime: 24 * 60 * 60 * 1000,
    retry: 1,
  });

  // Force refresh mutation
  const refreshMutation = useMutation({
    mutationFn: () => invoke<string[]>('fetch_provider_models', {
      providerId,
      accountId,
      forceRefresh: true,
    }),
    onSuccess: (newModels) => {
      // Update cache for this provider (shared across all accounts)
      queryClient.setQueryData(['provider-models', providerId], newModels);
      toast.success(t('token.modelsRefreshed', 'Models refreshed successfully'));
    },
    onError: (err: Error) => {
      toast.error(t('token.modelsRefreshError', 'Failed to refresh models: ') + err.message);
    },
  });

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Package className="h-5 w-5 text-muted-foreground" />
            <CardTitle className="text-base">
              {t('token.providerModels', 'Provider Models')}
            </CardTitle>
            <Badge variant="outline" className="text-xs">
              {providerName}
            </Badge>
          </div>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => refreshMutation.mutate()}
            disabled={refreshMutation.isPending || isLoading}
          >
            <RefreshCw className={`h-4 w-4 ${refreshMutation.isPending ? 'animate-spin' : ''}`} />
          </Button>
        </div>
      </CardHeader>
      <CardContent>
        {isLoading && (
          <div className="flex items-center gap-2 text-sm text-muted-foreground">
            <div className="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent" />
            <span>{t('token.loadingModels', 'Loading models...')}</span>
          </div>
        )}

        {isError && (
          <div className="text-sm text-destructive">
            {t('token.modelsLoadError', 'Failed to load models')}: {error instanceof Error ? error.message : String(error)}
          </div>
        )}

        {!isLoading && !isError && models.length > 0 && (
          <div className="space-y-2">
            <div className="text-xs text-muted-foreground">
              {t('token.totalModels', 'Total')}: {models.length} {t('token.modelsCount', 'models')}
            </div>
            <div className="flex flex-wrap gap-1.5 max-h-32 overflow-y-auto">
              {models.map((model) => (
                <Badge key={model} variant="secondary" className="text-xs">
                  {model}
                </Badge>
              ))}
            </div>
          </div>
        )}

        {!isLoading && !isError && models.length === 0 && (
          <div className="text-sm text-muted-foreground">
            {t('token.noModels', 'No models found')}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
