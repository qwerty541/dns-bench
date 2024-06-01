## Release checklist

This document is a checklist for the release process of the `dns-bench` project.

- [ ] Ensure that the [CHANGELOG.md](./CHANGELOG.md) contains all unreleased changes. then rename `Unreleased` title to the new version with date.
- [ ] Ensure that GitHub Actions CI is passing and MSRV is not changed, if changed update badge in [README.md](./README.md), [CHANGELOG.md](./CHANGELOG.md) and [Cargo.toml](./Cargo.toml).
- [ ] Update tag version in [README.md](./README.md) installation instructions.
- [ ] Update `version` property in [Cargo.toml](./Cargo.toml) to the new version.
- [ ] Update tag version in [Dockerfile](./Dockerfile), [docker-build.sh](./docker-build.sh) and [docker-push.sh](./docker-push.sh).
- [ ] Rebuild [Cargo.lock](./Cargo.lock) by running `cargo build`.
- [ ] Commit changes with message `v<version>`.
- [ ] Run `cargo publish` to publish the crate to crates.io.
- [ ] Run `./docker-build.sh` to build a new Docker image.
- [ ] Run `./docker-push.sh` to push the new Docker image to Docker Hub.
