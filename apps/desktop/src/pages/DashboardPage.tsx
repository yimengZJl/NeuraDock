import { Users, DollarSign, TrendingUp } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useAccounts } from '@/hooks/useAccounts';
import { useBalanceStatistics } from '@/hooks/useBalance';
import { ProviderModelsSection } from '@/components/account/ProviderModelsSection';
import { useTranslation } from 'react-i18next';

export function DashboardPage() {
  const { data: accounts, isLoading } = useAccounts();
  const { data: statistics, isLoading: statsLoading } = useBalanceStatistics();
  const { t } = useTranslation();

  const enabledAccounts = accounts?.filter(a => a.enabled).length || 0;
  const totalAccounts = accounts?.length || 0;

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{t('dashboard.title')}</h1>
        <p className="text-muted-foreground">
          {t('dashboard.description')}
        </p>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">{t('dashboard.stats.totalIncome')}</CardTitle>
            <DollarSign className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-blue-600">
              {statsLoading ? '...' : statistics ? `$${statistics.total_income.toFixed(2)}` : '$0.00'}
            </div>
            <p className="text-xs text-muted-foreground">
              {t('dashboard.stats.acrossAllProviders')}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">{t('dashboard.stats.historicalConsumption')}</CardTitle>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-orange-600">
              {statsLoading ? '...' : statistics ? `$${statistics.total_consumed.toFixed(2)}` : '$0.00'}
            </div>
            <p className="text-xs text-muted-foreground">
              {t('dashboard.stats.totalConsumption')}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">{t('dashboard.stats.currentBalance')}</CardTitle>
            <DollarSign className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold text-green-600">
              {statsLoading ? '...' : statistics ? `$${statistics.total_current_balance.toFixed(2)}` : '$0.00'}
            </div>
            <p className="text-xs text-muted-foreground">
              {t('dashboard.stats.availableBalance')}
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">{t('dashboard.stats.totalAccounts')}</CardTitle>
            <Users className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{isLoading ? '...' : totalAccounts}</div>
            <p className="text-xs text-muted-foreground">
              {enabledAccounts} {t('dashboard.stats.enabled')}
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Provider Breakdown */}
      {statistics && statistics.providers.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle>{t('dashboard.providerBreakdown')}</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-6">
              {statistics.providers.map((provider) => {
                // Get first account of this provider to fetch models
                const providerAccount = accounts?.find(
                  (acc) => acc.provider_id === provider.provider_id && acc.enabled
                );

                return (
                  <div key={provider.provider_id} className="space-y-3 border-b pb-4 last:border-0">
                    <div className="flex items-center justify-between">
                      <div className="space-y-1">
                        <p className="font-semibold">{provider.provider_name}</p>
                        <p className="text-xs text-muted-foreground">
                          {provider.account_count} {provider.account_count === 1 ? t('dashboard.account') : t('dashboard.accounts_plural')}
                        </p>
                      </div>
                      <div className="flex gap-6 text-sm">
                        <div className="text-right">
                          <p className="text-xs text-muted-foreground">{t('dashboard.stats.totalIncome')}</p>
                          <p className="font-semibold text-blue-600">${provider.total_income.toFixed(2)}</p>
                        </div>
                        <div className="text-right">
                          <p className="text-xs text-muted-foreground">{t('dashboard.stats.historicalConsumption')}</p>
                          <p className="font-semibold text-orange-600">${provider.total_consumed.toFixed(2)}</p>
                        </div>
                        <div className="text-right">
                          <p className="text-xs text-muted-foreground">{t('dashboard.stats.currentBalance')}</p>
                          <p className="font-semibold text-green-600">${provider.current_balance.toFixed(2)}</p>
                        </div>
                      </div>
                    </div>

                    {/* 模型列表显示 */}
                    {providerAccount && (
                      <ProviderModelsSection
                        providerId={provider.provider_id}
                        providerName={provider.provider_name}
                        accountId={providerAccount.id}
                        compact={false}
                      />
                    )}
                  </div>
                );
              })}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
