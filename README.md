# dns-bench

[![Crates.io][crates-badge]][crates-url]
![Rust version][rust-version]
![License][license-badge]
[![Workflow Status][workflow-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/dns-bench.svg
[crates-url]: https://crates.io/crates/dns-bench
[license-badge]: https://img.shields.io/crates/l/dns-bench.svg
[workflow-badge]: https://github.com/qwerty541/dns-bench/workflows/check/badge.svg
[actions-url]: https://github.com/qwerty541/dns-bench/actions
[rust-version]: https://img.shields.io/badge/rust-1.74.1%2B-lightgrey.svg?logo=rust

## Description

This repository provides DNS benchmarking command line tool written in Rust. It iterates through built-in list of public DNS servers, measures their response time and print table with sorted results in console. It can be used to find the fastest DNS in your location. An example of console output, list of features and list of built-in DNS servers can be found below.

### Example

![Example](./example.gif)

### Features

- Built-in list of public DNS servers.
- Requests count configuration. By default, 3 requests are made to each DNS server.
- Threads count configuration. By default, 8 threads are used.
- Timeout configuration. By default, 3 seconds timeout is used.
- Domain configuration. By default, google.com domain is used.
- Protocol configuration, either TCP or UDP. By default, UDP is used.
- Lookup IP version configuration, either IPv4 or IPv6. By default, IPv4 is used.
- Configuration of IP version used to establish connection, either IPv4 or IPv6. By default, IPv4 is used.

### List of built-in DNS servers

- Google Public DNS
- Cloudflare
- Quad9
- ControlD
- OpenDNS
- CleanBrowsing
- AdGuard DNS
- Comodo Secure DNS
- Level3
- Verisign

## Installation

### From crates.io (Recommended)

Run the following command and wait until the crate is compiled:

```sh
$ cargo install dns-bench
```

Now you can run compiled binary:

```sh
$ dns-bench
```

### From git repository

Run the following command and wait until the crate is compiled:

```sh
$ cargo install --git https://github.com/qwerty541/dns-bench.git --tag v0.3.0 dns-bench
```

Also you can remove tag option to install the latest development version.

Now you can run compiled binary:

```sh
$ dns-bench
```

## Options

Below is a list of currently supported options.

```
$ dns-bench --help
Find the fastest DNS in your location using simple command line tool.

Usage: dns-bench [OPTIONS]

Options:
      --domain <DOMAIN>
          The domain to resolve [default: google.com]
      --threads <THREADS>
          The number of threads to use [default: 8]
      --requests <REQUESTS>
          The number of requests to make [default: 3]
      --timeout <TIMEOUT>
          The timeout in seconds [default: 3]
      --protocol <PROTOCOL>
          The protocol to use [default: udp] [possible values: tcp, udp]
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
