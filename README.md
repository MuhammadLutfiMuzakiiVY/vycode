<div align="center">
  <!-- Letakkan gambar logo yang Anda unggah ke folder docs/vycode-logo.png -->
  <img src="docs/vycode-logo.png" alt="VyCode Logo" width="250" />

  # VyCode — AI Coding Terminal Assistant
  
  **Created by [Muhammad Lutfi Muzaki](https://github.com/MuhammadLutfiMuzakiiVY)**

  [![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![Status](https://img.shields.io/badge/Status-Production--Ready-success.svg)]()
</div>

> A modern, production-ready terminal AI coding assistant built in Rust, featuring a premium orange dark-mode TUI, multi-agent provider system, and seamless project context indexing.

---

## ✨ Features

### 🌍 Universal Cross-Platform & Omnipresent
- **Any OS, Anywhere**: Works flawlessly on **Windows**, **macOS**, **Linux**, **Termux** (Android), **Apple Mobile / iOS** (via iSH / a-Shell), and even **Smart TVs** (Android TV / webOS via terminal emulator or SSH).
- **Terminal Agnostic**: Built with `crossterm` to support Command Prompt, PowerShell, GNOME Terminal, iTerm2, Termux, and mobile SSH sessions alike.
- **Auto-Pathing**: Configuration paths automatically adapt to your OS architecture.

### Core Features
- 🤖 **Multi-Provider AI** — OpenAI, Anthropic, Gemini, OpenRouter, Groq, DeepSeek, Ollama, Custom
- 🔄 **Streaming Responses** — Real-time token-by-token output
- 📁 **Project Scanning** — Automatic project structure analysis
- 📄 **File Read/Write** — Direct file manipulation from chat
- 🐛 **Bug Fixing** — AI-powered code analysis and repair
- 💡 **Code Explanation** — Detailed code walkthroughs
- ⚡ **Shell Commands** — Execute terminal commands inline
- 📝 **Multiple Sessions** — Create, switch, and export chat sessions
- 🔐 **Encrypted Config** — API keys stored with local encryption
- 🔁 **Auto Retry** — Configurable retry logic for API failures
- 🎨 **Beautiful TUI** — Dark mode with orange accent theme

### Premium Features
- 🔧 **Self-Healing Mode** — Auto-analyze and patch errors
- 📊 **Project Context Indexing** — AI-aware project understanding
- 📤 **Session Export** — Export chats to markdown
- 🎨 **Custom Themes** — Configurable color schemes
- 🔌 **Plugin Architecture** — Future extensibility
- 🎙️ **Voice Input Ready** — Prepared for voice integration

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/MuhammadLutfiMuzakiiVY/vycode.git
cd vycode

# Build and install
cargo install --path .
```

### Usage

```bash
# Start VyCode
vycode

# Start with project path
vycode --path /my/project

# Skip splash screen
vycode --no-splash

# Reset configuration
vycode --reset-config
```

---

## 🎮 Commands

| Command | Description |
|---------|-------------|
| `/help` | Show all available commands |
| `/model [name]` | Change or view current model |
| `/provider` | Switch AI provider |
| `/apikey` | Update API key |
| `/scan` | Scan project files |
| `/fix [file]` | Auto-fix code issues |
| `/explain [file]` | Explain code in detail |
| `/read <file>` | Read a file |
| `/write <file> <content>` | Write content to file |
| `/exec <command>` | Execute shell command |
| `/session [name]` | Switch or create session |
| `/export` | Export current session |
| `/clear` | Clear chat history |
| `/exit` | Exit VyCode |

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `Ctrl+C` | Cancel action / Exit |
| `Ctrl+L` | Clear chat |
| `↑/↓` | Scroll messages |
| `Esc` | Cancel streaming / Exit |

---

## 🔧 Supported Providers

| Provider | API Format | Default Model |
|----------|-----------|---------------|
| **OpenRouter** | OpenAI-compatible | `openai/gpt-4o` |
| **OpenAI** | Native | `gpt-4o` |
| **Anthropic** | Messages API | `claude-sonnet-4-20250514` |
| **Google Gemini** | Gemini API | `gemini-2.5-flash` |
| **Groq** | OpenAI-compatible | `llama-3.3-70b-versatile` |
| **DeepSeek** | OpenAI-compatible | `deepseek-chat` |
| **Ollama** | Ollama API | `llama3` |
| **Custom** | OpenAI-compatible | Configurable |

### Custom Provider Setup

Custom providers support:
- Configurable base URL
- API key authentication
- Custom model names
- OpenAI-compatible `/v1/chat/completions` endpoint

---

## 📁 Project Structure

```
vycode/
├── Cargo.toml              # Project manifest
├── README.md               # This file
├── docs/
│   ├── INSTALLATION.md     # Detailed installation guide
│   ├── BUILD.md            # Build instructions
│   └── USAGE.md            # Usage documentation
└── src/
    ├── main.rs             # Entry point & CLI
    ├── app.rs              # Core application logic
    ├── providers/
    │   ├── mod.rs          # Provider trait & factory
    │   ├── streaming.rs    # SSE stream parsers
    │   ├── openai.rs       # OpenAI provider
    │   ├── anthropic.rs    # Anthropic provider
    │   ├── gemini.rs       # Gemini provider
    │   ├── openrouter.rs   # OpenRouter provider
    │   ├── groq.rs         # Groq provider
    │   ├── deepseek.rs     # DeepSeek provider
    │   ├── ollama.rs       # Ollama provider
    │   └── custom.rs       # Custom endpoint provider
    ├── ui/
    │   ├── mod.rs          # UI module
    │   ├── theme.rs        # Color theme system
    │   ├── logo.rs         # ASCII branding
    │   ├── renderer.rs     # Screen rendering
    │   └── widgets.rs      # Reusable widgets
    ├── config/
    │   ├── mod.rs          # Configuration management
    │   └── encryption.rs   # API key encryption
    ├── commands/
    │   ├── mod.rs          # Slash command parsing
    │   └── handler.rs      # Command execution
    ├── context/
    │   └── mod.rs          # Project context indexing
    └── session/
        └── mod.rs          # Session management
```

---

## ⚙️ Configuration

Configuration is stored at:
- **Windows**: `%APPDATA%\vycode\config.json`
- **Linux/macOS**: `~/.config/vycode/config.json`

### Config Options

```json
{
  "provider_type": "OpenAI",
  "api_key_encrypted": "...",
  "model": "gpt-4o",
  "custom_base_url": null,
  "theme": {
    "accent_color": "orange",
    "dark_mode": true
  },
  "max_retries": 3,
  "retry_delay_ms": 1000,
  "max_context_tokens": 8192,
  "auto_scan": true
}
```

---

## 🛡️ Security

- API keys are encrypted before storage using XOR cipher with a fixed key
- No data is sent to any third party except the configured AI provider
- All processing is local except AI API calls

---

## 📜 License

MIT License — Created by **Muhammad Lutfi Muzaki**

GitHub: [https://github.com/MuhammadLutfiMuzakiiVY](https://github.com/MuhammadLutfiMuzakiiVY)
