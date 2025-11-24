# math - Mathematical Functions

The `math` module provides mathematical functions and constants for numerical computation.

## Importing

```graphoid
import "math"

# Use functions
result = math.sqrt(16)
pi_value = math.pi
```

---

## Constants

### math.pi

The mathematical constant π (pi).

**Value**: 3.14159265358979323846...

**Examples**:
```graphoid
import "math"

circumference = 2 * math.pi * radius
area = math.pi * radius ** 2

# Radians to degrees
degrees = radians * 180 / math.pi
```

**See also**: `tau`, `e`

---

### math.e

The mathematical constant e (Euler's number).

**Value**: 2.71828182845904523536...

**Examples**:
```graphoid
import "math"

# Natural exponential growth
population = initial * math.e ** (rate * time)

# Compound interest (continuous)
amount = principal * math.e ** (rate * years)
```

**See also**: `exp()`, `log()`

---

### math.tau

The mathematical constant τ (tau) = 2π.

**Value**: 6.28318530717958647692...

**Examples**:
```graphoid
import "math"

# Full circle in radians
full_circle = math.tau

# Quarter circle
quarter = math.tau / 4
```

**See also**: `pi`

---

### math.phi

The golden ratio φ (phi).

**Value**: 1.61803398874989484820...

**Examples**:
```graphoid
import "math"

# Golden rectangle
width = 100
height = width / math.phi

# Fibonacci approximation
fib_n = (math.phi ** n - (-math.phi) ** (-n)) / math.sqrt(5)
```

---

### math.infinity

Positive infinity.

**Value**: ∞

**Examples**:
```graphoid
import "math"

max_value = math.infinity
min_value = -math.infinity

# Check for infinity
if value == math.infinity {
    print("Value is infinite")
}
```

**See also**: `is_infinite()`, `is_nan()`

---

### math.nan

Not a Number (NaN).

**Value**: NaN

**Examples**:
```graphoid
import "math"

undefined = 0.0 / 0.0  # math.nan

# Note: NaN != NaN
# Use is_nan() to test
if value.is_nan() {
    print("Undefined value")
}
```

**See also**: `is_nan()`

---

## Basic Functions

### math.abs(x)

Returns the absolute value of a number.

**Syntax**: `math.abs(x)`

**Parameters**:
- `x` (num): Number

**Returns**: (num) Absolute value

**Examples**:
```graphoid
import "math"

result = math.abs(-5)     # 5
result = math.abs(3.14)   # 3.14
result = math.abs(-2.5)   # 2.5

# Distance
distance = math.abs(a - b)
```

**See also**: `num.abs()`

---

### math.sqrt(x)

Returns the square root of a number.

**Syntax**: `math.sqrt(x)`

**Parameters**:
- `x` (num): Non-negative number

**Returns**: (num) Square root

**Examples**:
```graphoid
import "math"

result = math.sqrt(16)    # 4.0
result = math.sqrt(2)     # 1.414...
result = math.sqrt(0.25)  # 0.5

# Pythagorean theorem
hypotenuse = math.sqrt(a**2 + b**2)

# Distance formula
distance = math.sqrt((x2 - x1)**2 + (y2 - y1)**2)
```

**Errors**: Negative input raises error or returns NaN

**See also**: `pow()`, `**` operator

---

### math.pow(base, exponent)

Raises a number to a power.

**Syntax**: `math.pow(base, exponent)`

**Parameters**:
- `base` (num): Base number
- `exponent` (num): Exponent

**Returns**: (num) base^exponent

**Examples**:
```graphoid
import "math"

result = math.pow(2, 8)      # 256
result = math.pow(10, 3)     # 1000
result = math.pow(4, 0.5)    # 2.0 (square root)
result = math.pow(2, -3)     # 0.125

# Same as ** operator
result = 2 ** 8  # 256
```

**See also**: `sqrt()`, `exp()`, `**` operator

---

### math.exp(x)

Returns e raised to the power of x.

**Syntax**: `math.exp(x)`

**Parameters**:
- `x` (num): Exponent

**Returns**: (num) e^x

**Examples**:
```graphoid
import "math"

result = math.exp(1)    # 2.718... (e)
result = math.exp(0)    # 1.0
result = math.exp(-1)   # 0.368...

# Exponential growth
growth = initial * math.exp(rate * time)
```

**See also**: `log()`, `pow()`

---

### math.log(x, base)

Returns the logarithm of x.

**Syntax**: `math.log(x, base)`

**Parameters**:
- `x` (num): Positive number
- `base` (num, optional): Logarithm base (default: e - natural log)

**Returns**: (num) log_base(x)

**Examples**:
```graphoid
import "math"

# Natural logarithm (base e)
result = math.log(math.e)    # 1.0
result = math.log(10)        # 2.302...

# Logarithm with custom base
result = math.log(8, 2)      # 3.0 (log2(8))
result = math.log(1000, 10)  # 3.0 (log10(1000))

# Half-life calculation
half_life = math.log(2) / decay_rate
```

**Errors**: Non-positive input raises error

**See also**: `log10()`, `log2()`, `exp()`

---

### math.log10(x)

Returns the base-10 logarithm.

**Syntax**: `math.log10(x)`

**Parameters**:
- `x` (num): Positive number

**Returns**: (num) log₁₀(x)

**Examples**:
```graphoid
import "math"

result = math.log10(10)     # 1.0
result = math.log10(100)    # 2.0
result = math.log10(1000)   # 3.0

# pH calculation
ph = -math.log10(h_concentration)

# Decibels
db = 10 * math.log10(power_ratio)
```

**See also**: `log()`, `log2()`

---

### math.log2(x)

Returns the base-2 logarithm.

**Syntax**: `math.log2(x)`

**Parameters**:
- `x` (num): Positive number

**Returns**: (num) log₂(x)

**Examples**:
```graphoid
import "math"

result = math.log2(8)      # 3.0
result = math.log2(16)     # 4.0
result = math.log2(1024)   # 10.0

# Bits needed to represent n values
bits = math.ceil(math.log2(n))

# Binary search depth
depth = math.log2(array_size)
```

**See also**: `log()`, `log10()`

---

## Rounding Functions

### math.ceil(x)

Rounds up to the nearest integer.

**Syntax**: `math.ceil(x)`

**Parameters**:
- `x` (num): Number

**Returns**: (num) Smallest integer ≥ x

**Examples**:
```graphoid
import "math"

result = math.ceil(3.2)    # 4
result = math.ceil(3.8)    # 4
result = math.ceil(-3.2)   # -3
result = math.ceil(5.0)    # 5

# Pages needed
pages = math.ceil(items / items_per_page)

# Round up to multiple of 10
rounded = math.ceil(value / 10) * 10
```

**See also**: `floor()`, `round()`

---

### math.floor(x)

Rounds down to the nearest integer.

**Syntax**: `math.floor(x)`

**Parameters**:
- `x` (num): Number

**Returns**: (num) Largest integer ≤ x

**Examples**:
```graphoid
import "math"

result = math.floor(3.2)   # 3
result = math.floor(3.8)   # 3
result = math.floor(-3.2)  # -4
result = math.floor(5.0)   # 5

# Integer division
quotient = math.floor(a / b)

# Round down to multiple of 5
rounded = math.floor(value / 5) * 5
```

**See also**: `ceil()`, `round()`

---

### math.round(x, decimals)

Rounds to the nearest integer or decimal places.

**Syntax**: `math.round(x, decimals)`

**Parameters**:
- `x` (num): Number
- `decimals` (num, optional): Decimal places (default: 0)

**Returns**: (num) Rounded value

**Examples**:
```graphoid
import "math"

# Round to integer
result = math.round(3.5)      # 4
result = math.round(3.4)      # 3
result = math.round(-3.5)     # -4

# Round to decimal places
result = math.round(3.14159, 2)   # 3.14
result = math.round(2.71828, 3)   # 2.718

# Currency rounding
price = math.round(19.999, 2)  # 20.00

# Round to nearest 0.5
rounded = math.round(value * 2) / 2
```

**See also**: `ceil()`, `floor()`

---

### math.trunc(x)

Truncates to integer (removes fractional part).

**Syntax**: `math.trunc(x)`

**Parameters**:
- `x` (num): Number

**Returns**: (num) Integer part

**Examples**:
```graphoid
import "math"

result = math.trunc(3.7)    # 3
result = math.trunc(-3.7)   # -3
result = math.trunc(5.0)    # 5

# Integer part
integer = math.trunc(value)
fraction = value - integer
```

**See also**: `floor()`, `round()`

---

## Trigonometric Functions

### math.sin(x)

Returns the sine of x (x in radians).

**Syntax**: `math.sin(x)`

**Parameters**:
- `x` (num): Angle in radians

**Returns**: (num) sin(x) in range [-1, 1]

**Examples**:
```graphoid
import "math"

result = math.sin(0)                # 0.0
result = math.sin(math.pi / 2)      # 1.0
result = math.sin(math.pi)          # 0.0
result = math.sin(3 * math.pi / 2)  # -1.0

# Degrees to radians
angle_rad = angle_deg * math.pi / 180
y = math.sin(angle_rad)

# Circular motion
x = radius * math.cos(angle)
y = radius * math.sin(angle)
```

**See also**: `cos()`, `tan()`, `asin()`, `degrees()`, `radians()`

---

### math.cos(x)

Returns the cosine of x (x in radians).

**Syntax**: `math.cos(x)`

**Parameters**:
- `x` (num): Angle in radians

**Returns**: (num) cos(x) in range [-1, 1]

**Examples**:
```graphoid
import "math"

result = math.cos(0)           # 1.0
result = math.cos(math.pi / 2) # 0.0
result = math.cos(math.pi)     # -1.0

# Law of cosines
c_squared = a**2 + b**2 - 2*a*b*math.cos(angle_C)
```

**See also**: `sin()`, `tan()`, `acos()`

---

### math.tan(x)

Returns the tangent of x (x in radians).

**Syntax**: `math.tan(x)`

**Parameters**:
- `x` (num): Angle in radians

**Returns**: (num) tan(x)

**Examples**:
```graphoid
import "math"

result = math.tan(0)            # 0.0
result = math.tan(math.pi / 4)  # 1.0

# Slope from angle
slope = math.tan(angle)

# Angle from slope
angle = math.atan(slope)
```

**See also**: `sin()`, `cos()`, `atan()`

---

### math.asin(x)

Returns the arcsine (inverse sine) of x.

**Syntax**: `math.asin(x)`

**Parameters**:
- `x` (num): Value in range [-1, 1]

**Returns**: (num) Angle in radians in range [-π/2, π/2]

**Examples**:
```graphoid
import "math"

result = math.asin(0)     # 0.0
result = math.asin(1)     # π/2 (1.5708...)
result = math.asin(-1)    # -π/2 (-1.5708...)

# Angle from sine value
angle = math.asin(sine_value)
```

**Errors**: Input outside [-1, 1] raises error

**See also**: `sin()`, `acos()`, `atan()`

---

### math.acos(x)

Returns the arccosine (inverse cosine) of x.

**Syntax**: `math.acos(x)`

**Parameters**:
- `x` (num): Value in range [-1, 1]

**Returns**: (num) Angle in radians in range [0, π]

**Examples**:
```graphoid
import "math"

result = math.acos(1)     # 0.0
result = math.acos(0)     # π/2
result = math.acos(-1)    # π

# Angle from cosine value
angle = math.acos(cosine_value)

# Law of cosines (find angle)
cos_C = (a**2 + b**2 - c**2) / (2*a*b)
angle_C = math.acos(cos_C)
```

**Errors**: Input outside [-1, 1] raises error

**See also**: `cos()`, `asin()`, `atan()`

---

### math.atan(x)

Returns the arctangent (inverse tangent) of x.

**Syntax**: `math.atan(x)`

**Parameters**:
- `x` (num): Value

**Returns**: (num) Angle in radians in range [-π/2, π/2]

**Examples**:
```graphoid
import "math"

result = math.atan(0)     # 0.0
result = math.atan(1)     # π/4
result = math.atan(-1)    # -π/4

# Angle from slope
slope = (y2 - y1) / (x2 - x1)
angle = math.atan(slope)
```

**See also**: `tan()`, `atan2()`

---

### math.atan2(y, x)

Returns the angle from the origin to point (x, y).

**Syntax**: `math.atan2(y, x)`

**Parameters**:
- `y` (num): Y-coordinate
- `x` (num): X-coordinate

**Returns**: (num) Angle in radians in range [-π, π]

**Examples**:
```graphoid
import "math"

# Full range angle (handles all quadrants)
angle = math.atan2(y, x)

result = math.atan2(1, 1)    # π/4 (45°)
result = math.atan2(1, -1)   # 3π/4 (135°)
result = math.atan2(-1, -1)  # -3π/4 (-135°)
result = math.atan2(-1, 1)   # -π/4 (-45°)

# Direction to target
dx = target_x - current_x
dy = target_y - current_y
direction = math.atan2(dy, dx)
```

**See also**: `atan()`

---

## Hyperbolic Functions

### math.sinh(x)

Returns the hyperbolic sine of x.

**Syntax**: `math.sinh(x)`

**Parameters**:
- `x` (num): Value

**Returns**: (num) sinh(x) = (e^x - e^(-x)) / 2

**Examples**:
```graphoid
import "math"

result = math.sinh(0)   # 0.0
result = math.sinh(1)   # 1.175...
```

**See also**: `cosh()`, `tanh()`

---

### math.cosh(x)

Returns the hyperbolic cosine of x.

**Syntax**: `math.cosh(x)`

**Parameters**:
- `x` (num): Value

**Returns**: (num) cosh(x) = (e^x + e^(-x)) / 2

**Examples**:
```graphoid
import "math"

result = math.cosh(0)   # 1.0
result = math.cosh(1)   # 1.543...
```

**See also**: `sinh()`, `tanh()`

---

### math.tanh(x)

Returns the hyperbolic tangent of x.

**Syntax**: `math.tanh(x)`

**Parameters**:
- `x` (num): Value

**Returns**: (num) tanh(x) = sinh(x) / cosh(x)

**Examples**:
```graphoid
import "math"

result = math.tanh(0)   # 0.0
result = math.tanh(1)   # 0.762...

# Activation function in neural networks
output = math.tanh(input)
```

**See also**: `sinh()`, `cosh()`

---

## Utility Functions

### math.degrees(radians)

Converts radians to degrees.

**Syntax**: `math.degrees(radians)`

**Parameters**:
- `radians` (num): Angle in radians

**Returns**: (num) Angle in degrees

**Examples**:
```graphoid
import "math"

result = math.degrees(math.pi)      # 180.0
result = math.degrees(math.pi / 2)  # 90.0
result = math.degrees(0)            # 0.0

# Convert output of trig functions
angle_deg = math.degrees(math.asin(0.5))  # 30.0
```

**See also**: `radians()`

---

### math.radians(degrees)

Converts degrees to radians.

**Syntax**: `math.radians(degrees)`

**Parameters**:
- `degrees` (num): Angle in degrees

**Returns**: (num) Angle in radians

**Examples**:
```graphoid
import "math"

result = math.radians(180)  # π
result = math.radians(90)   # π/2
result = math.radians(45)   # π/4

# Use with trig functions
angle_rad = math.radians(30)
sine = math.sin(angle_rad)  # 0.5
```

**See also**: `degrees()`

---

### math.min(values...)

Returns the minimum value.

**Syntax**: `math.min(a, b, ...)`

**Parameters**:
- `values...`: One or more numbers, or a list

**Returns**: (num) Minimum value

**Examples**:
```graphoid
import "math"

result = math.min(5, 3, 8, 1)   # 1
result = math.min([5, 3, 8, 1]) # 1

# Clamp value
clamped = math.max(minimum, math.min(value, maximum))
```

**See also**: `max()`, `list.min()`

---

### math.max(values...)

Returns the maximum value.

**Syntax**: `math.max(a, b, ...)`

**Parameters**:
- `values...`: One or more numbers, or a list

**Returns**: (num) Maximum value

**Examples**:
```graphoid
import "math"

result = math.max(5, 3, 8, 1)   # 8
result = math.max([5, 3, 8, 1]) # 8

# Find peak
peak = math.max(data_points)
```

**See also**: `min()`, `list.max()`

---

### math.clamp(value, min, max)

Clamps a value to a range.

**Syntax**: `math.clamp(value, min, max)`

**Parameters**:
- `value` (num): Value to clamp
- `min` (num): Minimum value
- `max` (num): Maximum value

**Returns**: (num) Clamped value

**Examples**:
```graphoid
import "math"

result = math.clamp(5, 0, 10)    # 5
result = math.clamp(15, 0, 10)   # 10
result = math.clamp(-5, 0, 10)   # 0

# Keep value in valid range
health = math.clamp(health, 0, 100)
volume = math.clamp(volume, 0.0, 1.0)
```

**See also**: `min()`, `max()`

---

### math.sign(x)

Returns the sign of a number.

**Syntax**: `math.sign(x)`

**Parameters**:
- `x` (num): Number

**Returns**: (num) -1, 0, or 1

**Examples**:
```graphoid
import "math"

result = math.sign(5)     # 1
result = math.sign(-3)    # -1
result = math.sign(0)     # 0

# Direction multiplier
velocity = speed * math.sign(direction)
```

**See also**: `abs()`

---

## Advanced Functions

### math.gcd(a, b)

Returns the greatest common divisor.

**Syntax**: `math.gcd(a, b)`

**Parameters**:
- `a` (num): First integer
- `b` (num): Second integer

**Returns**: (num) GCD of a and b

**Examples**:
```graphoid
import "math"

result = math.gcd(48, 18)   # 6
result = math.gcd(100, 25)  # 25

# Reduce fraction
numerator = numerator / math.gcd(numerator, denominator)
denominator = denominator / math.gcd(numerator, denominator)
```

**See also**: `lcm()`

---

### math.lcm(a, b)

Returns the least common multiple.

**Syntax**: `math.lcm(a, b)`

**Parameters**:
- `a` (num): First integer
- `b` (num): Second integer

**Returns**: (num) LCM of a and b

**Examples**:
```graphoid
import "math"

result = math.lcm(12, 18)   # 36
result = math.lcm(4, 6)     # 12

# Find common period
period = math.lcm(period1, period2)
```

**See also**: `gcd()`

---

### math.factorial(n)

Returns the factorial of n.

**Syntax**: `math.factorial(n)`

**Parameters**:
- `n` (num): Non-negative integer

**Returns**: (num) n! = n × (n-1) × ... × 2 × 1

**Examples**:
```graphoid
import "math"

result = math.factorial(5)    # 120
result = math.factorial(10)   # 3628800
result = math.factorial(0)    # 1

# Combinations formula
combinations = math.factorial(n) / (math.factorial(k) * math.factorial(n - k))
```

**Errors**: Negative input raises error

**See also**: `pow()`

---

### math.is_nan(x)

Tests if a value is NaN.

**Syntax**: `math.is_nan(x)`

**Parameters**:
- `x` (num): Value to test

**Returns**: (bool) `true` if NaN, `false` otherwise

**Examples**:
```graphoid
import "math"

result = math.is_nan(math.nan)       # true
result = math.is_nan(0.0 / 0.0)      # true
result = math.is_nan(5)              # false

# Validate calculation
if math.is_nan(result) {
    print("Calculation produced undefined result")
}
```

**See also**: `is_infinite()`, `nan`

---

### math.is_infinite(x)

Tests if a value is infinite.

**Syntax**: `math.is_infinite(x)`

**Parameters**:
- `x` (num): Value to test

**Returns**: (bool) `true` if infinite, `false` otherwise

**Examples**:
```graphoid
import "math"

result = math.is_infinite(math.infinity)  # true
result = math.is_infinite(1.0 / 0.0)      # true
result = math.is_infinite(5)              # false

# Bounds checking
if math.is_infinite(value) {
    print("Value out of bounds")
}
```

**See also**: `is_nan()`, `infinity`

---

## See Also

- [num](../core/num.md) - Number type and operators
- [statistics](statistics.md) - Statistical functions
- [constants](constants.md) - More mathematical constants
- [random](random.md) - Random number generation
