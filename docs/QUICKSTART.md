# Graphoid Quick Start Guide

Welcome to **Graphoid** - the revolutionary graph-theoretic programming language where **everything is a graph**!

## What Makes Graphoid Different?

Unlike traditional languages that bolt graphs onto the side, Graphoid makes graphs the fundamental abstraction at every level. Lists are graphs. Maps are graphs. Even your variable storage is a graph!

Plus, Graphoid includes a powerful **behavior system** that automatically transforms values, keeping your data clean without manual validation.

## Installation & Setup

### Prerequisites

You need Rust and Cargo installed. The Graphoid binary is built from the Rust implementation.

### Building Graphoid

```bash
cd rust/
~/.cargo/bin/cargo build --release
```

The compiled binary will be at `rust/target/release/graphoid`.

### Quick Test

```bash
# From rust/ directory
~/.cargo/bin/cargo run --quiet
```

You should see the Graphoid REPL prompt:

```
graphoid>
```

Type `exit` or press Ctrl+D to quit.

## Your First Graphoid Program

### Interactive Mode (REPL)

The fastest way to try Graphoid is the interactive REPL:

```bash
~/.cargo/bin/cargo run --quiet
```

Try these commands:

```graphoid
graphoid> print("Hello, Graphoid!")
Hello, Graphoid!

graphoid> name = "Alice"
Alice

graphoid> age = 25
25

graphoid> print(name, "is", age, "years old")
Alice is 25 years old

graphoid> numbers = [1, 2, 3, 4, 5]
[1, 2, 3, 4, 5]

graphoid> doubled = numbers.map(x => x * 2)
[2, 4, 6, 8, 10]

graphoid> print("Doubled:", doubled)
Doubled: [2, 4, 6, 8, 10]

graphoid> exit
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
~/.cargo/bin/cargo run --quiet hello.gr
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

Check out the `rust/examples/` directory for more complete examples:

```bash
# From rust/ directory
~/.cargo/bin/cargo run --quiet examples/hello_world.gr
~/.cargo/bin/cargo run --quiet examples/collections.gr
~/.cargo/bin/cargo run --quiet examples/behaviors.gr
~/.cargo/bin/cargo run --quiet examples/functions.gr
~/.cargo/bin/cargo run --quiet examples/graphs.gr
```

See `rust/examples/README.md` for detailed descriptions.

### Learn the Language

- **Complete syntax**: See `dev_docs/LANGUAGE_SPECIFICATION.md`
- **Implementation details**: See `dev_docs/ARCHITECTURE_DESIGN.md`
- **Roadmap**: See `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`

### Run Tests

See the comprehensive test suite:

```bash
~/.cargo/bin/cargo test
```

Currently **768+ tests passing** with zero failures!

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

## Getting Help

- **Examples**: `rust/examples/`
- **Documentation**: `docs/` and `dev_docs/`
- **Tests**: Run `~/.cargo/bin/cargo test` to see working examples
- **REPL**: Try things interactively with `~/.cargo/bin/cargo run --quiet`

## Tips & Tricks

1. **Use the REPL for experimentation** - It's the fastest way to try new ideas
2. **Chain methods** - Graphoid is designed for method chaining
3. **Use behaviors for data cleaning** - Let the system handle transformations
4. **Everything is immutable** - Operations return new values
5. **Graphs are everywhere** - Even lists and maps are graphs underneath

---

**Welcome to Graphoid!** Start with the REPL, try the examples, and explore the power of graph-theoretic programming. Everything is a graph here! 🚀
