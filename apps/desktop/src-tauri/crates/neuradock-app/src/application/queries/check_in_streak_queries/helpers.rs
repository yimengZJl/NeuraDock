use sqlx::SqlitePool;
use std::sync::Arc;

use crate::application::ResultExt;
use neuradock_domain::shared::DomainError;

use super::types::{AccountInfoRow, AccountListRow, DailySummaryRow};

/// Get account metadata (name + provider)
pub async fn get_account_info(
    db: &Arc<SqlitePool>,
    account_id: &str,
) -> Result<AccountInfoRow, DomainError> {
    let query = r#"
        SELECT
            a.name as name,
            a.provider_id as provider_id,
            COALESCE(p.name, a.provider_id) as provider_name
        FROM accounts a
        LEFT JOIN providers p ON p.id = a.provider_id
        WHERE a.id = ?1
    "#;

    let row: Option<AccountInfoRow> = sqlx::query_as(query)
        .bind(account_id)
        .fetch_optional(&**db)
        .await
        .to_repo_err()?;

    row.ok_or_else(|| DomainError::AccountNotFound(account_id.to_string()))
}

pub async fn get_all_account_infos(
    db: &Arc<SqlitePool>,
) -> Result<Vec<AccountListRow>, DomainError> {
    let query = r#"
        SELECT
            a.id as account_id,
            a.name as name,
            a.provider_id as provider_id,
            COALESCE(p.name, a.provider_id) as provider_name
        FROM accounts a
        LEFT JOIN providers p ON p.id = a.provider_id
        ORDER BY name ASC
    "#;

    sqlx::query_as(query).fetch_all(&**db).await.to_repo_err()
}

pub async fn fetch_all_daily_summaries(
    db: &Arc<SqlitePool>,
    account_id: &str,
) -> Result<Vec<DailySummaryRow>, DomainError> {
    let query = r#"
        SELECT
            DATE(recorded_at) AS check_in_date,
            MAX(total_income) AS daily_total_income,
            MAX(current_balance) AS daily_balance,
            MAX(total_consumed) AS daily_consumed
        FROM balance_history
        WHERE account_id = ?1
        GROUP BY DATE(recorded_at)
        ORDER BY check_in_date ASC
    "#;

    sqlx::query_as(query)
        .bind(account_id)
        .fetch_all(&**db)
        .await
        .to_repo_err()
}
