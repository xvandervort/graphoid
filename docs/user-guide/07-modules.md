# Chapter 7: Modules

Modules help you organize code into reusable components. In this chapter, we'll learn how to import, export, and structure Graphoid programs.

## What is a Module?

A module is a `.gr` file that contains reusable code. Modules can export functions, variables, and types for use in other programs.

## Basic Import

### Importing from Standard Library

```graphoid
# Import the math module
import "math"

# Use functions from math
result = math.sqrt(16)
print(result)  # 4

pi = math.pi
print(pi)  # 3.14159...
```

### Built-in Module Aliases

Standard library modules have built-in short aliases:

```graphoid
import "random"

# Both work automatically:
random.choice([1, 2, 3])
rand.choice([1, 2, 3])    # 'rand' alias works without explicit declaration

import "statistics"
statistics.mean([1, 2, 3])
stats.mean([1, 2, 3])     # 'stats' alias works automatically
```

**Note**: You don't need to use `as` for built-in aliases - they're available automatically!

### Importing Your Own Modules

```graphoid
# File: myproject/utils.gr
fn greet(name) {
    return "Hello, " + name + "!"
}

fn add(a, b) {
    return a + b
}
```

```graphoid
# File: myproject/main.gr
import "utils"

message = utils.greet("Alice")
print(message)  # "Hello, Alice!"

sum = utils.add(5, 3)
print(sum)  # 8
```

## Export

By default, all top-level functions and variables in a module are exported.

### Explicit Export

```graphoid
# File: calculator.gr

# Public functions (exported)
export fn add(a, b) {
    return a + b
}

export fn subtract(a, b) {
    return a - b
}

# Private function (not exported)
fn internal_helper(x) {
    return x * 2
}
```

### Default Export

Modules can have a default export:

```graphoid
# File: greeter.gr

fn greet(name) {
    return "Hello, " + name + "!"
}

export default greet
```

```graphoid
# File: main.gr
import greet from "greeter"

print(greet("Alice"))  # "Hello, Alice!"
```

## Import Variants

### Import Specific Items

```graphoid
# Import only specific functions
import { sqrt, pow } from "math"

result = sqrt(16)  # No need for math. prefix
print(result)  # 4
```

### Import with Renaming

```graphoid
# Rename on import
import { random_int as rand_int } from "random"

num = rand_int(1, 10)
```

### Import Everything

```graphoid
# Import all exports (use sparingly)
import * from "math"

# Now all math functions are in global scope
result = sqrt(16)
pi_value = pi
```

**Warning**: This can pollute your namespace. Use carefully!

## Module Organization

### Project Structure

```
myproject/
  main.gr           # Entry point
  utils/
    strings.gr      # String utilities
    numbers.gr      # Number utilities
    graphs.gr       # Graph utilities
  models/
    user.gr         # User model
    post.gr         # Post model
  lib/
    external.gr     # Third-party code
```

### Relative Imports

```graphoid
# File: myproject/utils/strings.gr
fn capitalize(s) {
    return s[0].to_upper() + s.substring(1)
}

# File: myproject/main.gr
import "utils/strings"

result = strings.capitalize("hello")
print(result)  # "Hello"
```

### Parent Directory Imports

```graphoid
# File: myproject/models/user.gr
import "../utils/strings"

fn format_name(name) {
    return strings.capitalize(name)
}
```

## Module Namespaces

Modules create their own namespace:

```graphoid
# File: module_a.gr
x = 10

fn get_x() {
    return x
}

# File: module_b.gr
x = 20

fn get_x() {
    return x
}

# File: main.gr
import "module_a"
import "module_b"

print(module_a.get_x())  # 10
print(module_b.get_x())  # 20
```

Each module has its own `x` variable - no conflicts!

## Module Initialization

Code at the module level runs once when the module is first imported:

```graphoid
# File: config.gr
print("Loading config...")

settings = {
    "debug": true,
    "port": 8080
}

print("Config loaded!")
```

```graphoid
# File: main.gr
import "config"     # Prints: "Loading config..." then "Config loaded!"
import "config"     # (Already loaded, no output)

print(config.settings["port"])  # 8080
```

## Circular Dependencies

Avoid circular dependencies where Module A imports Module B and Module B imports Module A.

### ❌ Bad: Circular Dependency

```graphoid
# File: user.gr
import "post"

fn get_user_posts(user) {
    return post.get_posts_by_user(user)
}

# File: post.gr
import "user"  # Circular!

fn get_posts_by_user(user) {
    return user.get_user_posts(user)  # Infinite loop!
}
```

### ✅ Good: Restructure

```graphoid
# File: user.gr
fn get_user_posts(user) {
    # Implementation without importing post
}

# File: post.gr
import "user"

fn get_posts_by_user(user) {
    return user.get_user_posts(user)
}
```

Or extract shared code to a third module:

```graphoid
# File: shared.gr
fn process_data(data) {
    # Shared logic
}

# File: user.gr
import "shared"

# File: post.gr
import "shared"
```

## Standard Library Modules

