# Directives Reference

Directives control how Graphoid interprets and executes code. They are specified in `configure` blocks using symbols (words starting with `:`).

## Syntax

```graphoid
configure { :directive1, :directive2 } {
    # Code here runs with directives enabled
}
# Code here runs with default settings
```

## Nested Configure Blocks

Directives can be nested, with inner blocks overriding outer ones:

```graphoid
configure { :unsigned } {
    x = 255

    configure { :32bit } {
        y = x + 1  # 32-bit unsigned arithmetic
    }

    z = x + 1  # Regular unsigned arithmetic
}
```

---

## Numeric Mode Directives

### :integer

Forces integer arithmetic (truncates division).

**Effect**: All arithmetic operations use integer math

**Use Cases**:
- Counting and indexing
- Avoiding floating-point rounding errors
- Performance-critical integer operations

**Examples**:
```graphoid
# Default: floating-point division
x = 10 / 3
print(x)  # 3.333...

# With :integer
configure { :integer } {
    x = 10 / 3
    print(x)  # 3 (truncated)
}
```

**Interactions**:
- Automatically enabled by `:32bit`
- Compatible with `:unsigned` and `:high`

**See also**: `:unsigned`, `:32bit`

---

### :unsigned

Treats numbers as unsigned (non-negative).

**Effect**:
- Numbers are unsigned (no negatives)
- Right shift is logical (not arithmetic)
- Enables unsigned wraparound behavior

**Use Cases**:
- Binary data processing
- Bit manipulation
- Working with unsigned protocols

**Examples**:
```graphoid
configure { :unsigned } {
    x = 255
    y = 1
    result = x + y
    print(result)  # 256

    # Logical right shift
    shifted = 128 >> 1
    print(shifted)  # 64 (fills with 0s)
}
```

**Interactions**:
- Required by `:32bit`
- Compatible with `:integer` and `:high`

**See also**: `:32bit`, bitwise operators

---

### :32bit

Enables 32-bit integer wrapping arithmetic.

**Effect**:
- All integer operations wrap at 32 bits (0xFFFFFFFF)
- Automatically enables `:integer` mode
- Automatically enables `:high` precision mode
- Perfect for cryptographic algorithms

**Use Cases**:
- Cryptography (SHA-256, MD5, etc.)
- Binary protocols requiring 32-bit arithmetic
- Embedded systems with 32-bit constraints

**Examples**:
```graphoid
configure { :unsigned, :32bit } {
    x = 0xFFFFFFFF  # Max 32-bit value
    y = 1
    result = x + y
    print(result)  # 0 (wraps around)
}

# Clean crypto code
configure { :unsigned, :32bit } {
    fn rotr(x, n) {
        # Right rotation - no manual masking needed!
        return (x >> n) | (x << (32 - n))
    }

    result = rotr(0x12345678, 8)
}
```

**Interactions**:
- Requires `:unsigned` (auto-enabled if not specified)
- Automatically enables `:integer`
- Automatically enables `:high` precision

**Implementation Notes**:
- Values are masked to 32 bits: `value & 0xFFFFFFFF`
- Uses wrapping arithmetic: `wrapping_add`, `wrapping_sub`, etc.

**See also**: `:unsigned`, `:integer`, bitwise operators

---

### :high

Uses higher precision for numbers.

**Effect**:
- 128-bit floating point instead of 64-bit
- Better precision for scientific/financial calculations
- Larger range for integers

**Use Cases**:
- Scientific computing
- Financial calculations
- When precision matters more than performance

**Examples**:
```graphoid
# Default: 64-bit floating point
pi = 3.14159265358979323846
print(pi)  # Limited precision

# High precision: 128-bit
configure { :high } {
    pi = 3.14159265358979323846
    print(pi)  # Full precision retained
}
```

**Interactions**:
- Automatically enabled by `:32bit`
- Compatible with `:integer` and `:unsigned`
- May impact performance

**See also**: BigNum types, `:integer`

---

## Type Checking Directives

### :strict_types

Enforces strict type constraints.

**Effect**:
- Type annotations are strictly enforced
- No implicit type conversions
- Type mismatches cause errors

**Use Cases**:
- Production code requiring type safety
- APIs with strict contracts
- Catching type errors early

