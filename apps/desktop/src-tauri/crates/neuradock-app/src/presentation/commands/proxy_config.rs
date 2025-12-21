use tauri::State;

use crate::application::dtos::{ProxyConfigDto, UpdateProxyConfigInput};
use crate::presentation::error::CommandError;
use crate::presentation::state::Services;

/// Get current proxy configuration
#[tauri::command]
#[specta::specta]
pub async fn get_proxy_config(state: State<'_, Services>) -> Result<ProxyConfigDto, CommandError> {
    state
        .proxy_config
        .get()
        .await
        .map_err(CommandError::from)
}

/// Update proxy configuration
#[tauri::command]
#[specta::specta]
pub async fn update_proxy_config(
    input: UpdateProxyConfigInput,
    state: State<'_, Services>,
) -> Result<ProxyConfigDto, CommandError> {
    state
        .proxy_config
        .update(input)
        .await
        .map_err(CommandError::from)
}
