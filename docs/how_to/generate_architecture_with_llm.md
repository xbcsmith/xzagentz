# How to Generate Architecture with LLM

This guide explains how to use xzagentz to generate software architecture documents using Large Language Models (LLMs) via Ollama.

## Prerequisites

Before you begin, ensure you have:

1. **xzagentz installed**: Follow the installation instructions in the main README
2. **Ollama running**: Install and start Ollama service
   - Download from https://ollama.ai
   - Start the service: `ollama serve`
   - Pull a model: `ollama pull llama3`
3. **Configuration file** (optional): Located at `~/.config/xzagentz/config.yaml`

## Quick Start

### Generate from Requirements

The simplest way to generate an architecture document is from a requirements description:

```bash
xzagentz architecture generate \
  --requirements "E-commerce API with payments and inventory management" \
  --output docs/architecture/ecommerce.md
```

This will:
1. Connect to your local Ollama instance
2. Use the LLM to generate a complete architecture document
3. Save the result to the specified output file

### Generate from Template

Use a predefined template for common architecture patterns:

```bash
xzagentz architecture generate \
  --template microservices \
  --customization "Focus on payment processing and fraud detection" \
  --output docs/architecture/payments.md
```

Available templates:
- `microservices` - Distributed services architecture
- `monolithic` - Traditional layered monolith
- `event-driven` - Event-based reactive architecture
- `layered` - Clean architecture with clear layers

### Interactive Mode

For guided architecture generation with prompts:

```bash
xzagentz architecture generate --interactive
```

The interactive mode will:
1. Ask if you want to generate from requirements or template
2. Prompt for architecture pattern and complexity level
3. Ask about technology preferences
4. Allow you to review and refine the generated document
5. Save the final result

## Configuration

### Configuration File

Create or edit `~/.config/xzagentz/config.yaml`:

```yaml
architecture:
  ollama:
    base_url: "http://localhost:11434"
    default_model: "llama3"
    timeout_seconds: 600
    max_retries: 3

  generation:
    default_pattern: "layered"
    default_complexity: "moderate"
    include_deployment: true
    include_quality_attributes: true
    max_components: 20

  output:
    default_directory: "docs/architecture"
    format: "markdown"
    create_backup: true

  templates:
    directory: "templates/architecture"
    auto_create_defaults: true

  interactive:
    enable_colors: true
    show_progress: true
    confirm_before_save: true
    show_token_usage: false
```

### Configuration Options Explained

**architecture.ollama**:
- `base_url`: URL of your Ollama service
- `default_model`: Model to use for generation (e.g., llama3, llama3.2, codellama)
- `timeout_seconds`: How long to wait for generation (600 = 10 minutes)
- `max_retries`: Number of retry attempts on failure

**architecture.generation**:
- `default_pattern`: Default architecture pattern to use
- `default_complexity`: Default complexity level (simple, moderate, complex, enterprise)
- `include_deployment`: Include deployment architecture section by default
- `include_quality_attributes`: Include quality attributes section by default
- `max_components`: Maximum number of components to generate

**architecture.output**:
- `default_directory`: Where to save architecture documents
- `format`: Output format (currently only markdown supported)
- `create_backup`: Automatically backup before overwriting files

**architecture.templates**:
- `directory`: Location of architecture templates
- `auto_create_defaults`: Create default templates if missing

**architecture.interactive**:
- `enable_colors`: Use colored output in terminal
- `show_progress`: Show progress indicators
- `confirm_before_save`: Ask for confirmation before saving
- `show_token_usage`: Display token usage statistics

## Advanced Usage

### Specify Architecture Pattern

Choose a specific architecture pattern:

```bash
xzagentz architecture generate \
  --requirements "IoT data collection platform" \
  --pattern event-driven \
  --complexity complex \
  --output iot-architecture.md
```

Supported patterns:
- `microservices` - Independent deployable services
- `monolithic` - Single deployment unit
- `event-driven` - Event-based communication
- `layered` - Traditional layered architecture
- `hexagonal` - Ports and adapters pattern
- `cqrs` - Command Query Responsibility Segregation
- `serverless` - Function-as-a-Service architecture

### Technology Preferences

Specify preferred technologies:

```bash
xzagentz architecture generate \
  --requirements "Real-time chat application" \
  --tech-stack "rust,postgresql,redis,websockets" \
  --output chat-arch.md
```

The LLM will incorporate these technologies into the architecture design.

### Control Output Details

Fine-tune what's included:

```bash
xzagentz architecture generate \
  --requirements "Simple blog CMS" \
  --complexity simple \
  --no-include-deployment \
  --no-include-quality \
  --max-components 10 \
  --output blog-arch.md
```

