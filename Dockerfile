# Stage 1: Build the Rust application
FROM rust:1.65-slim AS builder

WORKDIR /app

# Copy the source code
COPY src/main.rs ./main.rs

# Compile the Rust program
RUN rustc main.rs -o binomial-theorem

# Stage 2: Create a lightweight runtime image
FROM debian:bullseye-slim

WORKDIR /app

# Install Bash (if not already installed)
RUN apt-get update && apt-get install -y bash

# Copy the compiled binary from the builder stage
COPY --from=builder /app/binomial-theorem /app/binomial-theorem

# Copy the Bash script
COPY scripts/start.sh /app/start.sh

# Set the entry point
ENTRYPOINT ["/app/start.sh"]