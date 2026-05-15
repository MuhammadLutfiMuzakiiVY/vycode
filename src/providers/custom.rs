// VyCode - Custom Endpoint Provider
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::streaming::parse_openai_sse;
use super::{AiProvider, ChatMessage, StreamEvent};
use crate::config::AppConfig;

pub struct CustomProvider;

#[async_trait]
impl AiProvider for CustomProvider {
    fn name(&self) -> &str {
        "Custom"
    }

    async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        config: &AppConfig,
        tx: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<()> {
        let api_key = config.get_api_key().unwrap_or_default();
        let model = config.model.as_deref().unwrap_or("default");
        let base_url = config
            .custom_base_url
            .as_deref()
            .unwrap_or("http://localhost:8080");
        let client = reqwest::Client::new();

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
            "temperature": 0.7,
        });

        let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json");

        if !api_key.is_empty() {
            request = request.header("Authorization", format!("Bearer {api_key}"));
        }

        let response = request.json(&body).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let _ = tx.send(StreamEvent::Error(format!(
                "Custom API error ({status}): {error_body}"
            )));
            return Ok(());
        }

        parse_openai_sse(response, tx).await
    }
}
