#!/bin/bash

set -e

rustup toolchain install beta
rustup toolchain install nightly

rustup target add x86_64-pc-windows-gnu
