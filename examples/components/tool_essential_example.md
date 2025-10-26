---
component:
  name: docker_essential
  category: tools
  version: 2.0.0
  tier: essential
  description: Essential Docker commands and workflows for containerization
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
    - id: common_workflows
      language_specific: false
      required: true
---

# Docker - Essential Guide

## Basic Commands

Essential Docker commands for daily development work.

### Container Operations

```bash
# Run a container
docker run <image>
docker run -d <image>                    # Run in background (detached)
docker run -p 8080:80 <image>           # Map ports (host:container)
docker run --name myapp <image>         # Name the container
docker run -v $(pwd):/app <image>       # Mount volume

# List containers
docker ps                                # Running containers
docker ps -a                             # All containers (including stopped)

# Stop and remove containers
docker stop <container>                  # Stop container gracefully
docker kill <container>                  # Force stop container
docker rm <container>                    # Remove stopped container
docker rm -f <container>                 # Force remove running container
```

### Image Operations

```bash
# Pull images from registry
docker pull <image>
docker pull <image>:<tag>

# List local images
docker images
docker images -a                         # Include intermediate images

# Remove images
docker rmi <image>
docker rmi -f <image>                    # Force remove

# Build images
docker build -t myapp:latest .
docker build -f Dockerfile.prod -t myapp:prod .
```

### Inspection and Logs

```bash
# View container logs
docker logs <container>
docker logs -f <container>               # Follow logs (like tail -f)
docker logs --tail 100 <container>       # Last 100 lines

# Inspect containers
docker inspect <container>
docker inspect --format '{{.State.Status}}' <container>

# Execute commands in running container
docker exec <container> <command>
docker exec -it <container> /bin/bash    # Interactive shell
```

---

## Container Lifecycle

Understanding container states and lifecycle.

### Container States

```
Created → Running → Stopped → Removed
   ↓         ↓
   └────→ Exited ←─────┘
```

### Lifecycle Commands

```bash
# Create container without starting
docker create --name myapp myimage

# Start existing container
docker start myapp
docker start -i myapp                    # Start with interactive terminal

# Restart container
docker restart myapp

# Pause and unpause
docker pause myapp                       # Pause processes
docker unpause myapp                     # Resume processes

# Stop container
docker stop myapp                        # Graceful stop (SIGTERM)
docker stop -t 30 myapp                  # Wait 30 seconds before killing

# Remove container
docker rm myapp
docker rm -f myapp                       # Force remove running container
```

### Health Checks

```bash
# View container health status
docker inspect --format='{{.State.Health.Status}}' <container>

# Example Dockerfile with health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost/ || exit 1
```

---

## Image Management

Building and managing Docker images.

### Dockerfile Basics

```dockerfile
# Essential Dockerfile structure
FROM ubuntu:22.04

# Set working directory
WORKDIR /app

# Copy files
COPY . .

# Install dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    && rm -rf /var/lib/apt/lists/*

# Expose port
EXPOSE 8080

# Run command
CMD ["python3", "app.py"]
```

### Building Images

```bash
# Build from Dockerfile
docker build -t myapp:latest .

# Build with specific Dockerfile
docker build -f Dockerfile.dev -t myapp:dev .

# Build with build arguments
docker build --build-arg VERSION=1.0 -t myapp:1.0 .

# Build without cache
docker build --no-cache -t myapp:latest .
```

### Tagging Images

```bash
# Tag image
docker tag myapp:latest myapp:v1.0
docker tag myapp:latest myregistry.com/myapp:latest

# Multiple tags for same image
docker build -t myapp:latest -t myapp:v1.0 .
```

### Registry Operations

```bash
# Login to registry
docker login
docker login myregistry.com

# Push image to registry
docker push myapp:latest
docker push myregistry.com/myapp:latest

# Pull specific version
docker pull myapp:v1.0
```

---

## Common Workflows

Practical Docker workflows for development.

### Development Workflow

```bash
# 1. Build application image
docker build -t myapp:dev .

# 2. Run with development settings
docker run -d \
  --name myapp-dev \
  -p 8080:8080 \
  -v $(pwd):/app \
  -e ENV=development \
  myapp:dev

# 3. View logs
docker logs -f myapp-dev

# 4. Make changes (code auto-reloads with volume mount)

# 5. Restart if needed
docker restart myapp-dev

# 6. Cleanup
docker stop myapp-dev
docker rm myapp-dev
```

### Quick Cleanup

```bash
# Remove all stopped containers
docker container prune

# Remove unused images
docker image prune
docker image prune -a                    # Remove all unused images

# Remove all unused resources
docker system prune
docker system prune -a --volumes         # Include volumes
```

### Debugging Containers

```bash
# View resource usage
docker stats
docker stats <container>

# Inspect container details
docker inspect <container>

# Access container filesystem
docker exec -it <container> /bin/bash

# Copy files from container
docker cp <container>:/path/to/file ./local-path

# Copy files to container
docker cp ./local-file <container>:/path/to/dest
```

### Multi-Container Setup

```bash
# Create network
docker network create myapp-network

# Run database
docker run -d \
  --name postgres \
  --network myapp-network \
  -e POSTGRES_PASSWORD=secret \
  postgres:14

# Run application (connected to same network)
docker run -d \
  --name myapp \
  --network myapp-network \
  -p 8080:8080 \
  -e DATABASE_URL=postgresql://postgres:secret@postgres:5432/mydb \
  myapp:latest

# Verify network connectivity
docker exec myapp ping postgres
```

---

## Safety and Best Practices

### Image Size Optimization

```dockerfile
# Use specific base image versions
FROM ubuntu:22.04

# Combine RUN commands to reduce layers
RUN apt-get update && apt-get install -y \
    package1 \
    package2 \
    && rm -rf /var/lib/apt/lists/*

# Use multi-stage builds
FROM golang:1.21 AS builder
WORKDIR /app
COPY . .
RUN go build -o myapp

FROM ubuntu:22.04
COPY --from=builder /app/myapp /usr/local/bin/
CMD ["myapp"]
```

### Security Basics

```dockerfile
# Don't run as root
RUN useradd -m -u 1000 appuser
USER appuser

# Use specific versions (not latest)
FROM ubuntu:22.04

# Don't include secrets in images
# Use environment variables or secrets management
```

### Resource Limits

```bash
# Limit memory
docker run -m 512m myapp

# Limit CPU
docker run --cpus=1.5 myapp

# Set restart policy
docker run --restart=unless-stopped myapp
```

## Summary

Essential Docker commands:
- `docker run` - Run containers
- `docker ps` - List containers
- `docker logs` - View logs
- `docker build` - Build images
- `docker images` - List images
- `docker stop/rm` - Stop and remove containers
- `docker exec` - Execute commands in containers
- `docker prune` - Cleanup unused resources

These commands cover 90% of daily Docker usage for development and deployment.
