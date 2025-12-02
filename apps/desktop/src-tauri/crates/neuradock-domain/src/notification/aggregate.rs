use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

use super::value_objects::{ChannelConfig, ChannelType, NotificationChannelId};
use crate::shared::DomainError;

/// NotificationChannel aggregate root
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NotificationChannel {
    id: NotificationChannelId,
    channel_type: ChannelType,
    config: ChannelConfig,
    enabled: bool,
    created_at: DateTime<Utc>,
}

impl NotificationChannel {
    /// Create a new notification channel
    pub fn new(config: ChannelConfig) -> Result<Self, DomainError> {
        // Validate configuration
        config.validate()?;

        let channel_type = config.channel_type();

        Ok(Self {
            id: NotificationChannelId::new(),
            channel_type,
            config,
            enabled: true,
            created_at: Utc::now(),
        })
    }

    /// Reconstruct from persistence
    pub fn from_persistence(
        id: NotificationChannelId,
        channel_type: ChannelType,
        config: ChannelConfig,
        enabled: bool,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            channel_type,
            config,
            enabled,
            created_at,
        }
    }

    // Getters
    pub fn id(&self) -> &NotificationChannelId {
        &self.id
    }

    pub fn channel_type(&self) -> &ChannelType {
        &self.channel_type
    }

    pub fn config(&self) -> &ChannelConfig {
        &self.config
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    // Business methods

    /// Enable the notification channel
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the notification channel
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Update configuration
    pub fn update_config(&mut self, new_config: ChannelConfig) -> Result<(), DomainError> {
        // Validate new configuration
        new_config.validate()?;

        // Ensure channel type matches
        if new_config.channel_type() != self.channel_type {
            return Err(DomainError::InvalidInput(
                "Cannot change channel type after creation".to_string(),
            ));
        }

        self.config = new_config;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_feishu_channel() {
        let config = ChannelConfig::Feishu {
            webhook_key: "test_key_123".to_string(),
        };

        let channel = NotificationChannel::new(config).unwrap();
        assert_eq!(channel.channel_type(), &ChannelType::Feishu);
        assert!(channel.is_enabled());
    }

    #[test]
    fn test_invalid_config() {
        let config = ChannelConfig::Feishu {
            webhook_key: "".to_string(),
        };

        let result = NotificationChannel::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_disable() {
        let config = ChannelConfig::Feishu {
            webhook_key: "test_key_123".to_string(),
        };

        let mut channel = NotificationChannel::new(config).unwrap();
        assert!(channel.is_enabled());

        channel.disable();
        assert!(!channel.is_enabled());

        channel.enable();
        assert!(channel.is_enabled());
    }

    #[test]
    fn test_cannot_change_channel_type() {
        let config = ChannelConfig::Feishu {
            webhook_key: "test_key_123".to_string(),
        };

        let mut channel = NotificationChannel::new(config).unwrap();

        let new_config = ChannelConfig::DingTalk {
            webhook_key: "test_key_456".to_string(),
            secret: None,
        };

        let result = channel.update_config(new_config);
        assert!(result.is_err());
    }
}