**Examples**:
```graphoid
configure { :strict_types } {
    num x = 10
    # x = "hello"  # Error: Type mismatch

    list<num> scores = [95, 87, 92]
    # scores.append("A")  # Error: Type mismatch
}
```

**See also**: `:lenient` (types)

---

### :lenient (Types)

Allows implicit type conversions.

**Effect**:
- Types are more flexible
- Automatic type coercion when safe
- String concatenation coerces numbers

**Use Cases**:
- Quick prototyping
- Scripts where flexibility matters
- Interactive REPL usage

**Examples**:
```graphoid
configure { :lenient } {
    # String concatenation with number
    result = "Count: " + 5  # "Count: 5"

    # Mixed arithmetic
    x = "10" + 5  # May convert string to number
}
```

**Note**: `:lenient` also affects error handling (see Error Handling section)

**See also**: `:strict_types`

---

## Error Handling Directives

### :strict

Errors stop execution immediately (default behavior).

**Effect**:
- Errors raise exceptions and halt execution
- Stack trace is printed
- No error recovery

**Use Cases**:
- Production code (default)
- When errors should never be ignored
- Debugging with full error information

**Examples**:
```graphoid
configure { :strict } {
    result = 10 / 0  # Error: Division by zero
    # Code here never runs
}
```

**Default**: Yes (this is the default mode)

**See also**: `:lenient`, `:collect`

---

### :lenient (Errors)

Errors return `none` instead of stopping execution.

**Effect**:
- Operations that would error return `none`
- Execution continues
- Errors are not reported unless checked

**Use Cases**:
- Graceful degradation
- Optional operations
- Batch processing where some failures are acceptable

**Examples**:
```graphoid
configure { :lenient } {
    result = 10 / 0  # Returns none (no error)
    print(result)     # none

    # Array access out of bounds
    arr = [1, 2, 3]
    x = arr[10]  # none (no error)
}
```

**Warnings**:
- Silent failures can hide bugs
- Always check for `none` results
- Not recommended for production code

**See also**: `:strict`, `:collect`

---

### :collect

Collects all errors without stopping execution.

**Effect**:
- Errors are recorded but don't halt execution
- Multiple errors can accumulate
- Errors can be retrieved with `get_errors()`

**Use Cases**:
- Batch validation
- Testing multiple conditions
- Form validation with multiple fields

**Examples**:
```graphoid
configure { :collect } {
    x = 10 / 0      # Error collected
    y = 5 / 0       # Error collected
    z = undefined   # Error collected

    errors = get_errors()
    print(errors.length())  # 3

    for error in errors {
        print(error.message)
    }
}
```

**See also**: `:strict`, `:lenient`, `get_errors()`

---

## Bounds Checking Directives

### :bounds_strict

Array/collection access out of bounds causes error (default).

**Effect**:
- Out-of-bounds access raises error
- Prevents silent bugs
- Ensures index validity

**Use Cases**:
- Production code (default)
- Safety-critical applications
- Debugging index issues

**Examples**:
```graphoid
configure { :bounds_strict } {
    arr = [1, 2, 3]
    # x = arr[10]  # Error: Index out of bounds
}
```

**Default**: Yes (this is the default mode)

**See also**: `:bounds_lenient`

---

### :bounds_lenient

Out-of-bounds access returns `none` instead of error.

**Effect**:
- Out-of-bounds reads return `none`
- No error raised
- Execution continues

**Use Cases**:
- Sparse data structures
- Optional lookups
- Quick prototyping

**Examples**:
```graphoid
configure { :bounds_lenient } {
    arr = [1, 2, 3]
    x = arr[10]  # Returns none (no error)

    if x != none {
        print(x)
    } else {
        print("Index not found")
    }
}
```

**Warnings**:
- Can hide indexing bugs
- Always check for `none`

**See also**: `:bounds_strict`

---

## Performance Directives

### :optimize

Enables aggressive optimizations for speed.

**Effect**:
- Compiler may inline functions
- Loop unrolling
- Dead code elimination
- May reduce debugging information

**Use Cases**:
- Production builds
- Performance-critical code
- After debugging is complete

**Examples**:
```graphoid
configure { :optimize } {
    # Heavy computation optimized
    result = heavy_computation()
}
```

**Trade-offs**:
- Faster execution
- Longer compile time
- Less debuggable

**See also**: `:debug`

---

### :debug

Enables extra runtime checks and debugging information.

