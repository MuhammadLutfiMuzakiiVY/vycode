// VyCode - Configuration Module
pub mod encryption;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::providers::ProviderType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub provider_type: Option<ProviderType>,
    pub api_key_encrypted: Option<String>,
    pub model: Option<String>,
    pub custom_base_url: Option<String>,
    pub theme: ThemeConfig,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub max_context_tokens: usize,
    pub auto_scan: bool,
    pub sessions_dir: Option<String>,
    pub telegram_token: Option<String>,    // Secure Encrypted Telegram Bot Token
    pub telegram_chat_id: Option<String>,  // Remote Telegram Receiver ID
    pub discord_webhook: Option<String>,   // Discord Webhook URL Integration
    pub omni: OmniMessagingConfig,         // Omni-Channel Global Network
    pub infra: InfraConfig,                // Enterprise Infrastructure Automations
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub accent_color: String,
    pub dark_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OmniMessagingConfig {
    // Slack & Teams Webhooks
    pub slack_webhook: Option<String>,
    pub teams_webhook: Option<String>,
    
    // Matrix credentials
    pub matrix_homeserver: Option<String>,
    pub matrix_room_id: Option<String>,
    pub matrix_access_token: Option<String>,
    
    // Signal REST CLI APIs
    pub signal_api_url: Option<String>,
    pub signal_recipient: Option<String>,
    
    // WhatsApp Gateways
    pub whatsapp_api_url: Option<String>,
    pub whatsapp_token: Option<String>,
    
    // Email & SMS Gateways
    pub email_gateway_url: Option<String>,
    pub email_recipient: Option<String>,
    pub sms_gateway_url: Option<String>,
    pub sms_recipient: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfraConfig {
    pub github_token: Option<String>,
    pub github_repo: Option<String>,
    pub ssh_host: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_port: Option<String>,
    pub hass_url: Option<String>,
    pub hass_token: Option<String>,
    pub db_engine: Option<String>,
    pub db_uri: Option<String>,
    pub mcp_server_url: Option<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            accent_color: "orange".to_string(),
            dark_mode: true,
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            provider_type: None,
            api_key_encrypted: None,
            model: None,
            custom_base_url: None,
            theme: ThemeConfig::default(),
            max_retries: 3,
            retry_delay_ms: 1000,
            max_context_tokens: 8192,
            auto_scan: true,
            sessions_dir: None,
            telegram_token: None,
            telegram_chat_id: None,
            discord_webhook: None,
            omni: OmniMessagingConfig::default(),
            infra: InfraConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vycode");
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: AppConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        self.provider_type.is_some() && self.model.is_some()
            && (self.api_key_encrypted.is_some()
                || matches!(self.provider_type, Some(ProviderType::Ollama)))
    }

    pub fn set_api_key(&mut self, key: &str) {
        self.api_key_encrypted = Some(encryption::encrypt(key));
    }

    pub fn get_api_key(&self) -> Option<String> {
        self.api_key_encrypted
            .as_ref()
            .and_then(|k| encryption::decrypt(k).ok())
    }

    pub fn set_provider(&mut self, provider: ProviderType) {
        self.provider_type = Some(provider);
    }

    pub fn set_model(&mut self, model: &str) {
        self.model = Some(model.to_string());
    }

    pub fn set_custom_base_url(&mut self, url: &str) {
        self.custom_base_url = Some(url.to_string());
    }
}
