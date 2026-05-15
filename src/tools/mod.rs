// VyCode - Tool Router & System Abstraction
pub mod file_read;
pub mod file_write;
pub mod shell;
pub mod git;
pub mod search;
pub mod docs;
pub mod telegram;
pub mod discord;

use anyhow::Result;

/// The Tool Router facilitates directing dynamic commands to their sub-modules.
/// Highly modular implementation ensuring security boundaries & sandboxing.
pub struct ToolRouter;

impl ToolRouter {
    pub async fn route_command(tool: &str, args: &[&str]) -> Result<String> {
        match tool {
            "read" => {
                let path = args.get(0).ok_or_else(|| anyhow::anyhow!("No path provided"))?;
                file_read::execute(path)
            }
            "write" => {
                let path = args.get(0).ok_or_else(|| anyhow::anyhow!("No path provided"))?;
                let content = args.get(1).ok_or_else(|| anyhow::anyhow!("No content provided"))?;
                file_write::execute(path, content)
            }
            "shell" => {
                let cmd = args.get(0).ok_or_else(|| anyhow::anyhow!("No command string provided"))?;
                shell::execute(cmd).await
            }
            "git" => {
                let git_cmd = args.get(0).unwrap_or(&"status");
                git::execute(git_cmd).await
            }
            "search" => {
                let query = args.get(0).ok_or_else(|| anyhow::anyhow!("No query string provided"))?;
                search::execute(query)
            }
            "docs" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("No documentation URL provided"))?;
                docs::fetch_documentation(url).await
            }
            "telegram" => {
                let token = args.get(0).ok_or_else(|| anyhow::anyhow!("Telegram Token required"))?;
                let chat_id = args.get(1).ok_or_else(|| anyhow::anyhow!("Chat ID required"))?;
                let msg = args.get(2).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                telegram::send_message(token, chat_id, msg).await
            }
            "discord" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Discord Webhook URL required"))?;
                let msg = args.get(1).ok_or_else(|| anyhow::anyhow!("Message content required"))?;
                discord::send_webhook(url, msg).await
            }
            _ => Err(anyhow::anyhow!("Unknown tool target")),
        }
    }
}