**Effect**:
- Additional assertions
- Enhanced bounds checking
- Type validation
- Better error messages
- Stack traces with more detail

**Use Cases**:
- Development
- Debugging issues
- Testing

**Examples**:
```graphoid
configure { :debug } {
    # Extra checks enabled
    result = risky_operation()
}
```

**Trade-offs**:
- Slower execution
- Better error diagnostics
- Catches more bugs

**See also**: `:optimize`

---

## Common Directive Combinations

### Cryptography / Binary Operations

```graphoid
configure { :unsigned, :32bit } {
    # Perfect for SHA-256, MD5, binary protocols
    # Clean code without manual masking
}
```

**Why**: Cryptographic algorithms require exact 32-bit wraparound behavior

---

### Scientific Computing

```graphoid
configure { :high, :strict } {
    # High precision with strict error handling
    # Ensures accuracy and catches errors early
}
```

**Why**: Precision matters, errors should never be ignored

---

### Embedded Systems

```graphoid
configure { :integer, :32bit, :optimize } {
    # Integer math, 32-bit wrapping, optimized
    # Fast and predictable for resource-constrained systems
}
```

**Why**: Performance and predictability are critical

---

### Financial Calculations

```graphoid
configure { :high, :strict_types, :strict } {
    # Precision matters, no type mixing, strict errors
    # Money requires exactness
}
```

**Why**: Financial data requires precision and type safety

---

### Quick Prototyping

```graphoid
configure { :lenient, :bounds_lenient } {
    # More forgiving, faster iteration
    # Good for REPL and experiments
}
```

**Why**: Flexibility over safety during exploration

---

### Batch Validation

```graphoid
configure { :collect } {
    # Collect all validation errors
    # Show user all issues at once
}
```

**Why**: Better UX to show all errors, not just first one

---

## Querying Current Configuration

```graphoid
config = get_config()

# Check current modes
print(config.integer_mode)    # true/false
print(config.unsigned_mode)   # true/false
print(config.bit_width)       # :32bit or :64bit
print(config.precision_mode)  # :standard, :high, or :extended
print(config.error_mode)      # :strict, :lenient, or :collect
```

---

## Directive Scope Rules

1. **Directives are scoped to blocks**: Only code inside the `configure { }` block is affected
2. **Directives don't leak**: After the block, settings revert to previous state
3. **Nested blocks override**: Inner `configure` blocks override outer ones
4. **Stack-based**: Each `configure` pushes settings onto a stack

**Example**:
```graphoid
x = 10 / 3
print(x)  # 3.333... (default)

configure { :integer } {
    y = 10 / 3
    print(y)  # 3 (integer mode)

    configure { :high } {
        z = 10 / 3
        print(z)  # 3 (still integer, now high precision)
    }
}

w = 10 / 3
print(w)  # 3.333... (back to default)
```

---

## Directive Summary Table

| Directive | Category | Effect | Default | Auto-enables |
|-----------|----------|--------|---------|--------------|
| `:integer` | Numeric | Integer arithmetic | No | - |
| `:unsigned` | Numeric | Unsigned numbers | No | - |
| `:32bit` | Numeric | 32-bit wrapping | No | `:integer`, `:high` |
| `:high` | Numeric | High precision | No | - |
| `:strict_types` | Types | Strict type checking | No | - |
| `:lenient` | Types/Errors | Lenient mode | No | - |
| `:strict` | Errors | Strict errors | Yes | - |
| `:collect` | Errors | Collect errors | No | - |
| `:bounds_strict` | Bounds | Strict bounds | Yes | - |
| `:bounds_lenient` | Bounds | Lenient bounds | No | - |
| `:optimize` | Performance | Optimize speed | No | - |
| `:debug` | Performance | Debug mode | No | - |

---

## Best Practices

### 1. Use Specific Directives

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

### 2. Keep Directive Blocks Small

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

### 3. Document Why

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

### 4. Use Defaults When Possible

```graphoid
# ✅ GOOD: Only use directives when needed
x = 10 / 3  # Default float behavior is fine

# ❌ BAD: Unnecessary directive
configure { :lenient } {
    x = 10 / 3  # Default would work fine
}
```

---

## See Also

- [User Guide: Directives](../user-guide/08-directives.md) - Comprehensive directive guide with examples
- [Operators](operators.md) - How operators behave under different directives
- [num](core/num.md) - Numeric type affected by directives
