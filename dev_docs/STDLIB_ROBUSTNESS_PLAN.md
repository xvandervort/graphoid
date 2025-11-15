# Stdlib Robustness Implementation Plan
**Created**: November 14, 2025
**Status**: In Progress
**Goal**: Make Graphoid stdlib modules fully functional and usable for real-world applications

---

## Executive Summary

This document describes the plan to fix critical deficiencies in Graphoid's standard library modules, specifically:
1. **crypto.gr** - Currently a placeholder, must provide real cryptographic functions
2. **regex.gr** - Currently toy implementation, must support real regex patterns
3. **json.gr** - Number parsing bug (returns strings instead of numbers)

**Root Cause Analysis**: These issues stem from language feature gaps, not poor implementation:
- ❌ Missing character code primitives (`char_code()`, `to_char()`, `to_bytes()`)
- ✅ Bitwise operators already implemented (verified working)

**Philosophy**: "If it can be done in Graphoid, then it MUST be done in Graphoid." Native Rust modules should only be used when absolutely necessary (filesystem, network, OS interfaces).

---

## Issue #1: Character Code Primitives (Language Gap)

### Problem
Graphoid currently has NO way to convert between characters and their numeric codes. This makes it impossible to implement:
- Cryptographic hash functions (SHA-256, SHA-512)
- Hex encoding/decoding
- Base64 encoding/decoding
- Character-based algorithms

**Current workaround in crypto.gr (UNACCEPTABLE):**
```graphoid
# Only handles a-e!
if char == "a" { char_val = 97 }
if char == "b" { char_val = 98 }
if char == "c" { char_val = 99 }
if char == "d" { char_val = 100 }
if char == "e" { char_val = 101 }
```

### Solution: Add Character Code Methods

#### String Methods
```graphoid
# Get character code at index
code = "Hello".char_code(0)     # Returns 72 (ASCII 'H')
code = "A".char_code(0)          # Returns 65

# Get all character codes as list
bytes = "Hello".to_bytes()       # Returns [72, 101, 108, 108, 111]
```

#### Number Methods
```graphoid
# Convert number to character
char = (65).to_char()            # Returns "A"
char = (72).to_char()            # Returns "H"
```

#### List Methods
```graphoid
# Convert list of codes to string
text = [72, 101, 108, 108, 111].to_string()  # Returns "Hello"
```

### Implementation Steps

**1. Add methods to `src/values/string.rs`:**
```rust
// In StringMethods implementation
"char_code" => {
    // char_code(index) -> number
    match args.as_slice() {
        [index_val] => {
            let idx = index_val.as_number()? as usize;
            if idx >= s.len() {
                return Err(GraphoidError::runtime(
                    format!("Index {} out of bounds for string of length {}", idx, s.len())
                ));
            }
            let byte = s.as_bytes()[idx];
            Ok(Value::number(byte as f64))
        }
        _ => Err(GraphoidError::runtime("char_code() takes 1 argument"))
    }
}

"to_bytes" => {
    // to_bytes() -> list<num>
    match args.as_slice() {
        [] => {
            let bytes: Vec<Value> = s.bytes()
                .map(|b| Value::number(b as f64))
                .collect();
            Ok(Value::list(bytes))
        }
        _ => Err(GraphoidError::runtime("to_bytes() takes no arguments"))
    }
}
```

**2. Add method to `src/values/number.rs`:**
```rust
// In NumberMethods implementation
"to_char" => {
    // to_char() -> string
    match args.as_slice() {
        [] => {
            let code = n.trunc() as u32;
            if code > 127 {
                return Err(GraphoidError::runtime(
                    format!("Character code {} out of ASCII range (0-127)", code)
                ));
            }
            let ch = char::from_u32(code)
                .ok_or_else(|| GraphoidError::runtime(
                    format!("Invalid character code: {}", code)
                ))?;
            Ok(Value::string(ch.to_string()))
        }
        _ => Err(GraphoidError::runtime("to_char() takes no arguments"))
    }
}
```

