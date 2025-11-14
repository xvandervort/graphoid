# approx - Approximate Equality Module

The `approx` module provides functions for comparing values with tolerance, essential for working with floating-point numbers and time-based comparisons where exact equality is impractical or impossible.

## Overview

Floating-point arithmetic often produces results that are "close enough" but not exactly equal due to rounding errors. The `approx` module solves this by allowing you to specify acceptable tolerances for equality comparisons.

**Quick Example:**
```graphoid
import "approx"

# Floating-point calculation
calculated = 0.1 + 0.2  # Might be 0.30000000000000004
expected = 0.3

if approx.equal(calculated, expected, 0.0001) {
    print("Values match!")  # ✓ This will execute
}
```

## Functions

### equal(a, b, tolerance)

Checks if two values are approximately equal within an absolute tolerance.

**Parameters:**
- `a` - First value to compare
- `b` - Second value to compare
- `tolerance` - Maximum absolute difference allowed (number)

**Returns:** `true` if `|a - b| < tolerance`, `false` otherwise

**Examples:**
```graphoid
import "approx"

# Basic usage
approx.equal(3.14159, 3.14, 0.01)      # true (difference: 0.00159)
approx.equal(3.14159, 3.14, 0.001)     # false (difference: 0.00159)

# Floating-point precision issues
result = 0.1 + 0.2
approx.equal(result, 0.3, 0.0001)      # true

# Scientific calculations
measured = 9.807
expected = 9.81
approx.equal(measured, expected, 0.01) # true
```

---

### equal(a, b, tolerance, mode)

Checks if two values are approximately equal using different comparison modes.

**Parameters:**
- `a` - First value to compare
- `b` - Second value to compare
- `tolerance` - Tolerance value (interpretation depends on mode)
- `mode` - Comparison mode symbol (see below)

**Comparison Modes:**

#### `:relative` - Relative/Percentage Comparison
Compares values as a percentage of the larger value. Useful when comparing numbers of different magnitudes.

```graphoid
# 99 is within 2% of 100
approx.equal(100.0, 99.0, 0.02, :relative)    # true (1% difference)

# 95 is NOT within 2% of 100
approx.equal(100.0, 95.0, 0.02, :relative)    # false (5% difference)

# Works with large numbers
approx.equal(1000000.0, 990000.0, 0.02, :relative)  # true (1% difference)
```

**Formula:** `|a - b| / max(|a|, |b|) < tolerance`

#### `:seconds` - Time Comparison (Seconds)
Compares two time values within a tolerance measured in seconds.

```graphoid
time1 = time.from_timestamp(1704067200)
time2 = time.from_timestamp(1704067203)  # 3 seconds later

approx.equal(time1, time2, 5, :seconds)  # true (within 5 seconds)
approx.equal(time1, time2, 2, :seconds)  # false (exceeds 2 seconds)
```

#### `:minutes` - Time Comparison (Minutes)
Compares two time values within a tolerance measured in minutes.

```graphoid
time1 = time.from_timestamp(1704067200)
time2 = time.from_timestamp(1704067500)  # 5 minutes later

approx.equal(time1, time2, 10, :minutes)  # true (within 10 minutes)
approx.equal(time1, time2, 3, :minutes)   # false (exceeds 3 minutes)
```

#### `:hours` - Time Comparison (Hours)
Compares two time values within a tolerance measured in hours.

```graphoid
time1 = time.from_timestamp(1704067200)
time2 = time.from_timestamp(1704074400)  # 2 hours later

approx.equal(time1, time2, 3, :hours)  # true (within 3 hours)
approx.equal(time1, time2, 1, :hours)  # false (exceeds 1 hour)
```

#### `:days` - Time Comparison (Days)
Compares two time values within a tolerance measured in days.

```graphoid
time1 = time.from_timestamp(1704067200)
time2 = time.from_timestamp(1704153600)  # 1 day later

approx.equal(time1, time2, 2, :days)  # true (within 2 days)
approx.equal(time1, time2, 0.5, :days)  # false (exceeds 12 hours)
```

---

### eq(a, b, tolerance [, mode])

Alias for `equal()`. Shorter name for convenience.

**Examples:**
```graphoid
import "approx"

approx.eq(3.14, 3.15, 0.1)              # Same as equal()
approx.eq(100, 99, 0.02, :relative)     # Same as equal() with mode
```

---

### within(a, b, tolerance [, mode])

Alias for `equal()`. More natural phrasing for some use cases.

**Examples:**
```graphoid
import "approx"

approx.within(2.5, 2.501, 0.01)         # Same as equal()
approx.within(100, 99, 0.02, :relative) # Same as equal() with mode
```

---

## Common Use Cases

### 1. Floating-Point Arithmetic
```graphoid
import "approx"

# Division and multiplication
result = 10.0 / 3.0 * 3.0
approx.equal(result, 10.0, 0.0001)  # true

# Trigonometry (if available)
angle_radians = 3.14159
approx.equal(angle_radians, 3.14159265, 0.00001)  # true
```

### 2. Scientific Measurements
```graphoid
import "approx"

# Sensor readings with expected values
temperature = 98.6
measured = 98.63

if approx.equal(measured, temperature, 0.5) {
    print("Temperature is normal")
}

# Lab measurements with relative tolerance
concentration_a = 0.0523
concentration_b = 0.0519

if approx.equal(concentration_a, concentration_b, 0.01, :relative) {
    print("Concentrations match within 1%")
}
```

