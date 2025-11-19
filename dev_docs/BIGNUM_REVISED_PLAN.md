# BigNum Type: Revised Implementation Plan
**Created**: November 17, 2025
**Status**: Design Review
**Supersedes**: BIGNUM_PRECISION_PLAN.md

---

## Executive Summary

This document describes a **revised approach** to implementing high-precision numeric types in Graphoid. Unlike the original plan which treated bignum as a separate type that forbids mixing with num, this plan makes bignum **seamlessly interoperable** with num through automatic casting and promotion.

**Key Principles**:
1. **Bignum is a float by default** (like num) unless `:integer` directive is active
2. **Auto-promotion on overflow** - results that exceed num space become bignum automatically
3. **Mixed operations allowed** - num and bignum can be mixed (automatic casting)
4. **Explicit and implicit instantiation** - both `bignum x = 999` and `x = huge_literal` work
5. **Native Rust implementation** - cannot be done efficiently in pure Graphoid

---

## Why Native Rust Implementation is Required

**Question**: Can bignum be implemented in pure Graphoid as a core module?

**Answer**: NO. Here's why:

### Impossible to Implement in Pure Graphoid

1. **Arbitrary Precision Requires Multi-Word Arithmetic**
   ```graphoid
   # Hypothetical pure Graphoid attempt - TERRIBLE!
   bignum_a = [word1, word2, word3, word4]  # Represent as list of 64-bit words
   bignum_b = [word1, word2, word3, word4]

   # Addition with carry propagation - HORRIBLE!
   fn bignum_add(a, b) {
       result = []
       carry = 0
       for i in 0..a.length() {
           sum = a[i] + b[i] + carry
           result.append(sum & 0xFFFFFFFFFFFFFFFF)
           carry = sum >> 64
       }
       return result
   }
   ```

   **Problems**:
   - 100-1000x slower than native implementation
   - Complex carry handling is error-prone
   - Doesn't match algorithm specifications (crypto code becomes unreadable)
   - Memory inefficient (each word stored as f64!)

2. **Float128 (f128) Cannot Be Represented**
   - Graphoid's base `num` is f64 (53-bit mantissa, ~15-17 decimal digits)
   - Cannot store 128-bit floats (~34 decimal digits) in f64
   - Requires native library support (software or hardware)

3. **Performance Critical for Crypto and Science**
   - SHA-512, RSA, Ed25519 need fast 64-bit+ arithmetic
   - Scientific computing needs efficient high-precision floats
   - Pure Graphoid would be too slow for production use

**Conclusion**: BigNum MUST be a **native Rust-implemented core type**, like num, string, list, etc.

---

## Design Differences from Original Plan

| Aspect | Original Plan | New Plan |
|--------|---------------|----------|
| **Mixing num + bignum** | ❌ Forbidden (explicit conversion required) | ✅ Allowed (auto-cast to bignum) |
| **Bignum nature** | Integer-focused (i64/u64/f128/BigInt variants) | Float by default (like num) |
| **Overflow behavior** | Only inside `precision { :high }` blocks | Auto-promotion everywhere (unless disabled) |
| **Literal detection** | Only in precision blocks | Auto-detect large literals as bignum |
| **Integer mode** | Separate from :high (implicit when whole numbers) | Explicit `:integer` directive |
| **Configuration** | `configure { precision: :high }` | `precision { :high } { ... }` |

---

## Core Type: BigNum

### BigNum Variants (Internal Rust Enum)

```rust
// In src/values/mod.rs
pub enum BigNum {
    // High precision integers (64-bit)
    Int64(i64),           // :high + :integer mode (signed)
    UInt64(u64),          // :high + :integer + :unsigned mode

    // High precision floats (128-bit)
    Float128(f128),       // :high mode (default - float unless :integer)

    // Arbitrary precision integers
    BigInt(BigInt),       // :high + :integer + auto-growth OR explicit :extended

    // Arbitrary precision floats (future)
    // BigDecimal(BigDecimal),  // :extended mode for floats (Phase 2)
}
```

### Default Behavior: Bignum is Float

```graphoid
# Explicit bignum declaration - defaults to float
bignum a = 2.5
bignum b = 3.7
c = a + b  # c = 6.2 (bignum float)

# Integer directive forces integer mode
precision { :integer } {
    bignum x = 2.5  # Truncated to 2 (bignum integer)
    bignum y = 3.7  # Truncated to 3 (bignum integer)
    z = x + y       # z = 5 (bignum integer)
}

# High precision directive upgrades to bignum
precision { :high } {
    # Still defaults to float!
    a = 2.5  # bignum Float128
    b = 3.7  # bignum Float128
    c = a + b  # bignum Float128 (6.2 with ~34 decimal digits precision)
}

# Combined: high + integer
precision { :high, :integer } {
    a = 2.5  # bignum Int64 (truncated to 2)
    b = 3.7  # bignum Int64 (truncated to 3)
    c = a + b  # bignum Int64 (5)
}
```

**Rationale**: Makes bignum consistent with num - floats by default, integers only when explicitly requested.

---

