# BigNum Type and Precision Modes Implementation Plan
**Created**: November 14, 2025
**Status**: Ready to Implement
**Priority**: Critical - Blocks production crypto module

---

## Executive Summary

This document describes the implementation of **high-precision numeric types** in Graphoid to support cryptographic operations and other use cases requiring precision beyond standard f64.

**Key Decision**: BigNum becomes a **user-facing type** (not hidden as originally planned) because values must persist when exiting precision configuration blocks.

**Approach**: Configuration-based precision control using `configure { precision: :high }` and `configure { precision: :extended }` blocks, following Graphoid's philosophy of **configuration over types**.

---

## Rationale

### Problem
Standard Graphoid numbers are f64 (IEEE 754 double precision):
- **Integer precision**: 53 bits (safe range: -2^53 to 2^53)
- **Float precision**: ~15-17 decimal digits

**This is insufficient for:**
- SHA-512 (requires exact 64-bit integer arithmetic)
- RSA cryptography (requires 2048-4096 bit integers)
- Ed25519 signatures (requires 255-bit integers)
- High-precision scientific computing (needs > 64-bit floats)
- Financial calculations requiring exact decimal arithmetic

### Why Not Add i64/u64/i128 Types?
**That's not the Graphoid way.** We don't proliferate types like Rust/Java. Instead:
- Use **configuration blocks** to change behavior
- Single `bignum` type handles all high-precision cases
- Precision mode determines interpretation

### Alternative Rejected: Pure Graphoid Multi-Precision Library
Could implement 64-bit and arbitrary precision using lists:
```graphoid
# Horrible!
result = add64([high1, low1], [high2, low2])
```

**Problems:**
- ❌ Unreadable - crypto code becomes list manipulation
- ❌ Slow - 50-100x slower than native operations
- ❌ Error-prone - easy to mix up high/low, miss carries
- ❌ Hard to verify - doesn't match algorithm specifications

---

## User-Facing Design

### New Type: `bignum`

```graphoid
# BigNum values are created in precision blocks
configure { precision: :high } {
    large = 9007199254740992  # Exceeds f64 precision
    result = large + 1         # Exact 64-bit arithmetic
}

# BigNum persists outside the block
print(result.type_name())  # "bignum"
print(result)              # Full precision maintained

# Convert back to num if desired
standard = result.to_num()  # May lose precision, returns none if overflow
```

### Type System Integration

**Type checking:**
```graphoid
bignum x = 12345678901234567890  # BigNum literal
num y = 123.45                    # Standard number

# Type inference works
value = some_crypto_function()    # Returns bignum
```

**Methods on bignum:**
```graphoid
x = bignum_value

# All standard numeric methods
x.to_string()
x.to_num()        # Convert to f64 (may lose precision or return none)
x.to_bytes()      # Get byte representation
x.bit_length()    # Number of bits needed to represent
x.abs()
x.sign()
```

### Configuration Modes

#### Mode 1: Standard (Default)
```graphoid
# No configuration block - standard f64 behavior
result = a + b  # 53-bit integer precision
```

#### Mode 2: High Precision (:high)
```graphoid
configure { precision: :high } {
    # 64-bit integers (i64/u64) OR 128-bit floats (f128)

    # Integer operations (when operands are whole numbers)
    int_result = 9223372036854775807 + 1  # Exact i64 math

    # Float operations (when operands have decimal points)
    float_result = 1.23456789012345678901234567890 * 2.0  # f128 precision

    # Bitwise operations (always integer)
    bits = int_result >> 32
}

# Can combine with unsigned mode
configure { precision: :high, :unsigned } {
    # Uses u64 for integers instead of i64
    unsigned_result = 0xFFFFFFFFFFFFFFFF + 1  # Wraps to 0
}
```

**Precision guarantees:**
- **Integers**: Full 64-bit range (-2^63 to 2^63-1 signed, 0 to 2^64-1 unsigned)
- **Floats**: f128 (~34 decimal digits precision)

#### Mode 3: Extended Precision (:extended)
```graphoid
configure { precision: :extended } {
    # Arbitrary precision integers (BigInt)
    # Automatically grows to accommodate any size

    # RSA-sized numbers (2048 bits)
    prime = 2^2048 - 1
    result = prime ** exponent % modulus

    # Cryptographic operations
    signature = pow_mod(message, private_key, modulus)
}
```

**Precision guarantees:**
- **Integers**: Unlimited (only constrained by memory)
- **Operations**: Exact (no precision loss)

