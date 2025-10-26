---
component:
  name: kubernetes_comprehensive
  category: tools
  version: 1.0.0
  tier: comprehensive
  description: Comprehensive Kubernetes reference with advanced operations and troubleshooting
  languages: [rust, python, golang, typescript, bash]
  sections:
    - id: advanced_manifests
      language_specific: true
      required: true
    - id: advanced_operations
      language_specific: false
      required: true
    - id: observability
      language_specific: false
      required: true
    - id: troubleshooting
      language_specific: false
      required: true
---

# Kubernetes Comprehensive Guidelines

Complete Kubernetes reference including advanced operations, scaling, and production best practices.

---

## Advanced Kubernetes Manifests

<!-- LANG:rust -->

### Rust Application with Full Configuration

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-app
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: rust-app
  template:
    metadata:
      labels:
        app: rust-app
      annotations:
        prometheus.io/scrape: "true"
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
      containers:
        - name: rust-app
          image: myregistry/rust-app:1.0.0
          ports:
            - name: http
              containerPort: 8080
          env:
            - name: DATABASE_URL
              valueFrom:
                secretKeyRef:
                  name: rust-app-secrets
                  key: database-url
          volumeMounts:
            - name: config
              mountPath: /config
              readOnly: true
          resources:
            requests:
              memory: "256Mi"
              cpu: "200m"
            limits:
              memory: "512Mi"
              cpu: "1000m"
          livenessProbe:
            httpGet:
              path: /health
              port: http
            initialDelaySeconds: 30
          readinessProbe:
            httpGet:
              path: /ready
              port: http
            initialDelaySeconds: 10
      volumes:
        - name: config
          configMap:
            name: rust-app-config
```

<!-- /LANG -->

<!-- LANG:python -->

### Python Application with Auto-scaling

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
              memory: "512Mi"
              cpu: "250m"
            limits:
              memory: "1Gi"
              cpu: "1000m"
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: python-app-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: python-app
  minReplicas: 3
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
```

<!-- /LANG -->

<!-- LANG:golang -->

### Go Application with StatefulSet

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: go-app
spec:
  serviceName: go-app
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
          volumeMounts:
            - name: data
              mountPath: /data
          resources:
            requests:
              memory: "256Mi"
              cpu: "200m"
  volumeClaimTemplates:
    - metadata:
        name: data
      spec:
        accessModes: ["ReadWriteOnce"]
        resources:
          requests:
            storage: 10Gi
```

<!-- /LANG -->

<!-- LANG:typescript -->

### TypeScript/Node Application with Ingress

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
          resources:
            requests:
              memory: "512Mi"
              cpu: "250m"
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: node-app-ingress
spec:
  rules:
    - host: app.example.com
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: node-app-service
                port:
                  number: 80
```

<!-- /LANG -->

<!-- LANG:bash -->

### Bash CronJob

```yaml
apiVersion: batch/v1
kind: CronJob
metadata:
  name: maintenance-job
spec:
  schedule: "0 2 * * *"
  jobTemplate:
    spec:
      template:
        spec:
          containers:
            - name: maintenance
              image: bash:5
              command: ["bash", "-c", "echo 'Maintenance'"]
          restartPolicy: OnFailure
```

<!-- /LANG -->

---

## Advanced Deployment Strategies

### Blue-Green Deployment

```yaml
# Blue deployment (current)
apiVersion: apps/v1
kind: Deployment
metadata:
  name: myapp-blue
spec:
  replicas: 3
  selector:
    matchLabels:
      app: myapp
      version: blue
  template:
    metadata:
      labels:
        app: myapp
        version: blue
    spec:
      containers:
        - name: myapp
          image: myapp:v1.0.0
---
# Service switches between versions
apiVersion: v1
kind: Service
metadata:
  name: myapp-service
spec:
  selector:
    app: myapp
    version: blue
  ports:
    - port: 80
      targetPort: 8080
```

---

## Advanced Resource Management

### Resource Quotas

```yaml
apiVersion: v1
kind: ResourceQuota
metadata:
  name: compute-quota
spec:
  hard:
    requests.cpu: "10"
    requests.memory: 20Gi
    limits.cpu: "20"
    limits.memory: 40Gi
```

### Pod Disruption Budget

```yaml
apiVersion: policy/v1
kind: PodDisruptionBudget
metadata:
  name: myapp-pdb
spec:
  minAvailable: 2
  selector:
    matchLabels:
      app: myapp
```

---

## Network Policies

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: api-allow
spec:
  podSelector:
    matchLabels:
      app: api
  policyTypes:
    - Ingress
  ingress:
    - from:
        - podSelector:
            matchLabels:
              app: frontend
      ports:
        - protocol: TCP
          port: 8080
