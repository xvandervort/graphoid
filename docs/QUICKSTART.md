# Graphoid Quick Start Guide

Welcome to **Graphoid** - the revolutionary graph-theoretic programming language where **everything is a graph**!

## What Makes Graphoid Different?

Unlike traditional languages that bolt graphs onto the side, Graphoid makes graphs the fundamental abstraction at every level. Lists are graphs. Maps are graphs. Even your variable storage is a graph!

Plus, Graphoid includes a powerful **behavior system** that automatically transforms values, keeping your data clean without manual validation.

## Installation & Setup

### Prerequisites

You need Rust and Cargo installed. If you don't have them:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installing Graphoid

```bash
# Clone the repository
git clone https://github.com/yourusername/graphoid.git
cd graphoid

# Install to ~/.local (recommended)
make install

# Or install system-wide
sudo make install PREFIX=/usr/local
```

Ensure `~/.local/bin` is in your PATH. Add to your `~/.bashrc` if needed:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

After installation, `gr` is available anywhere:

```bash
gr version
gr samples/01-basics/hello_world.gr
gr   # Starts the REPL
```

### Quick Test

```bash
gr
```

You should see the Graphoid REPL prompt:

```
Graphoid v0.1.0
Type /exit to quit, /help for help
>
```

Type `/exit` or press Ctrl+D to quit.

## Your First Graphoid Program

### Interactive Mode (REPL)

The fastest way to try Graphoid is the interactive REPL:

```bash
gr
```

Try these commands:

```graphoid
> print("Hello, Graphoid!")
Hello, Graphoid!

> name = "Alice"
Alice

> age = 25
25

> print(name, "is", age, "years old")
Alice is 25 years old

> numbers = [1, 2, 3, 4, 5]
[1, 2, 3, 4, 5]

> doubled = numbers.map(x => x * 2)
[2, 4, 6, 8, 10]

> print("Doubled:", doubled)
Doubled: [2, 4, 6, 8, 10]

> /exit
```

### File Mode

Create a file called `hello.gr`:

```graphoid
# hello.gr - My first Graphoid program

print("Hello, World!")

# Variables
name = "Alice"
age = 25
print(name, "is", age, "years old")

# Collections
numbers = [1, 2, 3, 4, 5]
print("Numbers:", numbers)

# Functional programming
doubled = numbers.map(x => x * 2)
print("Doubled:", doubled)

evens = numbers.filter(x => x % 2 == 0)
print("Even numbers:", evens)
```

Run it:

```bash
gr hello.gr
```

Output:
```
Hello, World!
Alice is 25 years old
Numbers: [1, 2, 3, 4, 5]
Doubled: [2, 4, 6, 8, 10]
Even numbers: [2, 4]
```

## Core Concepts

### 1. Everything is Immutable (by default)

Operations return **new values** rather than modifying existing ones:

```graphoid
numbers = [1, 2, 3, 4, 5]
with_99 = numbers.insert(2, 99)

print("Original:", numbers)      # [1, 2, 3, 4, 5]
print("Modified:", with_99)       # [1, 2, 99, 3, 4, 5]
```

### 2. Collections are Graphs

Lists, maps, and graphs are all built on the same graph foundation:

```graphoid
# Lists are linked graphs
items = [1, 2, 3]

# Maps are hash graphs
person = {"name": "Alice", "age": 30}

# Graphs are... graphs!
network = graph { type: :directed }
network.add_node("A", 100)
network.add_edge("A", "B", "connects")
```

### 3. Powerful Method Chaining

Chain operations for expressive data transformations:

```graphoid
result = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    .filter(x => x % 2 == 0)     # Get evens: [2, 4, 6, 8, 10]
    .map(x => x * x)              # Square them: [4, 16, 36, 64, 100]
    .reject(x => x > 50)          # Remove > 50: [4, 16, 36]

print(result)  # [4, 16, 36]
```

### 4. Automatic Value Transformation (Behaviors)

Graphoid's unique **behavior system** automatically transforms values:

```graphoid
# Replace none values with 0
temperatures = [98.6, none, 102.5, none]
clean = temperatures.add_rule(:none_to_zero)
print(clean)  # [98.6, 0, 102.5, 0]

# Convert negative numbers to positive
numbers = [-5, 10, -3, 7]
positive = numbers.add_rule(:positive)
print(positive)  # [5, 10, 3, 7]

# Clamp values to a range
scores = [50, 110, 95, 130, 85]
clamped = scores.add_rule(:validate_range, 60, 100)
print(clamped)  # [60, 100, 95, 100, 85]
```

## Common Operations

### Working with Lists

