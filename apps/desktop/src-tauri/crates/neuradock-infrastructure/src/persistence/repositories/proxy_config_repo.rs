use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use std::sync::Arc;

use neuradock_domain::proxy_config::{ProxyConfig, ProxyConfigRepository, ProxyType};
use neuradock_domain::shared::DomainError;

use crate::persistence::result_ext::ResultExt;

/// Parse datetime string, supporting both old format '2025-12-20 12:34:56' and ISO 8601 format '2025-12-20T12:34:56Z'
fn parse_datetime_flexible(datetime_str: &str) -> Result<DateTime<Utc>, DomainError> {
    // Try ISO 8601 format first
    if let Ok(dt) = datetime_str.parse::<DateTime<Utc>>() {
        return Ok(dt);
    }

    // Try old SQLite format by converting to ISO 8601
    let iso_format = datetime_str.replace(' ', "T") + "Z";
    iso_format
        .parse::<DateTime<Utc>>()
        .map_err(|e| DomainError::Repository(format!("Invalid datetime format: {}", e)))
}

/// SQLite implementation of ProxyConfigRepository
pub struct SqliteProxyConfigRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteProxyConfigRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProxyConfigRepository for SqliteProxyConfigRepository {
    async fn get(&self) -> Result<ProxyConfig, DomainError> {
        let row = sqlx::query(
            r#"
            SELECT enabled, proxy_type, host, port, created_at, updated_at
            FROM proxy_settings
            WHERE id = 1
            "#,
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_repo_error("Failed to load proxy configuration")?;

        let Some(row) = row else {
            return Ok(ProxyConfig::new_disabled());
        };

        let enabled: bool = row.get("enabled");
        let proxy_type_str: String = row.get("proxy_type");
        let host: String = row.get("host");
        let port: i64 = row.get("port");
        let created_at: String = row.get("created_at");
        let updated_at: String = row.get("updated_at");

        let proxy_type = ProxyType::from_str(&proxy_type_str)?;

        let config = ProxyConfig::builder()
            .enabled(enabled)
            .proxy_type(proxy_type)
            .host(host)
            .port(port as u16)
            .created_at(parse_datetime_flexible(&created_at)?)
            .updated_at(parse_datetime_flexible(&updated_at)?)
            .build();

        Ok(config)
    }

    async fn save(&self, config: &ProxyConfig) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO proxy_settings (id, enabled, proxy_type, host, port, created_at, updated_at)
            VALUES (
                1,
                ?,
                ?,
                ?,
                ?,
                strftime('%Y-%m-%dT%H:%M:%SZ', 'now'),
                strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
            )
            ON CONFLICT(id) DO UPDATE SET
                enabled = excluded.enabled,
                proxy_type = excluded.proxy_type,
                host = excluded.host,
                port = excluded.port,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(config.is_enabled())
        .bind(config.proxy_type().as_str())
        .bind(config.host())
        .bind(config.port() as i64)
        .execute(self.pool.as_ref())
        .await
        .map_repo_error("Failed to save proxy configuration")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        // Run migrations
        sqlx::query(
            r#"
            CREATE TABLE proxy_settings (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                enabled BOOLEAN NOT NULL DEFAULT 0,
                proxy_type TEXT NOT NULL DEFAULT 'http' CHECK(proxy_type IN ('http', 'socks5')),
                host TEXT NOT NULL DEFAULT '',
                port INTEGER NOT NULL DEFAULT 0 CHECK(port >= 0 AND port <= 65535),
                created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
                updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
            );

            INSERT INTO proxy_settings (id, enabled, proxy_type, host, port, created_at, updated_at)
            VALUES (1, 0, 'http', '', 0, strftime('%Y-%m-%dT%H:%M:%SZ', 'now'), strftime('%Y-%m-%dT%H:%M:%SZ', 'now'));
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_get_default_config() {
        let pool = setup_test_db().await;
        let repo = SqliteProxyConfigRepository::new(Arc::new(pool));

        let config = repo.get().await.unwrap();
        assert!(!config.is_enabled());
        assert_eq!(config.proxy_type(), ProxyType::Http);
        assert_eq!(config.host(), "");
        assert_eq!(config.port(), 0);
    }

    #[tokio::test]
    async fn test_save_and_get() {
        let pool = setup_test_db().await;
        let repo = SqliteProxyConfigRepository::new(Arc::new(pool));

        let mut config = ProxyConfig::new(ProxyType::Socks5, "127.0.0.1".to_string(), 1080).unwrap();
        repo.save(&config).await.unwrap();

        let loaded = repo.get().await.unwrap();
        assert!(loaded.is_enabled());
        assert_eq!(loaded.proxy_type(), ProxyType::Socks5);
        assert_eq!(loaded.host(), "127.0.0.1");
        assert_eq!(loaded.port(), 1080);

        // Test update
        config.disable();
        repo.save(&config).await.unwrap();

        let loaded = repo.get().await.unwrap();
        assert!(!loaded.is_enabled());
    }
}
