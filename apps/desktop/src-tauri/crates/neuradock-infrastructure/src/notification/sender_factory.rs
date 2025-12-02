use std::sync::Arc;

use neuradock_domain::notification::{ChannelConfig, NotificationSender};
use neuradock_domain::shared::DomainError;

use super::feishu::FeishuWebhookSender;

/// Create a notification sender based on channel configuration
pub fn create_sender(config: &ChannelConfig) -> Result<Arc<dyn NotificationSender>, DomainError> {
    match config {
        ChannelConfig::Feishu { webhook_key } => {
            Ok(Arc::new(FeishuWebhookSender::new(webhook_key.clone())))
        }
        ChannelConfig::DingTalk { .. } => Err(DomainError::NotImplemented(
            "DingTalk notification not implemented yet".to_string(),
        )),
        ChannelConfig::Email { .. } => Err(DomainError::NotImplemented(
            "Email notification not implemented yet".to_string(),
        )),
    }
}
