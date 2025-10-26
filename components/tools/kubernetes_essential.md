---
component:
  name: kubernetes_essential
  category: tools
  version: 1.0.0
  tier: essential
  description: Essential Kubernetes commands and workflows for container orchestration
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: basic_manifests
      language_specific: true
      required: true
    - id: essential_commands
      language_specific: false
      required: true
    - id: pod_management
      language_specific: false
      required: true
---

# Kubernetes Essential Guidelines

Core Kubernetes commands and workflows for daily container orchestration.

---

## Basic Kubernetes Manifests

<!-- LANG:rust -->

### Rust Application Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-app
  template:
    metadata:
      labels:
        app: rust-app
    spec:
      containers:
        - name: rust-app
          image: myregistry/rust-app:1.0.0
          ports:
            - containerPort: 8080
          resources:
            requests:
              memory: "128Mi"
              cpu: "100m"
            limits:
              memory: "256Mi"
              cpu: "500m"
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Application Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: python-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: python-app
  template:
    metadata:
      labels:
        app: python-app
    spec:
      containers:
        - name: python-app
          image: myregistry/python-app:1.0.0
          ports:
            - containerPort: 8000
          resources:
            requests:
              memory: "256Mi"
              cpu: "100m"
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Application Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: go-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: go-app
  template:
    metadata:
      labels:
        app: go-app
    spec:
      containers:
        - name: go-app
          image: myregistry/go-app:1.0.0
          ports:
            - containerPort: 8080
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript/Node Application Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: node-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: node-app
  template:
    metadata:
      labels:
        app: node-app
    spec:
      containers:
        - name: node-app
          image: myregistry/node-app:1.0.0
          ports:
            - containerPort: 3000
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash Script Job

```yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: bash-job
spec:
  template:
    spec:
      containers:
        - name: bash-script
          image: bash:5
          command: ["bash", "-c", "./script.sh"]
      restartPolicy: OnFailure
```

<!-- /LANG -->

---

## Essential Commands

### Cluster Information

```bash
kubectl cluster-info
kubectl get nodes
kubectl version
```

### Pod Management

```bash
# List pods
kubectl get pods

# Get pod details
kubectl describe pod pod-name

# View logs
kubectl logs pod-name
kubectl logs -f pod-name

# Execute command
kubectl exec -it pod-name -- bash
```

### Deployment Operations

```bash
# Create deployment
kubectl create deployment myapp --image=myapp:latest

# Scale deployment
kubectl scale deployment myapp --replicas=5

# Update image
kubectl set image deployment/myapp myapp=myapp:v2

# View rollout status
kubectl rollout status deployment/myapp

# Rollback
kubectl rollout undo deployment/myapp
```

---

## Services

### Service Types

```yaml
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
spec:
  type: ClusterIP
  selector:
    app: myapp
  ports:
    - port: 80
      targetPort: 8080
```

### Service Commands

```bash
kubectl get services
kubectl describe service myapp-service
```

---

## ConfigMaps and Secrets

```bash
# ConfigMap
kubectl create configmap myapp-config --from-literal=key=value
kubectl get configmaps

# Secret
kubectl create secret generic myapp-secret --from-literal=password=secret
kubectl get secrets
```

---

## Resource Management

```bash
# Apply manifests
kubectl apply -f deployment.yaml
kubectl apply -f ./manifests/

# Delete resources
kubectl delete -f deployment.yaml

# Get resources
kubectl get all
kubectl get pods,services
```

---

## Debugging

```bash
# Check pod status
kubectl get pod pod-name
kubectl describe pod pod-name
kubectl logs pod-name

# Port forwarding
kubectl port-forward pod-name 8080:80
kubectl port-forward service/myapp 8080:80
```

---

## Best Practices

- Always set resource requests and limits
- Define readiness and liveness probes
- Use ConfigMaps for configuration
- Use Secrets for sensitive data
- Use namespaces for isolation
- Test rollbacks before production
