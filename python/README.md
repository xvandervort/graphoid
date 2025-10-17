# Glang

> **A practical programming language with revolutionary graph-theoretic features**

## Overview

Glang is a general-purpose programming language designed to be both practically useful and conceptually powerful. While it currently implements traditional container-based data structures (lists, hashes, data nodes), it is architected with a revolutionary vision: to become the first language where **true graph structures are first-class citizens**, enabling self-aware, self-modifying computational systems.

### Current Features (January 2025)
- 🔧 **Complete Function System** - Functions, lambdas, closures, and recursion
- ⚡ **Elegant Pattern Matching** - Implicit pattern functions with automatic fallthrough
- 🎯 **Strong Type System** - With optional type inference and constraints
- 📦 **Modern Collections** - Lists, hashes, and data nodes with method-based operations
- 📁 **File Loading System** - Modular programming with `.gr` files
- 🏗️ **Clean AST Architecture** - Reliable execution with excellent error messages
- 🖥️ **Interactive REPL** - Full command history and introspection capabilities

### Future Vision: Revolutionary Graph Computing

Glang is on a multi-year journey to become the first language with **true graph-computing capabilities**:
- **Graph Foundation** (2025): Nodes + edges with traversal algorithms
- **Self-Aware Data Structures** (2025): Collections that understand their own composition
- **Controlled Self-Mutation** (2025): Safe, governed self-modifying systems  
- **Distributed Graph Systems** (2026): Multi-machine graph distribution

*For the complete development plan, see [PRIMARY_ROADMAP.md](./PRIMARY_ROADMAP.md)*

## Installation

### Prerequisites
- Python 3.8 or higher

### Setup

```bash
# Clone the repository
git clone <repository-url> (There isn't one yet)
cd glang

# Create and activate virtual environment
python -m venv .venv
source .venv/bin/activate  # On Linux/Mac
# or
.venv\Scripts\activate     # On Windows

# Install in development mode with all dependencies
pip install -e ".[dev]"
```

### Activating the Environment Later

```bash
# Navigate to project directory
cd grang

# Activate virtual environment
source .venv/bin/activate  # On Linux/Mac
# or  
.venv\Scripts\activate     # On Windows
```

## Usage

### Running the REPL

```bash
# Using the installed command
glang

# Or run as a Python module
python -m glang.repl
```

### Language Examples

```glang
# Variable declarations with type inference
name = "Alice"                    # Infers string type
age = 25                         # Infers num type  
items = [1, 2, 3]               # Infers list type

# Type-constrained collections
list<num> scores = [95, 87, 92]
hash<string> config = {"theme": "dark", "lang": "en"}

# Functions and lambdas
func greet(name) {
    return "Hello, " + name + "!"
}
double = x => x * 2

# Elegant pattern matching functions (NEW!)
func factorial(n) {
    0 => 1
    1 => 1
    x => x * factorial(x - 1)
}

func classify_value(value) {
    42 => "The answer"
    "hello" => "A greeting"
    true => "Boolean true"
    x => "Other: " + x.to_string()
}

# Method-based operations
items.append(4)                  # [1, 2, 3, 4]
items.map("double")             # [2, 4, 6, 8]
config.get("theme")             # "dark"

# Control flow
if age >= 18 {
    status = "adult"
} else {
    status = "minor"
}

# File loading
load "config.gr"                # Load variables from other files
```

### REPL Commands

- `/help` - Show complete language reference
- `/namespace` - Show all current variables
- `/load <file>` - Load and execute a .gr file
- `/save <file>` - Save current session
- `/methods <var>` - Show methods for a variable
- `/type <var>` - Show type information
- `/exit` - Exit the REPL

## Documentation

- **[Language Cheat Sheet](docs/GLANG_CHEAT_SHEET.md)** - Quick syntax reference
- **[Functions Guide](docs/language_features/functions.md)** - Comprehensive function documentation
- **[Pattern Matching Guide](docs/language_features/pattern_matching.md)** - In-depth pattern matching
- **[Language Features](docs/language_features/)** - Advanced features documentation
- **[Why Glang?](docs/WHY_GLANG.md)** - Design philosophy and innovations

## Development

**Note:** Make sure your virtual environment is activated before running development commands.

### Running Tests

```bash
pytest test/
```

### Code Formatting

```bash
black src/ test/
```

### Type Checking

```bash
mypy src/
```

## Example REPL Session

```bash
$ glang
Glang v0.8.0
Modern AST-based programming language with graph-theoretic features
Type '/help' for available commands or '/exit' to quit.
✨ Try: string name = "Alice" then 'name' to see the magic! ✨

glang> string greeting = "Hello World"
glang> list<num> scores = [95, 87, 92]
glang> greeting
Hello World
glang> scores.append(88)
glang> scores.map("double")
[190, 174, 184, 176]
glang> func factorial(n) { 0 => 1; 1 => 1; x => x * factorial(x - 1) }
func factorial(n) { ... }
glang> factorial(5)
120
glang> /namespace
=== Variable Namespace ===
  [string] greeting → Hello World
  [list<num>] scores → [95, 87, 92, 88]
glang> /methods scores
=== Methods for 'scores' (list<num>) ===
  append, prepend, insert, remove, contains, size, length, sort, map, filter, each
glang> /exit
Goodbye!
```

## Project Status

### ✅ Current State (January 2025)
- [x] **Complete Function System** - Functions, lambdas, closures, recursion
- [x] **Elegant Pattern Matching** - Implicit pattern functions with automatic fallthrough
- [x] **Strong Type System** - Optional type inference and constraints
- [x] **Modern Collections** - Lists, hashes, data nodes with method operations
- [x] **File Loading System** - Modular programming with .gr files
- [x] **AST-based Execution** - Clean architecture with excellent error messages
- [x] **Standard Library Foundation** - Math and JSON modules implemented
- [x] **Comprehensive Testing** - 620+ tests with growing coverage
- [x] **Interactive REPL** - Full command history and introspection

### 🎯 Immediate Priorities (Q1-Q2 2025)
**Phase 1: Production Readiness**
- [ ] Complete I/O operations (file, network, console)
- [ ] String manipulation utilities and regular expressions
- [ ] Performance benchmarking and optimization
- [ ] Comprehensive error messages with stack traces
- [ ] IDE integration and package manager design

For the complete development roadmap through 2026, including the revolutionary graph computing features, see **[PRIMARY_ROADMAP.md](./PRIMARY_ROADMAP.md)**.

## Contributing

Glang is currently focused on achieving **practical language goals** before implementing revolutionary graph features. When contributing:

- **Practical First**: Ensure features work for real-world applications
- **Graph-Ready Architecture**: Design with future graph transformation in mind
- **Comprehensive Testing**: All features require full test coverage
- **Clean AST Design**: Follow the established AST visitor pattern

See **[CLAUDE.md](./CLAUDE.md)** for detailed development guidelines and **[PRIMARY_ROADMAP.md](./PRIMARY_ROADMAP.md)** for the complete development plan.

## License

MIT
