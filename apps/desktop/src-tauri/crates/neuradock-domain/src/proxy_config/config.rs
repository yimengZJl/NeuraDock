use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::str::FromStr;

use crate::shared::DomainError;

/// Proxy type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[serde(rename_all = "lowercase")]
pub enum ProxyType {
    Http,
    Socks5,
}

impl ProxyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProxyType::Http => "http",
            ProxyType::Socks5 => "socks5",
        }
    }
}

impl FromStr for ProxyType {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "http" | "https" => Ok(ProxyType::Http),
            "socks5" => Ok(ProxyType::Socks5),
            _ => Err(DomainError::Validation(format!(
                "Invalid proxy type: {s}. Must be 'http' or 'socks5'"
            ))),
        }
    }
}

/// Proxy configuration aggregate
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProxyConfig {
    enabled: bool,
    proxy_type: ProxyType,
    host: String,
    port: u16,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProxyConfig {
    /// Create a new disabled proxy configuration
    pub fn new_disabled() -> Self {
        Self {
            enabled: false,
            proxy_type: ProxyType::Http,
            host: String::new(),
            port: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a new enabled proxy configuration
    pub fn new(proxy_type: ProxyType, host: String, port: u16) -> Result<Self, DomainError> {
        // Validate host
        if host.trim().is_empty() {
            return Err(DomainError::Validation(
                "Proxy host cannot be empty".to_string(),
            ));
        }

        // Validate port
        if port == 0 {
            return Err(DomainError::Validation(
                "Proxy port must be greater than 0".to_string(),
            ));
        }

        Ok(Self {
            enabled: true,
            proxy_type,
            host: host.trim().to_string(),
            port,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Builder for restoring from persistence
    pub fn builder() -> ProxyConfigBuilder {
        ProxyConfigBuilder {
            enabled: false,
            proxy_type: ProxyType::Http,
            host: String::new(),
            port: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    // Getters
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn proxy_type(&self) -> ProxyType {
        self.proxy_type
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    /// Update proxy configuration
    pub fn update(
        &mut self,
        enabled: bool,
        proxy_type: ProxyType,
        host: String,
        port: u16,
    ) -> Result<(), DomainError> {
        // If enabling, validate host and port
        if enabled {
            if host.trim().is_empty() {
                return Err(DomainError::Validation(
                    "Proxy host cannot be empty when enabled".to_string(),
                ));
            }

            if port == 0 {
                return Err(DomainError::Validation(
                    "Proxy port must be greater than 0 when enabled".to_string(),
                ));
            }
        }

        self.enabled = enabled;
        self.proxy_type = proxy_type;
        self.host = host.trim().to_string();
        self.port = port;
        self.updated_at = Utc::now();

        Ok(())
    }

    /// Disable the proxy
    pub fn disable(&mut self) {
        self.enabled = false;
        self.updated_at = Utc::now();
    }

    /// Get proxy URL if enabled, None otherwise
    pub fn proxy_url(&self) -> Option<String> {
        if !self.enabled {
            return None;
        }

        Some(format!(
            "{}://{}:{}",
            self.proxy_type.as_str(),
            self.host,
            self.port
        ))
    }
}

/// Builder for ProxyConfig (used when restoring from database)
pub struct ProxyConfigBuilder {
    enabled: bool,
    proxy_type: ProxyType,
    host: String,
    port: u16,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ProxyConfigBuilder {
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn proxy_type(mut self, proxy_type: ProxyType) -> Self {
        self.proxy_type = proxy_type;
        self
    }

    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = created_at;
        self
    }

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn build(self) -> ProxyConfig {
        ProxyConfig {
            enabled: self.enabled,
            proxy_type: self.proxy_type,
            host: self.host,
            port: self.port,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proxy_type_from_str() {
        assert_eq!(ProxyType::from_str("http").unwrap(), ProxyType::Http);
        assert_eq!(ProxyType::from_str("HTTP").unwrap(), ProxyType::Http);
        assert_eq!(ProxyType::from_str("https").unwrap(), ProxyType::Http);
        assert_eq!(ProxyType::from_str("socks5").unwrap(), ProxyType::Socks5);
        assert_eq!(ProxyType::from_str("SOCKS5").unwrap(), ProxyType::Socks5);
        assert!(ProxyType::from_str("invalid").is_err());
    }

    #[test]
    fn test_new_disabled() {
        let config = ProxyConfig::new_disabled();
        assert!(!config.is_enabled());
        assert_eq!(config.proxy_url(), None);
    }

    #[test]
    fn test_new_enabled() {
        let config = ProxyConfig::new(ProxyType::Http, "127.0.0.1".to_string(), 7890).unwrap();
        assert!(config.is_enabled());
        assert_eq!(config.proxy_type(), ProxyType::Http);
        assert_eq!(config.host(), "127.0.0.1");
        assert_eq!(config.port(), 7890);
        assert_eq!(
            config.proxy_url(),
            Some("http://127.0.0.1:7890".to_string())
        );
    }

    #[test]
    fn test_validation() {
        // Empty host should fail
        assert!(ProxyConfig::new(ProxyType::Http, "".to_string(), 7890).is_err());

        // Port 0 should fail
        assert!(ProxyConfig::new(ProxyType::Http, "127.0.0.1".to_string(), 0).is_err());
    }

    #[test]
    fn test_update() {
        let mut config = ProxyConfig::new_disabled();

        // Enable with valid settings
        config
            .update(true, ProxyType::Socks5, "localhost".to_string(), 1080)
            .unwrap();
        assert!(config.is_enabled());
        assert_eq!(
            config.proxy_url(),
            Some("socks5://localhost:1080".to_string())
        );

        // Disable
        config.disable();
        assert!(!config.is_enabled());
        assert_eq!(config.proxy_url(), None);
    }
}