Options:
- `--complexity`: `simple`, `moderate`, `complex`, `enterprise`
- `--include-deployment` / `--no-include-deployment`: Deployment section
- `--include-quality` / `--no-include-quality`: Quality attributes section
- `--max-components`: Limit number of components

### Custom Ollama Configuration

Override default Ollama settings:

```bash
xzagentz architecture generate \
  --requirements "Multi-tenant SaaS platform" \
  --model llama3.2 \
  --ollama-url http://remote-server:11434 \
  --output saas-arch.md
```

### Force Overwrite

Overwrite existing files without prompting:

```bash
xzagentz architecture generate \
  --requirements "Updated requirements" \
  --output existing-arch.md \
  --force
```

## Refining Architecture

Refine an existing architecture document with additional requirements:

```bash
xzagentz architecture refine \
  --input docs/architecture/current.md \
  --refinement "Add support for multi-region deployment and disaster recovery" \
  --output docs/architecture/updated.md
```

Options:
- `--input`: Path to existing architecture document
- `--refinement`: Description of desired changes
- `--output`: Where to save refined version (optional, defaults to input file)
- `--backup`: Create backup before modifying (enabled by default)
- `--model`: Override default model
- `--ollama-url`: Override Ollama URL

## Managing Templates

### List Available Templates

See all available architecture templates:

```bash
xzagentz architecture list-templates
```

For detailed information:

```bash
xzagentz architecture list-templates --verbose
```

With pattern filter:

```bash
xzagentz architecture list-templates --pattern microservices
```

### Custom Templates

Create custom templates in `templates/architecture/`:

1. Create a YAML file (e.g., `templates/architecture/custom.yaml`)
2. Define template structure:

```yaml
info:
  name: "custom"
  pattern: "Custom"
  description: "Your custom architecture pattern"
  use_cases:
    - "Specific use case 1"
    - "Specific use case 2"

structure:
  layers:
    - "Layer 1"
    - "Layer 2"

  integration_patterns:
    - "Integration pattern 1"

  required_components:
    - "Required component 1"

default_components:
  - name: "Component Name"
    layer: "Layer 1"
    role: "Component responsibility"
    typical_technologies:
      - "Technology 1"
      - "Technology 2"

customization_points:
  - "Customization aspect 1"
  - "Customization aspect 2"
```

3. Use your custom template:

```bash
xzagentz architecture generate --template custom --output output.md
```

## Validating Architecture

Validate an architecture document structure:

```bash
xzagentz architecture validate --input docs/architecture/design.md
```

With verbose output:

```bash
xzagentz architecture validate --input design.md --verbose
```

The validator checks:
- Required metadata is present
- Document structure is complete
- Component references are valid
- Integration endpoints exist
- Layer dependencies are correct

## Troubleshooting

### Ollama Not Available

**Problem**: Error message "Cannot connect to Ollama"

**Solutions**:
1. Check if Ollama is running: `ps aux | grep ollama`
2. Start Ollama: `ollama serve`
3. Verify the URL in config matches your Ollama instance
4. Try: `curl http://localhost:11434/api/tags`

If running on a different port or host:

```bash
xzagentz architecture generate \
  --requirements "Test" \
  --ollama-url http://localhost:8080 \
  --output test.md
```

### Model Not Found

**Problem**: Error "model not found" or "model not loaded"

**Solutions**:
1. List available models: `ollama list`
2. Pull the required model: `ollama pull llama3`
3. Use a different model:

```bash
xzagentz architecture generate \
  --requirements "Test" \
  --model llama2 \
  --output test.md
```

### Generation Timeout

**Problem**: Generation times out before completion

**Solutions**:
1. Increase timeout in config:

```yaml
architecture:
  ollama:
    timeout_seconds: 1200  # 20 minutes
```

2. Reduce complexity:

```bash
xzagentz architecture generate \
  --requirements "Simple app" \
  --complexity simple \
  --max-components 10 \
  --output simple.md
```

3. Use a faster or smaller model
4. Simplify requirements description

### Poor Quality Output

**Problem**: Generated architecture is generic or low quality

**Solutions**:
1. Provide more detailed requirements
2. Use technology preferences to guide generation
3. Specify architecture pattern explicitly
4. Use a more capable model (e.g., llama3 instead of llama2)
5. Use interactive mode to refine iteratively

### File Permission Errors

**Problem**: Cannot write to output file or config directory

**Solutions**:
1. Check file permissions: `ls -l output.md`
2. Ensure directory exists: `mkdir -p docs/architecture`
3. Use a different output location with write permissions

