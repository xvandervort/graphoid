# Error Handling in Glang

*Language Feature Documentation*

## Overview

Glang provides comprehensive error handling capabilities designed for modern software development. The system combines traditional exception handling with functional error-as-data patterns, enhanced with professional-grade stack traces and debugging information.

## Stack Traces and Error Reporting

### Enhanced Error Messages

Glang automatically provides detailed stack traces when errors occur, showing the complete call chain with source context:

```glang
func outer_function(z) {
    return middle_function(z + 1)
}

func middle_function(y) {
    return inner_function(y * 2)
}

func inner_function(x) {
    return missing_var + x  # Error occurs here
}

result = outer_function(5)
```

**Error Output:**
```
Traceback (most recent call last):
  in inner_function() at line 7, column 20
    return missing_var + x
    ~~~~~~~~~~~~~~~~~~~^
    Local variables: {'x': '12'}
  in middle_function() at line 11, column 20
    return inner_function(y * 2)
    ~~~~~~~~~~~~~~~~~~~^
    Local variables: {'y': '6'}
  in outer_function() at line 14, column 18
    return middle_function(z + 1)
    ~~~~~~~~~~~~~~~~~~^
    Local variables: {'z': '5'}
VariableNotFoundError: Variable 'missing_var' not found
```

### Stack Trace Features

- **Complete Call Chain**: Shows every function call leading to the error
- **Source Position**: Exact line and column numbers for each frame
- **Visual Pointers**: `~~~^` markers highlight the exact error location
- **Local Variables**: Context showing relevant variable values
- **Function Arguments**: Parameter values passed to each function
- **Lambda Support**: Stack traces work with lambda expressions too

### Lambda Expression Errors

```glang
func apply_operation(f, x) {
    return f(x)
}

bad_lambda = y => undefined_var + y
result = apply_operation(bad_lambda, 10)
```

**Error Output:**
```
Traceback (most recent call last):
  in <lambda>() at line 5, column 19
    bad_lambda = y => undefined_var + y
    ~~~~~~~~~~~~~~~~~~^
    Local variables: {'y': '10'}
  in apply_operation() at line 2, column 12
    return f(x)
    ~~~~~~~~~~~^
    Local variables: {'f': '<lambda>', 'x': '10'}
VariableNotFoundError: Variable 'undefined_var' not found
```

## Error-as-Data Pattern

### Result Tuples

Glang supports functional error handling using result tuples with `:ok` and `:error` status symbols:

```glang
func safe_divide(a, b) {
    if b == 0 {
        return [:error, "Division by zero"]
    }
    return [:ok, a / b]
}

# Handle results with pattern matching
result = safe_divide(10, 2)
message = match result {
    [:ok, value] => "Success: " + value.to_string(),
    [:error, msg] => "Error: " + msg,
    _ => "Unknown result"
}

print(message)  # "Success: 5"
```

### Error Propagation

```glang
func calculate_average(numbers) {
    if numbers.size() == 0 {
        return [:error, "Cannot calculate average of empty list"]
    }

    total = 0
    for num in numbers {
        total = total + num
    }

    return [:ok, total / numbers.size()]
}

func process_data(data) {
    avg_result = calculate_average(data)

    return match avg_result {
        [:ok, avg] => [:ok, "Average is: " + avg.to_string()],
        [:error, msg] => [:error, "Processing failed: " + msg],
        _ => [:error, "Unexpected result format"]
    }
}

# Usage
numbers = [1, 2, 3, 4, 5]
result = process_data(numbers)

final_message = match result {
    [:ok, msg] => msg,
    [:error, err] => "ERROR: " + err,
    _ => "Unknown error"
}

print(final_message)  # "Average is: 3"
```

## Error Types

### Built-in Error Types

Glang provides several built-in error types, all with enhanced stack trace support:

- **VariableNotFoundError**: Variable referenced but not defined
- **TypeError**: Type mismatch or invalid type operation
- **IndexError**: List or string index out of bounds
- **KeyError**: Hash key not found
- **FunctionNotFoundError**: Function called but not defined
- **ArgumentError**: Wrong number or type of function arguments
- **RuntimeError**: General runtime errors

### Error Handling Examples

```glang
# Variable not found
func test_variable_error() {
    return nonexistent_variable  # VariableNotFoundError
}

# Type error
func test_type_error() {
    text = "hello"
    return text + 42  # TypeError: Cannot add string and number
}

# Index error
func test_index_error() {
    items = [1, 2, 3]
    return items[10]  # IndexError: List index out of range
}

# Key error
func test_key_error() {
    config = { "host": "localhost" }
    return config["missing_key"]  # KeyError: Key not found
}
```

## Best Practices

### 1. Use Error-as-Data for Expected Failures

For operations that commonly fail (like user input validation, file operations, network requests), prefer error-as-data:

