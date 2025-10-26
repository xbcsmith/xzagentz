---
component:
  name: docker_comprehensive
  category: tools
  version: 2.0.0
  tier: comprehensive
  description: Comprehensive Docker reference with advanced features and workflows
  languages:
    - bash
  sections:
    - id: basic_commands
      language_specific: false
      required: true
    - id: container_lifecycle
      language_specific: false
      required: true
    - id: image_management
      language_specific: false
      required: true
    - id: networking
      language_specific: false
      required: true
    - id: volumes_storage
      language_specific: false
      required: true
    - id: docker_compose
      language_specific: false
      required: true
    - id: advanced_workflows
      language_specific: false
      required: false
    - id: performance_optimization
      language_specific: false
      required: false
---

# Docker - Comprehensive Guide

## Basic Commands

Essential Docker commands for daily development work.

### Container Operations

```bash
# Run containers with various options
docker run <image>
docker run -d <image>                    # Run in background (detached)
docker run -p 8080:80 <image>           # Map ports (host:container)
docker run --name myapp <image>         # Name the container
docker run -v $(pwd):/app <image>       # Mount volume
docker run -e KEY=value <image>         # Set environment variable
docker run --rm <image>                 # Auto-remove when stopped
docker run -it <image> /bin/bash        # Interactive terminal

# Advanced run options
docker run --memory="512m" <image>      # Memory limit
docker run --cpus="1.5" <image>         # CPU limit
docker run --restart=always <image>     # Restart policy
docker run --network=mynet <image>      # Custom network
docker run --user 1000:1000 <image>     # Run as specific user

# List containers
docker ps                                # Running containers
docker ps -a                             # All containers
docker ps -q                             # Container IDs only
docker ps --filter "status=exited"       # Filter by status
docker ps --format "{{.Names}}: {{.Status}}"  # Custom format

# Stop and remove containers
docker stop <container>                  # Graceful stop (SIGTERM)
docker stop -t 30 <container>           # Wait 30 seconds before SIGKILL
docker kill <container>                  # Force stop (SIGKILL)
docker rm <container>                    # Remove stopped container
docker rm -f <container>                 # Force remove running container
docker rm $(docker ps -aq)              # Remove all stopped containers
```

### Image Operations

```bash
# Pull and push images
docker pull <image>
docker pull <image>:<tag>
docker pull --platform linux/amd64 <image>  # Specific platform

# List and inspect images
docker images
docker images -a                         # Include intermediate images
docker images --filter "dangling=true"   # Unused images
docker image inspect <image>             # Detailed image info

# Remove images
docker rmi <image>
docker rmi -f <image>                    # Force remove
docker rmi $(docker images -q)          # Remove all images

# Build images
docker build -t myapp:latest .
docker build -f Dockerfile.prod -t myapp:prod .
docker build --build-arg VERSION=1.0 -t myapp:1.0 .
docker build --no-cache -t myapp:latest .
docker build --target production -t myapp:prod .  # Multi-stage target
```

### Inspection and Debugging

```bash
# View logs
docker logs <container>
docker logs -f <container>               # Follow logs (like tail -f)
docker logs --tail 100 <container>       # Last 100 lines
docker logs --since 1h <container>       # Logs from last hour
docker logs --timestamps <container>     # Include timestamps

# Inspect resources
docker inspect <container>
docker inspect --format '{{.State.Status}}' <container>
docker inspect --format '{{.NetworkSettings.IPAddress}}' <container>

# Execute commands
docker exec <container> <command>
docker exec -it <container> /bin/bash    # Interactive shell
docker exec -u root <container> <cmd>    # Run as root

# Resource monitoring
docker stats                             # Live resource usage
docker stats --no-stream                 # Single snapshot
docker top <container>                   # Running processes
```

---

## Container Lifecycle

Advanced container management and lifecycle operations.

### Container States

```
Created → Running → Paused → Running → Stopped → Removed
   ↓         ↓         ↓          ↓
   └────→ Exited ←────┴──────────┘
```

### Lifecycle Commands

```bash
# Create without starting
docker create --name myapp myimage

# Start and attach
docker start myapp
docker start -i myapp                    # Interactive
docker start -a myapp                    # Attach to stdout/stderr

# Attach to running container
docker attach myapp

# Pause and unpause
docker pause myapp                       # Freeze all processes
docker unpause myapp                     # Resume processes

# Restart container
docker restart myapp
docker restart -t 10 myapp              # Graceful restart with timeout

# Wait for container to stop
docker wait myapp

# Send signals
docker kill -s SIGUSR1 myapp            # Send custom signal
```

### Health Checks

```bash
# View health status
docker inspect --format='{{.State.Health.Status}}' <container>

# Dockerfile health check
HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD curl -f http://localhost/health || exit 1

# Disable health check
HEALTHCHECK NONE

# Run health check manually
docker exec <container> curl -f http://localhost/health
```

