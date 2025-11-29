use async_trait::async_trait;
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;

use crate::application::services::AutoCheckInScheduler;
use crate::domain::account::AccountRepository;
use crate::domain::check_in::Provider;
use crate::domain::events::account_events::*;
use crate::domain::events::event_bus::EventHandler;
use crate::domain::shared::DomainError;

/// Handler for account lifecycle events that trigger scheduler reload
/// This decouples the scheduler from the command handlers
#[derive(Clone)]
pub struct SchedulerReloadEventHandler {
    scheduler: Arc<AutoCheckInScheduler>,
    account_repo: Arc<dyn AccountRepository>,
    providers: HashMap<String, Provider>,
    app_handle: tauri::AppHandle,
}

impl SchedulerReloadEventHandler {
    pub fn new(
        scheduler: Arc<AutoCheckInScheduler>,
        account_repo: Arc<dyn AccountRepository>,
        providers: HashMap<String, Provider>,
        app_handle: tauri::AppHandle,
    ) -> Self {
        Self {
            scheduler,
            account_repo,
            providers,
            app_handle,
        }
    }

    async fn reload_schedules(&self) -> Result<(), DomainError> {
        info!("Reloading scheduler schedules due to account change");
        
        self.scheduler
            .reload_schedules(
                self.providers.clone(),
                self.account_repo.clone(),
                self.app_handle.clone(),
            )
            .await
            .map_err(|e| {
                error!("Failed to reload schedules: {}", e);
                DomainError::Infrastructure(format!("Failed to reload schedules: {}", e))
            })?;
        
        info!("Scheduler schedules reloaded successfully");
        Ok(())
    }
}

#[async_trait]
impl EventHandler<AccountCreated> for SchedulerReloadEventHandler {
    async fn handle(&self, event: &AccountCreated) -> Result<(), DomainError> {
        info!(
            "Handling AccountCreated event for account: {} ({})",
            event.name, event.account_id
        );
        
        if event.auto_checkin_enabled {
            self.reload_schedules().await?;
        } else {
            info!("Auto check-in not enabled for account, skipping scheduler reload");
        }
        
        Ok(())
    }
}

#[async_trait]
impl EventHandler<AccountUpdated> for SchedulerReloadEventHandler {
    async fn handle(&self, event: &AccountUpdated) -> Result<(), DomainError> {
        info!("Handling AccountUpdated event for account: {}", event.account_id);
        
        // Reload if auto check-in config was updated
        if event.auto_checkin_config_updated {
            self.reload_schedules().await?;
        } else {
            info!("Auto check-in config not updated, skipping scheduler reload");
        }
        
        Ok(())
    }
}

#[async_trait]
impl EventHandler<AccountDeleted> for SchedulerReloadEventHandler {
    async fn handle(&self, event: &AccountDeleted) -> Result<(), DomainError> {
        info!(
            "Handling AccountDeleted event for account: {} ({})",
            event.name, event.account_id
        );
        
        self.reload_schedules().await?;
        Ok(())
    }
}

#[async_trait]
impl EventHandler<AccountToggled> for SchedulerReloadEventHandler {
    async fn handle(&self, event: &AccountToggled) -> Result<(), DomainError> {
        info!(
            "Handling AccountToggled event for account: {} (enabled: {})",
            event.account_id, event.enabled
        );
        
        self.reload_schedules().await?;
        Ok(())
    }
}
