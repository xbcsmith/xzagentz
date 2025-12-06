---
component:
    name: typescript_essential
    category: languages
    version: "1.0"
    description: Essential TypeScript guidelines and critical rules (concise version)
    tier: essential
    languages:
        - typescript
---

# TypeScript Development Guidelines (Essential)

## 1. Critical Rules

### File Extensions
- Use `.ts` for TypeScript source files
- Use `.tsx` for React/JSX components
- Use `.json` for configuration files (tsconfig.json, package.json)

### Naming Conventions
- Types/Interfaces/Classes: `PascalCase` (e.g., `ComponentLoader`, `ConfigError`)
- Functions/Variables: `camelCase` (e.g., `loadConfig`, `parseTemplate`)
- Constants: `UPPER_SNAKE_CASE` (e.g., `MAX_RETRIES`, `DEFAULT_TIMEOUT`)
- Files: `kebab-case` (e.g., `component-loader.ts`, `config-parser.ts`)
- Private members: prefix with `#` or use `private` keyword

### Quality Gates (MUST ALL PASS)
```bash
npm run build                    # Compile TypeScript
npm run lint                     # ESLint check (zero errors/warnings)
npm run format                   # Prettier formatting
npm test                         # Run tests (>80% coverage)
npm run type-check               # TypeScript strict check
```

## 2. Type System (MANDATORY)

### Always Use Strict Mode
```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "strictNullChecks": true,
    "noUncheckedIndexedAccess": true
  }
}
```

### Explicit Types for Public APIs
```typescript
// GOOD - explicit types
export function loadConfig(path: string): Config {
    const content = readFileSync(path, 'utf-8');
    return JSON.parse(content);
}

// BAD - implicit any
export function loadConfig(path) {
    return JSON.parse(readFileSync(path, 'utf-8'));
}
```

### Use Interfaces for Objects
```typescript
interface Component {
    name: string;
    content: string;
    tags?: string[];  // Optional property
    readonly created: Date;  // Immutable
}

// Use type for unions/intersections
type Result<T> = Success<T> | Failure;
type ConfigValue = string | number | boolean | null;
```

### Generic Types
```typescript
function loadComponent<T>(path: string, parser: (data: string) => T): T {
    const content = readFileSync(path, 'utf-8');
    return parser(content);
}

// Usage
const config = loadComponent<Config>('config.json', JSON.parse);
```

## 3. Error Handling (MANDATORY)

### Custom Error Classes
```typescript
export class ComponentError extends Error {
    constructor(message: string, public readonly path: string) {
        super(message);
        this.name = 'ComponentError';
        Error.captureStackTrace(this, this.constructor);
    }
}

export class ComponentNotFoundError extends ComponentError {
    constructor(path: string) {
        super(`Component not found: ${path}`, path);
        this.name = 'ComponentNotFoundError';
    }
}
```

### Proper Error Handling
```typescript
// GOOD - handle errors with types
function loadConfig(path: string): Config {
    try {
        const content = readFileSync(path, 'utf-8');
        return JSON.parse(content);
    } catch (error) {
        if (error instanceof SyntaxError) {
            throw new ConfigParseError(`Invalid JSON in ${path}`, error);
        }
        throw new ConfigReadError(`Failed to read ${path}`, error);
    }
}

// BAD - swallowing errors
function loadConfig(path: string): Config | null {
    try {
        return JSON.parse(readFileSync(path, 'utf-8'));
    } catch {
        return null;
    }
}
```

### Result Type Pattern
```typescript
type Result<T, E = Error> = 
    | { success: true; data: T }
    | { success: false; error: E };

function parseComponent(content: string): Result<Component> {
    try {
        const data = JSON.parse(content);
        return { success: true, data };
    } catch (error) {
        return { success: false, error: error as Error };
    }
}
```

## 4. Async/Await Patterns

