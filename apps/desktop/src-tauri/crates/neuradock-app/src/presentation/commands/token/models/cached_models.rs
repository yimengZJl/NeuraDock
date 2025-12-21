use crate::presentation::error::CommandError;
use tauri::State;

use crate::presentation::state::Services;
#[tauri::command]
#[specta::specta]
pub async fn get_cached_provider_models(
    provider_id: String,
    services: State<'_, Services>,
) -> Result<Vec<String>, CommandError> {
    log::info!("get_cached_provider_models: provider_id={}", provider_id);

    services
        .provider_models_query
        .get_cached(&provider_id)
        .await
        .map_err(CommandError::from)
}
