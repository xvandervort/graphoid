# num - Number Type

Numbers in Graphoid represent numeric values. By default, numbers are 64-bit floating point, but can be configured to use different modes via directives.

## Numeric Modes

Numbers can operate in different modes controlled by directives:

- **Default**: 64-bit floating point (f64)
- **`:integer`**: Integer arithmetic (truncates division)
- **`:unsigned`**: Unsigned (non-negative) arithmetic
- **`:high`**: High precision (128-bit float)
- **`:32bit`**: 32-bit wrapping arithmetic (requires `:unsigned`)

See [directives.md](../directives.md) for details.

---

## Arithmetic Operators

### Addition (`+`)

Adds two numbers.

**Syntax**: `a + b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (num) Sum of `a` and `b`

**Examples**:
```graphoid
# Basic addition
result = 5 + 3
print(result)  # 8

# Floating point
result = 2.5 + 1.5
print(result)  # 4.0

# With :integer directive
configure { :integer } {
    result = 10 + 3
    print(result)  # 13
}

# 32-bit wrapping
configure { :unsigned, :32bit } {
    x = 4294967295  # 0xFFFFFFFF
    result = x + 1
    print(result)  # 0 (wraps around)
}
```

**See also**: `-`, `*`, `/`

---

### Subtraction (`-`)

Subtracts one number from another.

**Syntax**: `a - b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (num) Difference of `a` and `b`

**Examples**:
```graphoid
result = 10 - 3
print(result)  # 7

result = 5.5 - 2.2
print(result)  # 3.3

# With :unsigned (no negative results)
configure { :unsigned } {
    result = 3 - 5
    print(result)  # Large positive number (wraps)
}
```

**See also**: `+`, `*`, `/`

---

### Multiplication (`*`)

Multiplies two numbers.

**Syntax**: `a * b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (num) Product of `a` and `b`

**Examples**:
```graphoid
result = 6 * 7
print(result)  # 42

result = 2.5 * 4
print(result)  # 10.0

# Integer mode
configure { :integer } {
    result = 5 * 3
    print(result)  # 15
}
```

**See also**: `+`, `-`, `/`, `**`

---

### Division (`/`)

Divides one number by another.

**Syntax**: `a / b`

**Parameters**:
- `a` (num): Numerator
- `b` (num): Denominator

**Returns**: (num) Quotient of `a` and `b`

**Examples**:
```graphoid
# Default: floating point division
result = 10 / 3
print(result)  # 3.333...

result = 15 / 5
print(result)  # 3.0

# Integer mode: truncates to integer
configure { :integer } {
    result = 10 / 3
    print(result)  # 3
}
```

**Errors**: Division by zero raises a runtime error

**See also**: `//`, `%`

---

### Integer Division (`//`)

Divides and truncates to integer (floor division).

**Syntax**: `a // b`

**Parameters**:
- `a` (num): Numerator
- `b` (num): Denominator

**Returns**: (num) Integer quotient of `a` and `b`

**Examples**:
```graphoid
result = 10 // 3
print(result)  # 3

result = 17 // 5
print(result)  # 3

result = -10 // 3
print(result)  # -4 (floor division)
```

**Errors**: Division by zero raises a runtime error

**See also**: `/`, `%`

---

### Modulo (`%`)

Computes the remainder after division.

**Syntax**: `a % b`

**Parameters**:
- `a` (num): Dividend
- `b` (num): Divisor

**Returns**: (num) Remainder of `a / b`

**Examples**:
```graphoid
result = 10 % 3
print(result)  # 1

result = 17 % 5
print(result)  # 2

# Check if even
is_even = (n % 2) == 0

# Circular indexing
index = (i % list.length())
```

**Errors**: Modulo by zero raises a runtime error

**See also**: `/`, `//`

---

### Exponentiation (`**`)

Raises a number to a power.

**Syntax**: `a ** b`

**Parameters**:
- `a` (num): Base
- `b` (num): Exponent

**Returns**: (num) `a` raised to the power of `b`

**Examples**:
```graphoid
result = 2 ** 8
print(result)  # 256

result = 10 ** 3
print(result)  # 1000

result = 4 ** 0.5
print(result)  # 2.0 (square root)

# Negative exponents
result = 2 ** -3
print(result)  # 0.125
```

**See also**: `math.pow()`, `math.sqrt()`

---

## Comparison Operators

### Equal (`==`)

Tests if two numbers are equal.

**Syntax**: `a == b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (bool) `true` if equal, `false` otherwise

**Examples**:
```graphoid
result = 5 == 5     # true
result = 5 == 3     # false
result = 2.0 == 2   # true

# In conditionals
if age == 18 {
    print("You can vote!")
}
```

**See also**: `!=`, `<`, `>`

---

### Not Equal (`!=`)

Tests if two numbers are not equal.

**Syntax**: `a != b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (bool) `true` if not equal, `false` otherwise

**Examples**:
```graphoid
result = 5 != 3     # true
result = 5 != 5     # false

# In loops
while count != 10 {
    count = count + 1
}
```

**See also**: `==`, `<`, `>`

---

