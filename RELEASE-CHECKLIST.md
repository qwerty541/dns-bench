## Release checklist

This document is a checklist for the release process of the `dns-bench` project.

- [ ] Ensure that the `CHANGELOG.md` contains all unreleased changes.
- [ ] Rename `Unreleased` section in `CHANGELOG.md` to the new version.
- [ ] Ensure that MSRV is not changed, if changed update badge in `README.md`, `CHANGELOG.md` and `Cargo.toml`.
- [ ] Update tag version in `README.md` installation instructions.
- [ ] Update `version` property in `Cargo.toml` to the new version.
- [ ] Rebuild `Cargo.lock` by running `cargo build`.
- [ ] Commit changes with message `v<version>`.
- [ ] Run `cargo publish` to publish the crate to crates.io.
