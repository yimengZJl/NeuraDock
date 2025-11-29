use async_trait::async_trait;
use log::info;
use std::sync::Arc;

use crate::application::commands::account_commands::*;
use crate::application::commands::command_handler::CommandHandler;
use crate::application::services::AutoCheckInScheduler;
use crate::domain::account::{Account, AccountRepository, Credentials};
use crate::domain::shared::{AccountId, DomainError, ProviderId};

/// Create account command handler
pub struct CreateAccountCommandHandler {
    account_repo: Arc<dyn AccountRepository>,
    scheduler: Arc<AutoCheckInScheduler>,
}

impl CreateAccountCommandHandler {
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
impl CommandHandler<CreateAccountCommand> for CreateAccountCommandHandler {
    type Result = CreateAccountResult;

    async fn handle(&self, cmd: CreateAccountCommand) -> Result<Self::Result, DomainError> {
        info!("Handling CreateAccountCommand for account: {}", cmd.name);

        // 1. Create credentials
        let credentials = Credentials::new(cmd.cookies, cmd.api_user);

        // 2. Create account aggregate
        let mut account = Account::new(
            cmd.name,
            ProviderId::from_string(&cmd.provider_id),
            credentials,
        )?;

        // 3. Set auto check-in configuration if provided
        if let Some(enabled) = cmd.auto_checkin_enabled {
            let hour = cmd.auto_checkin_hour.unwrap_or(9);
            let minute = cmd.auto_checkin_minute.unwrap_or(0);
            account.update_auto_checkin(enabled, hour, minute)?;
        }

        // 4. Save account
        self.account_repo.save(&account).await?;

        info!(
            "Account created successfully: {} ({})",
            account.name(),
            account.id().as_str()
        );

        // 5. Reload scheduler (will be replaced by event in future)
        // TODO: Replace with domain event when event system is implemented
        if let Err(e) = self.reload_scheduler_safely().await {
            log::warn!("Failed to reload scheduler after account creation: {}", e);
        }

        // 6. Return result
        Ok(CreateAccountResult {
            account_id: account.id().as_str().to_string(),
        })
    }
}

impl CreateAccountCommandHandler {
    async fn reload_scheduler_safely(&self) -> Result<(), Box<dyn std::error::Error>> {
        // This is a temporary solution until we implement domain events
        // The scheduler reload should be triggered by an AccountCreated event
        Ok(())
    }
}