### Container Updates

```bash
# Update container configuration
docker update --memory="1g" <container>
docker update --cpus="2" <container>
docker update --restart=unless-stopped <container>

# Rename container
docker rename old-name new-name

# Export container filesystem
docker export <container> > container.tar

# Create image from container
docker commit <container> myimage:v1
docker commit -m "Added feature" -a "Author" <container> myimage:v1
```

---

## Image Management

Advanced image building, optimization, and management.

### Advanced Dockerfile

```dockerfile
# Multi-stage build
FROM golang:1.21 AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o myapp

FROM alpine:3.18
RUN apk --no-cache add ca-certificates
WORKDIR /root/
COPY --from=builder /app/myapp .
USER 1000:1000
EXPOSE 8080
HEALTHCHECK --interval=30s CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1
CMD ["./myapp"]

# Build arguments
ARG VERSION=latest
ARG BUILD_DATE
LABEL version="${VERSION}" \
      build-date="${BUILD_DATE}"

# Conditional execution
ARG ENV=production
RUN if [ "$ENV" = "development" ]; then \
      apk add --no-cache vim curl; \
    fi
```

### Build Optimization

```bash
# Use BuildKit for faster builds
DOCKER_BUILDKIT=1 docker build -t myapp .

# Cache management
docker build --cache-from myapp:latest -t myapp:v2 .
docker build --no-cache -t myapp .

# Multi-platform builds
docker buildx build --platform linux/amd64,linux/arm64 -t myapp .

# Layer caching strategy
docker build --target dependencies -t myapp:deps .
docker build --cache-from myapp:deps -t myapp:latest .
```

### Image Analysis

```bash
# View image history
docker history <image>
docker history --no-trunc <image>       # Full output

# Check image size
docker images <image>

# Scan for vulnerabilities
docker scan <image>

# Export and import images
docker save <image> > image.tar
docker save <image> | gzip > image.tar.gz
docker load < image.tar

# Import from tarball
docker import image.tar myapp:imported
```

---

## Networking

Advanced Docker networking configurations.

### Network Types

```bash
# List networks
docker network ls

# Create networks
docker network create mynetwork                    # Bridge (default)
docker network create --driver bridge mynetwork
docker network create --driver overlay myoverlay   # Swarm mode
docker network create --driver host myhost
docker network create --driver none mynone

# Create with subnet
docker network create --subnet=172.20.0.0/16 mynetwork

# Create with gateway
docker network create --subnet=172.20.0.0/16 \
  --gateway=172.20.0.1 mynetwork
```

### Network Management

```bash
# Inspect network
docker network inspect mynetwork

# Connect/disconnect containers
docker network connect mynetwork mycontainer
docker network disconnect mynetwork mycontainer

# Run container on specific network
docker run -d --network mynetwork --name myapp myimage

# Assign static IP
docker run -d --network mynetwork --ip 172.20.0.10 myapp

# Network aliases
docker run -d --network mynetwork --network-alias api myapp
```

### Port Mapping

```bash
# Map single port
docker run -p 8080:80 myapp              # host:container

# Map multiple ports
docker run -p 8080:80 -p 8443:443 myapp

# Map all exposed ports randomly
docker run -P myapp

# Bind to specific interface
docker run -p 127.0.0.1:8080:80 myapp

# UDP ports
docker run -p 8080:80/udp myapp
```

### DNS and Service Discovery

```bash
# Container DNS
docker run --hostname myservice myapp
docker run --dns 8.8.8.8 myapp
docker run --dns-search example.com myapp

# Link containers (legacy)
docker run --link db:database myapp

# Use embedded DNS (recommended)
docker network create mynetwork
docker run -d --network mynetwork --name db postgres
docker run -d --network mynetwork --name app \
  -e DATABASE_HOST=db myapp
```

---

## Volumes and Storage

Advanced volume and data management.

### Volume Types

```bash
# Named volumes
docker volume create myvolume
docker volume create --driver local \
  --opt type=none \
  --opt device=/path/on/host \
  --opt o=bind myvolume

# List and inspect volumes
docker volume ls
docker volume inspect myvolume

# Remove volumes
docker volume rm myvolume
docker volume prune                     # Remove unused volumes
```

### Volume Usage

```bash
# Mount named volume
docker run -v myvolume:/data myapp
docker run --mount source=myvolume,target=/data myapp

# Bind mount (host directory)
docker run -v /host/path:/container/path myapp
docker run -v /host/path:/container/path:ro myapp  # Read-only

# tmpfs mount (memory)
docker run --tmpfs /tmp myapp
docker run --mount type=tmpfs,destination=/tmp,tmpfs-size=100m myapp
```

### Data Management