```graphoid
# Create a list
items = [1, 2, 3, 4, 5]

# Access elements
print("First:", items.first())      # 1
print("Last:", items.last())        # 5
print("Length:", items.len())       # 5

# Transform
doubled = items.map(x => x * 2)     # [2, 4, 6, 8, 10]
evens = items.filter(x => x % 2 == 0)  # [2, 4]
odds = items.reject(x => x % 2 == 0)   # [1, 3, 5]

# Modify (returns new list)
with_zero = items.insert(0, 0)      # [0, 1, 2, 3, 4, 5]
without_3 = items.remove(3)         # [1, 2, 4, 5]
reversed = items.reverse()          # [5, 4, 3, 2, 1]

# Utilities
unique = [1, 2, 2, 3, 3, 3].uniq()  # [1, 2, 3]
clean = [1, none, 2, none].compact()  # [1, 2]
```

### Working with Maps

```graphoid
# Create a map
person = {
    "name": "Alice",
    "age": 30,
    "city": "Portland"
}

# Access values
print(person["name"])               # Alice

# Query
print("Keys:", person.keys())       # [name, age, city]
print("Values:", person.values())   # [Alice, 30, Portland]
print("Has age?", person.has_key("age"))  # true
print("Size:", person.size())       # 3

# Modify (returns new map)
updated = person.remove("city")
print("Keys:", updated.keys())      # [name, age]
```

### Working with Strings

```graphoid
text = "hello world"

# Transform
print(text.upper())                 # HELLO WORLD
print(text.lower())                 # hello world
print(text.trim())                  # (removes whitespace)

# Query
print("Length:", text.len())        # 11
print("Index of 'world':", text.index_of("world"))  # 6

# Modify
print(text.replace("world", "Graphoid"))  # hello Graphoid

# Split
words = text.split(" ")             # [hello, world]
```

### Working with Graphs

```graphoid
# Create a directed graph
network = graph { type: :directed }

# Add nodes (with values)
network.add_node("Server1", 100)
network.add_node("Server2", 200)
network.add_node("Client", 50)

# Add edges
network.add_edge("Server1", "Server2", "connects")
network.add_edge("Client", "Server1", "requests")

# Query
print("Nodes:", network.nodes())
print("Neighbors:", network.neighbors("Server1"))
print("Has edge?", network.has_edge("Client", "Server1", "requests"))

# Traverse
print("BFS:", network.bfs("Client"))
print("DFS:", network.dfs("Client"))

# Find paths
path = network.shortest_path("Client", "Server2", none, false)
print("Shortest path:", path)
```

## Behavior Rules Reference

Transform values automatically with behaviors:

### Transformation Behaviors

```graphoid
# Replace none with 0
list.add_rule(:none_to_zero)

# Replace none with empty string
list.add_rule(:none_to_empty)

# Convert negatives to absolute values
list.add_rule(:positive)

# Round decimals to integers
list.add_rule(:round_to_int)

# Convert strings to uppercase
list.add_rule(:uppercase)

# Convert strings to lowercase
list.add_rule(:lowercase)

# Clamp numbers to range [min, max]
list.add_rule(:validate_range, 60, 100)
```

### Validation Behaviors

```graphoid
# Enforce unique values only
list.add_rule(:no_duplicates)

# For graphs: require weighted edges
graph.add_rule(:weighted_edges)

# For graphs: require unweighted edges
graph.add_rule(:unweighted_edges)
```

### Chaining Behaviors

Combine multiple behaviors for powerful transformations:

```graphoid
data = [-5.7, none, 3.2, -1.9, none, 8.1]

result = data
    .add_rule(:none_to_zero)
    .add_rule(:positive)
    .add_rule(:round_to_int)

print(result)  # [6, 0, 3, 2, 0, 8]
```

## Module System

Graphoid supports multi-file programs through modules. Organize your code into reusable, maintainable pieces.

### Creating a Module

**File: math.gr**
```graphoid
# Declare this file as a module
module math alias m

# Export constants
PI = 3.14159265359
E = 2.71828182846

# Export functions
fn square(x) {
    return x * x
}

fn circle_area(radius) {
    return PI * radius * radius
}

fn abs(x) {
    if x < 0 {
        return -x
    }
    return x
}
```

### Importing a Module

**File: main.gr**
```graphoid
# Import the math module
import "./math"

# Use module alias to access exports
print("PI =", m.PI)

area = m.circle_area(5)
print("Circle area:", area)

val = m.abs(-42)
print("Absolute value:", val)
```

### Module Declaration Syntax

```graphoid
# Full syntax
module <module_name> alias <short_name>

# Example
module utilities alias util
module database alias db
module network alias net
```

The `alias` provides a short name for accessing the module's members.

### Import Syntax

```graphoid
# Import with relative path
import "./math"           # Same directory
import "./lib/utils"      # Subdirectory
import "../shared/config" # Parent directory

# Import with alias override
import "./math" alias math_lib

# Module is bound by its declared alias (or override)
result = m.square(10)     # Uses module's declared alias
```

