use async_trait::async_trait;
use log::info;
use std::sync::Arc;

use crate::application::commands::account_commands::*;
use crate::application::commands::command_handler::CommandHandler;
use crate::application::services::AutoCheckInScheduler;
use crate::domain::account::AccountRepository;
use crate::domain::shared::{AccountId, DomainError};

/// Toggle account command handler
pub struct ToggleAccountCommandHandler {
    account_repo: Arc<dyn AccountRepository>,
    scheduler: Arc<AutoCheckInScheduler>,
}

impl ToggleAccountCommandHandler {
    pub fn new(
        account_repo: Arc<dyn AccountRepository>,
        scheduler: Arc<AutoCheckInScheduler>,
    ) -> Self {
        Self {
            account_repo,
            scheduler,
        }
    }
}

#[async_trait]
impl CommandHandler<ToggleAccountCommand> for ToggleAccountCommandHandler {
    type Result = ToggleAccountResult;

    async fn handle(&self, cmd: ToggleAccountCommand) -> Result<Self::Result, DomainError> {
        info!(
            "Handling ToggleAccountCommand for account: {} (enabled: {})",
            cmd.account_id, cmd.enabled
        );

        let account_id = AccountId::from_string(&cmd.account_id);

        // 1. Load account aggregate
        let mut account = self
            .account_repo
            .find_by_id(&account_id)
            .await?
            .ok_or_else(|| DomainError::AccountNotFound(cmd.account_id.clone()))?;

        // 2. Toggle account
        account.toggle(cmd.enabled);

        // 3. Save updated account
        self.account_repo.save(&account).await?;

        info!(
            "Account {} {}",
            account.name(),
            if cmd.enabled { "enabled" } else { "disabled" }
        );

        // 4. Reload scheduler (will be replaced by event in future)
        if let Err(e) = self.reload_scheduler_safely().await {
            log::warn!("Failed to reload scheduler after account toggle: {}", e);
        }

        Ok(ToggleAccountResult { success: true })
    }
}

impl ToggleAccountCommandHandler {
    async fn reload_scheduler_safely(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Replace with domain event
        Ok(())
    }
}
