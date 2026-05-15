// VyCode - Omni-Channel Advanced Global Messaging & Broadcast Engine
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Core network dispatcher helper for rapid webhook firing
async fn dispatch_json(url: &str, payload: serde_json::Value, channel_name: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    let response = client.post(url)
        .json(&payload)
        .send()
        .await?;

    let status = response.status();
    if status.is_success() || status == reqwest::StatusCode::NO_CONTENT {
        Ok(format!("🌐 [OMNI:{}] Successfully broadcasted event!", channel_name.to_uppercase()))
    } else {
        let err = response.text().await.unwrap_or_else(|_| "Unknown Dispatch Error".to_string());
        Err(anyhow::anyhow!("{} failure: HTTP {} - {}", channel_name, status, err))
    }
}

/// 🟢 SLACK Webhook Dispatcher
pub async fn send_slack(webhook: &str, message: &str) -> Result<String> {
    let payload = json!({ "text": format!("🤖 *VyCode Event:* {}", message) });
    dispatch_json(webhook, payload, "Slack").await
}

/// 🟢 MS TEAMS Webhook Dispatcher
pub async fn send_teams(webhook: &str, message: &str) -> Result<String> {
    let payload = json!({ "text": format!("🚀 **VyCode Event:** {}", message) });
    dispatch_json(webhook, payload, "Teams").await
}

/// 🟢 MATRIX Messaging Dispatcher (Client-Server HTTP Matrix API)
pub async fn send_matrix(homeserver: &str, room_id: &str, token: &str, message: &str) -> Result<String> {
    let url = format!("{}/_matrix/client/r0/rooms/{}/send/m.room.message?access_token={}", homeserver, room_id, token);
    let payload = json!({
        "msgtype": "m.text",
        "body": format!("VyCode Event: {}", message)
    });
    dispatch_json(&url, payload, "Matrix").await
}

/// 🟢 SIGNAL REST Gateway Dispatcher
pub async fn send_signal(url: &str, recipient: &str, message: &str) -> Result<String> {
    let payload = json!({
        "message": format!("🤖 VyCode Event: {}", message),
        "number": recipient,
        "recipients": vec![recipient]
    });
    dispatch_json(url, payload, "Signal").await
}

/// 🟢 WHATSAPP Gateway/Twilio Dispatcher
pub async fn send_whatsapp(url: &str, token: &str, message: &str) -> Result<String> {
    let payload = json!({
        "message": format!("🤖 *VyCode AI Event:*\n\n{}", message),
        "token": token
    });
    dispatch_json(url, payload, "WhatsApp").await
}

/// 🟢 EMAIL REST Gateway/SMTP Wrapper Dispatcher
pub async fn send_email(gateway_url: &str, recipient: &str, message: &str) -> Result<String> {
    let payload = json!({
        "to": recipient,
        "subject": "VyCode Autonomous Event Report",
        "text": format!("VyCode has executed an autonomous agent loop event:\n\n{}", message)
    });
    dispatch_json(gateway_url, payload, "Email").await
}

/// 🟢 SMS Carrier/Twilio Gateway Dispatcher
pub async fn send_sms(gateway_url: &str, recipient: &str, message: &str) -> Result<String> {
    let payload = json!({
        "to": recipient,
        "body": format!("🤖 VyCode Status: {}", message)
    });
    dispatch_json(gateway_url, payload, "SMS").await
}
