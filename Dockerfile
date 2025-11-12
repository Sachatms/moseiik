# Dockerfile for moseiik - Multi-architecture mosaic image generator
# Supports both x86_64 (linux/amd64) and aarch64 (linux/arm64) architectures

FROM rust:latest

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Build the project in release mode
RUN cargo build --release

# Run tests by default when container starts
ENTRYPOINT ["cargo", "test"]