**3. Enhance list `to_string()` to handle byte arrays:**
```rust
// In ListMethods implementation, update to_string()
"to_string" => {
    match args.as_slice() {
        [] => {
            // Check if this is a byte array (all numbers 0-127)
            let is_byte_array = items.iter().all(|v| {
                if let ValueKind::Number(n) = &v.kind {
                    let code = n.trunc() as i64;
                    code >= 0 && code <= 127
                } else {
                    false
                }
            });

            if is_byte_array {
                // Convert to string
                let chars: Result<String> = items.iter().map(|v| {
                    let code = v.as_number()? as u8;
                    Ok(char::from(code))
                }).collect();
                Ok(Value::string(chars?))
            } else {
                // Standard list stringification
                // ... existing code ...
            }
        }
        _ => Err(GraphoidError::runtime("to_string() takes no arguments"))
    }
}
```

**4. Add tests in `tests/unit/string_tests.rs`:**
```rust
#[test]
fn test_char_code() {
    let mut executor = Executor::new();
    executor.exec(r#"
        result = "Hello".char_code(0)
    "#).unwrap();
    assert_eq!(executor.get_var("result").unwrap().as_number().unwrap(), 72.0);
}

#[test]
fn test_to_bytes() {
    let mut executor = Executor::new();
    executor.exec(r#"
        result = "Hi".to_bytes()
    "#).unwrap();
    let list = executor.get_var("result").unwrap();
    // Should be [72, 105]
    assert_eq!(list.as_list().unwrap().len(), 2);
}

#[test]
fn test_to_char() {
    let mut executor = Executor::new();
    executor.exec(r#"
        result = (65).to_char()
    "#).unwrap();
    assert_eq!(executor.get_var("result").unwrap().as_string().unwrap(), "A");
}

#[test]
fn test_bytes_to_string() {
    let mut executor = Executor::new();
    executor.exec(r#"
        result = [72, 101, 108, 108, 111].to_string()
    "#).unwrap();
    assert_eq!(executor.get_var("result").unwrap().as_string().unwrap(), "Hello");
}
```

**5. Create integration test `samples/test_char_codes.gr`:**
```graphoid
print("=== Character Code Tests ===")
print("")

# char_code
print("'H'.char_code(0) = " + "H".char_code(0).to_string())
print("'A'.char_code(0) = " + "A".char_code(0).to_string())

# to_bytes
bytes = "Hello".to_bytes()
print("'Hello'.to_bytes() = " + bytes.to_string())

# to_char
print("(65).to_char() = " + (65).to_char())
print("(72).to_char() = " + (72).to_char())

# Round trip
original = "Test"
bytes = original.to_bytes()
reconstructed = bytes.to_string()
print("Original: " + original)
print("Bytes: " + bytes.to_string())
print("Reconstructed: " + reconstructed)
print("Match: " + (original == reconstructed).to_string())
```

**Estimated time**: 2-3 hours

---

## Issue #2: Bitwise Operators (Verification)

### Status: ✅ ALREADY IMPLEMENTED

Bitwise operators were implemented in Phase 13 and are **fully functional**:

**Operators available:**
- `&` - Bitwise AND
- `|` - Bitwise OR
- `^` - Bitwise XOR
- `~` - Bitwise NOT
- `<<` - Left shift
- `>>` - Right shift (arithmetic/signed by default)

**Verification test** (`/tmp/test_bitwise.gr`):
```graphoid
a = 12  # 1100 in binary
b = 5   # 0101 in binary

print("a & b = " + (a & b).to_string())  # 4  (0100)
print("a | b = " + (a | b).to_string())  # 13 (1101)
print("a ^ b = " + (a ^ b).to_string())  # 9  (1001)
print("~a = " + (~a).to_string())        # -13
print("a << 2 = " + (a << 2).to_string()) # 48
print("a >> 2 = " + (a >> 2).to_string()) # 3
```

**Result**: All operators work correctly! ✅

**Tasks**:
1. ✅ Verify bitwise operators work (DONE)
2. Run existing bitwise tests: `cargo test --lib bitwise`
3. No implementation needed - just validation

