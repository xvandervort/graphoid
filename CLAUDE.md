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

## Recent Changes (September 2025)

### 🔥 CRITICAL DISCOVERY: Foundational Architecture Issue (September 2025)
**THE MOST IMPORTANT DISCOVERY IN GLANG'S DEVELOPMENT**

- **🚨 CRITICAL BUG IDENTIFIED**: Glang uses variable-based function lookup instead of graph traversal
- **DEVASTATING IMPACT**: **Glang is not truly graph-based** - it's simulating graph features, not implementing them
- **SPECIFIC SYMPTOM**: Functions cannot call other functions within the same module (discovered during conversions library development)
- **ROOT CAUSE**: Functions stored as "variables" in flat dictionaries, not as graph nodes with connectivity
- **THEORETICAL IMPLICATIONS**: Without graph-based function discovery, Glang isn't Glang - it's just pretending to be graph-based
- **PRIORITY**: **🔥 ABSOLUTE HIGHEST PRIORITY** - blocks all other development until fixed
- **SOLUTION**: Implement true graph-based function discovery system using existing `GraphStructure` infrastructure
- **TRANSFORMATION**: Will change Glang from "fake graph language" to "revolutionary true graph language"

**📋 COMPLETE IMPLEMENTATION PLAN**: See [`dev_docs/FOUNDATIONAL_PRIORITY_CALL_GRAPH.md`](dev_docs/FOUNDATIONAL_PRIORITY_CALL_GRAPH.md)
**📊 ROADMAP IMPACT**: See updated [`dev_docs/PRIMARY_ROADMAP.md`](dev_docs/PRIMARY_ROADMAP.md) - all features blocked until this is complete

**This discovery fundamentally changes Glang's development priorities and reveals that the current implementation doesn't match the graph-theoretic vision.**

---

### ✅ Logical Operators & Parser Issues Complete
- **COMPLETE**: Added `and`/`or` logical operators with operator synonyms `&&`/`||`
- **Features**: Proper truthiness rules, short-circuit evaluation, correct precedence parsing
- **Syntax**: `if a and b { ... }` or `if a && b { ... }`
- **Multi-line Support**: Complex expressions with line breaks now work correctly
- **✅ ALL CRITICAL PARSER ISSUES RESOLVED (September 2025)**:
  - **Logical Operator Precedence**: Fixed - expressions like `a == 1 or b == 2` now parse correctly
  - **Hash Variable Key Access**: Fixed - `hash[variable_key]` syntax now works
  - **Variable Scoping**: Fixed - proper lexical scoping allows variable reuse in different scopes

### ✅ Bitcoin Price Tracker Success
- **Demonstrated**: Real-world web scraping capabilities using HTTP requests and string parsing
- **Achievement**: Successfully extracts current Bitcoin price ($115,411.29) from CoinMarketCap
- **Identified Needs**: HTML parsing library for standard library, parser precedence fixes

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

# Zero-argument methods: parentheses are optional
size = items.size()    # With parentheses  
size = items.size      # Without parentheses (property-like access)

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

Glang provides a powerful **intrinsic behavior system** where data structures (lists, hashes, and future graphs) can have behaviors attached that automatically apply to all values during operations:

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
```

#### Hash Behaviors
```glang
# Behaviors work on hashes too
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
- **nil_to_zero** - Convert nil/none to 0
- **nil_to_empty** - Convert nil/none to empty string
- **validate_range(min, max)** - Clamp numbers to range
- **map_colors** - Map color names to numbers
- **uppercase/lowercase** - String case conversion
- **round_to_int** - Round numbers to integers
- **positive** - Ensure numbers are positive

#### Key Benefits
- **Intrinsic to Data**: Behaviors are part of the container, not external processors
- **Automatic Application**: Once attached, behaviors apply to all current and future values
- **One-Step Process**: `list.add_rule("nil_to_zero")` - that's it!
- **Type-Safe**: Behaviors respect and work with type constraints
- **Composable**: Multiple behaviors apply in order
- **Graph Foundation**: Since lists and hashes are graph structures, behaviors are inherited by all graph types

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
# - Hashes: empty hash is false, non-empty is true

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
import "time" as Time

