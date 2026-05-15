// VyCode - UI Renderer
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, Scrollbar,
              ScrollbarOrientation, ScrollbarState, Wrap},
};

use crate::app::{App, AppScreen};
use crate::providers::{MessageRole, ProviderType};

use super::logo;
use super::theme::{self, Theme};
use super::widgets;

/// Render the entire application UI
pub fn render(app: &App, frame: &mut Frame) {
    // Clear with dark background
    let area = frame.area();
    frame.render_widget(
        Block::default().style(Style::default().bg(Theme::BG_PRIMARY)),
        area,
    );

    match app.screen {
        AppScreen::Startup => render_startup(app, frame, area),
        AppScreen::ProviderSelect => render_provider_select(app, frame, area),
        AppScreen::ApiKeyInput => render_input_screen(app, frame, area, "API Key", true),
        AppScreen::ModelInput => render_input_screen(app, frame, area, "Model Name", false),
        AppScreen::Chat => render_chat(app, frame, area),
    }
}

/// Render the startup/splash screen
fn render_startup(_app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Length(8),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Percentage(20),
            Constraint::Length(1),
        ])
        .split(area);

    // Logo
    let logo_text: Vec<Line> = logo::LOGO
        .lines()
        .map(|l| Line::from(Span::styled(l, Theme::title())))
        .collect();
    let logo_widget = Paragraph::new(logo_text).alignment(Alignment::Center);
    frame.render_widget(logo_widget, chunks[1]);

    // Tagline
    let tagline = Paragraph::new(Line::from(Span::styled(
        logo::TAGLINE,
        Theme::subtitle(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(tagline, chunks[2]);

    // Divider
    let divider = Paragraph::new(Line::from(Span::styled(
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
        Theme::dim_text(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(divider, chunks[3]);

    // Author
    let author = Paragraph::new(Line::from(Span::styled(
        logo::AUTHOR,
        Theme::text(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(author, chunks[4]);

    // GitHub
    let github = Paragraph::new(Line::from(Span::styled(
        logo::GITHUB,
        Theme::dim_text(),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(github, chunks[5]);

    // Version
    let version = Paragraph::new(Line::from(Span::styled(
        format!("v{}", logo::VERSION),
        Style::default().fg(Theme::ORANGE_DIM),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(version, chunks[6]);

    // Press any key
    let prompt = Paragraph::new(Line::from(Span::styled(
        "Press any key to continue...",
        Style::default()
            .fg(Theme::TEXT_SECONDARY)
            .add_modifier(Modifier::SLOW_BLINK),
    )))
    .alignment(Alignment::Center);
    frame.render_widget(prompt, chunks[9]);
}

/// Render provider selection screen
fn render_provider_select(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Min(10),
            Constraint::Length(2),
        ])
        .margin(2)
        .split(area);

    // Header
    let header = Paragraph::new(Line::from(vec![
        Span::styled("⚡ ", Theme::title()),
        Span::styled("Select AI Provider", Theme::title()),
    ]))
    .block(widgets::styled_block("VyCode Setup", true));
    frame.render_widget(header, chunks[0]);

    // Instructions
    let instructions = Paragraph::new(Line::from(Span::styled(
        "  Use ↑/↓ to navigate, Enter to select",
        Theme::dim_text(),
    )));
    frame.render_widget(instructions, chunks[1]);

    // Provider list
    let providers = ProviderType::all();
    let items: Vec<ListItem> = providers
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let style = if i == app.provider_select_index {
                Theme::selected()
            } else {
                Theme::text()
            };
            let prefix = if i == app.provider_select_index {
                " ▸ "
            } else {
                "   "
            };
            ListItem::new(Line::from(Span::styled(
                format!("{prefix}{}", p.display_name()),
                style,
            )))
        })
        .collect();

    let list = List::new(items)
        .block(widgets::styled_block("Providers", true))
        .highlight_style(Theme::selected());
    frame.render_widget(list, chunks[2]);

    // Footer hint
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  Esc", Style::default().fg(Theme::ORANGE).add_modifier(Modifier::BOLD)),
        Span::styled(" quit  ", Theme::dim_text()),
        Span::styled("Enter", Style::default().fg(Theme::ORANGE).add_modifier(Modifier::BOLD)),
        Span::styled(" select", Theme::dim_text()),
    ]));
    frame.render_widget(footer, chunks[3]);
}

/// Render an input screen (API key or model name)
fn render_input_screen(app: &App, frame: &mut Frame, area: Rect, label: &str, is_secret: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Percentage(30),
        ])
        .horizontal_margin(4)
        .split(area);

    // Title
    let provider_name = app.provider_type.display_name();
    let title = Paragraph::new(Line::from(vec![
        Span::styled("⚡ ", Theme::title()),
        Span::styled(
            format!("Configure {provider_name}"),
            Theme::title(),
        ),
    ]))
    .block(widgets::styled_block("Setup", true));
    frame.render_widget(title, chunks[1]);

    // Label
    let label_widget = Paragraph::new(Line::from(Span::styled(
        format!("  Enter {label}:"),
        Theme::subtitle(),
    )));
    frame.render_widget(label_widget, chunks[2]);

    // Input field
    let display_text = if is_secret && !app.input.is_empty() {
        "•".repeat(app.input.len())
    } else {
        app.input.clone()
    };

    let input = Paragraph::new(Line::from(Span::styled(&display_text, Theme::input())))
        .block(widgets::styled_block(label, true));
    frame.render_widget(input, chunks[3]);

    // Set cursor position
    let cursor_x = chunks[3].x + 1 + app.input_cursor as u16;
    let cursor_y = chunks[3].y + 1;
    frame.set_cursor_position(Position::new(cursor_x, cursor_y));

    // Footer hint
    let footer = Paragraph::new(Line::from(vec![
        Span::styled("  Enter", Style::default().fg(Theme::ORANGE).add_modifier(Modifier::BOLD)),
        Span::styled(" confirm  ", Theme::dim_text()),
        Span::styled("Esc", Style::default().fg(Theme::ORANGE).add_modifier(Modifier::BOLD)),
        Span::styled(" back", Theme::dim_text()),
    ]));
    frame.render_widget(footer, chunks[4]);
}

/// Render the main chat interface
fn render_chat(app: &App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header bar
            Constraint::Min(5),    // Messages area
            Constraint::Length(3), // Input area
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    render_header_bar(app, frame, chunks[0]);
    render_messages(app, frame, chunks[1]);
    render_input_bar(app, frame, chunks[2]);
    render_status_bar(app, frame, chunks[3]);
}

/// Render the top header bar
fn render_header_bar(app: &App, frame: &mut Frame, area: Rect) {
    let provider_name = app
        .provider
        .as_ref()
        .map(|p| p.name())
        .unwrap_or("None");
    let model = app.config.model.as_deref().unwrap_or("N/A");
    let session_name = app
        .session_manager
        .current_session()
        .map(|s| s.name.as_str())
        .unwrap_or("Default");

    let header = Paragraph::new(Line::from(vec![
        Span::styled(" ⚡ VyCode ", Theme::title()),
        Span::styled("│ ", Theme::dim_text()),
        Span::styled("Provider: ", Theme::dim_text()),
        Span::styled(provider_name, Theme::subtitle()),
        Span::styled(" │ ", Theme::dim_text()),
        Span::styled("Model: ", Theme::dim_text()),
        Span::styled(model, Theme::subtitle()),
        Span::styled(" │ ", Theme::dim_text()),
        Span::styled("Session: ", Theme::dim_text()),
        Span::styled(session_name, Theme::subtitle()),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::border_active())
            .style(Style::default().bg(Theme::BG_SECONDARY)),
    );
    frame.render_widget(header, area);
}

/// Render the message history area
fn render_messages(app: &App, frame: &mut Frame, area: Rect) {
    let mut lines: Vec<Line> = Vec::new();

    if app.messages.is_empty() && !app.is_streaming {
        // Welcome message
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Welcome to VyCode! 🚀",
            Theme::title(),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Type a message to start chatting with AI,",
            Theme::text(),
        )));
        lines.push(Line::from(Span::styled(
            "  or use /help to see available commands.",
            Theme::text(),
        )));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Quick Commands:",
            Theme::subtitle(),
        )));

        let commands = vec![
            ("/help", "Show all commands"),
            ("/scan", "Scan project files"),
            ("/model", "Change AI model"),
            ("/provider", "Switch provider"),
            ("/fix", "Auto-fix code issues"),
            ("/explain", "Explain code"),
            ("/clear", "Clear chat"),
            ("/exit", "Exit VyCode"),
        ];

        for (cmd, desc) in commands {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("    {cmd:<14}"),
                    Style::default()
                        .fg(Theme::ORANGE)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(desc, Theme::dim_text()),
            ]));
        }
    } else {
        // Render messages
        for msg in &app.messages {
            match msg.role {
                MessageRole::User => {
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            "  ▸ You: ",
                            Style::default()
                                .fg(Theme::ORANGE)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]));
                    for line in msg.content.lines() {
                        lines.push(Line::from(Span::styled(
                            format!("    {line}"),
                            Theme::text(),
                        )));
                    }
                }
                MessageRole::Assistant => {
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            "  ◆ VyCode: ",
                            Style::default()
                                .fg(Theme::SUCCESS)
                                .add_modifier(Modifier::BOLD),
                        ),
                    ]));
                    for line in msg.content.lines() {
                        lines.push(Line::from(Span::styled(
                            format!("    {line}"),
                            Theme::assistant_message(),
                        )));
                    }
                }
                MessageRole::System => {
                    lines.push(Line::from(Span::styled(
                        format!("  [system] {}", msg.content),
                        Theme::system_message(),
                    )));
                }
            }
        }

        // Streaming response in progress
        if app.is_streaming {
            lines.push(Line::from(""));
            let spinner = theme::spinner_frame(app.spinner_frame);
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  {spinner} VyCode: "),
                    Style::default()
                        .fg(Theme::SUCCESS)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            for line in app.streaming_text.lines() {
                lines.push(Line::from(Span::styled(
                    format!("    {line}"),
                    Theme::assistant_message(),
                )));
            }
            if app.streaming_text.is_empty() {
                lines.push(Line::from(Span::styled(
                    "    Thinking...",
                    Theme::dim_text(),
                )));
            }
        }
    }

    // Calculate scroll
    let content_height = lines.len();
    let visible_height = (area.height as usize).saturating_sub(2);
    let scroll_offset = if content_height > visible_height {
        content_height - visible_height
    } else {
        0
    };

    let messages_widget = Paragraph::new(lines)
        .block(widgets::styled_block("Chat", true))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset as u16, 0));
    frame.render_widget(messages_widget, area);

    // Scrollbar
    if content_height > visible_height {
        let mut scrollbar_state = ScrollbarState::new(content_height)
            .position(scroll_offset);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(Theme::ORANGE_DIM)),
            area,
            &mut scrollbar_state,
        );
    }
}