### Load Statement (Alternative)

The `load` statement merges a file's contents directly into your namespace:

```graphoid
# Load merges into current namespace (no module boundary)
load "./utilities.gr"

# Functions available directly (no namespace prefix)
result = square(5)  # No module prefix needed
```

The `load` statement accepts expressions, not just string literals:

```graphoid
# Load from a variable
config_path = "config/" + environment + ".gr"
load config_path

# Dynamic loading in a loop
for file in spec_files {
    load file
}
```

**Import vs Load:**
- **`import`**: Creates isolated namespace, access via alias
- **`load`**: Merges directly into current namespace

### Module Caching

Modules are loaded once and cached. Subsequent imports reuse the cached module:

```graphoid
import "./math"  # Loads and caches
import "./math"  # Reuses cache (instant)
```

### Circular Dependency Detection

Graphoid detects and prevents circular imports:

```graphoid
# File: a.gr
import "./b"  # b imports c, c imports a → ERROR

# Error: Circular dependency detected: a.gr → b.gr → c.gr → a.gr
```

### Module Resolution

Graphoid searches for modules in this order:

1. **Relative paths**: `./`, `../`
2. **Same directory** as importing file
3. **Search paths**: `src/`, `lib/`, `stdlib/`

Extensions are optional:
```graphoid
import "./math"     # Resolves to math.gr
import "./math.gr"  # Same result
```

### Example: Multi-File Project

```
my_project/
├── main.gr
├── lib/
│   ├── math.gr
│   └── string_utils.gr
└── services/
    └── api.gr
```

**lib/math.gr:**
```graphoid
module math alias m
PI = 3.14159
fn square(x) { return x * x }
```

**lib/string_utils.gr:**
```graphoid
module string_utils alias str
fn capitalize(text) {
    # Implementation
}
```

**services/api.gr:**
```graphoid
module api alias api
import "../lib/math"

fn calculate_area(r) {
    return m.PI * r * r
}
```

**main.gr:**
```graphoid
import "./lib/math"
import "./lib/string_utils"
import "./services/api"

# Use all modules
area = api.calculate_area(5)
print("Area:", area)
```

### Working Examples

See `samples/04-modules/` for complete working module examples:
- `load_vs_import.gr` - Understanding load vs import
- `priv_keyword.gr` - Private symbols and encapsulation
- `app_main.gr` - Multi-file project with module hierarchy

Run them:
```bash
gr samples/04-modules/app_main.gr
```

---

## Functions and Control Flow

### Functions

```graphoid
# Define a function
func greet(name) {
    print("Hello,", name)
}

greet("Alice")

# With return value
func add(a, b) {
    return a + b
}

result = add(5, 3)
print(result)  # 8
```

### Lambdas (Anonymous Functions)

```graphoid
# Single parameter
square = x => x * x
print(square(7))  # 49

# Multiple parameters
multiply = (a, b) => a * b
print(multiply(4, 5))  # 20

# With collections
numbers = [1, 2, 3, 4, 5]
squares = numbers.map(x => x * x)
print(squares)  # [1, 4, 9, 16, 25]
```

### Conditionals

```graphoid
# if-else
age = 25
if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}

# Inline conditional
status = if age >= 21 { "Can drink" } else { "Cannot drink" }

# unless (inverse of if)
unless age < 18 {
    print("Not a minor")
}
```

### Loops

```graphoid
# for loop
for item in [1, 2, 3, 4, 5] {
    print(item)
}

# while loop
count = 5
while count > 0 {
    print(count)
    count = count - 1
}
```

## Example Programs

Try these complete programs:

### Example 1: Data Cleaning with Behaviors

```graphoid
# sensor_data.gr - Clean sensor readings

# Raw sensor data with missing values and outliers
readings = [98.6, none, 102.5, none, 150, 99.1, -5]

print("Raw data:", readings)

# Clean the data
clean = readings
    .add_rule(:none_to_zero)           # Replace missing with 0
    .add_rule(:positive)                # Fix negative errors
    .add_rule(:validate_range, 95, 105) # Clamp outliers

print("Cleaned:", clean)
# Output: [98.6, 0, 102.5, 0, 105, 99.1, 5]

# Further processing
average = clean.filter(x => x > 0).map(x => x)  # Remove zeros
print("Valid readings:", average)
```

### Example 2: Social Network Graph

```graphoid
# network.gr - Social network analysis

# Create social network
social = graph { type: :undirected }

# Add people
social.add_node("Alice", none)
social.add_node("Bob", none)
social.add_node("Carol", none)
social.add_node("Dave", none)

# Add friendships
social.add_edge("Alice", "Bob", "friend")
social.add_edge("Alice", "Carol", "friend")
social.add_edge("Bob", "Dave", "friend")
social.add_edge("Carol", "Dave", "friend")

print("Network:", social.nodes())

# Find connections
print("Alice's friends:", social.neighbors("Alice"))
print("Path Alice to Dave:", social.shortest_path("Alice", "Dave", none, false))

# Analyze network
print("All connections from Alice:", social.bfs("Alice"))
```

