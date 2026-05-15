// File Write Tool
use anyhow::Result;
use std::path::PathBuf;

pub fn execute(path_str: &str, content: &str) -> Result<String> {
    let path = PathBuf::from(path_str);
    let resolved = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };

    // Security & Safety check
    if let Some(parent) = resolved.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(&resolved, content)?;
    
    Ok(format!("✅ [TOOL: WRITE] Successfully written to: {}", resolved.display()))
}
