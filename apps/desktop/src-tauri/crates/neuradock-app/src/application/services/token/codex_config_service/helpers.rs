use anyhow::{Context, Result};
use std::path::PathBuf;

pub(super) fn get_codex_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Cannot find home directory")?;
    Ok(home.join(".codex"))
}

pub(super) fn get_codex_config_path() -> Result<PathBuf> {
    Ok(get_codex_dir()?.join("config.toml"))
}

pub(super) fn get_codex_auth_path() -> Result<PathBuf> {
    Ok(get_codex_dir()?.join("auth.json"))
}

/// Ensure API key has sk- prefix
pub(super) fn ensure_sk_prefix(key: &str) -> String {
    if key.starts_with("sk-") {
        key.to_string()
    } else {
        format!("sk-{}", key)
    }
}

pub(super) fn sanitize_provider_slug(provider_id: &str) -> String {
    provider_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect()
}

pub(super) fn ensure_v1_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim_end_matches('/');
    if trimmed.ends_with("/v1") {
        trimmed.to_string()
    } else {
        format!("{}/v1", trimmed)
    }
}

pub(super) fn generate_provider_config(
    provider_slug: &str,
    provider_name: &str,
    base_url: &str,
    model: Option<&str>,
) -> String {
    let model_name = model.unwrap_or("gpt-5");
    let base_url_v1 = ensure_v1_base_url(base_url);

    format!(
        r#"model = "{}"
model_provider = "{}"
preferred_auth_method = "apikey"


[model_providers.{}]
name = "{}"
base_url = "{}"
wire_api = "responses"
"#,
        model_name, provider_slug, provider_slug, provider_name, base_url_v1
    )
}

/// Generate config.toml content for generic OpenAI-compatible provider (for independent keys)
pub(super) fn generate_generic_config(base_url: &str, model: Option<&str>) -> String {
    let model_name = model.unwrap_or("gpt-4o");
    let base_url_v1 = ensure_v1_base_url(base_url);

    format!(
        r#"model = "{}"
model_provider = "openai_compatible"
preferred_auth_method = "apikey"


[model_providers.openai_compatible]
name = "OpenAI Compatible API"
base_url = "{}"
wire_api = "responses"
"#,
        model_name, base_url_v1
    )
}
