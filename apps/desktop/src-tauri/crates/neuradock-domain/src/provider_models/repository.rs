use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::shared::DomainError;

/// Provider models domain entity
#[derive(Debug, Clone)]
pub struct ProviderModels {
    pub provider_id: String,
    pub models: Vec<String>,
    pub fetched_at: DateTime<Utc>,
}

/// Repository trait for provider models
#[async_trait]
pub trait ProviderModelsRepository: Send + Sync {
    /// Save or update provider models
    async fn save(&self, provider_id: &str, models: &[String]) -> Result<(), DomainError>;

    /// Find models by provider ID
    async fn find_by_provider(
        &self,
        provider_id: &str,
    ) -> Result<Option<ProviderModels>, DomainError>;

    /// Check if provider models are stale (older than specified hours)
    async fn is_stale(&self, provider_id: &str, max_age_hours: i64) -> Result<bool, DomainError>;

    /// Delete models for a provider
    async fn delete_by_provider(&self, provider_id: &str) -> Result<(), DomainError>;
}