```

---

## Storage Management

### Persistent Volume Claim

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: pvc-data
spec:
  accessModes:
    - ReadWriteOnce
  storageClassName: fast-ssd
  resources:
    requests:
      storage: 50Gi
```

---

## RBAC Configuration

```yaml
apiVersion: v1
kind: ServiceAccount
metadata:
  name: myapp
---
apiVersion: rbac.authorization.k8s.io/v1
kind: Role
metadata:
  name: pod-reader
rules:
  - apiGroups: [""]
    resources: ["pods"]
    verbs: ["get", "list", "watch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: RoleBinding
metadata:
  name: read-pods
subjects:
  - kind: ServiceAccount
    name: myapp
roleRef:
  kind: Role
  name: pod-reader
  apiGroup: rbac.authorization.k8s.io
```

---

## Advanced Commands

### Context Management

```bash
# List contexts
kubectl config get-contexts

# Switch context
kubectl config use-context prod-cluster

# Set namespace
kubectl config set-context --current --namespace=myapp
```

### Advanced Queries

```bash
# Get pods sorted by age
kubectl get pods --sort-by=.metadata.creationTimestamp

# Get with custom columns
kubectl get pods -o custom-columns=NAME:.metadata.name,STATUS:.status.phase

# Get resource usage
kubectl top nodes
kubectl top pods

# Get pods on specific node
kubectl get pods --field-selector spec.nodeName=node-1
```

### Batch Operations

```bash
# Delete all pods with label
kubectl delete pods -l app=myapp

# Restart deployment
kubectl rollout restart deployment/myapp

# Scale multiple deployments
kubectl scale deployment/app1 deployment/app2 --replicas=3
```

### Resource Editing

```bash
# Edit resource
kubectl edit deployment myapp

# Patch resource
kubectl patch deployment myapp -p '{"spec":{"replicas":5}}'

# Update image
kubectl set image deployment/myapp myapp=myapp:v2
```

---

## Troubleshooting

### Pod Issues

**CrashLoopBackOff:**

```bash
# Check logs
kubectl logs pod-name --previous

# Describe pod
kubectl describe pod pod-name

# Debug with ephemeral container
kubectl debug pod-name -it --image=busybox
```

**ImagePullBackOff:**

```bash
# Check image name
kubectl describe pod pod-name | grep Image

# Check secrets
kubectl get secrets
```

**Pending State:**

```bash
# Check node resources
kubectl describe nodes

# Check pod resources
kubectl describe pod pod-name
```

### Network Troubleshooting

```bash
# Test DNS resolution
kubectl run -it --rm debug --image=busybox --restart=Never -- nslookup kubernetes.default

# Test service connectivity
kubectl run -it --rm debug --image=curlimages/curl --restart=Never -- curl http://myapp-service

# Check network policies
kubectl get networkpolicies
```

### Performance Issues

```bash
# Check resource usage
kubectl top pods
kubectl top nodes

# Check HPA status
kubectl get hpa
kubectl describe hpa myapp-hpa
```

---

## Cluster Maintenance

### Node Operations

```bash
# Cordon node (prevent scheduling)
kubectl cordon node-1

# Drain node (evict pods)
kubectl drain node-1 --ignore-daemonsets

# Uncordon node
kubectl uncordon node-1

# Taint node
kubectl taint nodes node-1 key=value:NoSchedule

# Remove taint
kubectl taint nodes node-1 key:NoSchedule-
```

---

## Security Best Practices

### Pod Security

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: secure-pod
spec:
  securityContext:
    runAsNonRoot: true
    runAsUser: 1000
  containers:
    - name: app
      image: myapp:latest
      securityContext:
        allowPrivilegeEscalation: false
        readOnlyRootFilesystem: true
        capabilities:
          drop:
            - ALL
```

### Secret Management

```bash
# Create TLS secret
kubectl create secret tls tls-secret --cert=path/to/cert --key=path/to/key

# Create registry secret
kubectl create secret docker-registry regcred \
  --docker-server=myregistry.com \
  --docker-username=user \
  --docker-password=pass
```

---

## Monitoring and Logging

### Logging Configuration

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: fluent-bit-config
data:
  fluent-bit.conf: |
    [INPUT]
        Name              tail
        Path              /var/log/containers/*.log
        Parser            docker

    [OUTPUT]
        Name   es
        Match  *
        Host   elasticsearch
        Port   9200
```

---

## Best Practices Summary

### Resource Management

- Always set resource requests and limits
- Use HPA for automatic scaling
- Implement PodDisruptionBudgets
- Use ResourceQuotas and LimitRanges

### High Availability

- Run multiple replicas
- Use pod anti-affinity rules
- Implement proper health checks
- Plan for node failures

### Security

- Use RBAC for access control
- Run containers as non-root
- Use network policies
- Scan images for vulnerabilities

### Operations

- Use declarative configuration
- Version control all manifests
- Implement proper logging
- Monitor cluster health
