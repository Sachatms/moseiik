# Dockerfile for moseiik - Multi-architecture Rust project
# Supports x86_64 (amd64) and aarch64 (arm64) architectures

FROM rust:1.83 AS builder

WORKDIR /app

# Copy dependency manifests first for better layer caching
# This allows Docker to cache dependencies unless Cargo.toml/Cargo.lock changes
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY assets ./assets

RUN cargo build --release

FROM rust:1.83

WORKDIR /app

# OCI metadata labels
LABEL org.opencontainers.image.source="https://github.com/Sachatms/moseiik"
LABEL org.opencontainers.image.description="Moseiik - Multi-architecture mosaic image generator"

COPY --from=builder /app/Cargo.toml /app/Cargo.lock ./
COPY --from=builder /app/src ./src
COPY --from=builder /app/tests ./tests
COPY --from=builder /app/assets ./assets

COPY --from=builder /app/target ./target

ENTRYPOINT ["cargo", "test"]
CMD ["--release"]
