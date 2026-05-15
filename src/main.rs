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

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

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
    #[arg(short, long)]
    path: Option<String>,

    /// Skip the startup splash screen
    #[arg(long)]
    no_splash: bool,

    /// Reset configuration to defaults
    #[arg(long)]
    reset_config: bool,
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
