# statistics - Statistical Analysis Module

The `statistics` module provides comprehensive statistical functions for analyzing numeric data. It includes measures of central tendency, variability, and distribution analysis.

## Overview

Statistical analysis is essential for understanding datasets, identifying trends, and making data-driven decisions. The `statistics` module implements common statistical measures in pure Graphoid code.

**Quick Example:**
```graphoid
import "statistics"

scores = [85, 92, 78, 90, 88, 95, 82, 87, 91, 89]

print("Average: " + statistics.mean(scores).to_string())
print("Median: " + statistics.median(scores).to_string())
print("Range: " + statistics.range(scores).to_string())
```

**Module Alias:** Can also be imported as `stats` (when module aliasing is implemented)

## Functions

### Central Tendency

#### mean(values)

Calculates the arithmetic mean (average) of a list of numbers.

**Parameters:**
- `values` - List of numeric values

**Returns:** The mean value, or `none` if list is empty

**Examples:**
```graphoid
import "statistics"

numbers = [1, 2, 3, 4, 5]
avg = statistics.mean(numbers)  # 3

test_scores = [85, 92, 78, 90, 88]
class_average = statistics.mean(test_scores)  # 86.6
```

**Time Complexity:** O(n) where n is the length of the list

---

#### mean(values, default)

Calculates the mean with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The mean value, or `default` if list is empty

**Examples:**
```graphoid
import "statistics"

empty = []
result = statistics.mean(empty, 0)  # 0 (uses default)

data = [10, 20, 30]
result = statistics.mean(data, 0)   # 20 (calculates mean)
```

---

#### median(values)

Calculates the median (middle value) of a list of numbers.

**Parameters:**
- `values` - List of numeric values

**Returns:** The median value, or `none` if list is empty

**Behavior:**
- For odd-length lists: Returns the middle element
- For even-length lists: Returns the average of the two middle elements

**Examples:**
```graphoid
import "statistics"

# Odd number of elements
odd_list = [1, 2, 3, 4, 5]
statistics.median(odd_list)  # 3

# Even number of elements
even_list = [1, 2, 3, 4]
statistics.median(even_list)  # 2.5 (average of 2 and 3)

# Unsorted data (automatically sorted)
unsorted = [5, 1, 4, 2, 3]
statistics.median(unsorted)  # 3
```

**Time Complexity:** O(n²) due to bubble sort (efficient enough for typical use cases)

---

#### median(values, default)

Calculates the median with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The median value, or `default` if list is empty

---

#### mode(values)

Finds the most frequently occurring value in a list.

**Parameters:**
- `values` - List of numeric values

**Returns:** The mode value, or `none` if list is empty

**Behavior:**
- If multiple values have the same highest frequency, returns the first one encountered

**Examples:**
```graphoid
import "statistics"

# Clear mode
data1 = [1, 2, 2, 3, 2, 4, 5]
statistics.mode(data1)  # 2 (appears 3 times)

# All values appear once
data2 = [1, 2, 3, 4, 5]
statistics.mode(data2)  # 1 (first value when all same frequency)

# Multiple modes (returns first)
data3 = [1, 1, 2, 2, 3]
statistics.mode(data3)  # 1 (first mode encountered)
```

**Time Complexity:** O(n²) - suitable for small to medium datasets

---

#### mode(values, default)

Finds the mode with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The mode value, or `default` if list is empty

---

### Spread and Variability

#### variance(values)

Calculates the variance (average squared deviation from the mean).

**Parameters:**
- `values` - List of numeric values

**Returns:** The variance, or `none` if list is empty

**Formula:** `variance = Σ(x - mean)² / n`

**Examples:**
```graphoid
import "statistics"

data = [2, 4, 4, 4, 5, 5, 7, 9]
v = statistics.variance(data)  # 4.0

# Low variance (data is consistent)
consistent = [10, 10.5, 10.2, 10.3, 10.1]
statistics.variance(consistent)  # ~0.03

# High variance (data is spread out)
spread = [1, 10, 20, 30, 100]
statistics.variance(spread)  # ~1186.2
```

**Time Complexity:** O(n)

---

#### variance(values, default)

Calculates variance with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The variance, or `default` if list is empty

---

#### std_dev(values)

Calculates the standard deviation (square root of variance).

**Parameters:**
- `values` - List of numeric values

**Returns:** The standard deviation, or `none` if list is empty

**Formula:** `std_dev = √variance`

**Examples:**
```graphoid
import "statistics"

data = [2, 4, 4, 4, 5, 5, 7, 9]
sd = statistics.std_dev(data)  # 2.0

# Interpreting standard deviation
scores = [85, 92, 78, 90, 88, 95, 82, 87, 91, 89]
avg = statistics.mean(scores)       # 87.7
sd = statistics.std_dev(scores)     # ~4.73

# About 68% of values fall within ±1 std dev of mean
# Range: [87.7 - 4.73, 87.7 + 4.73] = [82.97, 92.43]
```

