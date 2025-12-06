---
component:
  name: typescript_comprehensive
  category: languages
  version: 2.0.0
  description: Comprehensive TypeScript language guidelines, patterns, and best practices
  tier: comprehensive
  languages:
    - typescript
  sections:
    - id: version_setup
      language_specific: false
      required: true
    - id: code_style
      language_specific: false
      required: true
    - id: type_system
      language_specific: false
      required: true
    - id: error_handling
      language_specific: false
      required: true
    - id: async_patterns
      language_specific: false
      required: true
    - id: best_practices
      language_specific: false
      required: true
    - id: project_structure
      language_specific: false
      required: true
---

# TypeScript Language Guidelines

This component provides TypeScript-specific guidelines, patterns, and best practices for robust application development.

---

## Version and Setup

### Required Version

**Minimum**: TypeScript 4.5 or later (5.x recommended)

Check your version:

```bash
tsc --version
node --version
```

Install or update TypeScript:

```bash
npm install -g typescript
npm install --save-dev typescript
```

### Project Initialization

Create a new TypeScript project:

```bash
npm init -y
npm install --save-dev typescript @types/node
npx tsc --init
```

### tsconfig.json Configuration

Recommended strict configuration:

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
    "sourceMap": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "moduleResolution": "node"
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist", "**/*.test.ts"]
}
```

---

## Code Style and Formatting

### Formatting Rules

Use Prettier for consistent formatting:

```bash
npm install --save-dev prettier
```

Create `.prettierrc`:

```json
{
  "semi": true,
  "trailingComma": "es5",
  "singleQuote": true,
  "printWidth": 100,
  "tabWidth": 2
}
```

### Linting with ESLint

Install ESLint with TypeScript support:

```bash
npm install --save-dev eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin
```

Create `.eslintrc.json`:

```json
{
  "parser": "@typescript-eslint/parser",
  "extends": ["eslint:recommended", "plugin:@typescript-eslint/recommended"],
  "rules": {
    "@typescript-eslint/no-explicit-any": "error",
    "@typescript-eslint/explicit-function-return-type": "warn",
    "@typescript-eslint/no-unused-vars": "error"
  }
}
```

### Naming Conventions

**Interfaces and Types**

- Use `PascalCase` for interfaces and type aliases
- Prefix interfaces with `I` is optional (not recommended in modern TypeScript)
- Examples: `User`, `ConfigOptions`, `ApiResponse`

**Classes**

- Use `PascalCase` for class names
- Examples: `UserService`, `DatabaseConnection`, `EventEmitter`

**Functions and Variables**

- Use `camelCase` for functions, methods, and variables
- Examples: `fetchUser`, `parseResponse`, `userId`

**Constants**

- Use `UPPER_SNAKE_CASE` for global constants
- Examples: `MAX_RETRIES`, `DEFAULT_TIMEOUT`, `API_VERSION`

**Enums**

- Use `PascalCase` for enum names and members
- Examples: `enum Status { Active, Inactive, Pending }`

---

## Type System Best Practices

### Strong Typing

Always prefer explicit types over `any`:

```typescript
// BAD: Using any
function processData(data: any): any {
  return data.value;
}

// GOOD: Explicit types
interface DataInput {
  value: string;
  timestamp: number;
}

function processData(data: DataInput): string {
  return data.value;
}
```

### Type Aliases vs Interfaces

Use interfaces for object shapes that may be extended:

```typescript
interface User {
  id: string;
  name: string;
  email: string;
}

interface AdminUser extends User {
  permissions: string[];
}
```

Use type aliases for unions, intersections, and primitives:

```typescript
type Status = "active" | "inactive" | "pending";
type ID = string | number;
type Nullable<T> = T | null;
```

### Generics

Use generics for reusable, type-safe functions:

```typescript
function getById<T extends { id: string }>(
  items: T[],
  id: string
): T | undefined {
  return items.find((item) => item.id === id);
}

class Cache<K, V> {
  private store = new Map<K, V>();

  set(key: K, value: V): void {
    this.store.set(key, value);
  }

  get(key: K): V | undefined {
    return this.store.get(key);
  }
}
```

### Utility Types

Leverage built-in utility types:

```typescript
interface User {
  id: string;
  name: string;
  email: string;
  password: string;
}

// Partial - all properties optional
type UserUpdate = Partial<User>;

// Pick - select specific properties
type UserPublic = Pick<User, "id" | "name" | "email">;

// Omit - exclude specific properties
type UserWithoutPassword = Omit<User, "password">;

// Required - all properties required
type UserRequired = Required<Partial<User>>;

// Readonly - all properties readonly
type ImmutableUser = Readonly<User>;

// Record - create object type with specific keys
type UserRoles = Record<string, "admin" | "user" | "guest">;
```

### Type Guards

Implement type guards for runtime type checking:

```typescript
interface Dog {
  bark(): void;
}

interface Cat {
  meow(): void;
}

function isDog(pet: Dog | Cat): pet is Dog {
  return (pet as Dog).bark !== undefined;
}

function handlePet(pet: Dog | Cat): void {
  if (isDog(pet)) {
    pet.bark(); // TypeScript knows pet is Dog
  } else {
    pet.meow(); // TypeScript knows pet is Cat
  }
}
```

---

## Error Handling

### Custom Error Classes

Define typed error classes:

```typescript
class ValidationError extends Error {
  constructor(message: string, public field: string, public value: unknown) {
    super(message);
    this.name = "ValidationError";
    Error.captureStackTrace(this, this.constructor);
  }
}

class DatabaseError extends Error {
  constructor(message: string, public query: string, public cause?: Error) {
    super(message);
    this.name = "DatabaseError";
    this.cause = cause;
  }
}

class NotFoundError extends Error {
  constructor(public resourceType: string, public resourceId: string) {
    super(`${resourceType} with id ${resourceId} not found`);
    this.name = "NotFoundError";
  }
}
```

### Result Type Pattern

Implement Result type for functional error handling:

```typescript
type Result<T, E = Error> =
  | { success: true; value: T }
  | { success: false; error: E };

function divide(a: number, b: number): Result<number, string> {
  if (b === 0) {
    return { success: false, error: "Division by zero" };
  }
  return { success: true, value: a / b };
}

function handleResult(result: Result<number, string>): void {
  if (result.success) {
    console.log(`Result: ${result.value}`);
  } else {
    console.error(`Error: ${result.error}`);
  }
}
```

### Try-Catch with Typed Errors

Handle errors with proper typing:

```typescript
async function fetchUser(id: string): Promise<User> {
  try {
    const response = await fetch(`/api/users/${id}`);

    if (!response.ok) {
      throw new NotFoundError("User", id);
    }

    const data = await response.json();
    return data as User;
  } catch (error) {
    if (error instanceof NotFoundError) {
      console.error("User not found:", error.resourceId);
      throw error;
    }

    if (error instanceof Error) {
      throw new DatabaseError(
        "Failed to fetch user",
        `SELECT * FROM users WHERE id = ${id}`,
        error
      );
    }

    throw new Error("Unknown error occurred");
  }
}
```

---

## Async Patterns

### Promises with Proper Types

Always type Promise return values:

```typescript
async function loadConfig(): Promise<ConfigOptions> {
  const response = await fetch("/api/config");
  const data: unknown = await response.json();

  if (!isValidConfig(data)) {
    throw new ValidationError("Invalid config format", "config", data);
  }

  return data;
}

function isValidConfig(data: unknown): data is ConfigOptions {
  return (
    typeof data === "object" &&
    data !== null &&
    "apiKey" in data &&
    "timeout" in data
  );
}
```

### Async Iterators

Use async iterators for streaming data:

```typescript
async function* fetchPages<T>(
  endpoint: string,
  pageSize: number
): AsyncGenerator<T[], void, undefined> {
  let page = 1;
  let hasMore = true;

  while (hasMore) {
    const response = await fetch(`${endpoint}?page=${page}&size=${pageSize}`);
    const data: T[] = await response.json();

    if (data.length === 0) {
      hasMore = false;
    } else {
      yield data;
      page++;
    }
  }
}

// Usage
async function processAllUsers(): Promise<void> {
  for await (const users of fetchPages<User>("/api/users", 100)) {
    users.forEach((user) => console.log(user.name));
  }
}
```

### Concurrent Operations

Handle multiple async operations efficiently:

```typescript
// Parallel execution
async function loadMultipleUsers(ids: string[]): Promise<User[]> {
  const promises = ids.map((id) => fetchUser(id));
  return Promise.all(promises);
}

// Sequential with error handling
async function processInSequence<T, R>(
  items: T[],
  processor: (item: T) => Promise<R>
): Promise<Result<R[], Error[]>> {
  const results: R[] = [];
  const errors: Error[] = [];

  for (const item of items) {
    try {
      const result = await processor(item);
      results.push(result);
    } catch (error) {
      errors.push(error instanceof Error ? error : new Error(String(error)));
    }
  }

  if (errors.length > 0) {
    return { success: false, error: errors };
  }

  return { success: true, value: results };
}
```

---

## Best Practices

### Immutability

Prefer immutable data patterns:

```typescript
// BAD: Mutating objects
function updateUser(user: User, name: string): void {
  user.name = name;
}

// GOOD: Return new objects
function updateUser(user: User, name: string): User {
  return { ...user, name };
}

// Use readonly for immutable interfaces
interface Config {
  readonly apiKey: string;
  readonly endpoints: readonly string[];
}
```

### Null Safety

Use strict null checks and optional chaining:

```typescript
// Enable in tsconfig.json
{
  "strictNullChecks": true
}

// Use optional chaining
const userName = user?.profile?.name ?? 'Anonymous';

// Use nullish coalescing
const timeout = config.timeout ?? 5000;

// Type narrowing with null checks
function processUser(user: User | null): void {
  if (user === null) {
    return;
  }
  // TypeScript knows user is not null here
  console.log(user.name);
}
```

### Function Overloads

Use function overloads for better type inference:

```typescript
function createElement(tag: "div"): HTMLDivElement;
function createElement(tag: "span"): HTMLSpanElement;
function createElement(tag: string): HTMLElement;
function createElement(tag: string): HTMLElement {
  return document.createElement(tag);
}

// TypeScript infers correct return type
const div = createElement("div"); // HTMLDivElement
const span = createElement("span"); // HTMLSpanElement
```

### Discriminated Unions

Use discriminated unions for type-safe state machines:

```typescript
type RequestState<T> =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "success"; data: T }
  | { status: "error"; error: Error };

function handleRequest<T>(state: RequestState<T>): void {
  switch (state.status) {
    case "idle":
      console.log("Not started yet");
      break;
    case "loading":
      console.log("Loading...");
      break;
    case "success":
      console.log("Data:", state.data); // data is available
      break;
    case "error":
      console.error("Error:", state.error); // error is available
      break;
  }
}
```

### Const Assertions

Use const assertions for literal types:

```typescript
// Without const assertion
const colors1 = ["red", "green", "blue"]; // string[]

// With const assertion
const colors2 = ["red", "green", "blue"] as const; // readonly ["red", "green", "blue"]

type Color = (typeof colors2)[number]; // "red" | "green" | "blue"

// Object const assertion
const config = {
  endpoint: "/api",
  timeout: 5000,
} as const;
// { readonly endpoint: "/api"; readonly timeout: 5000 }
```

---

## Project Structure

### Recommended Directory Layout

```
src/
├── types/           # Type definitions
│   ├── index.ts
│   └── models.ts
├── services/        # Business logic
│   ├── user.service.ts
│   └── config.service.ts
├── utils/           # Utility functions
│   ├── validation.ts
│   └── helpers.ts
├── errors/          # Custom error classes
│   └── index.ts
├── config/          # Configuration
│   └── index.ts
└── index.ts         # Entry point
```

### Module Exports

Use barrel exports for clean imports:

```typescript
// types/index.ts
export * from "./models";
export * from "./api";

// Usage
import { User, Config, ApiResponse } from "./types";
```

### Dependency Management

Keep dependencies organized:

```typescript
// services/user.service.ts
import { User, UserId } from "../types";
import { ValidationError } from "../errors";
import { validateEmail } from "../utils/validation";

export class UserService {
  async create(data: Partial<User>): Promise<User> {
    if (!data.email || !validateEmail(data.email)) {
      throw new ValidationError("Invalid email", "email", data.email);
    }

    // Implementation
    return {} as User;
  }
}
```

### Declaration Files

Create declaration files for JavaScript libraries:

```typescript
// types/custom.d.ts
declare module "legacy-library" {
  export function doSomething(value: string): number;
  export interface Options {
    timeout: number;
  }
}
```

---

## Testing

### Testing

Use Jest with TypeScript:

```bash
npm install --save-dev jest @types/jest ts-jest
```

Write type-safe tests:

```typescript
describe("UserService", () => {
  test("creates user with valid data", async () => {
    const user = await service.create({ email: "john@example.com" });
    expect(user.email).toBe("john@example.com");
  });

  test("throws ValidationError for invalid email", async () => {
    await expect(service.create({ email: "invalid" })).rejects.toThrow(
      ValidationError
    );
  });
});
```

---

## Summary

TypeScript best practices:

- Enable strict mode in tsconfig.json
- Avoid using `any` - use `unknown` and type guards instead
- Leverage utility types for code reuse
- Implement custom error classes for better error handling
- Use async/await with proper type annotations
- Prefer immutability and functional patterns
- Use discriminated unions for type-safe state
- Write tests with full type safety
- Follow consistent naming conventions
- Keep dependencies well-organized

These practices ensure type-safe, maintainable TypeScript applications.
