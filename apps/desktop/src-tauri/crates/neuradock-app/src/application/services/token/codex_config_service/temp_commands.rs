use anyhow::Result;

use neuradock_domain::token::ApiToken;

/// Generate temporary export commands for current shell session
/// Note: This feature is temporarily unavailable
#[allow(dead_code)]
pub(super) fn generate_temp_commands_impl(
    _token: &ApiToken,
    _provider_id: &str,
    _provider_name: &str,
    _base_url: &str,
    _model: Option<&str>,
) -> Result<String> {
    Err(anyhow::anyhow!(
        "Temporary configuration is currently unavailable. Please use global configuration instead."
    ))
}

/// Generate temporary export commands with API key string (for independent keys)
#[allow(dead_code)]
pub(super) fn generate_temp_commands_with_key_impl(
    _api_key: &str,
    _base_url: &str,
    _model: Option<&str>,
) -> Result<String> {
    Err(anyhow::anyhow!(
        "Temporary configuration is currently unavailable. Please use global configuration instead."
    ))
}
