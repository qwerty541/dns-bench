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