**Estimated time**: 15 minutes (verification only)

---

## Issue #3: JSON Number Parsing Bug

### Problem
The `parse_number()` function in `stdlib/json.gr` collects digits into a string but returns the string instead of converting to a number.

**Current code (lines 61-84):**
```graphoid
fn parse_number(text, pos) {
    result = ""
    # ... collect digits into result string ...

    # TODO: Need string-to-number conversion
    return [result, pos]  # ← Returns STRING not NUMBER
}
```

**Test showing bug:**
```graphoid
import "json"
data = '{"age": 30, "pi": 3.14}'
parsed = json.parse(data)
print(parsed.to_string())
# Result: {"age": "30", "pi": "3.14"}  ← Numbers are strings!
```

### Solution: Use `.to_num()` Method

**Fix** (update line 72 in `stdlib/json.gr`):
```graphoid
fn parse_number(text, pos) {
    result = ""
    size = text.size()

    # Collect sign if present
    if pos < size && text[pos] == "-" {
        result = result + "-"
        pos = pos + 1
    }

    # Collect digits
    while pos < size {
        char = text[pos]
        is_num_char = false

        if char == "0" || char == "1" || char == "2" || char == "3" || char == "4" ||
           char == "5" || char == "6" || char == "7" || char == "8" || char == "9" ||
           char == "." {
            is_num_char = true
        }

        if is_num_char {
            result = result + char
            pos = pos + 1
        } else {
            # FIX: Convert string to number using .to_num()
            num_value = result.to_num()
            if num_value == none {
                # Invalid number format - return 0
                num_value = 0
            }
            return [num_value, pos]  # ← Return NUMBER not STRING
        }
    }

    # End of string - convert final result
    num_value = result.to_num()
    if num_value == none {
        num_value = 0
    }
    return [num_value, pos]
}
```

**Test:**
```graphoid
import "json"

print("=== JSON Number Parsing Test ===")

# Test integers
data1 = '{"age": 30, "count": 100}'
parsed1 = json.parse(data1)
print("Age: " + parsed1["age"].to_string())
print("Age + 5 = " + (parsed1["age"] + 5).to_string())  # Should be 35

# Test floats
data2 = '{"pi": 3.14159, "e": 2.71828}'
parsed2 = json.parse(data2)
print("Pi: " + parsed2["pi"].to_string())
print("Pi * 2 = " + (parsed2["pi"] * 2).to_string())  # Should be 6.28318

# Test negative numbers
data3 = '{"temp": -5, "balance": -123.45}'
parsed3 = json.parse(data3)
print("Temp: " + parsed3["temp"].to_string())
print("Balance: " + parsed3["balance"].to_string())
```

**Estimated time**: 15 minutes

---

## Issue #4: Regex Enhancement (Make It Usable)

### Problem
Current `stdlib/regex.gr` is a **toy implementation** that cannot handle real-world regex patterns:

**Currently supports:**
- `.` (any character)
- `*`, `+`, `?` (quantifiers)
- `^`, `$` (anchors)

**DOES NOT support (critical missing features):**
- ❌ Character classes `[abc]`, `[a-z]`, `[0-9]`, `[^abc]`
- ❌ Escape sequences `\.`, `\*`, `\(`, `\)`
- ❌ Character shortcuts `\d` (digits), `\w` (word chars), `\s` (whitespace)
- ❌ Quantifiers `{n}`, `{n,m}`
- ❌ Grouping `(...)` and alternation `|`

**Real-world patterns that FAIL:**
```graphoid
regex.matches("[0-9]", "5")              # FAILS - tries to match literal "["
regex.matches("\\d+", "123")             # FAILS - \d not supported
regex.matches("example\\.com", "ex.com") # FAILS - no escape support
regex.matches("[a-z]+@[a-z]+", "test@ex")# FAILS - ranges not supported
```

### Solution: Implement Core Missing Features

