// VyCode - Project Context Module
#![allow(dead_code)]
pub mod memory;

use std::path::PathBuf;
use walkdir::WalkDir;
use self::memory::ProjectMemory;

/// Manages project context for AI conversations
pub struct ProjectContext {
    pub root_path: PathBuf,
    pub indexed_files: Vec<FileEntry>,
    pub context_summary: Option<String>,
    pub memory: ProjectMemory, // Deep Persistent Memory
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub extension: String,
    pub size: u64,
}

impl ProjectContext {
    pub fn new(path: Option<String>) -> Self {
        let root_path = path
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let memory = ProjectMemory::load();

        Self {
            root_path,
            indexed_files: Vec::new(),
            context_summary: None,
            memory,
        }
    }

    /// Index all files in the project directory
    pub fn index(&mut self) {
        self.indexed_files.clear();

        let ignore_dirs = [
            "node_modules", ".git", "target", "__pycache__",
            ".next", "dist", "build", ".vscode", ".idea",
        ];

        for entry in WalkDir::new(&self.root_path)
            .max_depth(8)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !ignore_dirs.contains(&name.as_ref())
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry
                    .path()
                    .strip_prefix(&self.root_path)
                    .unwrap_or(entry.path())
                    .to_string_lossy()
                    .to_string();

                let extension = entry
                    .path()
                    .extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_default();

                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

                self.indexed_files.push(FileEntry {
                    path,
                    extension,
                    size,
                });
            }
        }

        self.build_summary();
    }

    /// Build a context summary for AI conversations
    fn build_summary(&mut self) {
        if self.indexed_files.is_empty() {
            self.context_summary = None;
            return;
        }

        let mut summary = format!("Project root: {}\n", self.root_path.display());
        summary.push_str(&format!("Total files: {}\n", self.indexed_files.len()));

        // Count files by extension
        let mut ext_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for file in &self.indexed_files {
            if !file.extension.is_empty() {
                *ext_counts.entry(file.extension.clone()).or_insert(0) += 1;
            }
        }

        summary.push_str("File types: ");
        let mut ext_list: Vec<_> = ext_counts.into_iter().collect();
        ext_list.sort_by(|a, b| b.1.cmp(&a.1));
        for (ext, count) in ext_list.iter().take(10) {
            summary.push_str(&format!(".{ext}({count}) "));
        }
        summary.push('\n');

        // Key project files
        let key_files = [
            "Cargo.toml",
            "package.json",
            "pyproject.toml",
            "go.mod",
            "pom.xml",
            "build.gradle",
            "Makefile",
            "Dockerfile",
            "README.md",
        ];

        let found_key_files: Vec<_> = self
            .indexed_files
            .iter()
            .filter(|f| key_files.iter().any(|k| f.path.ends_with(k)))
            .map(|f| f.path.clone())
            .collect();

        if !found_key_files.is_empty() {
            summary.push_str("Key files: ");
            summary.push_str(&found_key_files.join(", "));
            summary.push('\n');
        }

        self.context_summary = Some(summary);
    }

    /// Get the context summary for use in system prompts
    pub fn get_summary(&self) -> Option<&str> {
        self.context_summary.as_deref()
    }
}
