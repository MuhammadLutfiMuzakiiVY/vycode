// VyCode - Theme Configuration
#![allow(dead_code)]
use ratatui::style::{Color, Modifier, Style};

/// VyCode orange theme palette
pub struct Theme;

impl Theme {
    // ── Primary Colors ──────────────────────────────────────────────
    pub const ORANGE: Color = Color::Rgb(255, 140, 0);
    pub const ORANGE_LIGHT: Color = Color::Rgb(255, 180, 60);
    pub const ORANGE_DIM: Color = Color::Rgb(180, 100, 0);
    pub const ORANGE_DARK: Color = Color::Rgb(120, 60, 0);

    // ── Background Colors ───────────────────────────────────────────
    pub const BG_PRIMARY: Color = Color::Rgb(15, 15, 20);
    pub const BG_SECONDARY: Color = Color::Rgb(25, 25, 35);
    pub const BG_ELEVATED: Color = Color::Rgb(35, 35, 50);
    pub const BG_HIGHLIGHT: Color = Color::Rgb(45, 40, 30);

    // ── Text Colors ─────────────────────────────────────────────────
    pub const TEXT_PRIMARY: Color = Color::Rgb(230, 230, 240);
    pub const TEXT_SECONDARY: Color = Color::Rgb(160, 160, 180);
    pub const TEXT_DIM: Color = Color::Rgb(100, 100, 120);
    pub const TEXT_MUTED: Color = Color::Rgb(70, 70, 90);

    // ── Accent Colors ───────────────────────────────────────────────
    pub const SUCCESS: Color = Color::Rgb(80, 200, 120);
    pub const ERROR: Color = Color::Rgb(240, 80, 80);
    pub const WARNING: Color = Color::Rgb(240, 180, 40);
    pub const INFO: Color = Color::Rgb(80, 160, 240);

    // ── Border Colors ───────────────────────────────────────────────
    pub const BORDER: Color = Color::Rgb(60, 60, 80);
    pub const BORDER_ACTIVE: Color = Color::Rgb(255, 140, 0);

    // ── Styles ──────────────────────────────────────────────────────
    pub fn title() -> Style {
        Style::default()
            .fg(Self::ORANGE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn subtitle() -> Style {
        Style::default().fg(Self::ORANGE_LIGHT)
    }

    pub fn text() -> Style {
        Style::default().fg(Self::TEXT_PRIMARY)
    }

    pub fn dim_text() -> Style {
        Style::default().fg(Self::TEXT_DIM)
    }

    pub fn muted_text() -> Style {
        Style::default().fg(Self::TEXT_MUTED)
    }

    pub fn user_message() -> Style {
        Style::default()
            .fg(Self::ORANGE_LIGHT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn assistant_message() -> Style {
        Style::default()
            .fg(Self::TEXT_PRIMARY)
    }

    pub fn system_message() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
            .add_modifier(Modifier::ITALIC)
    }

    pub fn error() -> Style {
        Style::default()
            .fg(Self::ERROR)
            .add_modifier(Modifier::BOLD)
    }

    pub fn success() -> Style {
        Style::default().fg(Self::SUCCESS)
    }

    pub fn border() -> Style {
        Style::default().fg(Self::BORDER)
    }

    pub fn border_active() -> Style {
        Style::default().fg(Self::BORDER_ACTIVE)
    }

    pub fn selected() -> Style {
        Style::default()
            .fg(Self::BG_PRIMARY)
            .bg(Self::ORANGE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn status_bar() -> Style {
        Style::default()
            .fg(Self::TEXT_SECONDARY)
            .bg(Self::BG_ELEVATED)
    }

    pub fn input() -> Style {
        Style::default().fg(Self::TEXT_PRIMARY)
    }

    pub fn keyword() -> Style {
        Style::default()
            .fg(Self::ORANGE)
            .add_modifier(Modifier::BOLD)
    }
}

/// Spinner frames for loading animation
pub const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

pub fn spinner_frame(tick: usize) -> &'static str {
    SPINNER_FRAMES[tick % SPINNER_FRAMES.len()]
}
