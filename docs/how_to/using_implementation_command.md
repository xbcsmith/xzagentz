# Using the Implementation Command

## Overview

The `implementation` command generates detailed implementation plans from architecture documents using Ollama LLM services. It supports both interactive and non-interactive modes, making it suitable for various workflows.

## Prerequisites

Before using the implementation command, ensure you have:

1. Ollama installed and running
2. At least one model downloaded (e.g., llama3)
3. An architecture document in Markdown format
4. Network access to the Ollama service

## Checking Ollama Status

Verify Ollama is running:

```bash
curl http://localhost:11434/api/tags
```

If Ollama is not running, start it:

```bash
ollama serve
```

## Interactive Mode

Interactive mode guides you through the plan generation process with prompts.

### Basic Usage

```bash
xzagentz implementation --interactive
```

### What Happens in Interactive Mode

1. You are prompted to select an architecture file
2. You choose the Ollama model to use
3. You specify the output location
4. The plan is generated and displayed
5. You confirm before saving

### Interactive Mode Options

```bash
# Disable colors
xzagentz implementation --interactive --no-color

# Disable progress indicators
xzagentz implementation --interactive --no-progress

# Skip confirmation prompts
xzagentz implementation --interactive --yes

# Use a specific model
xzagentz implementation --interactive --model llama3

# Use custom Ollama URL
xzagentz implementation --interactive --ollama-url http://remote:11434
```

## Non-Interactive Mode

Non-interactive mode is ideal for automation and CI/CD pipelines.

### Basic Usage

```bash
xzagentz implementation \
  --architecture docs/architecture.md \
  --output plans/implementation_plan.md
```

### Required Arguments

In non-interactive mode, you must specify:

- `--architecture`: Path to the architecture document
- `--output`: Path where the plan should be saved

### Non-Interactive Options

```bash
# Specify the model
xzagentz implementation \
  --architecture arch.md \
  --output plan.md \
  --model llama3

# Use custom Ollama URL
xzagentz implementation \
  --architecture arch.md \
  --output plan.md \
  --ollama-url http://localhost:11434

# Set number of phases
xzagentz implementation \
  --architecture arch.md \
  --output plan.md \
  --num-phases 7

# Force overwrite existing file
xzagentz implementation \
  --architecture arch.md \
  --output plan.md \
  --force

# Skip Ollama health check
xzagentz implementation \
  --architecture arch.md \
  --output plan.md \
  --skip-health-check
```

## Configuration File

You can create a configuration file to avoid specifying options repeatedly.

### Configuration Location

Create a file at `~/.config/xzagentz/config.yaml`:

```yaml
ollama:
  base_url: "http://localhost:11434"
  default_model: "llama3"
  timeout_seconds: 300
  max_retries: 3

planning:
  default_phases: 7
  default_output_dir: "docs/plans"
  output_format: "markdown"

interactive:
  enable_colors: true
  show_progress: true
  confirm_before_save: true
```

### Configuration Priority

Command-line arguments override configuration file values:

```bash
# Uses llama2 instead of config's default_model
xzagentz implementation --interactive --model llama2
```

## Common Workflows

### Workflow 1: Quick Plan Generation

For rapid iteration during planning:

```bash
xzagentz implementation \
  --architecture docs/architecture.md \
  --output plans/draft.md \
  --force
```

The `--force` flag allows overwriting the output file as you refine your architecture.

### Workflow 2: Production Plan

For final implementation plans:

```bash
xzagentz implementation \
  --architecture docs/final_architecture.md \
  --output plans/implementation_plan_v1.md \
  --model llama3 \
  --num-phases 10
```

### Workflow 3: CI/CD Integration

For automated plan generation in pipelines:

```bash
xzagentz implementation \
  --architecture docs/architecture.md \
  --output plans/auto_generated_plan.md \
  --yes \
  --skip-health-check \
  --force
```

### Workflow 4: Remote Ollama Service

When using Ollama on a different machine:

```bash
xzagentz implementation \
  --architecture docs/architecture.md \
  --output plans/plan.md \
  --ollama-url http://ollama-server:11434 \
  --model llama3
```

## Troubleshooting

### Issue: Cannot Connect to Ollama

**Symptoms:**

```
Failed to connect to Ollama at http://localhost:11434
```

**Solutions:**

1. Check if Ollama is running:
   ```bash
   ps aux | grep ollama
   ```

