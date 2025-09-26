# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Glang** is a general-purpose programming language with revolutionary graph-theoretic features, designed to be both practically useful and conceptually powerful.

### Current Vision (Updated January 2025)
Glang aims to be:
1. **A Practical Language First** - Complete with I/O, networking, databases, and standard library
2. **Then a Graph Language** - With true graph structures including nodes, edges, and traversal
3. **Finally a Revolutionary Platform** - Enabling self-aware, self-mutating, distributed graph systems

### Core Architecture
- **Graph-Based Function Discovery** - Functions stored as nodes, calls use graph traversal
- **Method-Based Collections** - Everything uses methods: `list.append()`, `string.upper()`, `num.abs()`
- **Optional Type System** - Types when needed, inference when obvious
- **Behavior-Driven Data** - Collections with automatic value transformations

### Design Principles:
- **Practical First** - Must be useful for real-world applications before adding advanced features
- **Graph-Theoretic Foundation** - All data will eventually be conceptualized as graphs with nodes and edges
- **Self-Aware Data Structures** - Future: Collections that understand their own structure and relationships
- **Intuitive Syntax** - Natural programming constructs that feel familiar
- **Developer Experience** - Excellent error messages, REPL environment, and reflection capabilities
- **KISS Principle** - Keep It Simple, Stupid! Glang despises unnecessary verbiage and redundant syntax

### Current Status (January 2025)
**BREAKTHROUGH COMPLETE**: Glang now has true graph-based function discovery system. Functions are stored as graph nodes and calls use graph traversal instead of variable lookup. This transforms Glang from a simulated graph language to a genuinely graph-theoretic programming platform.

## Current Development Focus

### Next Priorities
1. **Tree & Graph Data Structures** - Enable pure Glang DOM/XML processing
2. **Statistics Module** - Mathematical capabilities for data analysis
3. **Testing Framework** - Quality assurance beyond basic assertions
4. **Rust Migration Planning** - Parallel development strategy

### Recent Achievements
See [COMPLETED_MILESTONES.md](dev_docs/COMPLETED_MILESTONES.md) for detailed development history including:
- True graph-based function discovery system
- 80% self-hosting (network/HTML processing mostly pure Glang)
- SQL query builder module
- String processing enhancements

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
- `docs/` - **User-facing documentation ONLY** (language cheat sheet, module references)
- `dev_docs/` - Internal development documents: design, planning, architecture (NOT for users)

### Documentation Organization Rules
**IMPORTANT**: NEVER place documentation files (.md) in the root directory!
- Root directory: Only README.md and CLAUDE.md belong here
- User documentation: Always goes in `docs/`
- Development/design docs: Always goes in `dev_docs/`

## Development Setup

This project uses a Python virtual environment. Make sure it's activated before running commands:

```bash
# Activate virtual environment (if not already activated)
source .venv/bin/activate  # On Linux/Mac
# or
.venv\Scripts\activate     # On Windows

# Install glang package in development mode (required for running glang programs)
pip install -e ".[dev]"
```

## Development Commands

```bash
# Run the REPL (with full navigation: ↑/↓ history, tab completion)
python -m glang.repl
# or after installation:
glang

# Execute a .gr file directly
python -m glang.cli path/to/file.gr

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
echo -e "string greeting = \"Hello World\"\nlist items = [1, 2, 3]\ndata user = { \"name\": \"Alice\" }\nmap config = { \"host\": \"localhost\", \"port\": 8080 }\nitems.append(4)\nconfig[\"debug\"] = true\n/namespace\n/exit" | glang
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

# Binary trees (type inferred from usage or explicit constraints)
my_tree = tree{}                   # Infers tree type from literal
tree<num> numbers = tree{}         # Explicit constraint for homogeneous trees
tree<string> words = tree{}        # String-only binary search tree

# Data nodes (type inferred from literal)
user = { "name": "Alice" }           # Infers data type
data<num> score = { "final": 95 }    # Explicit constraint

# Maps (type inferred from literal)
config = { "host": "localhost", "port": 8080, "debug": true }  # Infers map type
map<string> settings = { "theme": "dark", "lang": "en" }       # Explicit constraint
```

