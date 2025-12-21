use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{FromRow, SqlitePool};
use std::sync::Arc;

use crate::persistence::SqliteRepositoryBase;
use neuradock_domain::balance_history::{
    BalanceHistoryDailySummary, BalanceHistoryRecord, BalanceHistoryRepository,
};
use neuradock_domain::shared::{AccountId, DomainError};

#[derive(FromRow)]
struct BalanceHistoryRow {
    id: String,
    account_id: String,
    current_balance: f64,
    total_consumed: f64,
    total_quota: f64,
    recorded_at: DateTime<Utc>,
}

impl BalanceHistoryRow {
    fn into_record(self) -> BalanceHistoryRecord {
        BalanceHistoryRecord::restore(
            self.id,
            AccountId::from_string(&self.account_id),
            self.current_balance,
            self.total_consumed,
            self.total_quota,
            self.recorded_at,
        )
    }
}

#[derive(FromRow)]
struct DailySummaryRow {
    check_in_date: String,
    daily_total_quota: f64,
    daily_balance: f64,
    daily_consumed: f64,
}

impl DailySummaryRow {
    fn try_into_summary(self) -> Result<BalanceHistoryDailySummary, DomainError> {
        let date = NaiveDate::parse_from_str(&self.check_in_date, "%Y-%m-%d").map_err(|e| {
            DomainError::Validation(format!(
                "Invalid check_in_date: {} ({})",
                self.check_in_date, e
            ))
        })?;

        Ok(BalanceHistoryDailySummary::restore(
            date,
            self.daily_total_quota,
            self.daily_balance,
            self.daily_consumed,
        ))
    }
}

pub struct SqliteBalanceHistoryRepository {
    base: SqliteRepositoryBase,
}

impl SqliteBalanceHistoryRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            base: SqliteRepositoryBase::new(pool),
        }
    }
}

