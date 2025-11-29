use async_trait::async_trait;
use log::info;
use std::sync::Arc;

use crate::application::commands::account_commands::*;
use crate::application::commands::command_handler::CommandHandler;
use crate::application::services::AutoCheckInScheduler;
use crate::domain::account::{AccountRepository, Credentials};
use crate::domain::shared::{AccountId, DomainError};

/// Update account command handler
pub struct UpdateAccountCommandHandler {
    account_repo: Arc<dyn AccountRepository>,
    scheduler: Arc<AutoCheckInScheduler>,
}

impl UpdateAccountCommandHandler {
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
impl CommandHandler<UpdateAccountCommand> for UpdateAccountCommandHandler {
    type Result = UpdateAccountResult;

    async fn handle(&self, cmd: UpdateAccountCommand) -> Result<Self::Result, DomainError> {
        info!("Handling UpdateAccountCommand for account: {}", cmd.account_id);

        let account_id = AccountId::from_string(&cmd.account_id);

        // 1. Load account aggregate
        let mut account = self
            .account_repo
            .find_by_id(&account_id)
            .await?
            .ok_or_else(|| DomainError::AccountNotFound(cmd.account_id.clone()))?;

        // 2. Update name if provided
        if let Some(name) = cmd.name {
            account.update_name(name)?;
        }

        // 3. Update credentials if provided
        if let Some(cookies) = cmd.cookies {
            let api_user = cmd.api_user.unwrap_or_else(|| {
                account.credentials().api_user().to_string()
            });
            let credentials = Credentials::new(cookies, api_user);
            account.update_credentials(credentials)?;
        }

        // 4. Update auto check-in configuration if provided
        if let Some(enabled) = cmd.auto_checkin_enabled {
            let hour = cmd.auto_checkin_hour.unwrap_or(account.auto_checkin_hour());
            let minute = cmd.auto_checkin_minute.unwrap_or(account.auto_checkin_minute());
            account.update_auto_checkin(enabled, hour, minute)?;
        }

        // 5. Save updated account
        self.account_repo.save(&account).await?;

        info!("Account updated successfully: {}", account.name());

        // 6. Reload scheduler (will be replaced by event in future)
        if let Err(e) = self.reload_scheduler_safely().await {
            log::warn!("Failed to reload scheduler after account update: {}", e);
        }

        Ok(UpdateAccountResult { success: true })
    }
}

impl UpdateAccountCommandHandler {
    async fn reload_scheduler_safely(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Replace with domain event
        Ok(())
    }
}
