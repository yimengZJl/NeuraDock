use crate::presentation::error::CommandError;
use crate::presentation::state::Services;
use tauri::State;
/// Refresh provider models with WAF bypass
/// This command will:
/// 1. Check for cached WAF cookies (valid for 24 hours)
/// 2. If cached cookies are expired or missing, run WAF bypass
/// 3. Use cookies to fetch model list
/// 4. Save models and WAF cookies to database
#[tauri::command]
#[specta::specta]
pub async fn refresh_provider_models_with_waf(
    provider_id: String,
    account_id: String,
    services: State<'_, Services>,
) -> Result<Vec<String>, CommandError> {
    log::info!(
        "refresh_provider_models_with_waf: provider_id={}, account_id={}",
        provider_id,
        account_id
    );
    services
        .provider_models_query
        .refresh_with_waf(provider_id, account_id)
        .await
        .map_err(CommandError::from)
}
