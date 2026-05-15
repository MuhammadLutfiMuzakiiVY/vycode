// VyCode - Main Application Logic
#![allow(dead_code)]
use anyhow::Result;
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyModifiers};
use futures::StreamExt;
use ratatui::prelude::*;
use tokio::sync::mpsc;

use crate::commands::{handler as cmd_handler, CommandHandler, SlashCommand};
use crate::config::AppConfig;
use crate::context::ProjectContext;
use crate::providers::{self, AiProvider, ChatMessage, ProviderType, StreamEvent};
use crate::session::SessionManager;
use crate::ui;

// ── App State Enums ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum AppScreen {
    Startup,
    ProviderSelect,
    ApiKeyInput,
    ModelInput,
    Chat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

// ── Main Application Struct ─────────────────────────────────────────

pub struct App {
    pub screen: AppScreen,
    pub input_mode: InputMode,
    pub input: String,
    pub input_cursor: usize,
    pub messages: Vec<ChatMessage>,
    pub config: AppConfig,
    pub provider: Option<Box<dyn AiProvider>>,
    pub provider_type: ProviderType,
    pub session_manager: SessionManager,
    pub project_context: ProjectContext,
    pub command_handler: CommandHandler,
    pub should_quit: bool,
    pub scroll_offset: usize,
    pub streaming_text: String,
    pub is_streaming: bool,
    pub spinner_frame: usize,
    pub status_message: String,
    pub provider_select_index: usize,
    pub no_splash: bool,
    stream_tx: Option<mpsc::UnboundedSender<StreamEvent>>,
    stream_rx: Option<mpsc::UnboundedReceiver<StreamEvent>>,
    pub agent_chain_active: bool,
    pub agent_chain_steps: usize,
    pub pending_agent_action: Option<String>,
}

impl App {
    /// Create a new App instance, loading persisted config and sessions
    pub async fn new(project_path: Option<String>, no_splash: bool) -> Result<Self> {
        let config = AppConfig::load()?;
        let mut project_context = ProjectContext::new(project_path);
        let session_manager = SessionManager::load()?;

        let provider_type = config
            .provider_type
            .clone()
            .unwrap_or(ProviderType::OpenAI);

        let screen = if no_splash {
            if config.is_configured() {
                AppScreen::Chat
            } else {
                AppScreen::ProviderSelect
            }
        } else {
            AppScreen::Startup
        };

        let provider: Option<Box<dyn AiProvider>> = if config.is_configured() {
            Some(providers::create_provider(
                config.provider_type.as_ref().unwrap_or(&ProviderType::OpenAI),
                &config,
            ))
        } else {
            None
        };

        let messages = if let Some(session) = session_manager.current_session() {
            session.messages.clone()
        } else {
            Vec::new()
        };

        // Auto-index project if configured
        if config.auto_scan {
            project_context.index();
        }

        Ok(Self {
            screen,
            input_mode: InputMode::Editing,
            input: String::new(),
            input_cursor: 0,
            messages,
            config,
            provider,
            provider_type,
            session_manager,
            project_context,
            command_handler: CommandHandler::new(),
            should_quit: false,
            scroll_offset: 0,
            streaming_text: String::new(),
            is_streaming: false,
            spinner_frame: 0,
            status_message: String::from("Ready"),
            provider_select_index: 0,
            no_splash,
            stream_tx: None,
            stream_rx: None,
            agent_chain_active: false,
            agent_chain_steps: 0,
            pending_agent_action: None,
        })
    }

    /// Main event loop
    pub async fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
        let mut event_stream = EventStream::new();
        let mut spinner_interval = tokio::time::interval(tokio::time::Duration::from_millis(100));

