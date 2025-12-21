use serde::{Deserialize, Serialize};
use specta::Type;

use neuradock_domain::proxy_config::{ProxyConfig, ProxyType};

/// Proxy configuration DTO for frontend
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProxyConfigDto {
    pub enabled: bool,
    pub proxy_type: String, // "http" or "socks5"
    pub host: String,
    pub port: u16,
}

impl From<&ProxyConfig> for ProxyConfigDto {
    fn from(config: &ProxyConfig) -> Self {
        Self {
            enabled: config.is_enabled(),
            proxy_type: config.proxy_type().as_str().to_string(),
            host: config.host().to_string(),
            port: config.port(),
        }
    }
}

/// Input for updating proxy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UpdateProxyConfigInput {
    pub enabled: bool,
    pub proxy_type: String, // "http" or "socks5"
    pub host: String,
    pub port: u16,
}

impl UpdateProxyConfigInput {
    pub fn to_proxy_type(&self) -> Result<ProxyType, String> {
        ProxyType::from_str(&self.proxy_type).map_err(|e| e.to_string())
    }
}
