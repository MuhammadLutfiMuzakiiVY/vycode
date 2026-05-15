// VyCode - Advanced Background Process Manager & Runner Subsystem
use anyhow::Result;
use std::collections::HashMap;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub id: String,
    pub command: String,
    pub is_running: bool,
    pub exit_code: Option<i32>,
}

/// Stores background stdout buffers
type LogStore = Arc<Mutex<HashMap<String, Vec<String>>>>;

/// Relational Pool managing active background daemons and package runners
pub struct ProcessManager {
    processes: HashMap<String, Child>,
    meta: HashMap<String, ProcessInfo>,
    logs: LogStore,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            meta: HashMap::new(),
            logs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Spawns a long-running background runner (npm start, cargo run, watch)
    pub async fn spawn_background(&mut self, label: &str, cmd_str: &str) -> Result<String> {
        let id = Uuid::new_v4().to_string()[0..6].to_string();
        let key = format!("{}-{}", label.replace(' ', "-"), id);

        #[cfg(target_os = "windows")]
        let mut child = Command::new("powershell")
            .args(["-Command", cmd_str])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        #[cfg(not(target_os = "windows"))]
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(cmd_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        // Insert log buffers for the background stream
        let logs_ref = self.logs.clone();
        let key_ref = key.clone();
        {
            let mut lock = logs_ref.lock().unwrap();
            lock.insert(key_ref.clone(), vec![format!("🚀 Process initiated: {}", cmd_str)]);
        }

        // Spawn async listener thread to capture stdout/stderr asynchronously
        tokio::spawn(async move {
            let mut stdout_reader = BufReader::new(stdout).lines();
            let mut stderr_reader = BufReader::new(stderr).lines();

            loop {
                tokio::select! {
                    Ok(Some(line)) = stdout_reader.next_line() => {
                        let mut lock = logs_ref.lock().unwrap();
                        if let Some(buf) = lock.get_mut(&key_ref) {
                            buf.push(line);
                            if buf.len() > 150 { buf.remove(0); } // Rolling log limit
                        }
                    }
                    Ok(Some(line)) = stderr_reader.next_line() => {
                        let mut lock = logs_ref.lock().unwrap();
                        if let Some(buf) = lock.get_mut(&key_ref) {
                            buf.push(format!("⚠️ ERR: {}", line));
                            if buf.len() > 150 { buf.remove(0); }
                        }
                    }
                    else => break, // IO closed, process exited
                }
            }
        });

        // Store handles
        self.processes.insert(key.clone(), child);
        self.meta.insert(
            key.clone(),
            ProcessInfo {
                id: key.clone(),
                command: cmd_str.to_string(),
                is_running: true,
                exit_code: None,
            },
        );

        Ok(key)
    }

    /// Checks running processes and harvests dead ones
    pub fn harvest(&mut self) {
        for (key, child) in self.processes.iter_mut() {
            if let Ok(Some(status)) = child.try_wait() {
                if let Some(info) = self.meta.get_mut(key) {
                    info.is_running = false;
                    info.exit_code = status.code();
                }
            }
        }
    }

    /// Retrieve detailed process report and the last 30 lines of activity logs
    pub fn get_status(&mut self, key: &str) -> Result<String> {
        self.harvest();

        let info = self.meta.get(key).ok_or_else(|| anyhow::anyhow!("Process handle `{}` not found.", key))?;
        let mut report = format!(
            "📊 **Process Report:** `{}`\n💡 **Command:** `{}`\n🔥 **Status:** {}\n",
            info.id,
            info.command,
            if info.is_running { "🟢 RUNNING".to_string() } else { format!("🔴 EXITED (Code: {:?})", info.exit_code) }
        );

        report.push_str("━━━━━━━━━ LOG BUFFER (LAST 30 LINES) ━━━━━━━━━\n");
        let lock = self.logs.lock().unwrap();
        if let Some(lines) = lock.get(key) {
            let skip = lines.len().saturating_sub(30);
            for line in lines.iter().skip(skip) {
                report.push_str(&format!("{}\n", line));
            }
        } else {
            report.push_str("No active logs present in memory.\n");
        }
        report.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        
        Ok(report)
    }

    /// Terminates an active daemon background runner
    pub async fn terminate(&mut self, key: &str) -> Result<String> {
        self.harvest();

        if let Some(mut child) = self.processes.remove(key) {
            let _ = child.kill().await;
            if let Some(info) = self.meta.get_mut(key) {
                info.is_running = false;
            }
            Ok(format!("✅ Process `{}` was sent the SIGKILL termination signal.", key))
        } else {
            Err(anyhow::anyhow!("Unable to terminate: process handle `{}` is not running.", key))
        }
    }

    /// List all registered background processes
    pub fn list_active(&mut self) -> String {
        self.harvest();
        
        if self.meta.is_empty() {
            return "ℹ️ No active background runner processes registered in pool.".to_string();
        }

        let mut out = "📋 **Background Process Subsystem Metrics:**\n".to_string();
        for (k, info) in &self.meta {
            out.push_str(&format!(
                "🔹 `{}` | {} | cmd: `{}`\n",
                k,
                if info.is_running { "🟢 Active" } else { "🔴 Exited" },
                info.command
            ));
        }
        out
    }
}