### Example 3: Data Processing Pipeline

```graphoid
# pipeline.gr - Process and analyze data

# Student scores with some issues
scores = [85, none, 92, 150, -5, 88, none, 78, 95]

print("=== Data Processing Pipeline ===")
print("Raw scores:", scores)

# Step 1: Clean data
clean_scores = scores
    .add_rule(:none_to_zero)
    .add_rule(:positive)
    .add_rule(:validate_range, 0, 100)

print("After cleaning:", clean_scores)

# Step 2: Filter valid scores
valid_scores = clean_scores.filter(x => x > 0)
print("Valid scores only:", valid_scores)

# Step 3: Categorize
passing = valid_scores.filter(x => x >= 60)
print("Passing scores:", passing)
print("Number of passing:", passing.len())

# Step 4: Calculate statistics
sorted_scores = passing.sort()
print("Sorted:", sorted_scores)
print("Lowest passing:", sorted_scores.first())
print("Highest:", sorted_scores.last())
```

## Next Steps

### Explore More Examples

Check out the `samples/` directory for more complete examples:

```bash
gr samples/01-basics/hello_world.gr
gr samples/01-basics/collections.gr
gr samples/02-intermediate/behaviors.gr
gr samples/04-modules/app_main.gr
gr samples/01-basics/functions.gr
gr samples/01-basics/graphs.gr
gr samples/06-projects/web_server/simple.gr   # Web server!
```

See `samples/README.md` for detailed descriptions.

### Learn the Language

- **Complete syntax**: See `dev_docs/LANGUAGE_SPECIFICATION.md`
- **Implementation details**: See `dev_docs/ARCHITECTURE_DESIGN.md`
- **Roadmap**: See `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`

### Run Tests

See the comprehensive test suite:

```bash
make test          # Rust unit tests
gr spec tests/     # Graphoid spec tests
```

Currently **2,228+ tests passing** with zero failures!

## Quick Reference Card

### Data Types

```graphoid
42                    # Number
3.14                  # Number (float)
"hello"               # String
'world'               # String
true, false           # Boolean
none                  # None/null
[1, 2, 3]            # List
{"key": "value"}      # Map
:symbol               # Symbol
```

### Operators

```graphoid
+, -, *, /           # Arithmetic
//                   # Integer division
%                    # Modulo
==, !=               # Equality
<, >, <=, >=        # Comparison
and, or, not        # Logical
```

### Collection Methods

```graphoid
# Lists
.len(), .size(), .length()
.first(), .last()
.map(fn), .filter(fn), .reject(fn)
.insert(i, val), .remove(val), .append(val)
.reverse(), .sort(), .uniq(), .compact()

# Maps
.keys(), .values()
.has_key(key), .size(), .is_empty()
.remove(key)

# Strings
.len(), .upper(), .lower(), .trim()
.replace(old, new), .index_of(sub), .split(delim)

# Graphs
.nodes(), .edges()
.add_node(id, val), .add_edge(from, to, type)
.neighbors(id), .bfs(start), .dfs(start)
.shortest_path(from, to, edge_type, weighted)
```

### Behavior Rules

```graphoid
:none_to_zero        # none → 0
:none_to_empty       # none → ""
:positive            # abs(n)
:round_to_int        # round(n)
:uppercase           # STRING
:lowercase           # string
:validate_range      # clamp(min, max)
:no_duplicates       # unique values only
```

### Module System

```graphoid
# Module declaration (in module file)
module math alias m

# Import module (in main file)
import "./math"
import "./lib/utils"
import "../config"

# Load file (merges namespace)
load "./utilities.gr"

# Access module members
m.square(5)          # Call function
m.PI                 # Access constant

# Module caching & circular detection built-in
```

## Getting Help

- **Examples**: `samples/`
- **Documentation**: `docs/` and `dev_docs/`
- **Tests**: Run `make test` or `gr spec tests/`
- **REPL**: Try things interactively with `gr`

## Tips & Tricks

1. **Use the REPL for experimentation** - It's the fastest way to try new ideas
2. **Chain methods** - Graphoid is designed for method chaining
3. **Use behaviors for data cleaning** - Let the system handle transformations
4. **Everything is immutable** - Operations return new values
5. **Graphs are everywhere** - Even lists and maps are graphs underneath

---

**Welcome to Graphoid!** Start with the REPL, try the examples, and explore the power of graph-theoretic programming. Everything is a graph here! 🚀
