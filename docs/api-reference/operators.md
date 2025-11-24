# Operators Reference

Complete reference of all operators in Graphoid, including precedence, associativity, and behavior.

## Operator Precedence Table

Operators are listed from highest to lowest precedence:

| Precedence | Operator | Description | Associativity |
|------------|----------|-------------|---------------|
| 1 | `()` `[]` `.` | Grouping, indexing, member access | Left-to-right |
| 2 | `**` | Exponentiation | Right-to-left |
| 3 | `~` `not` `-` (unary) | Bitwise NOT, logical NOT, negation | Right-to-left |
| 4 | `*` `/` `//` `%` | Multiplication, division, int division, modulo | Left-to-right |
| 5 | `.*` `./` | Element-wise multiply, divide | Left-to-right |
| 6 | `+` `-` | Addition, subtraction | Left-to-right |
| 7 | `.+` `.-` | Element-wise add, subtract | Left-to-right |
| 8 | `<<` `>>` | Bitwise shift left, shift right | Left-to-right |
| 9 | `&` | Bitwise AND | Left-to-right |
| 10 | `^` | Bitwise XOR | Left-to-right |
| 11 | `\|` | Bitwise OR | Left-to-right |
| 12 | `<` `<=` `>` `>=` | Comparison | Left-to-right |
| 13 | `==` `!=` | Equality, inequality | Left-to-right |
| 14 | `in` | Membership test | Left-to-right |
| 15 | `and` | Logical AND | Left-to-right |
| 16 | `or` | Logical OR | Left-to-right |
| 17 | `=` `+=` `-=` `*=` `/=` etc. | Assignment | Right-to-left |

---

## Arithmetic Operators

### Addition (`+`)

**Binary operator**: Adds two values.

**Syntax**: `a + b`

**Types**:
- `num + num` → `num` (addition)
- `string + string` → `string` (concatenation)
- `list + list` → `list` (concatenation)
- `hash + hash` → `hash` (merge)

**Examples**:
```graphoid
# Numbers
result = 5 + 3          # 8
result = 2.5 + 1.5      # 4.0

# Strings
greeting = "Hello, " + "world!"  # "Hello, world!"

# Lists
combined = [1, 2] + [3, 4]  # [1, 2, 3, 4]

# Hashes
config = defaults + overrides
```

**With Directives**:
```graphoid
configure { :integer } {
    result = 10 + 3  # 13 (integer)
}

configure { :unsigned, :32bit } {
    result = 0xFFFFFFFF + 1  # 0 (wraps)
}
```

**See also**: `-`, `+=`, `.+`

---

### Subtraction (`-`)

**Binary operator**: Subtracts two numbers.

**Unary operator**: Negates a number.

**Syntax**: `a - b` (binary), `-a` (unary)

**Examples**:
```graphoid
# Binary
result = 10 - 3     # 7
result = 5.5 - 2.2  # 3.3

# Unary
negative = -5       # -5
x = -(-3)          # 3
```

**With Directives**:
```graphoid
configure { :unsigned } {
    result = 3 - 5  # Large positive (wraps)
}
```

**See also**: `+`, `-=`, `.-`

---

### Multiplication (`*`)

**Binary operator**: Multiplies two values.

**Syntax**: `a * b`

**Types**:
- `num * num` → `num` (multiplication)
- `string * num` → `string` (repetition)
- `list * num` → `list` (repetition)

**Examples**:
```graphoid
# Numbers
result = 6 * 7      # 42
result = 2.5 * 4    # 10.0

# String repetition
separator = "-" * 20  # "--------------------"

# List repetition
zeros = [0] * 5  # [0, 0, 0, 0, 0]
```

**See also**: `/`, `*=`, `**`, `.*`

---

### Division (`/`)

**Binary operator**: Divides two numbers.

**Syntax**: `a / b`

**Examples**:
```graphoid
# Default: floating-point
result = 10 / 3     # 3.333...
result = 15 / 5     # 3.0
```

**With Directives**:
```graphoid
configure { :integer } {
    result = 10 / 3  # 3 (truncated)
}
```

**Errors**: Division by zero raises error

**See also**: `//`, `%`, `/=`, `./`

---

### Integer Division (`//`)

**Binary operator**: Divides and truncates to integer (floor division).

**Syntax**: `a // b`

**Examples**:
```graphoid
result = 10 // 3    # 3
result = 17 // 5    # 3
result = -10 // 3   # -4 (floor)
```

**Errors**: Division by zero raises error

**See also**: `/`, `%`

---

### Modulo (`%`)