## Automatic Promotion and Detection

### 1. Large Literal Detection

```graphoid
# Literals that overflow f64 precision become bignum automatically
a = 99999999999999999999999999  # Too big for f64 → bignum BigInt
b = 1.23456789012345678901234567890  # More precision than f64 → bignum Float128

# Normal literals stay as num
c = 100  # num (f64)
d = 3.14  # num (f64)
```

**Detection rules**:
- **Integer literals** > 2^53 (f64 safe integer range) → `bignum BigInt`
- **Float literals** with > 17 significant digits → `bignum Float128`
- **Normal literals** → `num` (f64)

### 2. Overflow Auto-Promotion

```graphoid
# Result exceeds num precision → auto-promote to bignum
a = 1000 ** 1000  # Result too large for f64 → bignum BigInt

# Very precise result → auto-promote to bignum
b = 1.0 / 3.0     # num (0.3333333333333333 - f64 precision)
c = very_precise_division()  # Returns bignum if precision needed

# Disable auto-promotion if desired
precision { :standard } {
    # Forces num even if result overflows (may lose precision or error)
    result = 1000 ** 1000  # ERROR: overflow in standard precision mode
}
```

**Promotion rules**:
- Operations that would overflow f64 → promote to `bignum` (unless `:standard` directive disallows)
- Operations on bignum values → result is bignum
- Operations mixing num and bignum → result is bignum (auto-cast)

### 3. Directive: `:high` Forces Bignum

```graphoid
precision { :high } {
    # Even small numbers become bignum for consistency
    a = 2.2       # bignum Float128 (not num!)
    b = 3.4456    # bignum Float128
    c = a + b     # bignum Float128

    # Ensures all operations in scope use high precision
}
```

---

## Mixed Operations: Automatic Casting

**Core principle**: num and bignum can be mixed freely - the num is auto-cast to bignum **for the operation only**.

**⚠️ CRITICAL: The original num variable is NOT mutated - only the result becomes bignum.**

```graphoid
num a = 5
bignum b = 10  # bignum Float128 (default)

# Mixed operation: a is TEMPORARILY cast to bignum for the operation
c = a * b  # c = 50 (bignum Float128)

# IMPORTANT: a is still num! It was not mutated!
print(a.type_name())  # "num" - unchanged!
print(c.type_name())  # "bignum" - result is promoted

# Works in both directions
d = b + a  # d = 15 (bignum Float128)
print(a.type_name())  # "num" - still unchanged!

# Casting rules:
# num → bignum (for operation): Always safe, temporary, no mutation
# bignum → num: Requires explicit .to_num() (may lose precision)
```

### Casting Methods

```graphoid
# Explicit conversions
bignum big_val = 123456789012345678901234567890

# bignum → num (may lose precision)
small = big_val.to_num()  # Returns num, may lose precision or error if too large

# num → bignum (always safe)
num regular = 42
big = regular.to_bignum()  # Explicit conversion (though auto-cast works too)

# Detect type
print(big_val.type_name())  # "bignum"
print(regular.type_name())  # "num"

# Check if conversion is safe
if big_val.fits_in_num() {  # New method
    safe = big_val.to_num()
}
```

---

## Precision Directives

### Standard Precision (Default)

```graphoid
# Default: f64 arithmetic, auto-promote on overflow
a = 100 + 200  # num (300.0 as f64)

b = 1000 ** 1000  # bignum BigInt (auto-promoted - too large for f64)
```

### High Precision (`:high`)

```graphoid
precision { :high } {
    # ALL operations use bignum (Float128 by default)
    a = 2.2       # bignum Float128
    b = 3.4456    # bignum Float128
    c = a + b     # bignum Float128

    # ~34 decimal digits of precision vs ~17 for f64
}

# Combined with :integer
precision { :high, :integer } {
    # ALL operations use bignum integers (Int64 or auto-grow to BigInt)
    a = 2.2       # bignum Int64 (truncated to 2)
    b = 3.7       # bignum Int64 (truncated to 3)
    c = a + b     # bignum Int64 (5)

    # Auto-grows to BigInt on overflow
    huge = 2 ** 100  # bignum BigInt (too large for Int64)
}

# Combined with :unsigned
precision { :high, :integer, :unsigned } {
    # Uses UInt64, auto-grows to BigInt (unsigned) on overflow
    a = 18446744073709551615  # bignum UInt64 (max u64)
    b = a + 1                  # bignum BigInt (auto-promoted)
}
```

### Extended Precision (`:extended`) - Future

```graphoid
# Future: Arbitrary precision for floats
precision { :extended } {
    # BigDecimal for arbitrary precision floats
    pi = calculate_pi(1000)  # 1000 decimal digits

    # BigInt for arbitrary precision integers (same as :high + :integer with auto-growth)
    huge = 2 ** 10000
}
```

**Note**: Extended precision for floats (BigDecimal) is Phase 2. For now, `:extended` behaves like `:high`.

---

## Type Declarations

### Explicit Type Annotation

