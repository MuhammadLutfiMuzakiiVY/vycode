// VyCode - SSE Streaming Helpers
use anyhow::Result;
use futures::StreamExt;
use reqwest::Response;
use tokio::sync::mpsc;

use super::StreamEvent;

/// Parse an SSE stream from an HTTP response (OpenAI-compatible format)
/// Extracts content deltas from `data: {"choices":[{"delta":{"content":"..."}}]}`
pub async fn parse_openai_sse(
    response: Response,
    tx: mpsc::UnboundedSender<StreamEvent>,
) -> Result<()> {
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));

                // Process complete SSE lines
                while let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if line.is_empty() || line.starts_with(':') {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if data.trim() == "[DONE]" {
                            let _ = tx.send(StreamEvent::Done);
                            return Ok(());
                        }

                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(content) = json["choices"]
                                .get(0)
                                .and_then(|c| c["delta"]["content"].as_str())
                            {
                                if !content.is_empty() {
                                    let _ = tx.send(StreamEvent::Chunk(content.to_string()));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(StreamEvent::Error(format!("Stream error: {e}")));
                return Ok(());
            }
        }
    }

    let _ = tx.send(StreamEvent::Done);
    Ok(())
}

/// Parse Anthropic SSE stream format
pub async fn parse_anthropic_sse(
    response: Response,
    tx: mpsc::UnboundedSender<StreamEvent>,
) -> Result<()> {
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));

                while let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if line.is_empty() || line.starts_with(':') || line.starts_with("event:") {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            let event_type = json["type"].as_str().unwrap_or("");
                            match event_type {
                                "content_block_delta" => {
                                    if let Some(text) = json["delta"]["text"].as_str() {
                                        if !text.is_empty() {
                                            let _ =
                                                tx.send(StreamEvent::Chunk(text.to_string()));
                                        }
                                    }
                                }
                                "message_stop" => {
                                    let _ = tx.send(StreamEvent::Done);
                                    return Ok(());
                                }
                                "error" => {
                                    let msg = json["error"]["message"]
                                        .as_str()
                                        .unwrap_or("Unknown error");
                                    let _ = tx.send(StreamEvent::Error(msg.to_string()));
                                    return Ok(());
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(StreamEvent::Error(format!("Stream error: {e}")));
                return Ok(());
            }
        }
    }

    let _ = tx.send(StreamEvent::Done);
    Ok(())
}

/// Parse Gemini SSE stream format
pub async fn parse_gemini_sse(
    response: Response,
    tx: mpsc::UnboundedSender<StreamEvent>,
) -> Result<()> {
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));

                while let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if line.is_empty() || line.starts_with(':') {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(text) = json["candidates"]
                                .get(0)
                                .and_then(|c| c["content"]["parts"].get(0))
                                .and_then(|p| p["text"].as_str())
                            {
                                if !text.is_empty() {
                                    let _ = tx.send(StreamEvent::Chunk(text.to_string()));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(StreamEvent::Error(format!("Stream error: {e}")));
                return Ok(());
            }
        }
    }

    let _ = tx.send(StreamEvent::Done);
    Ok(())
}

/// Parse Ollama JSON lines stream
pub async fn parse_ollama_stream(
    response: Response,
    tx: mpsc::UnboundedSender<StreamEvent>,
) -> Result<()> {
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(bytes) => {
                buffer.push_str(&String::from_utf8_lossy(&bytes));

                while let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if line.is_empty() {
                        continue;
                    }

                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
                        if json["done"].as_bool() == Some(true) {
                            let _ = tx.send(StreamEvent::Done);
                            return Ok(());
                        }
                        if let Some(content) = json["message"]["content"].as_str() {
                            if !content.is_empty() {
                                let _ = tx.send(StreamEvent::Chunk(content.to_string()));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(StreamEvent::Error(format!("Stream error: {e}")));
                return Ok(());
            }
        }
    }

    let _ = tx.send(StreamEvent::Done);
    Ok(())
}
