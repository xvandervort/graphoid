# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Glang** is a general-purpose programming language with revolutionary graph-theoretic features, designed to be both practically useful and conceptually powerful.

### Current Vision (Updated January 2025)
Glang aims to be:
1. **A Practical Language First** - Complete with I/O, networking, databases, and standard library
2. **Then a Graph Language** - With true graph structures including nodes, edges, and traversal
3. **Finally a Revolutionary Platform** - Enabling self-aware, self-mutating, distributed graph systems

### Core Features (Implemented)
- **Complete Function System** - Functions, lambdas, closures, and recursion
- **Strong Type System** - With optional type inference and type constraints
- **Modern Collections** - Lists, hashes, and data nodes (currently container-based, not true graphs yet)
- **Method-Based Design** - Everything uses methods: `list.append()`, `string.upper()`, `num.abs()`
- **File Loading System** - Modular programming with `.gr` files
- **Clean AST Architecture** - Reliable execution with excellent error messages

### Design Principles:
- **Practical First** - Must be useful for real-world applications before adding advanced features
- **Graph-Theoretic Foundation** - All data will eventually be conceptualized as graphs with nodes and edges
- **Self-Aware Data Structures** - Future: Collections that understand their own structure and relationships
- **Intuitive Syntax** - Natural programming constructs that feel familiar
- **Developer Experience** - Excellent error messages, REPL environment, and reflection capabilities

### Architectural Discovery (January 2025)
**Critical Realization**: Current "graph" types (lists, hashes) are actually just containers. True graph features require:
- **Edges**: Explicit relationships between nodes with metadata
- **Node Awareness**: Nodes knowing their container and neighbors
- **Graph Traversal**: Real pathfinding and connectivity analysis

This represents a major architectural challenge but is essential for Glang's unique vision.

## Repository Structure

- `src/glang/` - Main language package
  - `ast/` - Abstract syntax tree nodes and visitor pattern
  - `lexer/` - Modern tokenizer for language lexing
  - `parser/` - AST parser for building typed syntax trees
  - `semantic/` - Semantic analysis and type checking
  - `execution/` - AST execution engine with type-safe runtime
  - `files/` - File loading/saving system for .gr programs
  - `repl/` - Modern REPL implementation
  - `cli.py` - Command-line interface
- `test/` - Comprehensive test suite (230+ tests with growing coverage)
- `stdlib/` - Standard library modules written in Glang
- `samples/` - Example programs and demonstrations
- `docs/` - Documentation files including language cheat sheet

## Development Setup

This project uses a Python virtual environment. Make sure it's activated before running commands:

```bash
# Activate virtual environment (if not already activated)
source .venv/bin/activate  # On Linux/Mac
# or
.venv\Scripts\activate     # On Windows
```

## Development Commands

```bash
# Run the REPL (with full navigation: ↑/↓ history, tab completion)
python -m glang.repl
# or after installation:
glang

# Quick demo (scripted)
echo -e "string name = \"Alice\"\\nname\\n/namespace\\n/exit" | glang

# Run tests
pytest test/

# Install in development mode with all dependencies
pip install -e ".[dev]"

# Code formatting
black src/ test/

# Type checking
mypy src/

# Demonstrate the language
echo -e "string greeting = \"Hello World\"\nlist items = [1, 2, 3]\ndata user = { \"name\": \"Alice\" }\nhash config = { \"host\": \"localhost\", \"port\": 8080 }\nitems.append(4)\nconfig[\"debug\"] = true\n/namespace\n/exit" | glang
```

## REPL Commands

### Basic Commands
- `/help` or `/h` - Show help information
- `/version` or `/ver` - Show version information  
- `/exit` or `/x` - Exit the REPL

### File Operations
- `/load <file>` - Load and execute a .gr file
- `/save <file>` - Save current session to .gr file
- `/run <file>` - Run .gr file in fresh session

### Session Management
- `/namespace` or `/ns` - Show all current variables
- `/stats` - Show session statistics
- `/clear` - Clear all variables

## Language Syntax

### Variable Declarations
```glang
# Optional type declarations (types can be omitted when obvious)
name = "Alice"         # Infers string type from string literal
age = 25               # Infers num type from numeric literal
active = true          # Infers bool type from boolean literal
items = [1, 2, 3]      # Infers list type from list literal

# Explicit types when needed (for clarity or constraints)
string username = "Bob"
num max_age = 100
bool is_valid = false

# Type-constrained lists (explicit type required for constraints)
list<num> scores = [95, 87, 92]
list<string> names = ["alice", "bob", "charlie"]
list<bool> flags = [true, false, true]

# Data nodes (type inferred from literal)
user = { "name": "Alice" }           # Infers data type
data<num> score = { "final": 95 }    # Explicit constraint

# Hashes (type inferred from literal)
config = { "host": "localhost", "port": 8080, "debug": true }  # Infers hash type
hash<string> settings = { "theme": "dark", "lang": "en" }       # Explicit constraint
```