```graphoid
# Force a value to be bignum even if it could be num
bignum a = 999  # Small number, but stored as bignum Float128

# Regular num
num b = 999  # Stored as f64

# Type inference (default)
c = 999  # num (f64) - fits in f64 precision
d = 99999999999999999999  # bignum BigInt - too large for f64
```

### Type Constraints

```graphoid
# Function parameters
fn process(bignum value) {
    # value must be bignum (or auto-cast from num)
    return value * 2  # Returns bignum
}

# Variable reassignment
bignum x = 10
x = 20  # OK - still bignum
x = huge_value  # OK - bignum

# Type checking
if value.is_bignum() {  # New method
    print("High precision value")
}
```

---

## Implementation Phases

### Phase 1A: `:integer` Directive with Existing `num` Type (2-3 days)

**Goal**: Implement and validate `:integer` directive truncation semantics with the existing `num` type BEFORE adding bignum complexity.

**Rationale**: Get truncation-on-assignment working first, validate the semantics are correct, THEN extend to bignum. This avoids implementation gaps and validates design decisions early.

**TDD Rhythm**: Write 3-8 tests → Implement feature → Tests pass → Refactor → Repeat

**Tasks**:

**1. Implement `:integer` directive support:**

Add to `src/execution/config.rs`:
```rust
pub struct Config {
    // ... existing fields ...
    pub integer_mode: bool,  // NEW - :integer directive
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // ... existing defaults ...
            integer_mode: false,  // NEW
        }
    }
}

// In push_with_changes, add handling for :integer
fn push_with_changes(...) {
    // ... existing code ...
    if let ValueKind::Symbol(sym) = &value.kind {
        if sym == "integer" {
            new_config.integer_mode = true;
            continue;
        }
        // ... existing unsigned check ...
    }
}
```

**TDD**: Write 3-5 tests for parsing and config handling, then implement

**2. Implement truncation-on-assignment when `integer_mode: true`:**

In `src/execution/executor.rs`, when assigning values:
```rust
fn truncate_if_integer_mode(&self, value: Value) -> Value {
    if !self.config_stack.current().integer_mode {
        return value;
    }

    match &value.kind {
        ValueKind::Number(n) => Value::number(n.trunc()),
        // Bignum support will be added in Phase 1B
        _ => value, // Non-numeric values pass through
    }
}
```

**TDD**: Write 5-7 tests for truncation behavior, then implement

**3. Test nested precision blocks:**

```graphoid
precision { :integer } {
    a = 5.7  # Truncated to 5.0
    b = 3.2  # Truncated to 3.0
    c = a + b  # 5.0 + 3.0 = 8.0 (not 8.9999...)
}
# a, b, c don't exist outside the block
```

**TDD**: Write 3-5 tests for scope handling, verify implementation

**4. Create example file:**

Create `rust/samples/integer_mode.gr` demonstrating `:integer` directive