### Always Use async/await
```typescript
// GOOD
async function loadComponent(path: string): Promise<Component> {
    const content = await fs.promises.readFile(path, 'utf-8');
    const component = await parseComponent(content);
    return component;
}

// AVOID - promise chains
function loadComponent(path: string): Promise<Component> {
    return fs.promises.readFile(path, 'utf-8')
        .then(parseComponent);
}
```

### Handle Promise Rejections
```typescript
async function loadComponents(paths: string[]): Promise<Component[]> {
    try {
        const promises = paths.map(path => loadComponent(path));
        return await Promise.all(promises);
    } catch (error) {
        throw new Error(`Failed to load components: ${error}`);
    }
}
```

## 5. Testing Requirements

### Test Structure (Jest/Vitest)
```typescript
import { describe, it, expect } from '@jest/globals';
import { loadComponent } from './component';

describe('loadComponent', () => {
    it('should load valid component', async () => {
        const component = await loadComponent('test.md');
        
        expect(component.name).toBe('test');
        expect(component.content).not.toBe('');
    });
    
    it('should throw error for missing file', async () => {
        await expect(loadComponent('nonexistent.md'))
            .rejects
            .toThrow(ComponentNotFoundError);
    });
    
    it('should handle invalid format', async () => {
        await expect(loadComponent('invalid.md'))
            .rejects
            .toThrow(ComponentParseError);
    });
});
```

### Test Requirements
- Write tests for ALL exported functions
- Test success cases, error cases, and edge cases
- Use descriptive test names
- Achieve >80% code coverage
- Mock external dependencies

## 6. Common Patterns

### Null Safety
```typescript
// Use optional chaining
const name = component?.metadata?.name ?? 'default';

// Use nullish coalescing
const timeout = config.timeout ?? 30;

// Type guards
function isComponent(obj: unknown): obj is Component {
    return (
        typeof obj === 'object' &&
        obj !== null &&
        'name' in obj &&
        'content' in obj
    );
}
```

### Readonly and Immutability
```typescript
interface Config {
    readonly name: string;
    readonly settings: readonly string[];
}

// Use as const for literal types
const STATUSES = ['pending', 'success', 'error'] as const;
type Status = typeof STATUSES[number];
```

### Discriminated Unions
```typescript
type ApiResponse = 
    | { status: 'success'; data: Component[] }
    | { status: 'error'; error: string }
    | { status: 'loading' };

function handleResponse(response: ApiResponse) {
    switch (response.status) {
        case 'success':
            return response.data;  // Type narrowed
        case 'error':
            throw new Error(response.error);
        case 'loading':
            return null;
    }
}
```

## 7. Quick Command Reference

```bash
# Development
npm init -y                      # Initialize project
npm install --save-dev typescript @types/node
npx tsc --init                   # Create tsconfig.json

# Build
npm run build                    # Compile TypeScript
tsc --watch                      # Watch mode

# Quality
npm run lint                     # ESLint
npm run format                   # Prettier
tsc --noEmit                     # Type check only

# Testing
npm test                         # Run tests
npm test -- --coverage           # With coverage
npm test -- --watch              # Watch mode
```

## 8. Essential tsconfig.json

```json
{
  "compilerOptions": {
    "target": "ES2022",
    "module": "commonjs",
    "lib": ["ES2022"],
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "resolveJsonModule": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}
```

## 9. Essential Best Practices

```typescript
// Use const for immutable variables
const config = loadConfig();

// Prefer arrow functions for callbacks
const names = components.map(c => c.name);

// Use template literals
const message = `Loading component: ${name}`;

// Destructure with types
const { name, content }: Component = loadComponent(path);

// Use async/await over promises
const data = await fetchData();

// Avoid type assertions (use type guards)
if (isComponent(obj)) {
    // obj is Component here
}
```

## 10. Validation Workflow

```text
1. Write code with explicit types
2. Add tests with type-safe assertions
3. Run: npm run format
4. Run: tsc --noEmit (type check)
5. Run: npm run lint
6. Run: npm test
7. All checks MUST pass before committing
```
