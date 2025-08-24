# Build stage
FROM rust:1.89.1 AS builder
RUN rustup target add x86_64-unknown-linux-musl
RUN apt -y update && apt install -y \
    musl-tools \
    musl-dev \
    build-essential \
    gcc-x86-64-linux-gnu \
    ca-certificates \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ./ .
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
# Build with optimizations
RUN cargo build --target x86_64-unknown-linux-musl --release

# Runtime stage
FROM scratch
# Add necessary runtime files
# COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
WORKDIR /app
# Create a non-root user (65532 is commonly used for scratch images)
# USER 65532:65532
# Add health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/link_server", "--health-check"] || exit 1
# Copy the binary and config
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/link_server ./
COPY --from=builder /app/app.env ./
COPY --from=builder /app/banks.json ./
COPY --from=builder /app/migrations ./db/migration
# Set resource limits
ENV RUST_MIN_STACK="8388608"
ENV RUST_MAX_THREADS="32"
# Configure graceful shutdown
STOPSIGNAL SIGTERM
# Run the binary
CMD ["/app/link_server"]