**Binary operator**: Returns remainder after division.

**Syntax**: `a % b`

**Examples**:
```graphoid
result = 10 % 3     # 1
result = 17 % 5     # 2

# Check even/odd
is_even = (n % 2) == 0

# Wrap index
index = i % list.length()
```

**Errors**: Modulo by zero raises error

**See also**: `/`, `//`

---

### Exponentiation (`**`)

**Binary operator**: Raises number to a power.

**Syntax**: `a ** b`

**Associativity**: Right-to-left (unusual!)

**Examples**:
```graphoid
result = 2 ** 8     # 256
result = 10 ** 3    # 1000
result = 4 ** 0.5   # 2.0 (square root)
result = 2 ** -3    # 0.125

# Right associativity
result = 2 ** 3 ** 2  # 2 ** (3 ** 2) = 2 ** 9 = 512
```

**See also**: `math.pow()`, `math.sqrt()`

---

## Comparison Operators

All comparison operators return boolean values (`true` or `false`).

### Equal (`==`)

Tests if two values are equal.

**Syntax**: `a == b`

**Examples**:
```graphoid
5 == 5          # true
5 == 3          # false
"hello" == "hello"  # true
[1, 2] == [1, 2]    # true (deep equality)
```

**See also**: `!=`

---

### Not Equal (`!=`)

Tests if two values are not equal.

**Syntax**: `a != b`

**Examples**:
```graphoid
5 != 3          # true
5 != 5          # false
"a" != "b"      # true
```

**See also**: `==`

---

### Less Than (`<`)

Tests if left value is less than right.

**Syntax**: `a < b`

**Examples**:
```graphoid
5 < 10          # true
10 < 5          # false
"a" < "b"       # true (lexicographic)
```

**See also**: `<=`, `>`, `>=`

---

### Less Than or Equal (`<=`)

Tests if left value is less than or equal to right.

**Syntax**: `a <= b`

**Examples**:
```graphoid
5 <= 10         # true
5 <= 5          # true
10 <= 5         # false
```

**See also**: `<`, `>`, `>=`

---

### Greater Than (`>`)

Tests if left value is greater than right.

**Syntax**: `a > b`

**Examples**:
```graphoid
10 > 5          # true
5 > 10          # false
"z" > "a"       # true
```

**See also**: `>=`, `<`, `<=`

---

### Greater Than or Equal (`>=`)

Tests if left value is greater than or equal to right.

**Syntax**: `a >= b`

**Examples**:
```graphoid
10 >= 5         # true
5 >= 5          # true
5 >= 10         # false
```

**See also**: `>`, `<`, `<=`

---

## Logical Operators

### Logical AND (`and`)

Returns true if both operands are truthy.

**Syntax**: `a and b`

**Short-circuiting**: If `a` is falsy, `b` is not evaluated

**Examples**:
```graphoid
true and true       # true
true and false      # false
false and true      # false

# Short-circuit
x > 0 and y / x > 5  # Safe: y/x only if x > 0

# Chaining
if age >= 18 and age <= 65 and has_license {
    print("Can drive")
}
```

**Truthy values**: All values except `false`, `none`, `0`, `""`, `[]`, `{}`

**See also**: `or`, `not`

---

### Logical OR (`or`)

Returns true if either operand is truthy.

**Syntax**: `a or b`

**Short-circuiting**: If `a` is truthy, `b` is not evaluated

**Examples**:
```graphoid
true or false       # true
false or true       # true
false or false      # false

# Default values
name = input or "Anonymous"
port = config["port"] or 8080

# Chaining
if is_admin or is_moderator or is_owner {
    print("Has privileges")
}
```

**See also**: `and`, `not`

---

### Logical NOT (`not`)

Negates a boolean value.

**Syntax**: `not a`

**Examples**:
```graphoid
not true            # false
not false           # true
not (5 > 3)         # false

# Readability
if not is_valid {
    print("Invalid")
}

# Double negative
if not not value {  # Same as: if value
    print("Truthy")
}
```

**See also**: `and`, `or`

---

## Bitwise Operators

Bitwise operators work on the binary representation of integers.

### Bitwise AND (`&`)

Performs bitwise AND.

**Syntax**: `a & b`

**Examples**:
```graphoid
configure { :unsigned } {
    result = 0b1100 & 0b1010  # 0b1000 (8)

    # Extract bits
    lower_byte = value & 0xFF

    # Check if bit is set
    is_set = (flags & 0x04) != 0
}
```

**See also**: `|`, `^`, `~`

---

### Bitwise OR (`|`)