**Time Complexity:** O(n)

---

#### std_dev(values, default)

Calculates standard deviation with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The standard deviation, or `default` if list is empty

---

#### stdev(values [, default])

Alias for `std_dev()`. Provided for compatibility and convenience.

**Examples:**
```graphoid
import "statistics"

data = [1, 2, 3, 4, 5]
statistics.stdev(data)      # Same as std_dev(data)
statistics.stdev([], 0)     # Same as std_dev([], 0)
```

---

#### range(values)

Calculates the range (difference between maximum and minimum values).

**Parameters:**
- `values` - List of numeric values

**Returns:** The range (max - min), or `none` if list is empty

**Examples:**
```graphoid
import "statistics"

temperatures = [68, 72, 75, 70, 73]
temp_range = statistics.range(temperatures)  # 7 (75 - 68)

# Range shows the spread of data
prices = [10.50, 12.00, 9.99, 15.50, 11.25]
price_range = statistics.range(prices)  # 5.51 (15.50 - 9.99)
```

**Time Complexity:** O(n)

---

#### range(values, default)

Calculates range with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The range, or `default` if list is empty

---

### Summary Statistics

#### min(values)

Finds the minimum value in a list.

**Parameters:**
- `values` - List of numeric values

**Returns:** The minimum value, or `none` if list is empty

**Examples:**
```graphoid
import "statistics"

numbers = [10, 25, 5, 30, 15]
statistics.min(numbers)  # 5

negatives = [-10, -5, -20, -1]
statistics.min(negatives)  # -20
```

**Time Complexity:** O(n)

---

#### min(values, default)

Finds minimum with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The minimum value, or `default` if list is empty

---

#### max(values)

Finds the maximum value in a list.

**Parameters:**
- `values` - List of numeric values

**Returns:** The maximum value, or `none` if list is empty

**Examples:**
```graphoid
import "statistics"

numbers = [10, 25, 5, 30, 15]
statistics.max(numbers)  # 30

mixed = [-10, 5, 0, -3, 12]
statistics.max(mixed)  # 12
```

**Time Complexity:** O(n)

---

#### max(values, default)

Finds maximum with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The maximum value, or `default` if list is empty

---

#### sum(values)

Calculates the sum of all values in a list.

**Parameters:**
- `values` - List of numeric values

**Returns:** The sum of all values (0 if list is empty)

**Examples:**
```graphoid
import "statistics"

numbers = [1, 2, 3, 4, 5]
statistics.sum(numbers)  # 15

prices = [10.50, 25.00, 15.75]
total = statistics.sum(prices)  # 51.25
```

**Time Complexity:** O(n)

---

#### sum(values, default)

Calculates sum with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `default` - Value to return if list is empty

**Returns:** The sum, or `default` if list is empty

---

#### count(values)

Counts the number of elements in a list.

**Parameters:**
- `values` - List of values

**Returns:** The number of elements (same as `values.length()`)

**Examples:**
```graphoid
import "statistics"

data = [1, 2, 3, 4, 5]
n = statistics.count(data)  # 5
```

**Note:** This is equivalent to calling `.length()` on the list directly.

**Time Complexity:** O(1)

---

### Distribution Analysis

#### quantile(values, q)

Calculates a quantile (percentile) of a list of numbers.

**Parameters:**
- `values` - List of numeric values
- `q` - Quantile value between 0.0 and 1.0 (e.g., 0.25 for 25th percentile)

**Returns:** The quantile value, or `none` if list is empty

**Common Quantiles:**
- 0.25 - First quartile (Q1), 25th percentile
- 0.50 - Median (Q2), 50th percentile
- 0.75 - Third quartile (Q3), 75th percentile
- 0.90 - 90th percentile
- 0.95 - 95th percentile
- 0.99 - 99th percentile

**Examples:**
```graphoid
import "statistics"

scores = [60, 65, 70, 75, 80, 85, 90, 95, 100]

# First quartile (25th percentile)
q1 = statistics.quantile(scores, 0.25)  # 67.5

# Median (50th percentile)
q2 = statistics.quantile(scores, 0.5)   # 80

# Third quartile (75th percentile)
q3 = statistics.quantile(scores, 0.75)  # 92.5

# Top 10% threshold
top_10 = statistics.quantile(scores, 0.90)  # 97.5
```

**Use Cases:**
- **Grade boundaries:** Find cutoffs for letter grades
- **Performance metrics:** Identify top/bottom performers
- **Outlier detection:** Values beyond 1st or 99th percentile
- **Percentile ranks:** Compare individual values to distribution

