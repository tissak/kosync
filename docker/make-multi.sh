#!/bin/bash

SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")
VERSION=$(awk -F '"' '/^version/ {print $2}' "$DIR/../Cargo.toml")

# Create and use a new builder instance
docker buildx create --name mybuilder --use

# Build multi-architecture image
cd "$DIR/.."
docker buildx build --platform linux/amd64,linux/arm64 \
  -f docker/Dockerfile.multi \
  -t kosync:${VERSION} \
  --push .