/// Render the input bar at the bottom
fn render_input_bar(app: &App, frame: &mut Frame, area: Rect) {
    let prefix = if app.is_streaming { "⏳ " } else { "▸ " };
    let input_text = if app.is_streaming {
        "Streaming response... (Esc to cancel)"
    } else {
        &app.input
    };

    let input = Paragraph::new(Line::from(vec![
        Span::styled(
            prefix,
            Style::default()
                .fg(Theme::ORANGE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(input_text, Theme::input()),
    ]))
    .block(widgets::styled_block("Input", !app.is_streaming));
    frame.render_widget(input, area);

    // Set cursor if editing
    if !app.is_streaming {
        let cursor_x = area.x + 1 + prefix.len() as u16 + app.input_cursor as u16;
        let cursor_y = area.y + 1;
        frame.set_cursor_position(Position::new(cursor_x, cursor_y));
    }
}

/// Render the bottom status bar
fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::styled(" VyCode ", Style::default().fg(Theme::BG_PRIMARY).bg(Theme::ORANGE).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!(" {} ", app.status_message),
            Theme::status_bar(),
        ),
        Span::styled(
            format!(" │ Messages: {} ", app.messages.len()),
            Theme::status_bar(),
        ),
        Span::styled(
            " │ /help for commands ",
            Theme::status_bar(),
        ),
    ]))
    .style(Style::default().bg(Theme::BG_ELEVATED));
    frame.render_widget(status, area);
}
