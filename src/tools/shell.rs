// Shell Execution Tool
use anyhow::Result;

pub async fn execute(cmd: &str) -> Result<String> {
    // Verify executing binaries using dynamic shell switching
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

    let mut result = format!("🖥️ [TOOL: SHELL] cmd: `{}` (Exit: {})\n", cmd, output.status.code().unwrap_or(-1));
    if !stdout.is_empty() {
        result.push_str(&format!("── stdout ──\n{}\n", stdout));
    }
    if !stderr.is_empty() {
        result.push_str(&format!("── stderr ──\n{}\n", stderr));
    }
    Ok(result)
}
