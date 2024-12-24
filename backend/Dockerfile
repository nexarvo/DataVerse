# Use an official Rust image as the base image for building
FROM rust:1.72 AS builder

# Set the working directory in the container to /app
WORKDIR /app

# Copy the backend code into the container
COPY backend/Cargo.toml backend/Cargo.lock ./

# Copy the source code of the backend
COPY backend/src ./src

# Build the backend application
RUN cargo build --release

# Use a minimal base image for the runtime (Debian in this case)
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Optionally, expose ports or configure networking
EXPOSE 8080

# Specify the entry point for the application
ENTRYPOINT ["/usr/local/bin/backend"]