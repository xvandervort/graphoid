# Development Tools

This directory contains development utilities for the Glang project.

## Files

### `self_hosting_metrics.gr`
**Pure Glang implementation** - Analyzes how much of Glang is written in Glang vs Python.

**Usage:**
```bash
# From command line
glang tools/self_hosting_metrics.gr

# From REPL
/load tools/self_hosting_metrics.gr
```

**Metrics calculated:**
- **LCR (Lines of Code Ratio)**: Python vs Glang source lines
- **MIR (Module Implementation Ratio)**: How modules are implemented
- **FSR (Functionality Self-sufficiency Ratio)**: Feature coverage

### `self_hosting_metrics.py`
**Python reference implementation** - Original version used for comparison and validation.

**Usage:**
```bash
python tools/self_hosting_metrics.py
```

## Development Notes

The Glang version (`self_hosting_metrics.gr`) demonstrates several advanced language features:
- Recursive directory traversal
- File analysis and classification
- Result-style error handling
- String processing and pattern matching
- Complex mathematical calculations

This tool helps track progress toward Glang's goal of becoming fully self-hosting.