2. Start Ollama:
   ```bash
   ollama serve
   ```

3. Verify the URL is correct:
   ```bash
   curl http://localhost:11434/api/tags
   ```

4. Check firewall settings if using remote Ollama

### Issue: Model Not Found

**Symptoms:**

```
Model 'llama3' not found
```

**Solutions:**

1. List available models:
   ```bash
   ollama list
   ```

2. Pull the model:
   ```bash
   ollama pull llama3
   ```

3. Specify a different model:
   ```bash
   xzagentz implementation --interactive --model <available-model>
   ```

### Issue: Architecture File Parse Error

**Symptoms:**

```
Failed to parse architecture document
```

**Solutions:**

1. Verify the file exists:
   ```bash
   ls -l docs/architecture.md
   ```

2. Check file format (must be Markdown)

3. Ensure file is readable:
   ```bash
   cat docs/architecture.md
   ```

4. Validate Markdown syntax

### Issue: Output File Already Exists

**Symptoms:**

```
Output file already exists. Use --force to overwrite.
```

**Solutions:**

1. Use `--force` to overwrite:
   ```bash
   xzagentz implementation ... --force
   ```

2. Choose a different output path:
   ```bash
   xzagentz implementation ... --output plans/new_plan.md
   ```

3. Remove the existing file:
   ```bash
   rm plans/existing_plan.md
   ```

### Issue: Timeout During Generation

**Symptoms:**

```
Request timed out after 300 seconds
```

**Solutions:**

1. Increase timeout in config file:
   ```yaml
   ollama:
     timeout_seconds: 600
   ```

2. Use a smaller architecture document

3. Reduce the number of phases:
   ```bash
   xzagentz implementation ... --num-phases 5
   ```

4. Check Ollama server performance

## Advanced Usage

### Using Different Models

Compare plans from different models:

```bash
# Generate with llama3
xzagentz implementation \
  --architecture arch.md \
  --output plans/llama3_plan.md \
  --model llama3

# Generate with mistral
xzagentz implementation \
  --architecture arch.md \
  --output plans/mistral_plan.md \
  --model mistral
```

### Batch Processing

Generate plans for multiple architecture documents:

```bash
for arch in docs/architectures/*.md; do
  basename=$(basename "$arch" .md)
  xzagentz implementation \
    --architecture "$arch" \
    --output "plans/${basename}_plan.md" \
    --force
done
```

### Verbose Output

Get detailed information about the generation process:

```bash
xzagentz -v implementation \
  --architecture arch.md \
  --output plan.md
```

## Best Practices

1. **Version Control**: Keep both architecture documents and generated plans in version control

2. **Naming Convention**: Use descriptive names for output files:
   ```
   plans/phase1_authentication_plan.md
   plans/phase2_database_plan.md
   ```

3. **Review Process**: Always review generated plans before implementation

4. **Iterative Refinement**: Generate multiple plans as your architecture evolves

5. **Documentation**: Document why certain models or options were chosen

6. **Backup**: Keep backups of important plans before regenerating with `--force`

7. **Model Selection**: Use more capable models for complex architectures

8. **Configuration**: Use config file for team consistency

## Examples

### Example 1: Microservices Architecture

```bash
xzagentz implementation \
  --architecture docs/microservices_architecture.md \
  --output plans/microservices_implementation.md \
  --model llama3 \
  --num-phases 12
```

### Example 2: Database Migration

```bash
xzagentz implementation \
  --architecture docs/database_migration.md \
  --output plans/migration_plan.md \
  --model mistral \
  --num-phases 5
```

### Example 3: API Redesign

```bash
xzagentz implementation \
  --interactive \
  --model llama3
```

Then follow the prompts to select `docs/api_redesign.md` and output to `plans/api_implementation.md`.

## Next Steps

After generating an implementation plan:

1. Review the generated phases and tasks
2. Adjust timelines based on team capacity
3. Assign tasks to team members
4. Track progress in your project management tool
5. Update the plan as needed during implementation

## Related Documentation

- Architecture Document Guidelines: `docs/how_to/writing_architecture_docs.md`
- Plan Structure Reference: `docs/reference/plan_format.md`
- Ollama Configuration: `docs/how_to/configuring_ollama.md`
- CI/CD Integration: `docs/how_to/cicd_integration.md`
