// VyCode - Telegram Direct Messaging Engine
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Broadcasts a live status or AI response string securely to a remote Telegram Client
pub async fn send_message(token: &str, chat_id: &str, message: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);
    
    let payload = json!({
        "chat_id": chat_id,
        "text": message,
        "parse_mode": "Markdown"
    });

    let response = client.post(&url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(format!("📡 [TELEGRAM] Message successfully broadcast to Chat: {}", chat_id))
    } else {
        let err_body = response.text().await.unwrap_or_else(|_| "Unknown Error".to_string());
        Err(anyhow::anyhow!("Telegram API Failure: HTTP {} - {}", err_body, chat_id))
    }
}
