# Dockerfile for moseiik - Multi-architecture Rust project
# Supports x86_64 (amd64) and aarch64 (arm64) architectures

FROM rust:1.83

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Create dummy source to build dependencies only
# Note: Dummy lib.rs must match the structure to avoid re-compilation
RUN mkdir -p src tests && \
    echo "pub fn main() {}" > src/main.rs && \
    echo "// Dummy lib for dependency caching" > src/lib.rs && \
    echo "" > tests/temp.rs && \
    cargo build --release && \
    rm -rf src tests

COPY src ./src
COPY tests ./tests
COPY assets ./assets

RUN cargo build --release --tests

ENTRYPOINT ["cargo", "test"]
CMD ["--release"]
