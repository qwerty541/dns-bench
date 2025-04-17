FROM rust:1.85.1

# Set the working directory
WORKDIR /app

# Copy only the necessary files for building the application
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Install the binary
RUN cargo install --path .

# Set the default command
CMD ["dns-bench"]
