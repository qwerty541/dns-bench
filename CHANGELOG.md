# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added FlashStart DNS to the built-in servers list.
- Added arrow symbol which indicates sorting column in the output table header.

### Changed

- Replaced previous results table columns `First duration`/`Average duration` with `Min.`/`Max.`/`Avg.` columns to provide more comprehensive statistics about DNS servers performance and better matching benchmarking best practices.
- Updated other output formats (JSON, XML and CSV) to reflect the changes in the results table structure.
- Centered the contents of the column titles for better appearance and readability.
- Changed base image of Docker container from [alpine](https://hub.docker.com/_/alpine) to [debian slim](https://hub.docker.com/_/debian) which improves compatibility with various systems and architectures, and also results in a slightly larger image size (~15 MiB => ~25 MiB).
- Restored the support of `linux/arm/v7`, `linux/386`, `linux/s390x` architectures in the Docker image. The docker image of previous version was republished to include these architectures.
- Significant internal refactoring of output formatting related code to improve its readability and maintainability.
- Integrated `derive_more` library to reduce some boilerplate code with derived implementations.

### Documentation

- Made the license badge clickable.
- Added COCOMO estimation badge into the readme file.
- Added Docker Hub pulls badge into the readme file.

### Dependencies

- Updated `clap` from 4.5.47 to 4.5.50
- Updated `serde` from 1.0.223 to 1.0.228
- Updated `toml` from 0.9.5 to 0.9.8
- Updated `csv` from 1.3.1 to 1.4.0
- Updated `indicatif` from 0.18.0 to 0.18.1

## v0.12.0 (28.09.2025)

### Added

- Implemented automatic detection of gateway (router) address on Linux, Windows and macOS platforms. It will be included in the benchmark in case this address is a DNS provider.
- Added `--skip-gateway-detection` option to skip the auto-detection of gateway (router) address.
- Implemented `dns-bench config list` subcommand to list the current configurations without necessity to run the benchmark or open the file.

### Changed

- Changed base image of Docker container from [rust](https://hub.docker.com/_/rust) to [alpine](https://hub.docker.com/_/alpine) which results in significant reduction of the image size (~650 MiB => ~15 MiB) by [@February30th](https://github.com/February30th) in https://github.com/qwerty541/dns-bench/pull/301.
- Docker container now uses a `dns-bench` binary entrypoint, so you can pass arguments directly to `docker run` command without necessity to specify `/bin/sh -c "dns-bench ..."`.
- Dropped the support of several architectures in the Docker image including `linux/arm/v7`, `linux/386`, `linux/s390x` due to migration on the alpine.

### Documentation

- Completely reworked the Docker Hub overview to match the style and follow the best practices of this platform.
- Updated documentation according to all the changes and newly added features.

### Dependencies

- Updated `clap` from 4.5.41 to 4.5.47
- Updated `toml` from 0.9.2 to 0.9.5
- Updated `quick-xml` from 0.38.0 to 0.38.3
- Updated `serde` from 1.0.219 to 1.0.223
- Updated `serde_json` from 1.0.141 to 1.0.145
- Updated `tokio` from 1.32.0 to 1.47.1

## v0.11.0 (04.08.2025)

### Added

- Implemented subcommands which allow users to perform configurations file management without the necessity to run benchmark each time. Here is the list of available subcommands:
    - `dns-bench config init` - initializes the configuration file with default values.
    - `dns-bench config set [--key value ...]` - sets the specified keys to the specified values inside the configuration file.
    - `dns-bench config reset` - resets the configuration file to its default values.
    - `dns-bench config delete` - deletes the configuration file.
- Added Vercara UltraDNS Public to the built-in servers list.

### Changed

- Updated base image of Docker container from `rust:1.87.0` to `rust:1.88.0`
- Changed help template of all commands to include author information and repository link.

### Documentation

- Reworked the options section inside documentation into command-line reference section which includes options and subcommands subsections.
- Updated license copyright years.

### Dependencies

- Updated `quick-xml` from 0.37.5 to 0.38.0
- Updated `indicatif` from 0.17.11 to 0.18.0
- Updated `toml` from 0.8.23 to 0.9.2
- Updated `clap` from 4.5.40 to 4.5.41
- Updated `anstream` from 0.6.7 to 0.6.19
- Updated `mio` from 0.8.8 to 0.8.11
- Updated `serde_json` from 1.0.140 to 1.0.141

## v0.10.1 (29.06.2025)

### Added

- Implemented build process of the Windows executable file. Starting from this version, the compiled executable file will be attached to each GitHub release.
- Added devcontainer configuration to the repository. It allows you to run the project in a containerized development environment using Visual Studio Code or any other compatible IDE.

### Fixed

- Adjusted the automatic detection of system DNS servers on Windows to work with non-English locales. Previously, it could fail if the system language was not English.

### Documentation

- Updated installation instructions in the readme file to include information about the Windows executable file.
- Updated changelog entries of previous versions to adhere to keep a changelog format.
- Fixed Docker Hub branding across all documents.

### Dependencies

- Updated `clap` from 4.5.39 to 4.5.40
- Updated `directories` from 5.0.1 to 6.0.0

## v0.10.0 (17.06.2025)

### Added

- Implemented automatic detection of the system DNS servers on Linux, Windows and macOS platforms and its inclusion in the benchmark in case these servers are not present in the built-in servers list.
- Servers that are currently configured in the system will be highlighted in the output table.
- Added `--skip-system-servers` option to skip the auto-detection of system DNS servers.

### Changed

- Changed MSRV from 1.74.1 to 1.82.0
- Updated base image of Docker container from `rust:1.86.0` to `rust:1.87.0`

### Documentation

- Added information about new features.
- Completely reworked features section to make it more informative and user-friendly.
- Reviewed grammar and spelling errors in the description section.
- Updated the preview gif-animation to reflect the new features.
- Added the preview image to the readme file and Docker Hub overview.

### Dependencies

- Updated `quick-xml` from 0.37.4 to 0.37.5
- Updated `toml` from 0.8.20 to 0.8.23
- Updated `tabled` from 0.18.0 to 0.20.0
- Updated `clap` from 4.5.37 to 4.5.39

## v0.9.1 (28.04.2025)

### Changed

- Updated base image of Docker container from `rust:1.85.1` to `rust:1.86.0`

### Fixed

- Resolved several uninlined format args clippy lints on the nightly toolchain.

### Build

- Removed preview git-animation from the published crate to reduce its size 30+ times (1.15 MiB => 31.1 KiB).

### Documentation

- Replaced outdated preview gif-animation with two new separate ones for crate and Docker Hub.
- Enhanced text in description section, fixed grammatical errors, and improved overall readability.

### Dependencies

- Updated `clap` from 4.5.34 to 4.5.37

## v0.9.0 (16.04.2025)

### Added

- Introduced a new CSV output format. It can be used by specifying the `--format` option with the value `csv`.
- Added Surfshark DNS to the built-in servers list.
- Added SafeServe to the built-in servers list.
- Enhanced numeric command line arguments validation to ensure they are within the specified range.
- Added requests timeout value to the summary output.
- From this version, the Docker images will be multi-arch. Here is a list of supported architectures: `linux/amd64`, `linux/arm64`, `linux/arm/v7`, `linux/386`, `linux/s390x`, `linux/ppc64le`. Previously, only `linux/amd64` was supported.
- From this version, the latest Docker image will be available under the `latest` tag. Previously, only the versioned tag was available.
- Minor documentation improvements.
- Improved tests coverage.

### Changed

- Updated base image of Docker container from `rust:1.85.0` to `rust:1.85.1`

### Dependencies

- Updated `clap` from 4.5.32 to 4.5.34

## v0.8.0 (04.04.2025)

### Added

- Introduced a new option `--format` to specify the output format. Supported formats are `human-readable`, `json`, and `xml`. The default format is `human-readable`.
- Added Hurricane Electric to the built-in servers list.
- Minor documentation improvements.
- Improved tests coverage.

### Changed

- Updated base image of Docker container from `rust:1.83.0` to `rust:1.85.0`

### Dependencies

- Updated `clap` from 4.5.23 to 4.5.32
- Updated `indicatif` from 0.17.9 to 0.17.11
- Updated `toml` from 0.8.19 to 0.8.20
- Updated `tabled` from 0.17.0 to 0.18.0
- Updated `hickory-resolver` from 0.24.2 to 0.24.4
- Updated `serde` from 1.0.217 to 1.0.219

## v0.7.2 (31.12.2024)

### Added

- Added Dyn to the built-in servers list.

### Changed

- Updated base image of Docker container from `rust:1.82.0` to `rust:1.83.0`

### Documentation

- Fixed default requests count in features section of Docker Hub overview.
- Added example command of using custom servers list feature with Docker container into readme file and Docker Hub overview.
- Removed table of contents from the Docker Hub overview because it doesn't work there.
- Table with built-in servers list was split into three columns to improve readability.

### Dependencies

- Updated `serde` from 1.0.214 to 1.0.217
- Updated `clap` from 4.5.20 to 4.5.23
- Updated `indicatif` from 0.17.8 to 0.17.9
- Updated `tabled` from 0.16.0 to 0.17.0
- Updated `hickory-resolver` from 0.24.1 to 0.24.2

## v0.7.1 (06.11.2024)

### Changed

- Updated base image of Docker container from `rust:1.81.0` to `rust:1.82.0`

### Documentation

- Fixed default requests count in features section of readme file.
- Added opt-in table of contents to the readme file and Docker Hub overview.
- Refactored list of built-in servers into two columns to make it more compact inside the readme file and Docker Hub overview.
- Added crates.io downloads badge into the readme file.

### Dependencies

- Updated `serde` from 1.0.210 to 1.0.214

## v0.7.0 (27.10.2024)

### Added

- Colorized the contents of the success rate and duration columns in the output table, depending on the values.
- Add SafeDNS to the built-in servers list.
- Add NextDNS to the built-in servers list.

### Changed

- Significantly modified the progress bar appearance; instead of a single progress bar, it now shows the progress of each DNS server separately.
- Increased the frequency of progress bar updates.
- Updated base image of Docker container from `rust:1.78.0` to `rust:1.81.0`
- Changed default number of requests to each DNS server from `10` to `25` to make the benchmark more accurate.

### Documentation

- Added `-it` arguments to Docker container usage command. Without this arguments, the progress bar will not be displayed and the stdout will be empty until the end of the benchmark.
- Fixed broken links on the custom servers file example inside the Docker overview.
- Updated the example gif-animation.

### Dependencies

- Updated `clap` from 4.5.4 to 4.5.20
- Updated `toml` from 0.8.13 to 0.8.19
- Updated `lazy_static` from 1.4.0 to 1.5.0
- Updated `serde` from 1.0.203 to 1.0.210
- Updated `tabled` from 0.15.0 to 0.16.0

## v0.6.0 (02.06.2024)

### Added

- Added `--custom-servers-file` option to specify a custom file with DNS servers list to use instead of built-in list.
- Added DNS.WATCH to built-in list.
- Added Norton ConnectSafe to built-in list.
- Added docker image and published it to Docker Hub for users who don't have Rust programming language environment installed on their machines.
- Covered some code with tests.

### Documentation

- Added "Which method to choose?" article into installation section.

### Dependencies

- Updated `serde` from 1.0.201 to 1.0.203
- Updated `toml` from 0.8.12 to 0.8.13

## v0.5.1 (16.05.2024)

### Added

- Covered some code with tests.

### Build

- Removed redundant default features of `serde` crate.

### Documentation

- Fixed default requests count in features list.
- Fixed default table style in features list.
- Updated example gif-animation.
- Make example gif-animation to take full page width.

## v0.5.0 (14.05.2024)

### Added

- Added `--style` option to configure table style. By default, ASCII style is used.
- Added `--save-config` option to save favorite configurations in a file inside user's home directory (`/home/user/.dns-bench/config.toml`) to avoid typing them every time.

### Changed

- Changed arguments summary formatting to make it more compact.
- Changed table columns names formatting to make to more comfortable to read (i.e. `server_name` => `Server name`, `last_resolved_ip` => `Last resolved IP`).
- Changed default table style from `ascii` to `rounded`.
- Changed default number of requests to each DNS server from `3` to `10`.
- Changed description property inside Cargo.toml.

### Dependencies

- Updated `hickory-resolver` from 0.24.0 to 0.24.1

## v0.4.0 (25.04.2024)

### Added

- Added `--protocol` option to specify protocol (either TCP or UDP).
- Added `--lookup-ip` option to specify lookup IP version (either IPv4 or IPv6).
- Added `--name-servers-ip` option to specify IP version used to establish connection (either IPv4 or IPv6).
- Significant code refactoring.
- Various minor documentation improvements.

### Changed

- Changed MSRV from 1.70.0 to 1.74.1

### Dependencies

- Updated `tabled` from 0.14.0 to 0.15.0
- Updated `clap` from 4.4.11 to 4.5.4

## v0.3.0 (23.12.2023)

### Added

- Added `--requests` option to specify custom number of requests to each DNS server.
- Added `--timeout` option to specify custom timeout in seconds.
- Added Verisign public DNS to built-in list.

### Fixed

- Fixed table sorting, now failed DNS entries are always at the end of the table.
- Fixed error handling, now it shows actual error descriptions instead of static "Failed to resolve" message.
- Fixed sending extra requests on fails, `hickory-resolver` got default retries count of 2.

### Changed

- Improved progress bar appearance.
- Improved documentation.

## v0.2.0 (17.12.2023)

### Added

- Boosted performance 5x by multithreaded implementation (now by default 8 threads).
- Added `--threads` option to specify custom number of threads.
- Added output of total benchmark time.

### Dependencies

- Updated `clap` from 4.4.7 to 4.4.11
- Replaced `trust-dns-resolver` with `hickory-resolver` due to deprecation.

## v0.1.3 (10.11.2023)

### Added

- Added Level3 DNS to built-in list.

### Documentation

- README improvements.

### Dependencies

- Updated `trust-dns-resolver` from 0.23.0 to 0.23.2
- Updated `clap` from 4.4.6 to 4.4.7

## v0.1.2 (05.10.2023)

### Added

- Added keywords and categories fields into Cargo.toml.

## v0.1.1 (04.10.2023)

### Added

- Added Comodo Secure DNS to built-in list.

### Documentation

- README improvements.

## v0.1.0 (04.10.2023)

### Added

- Initial release of the `dns-bench` application.
