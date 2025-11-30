#!/usr/bin/env bash
set -euo pipefail

# Defaults (override with: IMAGE=... NAME=... ./test.sh ...)
IMAGE="${IMAGE:-dns-bench-test:latest}"
NAME="${NAME:-dns-bench}"

docker run --rm -it --name "$NAME" "$IMAGE" \
  --skip-system-servers --skip-gateway-detection --timeout 1 --threads 16 --requests 25 \
  "$@"