### Advanced Operations
```glang
# Index access
items[0]           # Get first element from list
items[-1]          # Get last element (if supported)
config["host"]     # Get data node from map: { "host": "localhost" }

# Index assignment  
items[0] = 99      # Set first element in list
scores[1] = 100    # Update element
config["port"] = 9000  # Create/update data node in map

# Method calls
items.append(4)    # Add element to list
names.append("dave") # Type-checked append
config.get("host") # Get data node: { "host": "localhost" }
config.has_key("debug")  # Check if key exists: true/false
config.count_values("localhost")  # Count occurrences: 1

# Zero-argument methods: parentheses are optional
size = items.size()    # With parentheses  
size = items.size      # Without parentheses (property-like access)

# Data node operations
user.key()         # Get key: "name"
user.value()       # Get value: "Alice"

# List generator methods
nums = []
sequence = nums.generate(1, 10, 2)    # [1, 3, 5, 7, 9]
count = nums.upto(5)                  # [0, 1, 2, 3, 4, 5]
squares = nums.from_function(4, x => x * x)  # [0, 1, 4, 9]

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

### Enhanced String Methods - Unified Interface

Glang provides powerful string processing methods with a clean, unified interface that eliminates the need for regular expressions in most cases:

```glang
text = "Hello World 123! Contact support@example.com or call 555-1234"

# Basic string checks
text.starts_with("Hello")                   # Check if text starts with prefix: true
text.ends_with("1234")                      # Check if text ends with suffix: true

# Unified contains() method - single method with mode parameter
text.contains("any", "digits")              # Check if text contains any digits: true
text.contains("all", "letters", "digits")   # Check if text has both letters and digits: true  
text.contains("only", "letters", "spaces")  # Check if text contains only letters and spaces: false
text.contains("World")                       # Backward compatibility: substring search: true

# Unified extract() method - single method with pattern parameter
numbers = text.extract("numbers")           # Extract all numbers: ["123", "555", "1234"]
words = text.extract("words")               # Extract all words: ["Hello", "World", "Contact", ...]
emails = text.extract("emails")             # Extract email addresses: ["support@example.com"]

# Unified count() method - single method with pattern parameter  
digit_count = text.count("digits")          # Count digits in text: 10
word_count = text.count("words")            # Count words in text: 8
at_count = text.count_chars("@")            # Count specific characters: 1

# Find first occurrence
first_digit_pos = text.find_first("digits") # Position of first digit: 12
first_space_pos = text.find_first_char(" ") # Position of first space: 5

# Simple validation methods
email = "user@example.com"
email.is_email()                            # true - valid email format
"123.45".is_number()                        # true - valid number
"https://example.com".is_url()              # true - valid URL

# Enhanced splitting
mixed = "apple,banana;orange|grape"
fruits = mixed.split_on_any(",;|")          # ["apple", "banana", "orange", "grape"]
```

#### Pattern Types Supported
**Character Types:** `digits`/`numbers`, `letters`, `uppercase`, `lowercase`, `spaces`/`whitespace`, `punctuation`, `symbols`, `alphanumeric`  
**Content Types:** `words`, `emails` (for extraction)

#### Key Benefits
1. **No Regex Required:** Handle 90% of string processing without learning regular expressions
2. **Unified Interface:** Learn `method(mode, pattern...)` instead of dozens of method names  
3. **Semantic & Readable:** `text.contains("any", "digits")` is clearer than regex patterns
4. **Backward Compatible:** Old `contains("substring")` still works alongside new interface
5. **Extensible:** New patterns can be added without creating new methods

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

### Intrinsic Behavior System (NEW!)

Glang provides a powerful **intrinsic behavior system** where data structures (lists, maps, and future graphs) can have behaviors attached that automatically apply to all values during operations:

```glang
# Create a list with automatic nil handling
temperatures = [98.6, nil, 102.5]
temperatures.add_rule("nil_to_zero")        # Attach behavior to the list itself

# The nil is immediately transformed to 0
print(temperatures)                         # [98.6, 0, 102.5]

# All future nils are automatically transformed
temperatures.append(nil)                    # Becomes 0 automatically
print(temperatures)                         # [98.6, 0, 102.5, 0]

# Add range validation behavior
temperatures.add_rule("validate_range", 95, 105)  # Clamp to normal body temp

