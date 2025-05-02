# Deploying KOSync to Raspberry Pi 4

This guide explains how to deploy KOSync to a Raspberry Pi 4 using Docker Compose.

## Prerequisites

- Raspberry Pi 4 with 64-bit OS (e.g., Raspberry Pi OS 64-bit)
- Docker and Docker Compose installed
- Git installed (optional, if cloning the repository)

## Deployment Steps

### Option 1: Using Pre-built Images

1. Create a `docker-compose.yml` file with the following content:

```yaml
version: '3'

services:
  kosync:
    image: kosync:arm64  # Replace with your image name if published to a registry
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

2. Deploy using Docker Compose:

```bash
docker-compose up -d
```

### Option 2: Building from Source

1. Clone the repository:

```bash
git clone https://github.com/yourusername/kosync.git
cd kosync
```

2. Build and deploy using Docker Compose:

```bash
docker-compose up -d --build
```

### Verifying the Deployment

Check if the service is running:

```bash
docker-compose ps
```

Test the health check endpoint:

```bash
curl http://localhost:3044/healthcheck
```

## Configuration

KOSync can be configured using environment variables in the docker-compose.yml file:

| Variable | Description | Default |
|----------|-------------|---------|
| KOSYNC_ADDR | The address and port to listen on | 0.0.0.0:3044 |

Example configuration with custom port:

```yaml
environment:
  - KOSYNC_ADDR=0.0.0.0:8080
```

## Data Persistence

The database is stored in a Docker volume named `kosync-data`. This ensures that your data persists across container restarts and updates.

### Backing Up Data

To create a backup of your data:

```bash
docker run --rm -v kosync-data:/data -v $(pwd):/backup alpine tar -czf /backup/kosync-data-backup.tar.gz -C /data .
```

This will create a file named `kosync-data-backup.tar.gz` in your current directory.

### Restoring from Backup

To restore from a backup:

```bash
# Stop the container first
docker-compose down

# Restore the data
docker run --rm -v kosync-data:/data -v $(pwd):/backup alpine sh -c "rm -rf /data/* && tar -xzf /backup/kosync-data-backup.tar.gz -C /data"

# Start the container again
docker-compose up -d
```

## Updating KOSync

To update to a newer version:

1. Pull the latest changes (if building from source):

```bash
git pull
```

2. Rebuild and restart the container:

```bash
docker-compose down
docker-compose up -d --build
```

## Troubleshooting

### Container Not Starting

Check the container logs:

```bash
docker-compose logs kosync
```

### Health Check Failing

Verify the health check status:

```bash
docker inspect --format "{{json .State.Health }}" kosync
```

### Cannot Connect to the Service

1. Ensure the container is running:

```bash
docker-compose ps
```

2. Check if the port is correctly mapped:

```bash
docker-compose port kosync 3044
```

3. Verify that the service is listening on the correct interface:

```bash
docker exec kosync netstat -tulpn | grep 3044
```

### Performance Issues

If you experience performance issues on your Raspberry Pi:

1. Monitor resource usage:

```bash
docker stats kosync
```

2. Consider adjusting the JVM memory settings if they're too high for your Raspberry Pi model.

## Advanced Configuration

### Using a Custom Data Directory

If you want to store the data in a specific directory on your host system instead of using a Docker volume:

```yaml
volumes:
  - /path/on/host/data:/srv/data
```

### Setting Up Behind a Reverse Proxy

If you're running KOSync behind a reverse proxy like Nginx or Traefik:

1. Update the docker-compose.yml to only expose the port locally:

```yaml
ports:
  - "127.0.0.1:3044:3044"
```

2. Configure your reverse proxy to forward requests to `http://localhost:3044`.

## Resources

- [KOSync GitHub Repository](https://github.com/yourusername/kosync)
- [KOReader Documentation](https://github.com/koreader/koreader/wiki)
- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)