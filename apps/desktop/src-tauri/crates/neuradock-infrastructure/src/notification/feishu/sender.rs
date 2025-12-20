use async_trait::async_trait;

use neuradock_domain::notification::{NotificationMessage, NotificationSender};
use neuradock_domain::shared::DomainError;

#[async_trait]
impl NotificationSender for super::FeishuWebhookSender {
    async fn send(&self, message: &NotificationMessage) -> Result<(), DomainError> {
        let url = self.build_webhook_url();
        let payload = self.build_rich_message(message);

        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                DomainError::Infrastructure(format!("Failed to send Feishu notification: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(DomainError::Infrastructure(format!(
                "Feishu webhook failed with status {}: {}",
                status, body
            )));
        }

        // Parse response to check for errors
        let resp_body: serde_json::Value = response.json().await.map_err(|e| {
            DomainError::Infrastructure(format!("Failed to parse Feishu response: {}", e))
        })?;

        // Feishu returns {"code":0} for success
        if let Some(code) = resp_body.get("code").and_then(|c| c.as_i64()) {
            if code != 0 {
                let msg = resp_body
                    .get("msg")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error");
                return Err(DomainError::Infrastructure(format!(
                    "Feishu webhook error code {}: {}",
                    code, msg
                )));
            }
        }

        Ok(())
    }

    async fn test(&self) -> Result<(), DomainError> {
        let test_message = NotificationMessage::new(
            "测试通知",
            "这是一条来自 NeuraDock 的测试通知，如果您收到此消息，说明通知渠道配置成功！",
        );

        self.send(&test_message).await
    }
}
