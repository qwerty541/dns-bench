FROM rust:1.89.0 AS build

# Set the working directory
WORKDIR /app

# Add the MUSL target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Install the binary
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:3.22.1

COPY --from=build /usr/local/cargo/bin/dns-bench /usr/bin/dns-bench

# Set the default command
ENTRYPOINT [ "dns-bench" ]
CMD ["--skip-system-servers", "--skip-gateway-detection"]
