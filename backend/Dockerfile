# Build stage
FROM rust as builder

# Create a new empty shell project
WORKDIR /usr/src/app
RUN cargo new --bin app
WORKDIR /usr/src/app/app

# Copy manifests
COPY Cargo.lock ./
COPY Cargo.toml ./

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY src ./src

# Build for release
RUN rm ./target/release/deps/app*
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from builder
COPY --from=builder /usr/src/app/app/target/release/app /usr/local/bin/

# Set runtime environment variables
ENV RUST_LOG=info
EXPOSE 3000

# Run as non-root user
RUN useradd -m -U -s /bin/false app
USER app

# Run the binary
CMD ["app"]
