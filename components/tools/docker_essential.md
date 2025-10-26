---
component:
  name: docker_essential
  category: tools
  version: 1.0.0
  tier: essential
  description: Essential Docker commands and workflows for containerization
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: dockerfile_basics
      language_specific: true
      required: true
    - id: essential_commands
      language_specific: false
      required: true
    - id: container_management
      language_specific: false
      required: true
---

# Docker Essential Guidelines

Core Docker commands and workflows for daily container operations.

---

## Dockerfile Basics

<!-- LANG:rust -->

### Rust Dockerfile

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app /usr/local/bin/app

CMD ["app"]
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Dockerfile

```dockerfile
FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "app.py"]
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Dockerfile

```dockerfile
FROM golang:1.21 as builder

WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN CGO_ENABLED=0 GOOS=linux go build -o app

FROM alpine:latest
RUN apk --no-cache add ca-certificates

COPY --from=builder /app/app /app

CMD ["/app"]
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript/Node Dockerfile

```dockerfile
FROM node:20-slim

WORKDIR /app

COPY package*.json ./
RUN npm ci --production

COPY . .

CMD ["node", "dist/index.js"]
```

<!-- /LANG -->

<!-- LANG:bash -->

### Shell Script Container

```dockerfile
FROM bash:5

WORKDIR /scripts

COPY *.sh ./
RUN chmod +x *.sh

CMD ["./entrypoint.sh"]
```

<!-- /LANG -->

---

## Essential Commands

### Build and Run

```bash
# Build image
docker build -t myapp:latest .

# Build with specific Dockerfile
docker build -f Dockerfile.prod -t myapp:prod .

# Run container
docker run -d --name myapp myapp:latest

# Run with port mapping
docker run -d -p 8080:8080 --name myapp myapp:latest

# Run with environment variables
docker run -d -e DATABASE_URL=postgres://... --name myapp myapp:latest

# Run with volume mount
docker run -d -v $(pwd)/data:/app/data --name myapp myapp:latest
```

### Container Management

```bash
# List running containers
docker ps

# List all containers (including stopped)
docker ps -a

# Stop container
docker stop myapp

# Start container
docker start myapp

# Restart container
docker restart myapp

# Remove container
docker rm myapp

# Force remove running container
docker rm -f myapp
```

### Logs and Inspection

```bash
# View logs
docker logs myapp

# Follow logs
docker logs -f myapp

# View last 100 lines
docker logs --tail 100 myapp

# Inspect container
docker inspect myapp

# View container stats
docker stats myapp
```

### Execute Commands in Container

```bash
# Run command in container
docker exec myapp ls -la

# Interactive shell
docker exec -it myapp bash

# Run as specific user
docker exec -u root -it myapp bash
```

---

## Image Management

### Basic Operations

```bash
# List images
docker images

# Remove image
docker rmi myapp:latest

# Remove unused images
docker image prune

# Tag image
docker tag myapp:latest myapp:v1.0.0

# Pull image
docker pull nginx:latest

# Push image
docker push registry.example.com/myapp:latest
```

---

## Docker Compose Basics

### Basic compose file

```yaml
version: '3.8'

services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgres://db/myapp
    depends_on:
      - db

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=myapp
      - POSTGRES_PASSWORD=secret
    volumes:
      - db-data:/var/lib/postgresql/data

volumes:
  db-data:
```

### Compose Commands

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f

# Rebuild and restart
docker-compose up -d --build

# Run command in service
docker-compose exec app bash
```

---

## Cleanup

```bash
# Stop all containers
docker stop $(docker ps -q)

# Remove all stopped containers
docker container prune

# Remove all unused images
docker image prune -a

# Remove all unused volumes
docker volume prune

# Clean everything
docker system prune -a --volumes
```

---

## Best Practices

### Dockerfile

- Use specific base image tags (not `latest`)
- Minimize layers by combining RUN commands
- Use multi-stage builds for smaller images
- Copy dependency files before source code
- Use `.dockerignore` to exclude unnecessary files
- Run as non-root user when possible

### Security

- Don't store secrets in images
- Use environment variables for configuration
- Scan images for vulnerabilities
- Keep base images updated
- Use minimal base images (alpine, distroless)

### Performance

- Cache dependency layers
- Order commands from least to most frequently changed
- Use build cache effectively
- Minimize image size
