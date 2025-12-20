use anyhow::Result;
use serde_json::json;
use std::fs;

use super::helpers::{
    ensure_sk_prefix, generate_generic_config, generate_provider_config, get_codex_auth_path,
    get_codex_config_path, get_codex_dir, sanitize_provider_slug,
};
use neuradock_domain::token::ApiToken;

pub(super) fn configure_global_impl(
    token: &ApiToken,
    provider_id: &str,
    provider_name: &str,
    base_url: &str,
    model: Option<&str>,
) -> Result<String> {
    let codex_dir = get_codex_dir()?;
    let config_path = get_codex_config_path()?;
    let auth_path = get_codex_auth_path()?;

    // Ensure directory exists
    fs::create_dir_all(&codex_dir)?;

    // Generate config.toml based on provider metadata
    let provider_slug = sanitize_provider_slug(provider_id);
    let display_name = if provider_name.is_empty() {
        provider_id
    } else {
        provider_name
    };
    let config_content = generate_provider_config(&provider_slug, display_name, base_url, model);

    // Write config.toml
    fs::write(&config_path, &config_content)?;
    log::info!("Codex config.toml written to: {}", config_path.display());

    // Create auth.json with API key (ensure sk- prefix)
    let api_key = ensure_sk_prefix(token.key());
    let auth_content = json!({
        "OPENAI_API_KEY": api_key
    });

    let auth_json = serde_json::to_string_pretty(&auth_content)?;
    fs::write(&auth_path, auth_json)?;
    log::info!("Codex auth.json written to: {}", auth_path.display());

    Ok(format!(
        "Successfully configured Codex globally:\n  - config.toml: {}\n  - auth.json: {}",
        config_path.display(),
        auth_path.display()
    ))
}

pub(super) fn configure_global_with_key_impl(
    api_key: &str,
    base_url: &str,
    model: Option<&str>,
) -> Result<String> {
    let codex_dir = get_codex_dir()?;
    let config_path = get_codex_config_path()?;
    let auth_path = get_codex_auth_path()?;

    // Ensure directory exists
    fs::create_dir_all(&codex_dir)?;

    // Generate generic OpenAI-compatible config
    let config_content = generate_generic_config(base_url, model);

    // Write config.toml
    fs::write(&config_path, &config_content)?;
    log::info!("Codex config.toml written to: {}", config_path.display());

    // Create auth.json with API key (ensure sk- prefix)
    let api_key = ensure_sk_prefix(api_key);
    let auth_content = json!({
        "OPENAI_API_KEY": api_key
    });

    let auth_json = serde_json::to_string_pretty(&auth_content)?;
    fs::write(&auth_path, auth_json)?;
    log::info!("Codex auth.json written to: {}", auth_path.display());

    Ok(format!(
        "Successfully configured Codex globally:\n  - config.toml: {}\n  - auth.json: {}",
        config_path.display(),
        auth_path.display()
    ))
}

pub(super) fn clear_global_impl() -> Result<String> {
    let config_path = get_codex_config_path()?;
    let auth_path = get_codex_auth_path()?;
    let mut removed = vec![];

    if config_path.exists() {
        fs::remove_file(&config_path)?;
        removed.push(format!("config.toml: {}", config_path.display()));
    }

    if auth_path.exists() {
        fs::remove_file(&auth_path)?;
        removed.push(format!("auth.json: {}", auth_path.display()));
    }

    if removed.is_empty() {
        return Ok("No Codex configuration files found".to_string());
    }

    log::info!("Successfully cleared Codex configuration");

    Ok(format!(
        "Successfully cleared Codex configuration:\n  - {}",
        removed.join("\n  - ")
    ))
}
