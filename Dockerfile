# Multi-stage build for minimal image size
FROM rust:1.90-slim AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 ipapi

# Copy binary from builder
COPY --from=builder /app/target/release/ip-api /usr/local/bin/ip-api

# Set ownership
RUN chown ipapi:ipapi /usr/local/bin/ip-api

# Switch to non-root user
USER ipapi

# Expose port (default)
EXPOSE 7111

# Set default port
ENV PORT=7111

# Run the application
CMD ["/bin/sh", "-c", "ip-api --port ${PORT}"]