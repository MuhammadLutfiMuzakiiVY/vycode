// VyCode - Deep Persistent Project Memory Engine
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryFact {
    pub id: String,
    pub content: String,
    pub category: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectMemory {
    pub facts: Vec<MemoryFact>,
    pub last_updated: DateTime<Utc>,
}

impl ProjectMemory {
    /// Initialize an empty memory container
    pub fn new() -> Self {
        Self {
            facts: Vec::new(),
            last_updated: Utc::now(),
        }
    }

    /// Locate the local `.vycode/memory.json` path
    fn memory_path() -> Result<PathBuf> {
        let mut dir = std::env::current_dir()?;
        dir.push(".vycode");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }
        dir.push("memory.json");
        Ok(dir)
    }

    /// Load the persistent memory file
    pub fn load() -> Self {
        match Self::memory_path() {
            Ok(path) => {
                if path.exists() {
                    if let Ok(content) = std::fs::read_to_string(path) {
                        if let Ok(memory) = serde_json::from_str::<ProjectMemory>(&content) {
                            return memory;
                        }
                    }
                }
                Self::new()
            }
            Err(_) => Self::new(),
        }
    }

    /// Persist memory to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::memory_path()?;
        let serialized = serde_json::to_string_pretty(self)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    /// Insert a new project fact into long-term memory
    pub fn remember(&mut self, content: &str, category: Option<&str>) -> Result<String> {
        let id = Uuid::new_v4().to_string()[0..8].to_string();
        let fact = MemoryFact {
            id: id.clone(),
            content: content.to_string(),
            category: category.unwrap_or("general").to_string(),
            timestamp: Utc::now(),
        };
        self.facts.push(fact);
        self.last_updated = Utc::now();
        self.save()?;
        Ok(id)
    }

    /// Remove a memory fact by ID
    pub fn forget(&mut self, id: &str) -> Result<bool> {
        let initial_len = self.facts.len();
        self.facts.retain(|f| f.id != id);
        if self.facts.len() < initial_len {
            self.last_updated = Utc::now();
            self.save()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Clear all remembered knowledge
    pub fn wipe(&mut self) -> Result<()> {
        self.facts.clear();
        self.last_updated = Utc::now();
        self.save()?;
        Ok(())
    }

    /// Format active knowledge assets to present into AI Prompt
    pub fn get_prompt_injection(&self) -> String {
        if self.facts.is_empty() {
            return String::new();
        }

        let mut text = "\n\n🧠 LONG-TERM PROJECT MEMORY (Knowledge Context):\n".to_string();
        text.push_str("These facts were established across previous sessions. Maintain full alignment:\n");
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        for fact in &self.facts {
            text.push_str(&format!(
                "• [{}] [ID: {}]: {}\n",
                fact.category.to_uppercase(),
                fact.id,
                fact.content
            ));
        }
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text
    }

    /// Render ASCII table display of memory for TUI
    pub fn display_stats(&self) -> String {
        let mut view = "🧠 VyCode Project Memory Stats\n".to_string();
        view.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        if self.facts.is_empty() {
            view.push_str("No facts recorded yet. Teach me with `/remember <fact>`!\n");
        } else {
            view.push_str(&format!("Total remembered facts: {}\n\n", self.facts.len()));
            for fact in &self.facts {
                view.push_str(&format!(
                    "🆔 {} | [{}] {}\n",
                    fact.id,
                    fact.category,
                    fact.content
                ));
            }
        }
        view.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        view
    }
}
