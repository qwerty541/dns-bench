FROM rust:1.89.0 AS build

# Set the working directory
WORKDIR /app

# Install MUSL toolchain (needed on Debian-based rust images for musl targets)
RUN apt-get update && apt-get install -y --no-install-recommends musl-tools && rm -rf /var/lib/apt/lists/*

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Install the binary for the correct platform-specific MUSL target
#   linux/amd64      -> x86_64-unknown-linux-musl
#   linux/arm64/v8   -> aarch64-unknown-linux-musl
#   linux/arm/v7     -> armv7-unknown-linux-musleabihf
#   linux/386        -> i686-unknown-linux-musl
#   linux/s390x      -> s390x-unknown-linux-musl
#   linux/ppc64le    -> powerpc64le-unknown-linux-musl
ARG TARGETPLATFORM
RUN set -eux; \
    case "$TARGETPLATFORM" in \
    "linux/amd64")   RUST_TARGET=x86_64-unknown-linux-musl ;; \
    "linux/arm64"|"linux/arm64/v8") RUST_TARGET=aarch64-unknown-linux-musl ;; \
    "linux/arm/v7")  RUST_TARGET=armv7-unknown-linux-musleabihf ;; \
    "linux/386")     RUST_TARGET=i686-unknown-linux-musl ;; \
    "linux/s390x")   RUST_TARGET=s390x-unknown-linux-musl ;; \
    "linux/ppc64le") RUST_TARGET=powerpc64le-unknown-linux-musl ;; \
    *) echo "Unsupported TARGETPLATFORM: $TARGETPLATFORM" >&2; exit 1 ;; \
    esac; \
    rustup target add "$RUST_TARGET"; \
    cargo install --locked --target "$RUST_TARGET" --path .

FROM alpine:3.22.1

COPY --from=build /usr/local/cargo/bin/dns-bench /usr/bin/dns-bench

# Set the default command
ENTRYPOINT [ "dns-bench" ]
CMD ["--skip-system-servers", "--skip-gateway-detection"]
