# Number Methods

Number (num) is a built-in numeric type in Glang representing both integers and floating-point values. Numbers support mathematical operations and various utility methods.

## Type Information

### type()
Returns the type of the value.
```glang
x = 42
x.type()  # Returns "num"
```

### methods()
Returns a list of all available methods for numbers.
```glang
x = 3.14
x.methods()  # Returns ["type", "methods", "can", "inspect", "abs", "sqrt", "log", ...]
```

### can(method_name)
Checks if a method is available on the number.
```glang
x = 42
x.can("abs")  # Returns true
x.can("invalid")  # Returns false
```

### inspect()
Returns detailed information about the number.
```glang
x = 3.14159
x.inspect()  # Returns detailed number information
```

### size()
For numbers, returns 1 (atomic value).
```glang
x = 42
x.size()  # Returns 1
```

## Mathematical Functions

### abs()
Returns the absolute value.
```glang
x = -42
x.abs()  # Returns 42

y = 3.14
y.abs()  # Returns 3.14
```

### sqrt()
Returns the square root.
```glang
x = 16
x.sqrt()  # Returns 4.0

y = 2
y.sqrt()  # Returns 1.4142135623730951
```

### pow(exponent)
Raises the number to a power.
```glang
x = 2
x.pow(3)  # Returns 8

y = 10
y.pow(2)  # Returns 100
```

### log()
Returns the natural logarithm (base e).
```glang
x = 2.718281828
x.log()  # Returns approximately 1.0

y = 10
y.log()  # Returns 2.302585092994046
```

### log(base)
Returns the logarithm with a specified base.
```glang
x = 100
x.log(10)  # Returns 2.0

y = 8
y.log(2)  # Returns 3.0
```

## Rounding Methods

### rnd()
Rounds to the nearest integer.
```glang
x = 3.14159
x.rnd()  # Returns 3

y = 3.7
y.rnd()  # Returns 4
```

### rnd(places)
Rounds to a specified number of decimal places.
```glang
x = 3.14159
x.rnd(2)  # Returns 3.14
x.rnd(4)  # Returns 3.1416
```

### rnd_up()
Rounds up to the nearest integer (ceiling).
```glang
x = 3.14
x.rnd_up()  # Returns 4

y = 3.0
y.rnd_up()  # Returns 3
```

### rnd_up(places)
Rounds up to a specified number of decimal places.
```glang
x = 3.14159
x.rnd_up(2)  # Returns 3.15
x.rnd_up(3)  # Returns 3.142
```

### rnd_dwn()
Rounds down to the nearest integer (floor).
```glang
x = 3.14
x.rnd_dwn()  # Returns 3

y = 3.9
y.rnd_dwn()  # Returns 3
```

### rnd_dwn(places)
Rounds down to a specified number of decimal places.
```glang
x = 3.14159
x.rnd_dwn(2)  # Returns 3.14
x.rnd_dwn(3)  # Returns 3.141
```

### to(places)
Truncates to a specified number of decimal places.
```glang
x = 3.14159
x.to(2)  # Returns 3.14
x.to(0)  # Returns 3.0
```

### round()
Alias for rnd(). Rounds to nearest integer.
```glang
x = 3.6
x.round()  # Returns 4
```

## Type Conversion

### to_string()
Converts the number to a string.
```glang
x = 42
x.to_string()  # Returns "42"

y = 3.14
y.to_string()  # Returns "3.14"
```

### to_num()
Returns the number itself (identity operation for numbers).
```glang
x = 42
x.to_num()  # Returns 42
```

### to_bool()
Converts the number to a boolean. Zero is false, non-zero is true.
```glang
(0).to_bool()  # Returns false
(1).to_bool()  # Returns true
(-5).to_bool()  # Returns true
(0.0).to_bool()  # Returns false
(0.1).to_bool()  # Returns true
```

## Immutability Methods

### freeze()
Makes the number immutable. Returns self for chaining.
```glang
x = 42
x.freeze()
x.is_frozen()  # Returns true
```

