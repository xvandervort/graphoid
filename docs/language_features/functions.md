# Functions in Glang

Functions are first-class citizens in Glang, providing powerful capabilities for organizing and reusing code. Glang supports traditional function declarations, lambda expressions, closures, recursion, and elegant pattern matching functions.

## Table of Contents
- [Function Basics](#function-basics)
- [Function Declaration](#function-declaration)
- [Lambda Expressions](#lambda-expressions)
- [Closures](#closures)
- [Recursion](#recursion)
- [Pattern Matching Functions](#pattern-matching-functions)
- [Higher-Order Functions](#higher-order-functions)
- [Function Scope](#function-scope)
- [Best Practices](#best-practices)

---

## Function Basics

### Simple Function

```glang
func greet(name) {
    return "Hello, " + name + "!"
}

message = greet("Alice")  # "Hello, Alice!"
```

### Multiple Parameters

```glang
func add(x, y) {
    return x + y
}

result = add(15, 27)  # 42
```

### No Parameters

```glang
func get_pi() {
    return 3.14159
}

pi = get_pi()  # 3.14159
```

### Implicit Return

Functions without explicit `return` statements return `none`:

```glang
func print_hello() {
    print("Hello!")
    # Implicitly returns none
}
```

---

## Function Declaration

### Syntax

```glang
func function_name(param1, param2, ...) {
    # function body
    return result
}
```

### Single Statement Functions

```glang
func square(x) {
    return x * x
}

func is_even(n) {
    return n % 2 == 0
}
```

### Multi-Statement Functions

```glang
func calculate_discount(price, percent) {
    discount = price * (percent / 100)
    final_price = price - discount
    return final_price
}
```

### Functions with Local Variables

```glang
func process_list(items) {
    total = 0
    count = items.size()

    for item in items {
        total = total + item
    }

    average = total / count
    return average
}
```

---

## Lambda Expressions

Lambdas are anonymous functions that can be assigned to variables or passed as arguments.

### Basic Lambda Syntax

```glang
# Single parameter - no parentheses needed
double = x => x * 2

# Multiple parameters - parentheses required
multiply = (x, y) => x * y

# No parameters - empty parentheses
get_random = () => 42
```

### Using Lambdas

```glang
# Assign to variable
square = x => x * x
result = square(5)  # 25

# Pass as argument
numbers = [1, 2, 3, 4, 5]
doubled = numbers.map(x => x * 2)  # [2, 4, 6, 8, 10]
```

### Multi-Line Lambda Bodies

```glang
# Lambda with block body
process = x => {
    temp = x * 2
    result = temp + 10
    return result
}
```

### Lambda Expressions vs Named Functions

**Lambda (anonymous):**
```glang
add = (x, y) => x + y
```

**Named Function:**
```glang
func add(x, y) {
    return x + y
}
```

Both work identically after definition!

---

## Closures

Closures capture variables from their surrounding scope:

### Basic Closure

```glang
func make_adder(n) {
    return x => x + n  # Captures 'n' from outer scope
}

add5 = make_adder(5)
result = add5(10)  # 15
```

### Closure with Multiple Variables

```glang
func make_multiplier(factor, offset) {
    return x => (x * factor) + offset
}

transform = make_multiplier(3, 10)
result = transform(5)  # (5 * 3) + 10 = 25
```

### Closures for Configuration

```glang
func create_logger(prefix) {
    return message => print(prefix + ": " + message)
}

info_log = create_logger("INFO")
error_log = create_logger("ERROR")

info_log("System started")   # INFO: System started
error_log("Connection failed")  # ERROR: Connection failed
```

### Closures in Functional Programming

```glang
func filter_by_threshold(threshold) {
    return list => list.filter(x => x > threshold)
}

filter_high = filter_by_threshold(50)
numbers = [20, 60, 30, 80, 45]
high_numbers = filter_high(numbers)  # [60, 80]
```

---

## Recursion

Functions can call themselves, enabling powerful recursive algorithms:

### Simple Recursion

```glang
func factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

result = factorial(5)  # 120
```

### Fibonacci Sequence

```glang
func fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fib_10 = fibonacci(10)  # 55
```

### List Recursion

```glang
func sum_list(items) {
    if items.size() == 0 {
        return 0
    }
    return items.first() + sum_list(items.tail())
}

total = sum_list([1, 2, 3, 4, 5])  # 15
```

### Tree Recursion

```glang
func count_nodes(tree) {
    if tree == none {
        return 0
    }
    return 1 + count_nodes(tree.left) + count_nodes(tree.right)
}
```

---

## Pattern Matching Functions

Pattern matching functions provide elegant syntax for value-based dispatch:

### Basic Pattern Function

```glang
func factorial(n) {
    0 => 1
    1 => 1
    x => x * factorial(x - 1)
}
```

### Multiple Value Patterns

```glang
func describe_number(n) {
    0 => "zero"
    1 => "one"
    2 => "two"
    42 => "the answer"
    x => "number: " + x.to_string()
}
```

### String Patterns

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

### Boolean Patterns

```glang
func negate(flag) {
    true => false
    false => true
}

func describe_bool(b) {
    true => "yes"
    false => "no"
}
```

### Automatic Fallthrough

Pattern functions return `none` if no pattern matches:

```glang
func weekday_name(n) {
    1 => "Monday"
    2 => "Tuesday"
    3 => "Wednesday"
}

day = weekday_name(1)  # "Monday"
invalid = weekday_name(99)  # none
```

See [Pattern Matching Guide](pattern_matching.md) for comprehensive documentation.

---

## Higher-Order Functions

Functions that take other functions as parameters or return functions:

### Functions as Parameters

```glang
func apply_twice(f, x) {
    return f(f(x))
}

double = x => x * 2
result = apply_twice(double, 3)  # 12 (3 * 2 * 2)
```

### Functions Returning Functions

```glang
func make_transformer(operation) {
    if operation == "double" {
        return x => x * 2
    }
    if operation == "square" {
        return x => x * x
    }
    return x => x
}

transformer = make_transformer("square")
result = transformer(5)  # 25
```

### Composing Functions

```glang
func compose(f, g) {
    return x => f(g(x))
}

add_one = x => x + 1
double = x => x * 2

add_then_double = compose(double, add_one)
result = add_then_double(5)  # 12 ((5 + 1) * 2)
```

### Map, Filter, Reduce Patterns

```glang
# Map with custom function
numbers = [1, 2, 3, 4, 5]
squared = numbers.map(x => x * x)  # [1, 4, 9, 16, 25]

# Filter with lambda
evens = numbers.filter(x => x % 2 == 0)  # [2, 4]

# Chain operations
result = numbers
    .filter(x => x > 2)
    .map(x => x * 10)  # [30, 40, 50]
```

---

## Function Scope

### Local Variables

Variables declared inside functions are local:

```glang
func calculate() {
    temp = 42  # Local to calculate()
    return temp * 2
}

result = calculate()  # 84
# temp is not accessible here
```

### Global Variables

Variables declared outside functions are global:

```glang
pi = 3.14159  # Global

func area_of_circle(radius) {
    return pi * radius * radius  # Accesses global 'pi'
}
```

### Parameter Shadowing

Function parameters shadow global variables:

```glang
value = 100  # Global

func process(value) {
    return value * 2  # Uses parameter, not global
}

result = process(5)  # 10 (not 200)
```

### Closure Variable Capture

Closures capture variables from enclosing scope:

```glang
func make_counter() {
    count = 0
    return () => {
        count = count + 1
        return count
    }
}

counter = make_counter()
counter()  # 1
counter()  # 2
counter()  # 3
```

---

## Best Practices

### 1. Use Descriptive Names

**Good:**
```glang
func calculate_total_price(items, tax_rate) {
    subtotal = sum_items(items)
    return subtotal * (1 + tax_rate)
}
```

**Less Clear:**
```glang
func calc(x, y) {
    z = do_thing(x)
    return z * (1 + y)
}
```

### 2. Keep Functions Focused

Each function should do one thing well:

**Good:**
```glang
func validate_email(email) {
    return email.contains("@") and email.contains(".")
}

func send_email(recipient, subject, body) {
    if validate_email(recipient) {
        # send email
        return true
    }
    return false
}
```

**Less Maintainable:**
```glang
func send_email(recipient, subject, body) {
    # Validation mixed with sending logic
    if not (recipient.contains("@") and recipient.contains(".")) {
        return false
    }
    # send email
    return true
}
```

### 3. Use Pattern Matching for Value Dispatch

**Good:**
```glang
func http_status_message(code) {
    200 => "OK"
    404 => "Not Found"
    500 => "Server Error"
    x => "Status: " + x.to_string()
}
```

**More Verbose:**
```glang
func http_status_message(code) {
    if code == 200 { return "OK" }
    if code == 404 { return "Not Found" }
    if code == 500 { return "Server Error" }
    return "Status: " + code.to_string()
}
```

### 4. Prefer Lambdas for Simple Operations

**Good:**
```glang
doubled = numbers.map(x => x * 2)
```

**Unnecessarily Verbose:**
```glang
func double_value(x) {
    return x * 2
}
doubled = numbers.map(double_value)
```

### 5. Use Early Returns

**Good:**
```glang
func process_value(x) {
    if x < 0 {
        return none
    }
    if x == 0 {
        return 1
    }
    return x * 2
}
```

**More Nested:**
```glang
func process_value(x) {
    if x >= 0 {
        if x == 0 {
            return 1
        } else {
            return x * 2
        }
    } else {
        return none
    }
}
```

### 6. Document Complex Functions

```glang
# Calculate the nth Fibonacci number using recursion
# Returns: The Fibonacci number at position n
# Note: Not optimized - exponential time complexity
func fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

### 7. Avoid Deep Nesting

**Good:**
```glang
func validate_user(user) {
    if not user.has_key("email") {
        return false
    }
    if not user.has_key("name") {
        return false
    }
    return validate_email(user["email"])
}
```

**Hard to Read:**
```glang
func validate_user(user) {
    if user.has_key("email") {
        if user.has_key("name") {
            return validate_email(user["email"])
        }
    }
    return false
}
```

---

## Common Function Patterns

### Factory Functions

```glang
func create_user(name, email) {
    return {
        "name": name,
        "email": email,
        "created_at": time.now()
    }
}
```

### Predicate Functions

```glang
func is_valid_age(age) {
    return age >= 0 and age <= 150
}

func is_adult(age) {
    return age >= 18
}
```

### Transformation Functions

```glang
func normalize_string(text) {
    return text.trim().lower()
}

func to_title_case(text) {
    return text.split(" ")
               .map(word => word.capitalize())
               .join(" ")
}
```

### Aggregation Functions

```glang
func sum(numbers) {
    total = 0
    for n in numbers {
        total = total + n
    }
    return total
}

func average(numbers) {
    if numbers.size() == 0 {
        return 0
    }
    return sum(numbers) / numbers.size()
}
```

### Builder Functions

```glang
func build_query(table) {
    return {
        "table": table,
        "where": [],
        "order": none
    }
}

func add_where(query, condition) {
    query["where"].append(condition)
    return query
}
```

---

## Summary

Glang functions offer:

- **Traditional declarations** - Clear, explicit function definitions
- **Lambda expressions** - Concise anonymous functions
- **Closures** - Capture variables from surrounding scope
- **Recursion** - Powerful self-referential algorithms
- **Pattern matching** - Elegant value-based dispatch
- **Higher-order capabilities** - Functions as first-class values
- **Flexible scope** - Local, global, and closure variables

Functions are the building blocks of Glang programs. Master them to write clean, maintainable, and powerful code.

**Next Steps:**
- Deep dive into [Pattern Matching](pattern_matching.md)
- Learn about [Closures and Scope](../GLANG_CHEAT_SHEET.md#functions-and-scope)
- Explore functional programming with [List Operations](../list_generators.md)