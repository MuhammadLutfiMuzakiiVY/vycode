// VyCode - Tool Router & System Abstraction
pub mod file_read;
pub mod file_write;
pub mod shell;
pub mod git;
pub mod search;
pub mod docs;
pub mod telegram;
pub mod discord;
pub mod omni;
pub mod github;
pub mod browser;
pub mod docker;
pub mod ssh;
pub mod hass;
pub mod db;
pub mod mcp;
pub mod runner;

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
            "slack" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Slack Webhook required"))?;
                let msg = args.get(1).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_slack(url, msg).await
            }
            "teams" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Teams Webhook required"))?;
                let msg = args.get(1).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_teams(url, msg).await
            }
            "matrix" => {
                let hs = args.get(0).ok_or_else(|| anyhow::anyhow!("Matrix Homeserver required"))?;
                let room = args.get(1).ok_or_else(|| anyhow::anyhow!("Room ID required"))?;
                let tok = args.get(2).ok_or_else(|| anyhow::anyhow!("Access Token required"))?;
                let msg = args.get(3).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_matrix(hs, room, tok, msg).await
            }
            "signal" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Signal REST URL required"))?;
                let rec = args.get(1).ok_or_else(|| anyhow::anyhow!("Recipient required"))?;
                let msg = args.get(2).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_signal(url, rec, msg).await
            }
            "whatsapp" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("WhatsApp Gateway URL required"))?;
                let tok = args.get(1).ok_or_else(|| anyhow::anyhow!("Auth Token required"))?;
                let msg = args.get(2).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_whatsapp(url, tok, msg).await
            }
            "email" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("Email Gateway URL required"))?;
                let rec = args.get(1).ok_or_else(|| anyhow::anyhow!("Recipient required"))?;
                let msg = args.get(2).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_email(url, rec, msg).await
            }
            "sms" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("SMS Gateway URL required"))?;
                let rec = args.get(1).ok_or_else(|| anyhow::anyhow!("Recipient required"))?;
                let msg = args.get(2).ok_or_else(|| anyhow::anyhow!("Message required"))?;
                omni::send_sms(url, rec, msg).await
            }
            "github" => {
                let token = args.get(0).ok_or_else(|| anyhow::anyhow!("GitHub Token required"))?;
                let repo = args.get(1).ok_or_else(|| anyhow::anyhow!("Repo slug (owner/name) required"))?;
                let op = args.get(2).ok_or_else(|| anyhow::anyhow!("Operation required"))?;
                let payload = args.get(3).unwrap_or(&"");
                github::execute_github(token, repo, op, payload).await
            }
            "browser" => {
                let op = args.get(0).ok_or_else(|| anyhow::anyhow!("Browser operation (dom/screenshot/pdf) required"))?;
                let url = args.get(1).ok_or_else(|| anyhow::anyhow!("URL required"))?;
                let target = args.get(2).unwrap_or(&"");
                // Synch call
                browser::drive_browser(op, url, target)
            }
            "docker" => {
                let op = args.get(0).ok_or_else(|| anyhow::anyhow!("Docker operation (ps/images/run/stop/logs) required"))?;
                let arg = args.get(1).unwrap_or(&"");
                docker::manage_docker(op, arg)
            }
            "ssh" => {
                let host = args.get(0).ok_or_else(|| anyhow::anyhow!("SSH host required"))?;
                let user = args.get(1).ok_or_else(|| anyhow::anyhow!("SSH user required"))?;
                let port = args.get(2).unwrap_or(&"22");
                let cmd = args.get(3).ok_or_else(|| anyhow::anyhow!("Remote command required"))?;
                ssh::run_ssh(host, user, port, cmd)
            }
            "hass" => {
                let url = args.get(0).ok_or_else(|| anyhow::anyhow!("HASS URL required"))?;
                let tok = args.get(1).ok_or_else(|| anyhow::anyhow!("Access Token required"))?;
                let op = args.get(2).ok_or_else(|| anyhow::anyhow!("Operation required"))?;
                let ent = args.get(3).unwrap_or(&"");
                hass::control_hass(url, tok, op, ent).await
            }
            "db" => {
                let eng = args.get(0).ok_or_else(|| anyhow::anyhow!("DB engine (sqlite/postgres/mysql) required"))?;
                let uri = args.get(1).ok_or_else(|| anyhow::anyhow!("Connection URI required"))?;
                let sql = args.get(2).ok_or_else(|| anyhow::anyhow!("SQL query required"))?;
                db::execute_query(eng, uri, sql)
            }
            "mcp" => {
                let srv = args.get(0).ok_or_else(|| anyhow::anyhow!("MCP Server URL required"))?;
                let op = args.get(1).ok_or_else(|| anyhow::anyhow!("MCP Operation (list/call) required"))?;
                if op == &"list" {
                    mcp::list_mcp_tools(srv).await
                } else {
                    let tool = args.get(2).ok_or_else(|| anyhow::anyhow!("Tool name required"))?;
                    let args_raw = args.get(3).unwrap_or(&"{}");
                    let args_val: serde_json::Value = serde_json::from_str(args_raw).unwrap_or(serde_json::json!({}));
                    mcp::execute_mcp_tool(srv, tool, args_val).await
                }
            }
            _ => Err(anyhow::anyhow!("Unknown tool target")),
        }
    }
}