### Advanced Operations
```glang
# Index access
items[0]           # Get first element from list
items[-1]          # Get last element (if supported)
config["host"]     # Get data node from hash: { "host": "localhost" }

# Index assignment  
items[0] = 99      # Set first element in list
scores[1] = 100    # Update element
config["port"] = 9000  # Create/update data node in hash

# Method calls
items.append(4)    # Add element to list
names.append("dave") # Type-checked append
config.get("host") # Get data node: { "host": "localhost" }
config.has_key("debug")  # Check if key exists: true/false
config.count_values("localhost")  # Count occurrences: 1

# Data node operations
user.key()         # Get key: "name"
user.value()       # Get value: "Alice"

# Functional programming operations (new!)
numbers = [1, 2, 3, 4, 5]
numbers.map("double")        # [2, 4, 6, 8, 10]
numbers.filter("even")       # [2, 4]
numbers.filter("positive")   # [1, 2, 3, 4, 5]
numbers.select("odd")        # [1, 3, 5] (alias for filter)
numbers.reject("even")       # [1, 3, 5] (opposite of filter)

# String transformations
names = ["alice", "bob"]
names.map("upper")           # ["ALICE", "BOB"]
names.map("lower")           # ["alice", "bob"]

# Type conversions
nums = [1, 2, 3]
nums.map("to_string")        # ["1", "2", "3"]
strings = ["1", "2", "3"]
strings.map("to_num")        # [1, 2, 3]

# Chaining operations
numbers.filter("positive").map("double").filter("even")
names.map("upper").each("print")  # Print each uppercase name
```

#### Available Transformations for map()
**Numeric:** `double`, `square`, `negate`, `increment`/`inc`, `decrement`/`dec`  
**String:** `upper`/`up`, `lower`/`down`, `trim`, `reverse`  
**Type Conversion:** `to_string`/`str`, `to_num`/`num`, `to_bool`/`bool`

#### Available Predicates for filter()
**Numeric:** `positive`/`pos`, `negative`/`neg`, `zero`, `even`, `odd`  
**String/Collection:** `empty`, `non_empty`, `uppercase`, `lowercase`, `alphabetic`/`alpha`, `numeric`/`digit`  
**Type Checks:** `is_string`/`string`, `is_number`/`number`, `is_bool`/`boolean`, `is_list`/`list`  
**General:** `truthy`, `falsy`

### Functions and Lambdas

```glang
# Function declarations
func greet(name) {
    return "Hello, " + name + "!"
}

func add(x, y) {
    return x + y
}

# Function calls
message = greet("World")      # "Hello, World!"
result = add(15, 27)          # 42

# Lambda expressions
double = x => x * 2
multiply = (x, y) => x * y

# Using lambdas
result = double(5)            # 10
product = multiply(7, 8)      # 56

# Recursive functions
func fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

### Control Flow

```glang
# If statements
if condition {
    # execute when true
}

# If-else statements  
if condition {
    # execute when true
} else {
    # execute when false  
}

# Precision context blocks (NEW!)
precision 5 {
    # All numeric calculations use 5-digit precision
    pi = 3.14159265358979323846  # Stored with 5 digits
    area = pi * radius * radius
}

precision 50 {
    # High-precision scientific calculations
    precise_calculation = complex_math()
}

# Nested precision contexts
precision 10 {
    outer_value = 1.0 / 3.0  # 0.3333333333
    
    precision 3 {
        inner_value = 1.0 / 3.0  # 0.333
    }
}

# While loops
while condition {
    # loop body
}

# For-in loops
for item in items {
    # process each item
}

# Break and continue
for item in items {
    if item == 5 {
        break      # exit loop
    }
    if item % 2 == 0 {
        continue   # skip to next iteration
    }
    print(item)
}

# Nested control structures
for row in matrix {
    for item in row {
        if item > threshold {
            result.append(item)
        }
    }
}

# Control flow with functional operations
if numbers.filter("even").size() > 0 {
    processed = numbers.map("double")
} else {
    processed = numbers.map("negate")
}
```

### File Loading
```glang
# Load another .gr file (language-level)
load "config.gr"     # Variables from config.gr are now available

# Example config.gr:
# debug = true
# max_items = 100

