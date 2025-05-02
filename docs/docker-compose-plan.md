# Docker Compose Deployment Plan for Raspberry Pi 4

This document outlines the plan for deploying KOSync to a Raspberry Pi 4 using Docker Compose.

## Current Docker Setup

The current Docker setup consists of:

1. A simple Dockerfile using Alpine Linux as the base image:
```dockerfile
FROM alpine:latest
WORKDIR /srv
COPY kosync /srv
CMD ["/srv/kosync"]
```

2. A build script (`docker/make.sh`) that:
   - Builds the application for x86_64 architecture
   - Creates a Docker image
   - Cleans up temporary files

## Issues for Raspberry Pi 4 Deployment

1. **Architecture mismatch**: The current build targets x86_64, but Raspberry Pi 4 uses ARM64/aarch64
2. **No Docker Compose configuration**: Missing docker-compose.yml for easy deployment
3. **Data persistence**: No explicit volume configuration for database persistence
4. **Limited configuration**: Environment variables could be better documented

## Implementation Plan

### 1. Create docker-compose.yml

Create a `docker-compose.yml` file in the project root with the following configuration:

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

### 2. Create ARM64-specific Dockerfile

Create a `docker/Dockerfile.arm64` file:

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

### 3. Update build script for ARM64

Create a new build script `docker/make-arm64.sh`:

```bash
#!/bin/bash

SCRIPT=$(readlink -f "$0")
DIR=$(dirname "$SCRIPT")
VERSION=$(awk -F '"' '/^version/ {print $2}' "$DIR/../Cargo.toml")

# Build Docker image for ARM64
cd "$DIR/.."
docker buildx build --platform linux/arm64 -f docker/Dockerfile.arm64 -t kosync:${VERSION}-arm64 .
```

### 4. Create deployment documentation

Create a file `docs/raspberry-pi-deployment.md`:

```markdown
# Deploying KOSync to Raspberry Pi 4

This guide explains how to deploy KOSync to a Raspberry Pi 4 using Docker Compose.

## Prerequisites

- Raspberry Pi 4 with 64-bit OS (e.g., Raspberry Pi OS 64-bit)
- Docker and Docker Compose installed
- Git installed

## Deployment Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/kosync.git
   cd kosync
   ```

2. Deploy using Docker Compose:
   ```bash
   docker-compose up -d
   ```

3. Verify the deployment:
   ```bash
   curl http://localhost:3044/healthcheck
   ```

## Configuration

KOSync can be configured using environment variables in the docker-compose.yml file:

- `KOSYNC_ADDR`: The address and port to listen on (default: 0.0.0.0:3044)

## Data Persistence

The database is stored in a Docker volume named `kosync-data`. This ensures that your data persists across container restarts and updates.

To backup your data:

```bash
docker run --rm -v kosync-data:/data -v $(pwd):/backup alpine tar -czf /backup/kosync-data-backup.tar.gz -C /data .
```

To restore from a backup:

```bash
docker run --rm -v kosync-data:/data -v $(pwd):/backup alpine sh -c "rm -rf /data/* && tar -xzf /backup/kosync-data-backup.tar.gz -C /data"
```

## Troubleshooting

If you encounter issues:

1. Check the container logs:
   ```bash
   docker-compose logs kosync
   ```

2. Ensure the container is running:
   ```bash
   docker-compose ps
   ```

3. Verify the health check is passing:
   ```bash
   docker inspect --format "{{json .State.Health }}" kosync
   ```
```

## Implementation Steps

1. Create the `docker-compose.yml` file in the project root
2. Create the `docker/Dockerfile.arm64` file
3. Create the `docker/make-arm64.sh` build script
4. Create the `docs/raspberry-pi-deployment.md` documentation
5. Test the deployment on a Raspberry Pi 4
6. Update the README.md to include information about Raspberry Pi deployment

## Benefits

1. **Simplified deployment**: Users can deploy with a single `docker-compose up -d` command
2. **Architecture compatibility**: The ARM64 build will run natively on Raspberry Pi 4
3. **Data persistence**: The Docker volume ensures data is preserved across container restarts
4. **Automatic recovery**: The restart policy ensures the service recovers from failures
5. **Health monitoring**: The health check ensures the service is functioning properly