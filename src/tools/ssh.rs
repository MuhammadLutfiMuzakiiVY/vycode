// VyCode - Enterprise SSH Shell Remote Connection Driver
use anyhow::Result;
use std::process::Command;

/// Executes arbitrary commands on a remote server securely over native SSH client wrappers
pub fn run_ssh(host: &str, user: &str, port: &str, cmd_to_run: &str) -> Result<String> {
    let ssh_target = format!("{}@{}", user, host);
    let mut cmd = Command::new("ssh");
    
    cmd.arg("-p").arg(port)
       .arg("-o").arg("StrictHostKeyChecking=no")
       .arg("-o").arg("BatchMode=yes") // Prevent hanging on password prompts
       .arg(&ssh_target)
       .arg(cmd_to_run);

    let output = cmd.output()?;
    if output.status.success() {
        Ok(format!("🔑 [SSH Output from {}]:\n{}", host, String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(anyhow::anyhow!("SSH Connection failure: {}", String::from_utf8_lossy(&output.stderr)))
    }
}
