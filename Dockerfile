# --- Build stage ---
FROM rust:1.86.0-slim AS builder

WORKDIR /app
COPY . .

# Install build dependencies (if any extra needed)
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl

# Build for GNU target (dynamic, but portable)
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim
ARG BUILD_DATE
ARG VCS_REF
LABEL org.opencontainers.image.title="fdic-bank-find-mcp-server" \
    org.opencontainers.image.description="MCP-compliant Rust API proxy for the FDIC BankFind API" \
    org.opencontainers.image.version="1.0.0" \
    org.opencontainers.image.licenses="MIT" \
    org.opencontainers.image.authors="Cali LaFollett <cali.lafollett+fdic-mcp-server@gmail.com>" \
    org.opencontainers.image.vendor="Cali LaFollett" \
    org.opencontainers.image.url="https://github.com/clafollett/fdic-bank-find-mcp-server" \
    org.opencontainers.image.source="https://github.com/clafollett/fdic-bank-find-mcp-server" \
    org.opencontainers.image.documentation="https://github.com/clafollett/fdic-bank-find-mcp-server#readme" \
    org.opencontainers.image.created=${BUILD_DATE} \
    org.opencontainers.image.revision=${VCS_REF}

# Set our working directory
WORKDIR /server
RUN mkdir -p ./logs
# Copy the server and public directory

# Install runtime dependencies for Rust binaries (openssl, ca-certificates, etc.)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/fdic-bank-find-mcp-server .

# `stdio` is the default configuration
CMD ["/server/fdic-bank-find-mcp-server"]
