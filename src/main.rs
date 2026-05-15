// ┌──────────────────────────────────────────────────┐
// │              VyCode - Entry Point                │
// │     AI Coding Terminal Assistant                  │
// │     Created by Muhammad Lutfi Muzaki             │
// │     https://github.com/MuhammadLutfiMuzakiiVY    │
// └──────────────────────────────────────────────────┘

mod app;
mod commands;
mod config;
mod context;
mod providers;
mod session;
mod ui;
pub mod tools; // Integrated Tool Router Subsystem

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{self, IsTerminal, Read};

/// VyCode - AI Coding Terminal Assistant
#[derive(Parser, Debug)]
#[command(
    name = "vycode",
    version,
    about = "VyCode - AI Coding Terminal Assistant by Muhammad Lutfi Muzaki",
    long_about = "A modern terminal AI coding assistant supporting multiple providers.\nOpenAI, Anthropic, Gemini, OpenRouter, Groq, DeepSeek, Ollama, and custom endpoints."
)]
struct Cli {
    /// Path to project directory to analyze
    #[arg(long)]
    path: Option<String>,

    /// Skip the startup splash screen
    #[arg(long)]
    no_splash: bool,

    /// Reset configuration to defaults
    #[arg(long)]
    reset_config: bool,

    /// [v2.0] Provide a prompt execution string directly for shell pipelines
    #[arg(short, long)]
    prompt: Option<String>,

    /// [v2.0] Direct autonomous task string execution
    #[arg(short, long)]
    run: Option<String>,

    /// [v2.0] Automatically continue the previous active session
    #[arg(short, long = "continue")]
    continue_session: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle config reset
    if cli.reset_config {
        let config = config::AppConfig::default();
        config.save()?;
        println!("✅ Configuration reset to defaults.");
        return Ok(());
    }

    // 1. CHECK FOR PIPED INPUT / STDIN INTEGRATION (Feature 9)
    let mut piped_input = String::new();
    let is_piped = !io::stdin().is_terminal();
    if is_piped {
        let mut buffer = Vec::new();
        // Read stdin dynamically up to EOF
        let _ = io::stdin().read_to_end(&mut buffer);
        piped_input = String::from_utf8_lossy(&buffer).trim().to_string();
    }

    // Construct unified prompt if in pipeline or one-shot mode
    let query_prompt = match (&cli.prompt, &cli.run) {
        (Some(p), _) => Some(p.to_string()),
        (_, Some(r)) => Some(format!("Execute autonomous agent task: {}", r)),
        (None, None) => {
            if is_piped && !piped_input.is_empty() {
                Some("".to_string()) // Just piped context
            } else {
                None
            }
        }
    };

    // 2. ROUTE ONE-SHOT CLI MODE OR TUI INTERACTIVE MODE
    if query_prompt.is_some() || (is_piped && !piped_input.is_empty()) {
        let mut query = query_prompt.unwrap_or_default();
        if !piped_input.is_empty() {
            query = format!("CONTEXT DATA (Piped Stdin):\n```\n{}\n```\n\nUSER DIRECTIVE: {}", piped_input, query);
        }

        // Execute in lightweight non-interactive one-shot mode
        let mut app = app::App::new(cli.path, true).await?;
        app.run_one_shot(&query).await?;
        return Ok(());
    }

    // 3. DEFAULT INTERACTIVE TUI MODE
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run application
    let result = app::App::new(cli.path, cli.no_splash)
        .await?
        .run(&mut terminal)
        .await;

    // Restore terminal (always, even on error)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Report errors after terminal restoration
    if let Err(err) = result {
        eprintln!("\n⚠️  VyCode encountered an error: {err:?}");
        std::process::exit(1);
    }

    println!("\n👋 Thanks for using VyCode! — Muhammad Lutfi Muzaki");
    Ok(())
}