**Time Complexity:** O(n²) due to sorting

---

#### quantile(values, q, default)

Calculates quantile with a default value for empty lists.

**Parameters:**
- `values` - List of numeric values
- `q` - Quantile value (0.0 to 1.0)
- `default` - Value to return if list is empty

**Returns:** The quantile value, or `default` if list is empty

---

## Common Use Cases

### 1. Grade Analysis
```graphoid
import "statistics"

test_scores = [85, 92, 78, 90, 88, 95, 82, 87, 91, 89]

# Summary statistics
avg = statistics.mean(test_scores)
print("Class average: " + avg.to_string())  # 87.7

median_score = statistics.median(test_scores)
print("Median score: " + median_score.to_string())  # 88.5

# Spread analysis
score_range = statistics.range(test_scores)
print("Score spread: " + score_range.to_string())  # 17 points

std = statistics.std_dev(test_scores)
print("Standard deviation: " + std.to_string())  # ~4.73

# Grade boundaries using quantiles
a_threshold = statistics.quantile(test_scores, 0.9)  # Top 10%
b_threshold = statistics.quantile(test_scores, 0.7)  # Top 30%
c_threshold = statistics.quantile(test_scores, 0.5)  # Median

print("A grade: " + a_threshold.to_string() + "+")
print("B grade: " + b_threshold.to_string() + "+")
print("C grade: " + c_threshold.to_string() + "+")
```

### 2. Temperature Monitoring
```graphoid
import "statistics"

daily_temps = [72, 68, 75, 70, 73, 69, 74, 71, 72, 70]

avg_temp = statistics.mean(daily_temps)
print("Average temperature: " + avg_temp.to_string() + "°F")

temp_variance = statistics.variance(daily_temps)
temp_std = statistics.std_dev(daily_temps)

print("Temperature stability:")
print("  Variance: " + temp_variance.to_string())
print("  Std Dev: " + temp_std.to_string())

# Low std dev = stable temperatures
# High std dev = variable temperatures
```

### 3. Financial Analysis
```graphoid
import "statistics"

stock_prices = [150.25, 152.50, 148.75, 155.00, 151.25, 153.50]

avg_price = statistics.mean(stock_prices)
price_range = statistics.range(stock_prices)
volatility = statistics.std_dev(stock_prices)

print("Stock Analysis:")
print("  Average price: $" + avg_price.to_string())
print("  Price range: $" + price_range.to_string())
print("  Volatility (std dev): $" + volatility.to_string())
```

### 4. Performance Benchmarking
```graphoid
import "statistics"

response_times = [120, 135, 125, 140, 128, 133, 122, 138, 130, 127]

# Central tendency
median_time = statistics.median(response_times)
avg_time = statistics.mean(response_times)

# Percentiles for SLA analysis
p50 = statistics.quantile(response_times, 0.50)  # Median
p95 = statistics.quantile(response_times, 0.95)  # 95th percentile
p99 = statistics.quantile(response_times, 0.99)  # 99th percentile

print("Response Time Analysis:")
print("  Median (p50): " + p50.to_string() + "ms")
print("  95th percentile: " + p95.to_string() + "ms")
print("  99th percentile: " + p99.to_string() + "ms")
```

### 5. Quality Control
```graphoid
import "statistics"

# Manufacturing measurements (target: 100.0mm)
measurements = [100.1, 99.9, 100.2, 100.0, 99.8, 100.1, 99.9, 100.0]

mean_size = statistics.mean(measurements)
std_dev = statistics.std_dev(measurements)
min_size = statistics.min(measurements)
max_size = statistics.max(measurements)

print("Quality Control Report:")
print("  Target: 100.0mm")
print("  Mean: " + mean_size.to_string() + "mm")
print("  Std Dev: " + std_dev.to_string() + "mm")
print("  Range: [" + min_size.to_string() + ", " + max_size.to_string() + "]")

# Check if within tolerance (±0.3mm)
if std_dev < 0.15 {
    print("✓ Quality: Excellent (low variability)")
}
```

---

## Edge Cases and Behavior

### Empty Lists
```graphoid
import "statistics"

empty = []

# Without defaults - returns none
statistics.mean(empty)      # none
statistics.median(empty)    # none
statistics.variance(empty)  # none

# With defaults - returns default value
statistics.mean(empty, 0)     # 0
statistics.median(empty, -1)  # -1
statistics.variance(empty, 0) # 0
```

### Single Element
```graphoid
import "statistics"

single = [42]

statistics.mean(single)     # 42
statistics.median(single)   # 42
statistics.mode(single)     # 42
statistics.variance(single) # 0
statistics.std_dev(single)  # 0
statistics.range(single)    # 0
```

