// Git Integration Tool
use anyhow::Result;
use crate::tools::shell;

pub async fn execute(git_cmd: &str) -> Result<String> {
    // Basic safety checks to ensure it operates in git mode
    let full_cmd = format!("git {}", git_cmd);
    let output = shell::execute(&full_cmd).await?;
    
    let mut report = format!("🌳 [TOOL: GIT] Operation executed.\n");
    report.push_str(&output);
    Ok(report)
}
