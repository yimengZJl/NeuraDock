use async_trait::async_trait;

use super::aggregate::NotificationChannel;
use super::value_objects::NotificationChannelId;
use crate::shared::DomainError;

/// Notification channel repository trait
#[async_trait]
pub trait NotificationChannelRepository: Send + Sync {
    /// Save a notification channel
    async fn save(&self, channel: &NotificationChannel) -> Result<(), DomainError>;

    /// Find a notification channel by ID
    async fn find_by_id(
        &self,
        id: &NotificationChannelId,
    ) -> Result<Option<NotificationChannel>, DomainError>;

    /// Find all notification channels
    async fn find_all(&self) -> Result<Vec<NotificationChannel>, DomainError>;

    /// Find all enabled notification channels
    async fn find_all_enabled(&self) -> Result<Vec<NotificationChannel>, DomainError>;

    /// Update a notification channel
    async fn update(&self, channel: &NotificationChannel) -> Result<(), DomainError>;

    /// Delete a notification channel by ID
    async fn delete(&self, id: &NotificationChannelId) -> Result<(), DomainError>;
}
