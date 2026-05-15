// VyCode - Custom Widgets
#![allow(dead_code)]
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::theme::Theme;

/// Render a styled section box with title
pub fn styled_block(title: &str, active: bool) -> Block<'_> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(if active {
            Theme::border_active()
        } else {
            Theme::border()
        })
        .title(format!(" {title} "))
        .title_style(Theme::title())
        .style(Style::default().bg(Theme::BG_PRIMARY))
}

/// Render a status indicator
pub fn status_indicator(connected: bool) -> Span<'static> {
    if connected {
        Span::styled("● Connected", Theme::success())
    } else {
        Span::styled("○ Disconnected", Theme::error())
    }
}

/// Create a help text paragraph
pub fn help_text(lines: Vec<(&str, &str)>) -> Vec<Line<'static>> {
    lines
        .into_iter()
        .map(|(cmd, desc)| {
            Line::from(vec![
                Span::styled(
                    format!("  {cmd:<14}"),
                    Style::default()
                        .fg(Theme::ORANGE)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(desc.to_string(), Theme::text()),
            ])
        })
        .collect()
}

/// Create a message bubble for chat
pub fn message_bubble<'a>(role: &str, content: &'a str, is_user: bool) -> Paragraph<'a> {
    let style = if is_user {
        Theme::user_message()
    } else {
        Theme::assistant_message()
    };

    let role_style = if is_user {
        Style::default()
            .fg(Theme::ORANGE)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
            .fg(Theme::SUCCESS)
            .add_modifier(Modifier::BOLD)
    };

    let mut lines = vec![Line::from(vec![
        Span::styled(format!("{role}: "), role_style),
    ])];

    for line in content.lines() {
        lines.push(Line::from(Span::styled(line.to_string(), style)));
    }
    lines.push(Line::from(""));

    Paragraph::new(lines).wrap(Wrap { trim: false })
}

/// Format a key-value pair for display
pub fn key_value<'a>(key: &'a str, value: &'a str) -> Line<'a> {
    Line::from(vec![
        Span::styled(
            format!("{key}: "),
            Style::default()
                .fg(Theme::ORANGE_DIM)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(value, Theme::text()),
    ])
}
