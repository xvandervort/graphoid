# Benchmark Module

The benchmark module provides performance measurement and comparison tools for Glang programs. It allows you to time operations, compare performance, and analyze code execution speed.

## Import

```glang
load "stdlib/benchmark.gr"
```

## Core Functions

### `time_operation(operation, iterations)`

Times a function by calling it repeatedly and measuring the total execution time.

**Parameters:**
- `operation` - A function to time (passed as function parameter)
- `iterations` - Number of times to call the function

**Returns:** A map containing timing statistics:
- `iterations` - Number of iterations performed
- `total_duration` - Total time in seconds for all iterations
- `average_duration` - Average time per iteration in seconds
- `operations_per_second` - Operations per second (throughput)

**Example:**
```glang
func my_operation() {
    nums = []
    result = nums.upto(100)
    return result.size()
}

timing = time_operation(my_operation, 10)
print("Average time: " + timing["average_duration"].to_string() + " seconds")
print("Throughput: " + timing["operations_per_second"].to_string() + " ops/sec")
```

### `start_timer()` and `end_timer(start_time)`

Lower-level timing functions for manual timing control.

**`start_timer()`**
- Returns the current time for use as a start timestamp

**`end_timer(start_time)`**
- Takes a start time and returns timing information
- Returns a map with `start_time`, `end_time`, and `duration`

**Example:**
```glang
start = start_timer()
# ... perform some operation ...
timing = end_timer(start)
print("Duration: " + timing["duration"].to_string() + " seconds")
```

## Predefined Benchmarks

The module includes several predefined benchmark functions for testing common operations:

- `benchmark_list_append()` - Tests list append performance (100 elements)
- `benchmark_list_generate()` - Tests list generation performance (1-100 range)
- `benchmark_list_map()` - Tests list mapping performance (100 elements, double transformation)
- `benchmark_list_filter()` - Tests list filtering performance (100 elements, even filter)

These can be used with `time_operation()`:

```glang
append_timing = time_operation(benchmark_list_append, 5)
generate_timing = time_operation(benchmark_list_generate, 5)
```

## Utility Functions

### `format_timing(timing, operation_name)`

Formats timing results into a readable string.

**Parameters:**
- `timing` - Results from `time_operation()`
- `operation_name` - Name of the operation for display

**Returns:** Formatted string with timing details

**Example:**
```glang
timing = time_operation(my_operation, 10)
formatted = format_timing(timing, "My Operation")
print(formatted)
```

### `quick_performance_test()`

Runs a predefined performance test comparing all built-in benchmark operations.

**Example:**
```glang
quick_performance_test()
# Output:
# === Quick Performance Test ===
# List Append: 532.01 ops/sec
# List Generate: 1236.52 ops/sec
# List Map: 294.69 ops/sec
# List Filter: 804.80 ops/sec
```

## Usage Patterns

### Basic Timing

```glang
func fibonacci(n) {
    if n <= 1 { return n }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

# Time a single operation
timing = time_operation(fibonacci, 1)  # Note: fibonacci expects no args here
print("Fibonacci timing: " + timing["average_duration"].to_string() + "s")
```

### Performance Comparison

```glang
func approach_a() {
    # Implementation A
    nums = []
    return nums.upto(1000).filter("even").size()
}

func approach_b() {
    # Implementation B
    nums = []
    result = nums.generate(0, 1000, 2)  # Generate only even numbers
    return result.size()
}

# Compare approaches
timing_a = time_operation(approach_a, 10)
timing_b = time_operation(approach_b, 10)

print("Approach A: " + timing_a["operations_per_second"].to_string() + " ops/sec")
print("Approach B: " + timing_b["operations_per_second"].to_string() + " ops/sec")

if timing_b["operations_per_second"] > timing_a["operations_per_second"] {
    print("Approach B is faster!")
} else {
    print("Approach A is faster!")
}
```

### Algorithm Analysis

```glang
func test_data_size(size) {
    # Create a wrapper function that captures the size
    func test_operation() {
        nums = []
        data = nums.upto(size)
        return data.map("double").filter("even").size()
    }
    return test_operation
}

# Test how performance scales with data size
sizes = [100, 500, 1000, 2000]
for size in sizes {
    operation = test_data_size(size)
    timing = time_operation(operation, 5)
    print("Size " + size.to_string() + ": " + timing["operations_per_second"].to_string() + " ops/sec")
}
```

## Notes

- All timing is performed using the `time` module with high precision
- Function parameters are passed directly - no string dispatch needed
- The module works with any zero-argument function
- For functions that need arguments, create wrapper functions that capture the arguments
- Performance can vary between runs due to system load and other factors
- Run multiple iterations to get more stable timing results

## See Also

- [Time Module Documentation](time.md) - For underlying timing functionality
- [List Methods](../builtins/list_methods.md) - For list operations being benchmarked
- [Function Parameters](../language/functions.md) - For passing functions as parameters