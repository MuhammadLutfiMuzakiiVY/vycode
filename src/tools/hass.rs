// VyCode - IoT Smart Home Assistant Enterprise API Bridge
use anyhow::Result;
use reqwest::Client;
use serde_json::json;
use std::time::Duration;

/// Communicates securely with a Home Assistant API endpoint for automated IoT controls
pub async fn control_hass(url: &str, token: &str, operation: &str, entity_id: &str) -> Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let api_url = format!("{}/api", url.trim_end_matches('/'));
    let response = match operation {
        "states" => {
            // List states or single entity state
            let target = if entity_id.is_empty() { format!("{}/states", api_url) } else { format!("{}/states/{}", api_url, entity_id) };
            client.get(&target)
                .bearer_auth(token)
                .send()
                .await?
        }
        "turn_on" | "turn_off" | "toggle" => {
            // Standard switch/light service callbacks
            if entity_id.is_empty() { return Err(anyhow::anyhow!("HASS operations require target `entity_id`!")); }
            let domain = entity_id.split('.').next().unwrap_or("homeassistant");
            let service_url = format!("{}/services/{}/{}", api_url, domain, operation);
            let payload = json!({ "entity_id": entity_id });
            
            client.post(&service_url)
                .bearer_auth(token)
                .json(&payload)
                .send()
                .await?
        }
        _ => return Err(anyhow::anyhow!("Unsupported Home Assistant command `{}`", operation))
    };

    let status = response.status();
    let txt = response.text().await.unwrap_or_default();

    if status.is_success() {
        Ok(format!("🏡 [HASS {}]: API Dispatch Success!\n{}", operation, txt))
    } else {
        Err(anyhow::anyhow!("HASS API failure (HTTP {}): {}", status, txt))
    }
}