Performs bitwise OR.

**Syntax**: `a | b`

**Examples**:
```graphoid
configure { :unsigned } {
    result = 0b1100 | 0b1010  # 0b1110 (14)

    # Set bits
    flags = flags | 0x04

    # Combine permissions
    permissions = READ | WRITE | EXECUTE
}
```

**See also**: `&`, `^`, `~`

---

### Bitwise XOR (`^`)

Performs bitwise XOR (exclusive OR).

**Syntax**: `a ^ b`

**Examples**:
```graphoid
configure { :unsigned } {
    result = 0b1100 ^ 0b1010  # 0b0110 (6)

    # Toggle bits
    value = value ^ 0x04

    # Swap without temp
    a = a ^ b
    b = a ^ b
    a = a ^ b
}
```

**See also**: `&`, `|`, `~`

---

### Bitwise NOT (`~`)

Performs bitwise NOT (complement).

**Syntax**: `~a`

**Examples**:
```graphoid
configure { :unsigned } {
    result = ~0b1100  # Inverts all bits

    # Create mask
    mask = ~0xFF  # All bits except lower 8
}
```

**See also**: `&`, `|`, `^`

---

### Left Shift (`<<`)

Shifts bits to the left.

**Syntax**: `a << b`

**Examples**:
```graphoid
result = 5 << 2     # 20 (5 * 4)
result = 1 << 8     # 256

configure { :unsigned, :32bit } {
    result = 0xFFFFFFFF << 1  # Wraps in 32-bit
}
```

**Effect**: Multiplies by 2^b

**See also**: `>>`, `*`

---

### Right Shift (`>>`)

Shifts bits to the right.

**Syntax**: `a >> b`

**Examples**:
```graphoid
result = 20 >> 2    # 5 (20 / 4)
result = 256 >> 8   # 1

configure { :unsigned } {
    result = 128 >> 1  # 64 (logical shift)
}
```

**Effect**: Divides by 2^b (floor division)

**Types**:
- **Signed**: Arithmetic shift (preserves sign bit)
- **Unsigned** (with `:unsigned`): Logical shift (fills with 0)

**See also**: `<<`, `/`

---

## Element-wise Operators

Element-wise operators operate on corresponding elements of two lists.

### Element-wise Addition (`.+`)

Adds corresponding elements.

**Syntax**: `list1 .+ list2`

**Examples**:
```graphoid
a = [1, 2, 3]
b = [10, 20, 30]
result = a .+ b
# [11, 22, 33]

# Different lengths: uses minimum
a = [1, 2, 3, 4]
b = [10, 20]
result = a .+ b
# [11, 22]
```

**See also**: `.-`, `.*`, `./`

---

### Element-wise Subtraction (`.-`)

Subtracts corresponding elements.

**Syntax**: `list1 .- list2`

**Examples**:
```graphoid
a = [10, 20, 30]
b = [1, 2, 3]
result = a .- b
# [9, 18, 27]
```

**See also**: `.+`, `.*`, `./`

---

### Element-wise Multiplication (`.*`)

Multiplies corresponding elements.

**Syntax**: `list1 .* list2`

**Examples**:
```graphoid
a = [2, 3, 4]
b = [10, 20, 30]
result = a .* b
# [20, 60, 120]
```

**See also**: `.+`, `.-`, `./`

---

### Element-wise Division (`./`)

Divides corresponding elements.

**Syntax**: `list1 ./ list2`

**Examples**:
```graphoid
a = [20, 40, 60]
b = [2, 4, 6]
result = a ./ b
# [10, 10, 10]
```

**Errors**: Division by zero in any element raises error

**See also**: `.+`, `.-`, `.*`

---

## Assignment Operators

### Simple Assignment (`=`)

Assigns a value to a variable.

**Syntax**: `variable = value`

**Examples**:
```graphoid
x = 5
name = "Alice"
items = [1, 2, 3]

# Multiple assignment (if supported)
a = b = c = 0
```

**See also**: `+=`, `-=`, `*=`, `/=`

---

### Compound Assignment

Performs operation and assignment in one step.

**Operators**: `+=`, `-=`, `*=`, `/=`, `//=`, `%=`, `**=`, `&=`, `|=`, `^=`, `<<=`, `>>=`

**Examples**:
```graphoid
# Addition assignment
x += 5      # Same as: x = x + 5

# Subtraction assignment
x -= 3      # Same as: x = x - 3

# Multiplication assignment
x *= 2      # Same as: x = x * 2

# Division assignment
x /= 4      # Same as: x = x / 4

# Bitwise assignment
flags |= 0x04   # Set bit
flags &= ~0x04  # Clear bit

# Exponentiation assignment
x **= 2     # Same as: x = x ** 2
```