### Negative Numbers
All functions work correctly with negative numbers:
```graphoid
import "statistics"

negatives = [-5, -2, -8, -1, -3]

statistics.mean(negatives)     # -3.8
statistics.median(negatives)   # -3
statistics.min(negatives)      # -8
statistics.max(negatives)      # -1
statistics.range(negatives)    # 7 (-1 - (-8))
```

### Mixed Positive and Negative
```graphoid
import "statistics"

mixed = [-10, -5, 0, 5, 10]

statistics.mean(mixed)    # 0
statistics.median(mixed)  # 0
statistics.sum(mixed)     # 0
```

---

## Performance Characteristics

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `mean()` | O(n) | O(1) | Single pass |
| `median()` | O(n²) | O(n) | Due to sorting |
| `mode()` | O(n²) | O(1) | Nested loops |
| `variance()` | O(n) | O(1) | Two passes |
| `std_dev()` | O(n) | O(1) | Uses variance |
| `range()` | O(n) | O(1) | Single pass |
| `min()` | O(n) | O(1) | Single pass |
| `max()` | O(n) | O(1) | Single pass |
| `sum()` | O(n) | O(1) | Single pass |
| `quantile()` | O(n²) | O(n) | Due to sorting |

**Notes:**
- Sorting uses bubble sort (O(n²)) - suitable for typical use cases
- For very large datasets (>10,000 elements), consider data streaming approaches
- Most functions require only a single pass through the data

---

## Tips and Best Practices

### 1. Choose the Right Measure
```graphoid
# For symmetric distributions: Use mean
normal_data = [10, 12, 11, 13, 12, 11, 10, 12]
statistics.mean(normal_data)

# For skewed distributions: Use median
skewed_data = [10, 12, 11, 13, 12, 100]  # 100 is an outlier
statistics.median(skewed_data)  # More representative than mean
```

### 2. Combine Multiple Measures
```graphoid
import "statistics"

data = [85, 92, 78, 90, 88, 95, 82, 87, 91, 89]

# Get a complete picture
print("Central Tendency:")
print("  Mean: " + statistics.mean(data).to_string())
print("  Median: " + statistics.median(data).to_string())

print("Spread:")
print("  Range: " + statistics.range(data).to_string())
print("  Std Dev: " + statistics.std_dev(data).to_string())

print("Extremes:")
print("  Min: " + statistics.min(data).to_string())
print("  Max: " + statistics.max(data).to_string())
```

### 3. Handle Empty Data Gracefully
```graphoid
import "statistics"

fn analyze_data(values) {
    if values.length() == 0 {
        print("No data to analyze")
        return
    }

    # Or use defaults
    avg = statistics.mean(values, 0)
    print("Average: " + avg.to_string())
}
```

### 4. Use Quantiles for Thresholds
```graphoid
import "statistics"

scores = [/* ... */]

# Define grade boundaries
a_cutoff = statistics.quantile(scores, 0.90)  # Top 10%
b_cutoff = statistics.quantile(scores, 0.70)  # Top 30%
c_cutoff = statistics.quantile(scores, 0.40)  # Top 60%

fn assign_grade(score) {
    if score >= a_cutoff { return "A" }
    if score >= b_cutoff { return "B" }
    if score >= c_cutoff { return "C" }
    return "D"
}
```

---

## API Reference Summary

### Central Tendency
| Function | Overloads | Description |
|----------|-----------|-------------|
| `mean(values)` | 2 | Arithmetic mean (average) |
| `median(values)` | 2 | Middle value |
| `mode(values)` | 2 | Most frequent value |

### Spread/Variability
| Function | Overloads | Description |
|----------|-----------|-------------|
| `variance(values)` | 2 | Average squared deviation |
| `std_dev(values)` | 2 | Standard deviation |
| `stdev(values)` | 2 | Alias for `std_dev()` |
| `range(values)` | 2 | Max - min |

### Summary Statistics
| Function | Overloads | Description |
|----------|-----------|-------------|
| `min(values)` | 2 | Minimum value |
| `max(values)` | 2 | Maximum value |
| `sum(values)` | 2 | Sum of all values |
| `count(values)` | 1 | Count of elements |

### Distribution
| Function | Overloads | Description |
|----------|-----------|-------------|
| `quantile(values, q)` | 2 | Calculate percentile |

**Note:** All functions with 2 overloads have a variant that accepts a `default` parameter for empty lists.

---

## See Also

- **approx module** - For comparing statistical results with tolerance
- **List methods** - `.sort()`, `.filter()`, `.map()` for data manipulation
- **Number methods** - `.abs()`, `.sqrt()`, `.round()` for calculations

---

## Implementation Note

The `statistics` module is implemented entirely in pure Graphoid (not Rust), demonstrating the language's capability for self-implementation and numerical computing. You can view the source at `stdlib/statistics.gr`.
