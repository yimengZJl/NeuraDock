use async_trait::async_trait;

use super::ProxyConfig;
use crate::shared::DomainError;

/// Proxy configuration repository trait
#[async_trait]
pub trait ProxyConfigRepository: Send + Sync {
    /// Get the proxy configuration (singleton)
    async fn get(&self) -> Result<ProxyConfig, DomainError>;

    /// Save the proxy configuration
    async fn save(&self, config: &ProxyConfig) -> Result<(), DomainError>;
}
