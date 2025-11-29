use async_trait::async_trait;
use log::info;
use std::sync::Arc;

use crate::application::commands::account_commands::*;
use crate::application::commands::command_handler::CommandHandler;
use crate::application::services::AutoCheckInScheduler;
use crate::domain::account::AccountRepository;
use crate::domain::shared::{AccountId, DomainError};

/// Delete account command handler
pub struct DeleteAccountCommandHandler {
    account_repo: Arc<dyn AccountRepository>,
    scheduler: Arc<AutoCheckInScheduler>,
}

impl DeleteAccountCommandHandler {
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
impl CommandHandler<DeleteAccountCommand> for DeleteAccountCommandHandler {
    type Result = DeleteAccountResult;

    async fn handle(&self, cmd: DeleteAccountCommand) -> Result<Self::Result, DomainError> {
        info!("Handling DeleteAccountCommand for account: {}", cmd.account_id);

        let account_id = AccountId::from_string(&cmd.account_id);

        // 1. Verify account exists
        let account = self
            .account_repo
            .find_by_id(&account_id)
            .await?
            .ok_or_else(|| DomainError::AccountNotFound(cmd.account_id.clone()))?;

        info!("Deleting account: {}", account.name());

        // 2. Delete account
        self.account_repo.delete(&account_id).await?;

        info!("Account deleted successfully: {}", account.name());

        // 3. Reload scheduler (will be replaced by event in future)
        if let Err(e) = self.reload_scheduler_safely().await {
            log::warn!("Failed to reload scheduler after account deletion: {}", e);
        }

        Ok(DeleteAccountResult { success: true })
    }
}

impl DeleteAccountCommandHandler {
    async fn reload_scheduler_safely(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Replace with domain event
        Ok(())
    }
}
