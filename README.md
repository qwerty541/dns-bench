# dns-bench

[![Workflow Status][workflow-badge]][actions-url]

[workflow-badge]: https://github.com/qwerty541/dns-bench/workflows/check/badge.svg
[actions-url]: https://github.com/qwerty541/dns-bench/actions

## Description

This repository provides DNS benchmarking command line tool written on Rust. It iterates through prepared list of public DNS servers, measures their response time and print table with sorted results in console.

## Options

Below is a list of currently supported options.

```
$ ./target/debug/dns-bench --help
Determine the fastest DNS for yourself using simple command line tool.

Usage: dns-bench [OPTIONS]

Options:
      --domain <DOMAIN>
          [default: google.com]
  -h, --help
          Print help
  -V, --version
          Print version
```

## License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
