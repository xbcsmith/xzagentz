---
component:
    name: python_essential
    category: languages
    version: "1.0"
    description: Essential Python guidelines and critical rules (concise version)
    tier: essential
    languages:
        - python
---

# Python Development Guidelines (Essential)

## 1. Critical Rules

### File Extensions
- Use `.py` for all Python source files
- Use `.toml` for configuration files (pyproject.toml)
- Use `.txt` for requirements files (requirements.txt)

### Naming Conventions
- Modules/Packages: `lowercase_with_underscores` (e.g., `component_loader`, `config_parser`)
- Classes: `PascalCase` (e.g., `ComponentLoader`, `ConfigParser`)
- Functions/Variables: `lowercase_with_underscores` (e.g., `load_config`, `parse_template`)
- Constants: `UPPERCASE_WITH_UNDERSCORES` (e.g., `MAX_RETRIES`, `DEFAULT_TIMEOUT`)

### Quality Gates (MUST ALL PASS)
```bash
black .                          # Format code
isort .                          # Sort imports
flake8 .                         # Lint (zero errors)
mypy .                           # Type checking
pytest                           # Run tests (>80% coverage)
```

## 2. Type Hints (MANDATORY)

### Use Type Hints for All Functions
```python
from typing import Optional, List, Dict

def load_config(path: str) -> Dict[str, Any]:
    """Load configuration from file."""
    with open(path, 'r') as f:
        return yaml.safe_load(f)

def find_component(name: str, components: List[Component]) -> Optional[Component]:
    """Find component by name."""
    for comp in components:
        if comp.name == name:
            return comp
    return None
```

### Type Aliases for Complex Types
```python
from typing import Dict, List, Union

ComponentMap = Dict[str, Component]
ConfigValue = Union[str, int, bool, None]
```

## 3. Error Handling (MANDATORY)

### Use Specific Exceptions
```python
class ComponentError(Exception):
    """Base exception for component errors."""
    pass

class ComponentNotFoundError(ComponentError):
    """Raised when component cannot be found."""
    pass

class ComponentParseError(ComponentError):
    """Raised when component cannot be parsed."""
    pass
```

### Proper Exception Handling
```python
# BAD - bare except
try:
    data = load_file(path)
except:
    pass

# GOOD - specific exceptions with context
try:
    data = load_file(path)
except FileNotFoundError as e:
    raise ComponentNotFoundError(f"Component file not found: {path}") from e
except yaml.YAMLError as e:
    raise ComponentParseError(f"Invalid YAML in {path}") from e
```

### Context Managers for Resources
```python
# ALWAYS use context managers for files
with open(path, 'r') as f:
    content = f.read()

# For custom resources
from contextlib import contextmanager

@contextmanager
def component_lock(name: str):
    acquire_lock(name)
    try:
        yield
    finally:
        release_lock(name)
```

## 4. Testing Requirements

### Test Structure (pytest)
```python
import pytest
from myapp.component import load_component

def test_load_component_success():
    """Test loading valid component."""
    component = load_component("test.md")
    assert component.name == "test"
    assert component.content != ""

def test_load_component_not_found():
    """Test loading nonexistent component."""
    with pytest.raises(ComponentNotFoundError):
        load_component("nonexistent.md")

def test_load_component_invalid_format():
    """Test loading invalid component format."""
    with pytest.raises(ComponentParseError):
        load_component("invalid.md")
```

### Parametrized Tests
```python
@pytest.mark.parametrize("name,expected", [
    ("valid_name", True),
    ("", False),
    ("invalid@name", False),
])
def test_validate_name(name: str, expected: bool):
    """Test name validation."""
    result = validate_name(name)
    assert result == expected
```

### Test Requirements
- Write tests for ALL public functions
- Test success cases, error cases, and edge cases
- Use descriptive docstrings in test functions
- Achieve >80% code coverage

## 5. Documentation Standards

### Module Docstrings
```python
"""Component loading and management module.

This module provides functionality for loading markdown components
from the filesystem and managing their lifecycle.

Example:
    >>> from myapp.component import ComponentLoader
    >>> loader = ComponentLoader()
    >>> component = loader.load("components/core/header.md")
"""
```

### Function Docstrings (Google Style)
```python
def load_component(path: str, validate: bool = True) -> Component:
    """Load a component from the specified file path.
    
    Args:
        path: Path to the component file
        validate: Whether to validate the component structure
        
    Returns:
        Loaded Component instance
        
    Raises:
        ComponentNotFoundError: If file does not exist
        ComponentParseError: If file format is invalid
        
    Example:
        >>> component = load_component("components/core/header.md")
        >>> print(component.name)
        'header'
    """
    # Implementation
```

## 6. Common Patterns

### List Comprehensions
```python
# PREFERRED - concise and readable
names = [comp.name for comp in components if comp.is_valid()]

# LESS PYTHONIC
names = []
for comp in components:
    if comp.is_valid():
        names.append(comp.name)
```

### Dictionary Comprehensions
```python
# Create lookup dictionary
component_map = {comp.name: comp for comp in components}

# Filter and transform
valid_names = {name: name.upper() for name, comp in items.items() if comp.is_valid()}
```

### Use dataclasses for Data Structures
```python
from dataclasses import dataclass, field
from typing import List

@dataclass
class Component:
    """Represents a markdown component."""
    name: str
    content: str
    tags: List[str] = field(default_factory=list)
    
    def is_valid(self) -> bool:
        """Check if component is valid."""
        return bool(self.name and self.content)
```

## 7. Quick Command Reference

```bash
# Development
python -m venv venv            # Create virtual environment
source venv/bin/activate       # Activate (Unix)
venv\Scripts\activate          # Activate (Windows)

# Dependencies
pip install -r requirements.txt
pip install -e .               # Install in editable mode

# Quality
black .                        # Format code
isort .                        # Sort imports
flake8 .                       # Lint
mypy .                         # Type check

# Testing
pytest                         # Run all tests
pytest -v                      # Verbose output
pytest --cov=myapp             # Coverage report
pytest -k test_name            # Run specific test
```

## 8. Essential Best Practices

```python
# Use pathlib instead of os.path
from pathlib import Path

config_path = Path("config") / "settings.yaml"
if config_path.exists():
    content = config_path.read_text()

# Use enumerate for index and value
for index, component in enumerate(components):
    print(f"{index}: {component.name}")

# Use f-strings for formatting
name = "component"
message = f"Loading {name} from {path}"

# Use get() with default for dictionaries
value = config.get("timeout", 30)

# Chain comparison operators
if 0 < age < 120:
    print("Valid age")
```

## 9. Validation Workflow

```text
1. Write code with type hints and docstrings
2. Add tests with pytest
3. Run: black .
4. Run: isort .
5. Run: flake8 .
6. Run: mypy .
7. Run: pytest --cov
8. All checks MUST pass before committing
```
