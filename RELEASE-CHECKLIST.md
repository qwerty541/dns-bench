## Release checklist

This document is a checklist for the release process of the `dns-bench` project.

- [ ] Ensure that the [CHANGELOG.md](./CHANGELOG.md) contains all unreleased changes and adheres to the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.
- [ ] Ensure that the [README.md](./README.md) and [/docker/OVERVIEW.md](./docker/OVERVIEW.md) contain all the necessary information about the new version.
- [ ] Ensure that GitHub Actions CI is passing and MSRV is not changed, if changed update badge in [README.md](./README.md), [CHANGELOG.md](./CHANGELOG.md) and [Cargo.toml](./Cargo.toml).
- [ ] Define a new version according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) and update it inside the following files:
  - [ ] Tag version in [README.md](./README.md) and [/docker/OVERVIEW.md](./docker/OVERVIEW.md) installation instructions.
  - [ ] `version` property in [Cargo.toml](./Cargo.toml) to the new version.
  - [ ] Tag version in [Dockerfile](./docker/Dockerfile), [docker-build.sh](./docker/build.sh) and [docker-push.sh](./docker/push.sh).
  - [ ] Rename `Unreleased` section in [CHANGELOG.md](./CHANGELOG.md) to the new version and date.
- [ ] Rebuild [Cargo.lock](./Cargo.lock) by running `cargo build`.
- [ ] Commit changes with message `v<version>`.
- [ ] Run `cargo publish` to publish the crate to crates.io.
- [ ] Run `/docker/build.sh` to build a new Docker image.
- [ ] Run `/docker/push.sh` to push the new Docker image to Docker Hub.
- [ ] Update repository overview in Docker Hub.