We need to add:
1. **Character classes** with ranges: `[a-z]`, `[0-9]`, `[abc]`, `[^abc]`
2. **Escape sequences**: `\.`, `\*`, `\+`, `\(`, `\)`, `\[`, `\]`
3. **Character shortcuts**: `\d`, `\w`, `\s`, `\D`, `\W`, `\S`

**Note**: Full PCRE regex (grouping, alternation, backreferences, lookahead) can be Phase 15+ work. Priority is making basic patterns work.

### Implementation Plan

#### 1. Add Character Class Parsing

**Create helper function:**
```graphoid
# Parse character class [abc] or [a-z] or [^abc]
fn parse_char_class(pattern, pos) {
    pos = pos + 1  # Skip opening [
    negate = false

    if pos < pattern.size() && pattern[pos] == "^" {
        negate = true
        pos = pos + 1
    }

    chars = []

    while pos < pattern.size() && pattern[pos] != "]" {
        char = pattern[pos]

        # Check for range (a-z, 0-9, etc.)
        if pos + 2 < pattern.size() && pattern[pos + 1] == "-" && pattern[pos + 2] != "]" {
            start_code = char.char_code(0)
            end_code = pattern[pos + 2].char_code(0)

            code = start_code
            while code <= end_code {
                chars = chars.append(code.to_char())
                code = code + 1
            }

            pos = pos + 3  # Skip char, -, char
        } else {
            # Single character
            chars = chars.append(char)
            pos = pos + 1
        }
    }

    if pos >= pattern.size() {
        # Unclosed character class - error
        return [[], false, pos]
    }

    pos = pos + 1  # Skip closing ]
    return [chars, negate, pos]
}

# Check if char matches character class
fn char_matches_class(char, chars, negate) {
    found = false
    i = 0
    while i < chars.length() {
        if char == chars[i] {
            found = true
        }
        i = i + 1
    }

    if negate {
        return not found
    }
    return found
}
```

**Update `char_matches()` function:**
```graphoid
fn char_matches(pattern_char, text_char, char_class_info) {
    # If it's a character class
    if char_class_info != none {
        chars = char_class_info[0]
        negate = char_class_info[1]
        return char_matches_class(text_char, chars, negate)
    }

    # Special patterns
    if pattern_char == "." {
        return true
    }

    # Literal match
    return pattern_char == text_char
}
```

#### 2. Add Escape Sequence Handling

**Create helper function:**
```graphoid
# Handle escape sequences like \d, \w, \., \*
fn handle_escape(pattern, pos) {
    if pos + 1 >= pattern.size() {
        # No character after backslash
        return ["\\", pos + 1]
    }

    pos = pos + 1  # Skip backslash
    char = pattern[pos]

    # Character shortcuts
    if char == "d" {
        # \d = [0-9]
        return [["0","1","2","3","4","5","6","7","8","9"], pos + 1]
    } else if char == "D" {
        # \D = [^0-9] (negated)
        return [[["0","1","2","3","4","5","6","7","8","9"], true], pos + 1]
    } else if char == "w" {
        # \w = [a-zA-Z0-9_]
        chars = make_word_chars()
        return [chars, pos + 1]
    } else if char == "W" {
        # \W = [^a-zA-Z0-9_]
        chars = make_word_chars()
        return [[chars, true], pos + 1]
    } else if char == "s" {
        # \s = [ \t\n\r]
        return [[" ", "\t", "\n", "\r"], pos + 1]
    } else if char == "S" {
        # \S = [^ \t\n\r]
        return [[[" ", "\t", "\n", "\r"], true], pos + 1]
    } else {
        # Escaped special character - treat as literal
        # \., \*, \+, \(, \), etc.
        return [char, pos + 1]
    }
}

# Helper: Create word character list [a-zA-Z0-9_]
fn make_word_chars() {
    chars = []

    # a-z
    code = 97
    while code <= 122 {
        chars = chars.append(code.to_char())
        code = code + 1
    }

    # A-Z
    code = 65
    while code <= 90 {
        chars = chars.append(code.to_char())
        code = code + 1
    }

    # 0-9
    code = 48
    while code <= 57 {
        chars = chars.append(code.to_char())
        code = code + 1
    }

    # Underscore
    chars = chars.append("_")

    return chars
}
```