### Less Than (`<`)

Tests if one number is less than another.

**Syntax**: `a < b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (bool) `true` if `a < b`, `false` otherwise

**Examples**:
```graphoid
result = 5 < 10     # true
result = 10 < 5     # false
result = 5 < 5      # false

# Range checking
if age < 18 {
    print("Minor")
}
```

**See also**: `<=`, `>`, `>=`

---

### Less Than or Equal (`<=`)

Tests if one number is less than or equal to another.

**Syntax**: `a <= b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (bool) `true` if `a <= b`, `false` otherwise

**Examples**:
```graphoid
result = 5 <= 10    # true
result = 5 <= 5     # true
result = 10 <= 5    # false

# Range validation
if score <= 100 {
    print("Valid score")
}
```

**See also**: `<`, `>`, `>=`

---

### Greater Than (`>`)

Tests if one number is greater than another.

**Syntax**: `a > b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (bool) `true` if `a > b`, `false` otherwise

**Examples**:
```graphoid
result = 10 > 5     # true
result = 5 > 10     # false
result = 5 > 5      # false

# Threshold checking
if temperature > 100 {
    print("Fever!")
}
```

**See also**: `>=`, `<`, `<=`

---

### Greater Than or Equal (`>=`)

Tests if one number is greater than or equal to another.

**Syntax**: `a >= b`

**Parameters**:
- `a` (num): Left operand
- `b` (num): Right operand

**Returns**: (bool) `true` if `a >= b`, `false` otherwise

**Examples**:
```graphoid
result = 10 >= 5    # true
result = 5 >= 5     # true
result = 5 >= 10    # false

# Minimum requirement
if age >= 21 {
    print("Can drink")
}
```

**See also**: `>`, `<`, `<=`

---

## Bitwise Operators

Bitwise operators work on the binary representation of integers.

### Bitwise AND (`&`)

Performs bitwise AND operation.

**Syntax**: `a & b`

**Parameters**:
- `a` (num): Left operand (converted to integer)
- `b` (num): Right operand (converted to integer)

**Returns**: (num) Bitwise AND of `a` and `b`

**Examples**:
```graphoid
# Extract lower 8 bits
result = 0xFF & value

# Check if bit is set
is_set = (flags & 0x04) != 0

# Common patterns
configure { :unsigned } {
    result = 0b1100 & 0b1010  # 0b1000 (8)
    print(result)  # 8
}
```

**See also**: `|`, `^`, `~`

---

### Bitwise OR (`|`)

Performs bitwise OR operation.

**Syntax**: `a | b`

**Parameters**:
- `a` (num): Left operand (converted to integer)
- `b` (num): Right operand (converted to integer)

**Returns**: (num) Bitwise OR of `a` and `b`

**Examples**:
```graphoid
# Set bits
flags = flags | 0x04

# Combine permissions
permissions = READ | WRITE | EXECUTE

configure { :unsigned } {
    result = 0b1100 | 0b1010  # 0b1110 (14)
    print(result)  # 14
}
```

**See also**: `&`, `^`, `~`

---

### Bitwise XOR (`^`)

Performs bitwise XOR (exclusive OR) operation.

**Syntax**: `a ^ b`

**Parameters**:
- `a` (num): Left operand (converted to integer)
- `b` (num): Right operand (converted to integer)

**Returns**: (num) Bitwise XOR of `a` and `b`

**Examples**:
```graphoid
# Toggle bits
value = value ^ 0x04

# Swap without temp variable
a = a ^ b
b = a ^ b
a = a ^ b

configure { :unsigned } {
    result = 0b1100 ^ 0b1010  # 0b0110 (6)
    print(result)  # 6
}
```

**See also**: `&`, `|`, `~`

---

### Bitwise NOT (`~`)

Performs bitwise NOT (complement) operation.

**Syntax**: `~a`

**Parameters**:
- `a` (num): Operand (converted to integer)

**Returns**: (num) Bitwise complement of `a`

**Examples**:
```graphoid
# Invert all bits
result = ~value

configure { :unsigned } {
    result = ~0b1100  # All bits flipped
}
```

**See also**: `&`, `|`, `^`

---

### Left Shift (`<<`)

Shifts bits to the left.

**Syntax**: `a << b`

**Parameters**:
- `a` (num): Value to shift
- `b` (num): Number of positions to shift

**Returns**: (num) `a` shifted left by `b` positions

**Examples**:
```graphoid
# Multiply by powers of 2
result = 5 << 2   # 20 (5 * 4)
result = 1 << 8   # 256

configure { :unsigned, :32bit } {
    result = 0xFFFFFFFF << 1  # Wraps in 32-bit mode
}
```

**See also**: `>>`, `*`

---

### Right Shift (`>>`)

Shifts bits to the right.

**Syntax**: `a >> b`

**Parameters**:
- `a` (num): Value to shift
- `b` (num): Number of positions to shift

**Returns**: (num) `a` shifted right by `b` positions

**Examples**:
```graphoid
# Divide by powers of 2
result = 20 >> 2  # 5 (20 / 4)
result = 256 >> 8 # 1

# Extract high byte
high_byte = value >> 8

configure { :unsigned } {
    result = 128 >> 1  # 64 (logical shift)
}
```

**See also**: `<<`, `/`

---

## Methods

### to_string()

Converts number to string representation.

**Syntax**: `num.to_string()`

**Returns**: (string) String representation of the number

**Examples**:
```graphoid
result = 42.to_string()
print(result)  # "42"

result = 3.14159.to_string()
print(result)  # "3.14159"

# In string concatenation
message = "The answer is " + answer.to_string()
```

**See also**: `string.to_num()`

---

### abs()

Returns the absolute value of a number.

**Syntax**: `num.abs()`

**Returns**: (num) Absolute value

**Examples**:
```graphoid
result = (-5).abs()
print(result)  # 5

result = 3.14.abs()
print(result)  # 3.14

# Distance calculation
distance = (a - b).abs()
```

**See also**: `math.abs()`

---

### is_integer()

Tests if a number is an integer (has no fractional part).

**Syntax**: `num.is_integer()`

**Returns**: (bool) `true` if integer, `false` otherwise

**Examples**:
```graphoid
result = 5.is_integer()       # true
result = 5.0.is_integer()     # true
result = 5.5.is_integer()     # false

# Validation
if not value.is_integer() {
    print("Must be whole number")
}
```

**See also**: `is_number()`

---

### is_nan()

Tests if a number is NaN (Not a Number).

**Syntax**: `num.is_nan()`

**Returns**: (bool) `true` if NaN, `false` otherwise

**Examples**:
```graphoid
result = (0.0 / 0.0).is_nan()  # true
result = 5.is_nan()             # false

# Safe division
if result.is_nan() {
    result = 0
}
```

**See also**: `is_infinite()`

---

### is_infinite()

Tests if a number is infinite.

**Syntax**: `num.is_infinite()`

**Returns**: (bool) `true` if infinite, `false` otherwise

**Examples**:
```graphoid
result = (1.0 / 0.0).is_infinite()  # true
result = 5.is_infinite()             # false

# Bounds checking
if value.is_infinite() {
    print("Value out of bounds")
}
```

**See also**: `is_nan()`

---

## Constants

### Infinity

Positive infinity.

**Value**: `infinity` or `inf`

**Examples**:
```graphoid
max = infinity
result = 1.0 / 0.0  # infinity

if value < infinity {
    print("Finite value")
}
```

---

### Negative Infinity

Negative infinity.

**Value**: `-infinity` or `-inf`

**Examples**:
```graphoid
min = -infinity
result = -1.0 / 0.0  # -infinity
```

---

### NaN

Not a Number (undefined result).

**Value**: `nan`

**Examples**:
```graphoid
result = 0.0 / 0.0  # nan

# Always use is_nan() to test
if result.is_nan() {
    print("Undefined")
}

# Note: NaN != NaN
print(nan == nan)  # false!
```

---

## Type Checking

### is_number()

Tests if a value is a number.

**Syntax**: `value.is_number()`

**Returns**: (bool) `true` if number, `false` otherwise

**Examples**:
```graphoid
result = 5.is_number()          # true
result = "hello".is_number()    # false

# Type validation
if not value.is_number() {
    print("Expected number")
}
```

**See also**: `is_string()`, `is_list()`

---

## BigNum Types

Graphoid supports arbitrary precision numbers via BigNum types when needed.

### BigInt

Arbitrary precision integers.

**Created**: Automatically when integers overflow normal range

**Examples**:
```graphoid
# Large integer
huge = 123456789012345678901234567890

# Factorial (grows quickly)
fn factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

result = factorial(100)  # Returns BigInt
```

---

### Float128

128-bit floating point numbers.

**Created**: With `:high` directive

**Examples**:
```graphoid
configure { :high } {
    pi = 3.14159265358979323846264338327950288
    print(pi)  # Full precision retained
}
```

---

## Named Transformations

Numbers support named transformation functions for use with `map()`, `filter()`, etc.

### "double"

Multiplies by 2.

**Examples**:
```graphoid
[1, 2, 3].map("double")  # [2, 4, 6]
```

---

### "square"

Squares the number.

**Examples**:
```graphoid
[2, 3, 4].map("square")  # [4, 9, 16]
```

---

### "positive"

Tests if number is positive (> 0).

**Examples**:
```graphoid
[-1, 0, 1, 2].filter("positive")  # [1, 2]
```

---

### "negative"

Tests if number is negative (< 0).

**Examples**:
```graphoid
[-2, -1, 0, 1].filter("negative")  # [-2, -1]
```

---

### "even"

Tests if number is even.

**Examples**:
```graphoid
[1, 2, 3, 4, 5].filter("even")  # [2, 4]
```

---

### "odd"

Tests if number is odd.

**Examples**:
```graphoid
[1, 2, 3, 4, 5].filter("odd")  # [1, 3, 5]
```

---

## See Also

- [math](../stdlib/math.md) - Mathematical functions
- [directives](../directives.md) - Numeric mode directives
- [operators](../operators.md) - Complete operator reference
- [string](string.md) - String type reference
