Source: https://github.com/qwerty541/dns-bench

# Description

This repository provides a DNS benchmarking command-line tool written in Rust. It iterates through a built-in list of public DNS servers as well as automatically detected system DNS servers, measures their response times, and prints a table of sorted results in the console. You can use it to find the fastest DNS server for your location, improving your internet browsing experience. A preview, list of features, and the list of built-in DNS servers are provided below.

## Preview

### Image

<img src="https://raw.githubusercontent.com/qwerty541/dns-bench/master/docs/assets/dockerhub-preview.png" width="100%" alt="Preview image" />

### Animation

<img src="https://raw.githubusercontent.com/qwerty541/dns-bench/master/docs/assets/dockerhub-preview.gif" width="100%" alt="Preview animation" />

## Features

### 🚀 Core Features

- **Built-in list of public DNS servers**  
  Includes popular providers like Google, Cloudflare, Quad9, and more.
- **Automatic detection of system DNS servers**  
  Detects and highlights your system's configured DNS servers (Linux, Windows, macOS).
- **Multi-threaded benchmarking**  
  Runs benchmarks in parallel for faster results.

### ⚙️ Configuration & Flexibility

- **Customizable request count, thread count, timeout, and domain**  
  Fine-tune how many requests, threads, and which domain to test.
- **Protocol and IP version selection**  
  Choose between UDP/TCP and IPv4/IPv6 for both lookup and connection.
- **Custom DNS server lists**  
  Use your own list of DNS servers instead of the built-in set.

### 📊 Output & Usability

- **Multiple output formats**  
  Human-readable table, JSON, XML, or CSV for easy integration and analysis.
- **Configurable table styles**  
  Choose from various table styles for better readability.
- **Save favorite configurations**  
  Store your preferred settings in a config file for quick reuse.
- **Config file management without running benchmarks**  
  Use [subcommands](#subcommands) to manage your config independently from benchmarking.

### 🐳 Platform & Integration

- **Docker support**  
  Run easily in a containerized environment (system DNS detection is skipped in Docker).
- **Cross-platform**  
  Works on Linux, Windows, and macOS.

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

</td><td>

- Dyn
- Hurricane Electric
- Surfshark DNS
- SafeServe
- Vercara UltraDNS Public

</td></tr>
</table>

# Usage

Run the following command to pull the image:

```sh
$ docker pull qwerty541/dns-bench:latest
```

Now you can run this tool inside the container:

```sh
$ docker run --rm -it --name dns-bench qwerty541/dns-bench:latest
```

If you want to pass some options, you can do it like this:

```sh
$ docker run --rm -it --name dns-bench qwerty541/dns-bench:latest /bin/bash -c "dns-bench --requests 20 --domain microsoft.com --style re-structured-text"
```

In case you want to use custom servers list, you have to mount the file with custom servers list to the container and pass the path to the file as an argument:

```sh
$ docker run --rm -it --name dns-bench --volume /home/alexandr/projects/dns-bench/examples/ipv4-custom-servers-example.txt:/ipv4-custom-servers-example.txt qwerty541/dns-bench:latest /bin/bash -c "dns-bench --custom-servers-file /ipv4-custom-servers-example.txt"
```

# Command-Line Reference

## Options

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
            <td>1..256</td>
        </tr>
        <tr>
            <td><code>--requests</code></td>
            <td>Number of requests to each DNS server.</td>
            <td>25</td>
            <td>1..1000</td>
        </tr>
        <tr>
            <td><code>--timeout</code></td>
            <td>Timeout in seconds.</td>
            <td>3</td>
            <td>1..60</td>
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
        <tr>
            <td><code>--format</code></td>
            <td>Format of the output.</td>
            <td>human-readable</td>
            <td>human-readable, json, xml, csv</td>
        </tr>
        <tr>
            <td><code>--skip-system-servers</code></td>
            <td>Skip auto-detection of system DNS servers.</td>
            <td></td>
            <td></td>
        </tr>
    </tbody>
</table>

## Subcommands

Below is a list of currently supported subcommands.

<table>
    <thead>
        <th>Subcommand</th>
        <th>Description</th>
    </thead>
    <tbody>
        <tr>
            <td><code>dns-bench config init</code></td>
            <td>Create a config file with default values if it does not exist.</td>
        </tr>
        <tr>
            <td><code>dns-bench config set [--key value ...]</code></td>
            <td>Set one or more config values. Supports all options listed above.</td>
        </tr>
        <tr>
            <td><code>dns-bench config reset</code></td>
            <td>Reset config file to default values.</td>
        </tr>
        <tr>
            <td><code>dns-bench config delete</code></td>
            <td>Delete config file.</td>
        </tr>
    </tbody>
</table>

# License

Licensed under either of

-   Apache License, Version 2.0, ([LICENSE-APACHE](https://github.com/qwerty541/dns-bench/blob/master/LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
-   MIT license ([LICENSE-MIT](https://github.com/qwerty541/dns-bench/blob/master/LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
