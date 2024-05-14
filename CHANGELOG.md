## Unreleased

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
