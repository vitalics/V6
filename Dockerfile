# Use the official Rust image as a parent image
FROM rust:1.75 as builder

# Set the working directory in the container
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the build script
COPY build.rs ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal base image for the final container
FROM debian:bookworm-slim

# Install SSL certificates and other runtime dependencies
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl3 \
        && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false v6user

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/v6 /usr/local/bin/v6

# Make the binary executable
RUN chmod +x /usr/local/bin/v6

# Change ownership to the non-root user
RUN chown v6user:v6user /usr/local/bin/v6

# Switch to the non-root user
USER v6user

# Set the default command
ENTRYPOINT ["v6"]
CMD ["--help"]