**See also**: `=`

---

## Member Access Operators

### Dot (`.`)

Accesses object members or calls methods.

**Syntax**: `object.member` or `object.method(args)`

**Examples**:
```graphoid
# Method call
result = list.length()
upper = text.to_upper()

# Chaining
result = numbers
    .filter("positive")
    .map("square")
    .sum()

# Module access
import "math"
pi = math.pi
sqrt_val = math.sqrt(16)
```

**See also**: `[]`

---

### Indexing (`[]`)

Accesses elements by index or key.

**Syntax**: `collection[index]` or `collection[key]`

**Examples**:
```graphoid
# List indexing
first = list[0]
last = list[-1]

# Hash access
value = hash["key"]

# String indexing
char = string[0]

# Nested access
value = data["user"]["name"]
```

**See also**: `.`, slicing

---

### Slicing (`[start:end]`)

Extracts a subsequence.

**Syntax**: `collection[start:end]`

**Examples**:
```graphoid
# List slicing
subset = list[1:4]      # Elements 1, 2, 3
first_three = list[:3]  # First 3 elements
last_two = list[-2:]    # Last 2 elements

# String slicing
substr = text[0:5]      # First 5 chars
```

**See also**: `[]`, `substring()`, `slice()`

---

## Membership Operator

### In (`in`)

Tests if a value exists in a collection.

**Syntax**: `value in collection`

**Examples**:
```graphoid
# List membership
result = 3 in [1, 2, 3, 4]  # true

# String substring
result = "world" in "hello world"  # true

# Hash key
result = "name" in user  # true if key exists

# Conditional
if "admin" in roles {
    print("Admin access")
}
```

**See also**: `contains()`, `has_key()`

---

## Range Operator

### Range (`..`)

Creates a range of numbers (if supported).

**Syntax**: `start..end`

**Examples**:
```graphoid
# Range iteration
for i in 0..10 {
    print(i)  # 0 through 10
}

# Exclusive range
for i in 0..<10 {
    print(i)  # 0 through 9
}
```

**Note**: May use `list.generate()` instead:
```graphoid
for i in list.generate(0, 10) {
    print(i)
}
```

**See also**: `list.generate()`

---

## Operator Behavior Under Directives

### Integer Mode (`:integer`)

```graphoid
configure { :integer } {
    10 / 3      # 3 (not 3.333...)
    10 // 3     # 3 (same as /)
    10 % 3      # 1 (works as expected)
}
```

---

### Unsigned Mode (`:unsigned`)

```graphoid
configure { :unsigned } {
    3 - 5       # Large positive (wraps)
    128 >> 1    # 64 (logical shift)
    -5          # Still works (literal)
}
```

---

### 32-bit Mode (`:32bit`)

```graphoid
configure { :unsigned, :32bit } {
    0xFFFFFFFF + 1      # 0 (wraps at 32 bits)
    0xFFFFFFFF << 1     # 0xFFFFFFFE (wraps)

    # Automatic wrapping for all operations
    x = 0x80000000
    result = x * 2      # 0 (wraps)
}
```

---

## Special Cases

### NaN Behavior

```graphoid
# NaN propagates
x = 0.0 / 0.0   # NaN
y = x + 5       # NaN
z = x * 2       # NaN

# NaN != NaN
nan == nan      # false (!)

# Use is_nan() to test
if x.is_nan() {
    print("Undefined")
}
```

---

### Infinity Behavior

```graphoid
# Infinity in arithmetic
infinity + 5        # infinity
infinity * 2        # infinity
infinity - infinity # NaN
1.0 / infinity      # 0

# Comparisons
5 < infinity        # true
infinity == infinity # true
```

---

## Operator Overloading

Graphoid does **not** support user-defined operator overloading. Operators have fixed meanings for built-in types.

**Alternative**: Use methods with descriptive names:
```graphoid
# Instead of: vector1 + vector2
result = vector1.add(vector2)

# Instead of: matrix1 * matrix2
result = matrix1.multiply(matrix2)
```

---

## See Also

- [num](core/num.md) - Numeric operators in detail
- [string](core/string.md) - String operators
- [list](core/list.md) - List operators and element-wise operations
- [hash](core/hash.md) - Hash operators
- [directives](directives.md) - How directives affect operators
- [User Guide: Basics](../user-guide/02-basics.md) - Operator tutorial
