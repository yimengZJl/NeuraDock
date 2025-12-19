use tauri::State;

use crate::presentation::state::AppState;
use neuradock_domain::shared::{AccountId, ProviderId};

#[tauri::command]
#[specta::specta]
pub async fn configure_codex_global(
    token_id: i64,
    account_id: String,
    provider_id: String,
    base_url: String,
    model: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let account_id = AccountId::from_string(&account_id);
    let token_id = neuradock_domain::token::TokenId::new(token_id);

    // Get token from cache
    let tokens = state
        .token_service
        .get_cached_tokens(&account_id)
        .await
        .map_err(|e| e.to_string())?;

    let token = tokens
        .iter()
        .find(|t| t.id() == &token_id)
        .ok_or_else(|| "Token not found".to_string())?;

    let provider_id_obj = ProviderId::from_string(&provider_id);
    let provider = state
        .provider_repo
        .find_by_id(&provider_id_obj)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Provider {} not found", provider_id))?;

    // Configure to Codex
    let result = state
        .codex_config_service
        .configure_global(
            token,
            provider.id().as_str(),
            provider.name(),
            &base_url,
            model.as_deref(),
        )
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
#[specta::specta]
pub async fn generate_codex_temp_commands(
    token_id: i64,
    account_id: String,
    provider_id: String,
    base_url: String,
    model: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let account_id = AccountId::from_string(&account_id);
    let token_id = neuradock_domain::token::TokenId::new(token_id);

    // Get token from cache
    let tokens = state
        .token_service
        .get_cached_tokens(&account_id)
        .await
        .map_err(|e| e.to_string())?;

    let token = tokens
        .iter()
        .find(|t| t.id() == &token_id)
        .ok_or_else(|| "Token not found".to_string())?;

    let provider_id_obj = ProviderId::from_string(&provider_id);
    let provider = state
        .provider_repo
        .find_by_id(&provider_id_obj)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Provider {} not found", provider_id))?;

    // Generate temp commands
    let commands = state
        .codex_config_service
        .generate_temp_commands(
            token,
            provider.id().as_str(),
            provider.name(),
            &base_url,
            model.as_deref(),
        )
        .map_err(|e| e.to_string())?;

    Ok(commands)
}

#[tauri::command]
#[specta::specta]
pub async fn clear_codex_global(state: State<'_, AppState>) -> Result<String, String> {
    state
        .codex_config_service
        .clear_global()
        .map_err(|e| e.to_string())
}
