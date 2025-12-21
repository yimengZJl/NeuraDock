use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::shared::DomainError;

/// WAF cookies domain entity
#[derive(Debug, Clone)]
pub struct WafCookies {
    pub provider_id: String,
    pub cookies: HashMap<String, String>,
    pub fetched_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl WafCookies {
    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

/// Repository trait for WAF cookies
#[async_trait]
pub trait WafCookiesRepository: Send + Sync {
    /// Save or update WAF cookies for a provider
    async fn save(
        &self,
        provider_id: &str,
        cookies: &HashMap<String, String>,
    ) -> Result<(), DomainError>;

    /// Get valid (non-expired) WAF cookies for a provider
    async fn get_valid(&self, provider_id: &str) -> Result<Option<WafCookies>, DomainError>;

    /// Delete WAF cookies for a provider
    async fn delete(&self, provider_id: &str) -> Result<(), DomainError>;

    /// Clean up all expired WAF cookies
    async fn cleanup_expired(&self) -> Result<u64, DomainError>;
}
