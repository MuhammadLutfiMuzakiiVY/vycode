// VyCode - Docker Daemon Control Driver
use anyhow::Result;
use std::process::Command;

/// Orchestrates containers natively via direct bridge calls to the docker engine CLI
pub fn manage_docker(operation: &str, arg: &str) -> Result<String> {
    let mut cmd = Command::new("docker");
    
    match operation {
        "ps" => {
            cmd.arg("ps").arg("--format").arg("table {{.ID}}\t{{.Names}}\t{{.Status}}\t{{.Ports}}");
        }
        "images" => {
            cmd.arg("images");
        }
        "run" => {
            if arg.is_empty() { return Err(anyhow::anyhow!("Docker `run` requires an image parameter.")); }
            cmd.arg("run").arg("-d").arg(arg);
        }
        "stop" => {
            if arg.is_empty() { return Err(anyhow::anyhow!("Docker `stop` requires a container ID/Name.")); }
            cmd.arg("stop").arg(arg);
        }
        "logs" => {
            if arg.is_empty() { return Err(anyhow::anyhow!("Docker `logs` requires a container ID/Name.")); }
            cmd.arg("logs").arg("--tail").arg("100").arg(arg);
        }
        _ => return Err(anyhow::anyhow!("Unsupported Docker operation `{}`", operation))
    }

    let output = cmd.output()?;
    if output.status.success() {
        Ok(format!("🐳 [Docker {}]:\n{}", operation, String::from_utf8_lossy(&output.stdout)))
    } else {
        Err(anyhow::anyhow!("Docker API failure: {}", String::from_utf8_lossy(&output.stderr)))
    }
}
