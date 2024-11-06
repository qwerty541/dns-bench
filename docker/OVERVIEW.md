Source: https://github.com/qwerty541/dns-bench

<details>
<summary>Table of contents</summary>

- [Description](#description)
  - [Example](#example)
  - [Features](#features)
  - [List of built-in DNS servers](#list-of-built-in-dns-servers)
- [Usage](#usage)
- [Options](#options)
- [License](#license)
</details>

# Description

This repository provides DNS benchmarking command line tool written in Rust. It iterates through built-in list of public DNS servers, measures their response time and print table with sorted results in console. It can be used to find the fastest DNS in your location for better internet browsing experience. An example of console output, list of features and list of built-in DNS servers can be found below.

## Example

<img src="https://raw.githubusercontent.com/qwerty541/dns-bench/master/example.gif" width="100%" alt="Example" />

## Features

- Built-in list of public DNS servers.
- Requests count configuration. By default, 10 requests are made to each DNS server.
- Threads count configuration. By default, 8 threads are used.
- Timeout configuration. By default, 3 seconds timeout is used.
- Domain configuration. By default, google.com domain is used.
- Protocol configuration, either TCP or UDP. By default, UDP is used.
- Lookup IP version configuration, either IPv4 or IPv6. By default, IPv4 is used.
- Configuration of IP version used to establish connection, either IPv4 or IPv6. By default, IPv4 is used.
- Table style configuration. By default, rounded style is used.
- Ability to save favorite configurations in a file inside user's home directory (`/home/user/.dns-bench/config.toml`) to avoid typing them every time.
- Ability to provide custom servers list instead of built-in list.

## List of built-in DNS servers

<table>
<tr><td>

- Google Public DNS
- Cloudflare
- Quad9
- ControlD
- OpenDNS
- CleanBrowsing
- AdGuard DNS

</td><td>

- Comodo Secure DNS
- Level3
- Verisign
- DNS.WATCH
- Norton ConnectSafe
- SafeDNS
- NextDNS

</td></tr>
</table>

# Usage

Run the following command to pull the image:

```sh
$ docker pull qwerty541/dns-bench:0.7.1
```

Now you can run this tool inside the container:

```sh
$ docker run --rm -it --name dns-bench qwerty541/dns-bench:0.7.1
```

If you want to pass some options, you can do it like this:

```sh
$ docker run --rm -it --name dns-bench qwerty541/dns-bench:0.7.1 /bin/bash -c "dns-bench --requests 20 --domain microsoft.com --style re-structured-text"
```

# Options

Below is a list of currently supported options.

<table>
    <thead>
        <th>Option</th>
        <th>Description</th>
        <th>Default value</th>
        <th>Possible values</th>
    </thead>
    <tbody>
        <tr>
            <td><code>--domain</code></td>
            <td>Domain to resolve.</td>
            <td>google.com</td>
            <td>Any domain</td>
        </tr>
         <tr>
            <td><code>--threads</code></td>
            <td>Number of threads to use.</td>
            <td>8</td>
            <td>Any positive integer</td>
        </tr>
        <tr>
            <td><code>--requests</code></td>
            <td>Number of requests to each DNS server.</td>
            <td>25</td>
            <td>Any positive integer</td>
        </tr>
        <tr>
            <td><code>--timeout</code></td>
            <td>Timeout in seconds.</td>
            <td>3</td>
            <td>Any positive integer</td>
        </tr>
        <tr>
            <td><code>--protocol</code></td>
            <td>Protocol to use.</td>
            <td>udp</td>
            <td>tcp, udp</td>
        </tr>
        <tr>
            <td><code>--name-servers-ip</code></td>
            <td>IP version to use for establishing connection.</td>
            <td>v4</td>
            <td>v4, v6</td>
        </tr>
        <tr>
            <td><code>--lookup-ip</code></td>
            <td>IP version to use for lookup.</td>
            <td>v4</td>
            <td>v4, v6</td>
        </tr>
        <tr>
            <td><code>--style</code></td>
            <td>Table style to use.</td>
            <td>rounded</td>
            <td>empty, blank, ascii, psql, markdown, modern, sharp, rounded, modern-rounded, extended, dots, re-structured-text, ascii-rounded</td>
        </tr>
        <tr>
            <td><code>--save-config</code></td>
            <td>Save the configurations to a file in users home directory.</td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td><code>--custom-servers-file</code></td>
            <td>Provide a path to a file with custom servers list to use instead of built-in list. An example of file format can be found <a href="https://github.com/qwerty541/dns-bench/blob/master/examples/ipv4-custom-servers-example.txt">here for IPv4</a> and <a href="https://github.com/qwerty541/dns-bench/blob/master/examples/ipv6-custom-servers-example.txt">here for IPv6</a>.</td>
            <td></td>
            <td></td>
        </tr>
    </tbody>
</table>

# License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](https://github.com/qwerty541/dns-bench/blob/master/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](https://github.com/qwerty541/dns-bench/blob/master/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
