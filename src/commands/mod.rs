// VyCode - Commands Module
pub mod handler;

/// All available slash commands
#[derive(Debug, Clone, PartialEq)]
pub enum SlashCommand {
    Help,
    Model(Option<String>),
    Provider,
    ApiKey,
    Scan,
    Fix(Option<String>),
    Explain(Option<String>),
    Clear,
    Exit,
    Read(String),
    Write(String, String),
    Exec(String),
    Session(Option<String>),
    Export,
    Theme(Option<String>),
    Graph,
    Heal(Option<String>),
    Chain(String),
    Git(String),
    Memory,
    Remember(String),
    Forget(String),
    Docs(String),
    TgSetup(String, String),
    Tg(String),
    Unknown(String),
}

impl SlashCommand {
    /// Parse a slash command from input string
    pub fn parse(input: &str) -> Option<Self> {
        let input = input.trim();
        if !input.starts_with('/') {
            return None;
        }

        let parts: Vec<&str> = input[1..].splitn(2, ' ').collect();
        let cmd = parts[0].to_lowercase();
        let arg = parts.get(1).map(|s| s.trim().to_string());

        Some(match cmd.as_str() {
            "help" | "h" => Self::Help,
            "model" | "m" => Self::Model(arg),
            "provider" | "p" => Self::Provider,
            "apikey" | "key" => Self::ApiKey,
            "scan" | "s" => Self::Scan,
            "fix" | "f" => Self::Fix(arg),
            "explain" | "e" => Self::Explain(arg),
            "clear" | "c" => Self::Clear,
            "exit" | "quit" | "q" => Self::Exit,
            "read" | "r" => {
                if let Some(path) = arg {
                    Self::Read(path)
                } else {
                    Self::Unknown("read requires a file path".to_string())
                }
            }
            "write" | "w" => {
                if let Some(args) = arg {
                    let parts: Vec<&str> = args.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        Self::Write(parts[0].to_string(), parts[1].to_string())
                    } else {
                        Self::Unknown("write requires: /write <path> <content>".to_string())
                    }
                } else {
                    Self::Unknown("write requires arguments".to_string())
                }
            }
            "exec" | "!" => {
                if let Some(cmd) = arg {
                    Self::Exec(cmd)
                } else {
                    Self::Unknown("exec requires a command".to_string())
                }
            }
            "session" => Self::Session(arg),
            "export" => Self::Export,
            "theme" => Self::Theme(arg),
            "graph" | "tree" => Self::Graph,
            "heal" | "watch" => Self::Heal(arg),
            "chain" => {
                if let Some(task) = arg {
                    Self::Chain(task)
                } else {
                    Self::Unknown("chain requires a multi-step task description".to_string())
                }
            }
            "git" | "g" => {
                if let Some(git_args) = arg {
                    Self::Git(git_args)
                } else {
                    Self::Git("status".to_string()) // default to status if no args
                }
            }
            "memory" => Self::Memory,
            "remember" => {
                if let Some(fact) = arg {
                    Self::Remember(fact)
                } else {
                    Self::Unknown("remember requires a fact: /remember <fact_text>".to_string())
                }
            }
            "forget" => {
                if let Some(id) = arg {
                    Self::Forget(id)
                } else {
                    Self::Unknown("forget requires a fact ID: /forget <fact_id>".to_string())
                }
            }
            "docs" | "doc" => {
                if let Some(url) = arg {
                    Self::Docs(url)
                } else {
                    Self::Unknown("docs requires a URL: /docs <url>".to_string())
                }
            }
            "tg-setup" => {
                if let Some(args) = arg {
                    let sub_parts: Vec<&str> = args.splitn(2, ' ').collect();
                    if sub_parts.len() == 2 {
                        Self::TgSetup(sub_parts[0].to_string(), sub_parts[1].to_string())
                    } else {
                        Self::Unknown("tg-setup requires: /tg-setup <token> <chat_id>".to_string())
                    }
                } else {
                    Self::Unknown("tg-setup requires arguments".to_string())
                }
            }
            "tg" => {
                if let Some(msg) = arg {
                    Self::Tg(msg)
                } else {
                    Self::Unknown("tg requires a message: /tg <message_text>".to_string())
                }
            }
            other => Self::Unknown(other.to_string()),
        })
    }
}

/// Command handler that processes slash commands
pub struct CommandHandler;

impl CommandHandler {
    pub fn new() -> Self {
        Self
    }

    /// Get help text for all commands
    pub fn help_text() -> String {
        r#"╔══════════════════════════════════════════════════╗
║              VyCode v2.1.0 Commands              ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  /help          Show this help message           ║
║  /model [name]  [HOT SWAP] Change active model   ║
║  /provider      Switch AI provider               ║
║  /apikey        Update API key                   ║
║  /scan          Scan project context             ║
║  /graph         Visual ASCII Dependency Tree     ║
║  /heal [file]   Self-healing active compiler     ║
║  /chain <task>  Autonomous Agentic loop execution║
║  /git <args>    Integrated Git Control Center    ║
║  /docs <url>    Read web docs + inject context   ║
║  /tg <msg>      Push message/update to Telegram  ║
║  /tg-setup <t><c>Configure remote Telegram Bot    ║
║  /memory        View Long-Term Project Memory    ║
║  /remember <f>  Store a fact in local memory     ║
║  /forget <id>   Wipe a fact by ID from memory    ║
║  /fix [file]    Auto-fix code issues             ║
║  /explain [file] Explain code structure          ║
║  /read <file>   Read a file                      ║
║  /write <f> <c> Write content to file            ║
║  /exec <cmd>    Execute shell command            ║
║  /session [name] Switch/create session           ║
║  /export        Export session to Markdown       ║
║  /clear         Clear screen history             ║
║  /exit          Exit VyCode                      ║
║                                                  ║
║  Shortcuts:                                      ║
║  Ctrl+C         Cancel streaming or exit         ║
║  Ctrl+L         Clear screen console             ║
║  ↑/↓            Scroll history viewport          ║
║                                                  ║
╚══════════════════════════════════════════════════╝"#
            .to_string()
    }
}
