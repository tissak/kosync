#!/bin/bash

SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")
VERSION=$(awk -F '"' '/^version/ {print $2}' "$DIR/../Cargo.toml")

# Build Docker image for ARM64
cd "$DIR/.."
docker buildx build --platform linux/arm64 -f docker/Dockerfile.arm64 -t kosync:${VERSION}-arm64 .