use serde::{Deserialize, Serialize};
use specta::Type;

use crate::application::commands::command_handler::Command;
use crate::application::dtos::{
    CreateNotificationChannelInput, NotificationChannelDto, UpdateNotificationChannelInput,
};

// ============================================================
// Create Notification Channel Command
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct CreateNotificationChannelCommand {
    pub input: CreateNotificationChannelInput,
}

impl Command for CreateNotificationChannelCommand {}

pub type CreateNotificationChannelResult = NotificationChannelDto;

// ============================================================
// Update Notification Channel Command
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct UpdateNotificationChannelCommand {
    pub input: UpdateNotificationChannelInput,
}

impl Command for UpdateNotificationChannelCommand {}

pub type UpdateNotificationChannelResult = NotificationChannelDto;

// ============================================================
// Delete Notification Channel Command
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct DeleteNotificationChannelCommand {
    pub channel_id: String,
}

impl Command for DeleteNotificationChannelCommand {}

pub type DeleteNotificationChannelResult = ();

// ============================================================
// Test Notification Channel Command
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TestNotificationChannelCommand {
    pub channel_id: String,
}

impl Command for TestNotificationChannelCommand {}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct TestNotificationChannelResult {
    pub success: bool,
    pub message: String,
}
