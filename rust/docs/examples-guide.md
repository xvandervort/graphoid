# Graphoid Examples

This directory contains example programs demonstrating Graphoid's features.

## Running Examples

From the `rust/` directory:

```bash
# Run an example
~/.cargo/bin/cargo run --quiet examples/hello_world.gr

# Or build once and run faster
~/.cargo/bin/cargo build --release
./target/release/graphoid examples/hello_world.gr
```

## Available Examples

### 1. hello_world.gr
**Your first Graphoid program**

Demonstrates:
- Basic print statements
- Variables
- String operations (upper, len, replace)
- Simple arithmetic

```bash
~/.cargo/bin/cargo run --quiet examples/hello_world.gr
```

### 2. collections.gr
**Lists and Maps**

Demonstrates:
- List creation and methods (first, last, len)
- Functional operations (map, filter, reject)
- Immutable operations (insert, remove)
- Map creation and access
- Map methods (keys, values, has_key, size, is_empty)

```bash
~/.cargo/bin/cargo run --quiet examples/collections.gr
```

### 3. behaviors.gr
**Automatic Value Transformations**

Demonstrates Graphoid's unique behavior system:
- `none_to_zero` - Replace none with 0
- `positive` - Convert negatives to absolute values
- `round_to_int` - Round decimals
- `validate_range` - Clamp values to min/max
- `uppercase` / `lowercase` - String transformations
- `no_duplicates` - Enforce uniqueness
- Chaining multiple behaviors

```bash
~/.cargo/bin/cargo run --quiet examples/behaviors.gr
```

### 4. functions.gr
**Functions and Control Flow**

Demonstrates:
- Function definitions with parameters
- Return values
- Lambdas (anonymous functions)
- Conditionals (if/else, unless)
- Loops (for, while)
- Pattern matching with pipe syntax

```bash
~/.cargo/bin/cargo run --quiet examples/functions.gr
```

### 5. graphs.gr
**Graph Data Structures**

Demonstrates the core feature of Graphoid:
- Creating directed, undirected, and DAG graphs
- Adding nodes and edges
- Querying neighbors and connections
- Graph traversal (BFS, DFS)
- Shortest path algorithms
- Weighted graphs for routing
- Dependency graphs

```bash
~/.cargo/bin/cargo run --quiet examples/graphs.gr
```

### 6. modules_basic.gr ✅ Working

**Module System Basics**

Explains the basic concepts of the module system:
- Module declaration syntax: `module name alias short`
- Import statements: `import "./path/to/module"`
- Namespace access: `module_alias.function()`
- Load vs import differences

This is an informational example explaining how modules work.

```bash
~/.cargo/bin/cargo run --quiet examples/modules_basic.gr
```

### 7. modules_math.gr + modules_main.gr ✅ Working

**Multi-File Module Example**

`modules_math.gr` is a complete module demonstrating:
- Module declaration with alias
- Exporting constants (PI, E, GOLDEN_RATIO)
- Exporting functions (square, cube, power, circle_area, etc.)
- Utility functions (abs, max, min)

`modules_main.gr` imports and uses the math module:
- Importing `modules_math.gr`
- Accessing module constants
- Calling module functions
- Full working example of multi-file programs

```bash
~/.cargo/bin/cargo run --quiet examples/modules_main.gr
```

Expected output:
```
=== Using Math Module ===

Mathematical constants:
  PI = 3.14159265359
  E = 2.71828182846
  Golden Ratio = 1.61803398875

Basic operations:
  square(7) = 49
  cube(3) = 27
  power(2, 10) = 1024

Circle with radius 5 :
  Area = 78.53981633974999
  Circumference = 31.4159265359

Sphere with radius 5 :
  Volume = 523.5987755983333

Utility functions:
  abs(-42) = 42
  max(10, 20) = 20
  min(10, 20) = 10
```

## Interactive REPL

Try Graphoid interactively:

```bash
~/.cargo/bin/cargo run --quiet
```

Example session:
```graphoid
graphoid> numbers = [1, 2, 3, 4, 5]
[1, 2, 3, 4, 5]
graphoid> doubled = numbers.map(x => x * 2)
[2, 4, 6, 8, 10]
graphoid> doubled.filter(x => x > 5)
[6, 8, 10]
graphoid> exit
```

## Quick Reference

### Data Types
- Numbers: `42`, `3.14`
- Strings: `"hello"`, `'world'`
- Booleans: `true`, `false`
- None: `none`
- Lists: `[1, 2, 3]`
- Maps: `{"key": "value"}`
- Symbols: `:symbol_name`

### Collection Methods

**List Methods:**
- `len()`, `size()`, `length()` - Get length
- `first()`, `last()` - Get first/last element
- `map(fn)`, `filter(fn)`, `reject(fn)` - Functional operations
- `insert(index, value)` - Insert at position
- `remove(value)` - Remove value
- `append(value)` - Add to end
- `reverse()`, `sort()`, `uniq()` - Transformations
- `add_rule(:rule)` - Add behavior

**Map Methods:**
- `keys()`, `values()` - Get all keys/values
- `has_key(key)` - Check if key exists
- `size()`, `len()`, `length()` - Get size
- `is_empty()` - Check if empty
- `remove(key)` - Remove key

**String Methods:**
- `len()`, `length()`, `size()` - Get length
- `upper()`, `lower()` - Change case
- `trim()` - Remove whitespace
- `replace(old, new)` - Replace substring
- `index_of(sub)` - Find substring position
- `split(delim)` - Split into list

### Module System

Organize code into reusable modules:

**Module Declaration (in module file):**
```graphoid
module math alias m

PI = 3.14159
fn square(x) { return x * x }
```

**Import Module (in main file):**
```graphoid
import "./math"
result = m.square(5)
pi = m.PI
```

**Load File (merges into current namespace):**
```graphoid
load "./utilities.gr"
# Can use functions directly without namespace
```

See `modules_math.gr` and `modules_main.gr` for complete working example.

### Behavior Rules

Transform values automatically:
- `:none_to_zero` - Replace none with 0
- `:none_to_empty` - Replace none with ""
- `:positive` - Absolute values
- `:round_to_int` - Round decimals
- `:uppercase` - Convert to uppercase
- `:lowercase` - Convert to lowercase
- `:validate_range` - Clamp to range (requires min, max)
- `:no_duplicates` - Enforce uniqueness

Example:
```graphoid
temps = [98.6, none, 102.5].add_rule(:none_to_zero)
values = [50, 110, 95].add_rule(:validate_range, 60, 100)
```

## Learn More

- See `../CLAUDE.md` for project overview
- See `../dev_docs/LANGUAGE_SPECIFICATION.md` for complete syntax
- Run `~/.cargo/bin/cargo test` to see all tests passing

## Creating Your Own .gr Files

Create a new file with `.gr` extension:

```bash
cat > my_program.gr << 'EOF'
# My first program
print("Hello from Graphoid!")

numbers = [1, 2, 3, 4, 5]
squares = numbers.map(x => x * x)
print("Squares:", squares)
EOF

~/.cargo/bin/cargo run --quiet my_program.gr
```

Happy coding with Graphoid!
