# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

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

- Fixed default requests count in features section of DockerHub overview.
- Added example command of using custom servers list feature with docker container into readme file and DockerHub overview.
- Removed table of contents from the DockerHub overview because it doesn't work there.
- Table with built-in servers list was splitted into three columns to improve readability.

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
- Added opt-in table of contents to the readme file and DockerHub overview.
- Refactored list of built-in servers into two columns to make it more compact inside the readme file and DockerHub overview.
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

-  Added `--protocol` option to specify protocol (either TCP or UDP).
-  Added `--lookup-ip` option to specify lookup IP version (either IPv4 or IPv6).
-  Added `--name-servers-ip` option to specify IP version used to establish connection (either IPv4 or IPv6).
-  Significant code refactoring.
-  Various minor documentation improvements.

### Changed

- Changed MSRV from 1.70.0 to 1.74.1

### Dependencies

- Updated `tabled` from 0.14.0 to 0.15.0
- Updated `clap` from 4.4.11 to 4.5.4

## v0.3.0 (23.12.2023)

-   Added `--requests` option to specify custom number of requests to each DNS server.
-   Added `--timeout` option to specify custom timeout in seconds.
-   Added Verisign public DNS to built-in list.
-   Fixed tabled sorting, now failed DNS entries are always at the end of the table.
-   Fixed error handling, now it shows actual error descriptions instead of static "Failed to resolve" message.
-   Fixed sending extra requests on fails, `hickory-resolver` got default retries count of 2.
-   Improved progress bar appearance.
-   Improved documentation.

## v0.2.0 (17.12.2023)

-   Boosted performance 5x times by multi thread implementation (now by default 8 threads).
-   Added `--threads` option to specify custom number of threads.
-   Added output of total benchmark time.
-   Updated dependencies:
    - `clap` from 4.4.7 to 4.4.11
    - Replaced `trust-dns-resolver` with `hickory-resolver`

## v0.1.3 (10.11.2023)

-   Add Level3 DNS to list
-   README improvements
-   Updated dependencies:
    - `trust-dns-resolver` from 0.23.0 to 0.23.2
    - `clap` from 4.4.6 to 4.4.7  

## v0.1.2 (05.10.2023)

- Add keywords and categories fields into Cargo.toml

## v0.1.1 (04.10.2023)

-   Add Comodo Secure DNS to list
-   README improvements

## v0.1.0 (04.10.2023)

Initial release
