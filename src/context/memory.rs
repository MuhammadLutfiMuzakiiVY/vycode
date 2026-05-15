// VyCode - High-Performance Relational & Semantic SQLite Memory Engine
use anyhow::Result;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use uuid::Uuid;

/// Relational Bridge controlling project long-term memories & Full-Text Virtual tables
pub struct ProjectMemory {
    db_path: PathBuf,
}

impl ProjectMemory {
    /// Locates the SQLite memory path and enforces schema migrations
    pub fn load() -> Self {
        let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        dir.push(".vycode");
        if !dir.exists() {
            let _ = std::fs::create_dir_all(&dir);
        }
        dir.push("memory.db");
        
        let instance = Self { db_path: dir };
        let _ = instance.initialize_schema();
        instance
    }

    /// Establishes full-relational tables and FTS5 virtual indices
    fn initialize_schema(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        
        // 1. Knowledge Base - Virtual Table for BM25 Semantic Full Text Search (FTS5)
        conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS knowledge_base USING fts5(
                id UNINDEXED,
                content,
                category,
                timestamp UNINDEXED
            );",
            [],
        )?;

        // 2. Workspace Summary Cache
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workspace_cache (
                file_path TEXT PRIMARY KEY,
                file_hash TEXT NOT NULL,
                summary TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );",
            [],
        )?;

        // 3. Session History (Inter-session Resume Engine)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS session_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                session_id TEXT NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );",
            [],
        )?;

        Ok(())
    }

    /// Insert memory fact natively into FTS5 Virtual Index
    pub fn remember(&self, content: &str, category: Option<&str>) -> Result<String> {
        let conn = Connection::open(&self.db_path)?;
        let id = Uuid::new_v4().to_string()[0..8].to_string();
        let cat = category.unwrap_or("general");
        let ts = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO knowledge_base (id, content, category, timestamp) VALUES (?, ?, ?, ?);",
            params![id, content, cat, ts],
        )?;

        Ok(id)
    }

    /// Perform ranking Semantic Full Text Search (BM25 Search) across all stored knowledge!
    pub fn semantic_search(&self, query: &str) -> Result<Vec<(String, String, String)>> {
        let conn = Connection::open(&self.db_path)?;
        
        // Safely sanitize query for FTS5 Match or fallback to simple LIKE
        let sanitized_query = query.replace('"', "\"\"").replace('*', "");
        let mut stmt = conn.prepare(
            "SELECT id, content, category FROM knowledge_base 
             WHERE knowledge_base MATCH ? 
             ORDER BY rank LIMIT 10;"
        )?;

        let rows = stmt.query_map(params![sanitized_query], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        });

        match rows {
            Ok(mapped_rows) => {
                let mut out = Vec::new();
                for item in mapped_rows {
                    if let Ok(fact) = item { out.push(fact); }
                }
                Ok(out)
            }
            Err(_) => {
                // Fallback search if MATCH syntax errors
                let mut stmt = conn.prepare(
                    "SELECT id, content, category FROM knowledge_base WHERE content LIKE ? LIMIT 10;"
                )?;
                let like_q = format!("%{}%", query);
                let mut out = Vec::new();
                let mapped = stmt.query_map(params![like_q], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                })?;
                for item in mapped {
                    if let Ok(fact) = item { out.push(fact); }
                }
                Ok(out)
            }
        }
    }

    /// Delete fact by ID
    pub fn forget(&self, id: &str) -> Result<bool> {
        let conn = Connection::open(&self.db_path)?;
        let deleted = conn.execute("DELETE FROM knowledge_base WHERE id = ?;", params![id])?;
        Ok(deleted > 0)
    }

    /// Read entire facts list ordered by arrival
    fn get_all_facts(&self) -> Result<Vec<(String, String, String)>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT id, content, category FROM knowledge_base;")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        let mut out = Vec::new();
        for r in rows {
            if let Ok(f) = r { out.push(f); }
        }
        Ok(out)
    }

    /// Wipes persistent memory database
    pub fn wipe(&self) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute("DELETE FROM knowledge_base;", [])?;
        conn.execute("DELETE FROM workspace_cache;", [])?;
        conn.execute("DELETE FROM session_history;", [])?;
        Ok(())
    }

    // --- Workspace Cache Module ---

    /// Retrieve cached AI summary of code file based on unique file contents hash
    pub fn get_summary_cache(&self, path: &str, current_hash: &str) -> Result<Option<String>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT summary FROM workspace_cache WHERE file_path = ? AND file_hash = ?;")?;
        let mut rows = stmt.query_map(params![path, current_hash], |r| r.get(0))?;
        
        if let Some(res) = rows.next() {
            Ok(Some(res?))
        } else {
            Ok(None)
        }
    }

    /// Cache/Refresh specific file AI summary
    pub fn set_summary_cache(&self, path: &str, hash: &str, summary: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT OR REPLACE INTO workspace_cache (file_path, file_hash, summary, updated_at) VALUES (?, ?, ?, ?);",
            params![path, hash, summary, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    // --- Inter-Session Resume History Engine ---

    /// Commits absolute interaction context line into SQLite cross-session history vault
    pub fn record_history(&self, session_id: &str, role: &str, content: &str) -> Result<()> {
        let conn = Connection::open(&self.db_path)?;
        conn.execute(
            "INSERT INTO session_history (session_id, role, content, timestamp) VALUES (?, ?, ?, ?);",
            params![session_id, role, content, Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    /// Fetches past absolute context history lines to resume historic interactions cleanly
    pub fn get_session_history(&self, session_id: &str) -> Result<Vec<(String, String)>> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT role, content FROM session_history WHERE session_id = ? ORDER BY id ASC;")?;
        let rows = stmt.query_map(params![session_id], |r| Ok((r.get(0)?, r.get(1)?)))?;
        
        let mut list = Vec::new();
        for row in rows {
            if let Ok(val) = row { list.push(val); }
        }
        Ok(list)
    }

    // --- Core Integration Formatting ---

    /// Returns facts injection text for Prompt Engineering
    pub fn get_prompt_injection(&self) -> String {
        let facts = self.get_all_facts().unwrap_or_default();
        if facts.is_empty() {
            return String::new();
        }

        let mut text = "\n\n🧠 [MATURE SQLITE PROJECT KNOWLEDGE STORE]\n".to_string();
        text.push_str("The following facts are persistently cached in SQLite Relational Memory. Align completely:\n");
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        for fact in facts {
            text.push_str(&format!(
                "• [{}] [ID: {}]: {}\n",
                fact.2.to_uppercase(),
                fact.0,
                fact.1
            ));
        }
        text.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        text
    }

    /// Outputs full visual memory metrics dashboard
    pub fn display_stats(&self) -> String {
        let facts = self.get_all_facts().unwrap_or_default();
        
        let mut view = "🧠 VyCode Real-Time SQLite Memory (FTS5 Engaged)\n".to_string();
        view.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        view.push_str(&format!("📁 Secure Database Location: {:?}\n", self.db_path));
        
        if facts.is_empty() {
            view.push_str("⚠️ No relational facts stored. Store them with `/remember <fact>`.\n");
        } else {
            view.push_str(&format!("📦 Total relational knowledge facts: {}\n\n", facts.len()));
            for fact in facts {
                view.push_str(&format!(
                    "🆔 {} | [{}] {}\n",
                    fact.0,
                    fact.2,
                    fact.1
                ));
            }
        }
        view.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        view
    }
}