### Automatic Type Promotion

```graphoid
# Standard arithmetic
a = 100        # num (f64)
b = 200        # num (f64)
c = a + b      # num (f64)

# High precision
configure { precision: :high } {
    d = a + b  # bignum (i64) - operands promoted
    e = c * 2  # bignum - result from block stays bignum
}

# Extended precision
configure { precision: :extended } {
    huge = 2 ** 1000  # bignum (BigInt)
}

# Mixed operations (outside config blocks)
result = d + a  # ERROR: Cannot mix bignum and num without explicit conversion
# Must do:
result = d.to_num() + a  # Convert to num
# OR
result = d + a.to_bignum()  # Convert to bignum (if .to_bignum() is added)
```

---

## Internal Implementation

### ValueKind Extension

```rust
// In src/values/mod.rs
pub enum ValueKind {
    None,
    Boolean(bool),
    Number(f64),          // Standard precision
    BigNumber(BigNum),    // High/extended precision
    String(String),
    Symbol(String),
    List(List),
    Hash(Hash),
    Function(Function),
    Graph(Graph),
    Tree(Tree),
}

pub enum BigNum {
    Int64(i64),           // :high mode with integers
    UInt64(u64),          // :high mode with :unsigned
    Float128(f128),       // :high mode with floats (uses f128 crate)
    Arbitrary(BigInt),    // :extended mode (uses num-bigint crate)
}
```

### Configuration System

```rust
// In src/execution/config.rs
pub struct Config {
    // ...existing fields...

    // Numeric precision
    pub precision_mode: PrecisionMode,
}

pub enum PrecisionMode {
    Standard,    // f64 (default)
    High,        // i64/u64/f128
    Extended,    // BigInt
}
```

### Arithmetic Operations

**Current (executor.rs):**
```rust
BinaryOp::Add => {
    if let (ValueKind::Number(a), ValueKind::Number(b)) = (&left.kind, &right.kind) {
        Ok(Value::number(a + b))
    } else {
        // Type error
    }
}
```

**Enhanced:**
```rust
BinaryOp::Add => {
    match (&left.kind, &right.kind) {
        // Standard + Standard
        (ValueKind::Number(a), ValueKind::Number(b)) => {
            // Check precision mode
            match self.config().precision_mode {
                PrecisionMode::Standard => Ok(Value::number(a + b)),
                PrecisionMode::High => {
                    // Promote to i64 or f128 based on whether values are integers
                    if a.fract() == 0.0 && b.fract() == 0.0 {
                        let result = (*a as i64).checked_add(*b as i64)
                            .ok_or_else(|| GraphoidError::runtime("Integer overflow in :high precision mode"))?;
                        Ok(Value::bignum(BigNum::Int64(result)))
                    } else {
                        let result = f128::from(*a) + f128::from(*b);
                        Ok(Value::bignum(BigNum::Float128(result)))
                    }
                },
                PrecisionMode::Extended => {
                    let result = BigInt::from(*a as i64) + BigInt::from(*b as i64);
                    Ok(Value::bignum(BigNum::Arbitrary(result)))
                },
            }
        },

        // BigNumber + BigNumber
        (ValueKind::BigNumber(a), ValueKind::BigNumber(b)) => {
            match (a, b) {
                (BigNum::Int64(x), BigNum::Int64(y)) => {
                    let result = x.checked_add(*y)
                        .ok_or_else(|| GraphoidError::runtime("Integer overflow"))?;
                    Ok(Value::bignum(BigNum::Int64(result)))
                },
                (BigNum::Arbitrary(x), BigNum::Arbitrary(y)) => {
                    Ok(Value::bignum(BigNum::Arbitrary(x + y)))
                },
                // Handle mixed BigNum types, f128, etc.
                _ => {
                    // Promote to common type
                    // ...
                }
            }
        },

        // Mixed Number + BigNumber
        (ValueKind::Number(_), ValueKind::BigNumber(_)) |
        (ValueKind::BigNumber(_), ValueKind::Number(_)) => {
            Err(GraphoidError::type_error(
                "Cannot mix num and bignum without explicit conversion"
            ))
        },

        _ => Err(GraphoidError::type_error(...))
    }
}
```

### Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
# For :extended precision
num-bigint = "0.4"
num-traits = "0.2"