**Update `match_at()` to handle escapes:**
```graphoid
fn match_at(pattern, pattern_pos, text, text_pos) {
    # ... existing code ...

    pattern_char = pattern[pattern_pos]

    # Check for escape sequence
    if pattern_char == "\\" {
        escape_info = handle_escape(pattern, pattern_pos)
        # ... handle escape pattern ...
    }

    # Check for character class
    if pattern_char == "[" {
        class_info = parse_char_class(pattern, pattern_pos)
        # ... handle character class ...
    }

    # ... rest of matching logic ...
}
```

#### 3. Comprehensive Testing

**Create test file `samples/test_regex_enhanced.gr`:**
```graphoid
import "regex"

print("=== Enhanced Regex Tests ===")
print("")

# Character classes
print("Character Classes:")
print("[abc] matches 'b': " + regex.matches("[abc]", "b").to_string())
print("[abc] matches 'x': " + regex.matches("[abc]", "x").to_string())
print("[a-z] matches 'm': " + regex.matches("[a-z]", "m").to_string())
print("[0-9] matches '5': " + regex.matches("[0-9]", "5").to_string())
print("[^0-9] matches 'a': " + regex.matches("[^0-9]", "a").to_string())
print("[^0-9] matches '5': " + regex.matches("[^0-9]", "5").to_string())
print("")

# Escape sequences
print("Escape Sequences:")
print("\\d+ matches '123': " + regex.matches("\\d+", "123").to_string())
print("\\d+ matches 'abc': " + regex.matches("\\d+", "abc").to_string())
print("\\w+ matches 'hello': " + regex.matches("\\w+", "hello").to_string())
print("\\s+ matches '   ': " + regex.matches("\\s+", "   ").to_string())
print("example\\.com matches 'example.com': " + regex.matches("example\\.com", "example.com").to_string())
print("example\\.com matches 'exampleXcom': " + regex.matches("example\\.com", "exampleXcom").to_string())
print("")

# Real-world patterns
print("Real-World Patterns:")
email = "[a-z]+@[a-z]+\\.[a-z]+"
print("Email pattern matches 'test@example.com': " + regex.matches(email, "test@example.com").to_string())

phone = "\\d{3}-\\d{4}"  # Note: {n} quantifiers need implementation
# print("Phone pattern matches '555-1234': " + regex.matches(phone, "555-1234").to_string())

# Find operations
print("")
print("Find Operations:")
digits = regex.find("\\d+", "age: 25 years")
print("Find digits in 'age: 25 years': " + digits)

all_digits = regex.find_all("\\d+", "room 101, floor 3")
print("Find all digits in 'room 101, floor 3': " + all_digits.to_string())

# Replace operations
print("")
print("Replace Operations:")
replaced = regex.replace("\\d+", "version 2.0", "X")
print("Replace digits in 'version 2.0': " + replaced)
```

**Estimated time**: 6-8 hours

---

## Issue #5: Crypto Module Rewrite (Pure Graphoid)

### Current Status: UNACCEPTABLE
File header states: "WARNING: This is a simplified implementation for educational purposes. Do NOT use for real cryptographic applications!"

**All functions are non-functional stubs.**

### Goal: Implement Real Cryptography in Pure Graphoid

With character code primitives and bitwise operators, we can implement:
1. **SHA-256 hashing** - Real cryptographic hash
2. **HMAC-SHA256** - Message authentication
3. **Ed25519 signatures** - Public key cryptography
4. **AES-256** - Symmetric encryption
5. **Hex encoding/decoding** - Utility functions

### Implementation: SHA-256 in Pure Graphoid

**Complete SHA-256 implementation** (following FIPS 180-4 standard):

