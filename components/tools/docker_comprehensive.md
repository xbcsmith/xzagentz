---
component:
  name: docker_comprehensive
  category: tools
  version: 1.0.0
  tier: comprehensive
  description: Comprehensive Docker reference with advanced operations and troubleshooting
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: dockerfile_advanced
      language_specific: true
      required: true
    - id: networking
      language_specific: false
      required: true
    - id: volumes
      language_specific: false
      required: true
    - id: troubleshooting
      language_specific: false
      required: true
---

# Docker Comprehensive Guidelines

Complete Docker reference including advanced operations, optimization, and troubleshooting.

---

## Advanced Dockerfile Patterns

<!-- LANG:rust -->

### Rust Multi-Stage Build

```dockerfile
# Builder stage
FROM rust:1.70 as builder

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# Build actual application
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 appuser
USER appuser

WORKDIR /app
COPY --from=builder --chown=appuser:appuser /app/target/release/app .

CMD ["./app"]
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Optimized Dockerfile

```dockerfile
FROM python:3.11-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends gcc && \
    rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt .
RUN pip wheel --no-cache-dir --no-deps --wheel-dir /app/wheels -r requirements.txt

# Runtime stage
FROM python:3.11-slim

WORKDIR /app

# Copy wheels and install
COPY --from=builder /app/wheels /wheels
RUN pip install --no-cache /wheels/*

# Create non-root user
RUN useradd -m -u 1000 appuser
USER appuser

COPY . .

CMD ["python", "app.py"]
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Optimized Dockerfile

```dockerfile
FROM golang:1.21 as builder

WORKDIR /app

# Cache dependencies
COPY go.mod go.sum ./
RUN go mod download

# Build application
COPY . .
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -ldflags="-w -s" -o app

# Runtime stage
FROM scratch

# Copy CA certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy binary
COPY --from=builder /app/app /app

ENTRYPOINT ["/app"]
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript/Node Production Dockerfile

```dockerfile
FROM node:20-alpine as builder

WORKDIR /app

# Install dependencies
COPY package*.json ./
RUN npm ci

# Build TypeScript
COPY tsconfig.json ./
COPY src ./src
RUN npm run build

# Runtime stage
FROM node:20-alpine

WORKDIR /app

# Install production dependencies only
COPY package*.json ./
RUN npm ci --production

# Copy built application
COPY --from=builder /app/dist ./dist

# Create non-root user
RUN addgroup -g 1000 appuser && \
    adduser -D -u 1000 -G appuser appuser
USER appuser

CMD ["node", "dist/index.js"]
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Script Container with Tools

```dockerfile
FROM alpine:latest

# Install common tools
RUN apk add --no-cache \
    bash \
    curl \
    jq \
    git \
    openssh-client

WORKDIR /scripts

COPY *.sh ./
RUN chmod +x *.sh

# Create non-root user
RUN addgroup -g 1000 scriptuser && \
    adduser -D -u 1000 -G scriptuser scriptuser
USER scriptuser

CMD ["./entrypoint.sh"]
```

<!-- /LANG -->

---

## Docker Compose Advanced

### Complete Application Stack

```yaml
version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile
      args:
        - BUILD_VERSION=${VERSION:-latest}
    image: myapp:${VERSION:-latest}
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://postgres:secret@db:5432/myapp
      - REDIS_URL=redis://redis:6379
      - LOG_LEVEL=info
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    restart: unless-stopped
    networks:
      - backend
    volumes:
      - app-data:/app/data
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"

  db:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=myapp
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=secret
    volumes:
      - db-data:/var/lib/postgresql/data
      - ./init.sql:/docker-entrypoint-initdb.d/init.sql:ro
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped
    networks:
      - backend

  redis:
    image: redis:7-alpine
    command: redis-server --appendonly yes
    volumes:
      - redis-data:/data
    restart: unless-stopped
    networks:
      - backend

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./certs:/etc/nginx/certs:ro
    depends_on:
      - app
    restart: unless-stopped
    networks:
      - backend

volumes:
  app-data:
  db-data:
  redis-data:

networks:
  backend:
    driver: bridge
```

### Development Override

```yaml
# docker-compose.override.yaml
version: '3.8'

services:
  app:
    build:
      target: development
    volumes:
      - .:/app
      - /app/target
    environment:
      - RUST_LOG=debug
    command: cargo watch -x run
```

---

## Networking

### Network Types

```bash
# Create bridge network
docker network create myapp-network

# Create overlay network (Swarm)
docker network create --driver overlay myapp-overlay

# Create host network (no isolation)
docker run --network host myapp

# Create custom bridge with subnet
docker network create --subnet=172.18.0.0/16 custom-network
```

### Network Commands

```bash
# List networks
docker network ls

# Inspect network
docker network inspect myapp-network

# Connect container to network
docker network connect myapp-network myapp

# Disconnect container from network
docker network disconnect myapp-network myapp

# Remove network
docker network rm myapp-network

# Prune unused networks
docker network prune
```

### Network Troubleshooting

```bash
# Test connectivity between containers
docker exec app ping db

# Check DNS resolution
docker exec app nslookup db

# View network configuration
docker inspect --format='{{json .NetworkSettings}}' myapp

# List container IPs
docker inspect --format='{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' myapp
```

---

## Volume Management

### Volume Types

**Named volumes:**

```bash
# Create volume
docker volume create myapp-data

# Use in container
docker run -v myapp-data:/app/data myapp
```

**Bind mounts:**

```bash
# Mount host directory
docker run -v $(pwd)/data:/app/data myapp

# Read-only mount
docker run -v $(pwd)/config:/app/config:ro myapp
```

**Tmpfs mounts:**

```bash
# In-memory mount
docker run --tmpfs /tmp myapp
```

### Volume Commands

```bash
# List volumes
docker volume ls

# Inspect volume
docker volume inspect myapp-data

# Remove volume
docker volume rm myapp-data

# Prune unused volumes
docker volume prune

# Backup volume
docker run --rm -v myapp-data:/data -v $(pwd):/backup alpine tar czf /backup/data.tar.gz /data

# Restore volume
docker run --rm -v myapp-data:/data -v $(pwd):/backup alpine tar xzf /backup/data.tar.gz -C /
```

---

## Container Management Advanced

### Resource Limits

```bash
# Limit CPU
docker run --cpus=2 myapp

# Limit memory
docker run --memory=512m myapp

# Limit memory with swap
docker run --memory=512m --memory-swap=1g myapp

# Set CPU shares (relative weight)
docker run --cpu-shares=512 myapp

# Set memory reservation
docker run --memory-reservation=256m myapp
```

### Health Checks

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1
```

```bash
# Check health status
docker inspect --format='{{.State.Health.Status}}' myapp

# View health check logs
docker inspect --format='{{json .State.Health}}' myapp
```

---

## Registry Operations

### Docker Registry

```bash
# Login to registry
docker login registry.example.com

# Tag image for registry
docker tag myapp:latest registry.example.com/myapp:v1.0.0

# Push to registry
docker push registry.example.com/myapp:v1.0.0

# Pull from registry
docker pull registry.example.com/myapp:v1.0.0

# Search registry
docker search nginx
```

### Private Registry

```bash
# Run local registry
docker run -d -p 5000:5000 --name registry registry:2

# Tag for local registry
docker tag myapp:latest localhost:5000/myapp:latest

# Push to local registry
docker push localhost:5000/myapp:latest
```

---

## Docker Build Advanced

### Build Arguments

```dockerfile
ARG BUILD_VERSION=1.0.0
ARG BUILD_DATE
ARG VCS_REF

LABEL version="${BUILD_VERSION}"
LABEL build-date="${BUILD_DATE}"
LABEL vcs-ref="${VCS_REF}"
```

```bash
# Pass build arguments
docker build \
  --build-arg BUILD_VERSION=1.0.0 \
  --build-arg BUILD_DATE=$(date -u +'%Y-%m-%dT%H:%M:%SZ') \
  --build-arg VCS_REF=$(git rev-parse --short HEAD) \
  -t myapp:1.0.0 .
```

### Build Cache

```bash
# Build without cache
docker build --no-cache -t myapp .

# Use cache from another image
docker build --cache-from=myapp:latest -t myapp:new .

# Use BuildKit for better caching
DOCKER_BUILDKIT=1 docker build -t myapp .
```

### Multi-Platform Builds

```bash
# Enable buildx
docker buildx create --use

# Build for multiple platforms
docker buildx build --platform linux/amd64,linux/arm64 -t myapp:latest .

# Push multi-platform image
docker buildx build --platform linux/amd64,linux/arm64 -t myapp:latest --push .
```

---

## Troubleshooting

### Container Issues

**Container won't start:**

```bash
# Check logs
docker logs myapp

# Inspect exit code
docker inspect --format='{{.State.ExitCode}}' myapp

# Run with interactive terminal
docker run -it myapp bash
```

**Container running but not accessible:**

```bash
# Check port mappings
docker port myapp

# Check if process is listening
docker exec myapp netstat -tuln

# Check firewall rules
sudo iptables -L -n
```

**High CPU/Memory usage:**

```bash
# Check stats
docker stats myapp

# View top processes
docker top myapp

# Limit resources
docker update --cpus=1 --memory=512m myapp
```

### Build Issues

**Build fails with out of space:**

```bash
# Clean up
docker system prune -a --volumes

# Check disk usage
docker system df
```

**Slow builds:**

```bash
# Use BuildKit
export DOCKER_BUILDKIT=1

# Add .dockerignore
echo "node_modules" >> .dockerignore
echo "target" >> .dockerignore
```

### Network Issues

**Containers can't communicate:**

```bash
# Check network
docker network inspect bridge

# Test connectivity
docker exec app ping db

# Check DNS
docker exec app cat /etc/resolv.conf
```

---

## Security Best Practices

### Image Security

```bash
# Scan for vulnerabilities
docker scan myapp:latest

# Use trusted base images
FROM docker.io/library/alpine:3.18

# Verify signatures
docker trust inspect myapp:latest
```

### Runtime Security

```dockerfile
# Don't run as root
RUN useradd -m -u 1000 appuser
USER appuser

# Use read-only filesystem
docker run --read-only myapp

# Drop capabilities
docker run --cap-drop=ALL --cap-add=NET_BIND_SERVICE myapp

# Use security options
docker run --security-opt=no-new-privileges myapp
```

### Secrets Management

```bash
# Use Docker secrets (Swarm)
echo "db-password" | docker secret create db_password -

# Use in service
docker service create --secret db_password myapp

# Use environment variables for development only
docker run -e SECRET_KEY=$(cat secret.key) myapp
```

---

## Performance Optimization

### Image Size Optimization

```dockerfile
# Use minimal base images
FROM alpine:3.18

# Combine RUN commands
RUN apk add --no-cache curl && \
    curl -o /tmp/file.tar.gz https://example.com/file.tar.gz && \
    tar xzf /tmp/file.tar.gz && \
    rm /tmp/file.tar.gz

# Remove unnecessary files
RUN apt-get update && \
    apt-get install -y package && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*
```

### Layer Caching

```dockerfile
# Copy dependency files first
COPY package.json package-lock.json ./
RUN npm ci

# Copy source code last
COPY . .
```

### Runtime Performance

```bash
# Use host network for better performance
docker run --network host myapp

# Increase shared memory
docker run --shm-size=1g myapp

# Use volumes for better I/O
docker run -v myapp-data:/data myapp
```

---

## Monitoring and Logging

### Logging Drivers

```bash
# JSON file (default)
docker run --log-driver json-file --log-opt max-size=10m --log-opt max-file=3 myapp

# Syslog
docker run --log-driver syslog --log-opt syslog-address=tcp://192.168.0.42:514 myapp

# Journald
docker run --log-driver journald myapp
```

### Monitoring

```bash
# Real-time stats
docker stats

# Export metrics
docker inspect --format='{{json .State}}' myapp

# Use Prometheus
docker run -d -p 9090:9090 prom/prometheus
```

---

## Docker Swarm Basics

### Initialize Swarm

```bash
# Initialize manager
docker swarm init

# Join as worker
docker swarm join --token TOKEN manager-ip:2377

# Deploy stack
docker stack deploy -c docker-compose.yaml myapp

# List services
docker service ls

# Scale service
docker service scale myapp_web=3
```

---

## Best Practices Summary

### Dockerfile

- Use specific versions for base images
- Leverage multi-stage builds
- Minimize layers
- Use .dockerignore
- Run as non-root user
- Add health checks

### Operations

- Use docker-compose for local development
- Tag images with versions
- Monitor resource usage
- Regular cleanup of unused resources
- Use named volumes for persistence

### Security

- Scan images regularly
- Keep images updated
- Use secrets management
- Limit container capabilities
- Use read-only filesystems when possible