# After loading, use variables directly:
if debug {
    print("Debug mode enabled")
}
```

## Architecture Notes

### Modern AST-Based Design

Glang uses a clean, modern architecture:

1. **Lexical Analysis**: Modern tokenizer with position tracking and comprehensive token types

2. **AST Parsing**: Recursive descent parser builds properly typed abstract syntax trees

3. **Semantic Analysis**: Type checking, symbol table management, and error detection

4. **Execution**: AST visitor pattern with type-safe runtime values

5. **File System**: Modular loading system with .gr file format

### Development Philosophy
- **Minimal Boilerplate**: Type declarations optional when obvious from context
- **Clear Error Messages**: Comprehensive error reporting with source positions
- **Extensible Design**: Clean visitor pattern allows easy language extensions
- **Testing Focus**: 230+ tests with comprehensive coverage ensure reliability
- **Self-Hosting Vision**: Write as much of Glang as possible in Glang itself
- **Standard Library in Glang**: Core functionality implemented in the language, not the host runtime
- **Dogfooding**: Use Glang extensively to validate its expressiveness and identify missing features

### Current Implementation Status  
- ✅ Modern lexer and AST parser
- ✅ Complete semantic analysis with symbol tables
- ✅ Type-safe execution engine
- ✅ **Complete function system** with declarations, calls, and returns
- ✅ **Lambda expressions** with `x => x * 2` syntax
- ✅ File loading system with .gr format
- ✅ Type inference for variable declarations
- ✅ Method calls with type constraint enforcement
- ✅ Index access and assignment for lists, strings, and maps
- ✅ Data nodes with key-value semantics and type constraints
- ✅ Hashes as collections of data nodes with Ruby hash-like syntax
- ✅ Functional programming operations: map, filter, each with built-in transformations
- ✅ Control flow structures: if/else, while, for-in, break/continue with proper nesting
- ✅ **Precision context blocks** with language-level numeric precision control (precision N { ... })
- ✅ CLI program execution with shebang support and command-line arguments
- ✅ Mathematical methods (abs, sqrt, log, pow, rounding) for numbers
- ✅ Type casting system (to_string, to_num, to_bool) for all basic types
- ✅ Standard library foundation with math constants module (stdlib/math.gr)
- ✅ **JSON module** with encode, decode, pretty printing, and validation (json.encode, json.decode, json.is_valid)
- ✅ **Comprehensive test suite** (597+ tests, 64% coverage)

### Development Guidelines
- **AST-first development** - All new features should extend the AST system
- **Smart type inference** - Infer types from context to reduce boilerplate
- **Comprehensive testing** - New features require full test coverage  
- **Clean error messages** - Users should understand exactly what went wrong

## Future Vision: The Path to Revolutionary Graph Computing

### Near-Term Priorities (Q2 2025)
**Make Glang Practical** - Standard libraries for real-world use:
- **✅ I/O Library**: File operations, user input, directory management
- **⏳ Network Library**: ✅ JSON support, HTTP client, email notifications 
- **⏳ Database Connectivity**: SQLite, PostgreSQL, MySQL support
- **⏳ System Library**: OS interaction, processes, date/time

### Medium-Term Goals (Q3 2025)
**Build True Graph Foundation** - Transform containers into real graphs:
- **Enhanced Binary Data Processing**: Hexadecimal literals, fixed-size lists, format detection, image processing
- **Graph Architecture**: Nodes + edges with metadata, not just containers
- **Node Awareness**: Nodes know their container and can access siblings
- **Graph Traversal**: Real pathfinding, connectivity analysis
- **Anonymous Functions**: Function references with `.call()` method

### Long-Term Vision (Q4 2025 and Beyond)
**Revolutionary Graph Features** - What makes Glang unique:

#### Self-Aware Data Structures
```glang
# Future: Hashes that act like classes
statistics = {
    'data': [85.4, 67.3, 92.1],
    'calc_average': func() {
        # This function can access sibling 'data'
        total = sum(this.sibling('data'))
        return total / this.sibling('data').length()
    }
}
average = statistics['calc_average'].call()
```

#### Self-Mutating Graphs with Governance
```glang
# Future: Graphs that safely modify their own structure
ecosystem = {
    __control__: {
        'max_nodes': 10000,
        'mutation_rate': 100,  # nodes per second
        'enforce_limits': func(operation) { ... }
    },
    methods: {
        'evolve': func() {
            # Add/remove species based on survival
            # All mutations go through __control__
        }
    },
    species: { ... }  # Mutable data region
}
```

#### Distributed Graph Systems
```glang
# Future: Graphs spanning multiple machines
distributed_system = {
    __control__: {
        'node_id': 'server_1',
        'peers': ['server_2', 'server_3'],
        'consensus': func(operation) { ... }
    },
    # Graph operations work transparently across network
}
```

### Ultimate Goal
Transform Glang from a programming language into a **platform for living, self-aware computational systems** that can:
- Understand their own structure
- Safely modify themselves
- Distribute across networks
- Govern their own evolution

This vision positions Glang as uniquely powerful for:
- Artificial Intelligence systems
- Smart contracts and blockchain
- Complex adaptive simulations
- Self-organizing distributed systems