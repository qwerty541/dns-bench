FROM rust:1.78.0

RUN cargo install --git https://github.com/qwerty541/dns-bench.git --tag v0.5.1 dns-bench

CMD ["dns-bench"]
