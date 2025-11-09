#!/bin/bash

set -e

# Ensure the target is added in case the default toolchain was changed
rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu
