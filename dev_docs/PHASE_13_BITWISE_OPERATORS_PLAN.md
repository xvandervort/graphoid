# Phase 13: Bitwise Operators & Power Operator Change

**Date**: November 12, 2025
**Duration**: 5-7 days
**Status**: Planning
**Priority**: HIGH - Unblocks self-hosting for crypto/compression/hashing

---

## Overview

Add bitwise operators to Graphoid while maintaining the single `num` type (f64). This enables implementing crypto, compression, and other algorithmic modules in pure Graphoid instead of temporary Rust implementations.

**Key Design Decision**: No new types (`int`, `uint`) - bitwise operators work on existing `num` type with configurable signed/unsigned interpretation.

---

## Operator Changes

### Power Operator: `^` → `**`

**Breaking Change**: The `^` operator changes meaning:

```graphoid
# OLD (Phases 0-12): ^ means power
x = 2 ^ 8  # 256

# NEW (Phase 13+): ** means power, ^ means XOR
x = 2 ** 8  # 256 (power)
y = 0b1010 ^ 0b0101  # 15 (XOR)
```

**Rationale**: Standard in most languages (Python, Ruby, etc.) - `**` for power, `^` for XOR

---

## New Bitwise Operators

All operators work on `num` type, treating values as 64-bit integers:

| Operator | Name | Example | Result |
|----------|------|---------|--------|
| `&` | Bitwise AND | `12 & 10` | `8` (0b1100 & 0b1010 = 0b1000) |
| `\|` | Bitwise OR | `12 \| 10` | `14` (0b1100 \| 0b1010 = 0b1110) |
| `^` | Bitwise XOR | `12 ^ 10` | `6` (0b1100 ^ 0b1010 = 0b0110) |
| `~` | Bitwise NOT | `~5` | `-6` (two's complement) |
| `<<` | Left shift | `3 << 2` | `12` (0b11 → 0b1100) |
| `>>` | Right shift | `12 >> 2` | `3` (0b1100 → 0b11) |

### Signed vs Unsigned Behavior

**Default: Signed 64-bit (i64)**
```graphoid
# Arithmetic right shift (sign-extending)
x = -8 >> 1  # -4 (bits: 11111000 → 11111100, sign extends)

# NOT uses two's complement
y = ~5  # -6
```

**Unsigned Mode: Unsigned 64-bit (u64)**
```graphoid
configure { :unsigned } {
    # Logical right shift (zero-fill)
    x = -8 >> 1  # Very large positive number

    # NOT treats as unsigned
    y = ~0  # 0xFFFFFFFFFFFFFFFF (all bits set)
}
```

---

## New Numeric Literals

### Binary Literals

```graphoid
x = 0b1010  # 10
y = 0b11111111  # 255
mask = 0b1100_1010  # 202 (underscores for readability)
```

### Hexadecimal Literals

```graphoid
x = 0xFF  # 255
y = 0xDEADBEEF  # 3735928559
mask = 0xFF_00_FF  # Underscores allowed
```

**Note**: Both literal types produce regular `num` values (f64), not special integer types.

---

## Configure Block Syntax

### Unsigned Mode

```graphoid
# Default is signed (i64 interpretation)
x = -8 >> 1  # -4

# Unsigned interpretation
configure { :unsigned } {
    x = -8 >> 1  # Large positive number (logical shift)
    mask = ~0  # 0xFFFFFFFFFFFFFFFF
}

# Back to signed
y = -8 >> 1  # -4
```

### Precision Blocks (Must Follow Unsigned Behavior)

Since `precision` blocks are syntactic sugar on `configure`, they MUST respect unsigned mode:

```graphoid
configure { :unsigned } {
    precision 0 {
        # Both unsigned AND integer display
        x = 0xFF >> 4  # 15 (unsigned shift, integer display)
    }
}
```

**Implementation Note**: Precision context must inherit and preserve unsigned flag.

---

## Operator Precedence

Updated precedence table (lower number = lower precedence):

```
1.  ||                      # Logical OR
2.  &&                      # Logical AND
3.  ==, !=, <, >, <=, >=   # Comparison
4.  |                       # Bitwise OR
5.  ^                       # Bitwise XOR (NEW - was power!)
6.  &                       # Bitwise AND
7.  <<, >>                  # Bit shifts
8.  +, -                    # Addition, Subtraction
9.  *, /, //, %            # Multiplication, Division, Integer Div, Modulo
10. **                      # Power (NEW operator!)
11. ~, !, - (unary)        # Bitwise NOT, Logical NOT, Negation
```

**Key Changes**:
- `^` moves from precedence 10 (power) to 5 (bitwise XOR, between OR and AND)
- `**` added at precedence 10 (right-associative: `2 ** 3 ** 2` = `2 ** (3 ** 2)` = 512)

---

## Implementation Plan

### Day 1-2: Lexer Changes

**File**: `rust/src/lexer/token.rs`, `rust/src/lexer/mod.rs`

#### 1. Add New Tokens

```rust
pub enum TokenType {
    // ... existing tokens

    // NEW: Power operator changes
    DoubleStar,      // ** (power, replaces ^)

    // NEW: Bitwise operators
    Ampersand,       // &
    Pipe,            // |
    Caret,           // ^ (now XOR, not power!)
    Tilde,           // ~
    LeftShift,       // <<
    RightShift,      // >>

    // ... rest
}
```

**Update**: Remove `TokenType::Caret` from power, add new `DoubleStar`

#### 2. Binary Literal Parsing

```rust
fn tokenize_number(&mut self) -> Result<Token> {
    // Check for 0b prefix (binary)
    if self.current_char() == '0' && self.peek_char() == Some('b') {
        self.advance(); // consume '0'
        self.advance(); // consume 'b'
        return self.parse_binary_literal();
    }

    // Check for 0x prefix (hex)
    if self.current_char() == '0' && self.peek_char() == Some('x') {
        self.advance(); // consume '0'
        self.advance(); // consume 'x'
        return self.parse_hex_literal();
    }

    // ... existing decimal parsing
}

fn parse_binary_literal(&mut self) -> Result<Token> {
    let mut value = 0i64;
    let mut has_digits = false;

    while let Some(ch) = self.current_char() {
        match ch {
            '0' | '1' => {
                value = value * 2 + (ch.to_digit(2).unwrap() as i64);
                has_digits = true;
                self.advance();
            }
            '_' => { self.advance(); } // Skip underscores
            _ => break,
        }
    }

    if !has_digits {
        return Err(GraphoidError::LexerError {
            message: "Invalid binary literal".to_string(),
            position: self.current_position(),
        });
    }

    Ok(Token::new(TokenType::Number(value as f64), self.current_position()))
}

fn parse_hex_literal(&mut self) -> Result<Token> {
    let mut value = 0i64;
    let mut has_digits = false;

    while let Some(ch) = self.current_char() {
        match ch {
            '0'..='9' | 'a'..='f' | 'A'..='F' => {
                value = value * 16 + (ch.to_digit(16).unwrap() as i64);
                has_digits = true;
                self.advance();
            }
            '_' => { self.advance(); }
            _ => break,
        }
    }

    if !has_digits {
        return Err(GraphoidError::LexerError {
            message: "Invalid hex literal".to_string(),
            position: self.current_position(),
        });
    }

    Ok(Token::new(TokenType::Number(value as f64), self.current_position()))
}
```

#### 3. Operator Tokenization

```rust
fn scan_token(&mut self) -> Result<Token> {
    let ch = self.advance();

    match ch {
        // ... existing cases

        // NEW: ** for power
        '*' => {
            if self.match_char('*') {
                Ok(Token::new(TokenType::DoubleStar, start_pos))
            } else {
                Ok(Token::new(TokenType::Star, start_pos))
            }
        }

        // NEW: Bitwise operators
        '&' => Ok(Token::new(TokenType::Ampersand, start_pos)),
        '|' => Ok(Token::new(TokenType::Pipe, start_pos)),
        '^' => Ok(Token::new(TokenType::Caret, start_pos)), // NOW XOR!
        '~' => Ok(Token::new(TokenType::Tilde, start_pos)),

        // NEW: Shifts
        '<' => {
            if self.match_char('<') {
                Ok(Token::new(TokenType::LeftShift, start_pos))
            } else if self.match_char('=') {
                Ok(Token::new(TokenType::LessEqual, start_pos))
            } else {
                Ok(Token::new(TokenType::Less, start_pos))
            }
        }

        '>' => {
            if self.match_char('>') {
                Ok(Token::new(TokenType::RightShift, start_pos))
            } else if self.match_char('=') {
                Ok(Token::new(TokenType::GreaterEqual, start_pos))
            } else {
                Ok(Token::new(TokenType::Greater, start_pos))
            }
        }

        // ... rest
    }
}
```

#### Tests (20+ tests)

**File**: `rust/tests/unit/lexer_tests.rs`

```rust
#[test]
fn test_binary_literals() {
    assert_token("0b1010", TokenType::Number(10.0));
    assert_token("0b11111111", TokenType::Number(255.0));
    assert_token("0b1100_1010", TokenType::Number(202.0)); // With underscores
}

#[test]
fn test_hex_literals() {
    assert_token("0xFF", TokenType::Number(255.0));
    assert_token("0xDEADBEEF", TokenType::Number(3735928559.0));
    assert_token("0xFF_00_FF", TokenType::Number(16711935.0)); // With underscores
}

#[test]
fn test_power_operator() {
    let tokens = tokenize("2 ** 8").unwrap();
    assert_eq!(tokens[1].token_type, TokenType::DoubleStar);
}

#[test]
fn test_bitwise_operators() {
    assert_token("&", TokenType::Ampersand);
    assert_token("|", TokenType::Pipe);
    assert_token("^", TokenType::Caret);
    assert_token("~", TokenType::Tilde);
    assert_token("<<", TokenType::LeftShift);
    assert_token(">>", TokenType::RightShift);
}

#[test]
fn test_shift_vs_comparison() {
    // Ensure << doesn't conflict with < <
    let tokens = tokenize("x << 2").unwrap();
    assert_eq!(tokens[1].token_type, TokenType::LeftShift);

    let tokens = tokenize("x < <2").unwrap(); // Space prevents <<
    assert_eq!(tokens[1].token_type, TokenType::Less);
}
```

---

### Day 3: Parser Changes

**File**: `rust/src/parser/mod.rs`, `rust/src/ast/nodes.rs`

#### 1. Update Binary Operators

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // ... existing ops

    // CHANGED: Power now uses **
    Power,           // ** (was ^)

    // NEW: Bitwise operators
    BitwiseAnd,      // &
    BitwiseOr,       // |
    BitwiseXor,      // ^ (moved from Power!)
    LeftShift,       // <<
    RightShift,      // >>
}
```

#### 2. Update Unary Operators

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,          // -
    Not,             // !
    BitwiseNot,      // ~ (NEW)
}
```

#### 3. Update Precedence

```rust
fn precedence(&self, token: &TokenType) -> u8 {
    match token {
        // Logical
        TokenType::Or => 10,
        TokenType::And => 20,

        // Comparison
        TokenType::Equal | TokenType::NotEqual => 30,
        TokenType::Less | TokenType::Greater
        | TokenType::LessEqual | TokenType::GreaterEqual => 30,

        // Bitwise
        TokenType::Pipe => 40,           // | (bitwise OR)
        TokenType::Caret => 45,          // ^ (bitwise XOR)
        TokenType::Ampersand => 50,      // & (bitwise AND)
        TokenType::LeftShift | TokenType::RightShift => 55,

        // Arithmetic
        TokenType::Plus | TokenType::Minus => 60,
        TokenType::Star | TokenType::Slash
        | TokenType::DoubleSlash | TokenType::Percent => 70,

        // Power (right-associative handled separately)
        TokenType::DoubleStar => 80,     // **

        _ => 0,
    }
}

fn is_right_associative(&self, token: &TokenType) -> bool {
    matches!(token, TokenType::DoubleStar) // ** is right-associative
}
```

#### 4. Update Parsing

```rust
fn parse_binary_expression(&mut self, min_precedence: u8) -> Result<Expr> {
    let mut left = self.parse_unary_expression()?;

    while let Some(token) = self.current_token() {
        let op_precedence = self.precedence(&token.token_type);
        if op_precedence < min_precedence {
            break;
        }

        let op = self.token_to_binary_op(&token.token_type)?;
        self.advance();

        // Handle right-associativity for **
        let next_precedence = if self.is_right_associative(&token.token_type) {
            op_precedence  // Same precedence for right-associative
        } else {
            op_precedence + 1  // Higher for left-associative
        };

        let right = self.parse_binary_expression(next_precedence)?;
        left = Expr::Binary { left: Box::new(left), op, right: Box::new(right) };
    }

    Ok(left)
}

fn parse_unary_expression(&mut self) -> Result<Expr> {
    if let Some(token) = self.current_token() {
        match token.token_type {
            TokenType::Minus => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Negate,
                    operand: Box::new(self.parse_unary_expression()?)
                })
            }
            TokenType::Bang => {
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::Not,
                    operand: Box::new(self.parse_unary_expression()?)
                })
            }
            TokenType::Tilde => { // NEW
                self.advance();
                Ok(Expr::Unary {
                    op: UnaryOp::BitwiseNot,
                    operand: Box::new(self.parse_unary_expression()?)
                })
            }
            _ => self.parse_primary_expression(),
        }
    } else {
        self.parse_primary_expression()
    }
}

fn token_to_binary_op(&self, token: &TokenType) -> Result<BinaryOp> {
    match token {
        // ... existing ops
        TokenType::DoubleStar => Ok(BinaryOp::Power),       // NEW
        TokenType::Ampersand => Ok(BinaryOp::BitwiseAnd),   // NEW
        TokenType::Pipe => Ok(BinaryOp::BitwiseOr),         // NEW
        TokenType::Caret => Ok(BinaryOp::BitwiseXor),       // NEW (was Power!)
        TokenType::LeftShift => Ok(BinaryOp::LeftShift),    // NEW
        TokenType::RightShift => Ok(BinaryOp::RightShift),  // NEW
        _ => Err(GraphoidError::ParserError {
            message: format!("Not a binary operator: {:?}", token),
        }),
    }
}
```

#### Tests (25+ tests)

**File**: `rust/tests/unit/parser_tests.rs`

```rust
#[test]
fn test_power_operator_precedence() {
    // 2 ** 3 ** 2 = 2 ** (3 ** 2) = 2 ** 9 = 512 (right-associative)
    let expr = parse_expr("2 ** 3 ** 2");
    // Verify AST structure shows right-associativity
}

#[test]
fn test_bitwise_precedence() {
    // 2 + 3 & 4 = (2 + 3) & 4 = 5 & 4 = 4
    let expr = parse_expr("2 + 3 & 4");
    // Verify + has higher precedence than &

    // 12 | 3 ^ 5 = 12 | (3 ^ 5) = 12 | 6 = 14
    let expr = parse_expr("12 | 3 ^ 5");
    // Verify ^ has higher precedence than |
}

#[test]
fn test_shift_precedence() {
    // 1 << 2 + 3 = 1 << (2 + 3) = 1 << 5 = 32
    let expr = parse_expr("1 << 2 + 3");
    // Verify + has higher precedence than <<
}

#[test]
fn test_bitwise_not_unary() {
    let expr = parse_expr("~5");
    assert!(matches!(expr, Expr::Unary { op: UnaryOp::BitwiseNot, .. }));
}

#[test]
fn test_binary_literals_parse() {
    let expr = parse_expr("0b1010");
    // Verify produces Number literal with value 10
}

#[test]
fn test_hex_literals_parse() {
    let expr = parse_expr("0xFF");
    // Verify produces Number literal with value 255
}
```

---

### Day 4-5: Execution Engine

**File**: `rust/src/execution/executor.rs`, `rust/src/values/mod.rs`

#### 1. Add Unsigned Configuration

```rust
// In Executor
pub struct Executor {
    // ... existing fields
    unsigned_mode: bool,  // NEW: track unsigned interpretation
}

impl Executor {
    pub fn new() -> Self {
        Self {
            // ... existing fields
            unsigned_mode: false,  // Default: signed
        }
    }

    fn push_unsigned_mode(&mut self, unsigned: bool) {
        // Save current mode, set new mode
        // Will need a stack if we support nested configure blocks
    }

    fn pop_unsigned_mode(&mut self) {
        // Restore previous mode
    }
}
```

#### 2. Handle Configure Blocks

```rust
fn execute_configure_block(&mut self, settings: &ConfigureSettings, body: &[Stmt]) -> Result<Value> {
    // Check for :unsigned symbol in settings
    let unsigned = settings.flags.contains(&Symbol::new("unsigned"));

    if unsigned {
        self.push_unsigned_mode(true);
    }

    let result = self.execute_block(body);

    if unsigned {
        self.pop_unsigned_mode();
    }

    result
}
```

#### 3. Implement Bitwise Operations

```rust
fn eval_binary_op(&mut self, left: Value, op: BinaryOp, right: Value) -> Result<Value> {
    match op {
        // ... existing ops

        BinaryOp::Power => {
            // ** operator (was ^)
            let l = left.to_number()?;
            let r = right.to_number()?;
            Ok(Value::number(l.powf(r)))
        }

        BinaryOp::BitwiseAnd => {
            let l = self.value_to_i64(&left)?;
            let r = self.value_to_i64(&right)?;
            Ok(Value::number((l & r) as f64))
        }

        BinaryOp::BitwiseOr => {
            let l = self.value_to_i64(&left)?;
            let r = self.value_to_i64(&right)?;
            Ok(Value::number((l | r) as f64))
        }

        BinaryOp::BitwiseXor => {
            let l = self.value_to_i64(&left)?;
            let r = self.value_to_i64(&right)?;
            Ok(Value::number((l ^ r) as f64))
        }

        BinaryOp::LeftShift => {
            let l = self.value_to_i64(&left)?;
            let r = right.to_number()? as u32;

            if r >= 64 {
                return Err(GraphoidError::RuntimeError {
                    message: format!("Shift amount {} too large (max 63)", r),
                });
            }

            Ok(Value::number((l << r) as f64))
        }

        BinaryOp::RightShift => {
            let r_shift = right.to_number()? as u32;

            if r_shift >= 64 {
                return Err(GraphoidError::RuntimeError {
                    message: format!("Shift amount {} too large (max 63)", r_shift),
                });
            }

            if self.unsigned_mode {
                // Logical shift (zero-fill)
                let l = self.value_to_u64(&left)?;
                Ok(Value::number((l >> r_shift) as f64))
            } else {
                // Arithmetic shift (sign-extend)
                let l = self.value_to_i64(&left)?;
                Ok(Value::number((l >> r_shift) as f64))
            }
        }
    }
}

fn eval_unary_op(&mut self, op: UnaryOp, operand: Value) -> Result<Value> {
    match op {
        // ... existing ops

        UnaryOp::BitwiseNot => {
            let val = self.value_to_i64(&operand)?;
            Ok(Value::number((!val) as f64))
        }
    }
}

// Helper conversions
fn value_to_i64(&self, val: &Value) -> Result<i64> {
    let num = val.to_number()?;

    // Truncate to i64 (handles fractional parts)
    Ok(num.trunc() as i64)
}

fn value_to_u64(&self, val: &Value) -> Result<u64> {
    let num = val.to_number()?;

    // Convert to u64 (reinterpret bits)
    let as_i64 = num.trunc() as i64;
    Ok(as_i64 as u64)
}
```

#### Tests (40+ tests)

**File**: `rust/tests/unit/bitwise_tests.rs`

```rust
#[test]
fn test_bitwise_and() {
    assert_eq!(execute("12 & 10"), Value::number(8.0));
    assert_eq!(execute("0xFF & 0x0F"), Value::number(15.0));
}

#[test]
fn test_bitwise_or() {
    assert_eq!(execute("12 | 10"), Value::number(14.0));
    assert_eq!(execute("0xF0 | 0x0F"), Value::number(255.0));
}

#[test]
fn test_bitwise_xor() {
    assert_eq!(execute("12 ^ 10"), Value::number(6.0));
    assert_eq!(execute("0xFF ^ 0xFF"), Value::number(0.0));
}

#[test]
fn test_bitwise_not() {
    assert_eq!(execute("~0"), Value::number(-1.0));
    assert_eq!(execute("~5"), Value::number(-6.0));
}

#[test]
fn test_left_shift() {
    assert_eq!(execute("1 << 3"), Value::number(8.0));
    assert_eq!(execute("0xFF << 8"), Value::number(65280.0));
}

#[test]
fn test_right_shift_signed() {
    // Default: signed (arithmetic shift)
    assert_eq!(execute("-8 >> 1"), Value::number(-4.0));
    assert_eq!(execute("-16 >> 2"), Value::number(-4.0));
}

#[test]
fn test_right_shift_unsigned() {
    let result = execute(r#"
        configure { :unsigned } {
            x = -8 >> 1
            return x
        }
    "#);
    // Should be large positive number (logical shift)
    assert!(result.to_number().unwrap() > 0.0);
}

#[test]
fn test_power_operator() {
    assert_eq!(execute("2 ** 8"), Value::number(256.0));
    assert_eq!(execute("2 ** 3 ** 2"), Value::number(512.0)); // Right-associative
}

#[test]
fn test_binary_literals() {
    assert_eq!(execute("0b1010"), Value::number(10.0));
    assert_eq!(execute("0b11111111"), Value::number(255.0));
}

#[test]
fn test_hex_literals() {
    assert_eq!(execute("0xFF"), Value::number(255.0));
    assert_eq!(execute("0xDEADBEEF"), Value::number(3735928559.0));
}

#[test]
fn test_complex_bitwise_expressions() {
    // Test precedence
    assert_eq!(execute("2 + 3 & 7"), Value::number(5.0)); // (2+3) & 7 = 5 & 7 = 5
    assert_eq!(execute("8 | 4 ^ 2"), Value::number(14.0)); // 8 | (4^2) = 8 | 6 = 14
}

#[test]
fn test_shift_overflow() {
    // Shifting by >= 64 should error
    let result = execute_with_error("1 << 64");
    assert!(result.is_err());
}
```