```graphoid
# crypto.gr - REAL CRYPTOGRAPHIC IMPLEMENTATIONS
# All algorithms implemented in pure Graphoid using bitwise operators
# and character code primitives

import "random"

# ==============================================================================
# SHA-256 IMPLEMENTATION
# ==============================================================================

# SHA-256 constants (first 32 bits of fractional parts of cube roots of first 64 primes)
fn sha256_k() {
    return [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
    ]
}

# Initial hash values (first 32 bits of fractional parts of square roots of first 8 primes)
fn sha256_h0() {
    return [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
    ]
}

# Right rotate
fn rotr(value, bits) {
    return ((value >> bits) | (value << (32 - bits))) & 0xffffffff
}

# SHA-256 main function
fn sha256(message) {
    # Convert string to bytes
    bytes = message.to_bytes()

    # Pre-processing: add padding
    padded = sha256_pad(bytes)

    # Initialize hash values
    H = sha256_h0()
    K = sha256_k()

    # Process message in 512-bit (64-byte) chunks
    i = 0
    while i < padded.length() {
        chunk = []
        j = 0
        while j < 64 {
            chunk = chunk.append(padded[i + j])
            j = j + 1
        }

        H = sha256_process_chunk(chunk, H, K)
        i = i + 64
    }

    # Produce final hash
    return sha256_hash_to_hex(H)
}

# Pad message according to SHA-256 spec
fn sha256_pad(bytes) {
    # Message length in bits
    msg_len = bytes.length() * 8

    # Append 1 bit (0x80 byte)
    padded = bytes
    padded = padded.append(0x80)

    # Pad with zeros until length ≡ 448 (mod 512) bits
    # That's 56 bytes mod 64 bytes
    while (padded.length() % 64) != 56 {
        padded = padded.append(0)
    }

    # Append length as 64-bit big-endian integer
    # For simplicity, we only support messages < 2^32 bits
    padded = padded.append(0).append(0).append(0).append(0)  # High 32 bits = 0
    padded = padded.append((msg_len >> 24) & 0xff)
    padded = padded.append((msg_len >> 16) & 0xff)
    padded = padded.append((msg_len >> 8) & 0xff)
    padded = padded.append(msg_len & 0xff)

    return padded
}

# Process one 512-bit chunk
fn sha256_process_chunk(chunk, H, K) {
    # Create message schedule W[0..63]
    W = []

    # First 16 words are from the chunk (big-endian)
    i = 0
    while i < 16 {
        word = (chunk[i*4] << 24) | (chunk[i*4 + 1] << 16) |
               (chunk[i*4 + 2] << 8) | chunk[i*4 + 3]
        W = W.append(word & 0xffffffff)
        i = i + 1
    }

    # Extend the first 16 words into remaining 48
    i = 16
    while i < 64 {
        s0 = rotr(W[i-15], 7) ^ rotr(W[i-15], 18) ^ (W[i-15] >> 3)
        s1 = rotr(W[i-2], 17) ^ rotr(W[i-2], 19) ^ (W[i-2] >> 10)
        W = W.append((W[i-16] + s0 + W[i-7] + s1) & 0xffffffff)
        i = i + 1
    }

    # Initialize working variables
    a = H[0]
    b = H[1]
    c = H[2]
    d = H[3]
    e = H[4]
    f = H[5]
    g = H[6]
    h = H[7]

    # Main loop
    i = 0
    while i < 64 {
        S1 = rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25)
        ch = (e & f) ^ ((~e) & g)
        temp1 = (h + S1 + ch + K[i] + W[i]) & 0xffffffff
        S0 = rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22)
        maj = (a & b) ^ (a & c) ^ (b & c)
        temp2 = (S0 + maj) & 0xffffffff

        h = g
        g = f
        f = e
        e = (d + temp1) & 0xffffffff
        d = c
        c = b
        b = a
        a = (temp1 + temp2) & 0xffffffff

        i = i + 1
    }

    # Add compressed chunk to hash values
    return [
        (H[0] + a) & 0xffffffff,
        (H[1] + b) & 0xffffffff,
        (H[2] + c) & 0xffffffff,
        (H[3] + d) & 0xffffffff,
        (H[4] + e) & 0xffffffff,
        (H[5] + f) & 0xffffffff,
        (H[6] + g) & 0xffffffff,
        (H[7] + h) & 0xffffffff
    ]
}

# Convert hash to hex string
fn sha256_hash_to_hex(H) {
    hex = ""
    hex_chars = "0123456789abcdef"

    i = 0
    while i < 8 {
        word = H[i]

        # Convert to 8 hex digits (32 bits)
        j = 7
        while j >= 0 {
            nibble = (word >> (j * 4)) & 0xf
            hex = hex + hex_chars[nibble]
            j = j - 1
        }

        i = i + 1
    }

    return hex
}

# ==============================================================================
# HEX ENCODING/DECODING
# ==============================================================================

fn encode_hex(data) {
    hex = ""
    hex_chars = "0123456789abcdef"
    bytes = data.to_bytes()

    i = 0
    while i < bytes.length() {
        byte = bytes[i]
        high = (byte >> 4) & 0xf
        low = byte & 0xf
        hex = hex + hex_chars[high] + hex_chars[low]
        i = i + 1
    }

    return hex
}

fn decode_hex(hex_string) {
    bytes = []
    i = 0

    while i < hex_string.length() - 1 {
        high = hex_char_to_num(hex_string[i])
        low = hex_char_to_num(hex_string[i + 1])
        byte = (high << 4) | low
        bytes = bytes.append(byte)
        i = i + 2
    }

    return bytes.to_string()
}

fn hex_char_to_num(char) {
    code = char.char_code(0)

    # 0-9
    if code >= 48 && code <= 57 {
        return code - 48
    }

    # a-f
    if code >= 97 && code <= 102 {
        return code - 97 + 10
    }

    # A-F
    if code >= 65 && code <= 70 {
        return code - 65 + 10
    }

    return 0
}

# ==============================================================================
# KEY GENERATION
# ==============================================================================

fn generate_key(length) {
    # Use native cryptographically secure random
    return random.token(length)
}

# ==============================================================================
# FUTURE: Ed25519, AES-256, HMAC
# ==============================================================================
# These require more complex implementations but follow same pattern
# using bitwise operators and character codes
```

