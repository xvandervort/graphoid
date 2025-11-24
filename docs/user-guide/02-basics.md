# Chapter 2: Basics

Now that you've written your first Graphoid programs, let's explore the fundamentals in depth.

## Variables

### Declaration and Assignment

In Graphoid, you don't need to declare variables before using them:

```graphoid
x = 42
name = "Alice"
is_active = true
```

Variables can be reassigned to different values:

```graphoid
x = 10
x = 20        # x is now 20
x = "hello"   # x is now a string
```

### Type Inference

Graphoid automatically infers the type of a variable:

```graphoid
age = 25              # Inferred as number
name = "Bob"          # Inferred as string
scores = [95, 87, 92] # Inferred as list
```

### Optional Type Annotations

You can add type annotations for clarity or constraints:

```graphoid
num x = 42                    # x must be a number
string name = "Alice"         # name must be a string
list<num> scores = [95, 87]   # list of numbers
```

**Note**: Type annotations are checked at runtime. If you try to assign the wrong type, you'll get an error.

```graphoid
num x = 42
x = "hello"  # Error: Type mismatch
```

## Data Types

### Numbers

Graphoid has a unified number type that handles integers and floats:

```graphoid
x = 42        # Integer
y = 3.14159   # Float
z = -17       # Negative integer
w = 2.5e10    # Scientific notation
```

**Hexadecimal, Octal, Binary:**

```graphoid
hex = 0xFF        # 255 in hexadecimal
octal = 0o77      # 63 in octal
binary = 0b1010   # 10 in binary
```

### Strings

Strings can use single or double quotes:

```graphoid
single = 'Hello'
double = "World"
```

**String Concatenation:**

```graphoid
greeting = "Hello, " + "World!"
# Result: "Hello, World!"
```

**String Interpolation** (planned feature):

```graphoid
name = "Alice"
message = "Hello, {name}!"  # Future feature
```

For now, use concatenation:

```graphoid
name = "Alice"
message = "Hello, " + name + "!"
```

### Booleans

Boolean values are `true` and `false`:

```graphoid
is_active = true
is_complete = false
```

### None

`none` represents the absence of a value:

```graphoid
result = none

if result == none {
    print("No result yet")
}
```

## Operators

### Arithmetic Operators

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `+` | Addition | `5 + 3` | `8` |
| `-` | Subtraction | `10 - 7` | `3` |
| `*` | Multiplication | `4 * 5` | `20` |
| `/` | Division | `20 / 4` | `5` |
| `//` | Integer Division | `7 // 2` | `3` |
| `%` | Modulo | `10 % 3` | `1` |
| `**` | Exponentiation | `2 ** 8` | `256` |

**Examples:**

```graphoid
# Basic arithmetic
sum = 10 + 5              # 15
difference = 10 - 5       # 5
product = 10 * 5          # 50
quotient = 10 / 5         # 2

# Integer division (truncates decimal)
result = 7 // 2           # 3 (not 3.5)

# Modulo (remainder)
remainder = 10 % 3        # 1

# Exponentiation
power = 2 ** 10           # 1024
```

### Comparison Operators

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `==` | Equal to | `5 == 5` | `true` |
| `!=` | Not equal to | `5 != 3` | `true` |
| `<` | Less than | `3 < 5` | `true` |
| `>` | Greater than | `5 > 3` | `true` |
| `<=` | Less than or equal | `5 <= 5` | `true` |
| `>=` | Greater than or equal | `5 >= 3` | `true` |

**Examples:**

```graphoid
x = 10
y = 20

print(x == y)   # false
print(x != y)   # true
print(x < y)    # true
print(x > y)    # false
print(x <= 10)  # true
print(y >= 20)  # true
```

### Logical Operators

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `and` | Logical AND | `true and false` | `false` |
| `or` | Logical OR | `true or false` | `true` |
| `not` | Logical NOT | `not true` | `false` |

**Examples:**

```graphoid
age = 25
has_license = true

# Can drive if age >= 16 AND has license
can_drive = age >= 16 and has_license  # true

# Weekend if Saturday OR Sunday
day = "Saturday"
is_weekend = day == "Saturday" or day == "Sunday"  # true

# Not a minor
is_adult = not (age < 18)  # true
```

**Short-circuit Evaluation:**

Graphoid uses short-circuit evaluation for `and` and `or`:

```graphoid
# If first condition is false, second is not evaluated
false and print("This won't print")

# If first condition is true, second is not evaluated
true or print("This won't print either")
```

### Bitwise Operators

For low-level bit manipulation:

| Operator | Description | Example | Result |
|----------|-------------|---------|--------|
| `&` | Bitwise AND | `12 & 10` | `8` |
| `\|` | Bitwise OR | `12 \| 10` | `14` |
| `^` | Bitwise XOR | `12 ^ 10` | `6` |
| `~` | Bitwise NOT | `~5` | `-6` |
| `<<` | Left shift | `3 << 2` | `12` |
| `>>` | Right shift | `12 >> 2` | `3` |

**Examples:**

```graphoid
# Binary: 12 = 0b1100, 10 = 0b1010

result = 12 & 10   # 0b1000 = 8 (AND)
result = 12 | 10   # 0b1110 = 14 (OR)
result = 12 ^ 10   # 0b0110 = 6 (XOR)
result = ~5        # -6 (NOT, two's complement)
result = 3 << 2    # 12 (shift left by 2: 3 * 2^2)
result = 12 >> 2   # 3 (shift right by 2: 12 / 2^2)
```