# For :high precision f128
f128 = "0.2"  # or "rug" for MPFR-based f128
```

---

## Implementation Phases

### Phase 1: Infrastructure (Week 1)
**Goal**: Add bignum type, :high precision for integers

**Tasks:**
1. ✅ Add `BigNum` enum to `ValueKind`
2. ✅ Add `PrecisionMode` to config system
3. ✅ Implement `configure { precision: :high }` parsing
4. ✅ Update arithmetic operators (+, -, *, /, %) for i64/u64
5. ✅ Update bitwise operators (&, |, ^, ~, <<, >>) for i64/u64
6. ✅ Add `bignum.to_string()`, `bignum.to_num()` methods
7. ✅ Add `bignum` type checking and inference
8. ✅ Write TDD tests (30+ tests)

**Success criteria:**
- Can do exact 64-bit integer arithmetic
- SHA-512 can be implemented using `:high` precision
- All tests pass

**Time estimate**: 4-5 days

### Phase 2: Extended Precision (Week 2)
**Goal**: Add :extended precision for arbitrary integers

**Tasks:**
1. ✅ Integrate `num-bigint` crate
2. ✅ Implement `configure { precision: :extended }`
3. ✅ Update arithmetic operators for BigInt
4. ✅ Add `bignum.bit_length()` method
5. ✅ Add modular exponentiation helper (for RSA)
6. ✅ Write TDD tests (20+ tests)

**Success criteria:**
- Can do 2048-bit RSA arithmetic
- Arbitrary precision math works correctly
- All tests pass

**Time estimate**: 3-4 days

### Phase 3: High-Precision Floats (Week 3)
**Goal**: Add f128 support for :high precision floats

**Tasks:**
1. ✅ Integrate f128 crate
2. ✅ Detect integer vs float in `:high` mode
3. ✅ Implement f128 arithmetic
4. ✅ Add f128 conversion methods
5. ✅ Write TDD tests (15+ tests)

**Success criteria:**
- Can do 128-bit floating point math
- Scientific computing use cases supported
- All tests pass

**Time estimate**: 2-3 days

### Phase 4: Documentation & Examples (Week 4)
**Goal**: Document bignum type and create examples

**Tasks:**
1. ✅ Update `LANGUAGE_SPECIFICATION.md` with bignum type
2. ✅ Document precision modes
3. ✅ Create example files in `samples/`:
   - `bignum_demo.gr`
   - `high_precision_math.gr`
   - `rsa_math_demo.gr`
4. ✅ Update type system documentation

**Success criteria:**
- Spec clearly explains bignum type
- Examples demonstrate all precision modes
- Users understand when/how to use bignum

**Time estimate**: 2-3 days

---

## Updated Crypto Module Plan

With bignum support, crypto implementation becomes straightforward:

### Phase 5: Crypto Foundation (Week 5)
**Tasks:**
1. Hex/Base64/Base32 encoding (uses existing string/byte operations)
2. MD5 implementation (32-bit operations)
3. SHA-1 implementation (32-bit operations)
4. SHA-256 implementation (32-bit operations)
5. HMAC-SHA256

**Time estimate**: 4-5 days

### Phase 6: 64-bit Crypto (Week 6)
**Tasks:**
1. SHA-512 using `configure { precision: :high }`
2. SHA-384 (truncated SHA-512)
3. BLAKE2b using `:high` precision
4. HMAC-SHA512

**Time estimate**: 3-4 days

### Phase 7: Asymmetric Crypto (Weeks 7-8)
**Tasks:**
1. Modular exponentiation helpers using `:extended` precision
2. RSA key generation (2048-bit)
3. RSA signatures (PKCS#1)
4. Basic Ed25519 (using 255-bit arithmetic)
5. Diffie-Hellman key exchange

**Time estimate**: 8-10 days

### Phase 8: Block Ciphers (Week 9)
**Tasks:**
1. AES-128 implementation
2. AES-256 implementation
3. ChaCha20 cipher
4. CBC and GCM modes

**Time estimate**: 5-6 days

### Phase 9: Password Hashing (Week 10)
**Tasks:**
1. PBKDF2 implementation
2. bcrypt implementation
3. scrypt implementation
4. Test vectors and security audit

**Time estimate**: 4-5 days

**Total crypto implementation: ~7-8 weeks after bignum support**

---

## Type System Impact

### Language Specification Updates

**Section: Basic Types**
```markdown
- `none` - Absence of value
- `bool` - Boolean (true/false)
- `num` - Number (IEEE 754 double precision, f64)
- `bignum` - High-precision number (i64/u64/f128/BigInt) **[NEW]**
- `string` - Text
- `symbol` - Named constant (e.g., :error)
- `list` - Ordered collection
- `hash` - Key-value map
- `function` - Function reference
- `graph` - Graph structure
- `tree` - Tree structure
```

**Section: Type Conversion**
```markdown
# Converting between num and bignum
x = 12345678901234567890.to_bignum()  # num → bignum (explicit)
y = bignum_value.to_num()             # bignum → num (may lose precision)

