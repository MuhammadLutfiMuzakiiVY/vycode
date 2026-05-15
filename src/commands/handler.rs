// VyCode - Command Handler (file ops, shell exec, project scan)
use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Read a file and return its contents
pub fn read_file(path: &str) -> Result<String> {
    let path = resolve_path(path);
    let content = std::fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read '{}': {}", path.display(), e))?;
    Ok(format!(
        "📄 File: {}\n───────────────────────────\n{}",
        path.display(),
        content
    ))
}

/// Write content to a file
pub fn write_file(path: &str, content: &str) -> Result<String> {
    let path = resolve_path(path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, content)?;
    Ok(format!("✅ Written to: {}", path.display()))
}

/// Execute a shell command and return output
pub async fn exec_command(cmd: &str) -> Result<String> {
    let output = if cfg!(target_os = "windows") {
        tokio::process::Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .await?
    } else {
        tokio::process::Command::new("sh")
            .args(["-c", cmd])
            .output()
            .await?
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let status_icon = if output.status.success() {
        "✅"
    } else {
        "❌"
    };

    let mut result = format!("{status_icon} Command: {cmd}\n");
    result.push_str(&format!(
        "Exit code: {}\n",
        output.status.code().unwrap_or(-1)
    ));

    if !stdout.is_empty() {
        result.push_str(&format!("───── stdout ─────\n{stdout}"));
    }
    if !stderr.is_empty() {
        result.push_str(&format!("───── stderr ─────\n{stderr}"));
    }

    Ok(result)
}

/// Scan project directory and return a file tree
pub fn scan_project(base_path: Option<&str>) -> Result<String> {
    let base = base_path
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let mut output = format!("📁 Project scan: {}\n", base.display());
    output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut file_count = 0u32;
    let mut dir_count = 0u32;
    let mut total_size = 0u64;

    let ignore_dirs = [
        "node_modules",
        ".git",
        "target",
        "__pycache__",
        ".next",
        "dist",
        "build",
        ".vscode",
        ".idea",
        "vendor",
    ];

    for entry in WalkDir::new(&base)
        .max_depth(6)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !ignore_dirs.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
    {
        let depth = entry.depth();
        let indent = "  ".repeat(depth);
        let name = entry.file_name().to_string_lossy();

        if entry.file_type().is_dir() {
            if depth > 0 {
                output.push_str(&format!("{indent}📁 {name}/\n"));
                dir_count += 1;
            }
        } else {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            total_size += size;
            let size_str = format_size(size);
            let icon = file_icon(&name);
            output.push_str(&format!("{indent}{icon} {name} ({size_str})\n"));
            file_count += 1;
        }
    }

    output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    output.push_str(&format!(
        "📊 {file_count} files, {dir_count} dirs, {} total\n",
        format_size(total_size)
    ));

    Ok(output)
}

/// Get file icon based on extension
fn file_icon(name: &str) -> &'static str {
    match name.rsplit('.').next().unwrap_or("") {
        "rs" => "🦀",
        "py" => "🐍",
        "js" | "jsx" => "📜",
        "ts" | "tsx" => "📘",
        "html" => "🌐",
        "css" | "scss" => "🎨",
        "json" => "📋",
        "toml" | "yaml" | "yml" => "⚙️",
        "md" => "📝",
        "txt" => "📄",
        "png" | "jpg" | "svg" => "🖼️",
        "sh" | "bash" => "🔧",
        "sql" => "🗃️",
        "go" => "🔵",
        "java" | "kt" => "☕",
        "c" | "cpp" | "h" => "⚡",
        "lock" => "🔒",
        _ => "📄",
    }
}

/// Format file size to human-readable
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes}B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1}KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1}MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Resolve a potentially relative path
fn resolve_path(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_absolute() {
        p
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(p)
    }
}
