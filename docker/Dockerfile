FROM rust:1.89.0-alpine3.22 AS build

RUN apk add musl-dev

# Set the working directory
WORKDIR /app

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Install the binary
RUN cargo install --path .

FROM alpine:3.22.1

COPY --from=build /usr/local/cargo/bin/dns-bench /usr/bin/dns-bench

# Set the default command
CMD ["dns-bench", "--skip-system-servers"]