# Existing and new values are clamped
temperatures.append(110)                    # Automatically clamped to 105
print(temperatures)                         # [98.6, 0, 102.5, 0, 105]

# Symbol syntax (coming soon with parser support)
temperatures.add_rule(:positive)            # Cleaner syntax with symbols

# Query and manage behaviors
temperatures.has_rule("nil_to_zero")        # true
temperatures.get_rules()                     # ["nil_to_zero", "positive", "validate_range"]
temperatures.remove_rule("positive")        # Remove specific behavior
temperatures.clear_rules()                   # Remove all behaviors

# Generic mapping behaviors - map any value to any other value
color_map = { "red": 1, "green": 2, "blue": 3 }
colors = ["red", "blue", "purple", "green"]
colors.add_mapping_rule(color_map)          # Maps colors using the hash graph
print(colors)                                # [1, 3, "purple", 2]

# Mapping with default value for unmapped keys
colors.add_mapping_rule(color_map, 0)       # Unmapped colors become 0
print(colors)                                # [1, 3, 0, 2]

# Employee to department mapping
dept_map = { "walter": "HR", "james": "IT", "emily": "Admin" }
employees = ["walter", "unknown", "james"]
employees.add_mapping_rule(dept_map, "Unknown")  # Default for unmapped names
print(employees)                             # ["HR", "Unknown", "IT"]
```

#### Map Behaviors
```glang
# Behaviors work on maps too
config = { "timeout": nil, "retries": -5, "port": 9999 }
config.add_rule("nil_to_zero")              # nil values become 0
config.add_rule("positive")                 # negative values become positive
config.add_rule("validate_range", 1, 9000)  # clamp port numbers

print(config["timeout"])                    # 0 (was nil)
print(config["retries"])                    # 5 (was -5)
print(config["port"])                        # 9000 (was 9999, clamped)

# New values are automatically processed
config["max_connections"] = nil             # Becomes 0
config["min_threads"] = -10                 # Becomes 10 (positive), then clamped to 10
```

#### Standard Behaviors
- **none_to_zero** - Convert none to 0
- **none_to_empty** - Convert none to empty string
- **validate_range(min, max)** - Clamp numbers to range
- **map_colors** - Map color names to numbers (deprecated - use add_mapping_rule instead)
- **uppercase/lowercase** - String case conversion
- **round_to_int** - Round numbers to integers
- **positive** - Ensure numbers are positive

#### Generic Mapping (NEW!)
Create custom value mappings using hash graphs - replaces hardcoded behaviors like `map_colors`:

```glang
# Define your own mappings as hash graphs
status_map = { "active": 1, "inactive": 0, "pending": 2 }
user_statuses = ["active", "unknown", "inactive"]
user_statuses.add_mapping_rule(status_map, -1)  # Default -1 for unmapped
print(user_statuses)                             # [1, -1, 0]

