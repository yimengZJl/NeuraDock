use crate::presentation::error::CommandError;
use crate::presentation::state::Services;
use tauri::State;
/// Fetch provider supported models
/// If forceRefresh is true, will fetch from API regardless of cache
/// Otherwise returns cached models if available and not stale (24 hours)
#[tauri::command]
#[specta::specta]
pub async fn fetch_provider_models(
    provider_id: String,
    account_id: String,
    force_refresh: bool,
    services: State<'_, Services>,
) -> Result<Vec<String>, CommandError> {
    log::info!(
        "fetch_provider_models: provider_id={}, account_id={}, force_refresh={}",
        provider_id,
        account_id,
        force_refresh
    );

    services
        .provider_models_query
        .fetch(provider_id, account_id, force_refresh)
        .await
        .map_err(CommandError::from)
}
