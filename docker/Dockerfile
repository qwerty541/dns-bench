FROM rust:1.91.0 AS build

# Set the working directory
WORKDIR /app

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build and install the binary
RUN cargo install --locked --path .

FROM debian:13.1-slim

# Copy the binary from the builder image
COPY --from=build /usr/local/cargo/bin/dns-bench /usr/bin/dns-bench

# Set the default command
ENTRYPOINT [ "dns-bench" ]
CMD ["--skip-system-servers", "--skip-gateway-detection"]
