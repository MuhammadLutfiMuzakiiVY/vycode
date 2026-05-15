// VyCode - Ollama Provider (Local)
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::streaming::parse_ollama_stream;
use super::{AiProvider, ChatMessage, StreamEvent};
use crate::config::AppConfig;

pub struct OllamaProvider;

#[async_trait]
impl AiProvider for OllamaProvider {
    fn name(&self) -> &str {
        "Ollama"
    }

    async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        config: &AppConfig,
        tx: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<()> {
        let model = config.model.as_deref().unwrap_or("llama3");
        let base_url = config
            .custom_base_url
            .as_deref()
            .unwrap_or("http://localhost:11434");
        let client = reqwest::Client::new();

        let ollama_messages: Vec<serde_json::Value> = messages
            .iter()
            .map(|m| {
                serde_json::json!({
                    "role": match m.role {
                        super::MessageRole::System => "system",
                        super::MessageRole::User => "user",
                        super::MessageRole::Assistant => "assistant",
                    },
                    "content": m.content,
                })
            })
            .collect();

        let body = serde_json::json!({
            "model": model,
            "messages": ollama_messages,
            "stream": true,
        });

        let url = format!("{base_url}/api/chat");

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            let _ = tx.send(StreamEvent::Error(format!(
                "Ollama error ({status}): {error_body}"
            )));
            return Ok(());
        }

        parse_ollama_stream(response, tx).await
    }
}