# Create time values
current = Time.now()                            # Current time
today = Time.today()                           # Start of today (00:00:00 UTC)
birthday = Time.from_components(1990, 12, 25) # Date only (midnight UTC)
meeting = Time.from_components(2025, 1, 15, 14, 30, 0) # Full date and time
parsed = Time.from_string("2025-01-15T14:30:00") # Parse ISO format

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
import "io" as io

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
- ✅ **Logical operators** with operator synonyms: `and`/`&&` and `or`/`||` with proper truthiness and short-circuiting
- ✅ **Precision context blocks** with language-level numeric precision control (precision N { ... })
- ✅ **Configuration blocks** with explicit behavior control and scoped settings (configure { ... } { ... })
- ✅ CLI program execution with shebang support and command-line arguments
- ✅ Mathematical methods (abs, sqrt, log, pow, rounding) for numbers
- ✅ Type casting system (to_string, to_num, to_bool) for all basic types
- ✅ **File handle I/O** with boundary capability semantics (auto-close on EOF for reads, manual control for writes)
- ✅ Standard library foundation with math constants module (stdlib/math.gr)
- ✅ **JSON module** with encode, decode, pretty printing, and validation (json.encode, json.decode, json.is_valid)
- ✅ **Time module** with single Time type, UTC timestamps, and full type casting (Time.now, Time.from_components, time.to_num, string.to_time)
- ✅ **Intrinsic behavior system** where lists/hashes have built-in behaviors that auto-apply to all values (GraphContainer mixin)
- ✅ **Comprehensive test suite** (1205+ tests, 67% coverage)

### Development Guidelines
- **AST-first development** - All new features should extend the AST system
- **Smart type inference** - Infer types from context to reduce boilerplate
- **Comprehensive testing** - New features require full test coverage  
- **Clean error messages** - Users should understand exactly what went wrong

## Future Vision: The Path to Revolutionary Graph Computing

**For detailed roadmap and development phases, see [`dev_docs/PRIMARY_ROADMAP.md`](dev_docs/PRIMARY_ROADMAP.md)**

### Near-Term Priorities (Q1-Q2 2025)

**COMPLETED PRIORITIES:**
- **✅ Fixed Logical Operator Precedence**: Successfully implemented proper parser precedence with `parse_logical_or` → `parse_logical_and` → `parse_comparison`. Complex expressions like `a == 1 or b == 2 and c > 3` now parse correctly without requiring parentheses. Also added short-circuit evaluation for performance.
- **✅ Hash Variable Key Access**: The `hash[variable_key]` syntax now works correctly (was previously reporting "Key must be a string literal" error).
- **✅ Variable Scoping**: Fixed global variable conflict issue. Variables can now be reused in different loop scopes without "Variable already declared" errors. Implemented proper scoped symbol table with `enter_scope()`/`exit_scope()`.
- **✅ Line Continuation**: Fixed multi-line logical expression parsing. Complex expressions like `a or b or\n   c or d` now parse correctly by skipping newlines within expressions.

**ALL CRITICAL PARSER ISSUES RESOLVED** 🎉
The Bitcoin tracker and cryptocurrency analytics experiments that identified these issues should now work without workarounds.

**Test Suite Status**: 1205/1205 tests passing (100% success rate) ✅
- All tests now pass including the complex pattern matching edge case with variable shadowing
- All critical parsing issues from Bitcoin analytics experiments are fully resolved

**Make Glang Practical** - Standard libraries for real-world use:
- **✅ I/O Library**: File operations, file handle I/O with auto-close semantics, user input, directory management
- **✅ Time Library**: Single Time type with UTC timestamps and full type casting
- **✅ Logical Operators**: `and`/`&&` and `or`/`||` with proper truthiness and short-circuiting (basic support complete, precedence needs fix)
- **✅ Behavior System**: Composable transformations for custom node types (Python API complete)
- **⏳ Network Library**: ✅ JSON support, HTTP client, email notifications
- **⏳ Database Connectivity**: SQLite, PostgreSQL, MySQL support
- **⏳ System Library**: OS interaction, processes
- **⏳ HTML Parsing Library**: Standard library for web scraping and HTML processing (identified during Bitcoin tracker development)
- **⏳ Native Behavior Syntax**: `value: type with [behaviors...]` language integration

### Medium-Term Goals (Q2-Q3 2025)
**Build True Graph Foundation** - Transform containers into real graphs:
- **Native Behavior Integration**: `with [behaviors...]` syntax in AST/parser/execution
- **Data Graphs for Statistics**: DataFrames as graph structures with attached behaviors
- **Enhanced Binary Data Processing**: Hexadecimal literals, fixed-size lists, format detection, image processing
- **Graph Architecture**: Nodes + edges with metadata, not just containers
- **Node Awareness**: Nodes know their container and can access siblings
- **Graph Traversal**: Real pathfinding, connectivity analysis
- **Anonymous Functions**: Function references with `.call()` method
- **Behavior-Aware Graphs**: True graphs with behaviors attached to specific nodes

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