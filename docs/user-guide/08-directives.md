# Chapter 8: Directives

Directives control how Graphoid interprets and executes your code. They let you fine-tune numeric behavior, type checking, and performance characteristics.

## What are Directives?

Directives are configuration settings that change how code behaves. They're specified in `configure` blocks using symbols (words starting with `:`).

```graphoid
configure { :unsigned, :high } {
    # Code here runs with unsigned arithmetic and high precision
    x = 255
    y = 1
    result = x + y  # 256 (unsigned)
}
```

## The configure Block

### Basic Syntax

```graphoid
configure { :directive1, :directive2 } {
    # Code with directives enabled
}
# Code here runs with default settings
```

### Nested configure Blocks

```graphoid
configure { :unsigned } {
    x = 255

    configure { :32bit } {
        y = x + 1  # 32-bit unsigned arithmetic
    }

    z = x + 1  # Regular unsigned arithmetic
}
```

## Numeric Mode Directives

### :integer - Integer Mode

Forces all arithmetic to use integer operations:

```graphoid
# Default behavior (floating point)
x = 10 / 3
print(x)  # 3.333...

# With :integer directive
configure { :integer } {
    x = 10 / 3
    print(x)  # 3 (truncated to integer)
}
```

**When to use**:
- Working with discrete quantities (counting, indexing)
- Avoiding floating-point rounding errors
- Performance-critical integer math

### :unsigned - Unsigned Arithmetic

Treats numbers as unsigned (non-negative):

```graphoid
configure { :unsigned } {
    x = 255
    y = 1
    result = x + y
    print(result)  # 256

    # Right shift is logical (not arithmetic)
    shifted = 128 >> 1
    print(shifted)  # 64
}
```

**When to use**:
- Binary data processing
- Bit manipulation
- Working with unsigned protocols

### :32bit - 32-bit Wrapping

Enables 32-bit integer wrapping (perfect for cryptography):

```graphoid
configure { :unsigned, :32bit } {
    x = 0xFFFFFFFF  # Max 32-bit value
    y = 1
    result = x + y
    print(result)  # 0 (wraps around)
}
```

**When to use**:
- Cryptographic algorithms (SHA-256, MD5)
- Binary protocols requiring 32-bit arithmetic
- Embedded systems with 32-bit constraints

**Example: Clean crypto code**

```graphoid
configure { :unsigned, :32bit } {
    fn rotr(x, n) {
        # Right rotation - clean and simple!
        return (x >> n) | (x << (32 - n))
    }

    result = rotr(0x12345678, 8)
    # No manual masking needed!
}
```

### :high - High Precision

Uses higher precision for numbers:

```graphoid
# Default: 64-bit floating point
pi = 3.14159265358979323846
print(pi)  # Limited precision

# High precision: 128-bit floating point
configure { :high } {
    pi = 3.14159265358979323846
    print(pi)  # Full precision
}
```

**When to use**:
- Scientific computing
- Financial calculations
- When precision matters more than performance

## Type Checking Directives

### :strict_types - Strict Type Checking

Enforces strict type constraints:

```graphoid
configure { :strict_types } {
    num x = 10
    # x = "hello"  # Error: Type mismatch
}
```

### :lenient - Lenient Type Checking

Allows implicit type conversions:

```graphoid
configure { :lenient } {
    x = "10" + 5  # "105" (converts 5 to string)
}
```

## Error Handling Directives

### :strict - Strict Error Mode

Errors stop execution immediately (default):

```graphoid
configure { :strict } {
    result = 10 / 0  # Error: Division by zero
    # Code here never runs
}
```

### :lenient - Lenient Error Mode

Errors return `none` instead of stopping:

```graphoid
configure { :lenient } {
    result = 10 / 0  # Returns none
    print(result)     # none
}
```

### :collect - Collect Errors

Collects all errors without stopping:

```graphoid
configure { :collect } {
    x = 10 / 0      # Error collected
    y = 5 / 0       # Error collected
    z = undefined   # Error collected

    errors = get_errors()
    print(errors.length())  # 3
}
```

## Bounds Checking Directives

### :bounds_strict - Strict Bounds Checking

Array access out of bounds causes error:

```graphoid
configure { :bounds_strict } {
    arr = [1, 2, 3]
    # x = arr[10]  # Error: Index out of bounds
}
```

### :bounds_lenient - Lenient Bounds Checking

Out of bounds returns `none`:

```graphoid
configure { :bounds_lenient } {
    arr = [1, 2, 3]
    x = arr[10]  # Returns none (no error)
}
```

## Combining Directives

You can combine multiple directives:

```graphoid
configure {
    :unsigned,
    :32bit,
    :strict_types,
    :bounds_strict
} {
    # All four directives active
}
```

## Directive Scope

Directives only affect code within their block:

```graphoid
x = 10 / 3
print(x)  # 3.333... (default behavior)

configure { :integer } {
    y = 10 / 3
    print(y)  # 3 (integer mode)
}

z = 10 / 3
print(z)  # 3.333... (back to default)
```