# Automatic promotion in precision blocks
configure { precision: :high } {
    result = 100 + 200  # Automatically creates bignum
}
```

**Section: Precision Modes**
```markdown
## Precision Configuration

Graphoid supports three precision modes for numeric operations:

### Standard Precision (default)
Uses IEEE 754 double precision (f64):
- Integer precision: 53 bits
- Float precision: ~15-17 decimal digits
- Type: `num`

### High Precision (:high)
Uses 64-bit integers or 128-bit floats:
- Integer precision: 64 bits (i64/u64)
- Float precision: ~34 decimal digits (f128)
- Type: `bignum`

Usage:
```graphoid
configure { precision: :high } {
    # Exact 64-bit integer math
    result = 9223372036854775807 + 1

    # High-precision floating point
    precise = 1.23456789012345678901234567890 * 2.0
}
```

### Extended Precision (:extended)
Uses arbitrary precision integers:
- Integer precision: Unlimited (memory constrained)
- Type: `bignum`

Usage:
```graphoid
configure { precision: :extended } {
    # RSA-sized arithmetic
    huge = 2 ** 2048
    result = huge % modulus
}
```

**Note on bignum type**: The `bignum` type was added to support cryptographic operations
and other use cases requiring precision beyond standard f64. Values created in precision
blocks retain their type when exiting the block.
```

### Breaking Changes

**None!** This is purely additive:
- Existing code continues to work (uses standard precision)
- No changes to existing numeric behavior
- Opt-in via configuration blocks

---

## Testing Strategy

### Unit Tests (Rust)

**Test file**: `tests/unit/bignum_tests.rs`

```rust
#[test]
fn test_high_precision_integer_arithmetic() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            # Exact 64-bit addition
            a = 9223372036854775806
            result = a + 1
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(BigNum::Int64(n)) => {
            assert_eq!(*n, 9223372036854775807i64);
        },
        _ => panic!("Expected BigNum::Int64"),
    }
}

#[test]
fn test_extended_precision_arbitrary() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :extended } {
            result = 2 ** 1000
        }
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    match &result.kind {
        ValueKind::BigNumber(BigNum::Arbitrary(_)) => {
            // Success
        },
        _ => panic!("Expected BigNum::Arbitrary"),
    }
}

#[test]
fn test_mixed_num_bignum_error() {
    let mut executor = Executor::new();

    let code = r#"
        configure { precision: :high } {
            a = 100
        }
        b = 50
        result = a + b  # Should error - mixing types
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Cannot mix num and bignum"));
}
```

### Integration Tests (.gr files)

**Test file**: `tests/integration/bignum_test.gr`

```graphoid
print("=== BigNum Type Tests ===")
print("")

# Test 1: High precision integers
configure { precision: :high } {
    large = 9223372036854775806
    result = large + 1
    print("High precision: " + result.to_string())
    print("Type: " + result.type_name())
}

# Test 2: Extended precision
configure { precision: :extended } {
    huge = 2 ** 256
    print("2^256 = " + huge.to_string())
}

# Test 3: Persistence outside blocks
print("Result still bignum outside block: " + result.type_name())

print("")
print("=== All BigNum Tests Passed ===")
```

---

## Documentation Examples

### Example 1: SHA-512 Implementation Snippet

```graphoid
# SHA-512 requires exact 64-bit arithmetic
configure { precision: :high, :unsigned } {
    fn rotate_right(value, bits) {
        return (value >> bits) | (value << (64 - bits))
    }

    fn sha512_round(a, b, c, d, e, f, g, h, k, w) {
        s1 = rotate_right(e, 14) ^ rotate_right(e, 18) ^ rotate_right(e, 41)
        ch = (e & f) ^ ((~e) & g)
        temp1 = h + s1 + ch + k + w

        s0 = rotate_right(a, 28) ^ rotate_right(a, 34) ^ rotate_right(a, 39)
        maj = (a & b) ^ (a & c) ^ (b & c)
        temp2 = s0 + maj

        return [temp1 + temp2, temp1]
    }
}
```

### Example 2: RSA Math

```graphoid
import "crypto"