        loop {
            // Render the UI
            terminal.draw(|f| ui::renderer::render(self, f))?;

            if self.should_quit {
                // Save state before exit
                self.session_manager.save()?;
                self.config.save()?;
                break;
            }

            tokio::select! {
                // Handle crossterm events
                Some(Ok(event)) = event_stream.next() => {
                    self.handle_event(event).await?;
                }

                // Handle streaming response chunks
                Some(event) = async {
                    if let Some(ref mut rx) = self.stream_rx {
                        rx.recv().await
                    } else {
                        std::future::pending::<Option<StreamEvent>>().await
                    }
                } => {
                    self.handle_stream_event(event);
                }

                // Spinner tick for loading animation
                _ = spinner_interval.tick() => {
                    if self.is_streaming {
                        self.spinner_frame = self.spinner_frame.wrapping_add(1);
                    }
                }

                // 💎 HIGH-PERFORMANCE FEATURE: Sovereign Agent Fully Autonomous Loop Handler
                _ = async {
                    if !self.is_streaming && self.agent_chain_active && self.pending_agent_action.is_some() {
                        // Small yield to allow TUI refresh to hit screen before blocking execute
                        tokio::time::sleep(tokio::time::Duration::from_millis(400)).await;
                    } else {
                        std::future::pending::<()>().await
                    }
                } => {
                    if let Some(action) = self.pending_agent_action.take() {
                        self.agent_chain_steps += 1;

                        // Absolute safety ceiling to prevent token burn / infinite agent loop
                        if self.agent_chain_steps > 10 {
                             self.add_system_message("⚠️🛑 **AGENT SAFETY HALT:** Reached maximum 10-step Sovereign limit! Halting autonomous chain to conserve resources.");
                             self.agent_chain_active = false;
                             self.agent_chain_steps = 0;
                             continue;
                        }

                        self.status_message = format!("🔄 Running Sovereign Agent Step {}...", self.agent_chain_steps);
                        
                        let feedback = if action.starts_with("exec:") {
                            let cmd = &action[5..];
                            self.add_system_message(&format!("⚡⚙️ **[Auto-Exec Phase {}]:** Running `{}`", self.agent_chain_steps, cmd));
                            match cmd_handler::exec_command(cmd).await {
                                Ok(stdout) => format!("✅ NATIVE EXECUTION STDOUT:\n```\n{}\n```", stdout),
                                Err(e) => format!("❌ NATIVE EXECUTION ERROR:\n```\n{}\n```", e),
                            }
                        } else if action.starts_with("write:") {
                            let rest = &action[6..];
                            if let Some(pipe_idx) = rest.find('|') {
                                let path = rest[..pipe_idx].trim();
                                let content = &rest[pipe_idx + 1..];
                                self.add_system_message(&format!("⚡💾 **[Auto-Exec Phase {}]:** Patched contents into `{}`", self.agent_chain_steps, path));
                                match cmd_handler::write_file(path, content) {
                                    Ok(msg) => format!("✅ FILE NATIVELY MODIFIED: {}", msg),
                                    Err(e) => format!("❌ FILE MODIFICATION ERROR: {}", e),
                                }
                            } else {
                                "❌ SCHEMA PARSE ERROR: Invalid syntax. Use `[WRITE: <filepath>|<content>]`. Notice the `|` separator!".to_string()
                            }
                        } else {
                            "❌ UNSUPPORTED COMMAND PROTOCOL".to_string()
                        };

                        // Pipe execution feedback directly back into prompt stream automatically!
                        let loop_prompt = format!(
                            "🤖⚙️ SOVEREIGN AUTO-FEEDBACK EVENT (Phase {} of 10):\n{}\n\nCRITICAL INSTRUCTION: Assess the results above. If the task objective is 100% completed, respond with `[DONE: <summary>]`. If additional work is required, issue your NEXT command now.",
                            self.agent_chain_steps,
                            feedback
                        );
                        
                        let _ = self.send_message(&loop_prompt).await;
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle a crossterm event
    async fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(key) => self.handle_key(key).await?,
            Event::Resize(_, _) => {} // ratatui handles resize
            _ => {}
        }
        Ok(())
    }

    /// Handle key press events
    async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        // Global: Ctrl+C always quits
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            if self.is_streaming {
                self.cancel_streaming();
            } else {
                self.should_quit = true;
            }
            return Ok(());
        }

        match self.screen {
            AppScreen::Startup => {
                // Any key progresses past startup
                if config_needs_setup(&self.config) {
                    self.screen = AppScreen::ProviderSelect;
                } else {
                    self.screen = AppScreen::Chat;
                }
            }

            AppScreen::ProviderSelect => {
                self.handle_provider_select(key).await?;
            }

            AppScreen::ApiKeyInput | AppScreen::ModelInput => {
                self.handle_text_input(key).await?;
            }

            AppScreen::Chat => {
                self.handle_chat_input(key).await?;
            }
        }

        Ok(())
    }

