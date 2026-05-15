// File Read Tool
use anyhow::Result;
use std::path::PathBuf;

pub fn execute(path_str: &str) -> Result<String> {
    let path = PathBuf::from(path_str);
    let resolved = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };

    let content = std::fs::read_to_string(&resolved)
        .map_err(|e| anyhow::anyhow!("Failed to read '{}': {}", resolved.display(), e))?;
    
    Ok(format!(
        "📄 [TOOL: READ] {} (Bytes: {})\n━━━━━━━━━━━━━━━━━━━━━━━━━━━\n{}",
        resolved.display(),
        content.len(),
        content
    ))
}
