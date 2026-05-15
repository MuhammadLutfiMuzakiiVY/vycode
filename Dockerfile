# --- Stage 1: Builder ---
FROM rust:slim as builder
WORKDIR /app

# Install dependencies for compilation (including build-essential for C bindings needed by crates like syntect)
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Copy source code
COPY . .

# Build for release
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime SSL certificates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/vycode /usr/local/bin/vycode

# Labeling
LABEL org.opencontainers.image.source="https://github.com/MuhammadLutfiMuzakiiVY/vycode"
LABEL org.opencontainers.image.description="VyCode Terminal AI Assistant Container"

# Run command
ENTRYPOINT ["vycode"]
