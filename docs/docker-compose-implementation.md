# Docker Compose Implementation

This document contains the exact content of the files that need to be created for deploying KOSync to a Raspberry Pi 4 using Docker Compose.

## docker-compose.yml

Create this file in the project root:

```yaml
version: '3'

services:
  kosync:
    build:
      context: .
      dockerfile: docker/Dockerfile.arm64
    image: kosync:arm64
    container_name: kosync
    restart: unless-stopped
    ports:
      - "3044:3044"
    environment:
      - KOSYNC_ADDR=0.0.0.0:3044
    volumes:
      - kosync-data:/srv/data
    healthcheck:
      test: ["CMD", "wget", "-q", "--spider", "http://localhost:3044/healthcheck"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 5s

volumes:
  kosync-data:
    driver: local
```

## docker/Dockerfile.arm64

Create this file in the docker directory:

```dockerfile
FROM --platform=linux/arm64 rust:1.70-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create a new empty project
WORKDIR /usr/src/kosync
COPY . .

# Build the application
RUN cargo build --release --target aarch64-unknown-linux-musl

# Runtime stage
FROM --platform=linux/arm64 alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create app directory
WORKDIR /srv

# Copy the binary from the builder stage
COPY --from=builder /usr/src/kosync/target/aarch64-unknown-linux-musl/release/kosync /srv/kosync

# Create data directory and set permissions
RUN mkdir -p /srv/data && chmod 777 /srv/data

# Expose the port
EXPOSE 3044

# Set the entrypoint
CMD ["/srv/kosync"]
```

## docker/make-arm64.sh

Create this file in the docker directory:

```bash
#!/bin/bash

SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")
VERSION=$(awk -F '"' '/^version/ {print $2}' "$DIR/../Cargo.toml")

# Build Docker image for ARM64
cd "$DIR/.."
docker buildx build --platform linux/arm64 -f docker/Dockerfile.arm64 -t kosync:${VERSION}-arm64 .
```

Make the script executable:

```bash
chmod +x docker/make-arm64.sh
```

## Alternative Approach: Cross-Platform Build

If you want to support both x86_64 and ARM64 architectures, you can use Docker Buildx to create multi-architecture images:

```bash
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
```

And create a `Dockerfile.multi` that supports both architectures:

```dockerfile
FROM --platform=$BUILDPLATFORM rust:1.70-alpine AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create a new empty project
WORKDIR /usr/src/kosync
COPY . .

# Build for the target platform
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
      "linux/amd64") TARGET="x86_64-unknown-linux-musl" ;; \
      "linux/arm64") TARGET="aarch64-unknown-linux-musl" ;; \
      *) TARGET="x86_64-unknown-linux-musl" ;; \
    esac && \
    cargo build --release --target $TARGET && \
    mkdir -p /build && \
    cp target/$TARGET/release/kosync /build/

# Runtime stage
FROM --platform=$TARGETPLATFORM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache ca-certificates tzdata

# Create app directory
WORKDIR /srv

# Copy the binary from the builder stage
COPY --from=builder /build/kosync /srv/kosync

# Create data directory and set permissions
RUN mkdir -p /srv/data && chmod 777 /srv/data

# Expose the port
EXPOSE 3044

# Set the entrypoint
CMD ["/srv/kosync"]
```

## Implementation Notes

1. The Docker Compose configuration:
   - Maps port 3044 to the host
   - Sets up a volume for data persistence
   - Configures a health check
   - Sets a restart policy

2. The ARM64 Dockerfile:
   - Uses a multi-stage build to minimize image size
   - Builds specifically for the aarch64-unknown-linux-musl target
   - Creates a data directory for persistence

3. The build script:
   - Extracts the version from Cargo.toml
   - Uses Docker Buildx for ARM64 builds

4. For deployment:
   - Simply run `docker-compose up -d` on the Raspberry Pi
   - The database will be stored in a Docker volume