## Best Practices

### 1. Start with Requirements

Provide clear, specific requirements:

**Good**:
```bash
--requirements "Multi-tenant SaaS CRM with role-based access, audit logging,
and integration with Stripe for payments and SendGrid for email"
```

**Poor**:
```bash
--requirements "CRM system"
```

### 2. Choose Appropriate Complexity

- `simple`: 5-10 components, basic features, single team
- `moderate`: 10-20 components, standard features, growing system
- `complex`: 20-40 components, advanced features, multiple teams
- `enterprise`: 40+ components, comprehensive features, large organization

### 3. Use Templates as Starting Points

Templates provide proven patterns. Customize with specific requirements:

```bash
xzagentz architecture generate \
  --template microservices \
  --customization "Add event sourcing for order processing and CQRS for reporting" \
  --output custom-arch.md
```

### 4. Iterate with Refinement

Generate initial architecture, then refine:

```bash
# Initial generation
xzagentz architecture generate --requirements "API platform" --output v1.md

# Refine with additional requirements
xzagentz architecture refine \
  --input v1.md \
  --refinement "Add rate limiting and API versioning" \
  --output v2.md
```

### 5. Validate Regularly

Always validate after generation or manual edits:

```bash
xzagentz architecture validate --input architecture.md --verbose
```

### 6. Use Configuration Files

For team consistency, commit a config file to your repository:

```bash
# Create project-specific config
cat > .xzagentz.yaml <<EOF
architecture:
  generation:
    default_pattern: "microservices"
    default_complexity: "complex"
  output:
    default_directory: "docs/architecture"
EOF
```

### 7. Version Control Architecture Documents

Track architecture evolution in git:

```bash
git add docs/architecture/
git commit -m "docs: add initial microservices architecture"
```

## Examples

### Example 1: Microservices E-commerce

Generate a complete microservices architecture for e-commerce:

```bash
xzagentz architecture generate \
  --requirements "E-commerce platform with product catalog, shopping cart,
  order processing, payment integration (Stripe), inventory management,
  user authentication, and email notifications" \
  --pattern microservices \
  --complexity complex \
  --tech-stack "java,spring-boot,postgresql,redis,kafka,kubernetes" \
  --include-deployment \
  --include-quality \
  --output docs/architecture/ecommerce-microservices.md
```

### Example 2: Event-Driven IoT Platform

Generate an event-driven architecture for IoT:

```bash
xzagentz architecture generate \
  --template event-driven \
  --customization "IoT sensors streaming temperature data, real-time
  anomaly detection, time-series database, mobile app notifications" \
  --tech-stack "rust,timescaledb,mqtt,kafka,grafana" \
  --complexity complex \
  --output docs/architecture/iot-platform.md
```

### Example 3: Simple Monolithic CMS

Generate a simple monolithic architecture:

```bash
xzagentz architecture generate \
  --requirements "Content Management System with blog posts, pages,
  media library, user roles, and REST API" \
  --pattern monolithic \
  --complexity simple \
  --tech-stack "python,django,postgresql" \
  --max-components 8 \
  --output docs/architecture/cms-monolith.md
```

### Example 4: Interactive Generation

Use interactive mode for guided generation:

```bash
xzagentz architecture generate --interactive
```

Follow the prompts:
1. Choose "Generate from requirements"
2. Enter: "Real-time collaboration platform with document editing"
3. Select pattern: "Microservices"
4. Select complexity: "Complex"
5. Technology preferences: "typescript,nodejs,postgresql,redis,websockets"
6. Review generated architecture
7. Choose "Refine" if needed
8. Save to desired location

## Next Steps

After generating your architecture:

1. **Review and customize**: The generated architecture is a starting point
2. **Share with team**: Get feedback from architects and developers
3. **Refine iteratively**: Use the `refine` command to incorporate feedback
4. **Keep updated**: Regenerate sections as requirements evolve
5. **Validate regularly**: Run validation after manual edits
6. **Document decisions**: Add ADRs (Architecture Decision Records) to explain choices

## Related Documentation

- Architecture Generation Plan: `docs/explanation/llm_architecture_command_plan.md`
- Phase 4 Implementation: `docs/explanation/phase4_cli_integration_implementation.md`
- Template Reference: `templates/architecture/`
- Configuration Reference: `config.example.yaml`

## Support

For issues or questions:
- Check troubleshooting section above
- Review example configurations in `config.example.yaml`
- Consult the architecture generation plan for technical details
- Open an issue on GitHub with reproduction steps
