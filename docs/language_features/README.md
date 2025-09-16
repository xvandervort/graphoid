# Glang Language Features

This directory contains comprehensive documentation for Glang's advanced language features that go beyond basic syntax and built-in methods.

## Available Features

### [Precision Context Blocks](precision_blocks.md)
Revolutionary language-level numeric precision control that allows you to specify exact decimal places for calculations within a specific scope.

- **Decimal places precision** (not significant digits)
- **Integer mode** with `precision 0`
- **Nested contexts** with automatic restoration
- **Memory-efficient implementation**
- **Financial and scientific calculation examples**

### [Configuration Blocks](configuration_blocks.md) ✅ **NEW**
Language-level behavior configuration system that provides explicit control over default behaviors and settings at different scope levels.

- **Explicit behavior control** (no hidden magic)
- **File-level and block-level scoping**
- **None handling configuration** (skip, convert, or error)
- **Type strictness control** (implicit conversions)
- **Domain-specific configuration** (data science, finance, systems)
- **Hierarchical inheritance** with clear override rules

## Planned Features

Future language features will be documented here as they are implemented:

- **Graph traversal blocks** - For navigating graph structures with specialized syntax
- **Immutability contexts** - Language-level frozen/unfrozen data management
- **Type constraint blocks** - Temporary type restrictions within scopes
- **Performance optimization blocks** - Compiler hints for specific code sections

---

*For basic syntax and built-in methods, see the main [docs](../) directory.*