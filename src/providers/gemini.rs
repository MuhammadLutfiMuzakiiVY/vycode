// VyCode - Google Gemini Provider
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;

use super::streaming::parse_gemini_sse;
use super::{AiProvider, ChatMessage, MessageRole, StreamEvent};
use crate::config::AppConfig;

pub struct GeminiProvider;

#[async_trait]
impl AiProvider for GeminiProvider {
    fn name(&self) -> &str {
        "Google Gemini"
    }

    async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        config: &AppConfig,
        tx: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<()> {
        let api_key = config.get_api_key().unwrap_or_default();
        let model = config.model.as_deref().unwrap_or("gemini-2.5-flash");
        let client = reqwest::Client::new();

        // Convert messages to Gemini format
        let contents: Vec<serde_json::Value> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| {
                serde_json::json!({
                    "role": match m.role {
                        MessageRole::User => "user",
                        MessageRole::Assistant => "model",
                        _ => "user",
                    },
                    "parts": [{ "text": m.content }],
                })
            })
            .collect();

        let system_instruction = messages
            .iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| {
                serde_json::json!({
                    "parts": [{ "text": m.content }]
                })
            });

        let mut body = serde_json::json!({
            "contents": contents,
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 4096,
            },
        });

        if let Some(sys) = system_instruction {
            body["systemInstruction"] = sys;
        }

        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{model}:streamGenerateContent?alt=sse&key={api_key}"
        );

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
                "Gemini API error ({status}): {error_body}"
            )));
            return Ok(());
        }

        parse_gemini_sse(response, tx).await
    }
}
