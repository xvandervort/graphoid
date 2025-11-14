# Function Overloading Implementation

## Summary

Implemented full function overloading support in Graphoid, allowing multiple functions with the same name but different arities (number of parameters).

## Implementation Date

November 13, 2025

## Problem

The approx stdlib module (written in pure Graphoid) defined multiple versions of the `equal()` function with different arities:
- `equal(a, b, tolerance)` - 3 parameters (absolute tolerance)
- `equal(a, b, tolerance, mode)` - 4 parameters (with mode: :relative, :seconds, etc.)

When calling the 4-parameter version, the error occurred:
```
Error: Runtime error: Internal error: arg_values length (4) doesn't match parameter count (3)
```

This was because the function lookup system only stored one function per name, with later definitions overwriting earlier ones.

## Solution

Changed the global function storage from `HashMap<String, Function>` to `HashMap<String, Vec<Function>>`, enabling multiple functions with the same name to coexist. Function resolution now finds the correct overload based on the number of arguments passed.

## Changes Made

### 1. Updated Executor Structure (`src/execution/executor.rs`)

**Changed global_functions type:**
```rust
// Before:
global_functions: HashMap<String, Function>

// After:
global_functions: HashMap<String, Vec<Function>>
```

### 2. Updated Function Declaration

**When a function is declared**, it's now added to a vector instead of overwriting:
```rust
// Store in global functions table (for recursion support and overloading)
self.global_functions
    .entry(name.clone())
    .or_insert_with(Vec::new)
    .push(func.clone());
```

### 3. Updated Function Lookup (Variable Reference)

**When looking up a function by name as a variable**, return the last-defined overload:
```rust
if let Some(funcs) = self.global_functions.get(name) {
    if let Some(func) = funcs.last() {
        Ok(Value::function(func.clone()))
    }
}
```

### 4. Updated Direct Function Calls

**When calling a function directly** (e.g., `greet("Alice")`), find the overload with matching arity:
```rust
if let Expr::Variable { name, .. } = callee {
    let arity = args.len();
    if let Some(overloads) = self.global_functions.get(name) {
        if let Some(func) = overloads.iter().find(|f| f.parameters.len() == arity).cloned() {
            // Call the matched overload
        }
    }
}
```

### 5. Updated Module Function Calls

**When calling a module function** (e.g., `approx.equal(100, 99, 0.02, :relative)`), find the overload with matching arity:
```rust
if !args.is_empty() {
    let arg_values = self.eval_arguments(args)?;
    let arity = arg_values.len();
    if let Some(overloads) = self.global_functions.get(method) {
        if let Some(func) = overloads.iter().find(|f| f.parameters.len() == arity) {
            return self.call_function(func, &arg_values);
        }
    }
}
```

### 6. Updated Module Loading

**When copying module functions**, extend the vector instead of replacing:
```rust
for (func_name, funcs) in module_executor.global_functions {
    self.global_functions
        .entry(func_name)
        .or_insert_with(Vec::new)
        .extend(funcs);
}
```

## Test Results

### Approx Module Tests
All approx module tests pass:
- ✅ Absolute tolerance (3 params)
- ✅ Relative tolerance (4 params with :relative)
- ✅ Time comparisons (4 params with :seconds, :minutes, :hours, :days)
- ✅ All three function aliases: equal(), eq(), within()

### Function Overloading Demo
Created `samples/function_overloading.gr` demonstrating:
- ✅ Multiple overloads of `greet()` with 1, 2, and 3 parameters
- ✅ Multiple overloads of `add()` with 1, 2, and 3 parameters
- ✅ Module function overloading with `approx.equal()`

### Full Test Suite
✅ All 961 tests passing (no regressions)

## Example Usage

### Direct Function Calls
```graphoid
# Define overloaded functions
fn greet(name) {
    return "Hello, " + name + "!"
}

fn greet(name, greeting) {
    return greeting + ", " + name + "!"
}

fn greet(name, greeting, punctuation) {
    return greeting + ", " + name + punctuation
}

# Call with different arities
print(greet("Alice"))                    # "Hello, Alice!"
print(greet("Bob", "Hi"))                # "Hi, Bob!"
print(greet("Charlie", "Hey", "!!!"))    # "Hey, Charlie!!!"
```

### Module Function Calls
```graphoid
import "approx"

# 3-parameter version (absolute tolerance)
result1 = approx.equal(3.14159, 3.14, 0.01)  # true

# 4-parameter version (relative tolerance)
result2 = approx.equal(100.0, 99.0, 0.02, :relative)  # true

# 4-parameter version (time comparison)
time1 = time.from_timestamp(1704067200)
time2 = time.from_timestamp(1704067203)
result3 = approx.equal(time1, time2, 5, :seconds)  # true
```

## Overloading Resolution Rules

1. **By Arity Only**: Overloading is based solely on the number of parameters
2. **Exact Match Required**: The number of arguments must exactly match the number of parameters
3. **First Match Wins**: When multiple overloads have the same arity (shouldn't happen), the first one registered is used
4. **Last Definition for Variables**: When referencing a function as a variable (not calling it), the last-defined overload is returned

## Benefits

1. **Pure Graphoid Stdlib**: Allows stdlib modules to be written entirely in Graphoid without needing Rust implementations
2. **Better API Design**: Enables cleaner APIs with optional parameters via overloading
3. **Dogfooding**: Proves that Graphoid is expressive enough to implement its own standard library
4. **Pattern Matching Compatibility**: Works seamlessly with function pattern matching (Phase 7)

## Files Modified

- `src/execution/executor.rs` - Core implementation
- `stdlib/approx.gr` - Pure Graphoid module demonstrating overloading
- `samples/approx_module_test.gr` - Comprehensive tests
- `samples/function_overloading.gr` - Overloading demo

## Notes

- Function overloading by arity is now a first-class feature
- No breaking changes to existing code
- All existing tests continue to pass
- Module functions and direct function calls both support overloading