### Operator Precedence

From highest to lowest:

1. `**` (exponentiation)
2. `~` (bitwise NOT), unary `-` (negation)
3. `*`, `/`, `//`, `%`
4. `+`, `-`
5. `<<`, `>>`
6. `&`
7. `^`
8. `|`
9. `<`, `<=`, `>`, `>=`, `==`, `!=`
10. `not`
11. `and`
12. `or`

**Use parentheses for clarity:**

```graphoid
result = (5 + 3) * 2      # 16 (not 11)
result = 2 ** (3 + 1)     # 16 (not 9)
result = (x > 5) and (y < 10)
```

## String Operations

### Concatenation

```graphoid
first = "Hello"
last = "World"
message = first + " " + last  # "Hello World"
```

### String Methods

**Length:**

```graphoid
text = "Hello"
len = text.length()  # 5
```

**Case Conversion:**

```graphoid
text = "Hello World"

upper = text.to_upper()   # "HELLO WORLD"
lower = text.to_lower()   # "hello world"
```

**Trimming:**

```graphoid
text = "  hello  "
trimmed = text.trim()     # "hello"
```

**Substrings:**

```graphoid
text = "Hello World"

# Get substring from index 0 to 5 (exclusive)
sub = text.substring(0, 5)  # "Hello"

# Get substring from index 6 to end
sub = text.substring(6)     # "World"
```

**String Indexing:**

```graphoid
text = "Hello"

first_char = text[0]   # "H"
last_char = text[4]    # "o"
```

**Contains/Search:**

```graphoid
text = "Hello World"

has_world = text.contains("World")  # true
has_foo = text.contains("foo")      # false

index = text.index_of("World")      # 6
index = text.index_of("foo")        # -1 (not found)
```

**Split and Join:**

```graphoid
# Split string into list
text = "apple,banana,orange"
fruits = text.split(",")  # ["apple", "banana", "orange"]

# Join list into string
result = fruits.join(", ")  # "apple, banana, orange"
```

## Number Methods

**Type Conversion:**

```graphoid
# Number to string
num = 42
text = num.to_string()  # "42"

# String to number
text = "123"
num = text.to_number()  # 123.0

# Float to integer (truncate)
x = 3.99
i = x.to_int()  # 3
```

**Math Operations:**

```graphoid
x = -5.7

abs_val = x.abs()      # 5.7 (absolute value)
floor = x.floor()      # -6 (round down)
ceil = x.ceil()        # -5 (round up)
rounded = x.round()    # -6 (round to nearest)
```

**Min/Max:**

```graphoid
a = 10
b = 20

minimum = a.min(b)  # 10
maximum = a.max(b)  # 20
```

## Type Checking and Conversion

### Type Checking

```graphoid
x = 42

is_num = x.is_number()    # true
is_str = x.is_string()    # false
is_list = x.is_list()     # false
is_bool = x.is_boolean()  # false
is_none = x.is_none()     # false
```

### Type Conversion

```graphoid
# To string
num = 42
str = num.to_string()     # "42"

# To number
str = "123"
num = str.to_number()     # 123.0

# To boolean
x = 1
b = x.to_boolean()        # true (non-zero is true)

x = 0
b = x.to_boolean()        # false (zero is false)
```

### Truthiness

In boolean contexts, values are converted to true/false:

**Falsy values** (evaluate to `false`):
- `false`
- `0`
- `"" ` (empty string)
- `none`
- `[]` (empty list)

**Truthy values** (evaluate to `true`):
- Everything else

**Example:**

```graphoid
if 0 {
    print("This won't print")  # 0 is falsy
}

if "hello" {
    print("This will print")   # Non-empty string is truthy
}
```

## Constants

For clarity, you can use UPPERCASE names for constants (by convention):

```graphoid
PI = 3.14159
MAX_USERS = 100
API_KEY = "abc123"
```

**Note**: These are still variables and can be reassigned. Graphoid doesn't enforce constant immutability (yet).

## Summary

In this chapter, you learned:

- ✅ Variables and type inference
- ✅ Data types: numbers, strings, booleans, none
- ✅ Arithmetic, comparison, logical, and bitwise operators
- ✅ Operator precedence
- ✅ String operations and methods
- ✅ Number methods and conversions
- ✅ Type checking and truthiness

---

## Quick Reference

### Common Operations

```graphoid
# Variables
x = 10
num x = 10  # With type annotation

# Strings
text = "Hello"
len = text.length()
upper = text.to_upper()
sub = text.substring(0, 5)

# Numbers
x = 42
abs_val = x.abs()
str = x.to_string()

# Type checking
x.is_number()
x.is_string()
x.is_list()
```

---

## Exercises

1. Write a program that converts temperature from Celsius to Fahrenheit
   - Formula: F = (C × 9/5) + 32

2. Create a program that calculates the area and perimeter of a rectangle

3. Write a program that checks if a number is even or odd using the modulo operator

4. Create a string formatter that takes a name and age and produces: "Name: Alice, Age: 30"

5. Write a program that uses bitwise operators to check if a number is a power of 2
   - Hint: A power of 2 has only one bit set

**Solutions** are available in `examples/02-basics/exercises.gr`

---

[← Previous: Getting Started](01-getting-started.md) | [Next: Control Flow →](03-control-flow.md)
