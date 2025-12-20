import { Users, DollarSign, TrendingUp, Wallet, Activity, Zap } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { useAccounts } from '@/hooks/useAccounts';
import { useBalanceStatistics } from '@/hooks/useBalance';
import { ProviderModelsSection } from '@/components/account/ProviderModelsSection';
import { useTranslation } from 'react-i18next';
import { motion, type Variants } from 'framer-motion';
import { formatCurrency } from '@/lib/formatters';

import { PageContainer } from '@/components/layout/PageContainer';
import { PageContent, Section } from '@/components/layout/PageContent';
import { BentoGrid } from '@/components/layout/CardGrid';
import { DashboardSkeleton } from '@/components/skeletons/DashboardSkeleton';
import { cn } from '@/lib/utils';
import { Badge } from '@/components/ui/badge';

export function HomePage() {
  const { data: accounts, isLoading } = useAccounts();
  const { data: statistics, isLoading: statsLoading } = useBalanceStatistics();
  const { t } = useTranslation();

  if (isLoading || statsLoading) {
    return (
      <PageContainer>
        <DashboardSkeleton />
      </PageContainer>
    );
  }

  const enabledAccounts = accounts?.filter(a => a.enabled).length || 0;
  const totalAccounts = accounts?.length || 0;

  // Improved animation variants
  const container: Variants = {
    hidden: { opacity: 0 },
    show: {
      opacity: 1,
      transition: {
        staggerChildren: 0.05,
        delayChildren: 0.1
      }
    }
  };

  const item: Variants = {
    hidden: { opacity: 0, y: 10, scale: 0.98 },
    show: { 
      opacity: 1, 
      y: 0, 
      scale: 1,
      transition: { 
        type: 'spring',
        stiffness: 260,
        damping: 20
      } 
    }
  };

  // Common card interactive styles
  const interactiveCardClass = "bg-card border shadow-sm transition-all duration-200 hover:scale-[1.02] hover:shadow-md active:scale-[0.98] cursor-pointer";

  return (
    <PageContainer title={t('dashboard.title')}>
      <PageContent maxWidth="lg">
        {/* Bento Grid Overview */}
        <motion.div
          variants={container}
          initial="hidden"
          animate="show"
        >
          <BentoGrid>
            {/* Main Balance Card - Spans 2 cols on large screens */}
            <motion.div variants={item} className="md:col-span-2">
              <Card className={cn(
                "h-full relative overflow-hidden border-primary/20 shadow-md",
                "bg-gradient-to-br from-background via-background to-primary/5 dark:from-background dark:via-background dark:to-primary/10",
                "transition-all duration-200 hover:scale-[1.01] hover:shadow-lg active:scale-[0.99] cursor-pointer"
              )}>
                <div className="absolute top-0 right-0 p-4 opacity-10">
                  <Wallet className="w-24 h-24 text-primary" />
                </div>
                <CardHeader className="pb-2">
                  <CardTitle className="text-sm font-medium text-muted-foreground flex items-center gap-2">
                    <Wallet className="h-4 w-4 text-primary" />
                    {t('dashboard.stats.currentBalance')}
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="text-4xl font-bold tracking-tight tabular-nums text-foreground">
                    {statsLoading ? '...' : statistics ? formatCurrency(statistics.total_current_balance) : '$0.00'}
                  </div>
                  <p className="text-sm text-muted-foreground mt-1 flex items-center gap-1">
                    <span className="inline-block w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
                    {t('dashboard.stats.availableBalance')}
                  </p>
                </CardContent>
              </Card>
            </motion.div>

            {/* Income & Consumption Stack */}
            <motion.div variants={item} className="space-y-4">
              <Card className={interactiveCardClass}>
                <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                  <CardTitle className="text-sm font-medium text-muted-foreground">{t('dashboard.stats.totalIncome')}</CardTitle>
                  <div className="p-2 rounded-full bg-blue-50 dark:bg-blue-900/20">
                    <DollarSign className="h-4 w-4 text-blue-600 dark:text-blue-400" />
                  </div>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold tabular-nums text-foreground">
                    {statsLoading ? '...' : statistics ? formatCurrency(statistics.total_income) : '$0.00'}
                  </div>
                </CardContent>
              </Card>
              <Card className={interactiveCardClass}>
                <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                  <CardTitle className="text-sm font-medium text-muted-foreground">{t('dashboard.stats.historicalConsumption')}</CardTitle>
                  <div className="p-2 rounded-full bg-orange-50 dark:bg-orange-900/20">
                    <TrendingUp className="h-4 w-4 text-orange-600 dark:text-orange-400" />
                  </div>
                </CardHeader>
                <CardContent>
                  <div className="text-2xl font-bold tabular-nums text-foreground">
                    {statsLoading ? '...' : statistics ? formatCurrency(statistics.total_consumed) : '$0.00'}
                  </div>
                </CardContent>
              </Card>
            </motion.div>

            {/* Accounts Status */}
            <motion.div variants={item}>
              <Card className={cn(interactiveCardClass, "h-full flex flex-col justify-center")}>
                <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                  <CardTitle className="text-sm font-medium text-muted-foreground">{t('dashboard.stats.totalAccounts')}</CardTitle>
                  <div className="p-2 rounded-full bg-emerald-50 dark:bg-emerald-900/20">
                    <Users className="h-4 w-4 text-emerald-600 dark:text-emerald-400" />
                  </div>
                </CardHeader>
                <CardContent>
                  <div className="text-3xl font-bold tabular-nums text-foreground">{isLoading ? '...' : totalAccounts}</div>
                  <div className="flex items-center gap-2 mt-2">
                    <Badge variant="secondary" className="bg-emerald-100 text-emerald-700 dark:bg-emerald-900/30 dark:text-emerald-400 border-0">
                      {enabledAccounts} {t('dashboard.stats.enabled')}
                    </Badge>
                  </div>
                </CardContent>
              </Card>
            </motion.div>
          </BentoGrid>
        </motion.div>

        {/* Provider Breakdown Section */}
        {statistics && statistics.providers.length > 0 && (
          <motion.div
            variants={container}
            initial="hidden"
            animate="show"
            className="mt-8"
          >
            <Section
              title={
                <div className="flex items-center gap-2">
                  <Activity className="h-5 w-5 text-primary" />
                  <span>{t('dashboard.providerBreakdown')}</span>
                </div>
              }
            >
              {/* Grid layout for scalability */}
              <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
                {statistics.providers.map((provider) => {
                  const providerAccount = accounts?.find(
                    (acc) => acc.provider_id === provider.provider_id && acc.enabled
                  );

                  return (
                    <motion.div key={provider.provider_id} variants={item} className="h-full">
                      <Card className={cn(interactiveCardClass, "h-full flex flex-col overflow-hidden")}>
                        {/* Provider Header */}
                        <div className="p-4 bg-muted/30 border-b flex items-center justify-between">
                          <div className="flex items-center gap-3">
                            <div className="p-2 bg-background rounded-full border shadow-sm">
                              <Zap className="h-4 w-4 text-yellow-500" />
                            </div>
                            <div>
                              <h3 className="font-semibold text-foreground">{provider.provider_name}</h3>
                              <p className="text-xs text-muted-foreground">
                                {provider.account_count} {provider.account_count === 1 ? t('dashboard.account') : t('dashboard.accounts_plural')}
                              </p>
                            </div>
                          </div>
                          <div className="text-right">
                             <div className="font-mono font-bold text-green-600 dark:text-green-400">
                               {formatCurrency(provider.current_balance)}
                             </div>
                          </div>
                        </div>

                        {/* Models List - Flex grow to push content */}
                        <CardContent className="p-4 flex-grow">
                          {providerAccount ? (
                            <ProviderModelsSection
                              providerId={provider.provider_id}
                              providerName={provider.provider_name}
                              accountId={providerAccount.id}
                              compact={true} // Use compact mode for grid layout
                            />
                          ) : (
                            <div className="h-full flex items-center justify-center text-sm text-muted-foreground italic min-h-[60px]">
                              {t('dashboard.noActiveAccounts')}
                            </div>
                          )}
                        </CardContent>
                      </Card>
                    </motion.div>
                  );
                })}
              </div>
            </Section>
          </motion.div>
        )}
      </PageContent>
    </PageContainer>
  );
}
