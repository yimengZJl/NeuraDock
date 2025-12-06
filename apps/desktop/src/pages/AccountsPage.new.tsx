import { useState, useMemo } from "react";
import { Plus, Upload, Search } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { PageContainer, PageHeader, PageSection } from "@/components/ui/page-container";
import { EmptyState } from "@/components/ui/empty-state";
import { LoadingState } from "@/components/ui/loading";
import { useAccounts } from "@/hooks/useAccounts";
import { useProviders } from "@/hooks/useProviders";
import { useBalanceStatistics, useRefreshAllBalances } from "@/hooks/useBalance";
import { AccountStatisticsCards } from "@/components/account/AccountStatisticsCards";
import { ProviderAccountsSection } from "@/components/account/ProviderAccountsSection";
import { AccountDialog } from "@/components/account/AccountDialog";
import { JsonImportDialog } from "@/components/account/JsonImportDialog";
import { BatchUpdateDialog } from "@/components/account/BatchUpdateDialog";
import { BatchCheckInButton } from "@/components/checkin/BatchCheckInButton";
import { Account, AccountDetail } from "@/lib/tauri-commands";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "sonner";

export function AccountsPage() {
  const { t } = useTranslation();
  const { data: accounts, isLoading } = useAccounts();
  const { data: providers } = useProviders();
  const { data: statistics } = useBalanceStatistics();
  const refreshAllBalancesMutation = useRefreshAllBalances();

  const [searchQuery, setSearchQuery] = useState("");
  const [accountDialogOpen, setAccountDialogOpen] = useState(false);
  const [jsonImportDialogOpen, setJsonImportDialogOpen] = useState(false);
  const [batchUpdateDialogOpen, setBatchUpdateDialogOpen] = useState(false);
  const [editingAccount, setEditingAccount] = useState<AccountDetail | null>(null);

  // Filter accounts
  const filteredAccounts = useMemo(() => {
    if (!accounts) return [];
    return accounts.filter(
      (account) =>
        account.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        account.provider_name.toLowerCase().includes(searchQuery.toLowerCase())
    );
  }, [accounts, searchQuery]);

  // Group by provider
  const accountsByProvider = useMemo(() => {
    return filteredAccounts.reduce((acc, account) => {
      const providerId = account.provider_id;
      if (!acc[providerId]) {
        acc[providerId] = [];
      }
      acc[providerId].push(account);
      return acc;
    }, {} as Record<string, Account[]>);
  }, [filteredAccounts]);

  const handleEdit = async (account: Account) => {
    try {
      const accountDetail = await invoke<AccountDetail>("get_account_detail", {
        accountId: account.id,
      });
      setEditingAccount(accountDetail);
      setAccountDialogOpen(true);
    } catch (error) {
      console.error("Failed to fetch account details:", error);
      toast.error(t("common.error"));
    }
  };

  const handleCreate = () => {
    setEditingAccount(null);
    setAccountDialogOpen(true);
  };

  const handleDialogClose = () => {
    setAccountDialogOpen(false);
    setEditingAccount(null);
  };

  const getProviderStats = (providerId: string) => {
    return statistics?.providers.find((p) => p.provider_id === providerId);
  };

  const handleRefreshProviderBalances = async (providerAccounts: Account[]) => {
    const enabledAccountIds = providerAccounts
      .filter((a) => a.enabled)
      .map((a) => a.id);

    if (enabledAccountIds.length === 0) {
      toast.error(t("accounts.noEnabledAccounts"));
      return;
    }

    try {
      await refreshAllBalancesMutation.mutateAsync(enabledAccountIds);
      toast.success(t("accounts.balancesRefreshed"));
    } catch (error) {
      console.error("Failed to refresh balances:", error);
      toast.error(t("common.error"));
    }
  };

  if (isLoading) {
    return (
      <PageContainer>
        <LoadingState message={t("common.loading")} />
      </PageContainer>
    );
  }

  return (
    <PageContainer>
      <PageHeader
        title={t("accounts.title")}
        description={t("accounts.description")}
        action={
          <div className="flex items-center gap-2">
            <BatchCheckInButton />
            <Button
              variant="outline"
              size="sm"
              onClick={() => setBatchUpdateDialogOpen(true)}
              className="rounded-full"
            >
              {t("accounts.batchUpdate")}
            </Button>
            <Button
              variant="outline"
              size="sm"
              onClick={() => setJsonImportDialogOpen(true)}
              className="rounded-full"
            >
              <Upload className="h-4 w-4" />
              {t("accounts.import")}
            </Button>
            <Button
              size="sm"
              onClick={handleCreate}
              className="rounded-full"
            >
              <Plus className="h-4 w-4" />
              {t("accounts.addAccount")}
            </Button>
          </div>
        }
      />

      <PageSection>
        <AccountStatisticsCards statistics={statistics} isLoading={isLoading} />
      </PageSection>

      <PageSection>
        <div className="relative">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder={t("accounts.searchPlaceholder")}
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            className="pl-10 rounded-full"
          />
        </div>
      </PageSection>

      {filteredAccounts.length === 0 ? (
        <EmptyState
          icon={<Plus className="h-16 w-16" />}
          title={t("accounts.noAccounts")}
          description={t("accounts.noAccountsDescription")}
          action={{
            label: t("accounts.addAccount"),
            onClick: handleCreate,
          }}
        />
      ) : (
        <PageSection>
          <div className="space-y-4">
            {Object.entries(accountsByProvider).map(
              ([providerId, providerAccounts]) => {
                const providerName =
                  providerAccounts[0]?.provider_name || "Unknown";
                return (
                  <ProviderAccountsSection
                    key={providerId}
                    providerName={providerName}
                    accounts={providerAccounts}
                    stats={getProviderStats(providerId)}
                    onEdit={handleEdit}
                    onRefreshBalances={handleRefreshProviderBalances}
                  />
                );
              }
            )}
          </div>
        </PageSection>
      )}

      <AccountDialog
        open={accountDialogOpen}
        onOpenChange={handleDialogClose}
        account={editingAccount}
        providers={providers || []}
      />

      <JsonImportDialog
        open={jsonImportDialogOpen}
        onOpenChange={setJsonImportDialogOpen}
        providers={providers || []}
      />

      <BatchUpdateDialog
        open={batchUpdateDialogOpen}
        onOpenChange={setBatchUpdateDialogOpen}
        accounts={accounts || []}
        providers={providers || []}
      />
    </PageContainer>
  );
}
