# Glang

A prototype programming language that uses graphs as first-class objects.

## Overview

Glang is designed with the following principles:
- **Intuitive structure and syntax** - Making graph-based programming accessible
- **Stability** - Reliable and consistent behavior
- **Flexibility** - Adaptable to various graph computation needs

This is the initial prototype implementation in Python, focusing on establishing the core REPL infrastructure.

## Installation

### Prerequisites
- Python 3.8 or higher

### Setup

```bash
# Clone the repository
git clone <repository-url>
cd grang

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

### Available REPL Commands

- `ver` or `version` - Show version information
- `h` or `help` - Show help information  
- `x` or `exit` - Exit the REPL

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

## Project Status

This is a very early prototype focusing on:
- [x] Basic REPL infrastructure
- [x] Core command system (version, help, exit)
- [ ] Graph data structures
- [ ] Graph syntax parsing
- [ ] Graph operations and evaluation

## License

MIT