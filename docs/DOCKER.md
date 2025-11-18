# Docker Deployment Guide

This guide covers deploying the IP API using Docker and Docker Compose.

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Build and start both IPv4 and IPv6 instances
docker-compose up -d

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Using Docker Directly

```bash
# Build the image
docker build -t ip-api:latest .

# Run IPv4 instance
docker run -d \
  --name ip-api-ipv4 \
  -p 7111:7111 \
  -e PORT=7111 \
  ip-api:latest

# Run IPv6 instance
docker run -d \
  --name ip-api-ipv6 \
  -p 7112:7112 \
  -e PORT=7112 \
  ip-api:latest
```

## Configuration

### Environment Variables

All configuration is done through environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 7111 | Port to bind to |
| `RATE_LIMIT_REQUESTS` | 60 | Max requests per window |
| `RATE_LIMIT_WINDOW_SECS` | 60 | Rate limit window (seconds) |
| `DNS_CACHE_TTL_SECS` | 300 | DNS cache TTL (seconds) |
| `REQUEST_TIMEOUT_SECS` | 30 | Request timeout (seconds) |
| `RUST_LOG` | info | Log level (trace, debug, info, warn, error) |
| `LOG_FORMAT` | - | Set to `json` for JSON logs |

### Example with Custom Configuration

```bash
docker run -d \
  --name ip-api \
  -p 8080:8080 \
  -e PORT=8080 \
  -e RATE_LIMIT_REQUESTS=100 \
  -e RATE_LIMIT_WINDOW_SECS=60 \
  -e DNS_CACHE_TTL_SECS=600 \
  -e RUST_LOG=debug \
  -e LOG_FORMAT=json \
  ip-api:latest
```

## Docker Compose Configuration

### Custom docker-compose.yml

```yaml
version: '3.8'

services:
  ip-api:
    image: ip-api:latest
    ports:
      - "7111:7111"
    environment:
      - PORT=7111
      - RATE_LIMIT_REQUESTS=120
      - DNS_CACHE_TTL_SECS=600
      - RUST_LOG=info
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:7111/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

## Behind Reverse Proxy

### With Traefik

```yaml
version: '3.8'

services:
  ip-api-ipv4:
    image: ip-api:latest
    environment:
      - PORT=7111
    networks:
      - traefik
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.ipv4-api.rule=Host(`ipv4.example.com`)"
      - "traefik.http.routers.ipv4-api.entrypoints=websecure"
      - "traefik.http.routers.ipv4-api.tls.certresolver=letsencrypt"
      - "traefik.http.services.ipv4-api.loadbalancer.server.port=7111"

networks:
  traefik:
    external: true
```

### With NGINX

Use the NGINX configuration from `conf/nginx.md` and proxy to `http://ip-api-ipv4:7111` and `http://ip-api-ipv6:7112`.

## Building Custom Images

### Build with Specific Version

```bash
docker build -t ip-api:2.0.0 .
docker tag ip-api:2.0.0 ip-api:latest
```

### Multi-platform Build

```bash
docker buildx create --use
docker buildx build \
  --platform linux/amd64,linux/arm64 \
  -t ip-api:latest \
  .
```

## Health Checks

The Docker Compose configuration includes health checks using the `/health` endpoint:

```yaml
healthcheck:
  test: ["CMD", "curl", "-f", "http://localhost:7111/health"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 10s
```

Check health status:

```bash
docker ps
docker inspect --format='{{.State.Health.Status}}' ip-api-ipv4
```

## Logging

### View Logs

```bash
# Follow logs
docker-compose logs -f

# View specific service
docker-compose logs -f ip-api-ipv4

# Last 100 lines
docker-compose logs --tail=100
```

### JSON Logs

Enable JSON logging for easier parsing:

```yaml
environment:
  - LOG_FORMAT=json
```

## Resource Limits

Add resource limits in docker-compose.yml:

```yaml
services:
  ip-api-ipv4:
    # ... other config ...
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
        reservations:
          cpus: '0.25'
          memory: 128M
```

## Persistence

The application doesn't require persistent storage, but you can mount volumes for logs if needed:

```yaml
volumes:
  - ./logs:/var/log/ip-api
```

## Security

### Run as Non-Root

The Dockerfile already creates a non-root user (`ipapi`). The container runs as UID 1000.

### Read-Only Filesystem

```yaml
security_opt:
  - no-new-privileges:true
read_only: true
tmpfs:
  - /tmp
```

### Resource Isolation

```yaml
security_opt:
  - no-new-privileges:true
cap_drop:
  - ALL
cap_add:
  - NET_BIND_SERVICE
```

## Troubleshooting

### Container Won't Start

```bash
# Check logs
docker logs ip-api-ipv4

# Check if port is already in use
sudo ss -tulpn | grep 7111

# Inspect container
docker inspect ip-api-ipv4
```

### Health Check Failing

```bash
# Test health endpoint
curl http://localhost:7111/health

# Check if service is running
docker exec ip-api-ipv4 ps aux

# View detailed health status
docker inspect --format='{{json .State.Health}}' ip-api-ipv4 | jq
```

### DNS Resolution Issues

```bash
# Test DNS from inside container
docker exec ip-api-ipv4 nslookup google.com

# Check container network
docker network inspect bridge
```

## Production Recommendations

1. **Use specific version tags** instead of `latest`
2. **Enable health checks** for automatic restart on failure
3. **Set resource limits** to prevent resource exhaustion
4. **Use JSON logging** for log aggregation
5. **Run behind reverse proxy** with SSL/TLS
6. **Monitor container metrics** with Prometheus/Grafana
7. **Set up log rotation** if using file logging
8. **Enable automatic updates** with Watchtower or similar
9. **Back up your configuration** files

## Updates

### Update to New Version

```bash
# Rebuild image
docker-compose build

# Recreate containers
docker-compose up -d

# Clean up old images
docker image prune
```

### Rollback

```bash
# Use specific version
docker-compose down
# Edit docker-compose.yml to use previous version
docker-compose up -d
```