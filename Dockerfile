# Use the official Rust image to build the application
FROM rust:latest as builder

# Create a new directory for the application code
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies
RUN cargo fetch

# Copy the source code to the container
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Install any necessary packages
RUN apt-get update && apt-get install -y \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/tax_api /usr/local/bin/tax_api

# Expose the port your API will run on
EXPOSE 8080

# Set the default command to run your API
CMD ["tax_api"]
