# VyCode Installation Guide

## Prerequisites

### All Platforms
- **Rust toolchain** (1.75+): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Windows
- **Visual Studio Build Tools 2019+** with C++ workload
- Or install via: `winget install Microsoft.VisualStudio.2022.BuildTools`

### Linux (Debian/Ubuntu)
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

### Linux (Fedora/RHEL)
```bash
sudo dnf install gcc openssl-devel
```

### macOS
```bash
xcode-select --install
```

---

## Installation Methods

### Method 1: Install from Source (Recommended)

```bash
git clone https://github.com/MuhammadLutfiMuzakiiVY/vycode.git
cd vycode
cargo install --path .
```

This installs the `vycode` binary to `~/.cargo/bin/` (which should be in your PATH).

### Method 2: Build Only

```bash
git clone https://github.com/MuhammadLutfiMuzakiiVY/vycode.git
cd vycode
cargo build --release
```

The binary will be at `target/release/vycode` (or `vycode.exe` on Windows).

### Method 3: Direct Run

```bash
git clone https://github.com/MuhammadLutfiMuzakiiVY/vycode.git
cd vycode
cargo run --release
```

---

## Verify Installation

```bash
vycode --version
# Output: vycode 1.0.0

vycode --help
# Shows all CLI options
```

---

## First Run

1. Run `vycode`
2. Press any key on the splash screen
3. Select your AI provider (↑/↓ + Enter)
4. Enter your API key
5. Confirm the model name
6. Start chatting!

---

## Updating

```bash
cd vycode
git pull
cargo install --path . --force
```

---

## Uninstalling

```bash
cargo uninstall vycode
```

To also remove configuration:
- **Windows**: Delete `%APPDATA%\vycode\`
- **Linux/macOS**: Delete `~/.config/vycode/`
