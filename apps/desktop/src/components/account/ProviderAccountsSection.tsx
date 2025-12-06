import { useState } from "react";
import { RefreshCw } from "lucide-react";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { AccountCard } from "./AccountCard";
import { Account } from "@/lib/tauri-commands";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from "@/components/ui/collapsible";

interface ProviderStats {
  provider_id: string;
  provider_name: string;
  account_count: number;
  total_balance: number;
}

interface ProviderAccountsSectionProps {
  providerName: string;
  accounts: Account[];
  stats?: ProviderStats;
  onEdit: (account: Account) => void;
  onRefreshBalances: (accounts: Account[]) => Promise<void>;
}

export function ProviderAccountsSection({
  providerName,
  accounts,
  stats,
  onEdit,
  onRefreshBalances,
}: ProviderAccountsSectionProps) {
  const { t } = useTranslation();
  const [isRefreshing, setIsRefreshing] = useState(false);
  const [isOpen, setIsOpen] = useState(true);

  const handleRefresh = async () => {
    const enabledAccounts = accounts.filter((a) => a.enabled);
    if (enabledAccounts.length === 0) {
      toast.error(t("accounts.noEnabledAccounts"));
      return;
    }

    setIsRefreshing(true);
    try {
      await onRefreshBalances(accounts);
    } finally {
      setIsRefreshing(false);
    }
  };

  return (
    <Collapsible open={isOpen} onOpenChange={setIsOpen}>
      <Card>
        <CollapsibleTrigger asChild>
          <CardHeader className="cursor-pointer hover:bg-accent/5 transition-colors">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <CardTitle className="text-xl">{providerName}</CardTitle>
                <Badge variant="secondary">{accounts.length} {t("accounts.accounts")}</Badge>
                {stats && stats.total_balance > 0 && (
                  <Badge variant="outline" className="bg-primary/5">
                    ¥{stats.total_balance.toFixed(2)}
                  </Badge>
                )}
              </div>
              <div className="flex items-center gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleRefresh();
                  }}
                  disabled={isRefreshing}
                  className="rounded-full"
                >
                  <RefreshCw className={`h-4 w-4 ${isRefreshing ? "animate-spin" : ""}`} />
                  {t("accounts.refreshBalances")}
                </Button>
              </div>
            </div>
          </CardHeader>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <CardContent>
            <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
              {accounts.map((account) => (
                <AccountCard
                  key={account.id}
                  account={account}
                  onEdit={() => onEdit(account)}
                />
              ))}
            </div>
          </CardContent>
        </CollapsibleContent>
      </Card>
    </Collapsible>
  );
}
