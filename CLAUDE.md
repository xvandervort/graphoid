# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Glang** is a modern programming language with powerful type inference, built on a clean AST-based architecture. The language provides:

### Core Features
- **Strong type system** with optional type inference
- **Modern syntax** for variables, lists, and method calls
- **File loading system** for modular programming
- **Clean AST-based execution** for reliability and extensibility

### Design Principles:
- **Intuitive syntax** - Natural programming constructs that feel familiar
- **Type safety** - Comprehensive type checking with helpful error messages  
- **Extensibility** - Clean AST architecture for future language features
- **Developer experience** - Excellent error messages and REPL environment

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
- `test/` - Comprehensive test suite (194 tests, 71% coverage)
- `doc/` - Documentation files

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
# Explicit type declarations
string name = "Alice"
num age = 25
bool active = true
list items = [1, 2, 3]

# Type-constrained lists
list<num> scores = [95, 87, 92]
list<string> names = ["alice", "bob", "charlie"]
list<bool> flags = [true, false, true]

# Data nodes (key-value pairs)
data user = { "name": "Alice" }
data<num> score = { "final": 95 }

# Maps (collections of data nodes)
map config = { "host": "localhost", "port": 8080, "debug": true }
map<string> settings = { "theme": "dark", "lang": "en" }

# Type inference (new!)
name = "Bob"           # Infers string type
count = 42             # Infers num type  
data = [1, 2, 3]       # Infers list type
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

# Data node operations
user.key()         # Get key: "name"
user.value()       # Get value: "Alice"
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
- **Type Safety First**: Every operation is type-checked at parse time
- **Clear Error Messages**: Comprehensive error reporting with source positions
- **Extensible Design**: Clean visitor pattern allows easy language extensions
- **Testing Focus**: 194 tests with 71% coverage ensure reliability

### Current Implementation Status  
- ✅ Modern lexer and AST parser
- ✅ Complete semantic analysis with symbol tables
- ✅ Type-safe execution engine
- ✅ File loading system with .gr format
- ✅ Type inference for variable declarations
- ✅ Method calls with type constraint enforcement
- ✅ Index access and assignment for lists, strings, and maps
- ✅ Data nodes with key-value semantics and type constraints
- ✅ Maps as collections of data nodes with Ruby hash-like syntax
- ✅ Comprehensive test suite (230+ tests, 39% coverage)

### Development Guidelines
- **AST-first development** - All new features should extend the AST system
- **Type safety everywhere** - Every operation must be type-checked
- **Comprehensive testing** - New features require full test coverage  
- **Clean error messages** - Users should understand exactly what went wrong