**Testing:**
```graphoid
import "crypto"

print("=== Crypto Module Tests ===")
print("")

# Test SHA-256
print("SHA-256 Tests:")
hash1 = crypto.sha256("")
print("SHA-256('') = " + hash1)
print("Expected:     e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
print("Match: " + (hash1 == "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855").to_string())
print("")

hash2 = crypto.sha256("abc")
print("SHA-256('abc') = " + hash2)
print("Expected:        ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad")
print("Match: " + (hash2 == "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad").to_string())
print("")

# Test hex encoding
print("Hex Encoding:")
hex = crypto.encode_hex("Hello")
print("encode_hex('Hello') = " + hex)
decoded = crypto.decode_hex(hex)
print("decode_hex('" + hex + "') = " + decoded)
print("Match: " + (decoded == "Hello").to_string())
```

**Estimated time**: 8-12 hours for full SHA-256 + hex + testing

**Note**: Ed25519 and AES-256 are more complex and can be Phase 13+ work. SHA-256 proves the concept.

---

## Implementation Timeline

### Phase 1: Foundation (3-4 hours)
1. ✅ Verify bitwise operators (15 min) - DONE
2. ⏳ Implement character code primitives (2-3 hours)
   - Add `char_code()`, `to_char()`, `to_bytes()` methods
   - Write unit tests
   - Create integration test

### Phase 2: Quick Wins (30 min)
3. ⏳ Fix JSON number parsing (15 min)
4. ⏳ Test JSON fix (15 min)

### Phase 3: Regex Enhancement (6-8 hours)
5. ⏳ Implement character classes `[abc]`, `[a-z]`, `[^abc]` (2-3 hours)
6. ⏳ Implement escape sequences `\.`, `\d`, `\w`, `\s` (2-3 hours)
7. ⏳ Test enhanced regex (2 hours)

