FROM rust:1.90.0-alpine3.22 AS build

RUN apk add musl-dev

# Set the working directory
WORKDIR /app

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Install the binary
RUN cargo install --path .

FROM alpine:3.22.2

COPY --from=build /usr/local/cargo/bin/dns-bench /usr/bin/dns-bench

# Set the default command
ENTRYPOINT [ "dns-bench" ]
CMD ["--skip-system-servers", "--skip-gateway-detection"]