Graphoid includes a rich standard library:

### Core Modules

```graphoid
import "math"         # Mathematical functions
import "string"       # String operations
import "io"          # Input/output
import "time"        # Time and date
import "random"      # Random numbers
```

### Data Modules

```graphoid
import "json"        # JSON parsing/serialization
import "csv"         # CSV handling
import "regex"       # Regular expressions
```

### System Modules

```graphoid
import "os"          # Operating system interface
import "fs"          # File system operations
import "net"         # Networking
```

### Advanced Modules

```graphoid
import "crypto"      # Cryptography
import "collections" # Advanced data structures
import "statistics"  # Statistical functions
```

See Chapter 9 for complete standard library reference.

## Module Best Practices

### Keep Modules Focused

```graphoid
# ✅ GOOD: Focused module
# File: validation.gr
fn validate_email(email) { ... }
fn validate_phone(phone) { ... }
fn validate_zip(zip) { ... }

# ❌ BAD: Too much in one module
# File: utils.gr
fn validate_email(email) { ... }
fn parse_json(text) { ... }
fn connect_database(host) { ... }
fn send_email(to, subject) { ... }
```

### Use Clear Names

```graphoid
# ✅ GOOD: Clear module names
import "user_validation"
import "email_sender"
import "database_connection"

# ❌ BAD: Vague names
import "utils"
import "helpers"
import "stuff"
```

### Export Only Public API

```graphoid
# File: calculator.gr

# Exported (public)
export fn add(a, b) {
    return internal_add(a, b)
}

# Not exported (private)
fn internal_add(a, b) {
    # Internal implementation details
    return a + b
}
```

### Document Your Modules

```graphoid
# File: string_utils.gr
# String utility functions for text processing
#
# Functions:
#   capitalize(s)  - Capitalize first letter
#   reverse(s)     - Reverse a string
#   truncate(s, n) - Truncate to n characters

export fn capitalize(s) {
    return s[0].to_upper() + s.substring(1)
}

export fn reverse(s) {
    # ...
}

export fn truncate(s, n) {
    # ...
}
```

### Organize by Feature

```graphoid
# Group related functionality
myapp/
  user/
    model.gr       # User data model
    validation.gr  # User validation
    service.gr     # User business logic
  post/
    model.gr
    validation.gr
    service.gr
```

## Module Loading

### Search Path

Graphoid searches for modules in this order:

1. Current directory
2. `./lib` directory
3. Standard library paths
4. Paths in `GRAPHOID_PATH` environment variable

### Setting Module Path

```bash
# Add custom module path
export GRAPHOID_PATH="/path/to/my/modules:$GRAPHOID_PATH"
```

### Checking Module Path

```graphoid
import "os"

paths = os.get_module_paths()
for path in paths {
    print(path)
}
```

## Private Modules

Use `priv` keyword to create module-private definitions:

```graphoid
# File: calculator.gr

# Private constant
priv SECRET_KEY = "abc123"

# Private function
priv fn internal_calc(x) {
    return x * 2
}

# Public function (can use private definitions)
export fn calculate(x) {
    return internal_calc(x) + SECRET_KEY.length()
}
```

External code cannot access `SECRET_KEY` or `internal_calc`.

## Module Caching

Modules are cached after first import for performance:

```graphoid
# First import: module loads and executes
import "expensive_module"

# Subsequent imports: returns cached module
import "expensive_module"  # Fast!
```

### Force Reload (Development Only)

```graphoid
# Reload module (useful during development)
import.reload("my_module")
```

## Summary

In this chapter, you learned:

- ✅ **Importing modules** - Standard library and custom modules
- ✅ **Exporting** - Making code reusable
- ✅ **Import variants** - Specific items, renaming, wildcards
- ✅ **Module organization** - Project structure and relative imports
- ✅ **Namespaces** - Each module has its own scope
- ✅ **Circular dependencies** - How to avoid them
- ✅ **Standard library** - Available modules
- ✅ **Best practices** - Focused modules, clear names, documentation

---

## Quick Reference

```graphoid
# Import entire module
import "math"
result = math.sqrt(16)

# Import specific items
import { sqrt, pow } from "math"
result = sqrt(16)

# Import with alias (for custom modules only)
import "my_module" as mm
result = mm.function()

# Export
export fn my_function() { ... }
export default value

# Private
priv fn internal() { ... }

# Reload (development)
import.reload("module")
```

---

## Exercises

1. **Math Library**: Create a module with functions for `factorial`, `fibonacci`, and `is_prime`

2. **String Utils**: Create a module with functions for `reverse`, `is_palindrome`, and `count_vowels`

3. **User Model**: Create a module that defines a user with validation functions

4. **Module Refactor**: Take a large program and split it into focused modules

5. **Import Graph**: Write a function that analyzes which modules import which other modules

6. **Module Bundle**: Create a module that re-exports multiple other modules as a single entry point

**Solutions** are available in `examples/07-modules/exercises.gr`

---

[← Previous: Graph Operations](06-graph-operations.md) | [Next: Directives →](08-directives.md)
