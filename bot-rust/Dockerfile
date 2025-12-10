# Multi-stage build for optimized Rust bot deployment
# Stage 1: Builder
FROM rustlang/rust:nightly-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml ./

# Copy source code
COPY src ./src

# Generate Cargo.lock (will use existing dependencies from Cargo.toml)
RUN cargo fetch

# Build for release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1001 botuser

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/curverider-bot /app/bot

# Set ownership
RUN chown -R botuser:botuser /app

# Switch to non-root user
USER botuser

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/api/health || exit 1

# Run the bot
CMD ["./bot"]
