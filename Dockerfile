# Dockerfile for moseiik - Multi-architecture Rust project
# Supports x86_64 (amd64) and aarch64 (arm64) architectures

FROM rust:1.83

WORKDIR /app

# Copy dependency files first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source to cache dependencies
# This matches the project structure (lib.rs + main.rs) to maximize cache hits
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source code and assets
COPY src ./src
COPY tests ./tests
COPY assets ./assets

# Build tests in release mode (cached if source unchanged)
RUN cargo build --release --tests

# Note: moseiik_test_images dataset is mounted at runtime via -v flag
# This allows the image to be built without requiring the large dataset
# and makes the ground-truth test optional

# ENTRYPOINT allows passing test filters as arguments
# Example: docker run moseiik test_ground_truth_kit
ENTRYPOINT ["cargo", "test", "--release", "--"]