### Phase 4: Crypto Implementation (8-12 hours)
8. ⏳ Implement SHA-256 in pure Graphoid (6-8 hours)
9. ⏳ Implement hex encoding/decoding (1-2 hours)
10. ⏳ Test crypto module (2 hours)

### Total Estimated Time: 18-25 hours

**Milestones:**
- After Phase 1+2: JSON works, char codes available
- After Phase 3: Regex usable for real patterns
- After Phase 4: Cryptography works without native code

---

## Success Criteria

### JSON Module
✅ Parse numbers correctly (not as strings)
✅ Integer, float, and negative number support
✅ All JSON tests pass

### Regex Module
✅ Character classes work: `[a-z]`, `[0-9]`, `[^abc]`
✅ Escape sequences work: `\.`, `\*`, `\d`, `\w`, `\s`
✅ Real-world patterns work: email, phone, URL validation
✅ All regex tests pass

### Crypto Module
✅ SHA-256 produces correct hashes (test against known vectors)
✅ Hex encoding/decoding works correctly
✅ No "educational only" warnings
✅ All crypto tests pass
✅ **100% pure Graphoid implementation** (no Rust)

### Language Completeness
✅ Character code primitives documented in spec
✅ All stdlib modules work from .gr programs
✅ No critical language gaps remain

---

## Future Enhancements (Phase 13+)

1. **Advanced Regex Features**:
   - Grouping `(...)`
   - Alternation `|`
   - Backreferences `\1`, `\2`
   - Lookahead/lookbehind
   - Quantifiers `{n}`, `{n,m}`

2. **Additional Crypto Algorithms**:
   - Ed25519 signatures
   - AES-256-GCM encryption
   - HMAC-SHA256
   - PBKDF2 key derivation
   - Argon2 password hashing

3. **Binary/Hex Literals**:
   - Already in spec: `0b1010`, `0xFF`
   - Makes crypto code more readable

---

## Appendix: Language Spec Updates

After implementation, update `LANGUAGE_SPECIFICATION.md`:

### Section: String Methods
Add:
```markdown
- `char_code(index)` - Returns numeric code of character at index (0-127 for ASCII)
- `to_bytes()` - Returns list of character codes
```

### Section: Number Methods
Add:
```markdown
- `to_char()` - Converts number (0-127) to single-character string
```

### Section: List Methods
Update `to_string()`:
```markdown
- `to_string()` - Converts to string representation. If list contains only numbers 0-127,
  interprets as byte array and converts to string.
```

### Section: Stdlib - Crypto Module
Update to reflect pure Graphoid implementation:
```markdown
All cryptographic functions implemented in pure Graphoid using bitwise operators
and character code primitives. No native Rust code required.
```

---

---

## CRITICAL UPDATE (November 14, 2025)

**Priority Changed**: Discovered that production crypto module requires high-precision numeric types.

**New Plan**: Implement BigNum type FIRST, then crypto module.

See `dev_docs/BIGNUM_PRECISION_PLAN.md` for complete details.

**Revised Implementation Order:**
1. ✅ **Phase 1-2 COMPLETE**: Character codes, JSON fixes, bitwise verification
2. ✅ **Phase 3 COMPLETE**: Regex enhancement (character classes, escapes, shortcuts)
3. **NEW Phase 4**: BigNum type and precision modes (3-4 weeks)
4. **Phase 5+**: Crypto module implementation (7-8 weeks)

**Rationale**: Cannot implement SHA-512, RSA, or other modern crypto without exact 64-bit integer arithmetic and arbitrary precision integers. Pure Graphoid multi-precision library would be too slow and unreadable.

**Solution**: Add user-facing `bignum` type with configuration-based precision modes:
- `configure { precision: :high }` → i64/u64/f128
- `configure { precision: :extended }` → arbitrary precision (BigInt)

---

**Document Status**: Superseded by BIGNUM_PRECISION_PLAN.md for crypto work
**Last Updated**: November 14, 2025
