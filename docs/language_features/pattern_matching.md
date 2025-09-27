# Pattern Matching in Glang

**Pattern matching** is one of Glang's most elegant features, bringing functional programming expressiveness to everyday code. Glang offers both explicit match expressions and implicit pattern functions, giving you flexibility in how you write your code.

## Table of Contents
- [Quick Start](#quick-start)
- [Implicit Pattern Functions (Recommended)](#implicit-pattern-functions-recommended)
- [Explicit Match Expressions](#explicit-match-expressions)
- [Pattern Types](#pattern-types)
- [Advanced Patterns](#advanced-patterns)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)

---

## Quick Start

The simplest way to use pattern matching in Glang is with **implicit pattern functions**:

```glang
func factorial(n) {
    0 => 1
    1 => 1
    x => x * factorial(x - 1)
}

result = factorial(5)  # 120
```

That's it! No `match` keyword, no ceremony—just clean, elegant pattern matching.

---

## Implicit Pattern Functions (Recommended)

### Basic Syntax

Pattern functions use the `pattern => expression` syntax directly in function bodies:

```glang
func function_name(parameter) {
    pattern1 => result1
    pattern2 => result2
    pattern3 => result3
}
```

### Simple Literal Matching

Match exact values:

```glang
func get_sound(animal) {
    "dog" => "woof"
    "cat" => "meow"
    "cow" => "moo"
    "bird" => "tweet"
}

sound = get_sound("dog")    # "woof"
unknown = get_sound("fish") # none (automatic fallthrough)
```

### Variable Capture

Use variables to capture and transform values:

```glang
func classify_number(n) {
    0 => "zero"
    1 => "one"
    42 => "the answer"
    x => "number: " + x.to_string()  # Captures any other value
}

classify_number(0)   # "zero"
classify_number(42)  # "the answer"
classify_number(99)  # "number: 99"
```

### Automatic Fallthrough

If no pattern matches, the function returns `none`:

```glang
func weekday_name(n) {
    1 => "Monday"
    2 => "Tuesday"
    3 => "Wednesday"
    4 => "Thursday"
    5 => "Friday"
}

weekday_name(3)  # "Wednesday"
weekday_name(8)  # none (no match)
```

This makes pattern functions practical for real-world use where you can check for `none` and handle appropriately.

### Boolean Patterns

```glang
func boolean_to_word(flag) {
    true => "yes"
    false => "no"
}

func negate_bool(flag) {
    true => false
    false => true
}
```

### Recursive Functions

Pattern matching shines with recursive algorithms:

```glang
# Fibonacci sequence
func fibonacci(n) {
    0 => 0
    1 => 1
    x => fibonacci(x - 1) + fibonacci(x - 2)
}

# List length
func length(list) {
    [] => 0
    items => 1 + length(items.tail())
}

# List sum
func sum(list) {
    [] => 0
    items => items.first() + sum(items.tail())
}
```

---

## Explicit Match Expressions

When you need pattern matching inside a function (not the entire function body), use explicit `match` expressions:

### Basic Syntax

```glang
result = match value {
    pattern1 => expression1
    pattern2 => expression2
    pattern3 => expression3
}
```

### Example Usage

```glang
func process_status(code) {
    message = match code {
        200 => "OK"
        404 => "Not Found"
        500 => "Server Error"
        _ => "Unknown Status"
    }

    print("Status: " + message)
    return message
}
```

### Multiple Match Expressions

```glang
func categorize_user(age, premium) {
    age_category = match age {
        x if x < 13 => "child"
        x if x < 18 => "teen"
        x if x < 65 => "adult"
        _ => "senior"
    }

    access_level = match premium {
        true => "premium"
        false => "basic"
    }

    return age_category + "_" + access_level
}
```

---

## Pattern Types

### 1. Literal Patterns

Match exact values:

```glang
func handle_command(cmd) {
    "start" => start_server()
    "stop" => stop_server()
    "restart" => restart_server()
    "status" => get_status()
    x => "Unknown command: " + x
}
```

### 2. Variable Patterns

Capture values with variables:

```glang
func double(x) {
    n => n * 2  # 'n' captures the input value
}

func describe(value) {
    v => "The value is: " + v.to_string()
}
```

### 3. Wildcard Pattern

Use `_` to match anything without capturing:

```glang
result = match status_code {
    200 => "Success"
    201 => "Created"
    _ => "Other status"  # Matches anything
}
```

### 4. List Patterns

Match list structures:

```glang
func handle_list(items) {
    [] => "empty list"
    [single] => "one item: " + single.to_string()
    [first, second] => "two items"
    many => "multiple items: " + many.size().to_string()
}
```

### 5. Boolean Patterns

```glang
func access_control(is_admin) {
    true => "Full access granted"
    false => "Limited access"
}
```

---

## Advanced Patterns

### Guards (Conditional Patterns)

Add conditions to patterns with `if`:

```glang
func grade_score(score) {
    match score {
        s if s >= 90 => "A"
        s if s >= 80 => "B"
        s if s >= 70 => "C"
        s if s >= 60 => "D"
        _ => "F"
    }
}
```

### Nested Patterns

```glang
func process_result(result) {
    match result {
        [ok, value] => "Success: " + value.to_string()
        [error, message] => "Error: " + message
        _ => "Unknown result format"
    }
}
```

### Type-Based Patterns

```glang
func describe_value(val) {
    match val.get_type() {
        "string" => "text: " + val
        "num" => "number: " + val.to_string()
        "list" => "list with " + val.size().to_string() + " items"
        "bool" => "boolean: " + val.to_string()
        _ => "unknown type"
    }
}
```

---

## Best Practices

### 1. Use Implicit Pattern Functions When Possible

**Good:**
```glang
func factorial(n) {
    0 => 1
    1 => 1
    x => x * factorial(x - 1)
}
```

**Less Ideal:**
```glang
func factorial(n) {
    return match n {
        0 => 1
        1 => 1
        x => x * factorial(x - 1)
    }
}
```

### 2. Order Patterns from Specific to General

```glang
func classify(value) {
    42 => "the answer"      # Most specific
    0 => "zero"
    x => "other: " + x.to_string()  # Most general (catch-all)
}
```

### 3. Use Meaningful Variable Names

**Good:**
```glang
func describe_age(years) {
    age => "Age: " + age.to_string()
}
```

**Less Clear:**
```glang
func describe_age(years) {
    x => "Age: " + x.to_string()  # What is 'x'?
}
```

### 4. Handle the None Case

When using pattern functions, remember they return `none` for no match:

```glang
sound = get_sound("elephant")

if sound == none {
    print("Unknown animal")
} else {
    print("Animal says: " + sound)
}
```

### 5. Use Guards for Complex Conditions

Instead of many similar patterns, use guards:

**Good:**
```glang
func tax_bracket(income) {
    match income {
        i if i < 10000 => 0.10
        i if i < 50000 => 0.20
        i if i < 100000 => 0.30
        _ => 0.40
    }
}
```

**Less Maintainable:**
```glang
func tax_bracket(income) {
    if income < 10000 { return 0.10 }
    if income < 50000 { return 0.20 }
    if income < 100000 { return 0.30 }
    return 0.40
}
```

---

## Common Patterns

### State Machine

```glang
func next_state(current) {
    "idle" => "processing"
    "processing" => "complete"
    "complete" => "idle"
    "error" => "idle"
    s => s  # Stay in unknown states
}
```

### Option/Maybe Pattern

```glang
func unwrap_or_default(value, default) {
    match value {
        none => default
        v => v
    }
}

result = unwrap_or_default(get_config("port"), 8080)
```

### Result Pattern

```glang
func handle_result(result) {
    match result {
        [ok, value] => process_success(value)
        [error, msg] => handle_error(msg)
        _ => handle_unknown()
    }
}
```

### Recursive List Processing

```glang
func map_double(list) {
    [] => []
    items => [items.first() * 2] + map_double(items.tail())
}

func filter_positive(list) {
    [] => []
    items => {
        first = items.first()
        rest = filter_positive(items.tail())
        if first > 0 {
            return [first] + rest
        }
        return rest
    }
}
```

### Command Dispatcher

```glang
func dispatch_command(cmd, args) {
    "create" => create_resource(args)
    "read" => read_resource(args)
    "update" => update_resource(args)
    "delete" => delete_resource(args)
    "list" => list_resources()
    unknown => "Unknown command: " + unknown
}
```

---

## Pattern Matching vs If-Else

### When to Use Pattern Matching

✅ **Use pattern matching when:**
- Matching on multiple discrete values
- Working with recursive data structures
- Implementing state machines
- Writing functional-style code
- The logic is value-based rather than conditional

### When to Use If-Else

✅ **Use if-else when:**
- Simple boolean conditions
- Complex multi-condition logic
- Side effects are primary concern
- Imperative control flow is clearer

### Example Comparison

**Pattern Matching (Better for discrete values):**
```glang
func http_status_message(code) {
    200 => "OK"
    201 => "Created"
    400 => "Bad Request"
    404 => "Not Found"
    500 => "Server Error"
    x => "Status " + x.to_string()
}
```

**If-Else (Better for ranges/conditions):**
```glang
func validate_temperature(temp) {
    if temp < -273.15 {
        return "Below absolute zero - impossible!"
    }
    if temp < 0 {
        return "Freezing"
    }
    if temp < 100 {
        return "Normal range"
    }
    return "Boiling or above"
}
```

---

## Summary

Pattern matching in Glang offers:

- **Elegant syntax** - Clean functional-style code without ceremony
- **Automatic fallthrough** - Returns `none` for unmatched patterns
- **Two styles** - Implicit pattern functions and explicit match expressions
- **Powerful patterns** - Literals, variables, lists, guards, and more
- **Perfect for recursion** - Natural fit for recursive algorithms

Start with simple pattern functions, then explore guards and complex patterns as needed. Pattern matching makes your code more expressive, maintainable, and elegant.

**Next Steps:**
- See [Functions Guide](functions.md) for more on function declaration
- Check [GLANG_CHEAT_SHEET.md](../GLANG_CHEAT_SHEET.md) for quick syntax reference
- Explore [Error Handling](ERROR_HANDLING.md) for result pattern techniques