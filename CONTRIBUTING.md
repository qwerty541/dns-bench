# Contributing to dns-bench

Thanks for your interest in contributing! This document outlines how to propose changes, report issues, and develop locally. The project follows common practices used across the Rust crates community.

## Code of Conduct

This project adheres to a Code of Conduct. By participating, you agree to uphold it.

- See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)

## Ways to Contribute

- Report bugs or suggest improvements via GitHub Issues
- Implement features (check open issues or propose new ones)
- Improve documentation (README, Docker overview, examples)
- Add tests and benchmarks
- Triage issues (labels, reproductions, platform checks)

## Development Setup

### Prerequisites

- Rust toolchain (stable) installed via [rustup](https://rustup.rs/)
- Cargo (bundled with rustup)
- For Docker-related work: Docker and optionally Buildx/QEMU for multi-arch

### Devcontainers (optional)

This repo supports devcontainers. You can open it in VS Code with the Container Tools extension (or GitHub Codespaces) to get a pre-configured environment:

1. Install VS Code and the "Container Tools" extension
2. Open the repository
3. Use "Reopen in Container" to start a reproducible dev environment

Devcontainers are helpful for consistent toolchains, multi-arch testing, and running CI-like builds locally.

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- [OPTIONS]
```

Common quick runs:

```bash
# IPv4 benchmark with short timeout
cargo run -- --name-servers-ip v4 --lookup-ip v4 --timeout 1

# Use custom servers list
cargo run -- --custom-servers-file ./examples/ipv4-custom-servers-example.txt
```

### Testing

```bash
cargo test
```

CI runs `cargo build`, `cargo test`, `cargo fmt --check`, and `cargo clippy -D warnings` across platforms and toolchains.

### Linting & Formatting

- Formatting: `cargo fmt`
- Linting: `cargo clippy --all-features -- -D warnings`

## Project Structure

- `src/` — application source code
  - `commands/` — subcommands handling
  - `output/` — formatters (table, json, xml, csv)
- `tests/` — test assets
- `docker/` — Dockerfiles and CI scripts for image builds
- `examples/` — example server lists and usage samples
- `README.md` — usage, installation, and reference docs
- `CHANGELOG.md` — release notes
- `SECURITY.md` — how to report security issues

## Feature Guidelines

- Keep default behavior sane. See `DnsBenchConfig` defaults and CLI.
- Add flags for opt-in changes (e.g., `--disable-adaptive-timeout`).
- Ensure output formats (table/JSON/XML/CSV) remain consistent.
- Prefer small, incremental PRs.

## Performance & Reliability

- Avoid unnecessary allocations and re-use resolvers thoughtfully.
- Keep benchmarks deterministic where practical; document adaptation logic.
- For long-running or multi-arch Docker builds, consider Buildx cache and matrix strategies.

## Tests

- Add unit tests for new logic (error classification, formatting, etc.)
- Maintain or improve coverage for changed code paths
- Include edge cases (timeouts, unreachable servers, invalid inputs)

## Documentation

- Update [README.md](./README.md) and examples when adding or changing CLI arguments
- Add changelog entries under `## Unreleased` and reference commits/issues

## Commit & PR Etiquette

- Use conventional, descriptive commit messages (e.g., `feat:`, `fix:`, `docs:`)
- Reference issues (e.g., `resolves #123`) when applicable
- Keep PRs focused; include notes on testing and potential impacts

## Release Process

To prepare and publish a new release:

- See [RELEASE.md](./RELEASE.md)

## Security

Please report vulnerabilities via the documented security protocol.

- See [SECURITY.md](./SECURITY.md)

## License

By contributing, you agree that your contributions will be licensed under the terms listed in [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT).
