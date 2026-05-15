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
║                VyCode Commands                   ║
╠══════════════════════════════════════════════════╣
║                                                  ║
║  /help          Show this help message           ║
║  /model [name]  Change or show current model     ║
║  /provider      Switch AI provider               ║
║  /apikey        Update API key                   ║
║  /scan          Scan project files               ║
║  /fix [file]    Auto-fix code issues             ║
║  /explain [file] Explain code                    ║
║  /read <file>   Read a file                      ║
║  /write <f> <c> Write content to file            ║
║  /exec <cmd>    Execute shell command            ║
║  /session [name] Switch/create session           ║
║  /export        Export current session           ║
║  /clear         Clear chat history               ║
║  /exit          Exit VyCode                      ║
║                                                  ║
║  Shortcuts:                                      ║
║  Ctrl+C         Cancel current action            ║
║  Ctrl+L         Clear screen                     ║
║  ↑/↓            Scroll messages                  ║
║                                                  ║
╚══════════════════════════════════════════════════╝"#
            .to_string()
    }
}
