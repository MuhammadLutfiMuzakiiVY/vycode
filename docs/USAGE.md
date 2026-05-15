# VyCode Usage Guide

## Starting VyCode

```bash
# Basic startup
vycode

# With project path
vycode --path /path/to/project

# Skip splash screen
vycode --no-splash

# Reset configuration
vycode --reset-config
```

## First-Time Setup

On first launch, VyCode will guide you through:

1. **Provider Selection** — Choose from 8 supported AI providers
2. **API Key** — Enter your provider's API key (stored encrypted)
3. **Model** — Confirm or change the default model name

## Chat Interface

Once configured, you'll see the main chat interface:

```
┌─ VyCode ──────────────────────────────────┐
│ Provider: OpenAI │ Model: gpt-4o          │
├───────────────────────────────────────────┤
│                                           │
│  Welcome to VyCode! 🚀                   │
│  Type a message or /help for commands     │
│                                           │
├───────────────────────────────────────────┤
│ ▸ _                                       │
└───────────────────────────────────────────┘
```

## Commands Reference

### `/help`
Display all available commands and shortcuts.

### `/model [name]`
View or change the current model.
```
/model                  # Show current model
/model gpt-4o-mini      # Switch to gpt-4o-mini
```

### `/provider`
Open the provider selection screen to switch AI backends.

### `/apikey`
Update the API key for the current provider.

### `/scan`
Scan the current project directory and display a file tree.
Automatically indexes project context for smarter AI responses.

### `/fix [file]`
Ask the AI to analyze and fix bugs in code.
```
/fix src/main.rs        # Fix specific file
/fix                    # Fix code from recent context
```

### `/explain [file]`
Get a detailed explanation of code.
```
/explain src/app.rs     # Explain specific file
/explain                # Explain recent code
```

### `/read <file>`
Read and display a file's contents.
```
/read Cargo.toml
```

### `/write <file> <content>`
Write content to a file.
```
/write output.txt Hello World
```

### `/exec <command>`
Execute a shell command and display output.
```
/exec cargo build
/exec dir               # Windows
/exec ls -la            # Linux/macOS
```

### `/session [name]`
Manage chat sessions.
```
/session                # List all sessions
/session debug          # Switch to or create "debug" session
```

### `/export`
Export the current session as a markdown file.

### `/clear`
Clear all messages in the current chat.

### `/exit`
Save and exit VyCode.

## Windows Usage

```powershell
# Install
cargo install --path .

# Run
vycode

# Run with project
vycode --path C:\Users\you\project

# The binary is at: %USERPROFILE%\.cargo\bin\vycode.exe
```

## Linux / macOS Usage

```bash
# Install
cargo install --path .

# Run
vycode

# Run with project
vycode --path ~/projects/myapp

# The binary is at: ~/.cargo/bin/vycode
```

## Termux (Android) Usage

VyCode berjalan dengan sangat ringan dan sempurna di Android via Termux!

```bash
# 1. Update and install dependencies
pkg update && pkg upgrade
pkg install rust git binutils

# 2. Clone repository
git clone https://github.com/MuhammadLutfiMuzakiiVY/vycode.git
cd vycode

# 3. Build & Install
cargo install --path .

# 4. Run directly on your phone!
## Apple Mobile (iOS / iPadOS) Usage

VyCode dapat digunakan di ekosistem Apple Mobile seperti iPhone dan iPad melalui emulator terminal seperti **iSH Shell** (tersedia di App Store).

```bash
# Buka aplikasi iSH di iPhone/iPad Anda
# 1. Install Rust via package manager Alpine
apk add rust cargo git gcc musl-dev

# 2. Clone repository & Install
git clone https://github.com/MuhammadLutfiMuzakiiVY/vycode.git
cd vycode
cargo install --path .

# 3. Jalankan asisten AI
vycode
```

*(Alternatif: Gunakan aplikasi **Blink Shell** untuk me-remote PC Anda via SSH dan membuka VyCode).*

## Smart TV Usage (Android TV / webOS)

Anda bahkan dapat melakukan *coding session* langsung dari layar ruang tamu Anda! VyCode mendukung Smart TV asalkan perangkat tersebut memiliki akses *shell*.

**Untuk Android TV:**
1. Install **Termux** (via Sideload/F-Droid).
2. Sambungkan keyboard bluetooth ke Smart TV.
3. Ikuti panduan instalasi **Termux (Android)** di atas.

**Untuk WebOS / OS Lainnya:**
Jika TV Anda di-*root* atau dalam *Developer Mode*, Anda bisa me-remote via SSH (menggunakan Termius, dll) dan menjalankan VyCode di layar besar.

## Tips

- Use **Ctrl+C** to cancel streaming or exit
- Use **Ctrl+L** to quickly clear the chat
- Use **↑/↓** arrows to scroll through message history
- Project scanning happens automatically on startup (configurable)
- Sessions are auto-saved — your chat history persists between runs
- Use `/exec` to run builds and tests directly from VyCode
