use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36";

/// HTTP retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (default: 3)
    pub max_retries: u32,
    /// Initial backoff duration in milliseconds (default: 1000ms)
    pub initial_backoff_ms: u64,
    /// Maximum backoff duration in milliseconds (default: 10000ms)
    pub max_backoff_ms: u64,
    /// Backoff multiplier (default: 2.0 for exponential backoff)
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 1000,
            max_backoff_ms: 10000,
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub quota: f64,
    pub used_quota: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInResult {
    pub success: bool,
    pub message: String,
}

/// Extract domain from URL (including port if present)
pub(super) fn extract_domain(url: &str) -> Result<String> {
    let parsed = url::Url::parse(url)?;
    let host = parsed.host_str().unwrap_or("");

    if let Some(port) = parsed.port() {
        Ok(format!("{}://{}:{}", parsed.scheme(), host, port))
    } else {
        Ok(format!("{}://{}", parsed.scheme(), host))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://example.com/api/user").unwrap(),
            "https://example.com"
        );
        assert_eq!(
            extract_domain("http://test.org:8080/path").unwrap(),
            "http://test.org:8080"
        );
    }
}
