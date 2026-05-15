// VyCode - Anthropic Provider
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::streaming::parse_anthropic_sse;
use super::{AiProvider, ChatMessage, MessageRole, StreamEvent};
use crate::config::AppConfig;

pub struct AnthropicProvider;

#[async_trait]
impl AiProvider for AnthropicProvider {
    fn name(&self) -> &str {
        "Anthropic"
    }

    async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        config: &AppConfig,
        tx: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<()> {
        let api_key = config.get_api_key().unwrap_or_default();
        let model = config.model.as_deref().unwrap_or("claude-sonnet-4-20250514");
        let client = reqwest::Client::new();

        // Anthropic separates system message from messages array
        let system_msg = messages
            .iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| m.content.clone())
            .unwrap_or_default();

        let chat_messages: Vec<serde_json::Value> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| {
                serde_json::json!({
                    "role": match m.role {
                        MessageRole::User => "user",
                        MessageRole::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content,
                })
            })
            .collect();

        let body = serde_json::json!({
            "model": model,
            "system": system_msg,
            "messages": chat_messages,
            "stream": true,
            "max_tokens": 4096,
        });

        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let _ = tx.send(StreamEvent::Error(format!(
                "Anthropic API error ({status}): {error_body}"
            )));
            return Ok(());
        }

        parse_anthropic_sse(response, tx).await
    }
}