## Performance Directives

### :optimize - Optimize for Speed

Enables aggressive optimizations:

```graphoid
configure { :optimize } {
    # Compiler may inline functions, unroll loops, etc.
    result = heavy_computation()
}
```

### :debug - Debug Mode

Enables extra runtime checks:

```graphoid
configure { :debug } {
    # Extra assertions, bounds checks, type checks
    result = risky_operation()
}
```

## Common Directive Combinations

### Crypto/Binary Operations

```graphoid
configure { :unsigned, :32bit } {
    # Perfect for SHA-256, MD5, binary protocols
}
```

### Scientific Computing

```graphoid
configure { :high, :strict } {
    # High precision with strict error handling
}
```

### Embedded Systems

```graphoid
configure { :integer, :32bit, :optimize } {
    # Integer math, 32-bit wrapping, optimized
}
```

### Financial Calculations

```graphoid
configure { :high, :strict_types, :strict } {
    # Precision matters, no type mixing, strict errors
}
```

### Quick Prototyping

```graphoid
configure { :lenient, :bounds_lenient } {
    # More forgiving, faster iteration
}
```

## Querying Current Directives

Check which directives are active:

```graphoid
config = get_config()

print(config.integer_mode)    # true/false
print(config.unsigned_mode)   # true/false
print(config.precision_mode)  # "standard", "high", "extended"
```

## Best Practices

### Use Specific Directives

```graphoid
# ✅ GOOD: Specific to the task
configure { :unsigned, :32bit } {
    # Crypto code
}

# ❌ BAD: Too many directives
configure { :unsigned, :32bit, :high, :strict, :optimize } {
    # Unclear what's needed
}
```

### Keep Directive Blocks Small

```graphoid
# ✅ GOOD: Small, focused scope
configure { :integer } {
    count = calculate_items()
}

# ❌ BAD: Entire file in directive block
configure { :integer } {
    # ... 1000 lines of code ...
}
```

### Document Why

```graphoid
# ✅ GOOD: Explain the reason
# Use 32-bit wrapping for SHA-256 implementation
configure { :unsigned, :32bit } {
    hash = sha256(data)
}

# ❌ BAD: No explanation
configure { :unsigned, :32bit } {
    hash = sha256(data)
}
```

### Use Defaults When Possible

```graphoid
# ✅ GOOD: Only use directives when needed
x = 10 / 3  # Default float behavior is fine

# ❌ BAD: Unnecessary directive
configure { :lenient } {
    x = 10 / 3  # Default would work fine
}
```

## Directive Reference

| Directive | Effect | Use Case |
|-----------|--------|----------|
| `:integer` | Integer arithmetic | Counting, indexing |
| `:unsigned` | Unsigned numbers | Bit manipulation |
| `:32bit` | 32-bit wrapping | Crypto, protocols |
| `:high` | High precision | Scientific, financial |
| `:strict` | Strict errors | Production code |
| `:lenient` | Lenient errors | Prototyping |
| `:collect` | Collect errors | Batch validation |
| `:strict_types` | Strict types | Type safety |
| `:bounds_strict` | Strict bounds | Safety |
| `:bounds_lenient` | Lenient bounds | Convenience |
| `:optimize` | Optimize speed | Performance |
| `:debug` | Extra checks | Development |

## Summary

In this chapter, you learned:

- ✅ **configure blocks** - How to apply directives
- ✅ **Numeric directives** - :integer, :unsigned, :32bit, :high
- ✅ **Type checking** - :strict_types, :lenient
- ✅ **Error handling** - :strict, :lenient, :collect
- ✅ **Bounds checking** - :bounds_strict, :bounds_lenient
- ✅ **Performance** - :optimize, :debug
- ✅ **Common combinations** - Crypto, scientific, embedded
- ✅ **Best practices** - Small scopes, documentation

---

## Quick Reference

```graphoid
# Numeric modes
configure { :integer } { ... }
configure { :unsigned } { ... }
configure { :32bit } { ... }
configure { :high } { ... }

# Error handling
configure { :strict } { ... }
configure { :lenient } { ... }
configure { :collect } { ... }

# Combine directives
configure { :unsigned, :32bit, :strict } { ... }

# Nested blocks
configure { :unsigned } {
    configure { :32bit } { ... }
}
```

---

## Exercises

1. **Temperature Converter**: Write a function that uses `:high` for precise temperature conversion

2. **Bit Manipulation**: Use `:unsigned` and `:32bit` to implement bit rotation functions

3. **Safe Calculator**: Use `:strict` and `:bounds_strict` for a calculator with strong error checking

4. **Error Collector**: Use `:collect` to validate multiple inputs and report all errors at once

5. **Performance Test**: Compare performance of code with and without `:optimize`

**Solutions** are available in `examples/08-directives/exercises.gr`

---

[← Previous: Modules](07-modules.md) | [Next: Standard Library →](09-standard-library.md)