### is_frozen()
Checks if the number is frozen (immutable).
```glang
x = 3.14
x.freeze()
x.is_frozen()  # Returns true
```

### contains_frozen()
For numbers, returns the same as is_frozen() since numbers are atomic values.
```glang
x = 42
x.freeze()
x.contains_frozen()  # Returns true
```

## Arithmetic Operations

Numbers support standard arithmetic operations:

### Basic Arithmetic
```glang
a = 10
b = 3

a + b  # Addition: 13
a - b  # Subtraction: 7
a * b  # Multiplication: 30
a / b  # Division: 3.333...
a % b  # Modulo: 1
a // b  # Integer division: 3
a ** b  # Exponentiation: 1000
```

### Comparison Operations
```glang
a = 10
b = 20

a == b  # Equal: false
a != b  # Not equal: true
a < b   # Less than: true
a > b   # Greater than: false
a <= b  # Less than or equal: true
a >= b  # Greater than or equal: false
```

### Unary Operations
```glang
x = 42
-x  # Negation: -42
+x  # Identity: 42
```

## Examples

### Mathematical Calculations
```glang
# Calculate circle properties
radius = 5.0
pi = 3.14159

area = pi * radius.pow(2)
circumference = 2 * pi * radius

print("Area: " + area.rnd(2).to_string())
print("Circumference: " + circumference.rnd(2).to_string())
```

### Temperature Conversion
```glang
# Celsius to Fahrenheit
celsius = 25
fahrenheit = celsius * 9 / 5 + 32
print(celsius.to_string() + "°C = " + fahrenheit.to_string() + "°F")

# Fahrenheit to Celsius
f_temp = 77
c_temp = (f_temp - 32) * 5 / 9
print(f_temp.to_string() + "°F = " + c_temp.rnd(1).to_string() + "°C")
```

### Financial Calculations
```glang
# Calculate compound interest
principal = 1000.0
rate = 0.05  # 5% annual
time = 10    # years

amount = principal * (1 + rate).pow(time)
interest = amount - principal

print("Principal: $" + principal.to_string())
print("Amount after " + time.to_string() + " years: $" + amount.rnd(2).to_string())
print("Interest earned: $" + interest.rnd(2).to_string())
```

### Statistical Operations
```glang
# Calculate statistics from a list
values = [10, 20, 30, 40, 50]
sum = values.sum()
count = values.size()
average = sum / count
variance = 0

for value in values {
    diff = value - average
    variance = variance + diff.pow(2)
}
variance = variance / count
std_dev = variance.sqrt()

print("Average: " + average.to_string())
print("Variance: " + variance.to_string())
print("Standard Deviation: " + std_dev.rnd(2).to_string())
```

### Number Formatting
```glang
# Format numbers for display
price = 19.95
tax_rate = 0.08
tax = price * tax_rate
total = price + tax

print("Price: $" + price.to_string())
print("Tax: $" + tax.rnd(2).to_string())
print("Total: $" + total.rnd(2).to_string())
```

### Scientific Calculations
```glang
# Exponential and logarithmic calculations
base = 2
exponent = 10
result = base.pow(exponent)
print(base.to_string() + "^" + exponent.to_string() + " = " + result.to_string())

# Verify with logarithm
log_result = result.log(base)
print("log_" + base.to_string() + "(" + result.to_string() + ") = " + log_result.to_string())

# Natural exponential
e = 2.718281828
x = 1
e_to_x = e.pow(x)
print("e^" + x.to_string() + " = " + e_to_x.to_string())
```

### Precision Control
```glang
# Working with different precision levels
pi = 3.141592653589793

print("Default: " + pi.to_string())
print("0 places: " + pi.rnd(0).to_string())
print("2 places: " + pi.rnd(2).to_string())
print("4 places: " + pi.rnd(4).to_string())
print("Truncate to 3: " + pi.to(3).to_string())
print("Round up to 3: " + pi.rnd_up(3).to_string())
print("Round down to 3: " + pi.rnd_dwn(3).to_string())
```