    /// Handle keys on the provider selection screen
    async fn handle_provider_select(&mut self, key: KeyEvent) -> Result<()> {
        let providers = ProviderType::all();
        match key.code {
            KeyCode::Up => {
                if self.provider_select_index > 0 {
                    self.provider_select_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.provider_select_index < providers.len() - 1 {
                    self.provider_select_index += 1;
                }
            }
            KeyCode::Enter => {
                let selected = providers[self.provider_select_index].clone();
                self.provider_type = selected.clone();
                self.config.set_provider(selected.clone());

                // Set default model
                self.config.set_model(selected.default_model());

                if selected.needs_api_key() {
                    self.input.clear();
                    self.input_cursor = 0;
                    self.screen = AppScreen::ApiKeyInput;
                } else if selected.needs_base_url() {
                    // For Ollama, go to model input with default base URL
                    self.config
                        .set_custom_base_url("http://localhost:11434");
                    self.input = selected.default_model().to_string();
                    self.input_cursor = self.input.len();
                    self.screen = AppScreen::ModelInput;
                } else {
                    self.input = selected.default_model().to_string();
                    self.input_cursor = self.input.len();
                    self.screen = AppScreen::ModelInput;
                }
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle text input (API key / model name screens)
    async fn handle_text_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    match self.screen {
                        AppScreen::ApiKeyInput => {
                            self.config.set_api_key(&self.input);
                            self.input = self
                                .provider_type
                                .default_model()
                                .to_string();
                            self.input_cursor = self.input.len();
                            self.screen = AppScreen::ModelInput;
                        }
                        AppScreen::ModelInput => {
                            self.config.set_model(&self.input);
                            self.config.save()?;

                            // Create provider and enter chat
                            self.provider = Some(providers::create_provider(
                                &self.provider_type,
                                &self.config,
                            ));
                            self.input.clear();
                            self.input_cursor = 0;
                            self.status_message =
                                format!("Connected to {}", self.provider_type.display_name());
                            self.screen = AppScreen::Chat;
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Char(c) => {
                self.input.insert(self.input_cursor, c);
                self.input_cursor += 1;
            }
            KeyCode::Backspace => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                    self.input.remove(self.input_cursor);
                }
            }
            KeyCode::Delete => {
                if self.input_cursor < self.input.len() {
                    self.input.remove(self.input_cursor);
                }
            }
            KeyCode::Left => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.input_cursor < self.input.len() {
                    self.input_cursor += 1;
                }
            }
            KeyCode::Home => {
                self.input_cursor = 0;
            }
            KeyCode::End => {
                self.input_cursor = self.input.len();
            }
            KeyCode::Esc => {
                // Go back
                match self.screen {
                    AppScreen::ApiKeyInput => {
                        self.screen = AppScreen::ProviderSelect;
                        self.input.clear();
                        self.input_cursor = 0;
                    }
                    AppScreen::ModelInput => {
                        if self.provider_type.needs_api_key() {
                            self.screen = AppScreen::ApiKeyInput;
                        } else {
                            self.screen = AppScreen::ProviderSelect;
                        }
                        self.input.clear();
                        self.input_cursor = 0;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Handle keys in the chat screen
    async fn handle_chat_input(&mut self, key: KeyEvent) -> Result<()> {
        // Ctrl+L clears screen
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('l') {
            self.messages.clear();
            self.session_manager.clear_current();
            self.status_message = "Chat cleared".to_string();
            return Ok(());
        }

        if self.is_streaming {
            if key.code == KeyCode::Esc {
                self.cancel_streaming();
            }
            return Ok(());
        }

        match key.code {
            KeyCode::Enter => {
                if !self.input.is_empty() {
                    let input = self.input.clone();
                    self.input.clear();
                    self.input_cursor = 0;

                    // Check for slash command
                    if let Some(cmd) = SlashCommand::parse(&input) {
                        self.execute_command(cmd).await?;
                    } else {
                        self.send_message(&input).await?;
                    }
                }
            }
            KeyCode::Char(c) => {
                self.input.insert(self.input_cursor, c);
                self.input_cursor += 1;
            }
            KeyCode::Backspace => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                    self.input.remove(self.input_cursor);
                }
            }
            KeyCode::Delete => {
                if self.input_cursor < self.input.len() {
                    self.input.remove(self.input_cursor);
                }
            }
            KeyCode::Left => {
                if self.input_cursor > 0 {
                    self.input_cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.input_cursor < self.input.len() {
                    self.input_cursor += 1;
                }
            }
            KeyCode::Home => {
                self.input_cursor = 0;
            }
            KeyCode::End => {
                self.input_cursor = self.input.len();
            }
            KeyCode::Up => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
            KeyCode::Down => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
            KeyCode::Esc => {
                self.should_quit = true;
            }
            _ => {}
        }
        Ok(())
    }

    /// Send a chat message and start streaming the response
    async fn send_message(&mut self, content: &str) -> Result<()> {
        // Add user message
        let user_msg = ChatMessage::user(content);
        self.messages.push(user_msg.clone());
        self.session_manager.add_message_to_current(user_msg);

        // Real-Time SQLite Relational History Persistence
        let s_id = self.session_manager.current_session()
            .map(|s| s.id.clone())
            .unwrap_or_else(|| "default".to_string());
        let _ = self.project_context.memory.record_history(&s_id, "user", content);

        // Build messages with system prompt and local instruction files (Feature 10: VYCODE.md)
        let base_sys = providers::build_system_prompt(self.project_context.get_summary());
        
        let mut custom_instructions = String::new();
        if let Ok(content) = std::fs::read_to_string("VYCODE.md") {
            custom_instructions = format!(
                "\n\n⚠️ IMPORTANT PROJECT-SPECIFIC DIRECTIVES (From VYCODE.md):\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n{}\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
                content
            );
        }

        // High-Performance Feature: Deep Persistent Project Memory Facts Injection
        let memory_instructions = self.project_context.memory.get_prompt_injection();

        let system_prompt = format!("{}{}{}", base_sys, custom_instructions, memory_instructions);
        let mut api_messages = vec![ChatMessage::system(&system_prompt)];
        api_messages.extend(self.messages.clone());

        // Start streaming
        self.is_streaming = true;
        self.streaming_text.clear();
        self.spinner_frame = 0;
        self.status_message = "Generating response...".to_string();

        let (tx, rx) = mpsc::unbounded_channel();
        self.stream_rx = Some(rx);

        if let Some(_provider) = &self.provider {
            let config = self.config.clone();
            let messages = api_messages;

            // We need to create a new provider for the spawned task
            let provider_type = self.provider_type.clone();
            let config_clone = config.clone();

            tokio::spawn(async move {
                let provider = providers::create_provider(&provider_type, &config_clone);
                let max_retries = config.max_retries;

                for attempt in 0..=max_retries {
                    match provider.stream_chat(&messages, &config, tx.clone()).await {
                        Ok(_) => break,
                        Err(e) => {
                            if attempt < max_retries {
                                let delay = config.retry_delay_ms * (attempt as u64 + 1);
                                let _ = tx.send(StreamEvent::Error(format!(
                                    "Retry {}/{max_retries}: {e}",
                                    attempt + 1
                                )));
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay))
                                    .await;
                            } else {
                                let _ = tx.send(StreamEvent::Error(format!(
                                    "Failed after {max_retries} retries: {e}"
                                )));
                                let _ = tx.send(StreamEvent::Done);
                            }
                        }
                    }
                }
            });
        } else {
            self.add_system_message("No provider configured. Use /provider to set up.");
            self.is_streaming = false;
        }

        Ok(())
    }

    /// Handle a streaming event from the provider
    fn handle_stream_event(&mut self, event: StreamEvent) {
        match event {
            StreamEvent::Chunk(text) => {
                self.streaming_text.push_str(&text);
            }
            StreamEvent::Done => {
                // Finalize the assistant message
                if !self.streaming_text.is_empty() {
                    let assistant_msg = ChatMessage::assistant(&self.streaming_text);
                    self.messages.push(assistant_msg.clone());
                    self.session_manager.add_message_to_current(assistant_msg);

                    // Real-Time SQLite Relational History Persistence
                    let s_id = self.session_manager.current_session()
                        .map(|s| s.id.clone())
                        .unwrap_or_else(|| "default".to_string());
                    let _ = self.project_context.memory.record_history(&s_id, "assistant", &self.streaming_text);

                    // Sovereign Agent Action Queue Interceptor
                    if self.agent_chain_active {
                        let text = self.streaming_text.clone();
                        
                        if let Some(start) = text.find("[EXEC: ") {
                            let sub = &text[start + 7..];
                            if let Some(end) = sub.find(']') {
                                self.pending_agent_action = Some(format!("exec:{}", &sub[..end].trim()));
                            }
                        } else if let Some(start) = text.find("[WRITE: ") {
                            let sub = &text[start + 8..];
                            if let Some(end) = sub.find(']') {
                                self.pending_agent_action = Some(format!("write:{}", &sub[..end].trim()));
                            }
                        } else if let Some(start) = text.find("[DONE: ") {
                            let sub = &text[start + 7..];
                            if let Some(end) = sub.find(']') {
                                let summary = &sub[..end].trim();
                                self.add_system_message(&format!("✅🏆 **SOVEREIGN AGENT COMPLETION EVENT:**\n{summary}"));
                            }
                            self.agent_chain_active = false;
                            self.agent_chain_steps = 0;
                            self.pending_agent_action = None;
                        } else {
                            // Auto-safety: if AI stops returning EXEC or DONE tags, automatically disengage
                            self.agent_chain_active = false;
                            self.add_system_message("ℹ️ *Sovereign Loop naturally dissolved: No further active schema commands provided.*");
                        }
                    }
                }
                self.streaming_text.clear();
                self.is_streaming = false;
                self.stream_rx = None;
                self.status_message = "Ready".to_string();

                // Auto-save session
                let _ = self.session_manager.save();
            }
            StreamEvent::Error(msg) => {
                self.status_message = format!("Error: {msg}");
                // Don't stop streaming on transient errors (retries)
                if msg.starts_with("Failed after") {
                    self.is_streaming = false;
                    self.stream_rx = None;
                    self.add_system_message(&format!("⚠️ {msg}"));
                }
            }
        }
    }

    /// Cancel the current streaming response
    fn cancel_streaming(&mut self) {
        if !self.streaming_text.is_empty() {
            let partial = ChatMessage::assistant(&format!("{} [cancelled]", self.streaming_text));
            self.messages.push(partial.clone());
            self.session_manager.add_message_to_current(partial);
        }
        self.streaming_text.clear();
        self.is_streaming = false;
        self.stream_rx = None;
        self.status_message = "Cancelled".to_string();
    }

    /// Execute a slash command
    async fn execute_command(&mut self, cmd: SlashCommand) -> Result<()> {
        match cmd {
            SlashCommand::Help => {
                let help = CommandHandler::help_text();
                self.add_system_message(&help);
            }
            SlashCommand::Model(name) => {
                if let Some(model) = name {
                    self.config.set_model(&model);
                    self.config.save()?;
                    // AI MULTI-MODEL HOT SWAP: Recreate active provider immediately without restart!
                    self.provider = Some(providers::create_provider(
                        &self.provider_type,
                        &self.config,
                    ));
                    self.status_message = format!("Model hot-swapped to: {model}");
                    self.add_system_message(&format!("🔄 [HOT SWAP] Provider re-initialized. Active model changed to: **{model}**"));
                } else {
                    let current = self.config.model.as_deref().unwrap_or("N/A");
                    self.add_system_message(&format!("Current active model: {current}"));
                }
            }
            SlashCommand::Provider => {
                self.screen = AppScreen::ProviderSelect;
                self.input.clear();
                self.input_cursor = 0;
            }
            SlashCommand::ApiKey => {
                self.screen = AppScreen::ApiKeyInput;
                self.input.clear();
                self.input_cursor = 0;
            }
            SlashCommand::Scan => {
                self.status_message = "Scanning project...".to_string();
                match cmd_handler::scan_project(None) {
                    Ok(result) => {
                        self.project_context.index();
                        self.add_system_message(&result);
                        self.status_message = "Scan complete".to_string();
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Scan failed: {e}"));
                    }
                }
            }
            SlashCommand::Graph => {
                self.status_message = "Building dependency graph...".to_string();
                match cmd_handler::visual_graph() {
                    Ok(graph) => {
                        self.add_system_message(&graph);
                        self.status_message = "Visual graph loaded".to_string();
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Graph generation failed: {e}"));
                    }
                }
            }
            SlashCommand::Heal(_file) => {
                self.status_message = "Checking compiler health...".to_string();
                match cmd_handler::get_compiler_errors().await {
                    Ok(report) => {
                        self.add_system_message(&report);
                        if report.contains("Compiler Error Detected") {
                            let prompt = format!(
                                "{} \n\nINSTRUCTION: Parse the above compiler error diagnostics and the current context. Provide the solution code directly. Return ONLY the corrected block or file contents in Markdown.",
                                report
                            );
                            self.send_message(&prompt).await?;
                        } else {
                            self.status_message = "System Healthy".to_string();
                        }
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Self-healing scan failed: {e}"));
                    }
                }
            }
            SlashCommand::Chain(task) => {
                self.status_message = "Initializing Sovereign Agent Loop...".to_string();
                self.add_system_message(&format!("🤖💎 **SOVEREIGN AGENT PROTOCOL ENGAGED:** \"{task}\"\n*Activating recursive autonomous execution loop...*"));
                
                // Engage internal loop registers
                self.agent_chain_active = true;
                self.agent_chain_steps = 0;
                self.pending_agent_action = None;

                let agent_prompt = format!(
                    "CRITICAL DIRECTIVE: YOU ARE NOW OPERATING AS A FULLY AUTONOMOUS SOVEREIGN AGENT (v3.0).\n\
                    Objective: \"{}\"\n\n\
                    IMPORTANT: The developer platform has granted you DIRECT NATIVE EXECUTION PRIVILEGES.\n\
                    Whenever you output an execution command, the VyCode Engine will IMMEDIATELY execute it natively and inject the terminal stdout/stderr directly back into your prompt automatically!\n\n\
                    🔥 NATIVE COMMAND SCHEMAS:\n\
                    - `[EXEC: <shell command>]` -> Executes shell command, installs packages, runs compilers/scripts.\n\
                    - `[WRITE: <filepath>|<filecontent>]` -> Safely writes or patches code files natively.\n\
                    - `[DONE: <result>]` -> Call ONLY when the task objective is 100% verified and complete.\n\n\
                    Analyze the objective, formulate the execution steps, and initiate STEP 1 immediately by outputting the relevant schema command! (Limit 1 action per turn).",
                    task
                );
                self.send_message(&agent_prompt).await?;
            }
            SlashCommand::Git(args) => {
                self.status_message = format!("Git: running 'git {}'...", args);
                let full_cmd = format!("git {}", args);
                match cmd_handler::exec_command(&full_cmd).await {
                    Ok(output) => {
                        self.add_system_message(&output);
                        self.status_message = "Git command completed".to_string();
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Git operation failed: {e}"));
                        self.status_message = "Git failure".to_string();
                    }
                }
            }
            SlashCommand::Memory => {
                let stats = self.project_context.memory.display_stats();
                self.add_system_message(&stats);
            }
            SlashCommand::Remember(fact) => {
                match self.project_context.memory.remember(&fact, None) {
                    Ok(id) => {
                        self.add_system_message(&format!(
                            "🧠 **Knowledge Stored Successfully!**\nFact registered in local Project Memory under ID: `{id}`.\nThis fact will now be injected into all future system prompts in this workspace!"
                        ));
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Failed to save memory fact: {e}"));
                    }
                }
            }
            SlashCommand::Forget(id) => {
                match self.project_context.memory.forget(&id) {
                    Ok(true) => {
                        self.add_system_message(&format!("🗑️ Memory Fact `{id}` wiped successfully. It is now forgotten."));
                    }
                    Ok(false) => {
                        self.add_system_message(&format!("⚠️ No memory fact with ID `{id}` was found in this workspace."));
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Failed to purge memory: {e}"));
                    }
                }
            }
            SlashCommand::Docs(url) => {
                self.status_message = format!("Downloading docs from {}...", url);
                self.add_system_message(&format!("🌐 **Retrieving Documentation Context** from `{url}`..."));
                
                use crate::tools::ToolRouter;
                match ToolRouter::route_command("docs", &[&url]).await {
                    Ok(content) => {
                        // Inject raw text to workspace chat context
                        self.add_system_message(&content);
                        
                        // Automatically prompt the AI to parse the newly ingested documentation
                        let feed_prompt = format!(
                            "CRITICAL DOCS EVENT: A direct documentation manual was downloaded and provided below:\n\n{}\n\nINSTRUCTION: Act as the technical expert for this documentation. Confirm you have fully read it and provide a concise 3-to-5 bullet-point technical summary for the developer, detailing the API methods or design patterns demonstrated above.",
                            content
                        );
                        self.send_message(&feed_prompt).await?;
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Documentation fetch failed: {e}"));
                        self.status_message = "Docs fetch failed".to_string();
                    }
                }
            }
            SlashCommand::TgSetup(token, chat_id) => {
                self.config.telegram_token = Some(token);
                self.config.telegram_chat_id = Some(chat_id);
                match self.config.save() {
                    Ok(_) => {
                        self.add_system_message("✅ **Telegram Bot Configured!**\nCredentials saved and secured in config layer. Try sending a test via: `/tg Hello from VyCode TUI!`");
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Failed to save Telegram configs: {e}"));
                    }
                }
            }
            SlashCommand::Tg(message) => {
                let token = self.config.telegram_token.clone();
                let chat_id = self.config.telegram_chat_id.clone();
                
                if let (Some(tok), Some(cid)) = (token, chat_id) {
                    self.status_message = "Broadcasting to Telegram...".to_string();
                    use crate::tools::ToolRouter;
                    match ToolRouter::route_command("telegram", &[&tok, &cid, &message]).await {
                        Ok(status) => {
                            self.add_system_message(&status);
                            self.status_message = "Telegram sent".to_string();
                        }
                        Err(e) => {
                            self.add_system_message(&format!("❌ Telegram broadcast failed: {e}"));
                            self.status_message = "Broadcast failure".to_string();
                        }
                    }
                } else {
                    self.add_system_message("⚠️ **Telegram is not configured.**\nRun `/tg-setup <bot_token> <chat_id>` first to establish connection rings!");
                }
            }
            SlashCommand::DiscordSetup(webhook_url) => {
                self.config.discord_webhook = Some(webhook_url);
                match self.config.save() {
                    Ok(_) => {
                        self.add_system_message("✅ **Discord Webhook Registered!**\nSaved and bound in configurations. Broadcast tests using `/dc Hello Discord Channel!`");
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Failed to save Discord configurations: {e}"));
                    }
                }
            }
            SlashCommand::Dc(message) => {
                if let Some(webhook) = &self.config.discord_webhook {
                    self.status_message = "Dispatching to Discord...".to_string();
                    let url = webhook.clone();
                    use crate::tools::ToolRouter;
                    match ToolRouter::route_command("discord", &[&url, &message]).await {
                        Ok(status) => {
                            self.add_system_message(&status);
                            self.status_message = "Discord sent".to_string();
                        }
                        Err(e) => {
                            self.add_system_message(&format!("❌ Discord dispatch failed: {e}"));
                            self.status_message = "Discord failure".to_string();
                        }
                    }
                } else {
                    self.add_system_message("⚠️ **Discord Webhook is not registered.**\nMount yours via: `/discord-setup <webhook_url>` to enable broadcasts!");
                }
            }
            SlashCommand::OmniSetup(key, value) => {
                let key_normalized = key.to_lowercase();
                let mut updated = true;
                match key_normalized.as_str() {
                    "slack" => self.config.omni.slack_webhook = Some(value),
                    "teams" => self.config.omni.teams_webhook = Some(value),
                    "matrix_hs" => self.config.omni.matrix_homeserver = Some(value),
                    "matrix_room" => self.config.omni.matrix_room_id = Some(value),
                    "matrix_token" => self.config.omni.matrix_access_token = Some(value),
                    "signal_url" => self.config.omni.signal_api_url = Some(value),
                    "signal_rec" => self.config.omni.signal_recipient = Some(value),
                    "whatsapp_url" => self.config.omni.whatsapp_api_url = Some(value),
                    "whatsapp_token" => self.config.omni.whatsapp_token = Some(value),
                    "email_url" => self.config.omni.email_gateway_url = Some(value),
                    "email_rec" => self.config.omni.email_recipient = Some(value),
                    "sms_url" => self.config.omni.sms_gateway_url = Some(value),
                    "sms_rec" => self.config.omni.sms_recipient = Some(value),
                    _ => {
                        updated = false;
                        self.add_system_message(&format!("❌ Unknown configuration key: `{key}`. Use standard keys (e.g., slack, teams, matrix_hs, etc.)."));
                    }
                }

                if updated {
                    match self.config.save() {
                        Ok(_) => {
                            self.add_system_message(&format!("✅ **Omni-Channel Securely Bound!**\nSuccessfully updated configuration key `{key}` in local storage!"));
                        }
                        Err(e) => {
                            self.add_system_message(&format!("❌ Failed to persist config updates: {e}"));
                        }
                    }
                }
            }
            SlashCommand::Broadcast(channel, message) => {
                let chan_normalized = channel.to_lowercase();
                self.status_message = format!("Broadcasting event to {}...", chan_normalized);
                
                use crate::tools::ToolRouter;
                let result = match chan_normalized.as_str() {
                    "slack" => {
                        if let Some(url) = &self.config.omni.slack_webhook {
                            ToolRouter::route_command("slack", &[url, &message]).await
                        } else {
                            Err(anyhow::anyhow!("Slack configuration missing. Configure via `/omni-setup slack <webhook>`"))
                        }
                    }
                    "teams" => {
                        if let Some(url) = &self.config.omni.teams_webhook {
                            ToolRouter::route_command("teams", &[url, &message]).await
                        } else {
                            Err(anyhow::anyhow!("Teams configuration missing. Configure via `/omni-setup teams <webhook>`"))
                        }
                    }
                    "matrix" => {
                        if let (Some(hs), Some(room), Some(tok)) = (&self.config.omni.matrix_homeserver, &self.config.omni.matrix_room_id, &self.config.omni.matrix_access_token) {
                            ToolRouter::route_command("matrix", &[hs, room, tok, &message]).await
                        } else {
                            Err(anyhow::anyhow!("Matrix credentials missing. (Need matrix_hs, matrix_room, matrix_token)"))
                        }
                    }
                    "signal" => {
                        if let (Some(url), Some(rec)) = (&self.config.omni.signal_api_url, &self.config.omni.signal_recipient) {
                            ToolRouter::route_command("signal", &[url, rec, &message]).await
                        } else {
                            Err(anyhow::anyhow!("Signal credentials missing. (Need signal_url, signal_rec)"))
                        }
                    }
                    "whatsapp" => {
                        if let (Some(url), Some(tok)) = (&self.config.omni.whatsapp_api_url, &self.config.omni.whatsapp_token) {
                            ToolRouter::route_command("whatsapp", &[url, tok, &message]).await
                        } else {
                            Err(anyhow::anyhow!("WhatsApp credentials missing. (Need whatsapp_url, whatsapp_token)"))
                        }
                    }
                    "email" => {
                        if let (Some(url), Some(rec)) = (&self.config.omni.email_gateway_url, &self.config.omni.email_recipient) {
                            ToolRouter::route_command("email", &[url, rec, &message]).await
                        } else {
                            Err(anyhow::anyhow!("Email credentials missing. (Need email_url, email_rec)"))
                        }
                    }
                    "sms" => {
                        if let (Some(url), Some(rec)) = (&self.config.omni.sms_gateway_url, &self.config.omni.sms_recipient) {
                            ToolRouter::route_command("sms", &[url, rec, &message]).await
                        } else {
                            Err(anyhow::anyhow!("SMS credentials missing. (Need sms_url, sms_rec)"))
                        }
                    }
                    _ => Err(anyhow::anyhow!("Target channel `{}` is not registered in Omni global dispatcher.", channel)),
                };

                match result {
                    Ok(status) => {
                        self.add_system_message(&status);
                        self.status_message = "Omni-Broadcast Success".to_string();
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Omni-Channel dispatch failed: {e}"));
                        self.status_message = "Omni-Broadcast Error".to_string();
                    }
                }
            }
            SlashCommand::InfraSetup(key, value) => {
                let key_norm = key.to_lowercase();
                let mut updated = true;
                match key_norm.as_str() {
                    "github_token" | "gh_token" => self.config.infra.github_token = Some(value),
                    "github_repo" | "gh_repo" => self.config.infra.github_repo = Some(value),
                    "ssh_host" => self.config.infra.ssh_host = Some(value),
                    "ssh_user" => self.config.infra.ssh_user = Some(value),
                    "ssh_port" => self.config.infra.ssh_port = Some(value),
                    "hass_url" => self.config.infra.hass_url = Some(value),
                    "hass_token" => self.config.infra.hass_token = Some(value),
                    "db_engine" => self.config.infra.db_engine = Some(value),
                    "db_uri" => self.config.infra.db_uri = Some(value),
                    "mcp_url" | "mcp_server" => self.config.infra.mcp_server_url = Some(value),
                    _ => {
                        updated = false;
                        self.add_system_message(&format!("❌ Unknown configuration key: `{key}`. Supported: gh_token, gh_repo, ssh_host, ssh_user, ssh_port, hass_url, hass_token, db_engine, db_uri, mcp_url"));
                    }
                }

                if updated {
                    match self.config.save() {
                        Ok(_) => {
                            self.add_system_message(&format!("✅ **Infrastructure Credentials Bound!**\nSuccessfully saved `{key}` configurations securely."));
                        }
                        Err(e) => {
                            self.add_system_message(&format!("❌ Failed to write config storage: {e}"));
                        }
                    }
                }
            }
            SlashCommand::Infra(subsystem, params) => {
                let sys_norm = subsystem.to_lowercase();
                self.status_message = format!("Invoking {} infrastructure system...", sys_norm);
                
                use crate::tools::ToolRouter;
                // Parse operational sub-arguments
                let op_args: Vec<&str> = params.split_whitespace().collect();

                let result = match sys_norm.as_str() {
                    "github" | "gh" => {
                        if let (Some(tok), Some(repo)) = (&self.config.infra.github_token, &self.config.infra.github_repo) {
                            let op = op_args.get(0).unwrap_or(&"issues");
                            let pay = if op_args.len() > 1 { &params[params.find(op_args[1]).unwrap_or(0)..] } else { "" };
                            ToolRouter::route_command("github", &[tok, repo, op, pay]).await
                        } else {
                            Err(anyhow::anyhow!("GitHub configs (gh_token, gh_repo) are missing! Run `/infra-setup`."))
                        }
                    }
                    "browser" | "chrome" => {
                        let op = op_args.get(0).unwrap_or(&"screenshot");
                        let url = op_args.get(1).unwrap_or(&"https://github.com");
                        let out = if op_args.len() > 2 { op_args[2] } else { "" };
                        ToolRouter::route_command("browser", &[op, url, out]).await
                    }
                    "docker" => {
                        let op = op_args.get(0).unwrap_or(&"ps");
                        let arg = if op_args.len() > 1 { op_args[1] } else { "" };
                        ToolRouter::route_command("docker", &[op, arg]).await
                    }
                    "ssh" => {
                        if let (Some(host), Some(user)) = (&self.config.infra.ssh_host, &self.config.infra.ssh_user) {
                            let port = self.config.infra.ssh_port.as_deref().unwrap_or("22");
                            let cmd = if !op_args.is_empty() { &params } else { "uname -a" };
                            ToolRouter::route_command("ssh", &[host, user, port, cmd]).await
                        } else {
                            Err(anyhow::anyhow!("SSH configs (ssh_host, ssh_user) missing!"))
                        }
                    }
                    "hass" | "homeassistant" => {
                        if let (Some(url), Some(tok)) = (&self.config.infra.hass_url, &self.config.infra.hass_token) {
                            let op = op_args.get(0).unwrap_or(&"states");
                            let ent = if op_args.len() > 1 { op_args[1] } else { "" };
                            ToolRouter::route_command("hass", &[url, tok, op, ent]).await
                        } else {
                            Err(anyhow::anyhow!("HASS configuration missing!"))
                        }
                    }
                    "db" | "database" | "sql" => {
                        if let (Some(eng), Some(uri)) = (&self.config.infra.db_engine, &self.config.infra.db_uri) {
                            let sql = if !op_args.is_empty() { &params } else { "SELECT 1;" };
                            ToolRouter::route_command("db", &[eng, uri, sql]).await
                        } else {
                            Err(anyhow::anyhow!("Database connection credentials missing!"))
                        }
                    }
                    "mcp" => {
                        if let Some(srv) = &self.config.infra.mcp_server_url {
                            let op = op_args.get(0).unwrap_or(&"list");
                            if op == &"list" {
                                ToolRouter::route_command("mcp", &[srv, "list"]).await
                            } else {
                                let tool = op_args.get(1).ok_or_else(|| anyhow::anyhow!("Tool name required!"))?;
                                let args_raw = if op_args.len() > 2 { &params[params.find(op_args[2]).unwrap_or(0)..] } else { "{}" };
                                ToolRouter::route_command("mcp", &[srv, "call", tool, args_raw]).await
                            }
                        } else {
                            Err(anyhow::anyhow!("MCP Server URL missing! Configure via `/infra-setup mcp_url <url>`"))
                        }
                    }
                    _ => Err(anyhow::anyhow!("Subsystem `{}` is not an enterprise automation target.", subsystem)),
                };

                match result {
                    Ok(resp) => {
                        self.add_system_message(&resp);
                        self.status_message = format!("{} successful", sys_norm);
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Infrastructure action failed: {e}"));
                        self.status_message = "Action failed".to_string();
                    }
                }
            }
            SlashCommand::Fix(file) => {
                let prompt = if let Some(f) = &file {
                    match cmd_handler::read_file(f) {
                        Ok(content) => format!(
                            "Please analyze and fix any bugs in this code:\n\n{content}"
                        ),
                        Err(e) => {
                            self.add_system_message(&format!("❌ {e}"));
                            return Ok(());
                        }
                    }
                } else {
                    "Please analyze and fix any bugs in the code I've shared.".to_string()
                };
                self.send_message(&prompt).await?;
            }
            SlashCommand::Explain(file) => {
                let prompt = if let Some(f) = &file {
                    match cmd_handler::read_file(f) {
                        Ok(content) => format!(
                            "Please explain this code in detail:\n\n{content}"
                        ),
                        Err(e) => {
                            self.add_system_message(&format!("❌ {e}"));
                            return Ok(());
                        }
                    }
                } else {
                    "Please explain the code I've shared.".to_string()
                };
                self.send_message(&prompt).await?;
            }
            SlashCommand::Clear => {
                self.messages.clear();
                self.session_manager.clear_current();
                self.status_message = "Chat cleared".to_string();
            }
            SlashCommand::Exit => {
                self.should_quit = true;
            }
            SlashCommand::Read(path) => match cmd_handler::read_file(&path) {
                Ok(content) => {
                    self.add_system_message(&content);
                }
                Err(e) => {
                    self.add_system_message(&format!("❌ {e}"));
                }
            },
            SlashCommand::Write(path, content) => match cmd_handler::write_file(&path, &content) {
                Ok(msg) => {
                    self.add_system_message(&msg);
                }
                Err(e) => {
                    self.add_system_message(&format!("❌ {e}"));
                }
            },
            SlashCommand::Exec(cmd_str) => {
                self.status_message = format!("Running: {cmd_str}");
                match cmd_handler::exec_command(&cmd_str).await {
                    Ok(output) => {
                        self.add_system_message(&output);
                        self.status_message = "Command complete".to_string();
                    }
                    Err(e) => {
                        self.add_system_message(&format!("❌ Command failed: {e}"));
                        self.status_message = "Command failed".to_string();
                    }
                }
            }
            SlashCommand::Session(name) => {
                if let Some(name) = name {
                    if self.session_manager.switch_session(&name) {
                        self.messages = self
                            .session_manager
                            .current_session()
                            .map(|s| s.messages.clone())
                            .unwrap_or_default();
                        self.add_system_message(&format!("✅ Switched to session: {name}"));
                    } else {
                        self.session_manager.create_session(&name);
                        self.messages.clear();
                        self.add_system_message(&format!("✅ Created new session: {name}"));
                    }
                    self.session_manager.save()?;
                } else {
                    let names = self.session_manager.session_names();
                    let active = self
                        .session_manager
                        .current_session()
                        .map(|s| s.name.clone())
                        .unwrap_or_default();
                    let list: Vec<String> = names
                        .iter()
                        .map(|n| {
                            if *n == active {
                                format!("  ▸ {n} (active)")
                            } else {
                                format!("    {n}")
                            }
                        })
                        .collect();
                    self.add_system_message(&format!(
                        "Sessions:\n{}",
                        list.join("\n")
                    ));
                }
            }
            SlashCommand::Export => {
                if let Some(md) = self.session_manager.export_current() {
                    let session_name = self
                        .session_manager
                        .current_session()
                        .map(|s| s.name.clone())
                        .unwrap_or_else(|| "session".to_string());
                    let filename = format!("vycode_export_{session_name}.md");
                    match cmd_handler::write_file(&filename, &md) {
                        Ok(msg) => self.add_system_message(&msg),
                        Err(e) => self.add_system_message(&format!("❌ Export failed: {e}")),
                    }
                }
            }
            SlashCommand::Theme(theme) => {
                if let Some(t) = theme {
                    self.config.theme.accent_color = t.clone();
                    self.config.save()?;
                    self.add_system_message(&format!("✅ Theme set to: {t}"));
                } else {
                    self.add_system_message(&format!(
                        "Current theme: {}",
                        self.config.theme.accent_color
                    ));
                }
            }
            SlashCommand::Unknown(cmd) => {
                self.add_system_message(&format!(
                    "❌ Unknown command: /{cmd}\nType /help for available commands."
                ));
            }
        }
        Ok(())
    }

    /// Add a system message to the chat
    fn add_system_message(&mut self, content: &str) {
        let msg = ChatMessage::assistant(content);
        self.messages.push(msg.clone());
        self.session_manager.add_message_to_current(msg);
    }

    /// Runs the application in non-interactive CLI mode (one-shot)
    /// Highly useful for shell pipelines, one-shot prompt executions and CI integrations.
    pub async fn run_one_shot(&mut self, query: &str) -> Result<()> {
        // Ensure provider is present
        if self.provider.is_none() {
            return Err(anyhow::anyhow!("Provider not configured! Run 'vycode' once in interactive mode to set up API keys."));
        }

        // Send user query
        self.send_message(query).await?;

        use std::io::Write;
        let mut stdout = std::io::stdout();

        println!("\n🤖 [VyCode Streaming Engine v2.0] ━━━━━━━━━━━━━━━━━━━━━━━━");

        // Stream output natively to terminal stdout
        let mut stream_completed = false;
        if let Some(ref mut rx) = self.stream_rx {
            while let Some(event) = rx.recv().await {
                match event {
                    StreamEvent::Chunk(chunk) => {
                        print!("{}", chunk);
                        let _ = stdout.flush();
                    }
                    StreamEvent::Done => {
                        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ ✅");
                        stream_completed = true;
                        break;
                    }
                    StreamEvent::Error(err) => {
                        eprintln!("\n⚠️ [Pipeline Error]: {}", err);
                        if err.starts_with("Failed after") {
                            break;
                        }
                    }
                }
            }
        }

        if !stream_completed {
            // Fallback loop just in case the stream hung but channels closed
            println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ ✅");
        }
        
        // Clean up state and save session before exit
        self.stream_rx = None;
        self.is_streaming = false;
        let _ = self.session_manager.save();
        Ok(())
    }
}

/// Check if the config requires initial setup
fn config_needs_setup(config: &AppConfig) -> bool {
    !config.is_configured()
}
