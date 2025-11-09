#!/bin/bash

set -e

# Keep this line for situations where the default toolchain was changed, so the target needs to be re-added
rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu
