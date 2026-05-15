// VyCode - Session Management Module
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::providers::ChatMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub messages: Vec<ChatMessage>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.messages.push(msg);
        self.updated_at = Utc::now();
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.updated_at = Utc::now();
    }

    /// Export session to markdown format
    pub fn export_markdown(&self) -> String {
        let mut md = format!("# VyCode Session: {}\n\n", self.name);
        md.push_str(&format!("Created: {}\n", self.created_at));
        md.push_str(&format!("Updated: {}\n\n", self.updated_at));
        md.push_str("---\n\n");

        for msg in &self.messages {
            match msg.role {
                crate::providers::MessageRole::User => {
                    md.push_str(&format!("## 👤 User\n\n{}\n\n", msg.content));
                }
                crate::providers::MessageRole::Assistant => {
                    md.push_str(&format!("## 🤖 VyCode\n\n{}\n\n", msg.content));
                }
                crate::providers::MessageRole::System => {
                    md.push_str(&format!("> System: {}\n\n", msg.content));
                }
            }
        }

        md
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionManager {
    pub sessions: Vec<Session>,
    pub active_index: usize,
}

impl SessionManager {
    pub fn sessions_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vycode")
            .join("sessions");
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub fn load() -> Result<Self> {
        let dir = Self::sessions_dir()?;
        let index_path = dir.join("index.json");

        if index_path.exists() {
            let content = fs::read_to_string(&index_path)?;
            let manager: SessionManager = serde_json::from_str(&content)?;
            Ok(manager)
        } else {
            let mut manager = Self {
                sessions: Vec::new(),
                active_index: 0,
            };
            manager.sessions.push(Session::new("Default"));
            manager.save()?;
            Ok(manager)
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::sessions_dir()?;
        let index_path = dir.join("index.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(index_path, content)?;
        Ok(())
    }

    pub fn current_session(&self) -> Option<&Session> {
        self.sessions.get(self.active_index)
    }

    pub fn current_session_mut(&mut self) -> Option<&mut Session> {
        self.sessions.get_mut(self.active_index)
    }

    pub fn create_session(&mut self, name: &str) {
        self.sessions.push(Session::new(name));
        self.active_index = self.sessions.len() - 1;
    }

    pub fn switch_session(&mut self, name: &str) -> bool {
        if let Some(idx) = self.sessions.iter().position(|s| s.name == name) {
            self.active_index = idx;
            true
        } else {
            false
        }
    }

    pub fn add_message_to_current(&mut self, msg: ChatMessage) {
        if let Some(session) = self.current_session_mut() {
            session.add_message(msg);
        }
    }

    pub fn clear_current(&mut self) {
        if let Some(session) = self.current_session_mut() {
            session.clear();
        }
    }

    pub fn export_current(&self) -> Option<String> {
        self.current_session().map(|s| s.export_markdown())
    }

    pub fn session_names(&self) -> Vec<String> {
        self.sessions.iter().map(|s| s.name.clone()).collect()
    }
}
