use tauri::State;
use crate::application::ResultExt;


use crate::application::dtos::TokenDto;
use crate::presentation::state::AppState;
use neuradock_domain::shared::AccountId;

#[tauri::command]
#[specta::specta]
pub async fn fetch_account_tokens(
    account_id: String,
    force_refresh: bool,
    state: State<'_, AppState>,
) -> Result<Vec<TokenDto>, String> {
    log::info!(
        "fetch_account_tokens called: account_id={}, force_refresh={}",
        account_id,
        force_refresh
    );
    let account_id = AccountId::from_string(&account_id);

    // Fetch tokens from service
    let tokens = state
        .token_service
        .fetch_and_cache_tokens(&account_id, force_refresh)
        .await
        .map_err(|e| {
            log::error!("Failed to fetch tokens: {}", e);
            e.to_string()
        })?;

    log::info!("Fetched {} tokens for account {}", tokens.len(), account_id);

    // Get account info to fill DTO
    let account = state
        .account_repo
        .find_by_id(&account_id)
        .await
        .to_string_err()?
        .ok_or_else(|| "Account not found".to_string())?;

    // Get provider info
    let provider_id = account.provider_id().as_str().to_string();
    let provider = state
        .provider_repo
        .find_by_id(account.provider_id())
        .await
        .to_string_err()?
        .ok_or_else(|| format!("Provider {} not found", provider_id))?;

    // Convert to DTOs
    let dtos = tokens
        .iter()
        .map(|token| {
            TokenDto::from_domain(
                token,
                account.name().to_string(),
                provider.name().to_string(),
            )
        })
        .collect();

    Ok(dtos)
}
