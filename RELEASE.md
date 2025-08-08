## Release checklist

This document is a checklist for the release process of the `dns-bench` project.

- Ensure that the [CHANGELOG.md](./CHANGELOG.md) contains all unreleased changes and adheres to the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format.
- Ensure that the [README.md](./README.md) and [/docker/OVERVIEW.md](./docker/OVERVIEW.md) contain all the necessary information about the new version.
- Ensure that GitHub Actions checks are passing and MSRV is not changed, if changed update the badge in [README.md](./README.md), add the note into [CHANGELOG.md](./CHANGELOG.md) and update `rust-version` property inside [Cargo.toml](./Cargo.toml).
- Define a new version according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) and update it inside the following files:
  - Update the `version` property in [Cargo.toml](./Cargo.toml) to the new version.
  - Update the tag version in [Docker workflow](./.github/workflows/docker.yml) to the new version.
  - Rename `Unreleased` section in [CHANGELOG.md](./CHANGELOG.md) to the new version and current date.
  - Update the tag version inside installation section of [README.md](./README.md) to the new version.
- Rebuild [Cargo.lock](./Cargo.lock) by running `cargo build`.
- Commit changes with message `v<version>`.
- Run `cargo publish` to publish the crate to crates.io.
- Push changes to the repository.
- Build executable file for Windows by running [build-win-exe.sh](./build-win-exe.sh) script. Note that this build requires a certain rustup target and mingw linker to be present in the system. All required tools are already installed in the devcontainer.
- Manually test the Windows executable file to ensure it works correctly.
- Draft a new release on GitHub with the new version, changelog and Windows executable file attached.
- Manually run the workflow "build, test, and push multi-arch docker image" on the GitHub Actions page. This will build and push the new Docker image to Docker Hub.
- Update repository overview in Docker Hub.
