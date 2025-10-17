# Glang Standard Library

This directory contains the standard library modules for the Glang programming language.

## Available Modules

### `math.gr` - Mathematical Constants and Utilities

Provides essential mathematical constants for calculations.

**Constants:**
- `pi` - Pi (3.141592653589793)
- `e` - Euler's number (2.718281828459045)
- `PI` - Alias for pi
- `E` - Alias for e

**Usage:**
```glang
load "stdlib/math.gr"

# Calculate circle area
radius = 5
area = pi * radius.pow(2)

# Natural logarithm base
ln_2 = 2.log()  # log base e
```

## Adding New Standard Library Modules

When adding new standard library modules:

1. Place them in the `stdlib/` directory
2. Use descriptive module names (e.g., `string_utils.gr`, `collections.gr`)
3. Include proper module declaration: `module module_name`
4. Document the module's purpose and API
5. Add comprehensive examples
6. Update this README

## Design Principles

Standard library modules should:
- Provide commonly needed functionality
- Have clear, intuitive APIs
- Include comprehensive documentation
- Be well-tested
- Follow Glang coding conventions
- Avoid external dependencies when possible