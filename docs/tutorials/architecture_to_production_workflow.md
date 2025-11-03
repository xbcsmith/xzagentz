# Architecture to Production Workflow Tutorial

## Overview

This tutorial walks you through the complete workflow for taking a software project from initial concept to production-ready implementation plan using xzagentz. You will learn how to:

1. Generate a software architecture document using AI
2. Create a phased implementation plan from the architecture
3. Set up project guidelines with an AGENTS.md file
4. Integrate these artifacts into your development workflow

By the end of this tutorial, you will have a complete set of documentation to guide your development team from design through implementation.

## Prerequisites

Before starting this tutorial, ensure you have:

1. xzagentz installed and available in your PATH
2. Ollama running locally (default: http://localhost:11434)
3. An LLM model installed in Ollama (recommended: llama3, llama2, or codellama)
4. Basic familiarity with command-line tools
5. A project idea or requirements document

### Verify Prerequisites

Check that xzagentz is installed:

```bash
xzagentz --version
```

Verify Ollama is running:

```bash
curl http://localhost:11434/api/tags
```

List available models:

```bash
ollama list
```

If you need to install a model:

```bash
ollama pull llama3
```

## Step 1: Initialize xzagentz

First, initialize xzagentz to set up the necessary templates and components.

```bash
xzagentz init
```

This creates the configuration directories and extracts embedded templates:

```text
Initializing xzagentz...
Created: ~/.config/xzagentz/templates/
Created: ~/.config/xzagentz/components/
Extracted: architecture templates
Extracted: implementation templates
Extracted: project templates
Initialization complete!
```

Verify the initialization:

```bash
xzagentz list templates
```

## Step 2: Generate Software Architecture

Now you will generate a software architecture document for your project. This tutorial uses an example e-commerce platform, but you can substitute your own project requirements.

### Option A: Interactive Mode (Recommended for Beginners)

Run the architecture command in interactive mode:

```bash
xzagentz architecture generate --interactive
```

Follow the prompts:

1. Select generation mode: "Generate from requirements"
2. Enter your project requirements when prompted:

```text
Build a high-performance e-commerce platform with the following features:
- User authentication and authorization
- Product catalog with search and filtering
- Shopping cart and checkout
- Payment processing integration
- Order management and tracking
- Admin dashboard for inventory management
- Real-time notifications
- Analytics and reporting
```

3. Select architecture pattern: "Microservices"
4. Select complexity level: "Moderate"
5. Choose whether to include deployment architecture: "Yes"
6. Choose whether to include quality attributes: "Yes"
7. Enter technology preferences: "Rust, PostgreSQL, Redis, Redpanda"

The generator will create your architecture document and display a summary:

```text
Generating architecture...
[====================] 100%

Architecture Generated Successfully!

Title: E-Commerce Platform Architecture
Layers: 5
Components: 12
Integrations: 18

Would you like to review and refine? (yes/no): no
Enter output file path (default: architecture.md): docs/architecture.md

Saved to: docs/architecture.md
```

### Option B: Non-Interactive Mode (For Automation)

For scripting or automation, use the non-interactive mode:

```bash
xzagentz architecture generate \
  --requirements "Build a high-performance e-commerce platform with user auth, product catalog, shopping cart, payment processing, order management, admin dashboard, real-time notifications, and analytics" \
  --pattern microservices \
  --complexity moderate \
  --include-deployment \
  --include-quality \
  --tech-stack "Rust,PostgreSQL,Redis,Redpanda" \
  --output docs/architecture.md \
  --model llama3
```

### Option C: Template-Based Generation

Start with a proven pattern:

```bash
xzagentz architecture generate \
  --template microservices \
  --customization "Add support for multi-currency, multi-language, and GDPR compliance" \
  --output docs/architecture.md
```

### Review the Generated Architecture

Open and review the generated architecture document:

```bash
cat docs/architecture.md
```

The document will include:

- Architecture metadata (title, version, pattern)
- Overview with business goals and constraints
- Layer definitions and responsibilities
- Component descriptions with interfaces
- Integration patterns and protocols
- Deployment architecture
- Quality attributes (performance, security, scalability)

## Step 3: Refine the Architecture (Optional)

If you need to make changes to the architecture, use the refine command:

```bash
xzagentz architecture refine \
  --input docs/architecture.md \
  --refinement "Add a recommendation engine component using machine learning to suggest products based on user behavior and purchase history" \
  --output docs/architecture.md \
  --backup
```

This creates a backup (architecture.md.backup) and updates the document with your refinement.

### Validate the Architecture

Ensure the architecture meets quality standards:

```bash
xzagentz architecture validate --input docs/architecture.md --verbose
```

Expected output:

```text
Validating: docs/architecture.md

Validation Results:
✓ Metadata complete
✓ Overview well-defined
✓ Layers properly structured (5 layers)
✓ Components properly defined (12 components)
✓ Integrations valid (18 integrations)
✓ No orphaned components
✓ No circular dependencies

Overall: VALID

The architecture document meets all quality standards.
```

## Step 4: Generate Implementation Plan

Now that you have a validated architecture, generate a phased implementation plan.

### Option A: Interactive Mode

```bash
xzagentz implementation --interactive
```

Follow the prompts:

1. Select architecture file: Navigate to and select `docs/architecture.md`
2. Review architecture summary
3. Specify number of phases (or choose auto-detect)
4. Enter any additional constraints or priorities:

```text
Priority 1: Authentication and user management
Priority 2: Product catalog and search
Priority 3: Shopping cart and checkout
Must complete within 12 weeks
```

5. Review the generated plan
6. Choose to refine or accept
7. Specify output path: `docs/implementation_plan.md`

The generator creates a comprehensive implementation plan:

```text
Generating implementation plan...
[====================] 100%

Implementation Plan Generated Successfully!

Total Phases: 6
Estimated Duration: 12 weeks
Components Scheduled: 12
Dependencies Resolved: 18

Saved to: docs/implementation_plan.md
```

### Option B: Non-Interactive Mode

```bash
xzagentz implementation \
  --architecture docs/architecture.md \
  --output docs/implementation_plan.md \
  --num-phases 6 \
  --model llama3 \
  --force
```

### Review the Implementation Plan

Open and review the generated plan:

```bash
cat docs/implementation_plan.md
```

The plan will include:

- Executive summary
- Overall timeline and phases
- For each phase:
  - Phase objectives and goals
  - Components to implement
  - Dependencies and prerequisites
  - Estimated effort and duration
  - Success criteria
  - Testing requirements
  - Documentation requirements
- Risk assessment
- Resource requirements
- Deployment strategy

### Understanding the Plan Structure

Each phase follows a logical dependency order:

```text
Phase 1: Foundation (Weeks 1-2)
- Domain models and core entities
- Database schema design
- Authentication infrastructure
- Basic API framework

Phase 2: User Management (Weeks 3-4)
- User registration and login
- Session management
- Role-based access control
- User profile management

Phase 3: Product Catalog (Weeks 5-6)
- Product data models
- Catalog API endpoints
- Search functionality
- Filtering and pagination

Phase 4: Shopping Cart (Weeks 7-8)
- Cart state management
- Cart operations (add, remove, update)
- Cart persistence
- Price calculation

Phase 5: Checkout and Payments (Weeks 9-10)
- Order creation workflow
- Payment gateway integration
- Order confirmation
- Receipt generation

Phase 6: Admin and Analytics (Weeks 11-12)
- Admin dashboard
- Inventory management
- Order management
- Analytics and reporting
```

## Step 5: Create Project Guidelines

Create an AGENTS.md file to establish development standards for your project.

### Option A: Interactive Mode

```bash
xzagentz create --interactive
```

Follow the prompts to customize:

1. Project name: "E-Commerce Platform"
2. Programming language: "Rust"
3. Architecture pattern: "Microservices"
4. Testing requirements: "Greater than 80% coverage"
5. Include Docker guidelines: "Yes"
6. Include CI/CD guidelines: "Yes"
7. Custom coding standards: Add any project-specific rules

### Option B: Template-Based Creation

```bash
xzagentz create \
  --template rust-microservices \
  --output AGENTS.md \
  --force
```

### Option C: Default Creation

Create with default settings:

```bash
xzagentz create --output AGENTS.md
```

### Customize the AGENTS.md

Edit the generated AGENTS.md to add project-specific rules:

```bash
vim AGENTS.md
```

Add sections for:

- Project-specific file naming conventions
- API design standards
- Database migration procedures
- Deployment procedures
- Code review checklists

### Validate the AGENTS.md

Ensure the file follows best practices:

```bash
xzagentz validate --detailed
```

Expected output:

```text
Validating: AGENTS.md

Validation Results:
✓ File structure correct
✓ All required sections present
✓ No formatting issues
✓ Code examples properly formatted
✓ No broken links
✓ Consistent naming conventions

Overall: VALID

The AGENTS.md file meets all quality standards.
```

## Step 6: Integrate into Development Workflow

Now you have three key documents:

1. `docs/architecture.md` - Software architecture
2. `docs/implementation_plan.md` - Phased implementation plan
3. `AGENTS.md` - Development guidelines

### Set Up Project Structure

Create the project structure based on your architecture:

```bash
mkdir -p src/{api,application,domain,infrastructure}
mkdir -p tests/{unit,integration}
mkdir -p docs/{explanations,how_to,reference,tutorials}
```

### Initialize Version Control

```bash
git init
git add docs/architecture.md docs/implementation_plan.md AGENTS.md
git commit -m "docs: add architecture, implementation plan, and development guidelines"
```

### Create Phase Branches

Set up branches for each implementation phase:

```bash
git branch phase-1-foundation
git branch phase-2-user-management
git branch phase-3-product-catalog
git branch phase-4-shopping-cart
git branch phase-5-checkout-payments
git branch phase-6-admin-analytics
```

### Set Up Issue Tracking

Create issues for each phase in your issue tracker (GitHub, Jira, etc.):

```bash
# Example for GitHub CLI
gh issue create --title "Phase 1: Foundation" \
  --body "$(grep -A 20 'Phase 1:' docs/implementation_plan.md)"

gh issue create --title "Phase 2: User Management" \
  --body "$(grep -A 20 'Phase 2:' docs/implementation_plan.md)"

# Repeat for all phases
```

### Begin Implementation

Start implementing Phase 1:

```bash
git checkout phase-1-foundation
```

Follow the implementation plan and AGENTS.md guidelines as you develop:

1. Implement components listed in Phase 1
2. Write tests (maintaining greater than 80% coverage per AGENTS.md)
3. Document code with proper doc comments
4. Run quality gates before committing:

```bash
cargo fmt --all
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

5. Create pull request following AGENTS.md conventions:

```bash
git add .
git commit -m "feat(foundation): implement domain models and authentication infrastructure (PROJ-101)"
git push origin phase-1-foundation
```

## Step 7: Track Progress and Iterate

### Update Implementation Plan

As you complete phases, update the implementation plan:

```bash
# Mark Phase 1 as complete
sed -i 's/Phase 1: Foundation (Weeks 1-2)/Phase 1: Foundation (Weeks 1-2) - COMPLETE/' docs/implementation_plan.md
```

### Refine Architecture as Needed

If you discover needed changes during implementation:

```bash
xzagentz architecture refine \
  --input docs/architecture.md \
  --refinement "Add caching layer using Redis for product catalog to improve performance" \
  --output docs/architecture.md \
  --backup
```

### Regenerate Implementation Plan

If architecture changes significantly:

```bash
xzagentz implementation \
  --architecture docs/architecture.md \
  --output docs/implementation_plan_v2.md \
  --num-phases 6 \
  --force
```

Compare and merge relevant changes:

```bash
diff docs/implementation_plan.md docs/implementation_plan_v2.md
```

## Advanced Workflow Techniques

### Using Configuration Files

Create a configuration file for consistent settings:

```bash
mkdir -p ~/.config/xzagentz
cat > ~/.config/xzagentz/config.yaml << 'EOF'
architecture:
  ollama:
    base_url: "http://localhost:11434"
    default_model: "llama3"
    timeout_seconds: 180
  generation:
    default_pattern: "Microservices"
    default_complexity: "Moderate"
    include_deployment: true
    include_quality_attributes: true
    max_components: 20
  output:
    default_directory: "docs"
    create_backup: true
  interactive:
    enable_colors: true
    show_progress: true
    confirm_before_save: true

implementation:
  ollama:
    base_url: "http://localhost:11434"
    default_model: "llama3"
  planning:
    default_phases: 6
    auto_detect_phases: true
  output:
    default_directory: "docs"
    create_backup: true
EOF
```

Now commands use these defaults:

```bash
xzagentz architecture generate --requirements "Build a task management app"
# Uses llama3, microservices pattern, moderate complexity automatically
```

### Batch Processing Multiple Projects

Create architectures for multiple projects:

```bash
#!/bin/bash
projects=(
  "task-management:Build a collaborative task management application"
  "inventory-system:Build a warehouse inventory tracking system"
  "booking-platform:Build a hotel booking platform"
)

for project in "${projects[@]}"; do
  name="${project%%:*}"
  requirements="${project#*:}"

  xzagentz architecture generate \
    --requirements "$requirements" \
    --output "projects/${name}/architecture.md"

  xzagentz implementation \
    --architecture "projects/${name}/architecture.md" \
    --output "projects/${name}/implementation_plan.md"

  xzagentz create \
    --output "projects/${name}/AGENTS.md"
done
```

### Continuous Architecture Updates

Set up a Git hook to validate architecture on commits:

```bash
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
if [ -f docs/architecture.md ]; then
  echo "Validating architecture..."
  xzagentz architecture validate --input docs/architecture.md
  if [ $? -ne 0 ]; then
    echo "Architecture validation failed!"
    exit 1
  fi
fi

if [ -f AGENTS.md ]; then
  echo "Validating AGENTS.md..."
  xzagentz validate
  if [ $? -ne 0 ]; then
    echo "AGENTS.md validation failed!"
    exit 1
  fi
fi
EOF
chmod +x .git/hooks/pre-commit
```

## Troubleshooting

### Ollama Connection Issues

If architecture or implementation generation fails with connection errors:

```bash
# Check Ollama status
curl http://localhost:11434/api/tags

# Restart Ollama if needed
systemctl restart ollama  # Linux
# or
brew services restart ollama  # macOS

# Specify custom URL if Ollama runs elsewhere
xzagentz architecture generate \
  --ollama-url http://192.168.1.100:11434 \
  --requirements "Your requirements"
```

### Model Not Found

If you get model not found errors:

```bash
# List available models
ollama list

# Pull the recommended model
ollama pull llama3

# Or specify a different model you have
xzagentz architecture generate \
  --model llama2 \
  --requirements "Your requirements"
```

### Generation Timeout

For large architectures that timeout:

```bash
# Increase timeout in config
cat > ~/.config/xzagentz/config.yaml << 'EOF'
architecture:
  ollama:
    timeout_seconds: 300  # 5 minutes instead of default 180
EOF

# Or reduce complexity/components
xzagentz architecture generate \
  --complexity simple \
  --requirements "Your requirements"
```

### Validation Failures

If architecture validation fails:

```bash
# Get detailed error information
xzagentz architecture validate \
  --input docs/architecture.md \
  --verbose

# Common issues and fixes:
# - Empty component names: Edit and add names
# - Invalid references: Fix component IDs in integrations
# - Missing metadata: Ensure title and description are present
```

## Best Practices

### Architecture Generation

1. Start with clear, detailed requirements
2. Use interactive mode for learning; non-interactive for automation
3. Choose appropriate architecture pattern for your project scale
4. Always validate after generation
5. Keep architecture documents in version control
6. Review and refine architecture with team before implementation

### Implementation Planning

1. Generate plan after architecture is validated
2. Review estimated timelines and adjust for your team size
3. Use the plan as a living document, not a rigid contract
4. Update the plan as you learn during implementation
5. Link implementation plan phases to your issue tracker
6. Regularly review completed phases and update estimates

### Project Guidelines

1. Customize AGENTS.md for your specific project needs
2. Include examples of expected code quality
3. Document your specific testing requirements
4. Add project-specific conventions (API design, naming, etc.)
5. Keep guidelines up-to-date as project evolves
6. Review AGENTS.md during onboarding of new team members

### Workflow Integration

1. Create architecture before writing code
2. Generate implementation plan to guide sprints
3. Use AGENTS.md for code review standards
4. Update documents when making architectural changes
5. Tag architecture and plan versions with Git tags
6. Include documentation updates in your Definition of Done

## Next Steps

Now that you have completed this tutorial, you can:

1. Generate architecture for your real project
2. Create detailed implementation plans
3. Set up comprehensive development guidelines
4. Integrate these artifacts into your CI/CD pipeline
5. Share the workflow with your team

### Further Reading

- User Guide: `docs/how_to/generate_architecture_with_llm.md`
- Architecture Templates: `docs/reference/architecture_templates.md`
- Implementation Planning: `docs/explanations/implementation_planning.md`
- AGENTS.md Best Practices: `AGENTS.md`

### Getting Help

If you encounter issues:

1. Check the troubleshooting section above
2. Run commands with `--verbose` flag for detailed output
3. Validate your documents with detailed reporting
4. Check Ollama logs for LLM-related issues
5. Review the documentation in the `docs/` directory

## Conclusion

You have successfully learned the complete workflow for taking a project from concept to production-ready implementation plan using xzagentz. This workflow provides:

- Automated architecture generation with AI assistance
- Structured implementation planning with dependency resolution
- Standardized development guidelines for consistency
- Integration with modern development workflows

By following this workflow, you can significantly reduce the time spent on initial project planning and ensure your team has clear, actionable guidance from day one.

Happy building!
