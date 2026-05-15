// In-Project Global Search Tool
use anyhow::Result;
use walkdir::WalkDir;
use std::path::PathBuf;

pub fn execute(query: &str) -> Result<String> {
    let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut matches = Vec::new();

    let ignore_dirs = ["node_modules", ".git", "target", "dist", "build"];

    for entry in WalkDir::new(&base)
        .max_depth(5)
        .into_iter()
        .filter_entry(|e| !ignore_dirs.contains(&e.file_name().to_string_lossy().as_ref()))
        .flatten()
    {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            // Read line by line looking for text matches
            if let Ok(contents) = std::fs::read_to_string(file_path) {
                for (num, line) in contents.lines().enumerate() {
                    if line.contains(query) {
                        let rel_path = file_path.strip_prefix(&base).unwrap_or(file_path);
                        matches.push(format!("🔍 {}:{}: {}", rel_path.display(), num + 1, line.trim()));
                        if matches.len() >= 50 { // Cap at top 50 matches for efficiency
                            break;
                        }
                    }
                }
            }
        }
        if matches.len() >= 50 {
            break;
        }
    }

    let mut output = format!("🔎 [TOOL: SEARCH] Total results for \"{}\": {}\n", query, matches.len());
    output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    if matches.is_empty() {
        output.push_str("No matching text patterns found.\n");
    } else {
        output.push_str(&matches.join("\n"));
    }
    Ok(output)
}
