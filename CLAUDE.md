# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Glang** is a prototype programming language that uses graphs as first-class objects. The project emphasizes:
- Intuitive structure and syntax
- Stability
- Flexibility

This prototype is implemented in Python for rapid development and testing.

## Repository Structure

- `src/` - Core glang implementation
  - `src/glang/` - Main language package
  - `src/glang/repl/` - REPL implementation
- `test/` - Test files
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
# Run the REPL
python -m glang.repl
# or after installation:
glang

# Run tests
pytest test/

# Install in development mode with all dependencies
pip install -e ".[dev]"

# Code formatting
black src/ test/

# Type checking
mypy src/
```

## REPL Commands

The REPL supports these initial commands:
- `ver` or `version` - Show version information
- `h` or `help` - Show help information
- `x` or `exit` - Exit the REPL

## Architecture Notes

The current phase focuses on establishing a basic REPL infrastructure that will serve as the foundation for implementing graph-based language features.