```bash
# Copy data to/from containers
docker cp /host/file <container>:/path
docker cp <container>:/path /host/

# Backup volume
docker run --rm -v myvolume:/data -v $(pwd):/backup \
  alpine tar czf /backup/backup.tar.gz -C /data .

# Restore volume
docker run --rm -v myvolume:/data -v $(pwd):/backup \
  alpine tar xzf /backup/backup.tar.gz -C /data

# Clone volume
docker volume create newvolume
docker run --rm -v oldvolume:/from -v newvolume:/to \
  alpine sh -c "cd /from && cp -av . /to"
```

---

## Docker Compose

Multi-container application orchestration.

### Compose File

```yaml
version: '3.8'

services:
  web:
    build:
      context: .
      dockerfile: Dockerfile
      args:
        VERSION: 1.0
    image: myapp:latest
    container_name: myapp-web
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://postgres:secret@db:5432/mydb
      - REDIS_URL=redis://cache:6379
    volumes:
      - ./config:/app/config:ro
      - app-data:/app/data
    networks:
      - backend
    depends_on:
      - db
      - cache
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 3s
      retries: 3

  db:
    image: postgres:14
    container_name: myapp-db
    environment:
      - POSTGRES_PASSWORD=secret
      - POSTGRES_DB=mydb
    volumes:
      - db-data:/var/lib/postgresql/data
    networks:
      - backend
    restart: unless-stopped

  cache:
    image: redis:7-alpine
    container_name: myapp-cache
    networks:
      - backend
    restart: unless-stopped

networks:
  backend:
    driver: bridge

volumes:
  app-data:
  db-data:
```

### Compose Commands

```bash
# Start services
docker-compose up
docker-compose up -d                    # Detached mode
docker-compose up --build               # Rebuild images
docker-compose up --scale web=3         # Scale service

# Stop services
docker-compose stop
docker-compose down                     # Stop and remove
docker-compose down -v                  # Include volumes

# View status
docker-compose ps
docker-compose logs
docker-compose logs -f web              # Follow specific service

# Execute commands
docker-compose exec web bash
docker-compose run web python manage.py migrate

# Restart services
docker-compose restart
docker-compose restart web
```

---

## Advanced Workflows

### CI/CD Integration

```bash
# Build for CI
docker build --target test -t myapp:test .
docker run myapp:test npm test

# Multi-stage production build
docker build --target production -t myapp:prod .
docker tag myapp:prod registry.example.com/myapp:${CI_COMMIT_SHA}
docker push registry.example.com/myapp:${CI_COMMIT_SHA}
```

### Container Orchestration Prep

```bash
# Label containers for orchestration
docker run --label "env=production" \
  --label "version=1.0" myapp

# Filter by labels
docker ps --filter "label=env=production"

# Export for Kubernetes
docker save myapp:latest | gzip > myapp.tar.gz
```

### Debugging and Troubleshooting

```bash
# Enter running container
docker exec -it <container> /bin/sh

# Debug stopped container
docker commit <stopped-container> debug-image
docker run -it debug-image /bin/sh

# View container changes
docker diff <container>

# Check resource usage
docker stats --no-stream
docker system df                        # Disk usage

# Event monitoring
docker events
docker events --filter "container=myapp"
```

---

## Performance Optimization

### Resource Constraints

```bash
# Memory limits
docker run --memory="512m" --memory-swap="1g" myapp
docker run --memory-reservation="256m" myapp
docker run --oom-kill-disable myapp     # Disable OOM killer

# CPU limits
docker run --cpus="1.5" myapp           # 1.5 CPU cores
docker run --cpu-shares=512 myapp       # Relative weight
docker run --cpuset-cpus="0,1" myapp    # Specific cores

# I/O limits
docker run --device-read-bps /dev/sda:1mb myapp
docker run --device-write-bps /dev/sda:1mb myapp
```

### Image Optimization

```dockerfile
# Use minimal base images
FROM alpine:3.18                        # ~5MB
FROM gcr.io/distroless/base            # ~20MB

# Layer optimization
RUN apt-get update && apt-get install -y \
    package1 \
    package2 \
    && rm -rf /var/lib/apt/lists/*     # Single layer

# Multi-stage builds
FROM node:18 AS builder
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
RUN npm run build

FROM node:18-alpine
COPY --from=builder /app/dist /app
CMD ["node", "/app/server.js"]
```

### Caching Strategies

```bash
# Leverage build cache
COPY package.json package-lock.json ./
RUN npm ci                              # Cache node_modules
COPY . .                                # Copy source after deps

# Cache mount (BuildKit)
RUN --mount=type=cache,target=/root/.cache/pip \
    pip install -r requirements.txt
```

## Summary

This comprehensive guide covers all major Docker features including container lifecycle management, advanced networking, volume management, Docker Compose orchestration, CI/CD integration, and performance optimization. Use this as a complete reference for Docker operations.
