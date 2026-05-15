// VyCode - AI Provider Module
pub mod anthropic;
pub mod custom;
pub mod deepseek;
pub mod gemini;
pub mod groq;
pub mod ollama;
pub mod openai;
pub mod openrouter;
pub mod streaming;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::AppConfig;

/// Chat message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// A single chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn user(content: &str) -> Self {
        Self {
            role: MessageRole::User,
            content: content.to_string(),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.to_string(),
        }
    }

    pub fn system(content: &str) -> Self {
        Self {
            role: MessageRole::System,
            content: content.to_string(),
        }
    }
}

/// Supported AI provider types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    Gemini,
    OpenRouter,
    Groq,
    DeepSeek,
    Ollama,
    Custom,
}

impl ProviderType {
    pub fn all() -> Vec<ProviderType> {
        vec![
            Self::OpenRouter,
            Self::OpenAI,
            Self::Anthropic,
            Self::Gemini,
            Self::Groq,
            Self::DeepSeek,
            Self::Ollama,
            Self::Custom,
        ]
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::OpenAI => "OpenAI",
            Self::Anthropic => "Anthropic",
            Self::Gemini => "Google Gemini",
            Self::OpenRouter => "OpenRouter",
            Self::Groq => "Groq",
            Self::DeepSeek => "DeepSeek",
            Self::Ollama => "Ollama (Local)",
            Self::Custom => "Custom Endpoint",
        }
    }

    pub fn default_model(&self) -> &str {
        match self {
            Self::OpenAI => "gpt-4o",
            Self::Anthropic => "claude-sonnet-4-20250514",
            Self::Gemini => "gemini-2.5-flash",
            Self::OpenRouter => "openai/gpt-4o",
            Self::Groq => "llama-3.3-70b-versatile",
            Self::DeepSeek => "deepseek-chat",
            Self::Ollama => "llama3",
            Self::Custom => "default",
        }
    }

    pub fn needs_api_key(&self) -> bool {
        !matches!(self, Self::Ollama)
    }

    pub fn needs_base_url(&self) -> bool {
        matches!(self, Self::Custom | Self::Ollama)
    }
}

/// Stream event sent from provider to the app
#[derive(Debug)]
pub enum StreamEvent {
    Chunk(String),
    Done,
    Error(String),
}

/// Trait that all AI providers must implement
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Provider display name
    fn name(&self) -> &str;

    /// Send a streaming chat request, pushing chunks into the sender
    async fn stream_chat(
        &self,
        messages: &[ChatMessage],
        config: &AppConfig,
        tx: mpsc::UnboundedSender<StreamEvent>,
    ) -> Result<()>;
}

/// Create a provider instance from the given type and config
pub fn create_provider(provider_type: &ProviderType, _config: &AppConfig) -> Box<dyn AiProvider> {
    match provider_type {
        ProviderType::OpenAI => Box::new(openai::OpenAiProvider),
        ProviderType::Anthropic => Box::new(anthropic::AnthropicProvider),
        ProviderType::Gemini => Box::new(gemini::GeminiProvider),
        ProviderType::OpenRouter => Box::new(openrouter::OpenRouterProvider),
        ProviderType::Groq => Box::new(groq::GroqProvider),
        ProviderType::DeepSeek => Box::new(deepseek::DeepSeekProvider),
        ProviderType::Ollama => Box::new(ollama::OllamaProvider),
        ProviderType::Custom => Box::new(custom::CustomProvider),
    }
}

/// Build the system prompt for coding assistant mode
pub fn build_system_prompt(project_context: Option<&str>) -> String {
    let mut prompt = String::from(
        r#"You are VyCode, an expert AI coding assistant created by Muhammad Lutfi Muzaki.
You help developers with:
- Writing, reviewing, and debugging code
- Explaining complex concepts clearly
- Suggesting best practices and optimizations
- Generating complete implementations
- Finding and fixing bugs
- Executing shell commands when needed

Guidelines:
- Be concise but thorough
- Use code blocks with language tags
- Suggest improvements proactively
- Ask clarifying questions when needed
- Format responses in markdown
"#,
    );

    if let Some(ctx) = project_context {
        prompt.push_str("\n\nProject context:\n");
        prompt.push_str(ctx);
    }

    prompt
}