### 3. Time-Based Scheduling
```graphoid
import "approx"

# Meeting attendance
meeting_time = time.from_numbers(2025, 1, 15, 14, 30, 0)
arrival_time = time.from_numbers(2025, 1, 15, 14, 27, 0)

if approx.equal(meeting_time, arrival_time, 5, :minutes) {
    print("On time!")  # ✓ Arrived 3 minutes early
}

# Event synchronization
event_start = time.now()
user_action = time.from_timestamp(event_start.to_num() + 1.5)

if approx.equal(event_start, user_action, 3, :seconds) {
    print("User responded quickly")
}
```

### 4. Financial Calculations
```graphoid
import "approx"

# Price comparisons (absolute tolerance for currency)
listed_price = 19.99
charged_price = 20.00

if approx.equal(listed_price, charged_price, 0.05) {
    print("Price matches within 5 cents")
}

# Percentage-based price matching
original = 1000.0
discounted = 950.0

if approx.equal(original, discounted, 0.10, :relative) {
    print("Price within 10% of original")  # ✓ 5% discount
}
```

### 5. Testing and Validation
```graphoid
import "approx"

# Function output validation
fn calculate_average(values) {
    sum = values.reduce(0, fn(acc, x) { return acc + x })
    return sum / values.length()
}

result = calculate_average([1.0, 2.0, 3.0])
expected = 2.0

if approx.equal(result, expected, 0.0001) {
    print("Test passed!")
}
```

---

## Choosing the Right Tolerance

### Absolute Tolerance (`equal(a, b, tolerance)`)

**Use when:**
- Working with numbers of similar magnitude
- You need a fixed margin of error
- Comparing currency or measurements with known precision

**Guideline:**
- Scientific: 0.001 to 0.0001 (1-4 decimal places)
- Financial: 0.01 (cents/pennies)
- General: 0.1 to 1.0 (depending on scale)

**Example:**
```graphoid
# Comparing temperatures in Celsius (0-100 range)
approx.equal(36.5, 36.7, 0.5)  # ✓ Within half a degree
```

### Relative Tolerance (`equal(a, b, tolerance, :relative)`)

**Use when:**
- Comparing numbers of vastly different magnitudes
- You want percentage-based comparison
- Working with scientific notation or large datasets

**Guideline:**
- Strict: 0.01 (1%)
- Normal: 0.05 (5%)
- Loose: 0.10 (10%)

**Example:**
```graphoid
# Comparing small and large numbers with same relative error
approx.equal(1000.0, 1050.0, 0.10, :relative)  # ✓ 5% difference
approx.equal(0.001, 0.00105, 0.10, :relative)  # ✓ 5% difference
```

### Time Tolerance

**Use when:**
- Scheduling and event timing
- Log file analysis
- User interaction timing

**Guideline:**
- Real-time: 1-5 seconds
- User actions: 5-30 seconds
- Scheduling: 5-15 minutes
- Date-based: 1-7 days

**Example:**
```graphoid
# API request timing
request_time = time.now()
response_time = time.from_timestamp(request_time.to_num() + 2.3)

if approx.equal(request_time, response_time, 5, :seconds) {
    print("Response within acceptable timeframe")
}
```

---

## Edge Cases and Behavior

### Zero Values
When comparing with zero, absolute tolerance is used even in relative mode:
```graphoid
import "approx"

approx.equal(0.0, 0.001, 0.01, :relative)  # true (uses absolute comparison)
```

### Negative Numbers
Works correctly with negative numbers in all modes:
```graphoid
import "approx"

approx.equal(-3.14, -3.15, 0.1)              # true (absolute)
approx.equal(-100.0, -99.0, 0.02, :relative) # true (relative)
```

### Time Precision
Time comparisons use the numeric difference in seconds (Unix timestamps):
```graphoid
import "approx"

time1 = time.from_timestamp(1704067200.5)    # Half-second precision
time2 = time.from_timestamp(1704067200.75)

approx.equal(time1, time2, 1, :seconds)  # true (0.25 second difference)
```

---

## Performance Tips

1. **Absolute tolerance is fastest** - Use when possible
2. **Relative mode adds overhead** - Includes division and max calculation
3. **Time modes are efficient** - Simple arithmetic on timestamps
4. **Pre-calculate tolerances** - Don't recompute in loops

```graphoid
# ✓ Good: Pre-calculate tolerance
tolerance = 0.01
for value in measurements {
    if approx.equal(value, target, tolerance) {
        # Process...
    }
}

# ✗ Avoid: Recalculating tolerance
for value in measurements {
    if approx.equal(value, target, 1.0 / 100.0) {  # Computed every iteration
        # Process...
    }
}
```

---

## API Reference Summary

| Function | Parameters | Description |
|----------|------------|-------------|
| `equal(a, b, tol)` | 3 | Absolute tolerance comparison |
| `equal(a, b, tol, mode)` | 4 | Mode-based comparison |
| `eq(a, b, tol)` | 3 | Alias for `equal()` |
| `eq(a, b, tol, mode)` | 4 | Alias for `equal()` with mode |
| `within(a, b, tol)` | 3 | Alias for `equal()` |
| `within(a, b, tol, mode)` | 4 | Alias for `equal()` with mode |

**Modes:**
- `:relative` - Percentage-based comparison
- `:seconds` - Time difference in seconds
- `:minutes` - Time difference in minutes
- `:hours` - Time difference in hours
- `:days` - Time difference in days

---

## See Also

- **time module** - For creating and manipulating time values
- **Number methods** - `.round()`, `.abs()`, etc. for value manipulation
- **Testing framework** - Use `approx` in test assertions

---

## Implementation Note

The `approx` module is implemented entirely in Graphoid (not Rust), demonstrating the language's capability for self-implementation. You can view the source at `stdlib/approx.gr`.