# Generate RSA keys
configure { precision: :extended } {
    # Pick two large primes (simplified)
    p = crypto.generate_prime(1024)
    q = crypto.generate_prime(1024)

    # Calculate modulus
    n = p * q  # 2048-bit number

    # Calculate totient
    phi = (p - 1) * (q - 1)

    # Public exponent
    e = 65537

    # Private exponent (modular inverse)
    d = crypto.mod_inverse(e, phi)

    print("Public key: (n=" + n.to_string() + ", e=" + e.to_string() + ")")
}
```

### Example 3: High-Precision Science

```graphoid
# Calculate pi to 30 decimal places
configure { precision: :high } {
    # Use Machin's formula: pi/4 = 4*arctan(1/5) - arctan(1/239)
    pi = calculate_pi_machin(1000)  # 1000 iterations
    print("Pi = " + pi.to_string())  # Full f128 precision
}
```

---

## Security Considerations

### Constant-Time Operations (Future Work)

```graphoid
configure { precision: :high, constant_time: true } {
    # All operations guaranteed constant-time
    # For timing-attack resistant crypto
    equals = constant_time_compare(signature, expected)
}
```

**Not in initial implementation** - requires careful runtime support.

### Overflow Handling

```rust
// In :high mode, overflow is an error
let result = a.checked_add(b)
    .ok_or_else(|| GraphoidError::runtime("Integer overflow"))?;

// In :extended mode, never overflows (grows as needed)
```

---

## Performance Characteristics

### Expected Performance

| Mode | Type | Operation | Speed vs f64 |
|------|------|-----------|--------------|
| Standard | f64 | Arithmetic | 1x (baseline) |
| High | i64 | Integer ops | 0.8-1.0x (native) |
| High | u64 | Integer ops | 0.8-1.0x (native) |
| High | f128 | Float ops | 2-5x slower |
| Extended | BigInt | Small ints | 10-20x slower |
| Extended | BigInt | Large ints (2048-bit) | 50-200x slower |

**Conclusion**: Performance hit is acceptable for crypto and scientific computing where correctness matters more than speed.

---

## Migration Path for Existing Code

**No migration needed!** All existing code continues to work:

```graphoid
# Existing code - unchanged
result = 100 + 200  # Still uses f64

# New capability - opt-in
configure { precision: :high } {
    precise = 100 + 200  # Uses i64
}
```

---

## Open Questions

1. **Should we add `.to_bignum()` method on numbers?**
   - Pro: Symmetric with `.to_num()`
   - Con: Which precision mode? Need argument?
   - Proposed: `x.to_bignum(:high)` or `x.to_bignum(:extended)`

2. **Literal syntax for bignum?**
   - Option A: `123456789012345678901234567890n` (JavaScript style)
   - Option B: Rely on precision blocks only
   - Proposed: Start with Option B, add literals later if needed

3. **Comparison across types?**
   ```graphoid
   big = some_bignum_value
   small = 100

   if big > small { ... }  # Allow or error?
   ```
   - Proposed: Allow comparison, but not arithmetic mixing

4. **String to bignum parsing?**
   ```graphoid
   "12345678901234567890".to_bignum()  # Should this work?
   ```
   - Proposed: Yes, add this method

---

## Success Criteria

✅ **Phase 1 Complete When:**
- Can perform exact 64-bit integer arithmetic in `:high` mode
- SHA-512 can be implemented cleanly
- Type system correctly handles bignum
- All tests pass (50+ tests)

✅ **Phase 2 Complete When:**
- Can perform arbitrary precision arithmetic in `:extended` mode
- RSA math (2048-bit) works correctly
- All tests pass (70+ tests)

✅ **Phase 3 Complete When:**
- Can perform f128 floating point in `:high` mode
- Scientific computing examples work
- All tests pass (85+ tests)

✅ **Full Implementation Complete When:**
- Documentation updated in spec
- Example files demonstrate all use cases
- Crypto module can be built on top of bignum
- Zero regressions in existing tests

---

**Next Steps**: Implement Phase 1 (bignum infrastructure + :high precision for integers)

**Estimated Total Time**: 3-4 weeks for complete bignum support, then 7-8 weeks for full crypto module

**Document Status**: Ready for implementation
**Last Updated**: November 14, 2025
