// VyCode - OpenRouter Provider
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::streaming::parse_openai_sse;
use super::{AiProvider, ChatMessage, StreamEvent};
use crate::config::AppConfig;

pub struct OpenRouterProvider;

#[async_trait]
impl AiProvider for OpenRouterProvider {
    fn name(&self) -> &str {
        "OpenRouter"
    }

    async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        config: &AppConfig,
        tx: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<()> {
        let api_key = config.get_api_key().unwrap_or_default();
        let model = config.model.as_deref().unwrap_or("openai/gpt-4o");
        let client = reqwest::Client::new();

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
            "temperature": 0.7,
        });

        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .header("Authorization", format!("Bearer {api_key}"))
            .header("HTTP-Referer", "https://github.com/MuhammadLutfiMuzakiiVY/vycode")
            .header("X-Title", "VyCode")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let _ = tx.send(StreamEvent::Error(format!(
                "OpenRouter API error ({status}): {error_body}"
            )));
            return Ok(());
        }

        parse_openai_sse(response, tx).await
    }
}
