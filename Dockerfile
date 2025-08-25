# Build stage
FROM rust:1.89.0 AS builder

# Install musl toolchain for static linking
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y \
    musl-tools \
    musl-dev \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Important: use musl-gcc, not x86_64-linux-gnu-gcc
ENV CC=musl-gcc
ENV CARGO_BUILD_TARGET=x86_64-unknown-linux-musl

# Build release binary statically linked
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime stage
FROM scratch
WORKDIR /app

# Copy necessary runtime files (SSL certs if needed)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy built binary and config
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/link_server ./
COPY --from=builder /app/app.env ./
COPY --from=builder /app/banks.json ./
COPY --from=builder /app/migrations ./db/migration

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/link_server", "--health-check"] || exit 1

# Environment tuning
ENV RUST_MIN_STACK=8388608
ENV RUST_MAX_THREADS=32

# Graceful shutdown
STOPSIGNAL SIGTERM

# Run binary
CMD ["/app/link_server"]
