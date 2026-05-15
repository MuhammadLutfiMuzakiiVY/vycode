// VyCode - Discord Webhook Direct Publisher
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Broadcasts live statuses, system alerts, or AI responses securely to a remote Discord Channel
pub async fn send_webhook(webhook_url: &str, message: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let payload = json!({
        "content": format!("🤖 **VyCode Remote Agent Event:**\n━━━━━━━━━━━━━━━━━━━━━━━\n{}", message)
    });

    let response = client.post(webhook_url)
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    if status.is_success() || status == reqwest::StatusCode::NO_CONTENT {
        Ok("🎮 [DISCORD] Webhook message successfully dispatched to channel!".to_string())
    } else {
        let err_body = response.text().await.unwrap_or_else(|_| "Unknown Discord Error".to_string());
        Err(anyhow::anyhow!("Discord API Failure: HTTP {} - {}", status, err_body))
    }
}