# Chain mappings for multi-stage transformations
codes = ["a", "b", "c"]
first_map = { "a": "alpha", "b": "beta", "c": "gamma" }
second_map = { "alpha": 1, "beta": 2, "gamma": 3 }
codes.add_mapping_rule(first_map)
codes.add_mapping_rule(second_map)
print(codes)                                     # [1, 2, 3]
```

#### Key Benefits
- **Intrinsic to Data**: Behaviors are part of the container, not external processors
- **Automatic Application**: Once attached, behaviors apply to all current and future values
- **One-Step Process**: `list.add_rule("none_to_zero")` - that's it!
- **Type-Safe**: Behaviors respect and work with type constraints
- **Composable**: Multiple behaviors apply in order
- **Graph Foundation**: Since lists and maps are graph structures, behaviors are inherited by all graph types
- **User-Defined Mappings**: Create any mapping with `add_mapping_rule()` using hash graphs

#### Future: Custom Behaviors
```glang
# Future syntax for custom behaviors (not yet implemented)
func normalize_temperature(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

temperatures.add_rule(:normalize_temperature)  # Use custom function as behavior
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

# Logical operators (NEW!)
if a and b {          # Both must be true
    # execute when both true
}

if a or b {           # Either can be true
    # execute when at least one true
}

# Operator synonyms also supported
if a && b {           # Same as 'and'
    # && is synonym for 'and'
}

if a || b {           # Same as 'or'
    # || is synonym for 'or'
}

# Truthiness rules for logical operators:
# - Booleans: true/false
# - Numbers: 0 is false, non-zero is true
# - Strings: empty string is false, non-empty is true
# - Lists: empty list is false, non-empty is true
# - Maps: empty map is false, non-empty is true

# KNOWN LIMITATION: Complex chained expressions need parentheses
# This will be fixed in the next language update
if (a == 1) or (b == 2) {     # Recommended: use parentheses for now
    # Complex expressions need explicit grouping
}

# Precision context blocks - Decimal Places Control (NEW!)
precision 0 {
    # Integer arithmetic - no decimal points
    pi = 3.14159265358979323846  # Result: 3 (integer)
    area = pi * 10 * 10          # Result: 300 (integer)
}

precision 2 {
    # Financial calculations with 2 decimal places  
    price = 19.99
    tax = price * 0.085          # Result: 1.70 (exactly 2 decimal places)
    total = price + tax          # Result: 21.69 (exactly 2 decimal places)
}

precision 5 {
    # Scientific calculations with 5 decimal places
    pi = 3.14159265358979323846  # Result: 3.14159 (5 decimal places)
    circumference = 2 * pi * 10  # Result: 62.83180 (5 decimal places)
}

# Nested precision contexts
precision 3 {
    outer_value = 22.0 / 7.0     # Result: 3.143 (3 decimal places)

    precision 1 {
        inner_value = 22.0 / 7.0 # Result: 3.1 (1 decimal place)
    }

    back_value = 22.0 / 7.0      # Result: 3.143 (3 decimal places restored)
}

# Configuration blocks - Behavior Control (NEW!)
# File-level configuration (applies to entire file)
configure { skip_none: false, decimal_places: 2 }

# Block-level configuration with explicit behavior control
configure { skip_none: true } {
    # All operations in this block skip none values
    data = [1, 2, none, 4]
    result = data.mean()         # Result: 2.33 (none skipped)
}

configure { strict_types: true } {
    # No implicit type conversions allowed
    # result = "5" + 3           # Error: cannot add string and number
}

# Multiple configuration settings
configure {
    skip_none: false,            # Error on none values
    decimal_places: 2,           # Exactly 2 decimal places
    strict_types: true           # No implicit conversions
} {
    # All operations use these explicit behaviors
    financial_calculation = 19.99 * 0.085  # Result: 1.70 (exactly)
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

### Time Module
```glang
# Import the time module
import "time"

# Create time values
current = time.now()                            # Current time
today = time.today()                           # Start of today (00:00:00 UTC)
birthday = time.from_components(1990, 12, 25) # Date only (midnight UTC)
meeting = time.from_components(2025, 1, 15, 14, 30, 0) # Full date and time
parsed = time.from_string("2025-01-15T14:30:00") # Parse ISO format

# Work with time values
print("Current: " + current.to_string())      # ISO format: "2025-01-15T14:30:00Z"  
print("Type: " + current.get_type())          # "time"

# Method calls work with or without parentheses (for zero-argument methods)
iso_format = current.to_string()              # With parentheses
iso_format = current.to_string                # Without parentheses (more elegant)
type_name = current.get_type                  # Property-like access

# Type casting - time values can be cast to/from numbers and strings
timestamp = current.to_num()                  # Convert to Unix timestamp (number)
time_from_num = timestamp.to_time()           # Convert number back to time
time_from_str = "2025-01-15T14:30:00".to_time() # Parse string to time

# All casting maintains round-trip consistency
original_str = current.to_string()
round_trip = current.to_num().to_time().to_string()
print("Consistent: " + (original_str == round_trip).to_string()) # "true"
```

### File Operations

#### File Loading
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

#### File Handle I/O
```glang
import "io"

# Read capabilities - auto-close on EOF
read_handle = io.open("data.txt", "r")
content = read_handle.read()        # Reads all content, auto-closes
# read_handle.read()                # Error: capability exhausted

# Write capabilities - manual control  
write_handle = io.open("output.txt", "w")
write_handle.write("Line 1\n")
write_handle.write("Line 2\n") 
write_handle.close()                # Must manually close

# Incremental processing
input = io.open("large_file.txt", "r")
output = io.open("processed.txt", "w")

while true {
    line = input.read_line()
    if line == "" {                 # EOF reached, auto-closed
        break
    }
    output.write(line.upper() + "\n")
}
output.close()
```

### Call Graph Debugging and Introspection

Glang provides powerful call graph introspection that lets you visualize and debug function relationships in real-time:

```glang
# Import the call graph module
import "call_graph"

# Basic information
scope = call_graph.current_scope()               # Current scope name
count = call_graph.count_functions()             # Total functions
scopes = call_graph.list_scopes()                # All available scopes
funcs = call_graph.get_reachable_functions()     # Functions you can call

# Detailed function information
info = call_graph.get_function_info("my_func")
print("Parameters: " + info["parameters"].to_string())
print("Connected to: " + info["connected_functions"].to_string())

# Path finding between functions
path = call_graph.find_path("main", "helper")
if path != none {
    print("Path: " + path.to_string())   # Shows function connectivity
}

# Visualization in multiple formats
text_viz = call_graph.visualize("text")          # Human-readable text
dot_viz = call_graph.visualize("dot")            # Graphviz DOT format
mermaid_viz = call_graph.visualize("mermaid")    # Mermaid diagram syntax

# Focus on specific scope
module_viz = call_graph.visualize_scope("MyModule")
```

**REPL Debugging Example:**
```glang
glang> func main() { process() }
glang> func process() { validate() }
glang> func validate() { return true }

glang> import "call_graph"
glang> call_graph.get_reachable_functions()
[main, process, validate]

glang> path = call_graph.find_path("main", "validate")
glang> path.to_string()
[main, validate]

glang> call_graph.visualize()
==================================================
COMPLETE CALL GRAPH
==================================================

[global]
  main
    → process
    → validate
  process
    → main
    → validate
  validate
    → main
    → process
```

The call graph system gives you unprecedented visibility into your program's structure, making debugging complex function relationships much easier. See [Call Graph Debugging Guide](docs/call_graph_debugging.md) for complete documentation.

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

### Implementation Status
**Core Language**: Complete - Functions, types, collections, control flow, file I/O
**Standard Library**: Extensive - Math, JSON, Time, HTML/Network processing (80% pure Glang)
**Graph Architecture**: Breakthrough complete - True graph-based function discovery
**Quality**: 1345+ tests passing, 66% coverage
**Self-Hosting**: 80% pure Glang (only network I/O still requires Python)

### Development Guidelines
- **AST-first development** - All new features should extend the AST system
- **Smart type inference** - Infer types from context to reduce boilerplate
- **Comprehensive testing** - New features require full test coverage
- **Clean error messages** - Users should understand exactly what went wrong
- **Clean import syntax** - NEVER use `import "module" as alias` - modules have built-in aliases that make this redundant

### Import Philosophy

**CRITICAL**: Glang follows the KISS principle religiously. The syntax `import "some_module" as "anything"` is almost always a bad choice because:

1. **Modules have built-in aliases** - `import "random"` gives you both `random` and `rand`
2. **Unnecessary verbiage** - Why write `import "math" as calc` when `import "math"` gives you `calc` automatically?
3. **Violates KISS** - Glang despises redundant syntax that adds no value

**Good Examples:**
```glang
import "random"      # Use rand.choice() or random.choice() - your choice!
import "math"        # Use calc.sqrt() or math.sqrt() - both work!
import "regex"       # Use re.match() or regex.match() - flexible!
```

**Bad Examples (avoid these patterns):**
```glang
import "random" as rand    # REDUNDANT - rand is already available!
import "math" as calc      # REDUNDANT - calc is already available!
import "regex" as re       # REDUNDANT - re is already available!
```

**When aliases ARE appropriate:**
- Only when overriding with something MORE descriptive: `import "utils.gr" as tools`
- Never for standard library modules - they already have perfect aliases

## Future Vision

**For detailed development roadmap, see [`dev_docs/PRIMARY_ROADMAP.md`](dev_docs/PRIMARY_ROADMAP.md)**

Transform Glang from a practical programming language into a **platform for living, self-aware computational systems** that can:
- Understand their own structure through graph introspection
- Safely modify themselves with governance rules
- Distribute transparently across networks
- Evolve and adapt to changing requirements

