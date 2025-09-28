# dns-bench (Docker)

A fast, cross-platform DNS benchmarking CLI. This image runs the `dns-bench` binary inside a container so you can benchmark public DNS servers and print results in multiple formats (table, JSON, XML, CSV).

- Source code: https://github.com/qwerty541/dns-bench
- Issues: https://github.com/qwerty541/dns-bench/issues
- Releases: https://github.com/qwerty541/dns-bench/releases
- Tags on Docker Hub: https://hub.docker.com/r/qwerty541/dns-bench/tags

## Preview

### Image

<img src="https://raw.githubusercontent.com/qwerty541/dns-bench/master/docs/assets/dockerhub-preview.png" width="100%" alt="Preview image" />

### Animation

<img src="https://raw.githubusercontent.com/qwerty541/dns-bench/master/docs/assets/dockerhub-preview.gif" width="100%" alt="Preview animation" />

## Quick start

Pull the image:

```sh
docker pull qwerty541/dns-bench:latest
```

Run an interactive benchmark (default command runs `dns-bench --skip-system-servers --skip-gateway-detection`):

```sh
docker run --rm -it --name dns-bench qwerty541/dns-bench:latest
```

Show help:

```sh
docker run --rm -it --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --help"
```

Run with options (example):

```sh
docker run --rm -it --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --requests 20 --domain microsoft.com --style re-structured-text"
```

## Volumes and configuration

- Custom DNS server list
  - Mount your file into the container and pass its path to `--custom-servers-file`.
  - File format examples:
    - [IPv4](https://github.com/qwerty541/dns-bench/blob/master/examples/ipv4-custom-servers-example.txt)
    - [IPv6](https://github.com/qwerty541/dns-bench/blob/master/examples/ipv6-custom-servers-example.txt)

Example (read-only mount):

```sh
docker run --rm -it --name dns-bench \
  --volume /path/to/ipv4-custom-servers.txt:/servers.txt:ro \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --custom-servers-file /servers.txt"
```

- Persisting configuration across runs
  - The app stores config in `/root/.dns-bench/config.toml` inside the container.
  - To persist between runs, bind-mount a host directory to `/root/.dns-bench`.

Example:

```sh
mkdir -p "$PWD/.dns-bench"
docker run --rm -it --name dns-bench \
  --volume "$PWD/.dns-bench:/root/.dns-bench" \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --save-config"
```

## Networking notes

- For the most accurate latency measurements on Linux, consider using host networking to minimize NAT overhead:

```sh
docker run --rm -it --name dns-bench \
  --network host \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench"
```

- Inside containers, system DNS autodetection is typically skipped. If you still want to include system DNS servers from the host you can override the default command as shown above.

## Output formats

`dns-bench` can print results as a human-readable table (default), JSON, XML, or CSV.

Examples:

```sh
# JSON
docker run --rm -it --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --format json"

# XML
docker run --rm -it --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --format xml"

# CSV (save to a file on the host)
docker run --rm --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --format csv" \
  > results.csv
```

## Running as non-root (optional)

If your Docker daemon user/group setup allows, you may run the container as your current user:

```sh
docker run --rm -it --name dns-bench \
  --user "$(id -u):$(id -g)" \
  --volume "$PWD/.dns-bench:/root/.dns-bench" \
  --volume /path/to/servers.txt:/servers.txt:ro \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --custom-servers-file /servers.txt"
```

Note: ensure mounted directories are readable/writable by the chosen UID/GID as needed.

## Examples

- Basic benchmark with defaults:

```sh
docker run --rm -it --name dns-bench qwerty541/dns-bench:latest
```

- Benchmark with 16 threads and 50 requests:

```sh
docker run --rm -it --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --threads 16 --requests 50"
```

- Use IPv6 for both lookup and server connection:

```sh
docker run --rm -it --name dns-bench \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --lookup-ip v6 --name-servers-ip v6"
```

- Persist config and reuse it:

```sh
mkdir -p "$PWD/.dns-bench"

# Save current flags
docker run --rm -it --name dns-bench \
  --volume "$PWD/.dns-bench:/root/.dns-bench" \
  qwerty541/dns-bench:latest \
  /bin/sh -c "dns-bench --save-config"

# Next run uses saved config by default
docker run --rm -it --name dns-bench \
  --volume "$PWD/.dns-bench:/root/.dns-bench" \
  qwerty541/dns-bench:latest
```

## Command-line reference

All CLI options and subcommands are documented in the project README:

- Options: https://github.com/qwerty541/dns-bench#options
- Subcommands: https://github.com/qwerty541/dns-bench#subcommands

## Troubleshooting

- No results or timeouts:
  - Ensure the container has outbound internet access.
  - Try `--network host` on Linux to reduce overhead.
  - Increase `--timeout` or reduce `--requests`/`--threads`.
- Custom servers file not loaded:
  - Verify the bind mount path and that the file is readable inside the container.
  - Validate format using the examples linked above.
- Config not persisted:
  - Ensure the `/root/.dns-bench` directory is bind-mounted and writable.

## License

Licensed under either of

- Apache License, Version 2.0: https://www.apache.org/licenses/LICENSE-2.0
- MIT license: https://opensource.org/licenses/MIT

Project license files:
- https://github.com/qwerty541/dns-bench/blob/master/LICENSE-APACHE
- https://github.com/qwerty541/dns-bench/blob/master/LICENSE-MIT