---

### Day 6: Integration Tests & Samples

**File**: `rust/tests/integration/bitwise_test.gr`

```graphoid
# Integration test for bitwise operators

# Basic operations
mask = 0xFF
result = mask & 0x0F
print(result)  # Should print 15

# XOR for toggling
x = 0b1010
y = x ^ 0b1111
print(y)  # Should print 5 (0b0101)

# Shifts
value = 1 << 8
print(value)  # Should print 256

# Power operator
power = 2 ** 10
print(power)  # Should print 1024

# Unsigned mode
configure { :unsigned } {
    # Logical shift
    x = -1 >> 1
    print(x)  # Should print large positive number
}
```

**File**: `rust/samples/bitwise_operations.gr`

```graphoid
# Bitwise Operations Example

import "io"

io.print("=== Bitwise Operations ===")

# Binary and hex literals
binary = 0b1010
hex = 0xFF
io.print("Binary 0b1010 = ${binary}")
io.print("Hex 0xFF = ${hex}")

# Basic operations
io.print("\n=== Basic Operations ===")
a = 12  # 0b1100
b = 10  # 0b1010

io.print("${a} & ${b} = ${a & b}")  # 8 (0b1000)
io.print("${a} | ${b} = ${a | b}")  # 14 (0b1110)
io.print("${a} ^ ${b} = ${a ^ b}")  # 6 (0b0110)
io.print("~${a} = ${~a}")           # -13

# Shifts
io.print("\n=== Shift Operations ===")
value = 5
io.print("${value} << 2 = ${value << 2}")  # 20
io.print("${value} >> 1 = ${value >> 1}")  # 2

# Signed vs unsigned
io.print("\n=== Signed (default) ===")
negative = -8
io.print("${negative} >> 1 = ${negative >> 1}")  # -4 (arithmetic)

io.print("\n=== Unsigned Mode ===")
configure { :unsigned } {
    io.print("${negative} >> 1 = ${negative >> 1}")  # Large positive (logical)
}

# Power operator
io.print("\n=== Power Operator ===")
io.print("2 ** 8 = ${2 ** 8}")           # 256
io.print("2 ** 3 ** 2 = ${2 ** 3 ** 2}") # 512 (right-associative)

# Practical: bit flags
io.print("\n=== Bit Flags Example ===")
READ = 0b001
WRITE = 0b010
EXECUTE = 0b100

permissions = READ | WRITE
io.print("Permissions: ${permissions}")
io.print("Can read: ${(permissions & READ) != 0}")
io.print("Can execute: ${(permissions & EXECUTE) != 0}")
```

---

### Day 7: Documentation & Examples

#### 1. Update Language Specification

Add bitwise operators section to `dev_docs/LANGUAGE_SPECIFICATION.md`:
- Operator reference table
- Signed vs unsigned behavior
- Configure block syntax
- Binary/hex literal syntax
- Precedence rules
- Examples

#### 2. Update Roadmap

Mark Phase 13 complete in `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md`

#### 3. Create Examples

- `rust/samples/crypto_primitives.gr` - Simple hash function using bitwise ops
- `rust/samples/bit_manipulation.gr` - Common bit tricks
- `rust/samples/flags_and_masks.gr` - Bit flags pattern

#### 4. Migration Guide

**File**: `dev_docs/PHASE_13_MIGRATION_GUIDE.md`

