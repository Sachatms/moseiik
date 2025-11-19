# Dockerfile for moseiik - Multi-architecture Rust project
# Supports x86_64 (amd64) and aarch64 (arm64) architectures

FROM rust:1.83

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY assets ./assets

RUN cargo build --release --tests

ENTRYPOINT ["cargo", "test"]
CMD ["--release"]
