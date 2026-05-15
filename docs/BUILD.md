# VyCode Build Guide

## Development Build

```bash
cargo build
```

## Release Build (Optimized)

```bash
cargo build --release
```

The release build applies these optimizations (configured in `Cargo.toml`):
- **opt-level = 3** — Maximum optimization
- **lto = true** — Link-time optimization for smaller binary
- **codegen-units = 1** — Single codegen unit for better optimization
- **strip = true** — Strip debug symbols

## Cross-Compilation

### Windows → Linux
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

### Linux → Windows
```bash
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

## Running Tests

```bash
cargo test
```

## Checking Code

```bash
cargo clippy
cargo fmt --check
```

## Build Artifacts

After `cargo build --release`, binaries are found at:

| Platform | Path |
|----------|------|
| Windows | `target/release/vycode.exe` |
| Linux | `target/release/vycode` |
| macOS | `target/release/vycode` |

## Binary Size

Expected release binary size: ~5-10 MB (varies by platform and features).

## Troubleshooting

### OpenSSL errors (Linux)
```bash
sudo apt install pkg-config libssl-dev
```

### Windows linker errors
Install Visual Studio Build Tools with the "Desktop development with C++" workload.