Document breaking change (` ^` → `**` for power) with migration examples.

---

## Success Criteria

### Functional Requirements

- ✅ All 6 bitwise operators (`&`, `|`, `^`, `~`, `<<`, `>>`) working
- ✅ Power operator changed from `^` to `**`
- ✅ Binary literals (`0b...`) parse and execute correctly
- ✅ Hex literals (`0x...`) parse and execute correctly
- ✅ Signed mode (default): arithmetic right shift
- ✅ Unsigned mode: logical right shift via `configure { :unsigned }`
- ✅ Correct operator precedence
- ✅ Right-associativity for `**`

### Test Coverage

- ✅ 20+ lexer tests (literals, operators)
- ✅ 25+ parser tests (precedence, associativity)
- ✅ 40+ execution tests (operations, signed/unsigned)
- ✅ 3+ integration tests (`.gr` files)
- ✅ Total: 88+ new tests

### Quality

- ✅ Zero compiler warnings
- ✅ All existing tests still pass (no regressions)
- ✅ Clear error messages for edge cases (shift overflow, etc.)
- ✅ Examples run successfully

### Documentation

- ✅ Language spec updated
- ✅ Roadmap updated
- ✅ Migration guide written
- ✅ Sample files created and commented

---

## Dependencies Unlocked

Once Phase 13 is complete, the following can be implemented in **pure Graphoid**:

1. **Crypto Module** - SHA-256, HMAC, encryption (no Rust needed!)
2. **Random Module** - ChaCha20 RNG in pure Graphoid
3. **Compression** - LZ77, Huffman coding
4. **UUID Generation** - UUID v4 using bitwise ops
5. **Hash Functions** - Custom hash algorithms
6. **Network Protocols** - Packet manipulation, checksums

**This achieves the 90%+ self-hosting goal!**

---

## Breaking Changes

### Power Operator: `^` → `**`

**Impact**: Any existing code using `^` for power will break.

**Migration**:
```graphoid
# OLD
x = 2 ^ 8

# NEW
x = 2 ** 8
```

**Timeline**: Phase 13 is early enough that impact should be minimal (no production code yet).

---

## Open Questions

1. **Should underscores in literals be mandatory or optional?**
   - Proposed: Optional (like Rust, Python)
   - `0b11111111` and `0b1111_1111` both valid

2. **Shift amount validation: error or saturate?**
   - Proposed: Error for shifts >= 64
   - Matches Rust behavior, prevents surprises

3. **Configure block nesting: does unsigned propagate?**
   - Proposed: Yes, inner blocks inherit outer unsigned mode
   - Can be overridden explicitly

---

## Risk Assessment

**Low Risk**:
- Bitwise operations are well-understood
- No new types, minimal type system impact
- Clear precedence rules from other languages

**Medium Risk**:
- Breaking change to `^` operator
- Mitigation: Early in development, minimal code affected
- Migration is trivial (find/replace)

**Testing Strategy**:
- Comprehensive unit tests (88+ tests)
- Integration tests with `.gr` files
- Property-based tests for bitwise identities:
  - `x & x == x`
  - `x | x == x`
  - `x ^ x == 0`
  - `x & 0 == 0`
  - `x | ~0 == ~0`

---

## Timeline Summary

| Day | Focus | Deliverable | Tests |
|-----|-------|-------------|-------|
| 1-2 | Lexer | Binary/hex literals, operator tokens | 20+ |
| 3 | Parser | Precedence, AST nodes | 25+ |
| 4-5 | Executor | Operations, unsigned mode | 40+ |
| 6 | Integration | `.gr` tests, samples | 3+ |
| 7 | Documentation | Spec, migration guide, examples | - |

**Total Duration**: 5-7 days
**Total New Tests**: 88+
**Breaking Changes**: 1 (power operator)

---

## Next Steps After Phase 13

With bitwise operators complete:

1. **Assert Module** (1 day) - Test infrastructure for samples
2. **Benchmark Module** (1 day) - Performance testing
3. **Complete Phase 12 Stdlib** (7-10 days) - I/O, JSON, Regex, YAML
4. **Crypto in Pure Graphoid** (3-4 days) - Self-hosted crypto module!
5. **Phase 11: Pure Graphoid Stdlib** (7-10 days) - Stats, CSV, SQL, HTTP, etc.

**Total to stdlib complete**: ~19-26 days

---

**END OF PLAN**