```glang
func validate_email(email) {
    if email.contains("@") && email.contains(".") {
        return [:ok, email]
    }
    return [:error, "Invalid email format"]
}

func safe_file_read(filename) {
    # This would use actual file I/O
    if file_exists(filename) {
        content = read_file(filename)
        return [:ok, content]
    }
    return [:error, "File not found: " + filename]
}
```

### 2. Let Exceptions Handle Programming Errors

Use regular exceptions (which get stack traces) for programming errors:

```glang
func calculate_factorial(n) {
    # Programming error - should be caught during development
    if n < 0 {
        # This will generate a stack trace
        return undefined_variable  # Intentional error
    }

    if n == 0 {
        return 1
    }
    return n * calculate_factorial(n - 1)
}
```

### 3. Combine Both Patterns

```glang
func robust_calculation(input) {
    # Validate input (expected failure)
    validation = validate_number(input)

    parsed_num = match validation {
        [:ok, num] => num,
        [:error, msg] => {
            return [:error, "Validation failed: " + msg]
        },
        _ => {
            return [:error, "Unexpected validation result"]
        }
    }

    # Perform calculation (programming errors get stack traces)
    result = complex_math_operation(parsed_num)
    return [:ok, result]
}
```

## Debugging Features

### Local Variable Inspection

Stack traces automatically show relevant local variables for each function call, helping with debugging:

```glang
func complex_calculation(a, b, c) {
    intermediate1 = a * 2
    intermediate2 = b + 5
    factor = c / 2

    # Error here will show all local variables
    return intermediate1 + intermediate2 + missing_var
}
```

**Error shows:**
```
Local variables: {'a': '10', 'b': '15', 'c': '8', 'intermediate1': '20', 'intermediate2': '20', 'factor': '4'}
```

### Source Context

Every stack frame shows the exact source line where the error occurred:

```glang
func problematic_function() {
    x = 10
    y = 20
    result = x + y + undefined_variable  # Error highlighted here
    return result
}
```

**Shows:**
```
result = x + y + undefined_variable
~~~~~~~~~~~~~~~~~^
```

## Performance Considerations

### Stack Trace Collection

- **Zero Overhead**: No performance impact when errors don't occur
- **Lazy Loading**: Source lines loaded only when errors need to display them
- **Memory Efficient**: Variable values are converted to strings to avoid memory leaks
- **Automatic Cleanup**: Stack frames automatically cleaned up when functions exit

### Best Performance Practices

1. **Use error-as-data for hot paths** where errors are common
2. **Let stack traces handle unexpected errors** in development and debugging
3. **Pattern matching is efficient** for result tuple handling
4. **Function calls have minimal overhead** for stack trace collection

## Integration with Language Features

### Pattern Matching

Error-as-data integrates seamlessly with Glang's pattern matching:

```glang
results = [
    safe_divide(10, 2),
    safe_divide(15, 3),
    safe_divide(20, 0)  # This will be an error
]

for result in results {
    message = match result {
        [:ok, value] => "Result: " + value.to_string(),
        [:error, msg] => "Failed: " + msg,
        _ => "Unknown"
    }
    print(message)
}
```

### Function Composition

Error handling works naturally with function composition:

```glang
func pipeline_step1(data) {
    if data.size() == 0 {
        return [:error, "Empty data"]
    }
    return [:ok, data.map("double")]
}

func pipeline_step2(data) {
    if data.filter("positive").size() == 0 {
        return [:error, "No positive numbers"]
    }
    return [:ok, data.filter("positive")]
}

func run_pipeline(input) {
    step1_result = pipeline_step1(input)

    step2_input = match step1_result {
        [:ok, data] => data,
        [:error, msg] => {
            return [:error, "Step 1 failed: " + msg]
        },
        _ => {
            return [:error, "Step 1 returned unexpected format"]
        }
    }

    return pipeline_step2(step2_input)
}
```

## Future Enhancements

### Planned Features

- **Configurable stack trace detail levels**
- **Interactive debugging integration**
- **Error aggregation for batch operations**
- **IDE integration with rich error display**
- **Network-enabled error reporting**

### Potential Extensions

- **Custom error types** with user-defined fields
- **Error recovery mechanisms** with automatic retry
- **Distributed error tracking** across multiple services
- **Performance profiling** integrated with stack traces

## Migration Guide

### From Basic Error Handling

If you have existing Glang code with basic error handling, it will continue to work unchanged. Enhanced stack traces are automatically added to all errors.

**Before (still works):**
```glang
func old_function() {
    return missing_variable  # Basic error message
}
```

**After (automatic enhancement):**
```glang
func old_function() {
    return missing_variable  # Now gets full stack trace
}
```

### Adding Error-as-Data

To add error-as-data patterns to existing code:

**Old approach:**
```glang
func risky_operation(input) {
    # Errors become exceptions
    return input / 0
}
```

**New approach:**
```glang
func risky_operation(input) {
    if input == 0 {
        return [:error, "Cannot divide by zero"]
    }
    return [:ok, input / 2]
}
```

This comprehensive error handling system makes Glang suitable for both rapid prototyping (with excellent debugging) and production applications (with robust error management).