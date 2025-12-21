use neuradock_domain::account::AccountRepository;
use neuradock_domain::check_in::Provider;
use neuradock_domain::shared::AccountId;
use std::sync::Arc;

/// Task metadata for health monitoring
#[derive(Debug, Clone)]
pub(super) struct TaskMetadata {
    pub account_name: String,
    pub last_execution: Option<chrono::DateTime<chrono::Utc>>,
}

/// Configuration for spawning a check-in task
pub(super) struct CheckInTaskConfig {
    pub account_id: AccountId,
    pub account_name: String,
    pub hour: u8,
    pub minute: u8,
    pub provider: Provider,
    pub account_repo: Arc<dyn AccountRepository>,
    pub app_handle: tauri::AppHandle,
}
