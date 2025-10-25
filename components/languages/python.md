# Python Language Guidelines

This section provides Python-specific guidelines for projects that include Python components.

---

## Python Version

**Recommended**: Python 3.9 or later

Check your version:
```bash
python --version
python3 --version
```

---

## Code Style and Formatting

### Use Black for Formatting

**Auto-format code:**
```bash
black .
```

**Configuration** (pyproject.toml):
```toml
[tool.black]
line-length = 100
target-version = ['py39']
```

### PEP 8 Compliance

Follow PEP 8 style guide:
- Indentation: 4 spaces
- Max line length: 100 characters (configurable)
- Imports: grouped and sorted
- Naming conventions: follow PEP 8

### Import Organization

**Use isort:**
```bash
isort .
```

**Import order:**
1. Standard library imports
2. Third-party imports
3. Local application imports

**Example:**
```python
import os
import sys
from pathlib import Path

import click
import yaml

from myproject.core import Component
from myproject.utils import helpers
```

---

## Type Hints

### Use Type Annotations

**Function signatures:**
```python
from typing import List, Optional, Dict, Any

def load_config(path: str) -> Dict[str, Any]:
    """Load configuration from file."""
    with open(path, 'r') as f:
        return yaml.safe_load(f)

def process_items(items: List[str], limit: Optional[int] = None) -> List[str]:
    """Process items with optional limit."""
    if limit:
        items = items[:limit]
    return [item.upper() for item in items]
```

### Type Checking with mypy

```bash
mypy .
```

**Configuration** (mypy.ini):
```ini
[mypy]
python_version = 3.9
warn_return_any = True
warn_unused_configs = True
disallow_untyped_defs = True
```

---

## Error Handling

### Use Specific Exceptions

```python
class ComponentError(Exception):
    """Base exception for component errors."""
    pass

class ComponentNotFoundError(ComponentError):
    """Raised when component cannot be found."""
    pass

class ComponentParseError(ComponentError):
    """Raised when component parsing fails."""
    pass

def load_component(path: str) -> str:
    """Load component from file.

    Args:
        path: Path to component file

    Returns:
        Component content

    Raises:
        ComponentNotFoundError: If file doesn't exist
        ComponentParseError: If content is invalid
    """
    if not os.path.exists(path):
        raise ComponentNotFoundError(f"Component not found: {path}")

    try:
        with open(path, 'r') as f:
            content = f.read()
    except Exception as e:
        raise ComponentParseError(f"Failed to read {path}: {e}")

    if not content.strip():
        raise ComponentParseError(f"Empty component: {path}")

    return content
```

---

## Testing

### Use pytest

**Test structure:**
```python
import pytest
from myproject.component import Component

def test_component_creation():
    """Test component can be created."""
    component = Component("test")
    assert component.name == "test"

def test_component_validation():
    """Test component validation."""
    with pytest.raises(ValueError):
        Component("")  # Empty name should raise

@pytest.fixture
def sample_component():
    """Fixture providing sample component."""
    return Component("sample")

def test_with_fixture(sample_component):
    """Test using fixture."""
    assert sample_component.is_valid()
```

### Test Coverage

**Run tests with coverage:**
```bash
pytest --cov=myproject --cov-report=html
```

**Minimum coverage**: 80%

---

## Documentation

### Docstrings

**Use Google-style docstrings:**
```python
def process_template(template: str, context: Dict[str, Any]) -> str:
    """Process template with given context.

    Args:
        template: Template string to process
        context: Dictionary of template variables

    Returns:
        Processed template string

    Raises:
        TemplateError: If template processing fails

    Examples:
        >>> process_template("Hello {{name}}", {"name": "World"})
        'Hello World'
    """
    # Implementation
    pass
```

### Module Documentation

```python
"""Component loading module.

This module provides functionality for loading and validating
component files from the filesystem.

Example:
    >>> from myproject.component import ComponentLoader
    >>> loader = ComponentLoader()
    >>> component = loader.load("components/header.md")
"""
```

---

## Virtual Environments

### Use venv or virtualenv

**Create environment:**
```bash
python -m venv venv
```

**Activate:**
```bash
# Unix/MacOS
source venv/bin/activate

# Windows
venv\Scripts\activate
```

### Requirements Management

**requirements.txt:**
```text
click>=8.0.0
pyyaml>=6.0
jinja2>=3.0.0
```

**Development requirements:**
```text
-r requirements.txt
pytest>=7.0.0
pytest-cov>=3.0.0
black>=22.0.0
isort>=5.10.0
mypy>=0.950
```

---

## Best Practices

### Context Managers

**Use with statements:**
```python
# Good
with open('file.txt', 'r') as f:
    content = f.read()

# Avoid
f = open('file.txt', 'r')
content = f.read()
f.close()
```

### List Comprehensions

**Prefer comprehensions for simple transformations:**
```python
# Good
names = [item.name for item in items if item.is_active]

# Less pythonic
names = []
for item in items:
    if item.is_active:
        names.append(item.name)
```

### Pathlib Over os.path

**Use pathlib for path operations:**
```python
from pathlib import Path

# Good
config_path = Path("config") / "settings.yaml"
if config_path.exists():
    content = config_path.read_text()

# Older style
import os
config_path = os.path.join("config", "settings.yaml")
if os.path.exists(config_path):
    with open(config_path, 'r') as f:
        content = f.read()
```

---

## Quality Tools

### Linting

**flake8:**
```bash
flake8 .
```

**pylint:**
```bash
pylint myproject
```

### Code Formatting

**black:**
```bash
black .
```

**isort:**
```bash
isort .
```

### Type Checking

**mypy:**
```bash
mypy .
```

---

## Common Patterns

### Dataclasses

```python
from dataclasses import dataclass
from typing import Optional

@dataclass
class Component:
    """Component data structure."""
    name: str
    content: str
    category: Optional[str] = None

    def is_valid(self) -> bool:
        """Check if component is valid."""
        return bool(self.name and self.content)
```

### Property Decorators

```python
class Component:
    def __init__(self, name: str):
        self._name = name

    @property
    def name(self) -> str:
        """Get component name."""
        return self._name

    @name.setter
    def name(self, value: str) -> None:
        """Set component name."""
        if not value:
            raise ValueError("Name cannot be empty")
        self._name = value
```

---

## Project Structure

```
myproject/
├── myproject/
│   ├── __init__.py
│   ├── core/
│   │   ├── __init__.py
│   │   └── component.py
│   ├── utils/
│   │   ├── __init__.py
│   │   └── helpers.py
│   └── cli.py
├── tests/
│   ├── __init__.py
│   ├── test_component.py
│   └── conftest.py
├── requirements.txt
├── requirements-dev.txt
├── setup.py
├── pyproject.toml
└── README.md
```

---

## Additional Resources

- **PEP 8**: https://www.python.org/dev/peps/pep-0008/
- **Python Docs**: https://docs.python.org/3/
- **Real Python**: https://realpython.com/
- **Python Type Hints**: https://docs.python.org/3/library/typing.html