**Success criteria**:
- ✅ `precision { :integer } { a = 5.7 }` truncates a to 5.0
- ✅ `precision { :integer } { b = 3.2; c = a + b }` results in 8.0 (from 5.0 + 3.0)
- ✅ Variables scoped correctly (don't leak outside blocks)
- ✅ All tests pass (11-17 new tests)
- ✅ Example file runs successfully

**Time estimate**: 2-3 days

---

### Phase 1B: Add BigNum Type (3-4 days)

**Goal**: Add `bignum` type with Float128 support, now that truncation semantics are validated

**Rationale**: With `:integer` working for `num`, extending to bignum is straightforward and we can reuse the truncation logic.

**TDD Rhythm**: Write 3-8 tests → Implement feature → Tests pass → Refactor → Repeat

**Prerequisites**: Phase 1A complete (`:integer` directive working with `num`)

**Tasks**:

**1. Add dependencies to `Cargo.toml`:**
```toml
[dependencies]
# ... existing dependencies ...

# BigNum support
num-bigint = "0.4"         # Arbitrary precision integers
num-traits = "0.2"         # Numeric trait abstractions
f128 = "0.2"               # 128-bit floats (or use "rug" for MPFR)
```

**TDD**: No tests needed for dependency addition

**2. Extend `BigNum` enum in `src/values/mod.rs`:**
```rust
pub enum BigNum {
    Int64(i64),        // Already exists
    UInt64(u64),       // Already exists
    Float128(f128),    // NEW - add this variant
    BigInt(BigInt),    // NEW - add this variant (for Phase 3)
}
```

**TDD**: Write 5 tests for bignum value creation and storage, then implement

**3. Add `bignum` keyword to lexer and parser:**

Update lexer and parser to recognize `bignum` type declarations

**TDD**: Write 3-5 tests for parsing, then implement

**4. Implement basic Float128 arithmetic:**

Update arithmetic operators in `eval_add`, `eval_multiply`, etc.

**TDD**: Write 10-12 tests for Float128 arithmetic, then implement

**5. Extend truncation logic for bignum:**

Update `truncate_if_integer_mode` to handle Float128:
```rust
fn truncate_if_integer_mode(&self, value: Value) -> Value {
    if !self.config_stack.current().integer_mode {
        return value;
    }

    match &value.kind {
        ValueKind::Number(n) => Value::number(n.trunc()),
        ValueKind::BigNumber(bn) => {
            // Truncate bignum values too
            match bn {
                BigNum::Float128(f) => Value::bignum(BigNum::Float128(f.trunc())),
                _ => value, // Already integer
            }
        }
        _ => value, // Non-numeric values pass through
    }
}
```

**TDD**: Write 5 tests for bignum + `:integer` interaction, then implement

**6. Implement mixed num/bignum operations (WITHOUT mutation):**

**CRITICAL**: Test that original `num` variable is NOT mutated

```rust
// In eval_add, eval_multiply, etc.
(ValueKind::Number(n), ValueKind::BigNumber(bn)) |
(ValueKind::BigNumber(bn), ValueKind::Number(n)) => {
    // Create TEMPORARY bignum copy for operation - do NOT mutate original
    let bn_temp = num_to_bignum_f128(*n);
    let result = bignum_add(bn_temp, bn.clone())?;
    Ok(Value::bignum(result))
    // Original num variable remains unchanged
}
```

**TDD**: Write 8-10 tests including mutation prevention tests, then implement

**7. Implement large literal detection:**

```rust
// In executor, when evaluating number literals
fn eval_number_literal(&self, value: f64, literal_str: &str) -> Result<Value> {
    // Check if literal exceeds f64 precision
    if literal_needs_bignum(literal_str) {
        return Ok(Value::bignum(parse_bignum_literal(literal_str)?));
    }
    Ok(Value::number(value))
}
```

**TDD**: Write 5 tests for literal detection, then implement

**8. Create example files:**

- `rust/samples/bignum_basics.gr` - Basic bignum usage
- `rust/samples/high_precision.gr` - Float128 demonstration

**Success criteria**:
- ✅ `bignum a = 2.5` works and stores as Float128
- ✅ `precision { :high } { ... }` forces bignum operations (Float128)
- ✅ `precision { :high, :integer } { ... }` forces bignum with truncation
- ✅ Large literals auto-detected as bignum
- ✅ `num a = 5; bignum b = 10; c = a * b` works (c is bignum)
- ✅ **CRITICAL**: `a` remains `num` after operation (no mutation)
- ✅ All tests pass (36-42 new tests)
- ✅ Example files run successfully

**Time estimate**: 3-4 days

---

### Combined Phase 1 Total: 5-7 days

**Phase 1A + Phase 1B = Complete Foundation**
- `:integer` directive working with `num` (2-3 days)
- `bignum` type with Float128 support (3-4 days)
- Total new tests: 47-59 tests

---

### Phase 2: Auto-Promotion and Casting (Week 2) - 4-6 days

**Goal**: Automatic overflow promotion and mixed num/bignum operations

**Tasks**:
1. ✅ Implement overflow detection in arithmetic:
   ```rust
   fn eval_add(&self, left: Value, right: Value) -> Result<Value> {
       match (&left.kind, &right.kind) {
           (ValueKind::Number(l), ValueKind::Number(r)) => {
               let result = l + r;

               // Check for overflow/precision loss
               if self.should_promote_to_bignum(*l, *r, result) {
                   // Auto-promote to bignum
                   return Ok(self.promote_to_bignum(result));
               }

               Ok(Value::number(result))
           }
           // ... other cases
       }
   }
   ```

2. ✅ Implement mixed operations (num + bignum):
   ```rust
   // In eval_add, eval_multiply, etc.
   (ValueKind::Number(n), ValueKind::BigNumber(bn)) |
   (ValueKind::BigNumber(bn), ValueKind::Number(n)) => {
       // IMPORTANT: Create a TEMPORARY bignum copy of num for operation
       // The original num value is NOT mutated!
       let bn_left = self.num_to_bignum(*n);  // Creates new bignum
       let result = self.bignum_add(bn_left, bn.clone())?;
       Ok(Value::bignum(result))
       // Note: Original num variable remains unchanged
   }
   ```

3. ✅ Implement `to_num()`, `to_bignum()`, `fits_in_num()`, `is_bignum()` methods

4. ✅ Update comparison operators to handle num/bignum mixing:
   ```rust
   // Allow comparisons across types
   (ValueKind::Number(n), ValueKind::BigNumber(bn)) => {
       Ok(Value::boolean((*n as f128) < bn.to_f128()))
   }
   ```

5. ✅ Write TDD tests (40+ tests):
   - Overflow auto-promotion (5 tests)
   - Mixed num + bignum operations (10 tests)
   - **CRITICAL: Test that original num is NOT mutated** (5 tests)
   - Casting methods (8 tests)
   - Comparison across types (7 tests)
   - Precision preservation (5 tests)

**Success criteria**:
- ✅ `1000 ** 1000` auto-promotes to bignum
- ✅ `num a = 5; bignum b = 10; c = a * b` works (c is bignum)
- ✅ **CRITICAL: `a` remains `num` after the operation (no mutation)**
- ✅ `bignum.to_num()` works with precision loss handling
- ✅ All tests pass (including mutation prevention tests)

**Time estimate**: 4-6 days

---

### Phase 3: Arbitrary Precision Integers (Week 3) - 3-5 days

**Goal**: BigInt support for arbitrarily large integers

**Tasks**:
1. ✅ Integrate `num-bigint` crate fully

2. ✅ Implement auto-growth from Int64/UInt64 to BigInt:
   ```rust
   fn eval_add_bignum(&self, left: BigNum, right: BigNum) -> Result<BigNum> {
       match (left, right) {
           (BigNum::Int64(l), BigNum::Int64(r)) => {
               match l.checked_add(r) {
                   Some(result) => Ok(BigNum::Int64(result)),
                   None => {
                       // Overflow - promote to BigInt
                       let big_l = BigInt::from(l);
                       let big_r = BigInt::from(r);
                       Ok(BigNum::BigInt(big_l + big_r))
                   }
               }
           }
           (BigNum::BigInt(l), BigNum::BigInt(r)) => {
               Ok(BigNum::BigInt(l + r))
           }
           // ... handle mixed Int64/BigInt, promotion, etc.
       }
   }
   ```

3. ✅ Implement all BigInt arithmetic operations

4. ✅ Add `bit_length()`, `to_bytes()`, `from_bytes()` methods for crypto

5. ✅ Write TDD tests (25+ tests):
   - Auto-growth to BigInt
   - Very large integer arithmetic
   - Bit operations on BigInt
   - Conversion methods

**Success criteria**:
- `2 ** 1000` creates BigInt
- `precision { :high, :integer } { ... }` auto-grows to BigInt on overflow
- RSA-sized arithmetic (2048-bit) works
- All tests pass

**Time estimate**: 3-5 days

---

### Phase 4: Documentation & Examples (Week 4) - 3-4 days

**Goal**: Complete documentation and example programs

**Tasks**:
1. ✅ Update `LANGUAGE_SPECIFICATION.md`:
   - Add bignum type documentation
   - Document precision directives (`:high`, `:integer`, `:unsigned`)
   - Document auto-promotion behavior
   - Document casting methods

2. ✅ Create example files in `rust/samples/`:
   - `bignum_basics.gr` - Basic bignum usage
   - `high_precision.gr` - Float128 precision demonstration
   - `large_integers.gr` - BigInt demonstration
   - `mixed_operations.gr` - num/bignum mixing
   - `crypto_math.gr` - RSA arithmetic example

3. ✅ Update `dev_docs/` roadmap with bignum status

4. ✅ Add bignum methods to stdlib documentation

**Success criteria**:
- Spec fully documents bignum type
- Example files demonstrate all features
- Users understand when/how to use bignum
- All examples run successfully

**Time estimate**: 3-4 days

---

## Total Implementation Timeline

| Phase | Focus | Duration | Cumulative |
|-------|-------|----------|------------|
| 1 | Foundation (Float128, Int64) | 5-7 days | Week 1 |
| 2 | Auto-Promotion & Casting | 4-6 days | Week 2 |
| 3 | Arbitrary Precision (BigInt) | 3-5 days | Week 3 |
| 4 | Documentation & Examples | 3-4 days | Week 4 |

**Total**: 15-22 days (~3-4 weeks)

---

## Type System Integration

### ValueKind Extension (Already Partially Done)

```rust
// In src/values/mod.rs
pub enum ValueKind {
    None,
    Boolean(bool),
    Number(f64),          // Standard precision (f64)
    BigNumber(BigNum),    // High/arbitrary precision
    String(String),
    Symbol(String),
    List(List),
    Map(Hash),
    Function(Function),
    Graph(Graph),
    // ... etc
}

pub enum BigNum {
    Int64(i64),           // :high + :integer (signed)
    UInt64(u64),          // :high + :integer + :unsigned
    Float128(f128),       // :high (default float mode)
    BigInt(BigInt),       // Arbitrary precision integers
    // BigDecimal(BigDecimal),  // Future: arbitrary precision floats
}

impl BigNum {
    /// Convert to f64 (may lose precision)
    pub fn to_f64(&self) -> f64 { /* ... */ }

    /// Convert to f128
    pub fn to_f128(&self) -> f128 { /* ... */ }

    /// Check if value fits in f64 without precision loss
    pub fn fits_in_f64(&self) -> bool { /* ... */ }

    /// Get bit length (for integers)
    pub fn bit_length(&self) -> Option<usize> { /* ... */ }

    /// Convert to bytes (for crypto)
    pub fn to_bytes(&self) -> Vec<u8> { /* ... */ }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self { /* ... */ }
}
```

### Parser Extensions

```rust
// In src/parser/mod.rs
pub enum TypeAnnotation {
    Num,       // Standard f64
    BigNum,    // High-precision numeric type
    String,
    Bool,
    List,
    // ... etc
}

// Parse type declarations
fn parse_type_annotation(&mut self) -> Result<TypeAnnotation> {
    match self.current_token() {
        Token::Num => Ok(TypeAnnotation::Num),
        Token::BigNum => Ok(TypeAnnotation::BigNum),  // NEW
        // ... etc
    }
}
```

---

## Language Specification Updates

### New Section: BigNum Type

```markdown
## BigNum Type

Graphoid provides a `bignum` type for high-precision numeric operations beyond the standard `num` (f64) precision.

### Characteristics

- **Arbitrary precision**: Can represent numbers too large or precise for f64
- **Float by default**: Like `num`, bignum is floating-point unless `:integer` directive is active
- **Auto-promotion**: Values that exceed f64 precision automatically become bignum
- **Seamless mixing**: bignum and num can be mixed in operations (auto-casting)

### Declaration Syntax

```graphoid
# Explicit type annotation
bignum large = 123456789012345678901234567890

# Type inference from literal
huge = 99999999999999999999999999  # Auto-detected as bignum

# In precision blocks
precision { :high } {
    a = 2.5  # bignum Float128
}
```

### Precision Modes

#### Standard (Default)
```graphoid
a = 100 + 200  # num (f64)
b = 1000 ** 1000  # bignum (auto-promoted - too large for f64)
```

#### High Precision
```graphoid
precision { :high } {
    # Float mode (default)
    a = 2.5  # bignum Float128 (~34 decimal digits)
}

precision { :high, :integer } {
    # Integer mode
    a = 2  # bignum Int64 (auto-grows to BigInt on overflow)
}
```

### Mixed Operations

```graphoid
num a = 5
bignum b = 10

# Automatic casting - result is bignum
c = a * b  # 50 (bignum)
```

### Conversion Methods

```graphoid
bignum big_val = 123456789012345678901234567890

# Convert to num (may lose precision)
small = big_val.to_num()  # num

# Check if safe to convert
if big_val.fits_in_num() {
    safe = big_val.to_num()
}

# Type checking
big_val.is_bignum()  # true
big_val.type_name()  # "bignum"
```

### Bignum Methods

All standard numeric methods work on bignum:

```graphoid
x = bignum_value

# Standard methods
x.abs()
x.sign()
x.round()
x.floor()
x.ceil()
x.to_string()

# Bignum-specific methods
x.to_num()           # Convert to num (may lose precision)
x.fits_in_num()      # Check if conversion is safe
x.bit_length()       # Number of bits (integers only)
x.to_bytes()         # Byte representation (for crypto)

# Type checking
x.is_bignum()        # true
x.type_name()        # "bignum"
```
```

---

## Example Programs

### Example 1: Basic BigNum Usage

```graphoid
# samples/bignum_basics.gr

print("=== BigNum Basics ===")

# Explicit declaration
bignum a = 2.5
bignum b = 3.7
c = a + b
print("Explicit: " + c.to_string() + " (type: " + c.type_name() + ")")

# Large literal auto-detection
huge = 99999999999999999999999999
print("Large literal: " + huge.to_string() + " (type: " + huge.type_name() + ")")

# Overflow auto-promotion
overflow = 1000 ** 1000
print("Auto-promoted: overflow result (type: " + overflow.type_name() + ")")

# Mixed operations - IMPORTANT: original num is not mutated!
num regular = 5
bignum precise = 10.123456789012345678901234567890
mixed = regular * precise
print("Mixed: " + mixed.to_string() + " (type: " + mixed.type_name() + ")")
print("Original regular: " + regular.to_string() + " (type: " + regular.type_name() + ")")  # Still num!

print("")
print("=== All Tests Passed ===")
```

### Example 2: High Precision Mode

```graphoid
# samples/high_precision.gr

print("=== High Precision Mode ===")

# Standard precision (f64)
a = 1.0 / 3.0
print("Standard: 1/3 = " + a.to_string())  # ~0.3333333333333333 (17 digits)

# High precision (f128)
precision { :high } {
    b = 1.0 / 3.0
    print("High: 1/3 = " + b.to_string())  # ~34 decimal digits

    # All operations use Float128
    c = 2.2 + 3.4456
    print("High precision sum: " + c.to_string())
}

# Integer mode
precision { :high, :integer } {
    x = 2.2  # Truncated to 2
    y = 3.7  # Truncated to 3
    z = x + y
    print("Integer mode: 2.2 + 3.7 = " + z.to_string())  # 5
}

print("")
print("=== All Tests Passed ===")
```

### Example 3: Crypto Math (RSA Example)

```graphoid
# samples/crypto_math.gr

import "random"

print("=== Crypto Math: RSA Example ===")

# RSA requires very large integer arithmetic
precision { :high, :integer } {
    # Simulate RSA-sized numbers (simplified)
    # Real RSA would use 2048-bit primes

    # Large prime-like numbers
    p = 1234567890123456789012345678901234567890
    q = 9876543210987654321098765432109876543210

    # Modulus
    n = p * q
    print("Modulus (n): " + n.to_string())
    print("Bit length: " + n.bit_length().to_string())

    # Totient (simplified)
    phi = (p - 1) * (q - 1)

    # Public exponent
    e = 65537

    # Message
    message = 42

    # Encrypt: c = m^e mod n
    encrypted = (message ** e) % n
    print("Encrypted: " + encrypted.to_string())

    # In real RSA, we'd compute d = e^-1 mod phi
    # and decrypt with: m = c^d mod n

    print("BigNum enables cryptographic operations!")
}

print("")
print("=== All Tests Passed ===")
```

---

## Testing Strategy

### Unit Tests (Rust)

**Test file**: `tests/unit/bignum_tests.rs`

```rust
#[cfg(test)]
mod bignum_tests {
    use super::*;

    #[test]
    fn test_explicit_bignum_declaration() {
        let mut executor = Executor::new();
        let code = r#"
            bignum a = 999
            result = a
        "#;
        executor.execute_source(code).unwrap();
        let result = executor.env().get("result").unwrap();
        assert!(matches!(result.kind, ValueKind::BigNumber(_)));
    }

    #[test]
    fn test_large_literal_auto_detection() {
        let mut executor = Executor::new();
        let code = r#"
            # Too large for f64 precision
            huge = 99999999999999999999999999
        "#;
        executor.execute_source(code).unwrap();
        let huge = executor.env().get("huge").unwrap();
        assert!(matches!(huge.kind, ValueKind::BigNumber(BigNum::BigInt(_))));
    }

    #[test]
    fn test_high_precision_float() {
        let mut executor = Executor::new();
        let code = r#"
            precision { :high } {
                a = 2.5
                b = 3.7
                c = a + b
            }
        "#;
        executor.execute_source(code).unwrap();
        let c = executor.env().get("c").unwrap();
        assert!(matches!(c.kind, ValueKind::BigNumber(BigNum::Float128(_))));
    }

    #[test]
    fn test_high_precision_integer() {
        let mut executor = Executor::new();
        let code = r#"
            precision { :high, :integer } {
                a = 2
                b = 3
                c = a + b
            }
        "#;
        executor.execute_source(code).unwrap();
        let c = executor.env().get("c").unwrap();
        assert!(matches!(c.kind, ValueKind::BigNumber(BigNum::Int64(_))));
    }

    #[test]
    fn test_mixed_num_bignum_addition() {
        let mut executor = Executor::new();
        let code = r#"
            num a = 5
            bignum b = 10
            c = a + b
        "#;
        executor.execute_source(code).unwrap();

        // Result should be bignum (auto-cast)
        let c = executor.env().get("c").unwrap();
        assert!(matches!(c.kind, ValueKind::BigNumber(_)));

        // CRITICAL: Original 'a' must still be num (NOT mutated)
        let a = executor.env().get("a").unwrap();
        assert!(matches!(a.kind, ValueKind::Number(_)));
        assert_eq!(a.to_number().unwrap(), 5.0);
    }

    #[test]
    fn test_overflow_auto_promotion() {
        let mut executor = Executor::new();
        let code = r#"
            result = 1000 ** 1000
        "#;
        executor.execute_source(code).unwrap();
        let result = executor.env().get("result").unwrap();
        // Should auto-promote to bignum
        assert!(matches!(result.kind, ValueKind::BigNumber(_)));
    }

    #[test]
    fn test_bignum_to_num_conversion() {
        let mut executor = Executor::new();
        let code = r#"
            bignum big_val = 42
            small = big_val.to_num()
        "#;
        executor.execute_source(code).unwrap();
        let small = executor.env().get("small").unwrap();
        assert!(matches!(small.kind, ValueKind::Number(_)));
        assert_eq!(small.to_number().unwrap(), 42.0);
    }

    // ... 30+ more tests
}
```

### Integration Tests (.gr files)

**Test file**: `tests/integration/bignum_test.gr`

```graphoid
print("=== BigNum Integration Tests ===")
print("")

# Test 1: Explicit declaration
bignum a = 999
print("Test 1: Explicit declaration - " + a.type_name())  # "bignum"

# Test 2: Large literal
huge = 99999999999999999999999999
print("Test 2: Large literal - " + huge.type_name())  # "bignum"

# Test 3: High precision mode
precision { :high } {
    b = 2.5
    print("Test 3: High precision - " + b.type_name())  # "bignum"
}

# Test 4: Integer mode
precision { :high, :integer } {
    c = 2.5  # Truncated
    print("Test 4: Integer mode - " + c.to_string())  # "2"
}

# Test 5: Mixed operations
num x = 5
bignum y = 10
z = x * y
print("Test 5: Mixed operation - " + z.type_name())  # "bignum"

# Test 6: Conversion
bignum w = 42
num_w = w.to_num()
print("Test 6: Conversion - " + num_w.type_name())  # "num"

print("")
print("=== All Tests Passed ===")
```

---

## Dependencies Required

Add to `rust/Cargo.toml`:

```toml
[dependencies]
# ... existing dependencies ...

# BigNum support
num-bigint = "0.4"         # Arbitrary precision integers
num-traits = "0.2"         # Numeric trait abstractions
f128 = "0.2"               # 128-bit floats (or use "rug" for MPFR)

# Alternative (more features, but heavier):
# rug = "1.24"             # MPFR/GMP bindings (f128, BigDecimal, etc.)
```

**Decision**: Start with `f128` crate (lighter). Consider `rug` later if we need BigDecimal (arbitrary precision floats).

---

## Breaking Changes

**None!** This is purely additive:

- Existing code continues to work (uses standard `num`)
- `bignum` is a new type, opt-in
- Auto-promotion only affects operations that would otherwise overflow/lose precision
- No changes to existing `num` behavior

---

## Comparison with Original Plan

### What We're Keeping

✅ Native Rust implementation (not pure Graphoid)
✅ `BigNum` enum with Int64, UInt64, Float128, BigInt variants
✅ `PrecisionMode` configuration system
✅ Explicit `bignum` type declarations
✅ High-precision crypto support (SHA-512, RSA, etc.)

### What We're Changing

❌ **Mixing policy**: Allow num + bignum (auto-cast), not forbidden
❌ **Default mode**: Bignum is float by default, not integer-focused
❌ **Overflow behavior**: Auto-promote everywhere, not just in precision blocks
❌ **Literal detection**: Auto-detect large literals, not just in blocks
❌ **Directive syntax**: `:integer` as explicit directive, not implicit

### Why These Changes

1. **Seamless interop**: Forbidding num + bignum mixing is too restrictive
2. **Consistency**: Bignum should behave like num (float by default)
3. **User-friendly**: Auto-promotion reduces boilerplate
4. **Practical**: Large literals should "just work" without blocks

---

## Success Criteria

### Phase 1 Complete When:
- ✅ `bignum a = 2.5` works and stores as Float128
- ✅ `precision { :high } { ... }` forces bignum
- ✅ `precision { :high, :integer } { ... }` forces bignum integers
- ✅ All tests pass (40+ tests)

### Phase 2 Complete When:
- ✅ `1000 ** 1000` auto-promotes to bignum
- ✅ `num + bignum` works seamlessly
- ✅ Conversion methods work
- ✅ All tests pass (75+ tests)

### Phase 3 Complete When:
- ✅ BigInt arithmetic works for RSA-sized numbers
- ✅ Auto-growth from Int64 to BigInt on overflow
- ✅ Crypto math examples run successfully
- ✅ All tests pass (100+ tests)

### Phase 4 Complete When:
- ✅ Documentation complete in spec
- ✅ Example files demonstrate all features
- ✅ All .gr examples run successfully
- ✅ Zero regressions in existing tests

---

## Implementation Status Summary

**What Already Exists:**
- ✅ `BigNum` enum with `Int64`, `UInt64` variants (partial)
- ✅ `PrecisionMode` enum: `Standard`, `High`, `Extended`
- ✅ `unsigned_mode: bool` for bitwise operations
- ✅ Basic bignum arithmetic for Int64/UInt64 in `:high` mode
- ✅ Configuration system with precision parsing

**What Needs Implementation:**
- ❌ `:integer` directive (new `integer_mode: bool` field)
- ❌ Truncation-on-assignment logic when `integer_mode: true`
- ❌ `Float128` variant in `BigNum` enum
- ❌ `BigInt` variant in `BigNum` enum
- ❌ Float128 arithmetic operations
- ❌ Auto-promotion on overflow
- ❌ Mixed num/bignum operations
- ❌ Large literal detection
- ❌ Casting methods (`to_bignum()`, `fits_in_num()`, etc.)
- ❌ Dependencies (f128, num-bigint crates)

---

## Open Questions for Review

1. **Literal syntax for bignum?**
   - Option A: `123456789n` (JavaScript-style suffix)
   - Option B: Auto-detection only (current plan)
   - **Proposed**: Start with Option B, add suffix later if users want it

2. **Precision directive naming?**
   - Current: `precision { :high } { ... }`
   - Alternative: `precision { :bignum } { ... }`
   - **Proposed**: Keep `:high` (matches original plan, clearer intent)

3. **Should we implement BigDecimal (arbitrary precision floats) now?**
   - Pro: Complete arbitrary precision support
   - Con: More complex, slower, rarely needed
   - **Proposed**: Phase 2 (after basic bignum works)

4. **Auto-promotion threshold?**
   - When should we auto-promote f64 to bignum on overflow?
   - Always? Configurable? Only when `:high` active?
   - **Proposed**: Always auto-promote (can disable with `:standard` directive)

5. **Comparison semantics?**
   - Should `bignum == num` be allowed?
   - Current plan: Yes (auto-cast for comparison)
   - **Proposed**: Allow comparisons across types

6. **`:integer` truncation timing?**
   - **Confirmed**: Truncate on assignment (Option 1)
   - Values are truncated when assigned, not when operated on
   - This ensures variables actually contain whole numbers in `:integer` mode

---

## Next Steps

1. **Review this plan** with user for approval
2. **Start Phase 1** (Foundation - Float128 + Int64)
3. **Write tests first** (TDD - RED-GREEN-REFACTOR)
4. **Create .gr examples** for each feature as implemented
5. **Document as we go** (update spec in parallel)

---

**Estimated Total Time**: 15-22 days (3-4 weeks)

**Document Status**: Awaiting Review
**Last Updated**: November 17, 2025
