use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::shared::DomainError;

/// Notification message to be sent
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NotificationMessage {
    /// Message title
    pub title: String,
    /// Message content/body
    pub content: String,
    /// Optional link URL
    pub link: Option<String>,
}

impl NotificationMessage {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            link: None,
        }
    }

    pub fn with_link(mut self, link: impl Into<String>) -> Self {
        self.link = Some(link.into());
        self
    }
}

/// Notification sender trait (Strategy pattern)
/// Each notification channel type implements this trait
#[async_trait]
pub trait NotificationSender: Send + Sync {
    /// Send a notification message
    async fn send(&self, message: &NotificationMessage) -> Result<(), DomainError>;

    /// Test the notification channel connectivity
    async fn test(&self) -> Result<(), DomainError>;
}
