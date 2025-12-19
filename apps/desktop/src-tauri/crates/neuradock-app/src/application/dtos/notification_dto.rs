use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct NotificationChannelDto {
    pub id: String,
    pub channel_type: String,
    #[specta(type = String)]
    pub config: serde_json::Value,
    pub enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CreateNotificationChannelInput {
    pub channel_type: String,
    #[specta(type = String)]
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UpdateNotificationChannelInput {
    pub channel_id: String,
    #[specta(type = String)]
    pub config: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}
