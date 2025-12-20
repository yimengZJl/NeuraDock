use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;

use crate::application::commands::handlers::*;
use crate::application::queries::{AccountQueryService, CheckInStreakQueries};
use crate::application::services::{
    AutoCheckInScheduler, ClaudeConfigService, CodexConfigService, ConfigService,
    NotificationService, TokenService,
};
use neuradock_domain::account::AccountRepository;
use neuradock_domain::check_in::{Provider, ProviderRepository};
use neuradock_domain::custom_node::CustomProviderNodeRepository;
use neuradock_domain::events::EventBus;
use neuradock_domain::independent_key::IndependentKeyRepository;
use neuradock_domain::notification::NotificationChannelRepository;
use neuradock_domain::session::SessionRepository;
use neuradock_domain::shared::DomainError;
use neuradock_domain::token::TokenRepository;
use neuradock_infrastructure::persistence::{
    repositories::SqliteProviderModelsRepository, repositories::SqliteWafCookiesRepository,
    Database,
};
use neuradock_infrastructure::security::EncryptionService;

/// Command handlers container
pub struct CommandHandlers {
    pub create_account: Arc<CreateAccountCommandHandler>,
    pub update_account: Arc<UpdateAccountCommandHandler>,
    pub delete_account: Arc<DeleteAccountCommandHandler>,
    pub toggle_account: Arc<ToggleAccountCommandHandler>,
    pub execute_check_in: Arc<ExecuteCheckInCommandHandler>,
    pub batch_execute_check_in: Arc<BatchExecuteCheckInCommandHandler>,
    pub create_notification_channel: Arc<CreateNotificationChannelHandler>,
    pub update_notification_channel: Arc<UpdateNotificationChannelHandler>,
    pub delete_notification_channel: Arc<DeleteNotificationChannelHandler>,
    pub test_notification_channel: Arc<TestNotificationChannelHandler>,
    pub create_provider: Arc<CreateProviderCommandHandler>,
    pub update_provider: Arc<UpdateProviderCommandHandler>,
    pub delete_provider: Arc<DeleteProviderCommandHandler>,
}

pub struct Runtime {
    pub pool: Arc<SqlitePool>,
    pub db: Arc<Database>,
    pub encryption_service: Arc<EncryptionService>,
    pub event_bus: Arc<dyn EventBus>,
    pub app_handle: tauri::AppHandle,
}

pub struct Repositories {
    pub account: Arc<dyn AccountRepository>,
    pub session: Arc<dyn SessionRepository>,
    pub notification_channel: Arc<dyn NotificationChannelRepository>,
    pub token: Arc<dyn TokenRepository>,
    pub custom_node: Arc<dyn CustomProviderNodeRepository>,
    pub independent_key: Arc<dyn IndependentKeyRepository>,
    pub provider: Arc<dyn ProviderRepository>,
    pub provider_models: Arc<SqliteProviderModelsRepository>,
    pub waf_cookies: Arc<SqliteWafCookiesRepository>,
}

pub struct Services {
    pub notification: Arc<NotificationService>,
    pub token: Arc<TokenService>,
    pub claude_config: Arc<ClaudeConfigService>,
    pub codex_config: Arc<CodexConfigService>,
    pub scheduler: Arc<AutoCheckInScheduler>,
    pub config: Arc<ConfigService>,
}

pub struct Queries {
    pub account: Arc<AccountQueryService>,
    pub streak: Arc<CheckInStreakQueries>,
}

pub struct AppState {
    pub runtime: Runtime,
    pub repositories: Repositories,
    pub services: Services,
    pub queries: Queries,
    pub command_handlers: CommandHandlers,
}

impl AppState {
    pub async fn new(app_handle: tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        crate::presentation::bootstrap::build_app_state(app_handle).await
    }

    pub async fn provider_map(&self) -> Result<HashMap<String, Provider>, DomainError> {
        let providers = self.repositories.provider.find_all().await?;
        Ok(providers
            .into_iter()
            .map(|provider| (provider.id().as_str().to_string(), provider))
            .collect())
    }
}