#[async_trait]
impl BalanceHistoryRepository for SqliteBalanceHistoryRepository {
    async fn save(&self, record: &BalanceHistoryRecord) -> Result<(), DomainError> {
        let query = r#"
            INSERT OR REPLACE INTO balance_history (
                id,
                account_id,
                current_balance,
                total_consumed,
                total_quota,
                recorded_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
        "#;

        self.base
            .execute(
                sqlx::query(query)
                    .bind(record.id())
                    .bind(record.account_id().as_str())
                    .bind(record.current_balance())
                    .bind(record.total_consumed())
                    .bind(record.total_quota())
                    .bind(record.recorded_at()),
                "Save balance history",
            )
            .await?;

        Ok(())
    }

    async fn find_latest_by_account_id(
        &self,
        account_id: &AccountId,
    ) -> Result<Option<BalanceHistoryRecord>, DomainError> {
        let query = r#"
            SELECT
                id,
                account_id,
                current_balance,
                total_consumed,
                total_quota,
                recorded_at
            FROM balance_history
            WHERE account_id = ?1
            ORDER BY recorded_at DESC
            LIMIT 1
        "#;

        let row: Option<BalanceHistoryRow> = self
            .base
            .fetch_optional(
                sqlx::query_as(query).bind(account_id.as_str()),
                "Find latest balance history by account ID",
            )
            .await?;

        Ok(row.map(|r| r.into_record()))
    }

    async fn find_latest_by_account_id_on_date(
        &self,
        account_id: &AccountId,
        date: NaiveDate,
    ) -> Result<Option<BalanceHistoryRecord>, DomainError> {
        let query = r#"
            SELECT
                id,
                account_id,
                current_balance,
                total_consumed,
                total_quota,
                recorded_at
            FROM balance_history
            WHERE account_id = ?1 AND DATE(recorded_at) = ?2
            ORDER BY recorded_at DESC
            LIMIT 1
        "#;

        let row: Option<BalanceHistoryRow> = self
            .base
            .fetch_optional(
                sqlx::query_as(query)
                    .bind(account_id.as_str())
                    .bind(date.format("%Y-%m-%d").to_string()),
                "Find latest balance history by account ID and date",
            )
            .await?;

        Ok(row.map(|r| r.into_record()))
    }

    async fn list_all_daily_summaries(
        &self,
        account_id: &AccountId,
    ) -> Result<Vec<BalanceHistoryDailySummary>, DomainError> {
        let query = r#"
            SELECT
                DATE(recorded_at) AS check_in_date,
                MAX(total_quota) AS daily_total_quota,
                MAX(current_balance) AS daily_balance,
                MAX(total_consumed) AS daily_consumed
            FROM balance_history
            WHERE account_id = ?1
            GROUP BY DATE(recorded_at)
            ORDER BY check_in_date ASC
        "#;

        let rows: Vec<DailySummaryRow> = self
            .base
            .fetch_all(
                sqlx::query_as(query).bind(account_id.as_str()),
                "List all daily summaries",
            )
            .await?;

        rows.into_iter().map(|r| r.try_into_summary()).collect()
    }

    async fn list_daily_summaries_in_range(
        &self,
        account_id: &AccountId,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<BalanceHistoryDailySummary>, DomainError> {
        let query = r#"
            WITH daily_summary AS (
                SELECT
                    DATE(recorded_at) AS check_in_date,
                    MAX(total_quota) AS daily_total_quota,
                    MAX(current_balance) AS daily_balance,
                    MAX(total_consumed) AS daily_consumed
                FROM balance_history
                WHERE account_id = ?1
                GROUP BY DATE(recorded_at)
            )
            SELECT
                check_in_date,
                daily_total_quota,
                daily_balance,
                daily_consumed
            FROM daily_summary
            WHERE check_in_date >= ?2
              AND check_in_date <= ?3
            ORDER BY check_in_date ASC
        "#;

        let rows: Vec<DailySummaryRow> = self
            .base
            .fetch_all(
                sqlx::query_as(query)
                    .bind(account_id.as_str())
                    .bind(start_date.format("%Y-%m-%d").to_string())
                    .bind(end_date.format("%Y-%m-%d").to_string()),
                "List daily summaries in range",
            )
            .await?;

        rows.into_iter().map(|r| r.try_into_summary()).collect()
    }

    async fn find_daily_summary(
        &self,
        account_id: &AccountId,
        date: NaiveDate,
    ) -> Result<Option<BalanceHistoryDailySummary>, DomainError> {
        let query = r#"
            WITH daily_summary AS (
                SELECT
                    DATE(recorded_at) AS check_in_date,
                    MAX(total_quota) AS daily_total_quota,
                    MAX(current_balance) AS daily_balance,
                    MAX(total_consumed) AS daily_consumed
                FROM balance_history
                WHERE account_id = ?1
                GROUP BY DATE(recorded_at)
            )
            SELECT
                check_in_date,
                daily_total_quota,
                daily_balance,
                daily_consumed
            FROM daily_summary
            WHERE check_in_date = ?2
            LIMIT 1
        "#;

        let row: Option<DailySummaryRow> = self
            .base
            .fetch_optional(
                sqlx::query_as(query)
                    .bind(account_id.as_str())
                    .bind(date.format("%Y-%m-%d").to_string()),
                "Find daily summary",
            )
            .await?;

        row.map(|r| r.try_into_summary()).transpose()
    }

    async fn list_distinct_account_ids(&self) -> Result<Vec<AccountId>, DomainError> {
        let query = "SELECT DISTINCT account_id FROM balance_history";
        let ids: Vec<String> = sqlx::query_scalar(query)
            .fetch_all(self.base.pool())
            .await
            .map_err(|e| DomainError::Repository(format!("List distinct account ids: {e}")))?;

        Ok(ids
            .into_iter()
            .map(|id| AccountId::from_string(&id))
            .collect())
    }
}
