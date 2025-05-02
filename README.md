# KOReader progress sync server

The API is compatible with `koreader-sync-server`, but instead of Redis, `kosync` uses `sled`.

- bin linux-musl-static 2.1MB
- docker image compressed 4.11MB

```bash
KOSYNC_ADDR=0.0.0.0:3000 ./kosync
```

## docker

```bash
docker pull lzyorstudio/kosync
```

## build

for linux

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

for docker

```bash
./docker/make.sh
```

## Raspberry Pi 4 Deployment

KOSync can be easily deployed to a Raspberry Pi 4 using Docker Compose.

### Prerequisites

- Raspberry Pi 4 with 64-bit OS
- Docker and Docker Compose installed

### Deployment

1. Clone the repository:

```bash
git clone https://github.com/yourusername/kosync.git
cd kosync
```

2. Deploy using Docker Compose:

```bash
docker-compose up -d
```

This will:
- Build the ARM64-compatible Docker image
- Create a persistent volume for data storage
- Start the KOSync server on port 3044

### Building ARM64 Docker Image

To build the ARM64 Docker image manually:

```bash
./docker/make-arm64.sh
```

### Multi-Architecture Support

To build a multi-architecture Docker image that supports both x86_64 and ARM64:

```bash
./docker/make-multi.sh
```

For more detailed information about Raspberry Pi deployment, see [docs/raspberry-pi-deployment.md](docs/raspberry-pi-deployment.md).

## WIP
