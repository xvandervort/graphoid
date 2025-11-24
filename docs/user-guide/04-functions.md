# Chapter 4: Functions

Functions are the building blocks of reusable code. Graphoid treats functions as first-class citizens, meaning they can be passed around, stored in variables, and returned from other functions.

## Defining Functions

### Basic Function

```graphoid
fn greet(name) {
    return "Hello, " + name + "!"
}

message = greet("Alice")
print(message)  # "Hello, Alice!"
```

### Function with Multiple Parameters

```graphoid
fn add(a, b) {
    return a + b
}

result = add(5, 3)
print(result)  # 8
```

### Function Without Return Value

Functions without an explicit `return` statement return `none`:

```graphoid
fn print_greeting(name) {
    print("Hello, " + name + "!")
    # No return statement - implicitly returns none
}

result = print_greeting("Bob")  # Prints: Hello, Bob!
print(result)                    # none
```

### Early Return

You can return early from a function:

```graphoid
fn check_age(age) {
    if age < 0 {
        return "Invalid age"
    }

    if age < 18 {
        return "Minor"
    }

    return "Adult"
}

print(check_age(15))   # "Minor"
print(check_age(25))   # "Adult"
print(check_age(-5))   # "Invalid age"
```

## Parameters

### Default Parameters

Functions can have default parameter values:

```graphoid
fn greet(name, greeting = "Hello") {
    return greeting + ", " + name + "!"
}

print(greet("Alice"))              # "Hello, Alice!"
print(greet("Bob", "Hi"))          # "Hi, Bob!"
print(greet("Charlie", "Hey"))     # "Hey, Charlie!"
```

### Type-Annotated Parameters

You can add type annotations to parameters:

```graphoid
fn calculate_area(num width, num height) {
    return width * height
}

area = calculate_area(10, 20)    # 200
# area = calculate_area("10", 20) # Error: Type mismatch
```

### Variadic Functions (Variable Arguments)

Functions can accept variable numbers of arguments:

```graphoid
fn sum(...numbers) {
    total = 0
    for num in numbers {
        total = total + num
    }
    return total
}

print(sum(1, 2, 3))        # 6
print(sum(1, 2, 3, 4, 5))  # 15
print(sum(10))             # 10
```

### Mixing Regular and Variadic Parameters

```graphoid
fn greet_all(greeting, ...names) {
    for name in names {
        print(greeting + ", " + name + "!")
    }
}

greet_all("Hello", "Alice", "Bob", "Charlie")
# Prints:
# Hello, Alice!
# Hello, Bob!
# Hello, Charlie!
```

## Return Values

### Single Return Value

```graphoid
fn square(x) {
    return x * x
}
```

### Multiple Return Values (Using Lists)

```graphoid
fn divide_with_remainder(a, b) {
    quotient = a // b
    remainder = a % b
    return [quotient, remainder]
}

result = divide_with_remainder(17, 5)
print(result)  # [3, 2]

# Destructuring (if supported)
# [q, r] = divide_with_remainder(17, 5)
```

### Returning Functions

Functions can return other functions:

```graphoid
fn make_multiplier(factor) {
    fn multiplier(x) {
        return x * factor
    }
    return multiplier
}

times_two = make_multiplier(2)
times_three = make_multiplier(3)

print(times_two(5))    # 10
print(times_three(5))  # 15
```

## Lambda Functions

Lambdas are anonymous functions, perfect for short operations:

### Basic Lambda

```graphoid
# Lambda syntax: parameters => expression
square = x => x * x

print(square(5))  # 25
```

### Lambda with Multiple Parameters

```graphoid
add = (a, b) => a + b

print(add(3, 4))  # 7
```

### Lambda with Block Body

```graphoid
calculate = (x, y) => {
    sum = x + y
    product = x * y
    return [sum, product]
}

result = calculate(3, 4)
print(result)  # [7, 12]
```

### Lambdas in Higher-Order Functions

```graphoid
numbers = [1, 2, 3, 4, 5]

# Map: transform each element
squared = numbers.map(x => x * x)
print(squared)  # [1, 4, 9, 16, 25]

# Filter: keep elements that match condition
evens = numbers.filter(x => x % 2 == 0)
print(evens)  # [2, 4]

# Reduce: combine all elements
sum = numbers.reduce((acc, x) => acc + x, 0)
print(sum)  # 15
```

## Closures

Closures are functions that "close over" variables from their surrounding scope:

### Basic Closure

```graphoid
fn make_counter() {
    count = 0

    fn increment() {
        count = count + 1
        return count
    }

    return increment
}

counter1 = make_counter()
counter2 = make_counter()

print(counter1())  # 1
print(counter1())  # 2
print(counter1())  # 3

print(counter2())  # 1
print(counter2())  # 2
```

Each counter has its own `count` variable!

### Closure with Parameters

```graphoid
fn make_adder(x) {
    return y => x + y
}

add_five = make_adder(5)
add_ten = make_adder(10)

print(add_five(3))   # 8
print(add_ten(3))    # 13
```

### Practical Closure Example

```graphoid
fn create_logger(prefix) {
    fn log(message) {
        print("[" + prefix + "] " + message)
    }
    return log
}

info_log = create_logger("INFO")
error_log = create_logger("ERROR")

info_log("Application started")   # [INFO] Application started
error_log("Connection failed")    # [ERROR] Connection failed
```

## Higher-Order Functions

Functions that take other functions as parameters or return functions:

### Functions as Parameters

```graphoid
fn apply_operation(a, b, operation) {
    return operation(a, b)
}

add = (x, y) => x + y
multiply = (x, y) => x * y

print(apply_operation(5, 3, add))       # 8
print(apply_operation(5, 3, multiply))  # 15
```

### Building a Simple Pipeline

```graphoid
fn pipeline(value, ...functions) {
    result = value
    for func in functions {
        result = func(result)
    }
    return result
}

double = x => x * 2
add_ten = x => x + 10
square = x => x * x

result = pipeline(5, double, add_ten, square)
# 5 -> 10 -> 20 -> 400
print(result)  # 400
```

## Recursion

Functions can call themselves:

### Factorial

```graphoid
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print(factorial(5))  # 120 (5 * 4 * 3 * 2 * 1)
```

### Fibonacci

```graphoid
fn fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

print(fibonacci(7))  # 13
```

### Tail Recursion

Tail recursion is when the recursive call is the last operation:

```graphoid
fn factorial_tail(n, accumulator = 1) {
    if n <= 1 {
        return accumulator
    }
    return factorial_tail(n - 1, n * accumulator)
}

print(factorial_tail(5))  # 120
```

**Note**: Graphoid may optimize tail-recursive functions to avoid stack overflow.

### Recursive List Processing

```graphoid
fn sum_list(numbers) {
    if numbers.length() == 0 {
        return 0
    }
    first = numbers[0]
    rest = numbers.slice(1)
    return first + sum_list(rest)
}

print(sum_list([1, 2, 3, 4, 5]))  # 15
```

## Function Pattern Matching

Functions can be defined multiple times with different patterns (multiple dispatch):

### Pattern Matching on Values

```graphoid
# Define factorial with pattern matching
fn factorial(0) { return 1 }
fn factorial(1) { return 1 }
fn factorial(n) { return n * factorial(n - 1) }

print(factorial(5))  # 120
```

### Pattern Matching on Types

```graphoid
fn process(num x) {
    return x * 2
}

fn process(string s) {
    return s + s
}

fn process(list l) {
    return l.length()
}

print(process(5))          # 10
print(process("hello"))    # "hellohello"
print(process([1, 2, 3]))  # 3
```

### Pattern Matching with Conditions

```graphoid
fn describe(n) when n < 0 {
    return "negative"
}

fn describe(n) when n == 0 {
    return "zero"
}

fn describe(n) when n > 0 {
    return "positive"
}

print(describe(-5))  # "negative"
print(describe(0))   # "zero"
print(describe(10))  # "positive"
```

## Scope and Variable Lifetime

### Local Scope

Variables defined in functions are local:

```graphoid
fn example() {
    local_var = 10
    print(local_var)  # 10
}

example()
# print(local_var)  # Error: Undefined variable
```

### Global Scope

Variables defined outside functions are global:

```graphoid
global_var = 100

fn read_global() {
    print(global_var)  # Can read global
}

read_global()  # 100
```

### Shadowing

Local variables can shadow global ones:

```graphoid
x = 10

fn example() {
    x = 20  # Creates new local x, doesn't modify global
    print(x)  # 20
}

example()
print(x)  # 10 (global x unchanged)
```

## Pure Functions

Pure functions have no side effects and always return the same output for the same input:

```graphoid
# ✅ Pure function
fn add(a, b) {
    return a + b
}

# ❌ Impure function (modifies global state)
counter = 0

fn increment() {
    counter = counter + 1  # Side effect!
    return counter
}

# ❌ Impure function (uses random/IO)
fn get_random() {
    return random.random()  # Non-deterministic
}
```

**Benefits of pure functions**:
- Easier to test
- Easier to reason about
- Can be safely parallelized
- Results can be cached (memoization)

## Common Patterns

### Factory Functions

```graphoid
fn create_person(name, age) {
    return {
        "name": name,
        "age": age,
        "greet": fn() {
            return "Hi, I'm " + name
        }
    }
}

person = create_person("Alice", 30)
print(person.greet())  # "Hi, I'm Alice"
```

### Partial Application

```graphoid
fn multiply(a, b) {
    return a * b
}

fn partial(func, ...fixed_args) {
    return fn(...args) {
        all_args = fixed_args + args
        return func(...all_args)
    }
}

double = partial(multiply, 2)
triple = partial(multiply, 3)

print(double(5))  # 10
print(triple(5))  # 15
```

### Memoization

```graphoid
fn memoize(func) {
    cache = {}

    return fn(arg) {
        if cache.has_key(arg.to_string()) {
            return cache[arg.to_string()]
        }

        result = func(arg)
        cache[arg.to_string()] = result
        return result
    }
}

# Slow fibonacci
fn fib_slow(n) {
    if n <= 1 { return n }
    return fib_slow(n - 1) + fib_slow(n - 2)
}

# Fast memoized fibonacci
fib_fast = memoize(fib_slow)

print(fib_fast(30))  # Much faster!
```

## Best Practices

### Keep Functions Small

```graphoid
# ❌ BAD: Function does too much
fn process_user_data(user) {
    # Validate
    # Transform
    # Save to database
    # Send email
    # Log activity
    # ... 100 lines of code
}

# ✅ GOOD: Break into smaller functions
fn validate_user(user) { ... }
fn transform_user(user) { ... }
fn save_user(user) { ... }
fn notify_user(user) { ... }

fn process_user_data(user) {
    validate_user(user)
    transform_user(user)
    save_user(user)
    notify_user(user)
}
```

### Use Descriptive Names

```graphoid
# ❌ BAD
fn calc(x, y) { return x * y }

# ✅ GOOD
fn calculate_area(width, height) {
    return width * height
}
```

### Single Responsibility

Each function should do one thing well:

```graphoid
# ✅ GOOD: Each function has single responsibility
fn read_file(path) { ... }
fn parse_data(content) { ... }
fn validate_data(data) { ... }
fn process_data(data) { ... }
```

### Prefer Pure Functions

When possible, write functions without side effects:

```graphoid
# ✅ GOOD: Pure function
fn calculate_total(items) {
    total = 0
    for item in items {
        total = total + item.price
    }
    return total
}

# ❌ AVOID: Function with side effect
fn calculate_and_log(items) {
    total = 0
    for item in items {
        total = total + item.price
        print("Processing: " + item.name)  # Side effect
    }
    return total
}
```

## Summary

In this chapter, you learned:

- ✅ Function definition and parameters
- ✅ Default parameters and variadic functions
- ✅ Return values and early returns
- ✅ Lambda functions for concise code
- ✅ Closures and variable capture
- ✅ Higher-order functions
- ✅ Recursion and tail recursion
- ✅ Function pattern matching
- ✅ Scope and variable lifetime
- ✅ Pure functions and common patterns

---

## Quick Reference

```graphoid
# Basic function
fn name(param1, param2) {
    return result
}

# Default parameters
fn greet(name, greeting = "Hello") { ... }

# Variadic
fn sum(...numbers) { ... }

# Lambda
square = x => x * x
add = (a, b) => a + b

# Closure
fn make_counter() {
    count = 0
    return fn() { count = count + 1; return count }
}

# Pattern matching
fn factorial(0) { return 1 }
fn factorial(n) { return n * factorial(n - 1) }
```

---

## Exercises

1. **Power Function**: Write a recursive function that calculates x^n (x to the power of n)

2. **Filter Function**: Implement your own `filter` function that takes a list and a predicate function

3. **Compose**: Write a `compose` function that takes two functions f and g and returns a new function that applies g then f

4. **Sum of Squares**: Write a function that takes a list of numbers and returns the sum of their squares using `map` and `reduce`

5. **Currying**: Write a `curry` function that converts a function of N arguments into N functions of 1 argument each

6. **Binary Search**: Implement binary search recursively

**Solutions** are available in `examples/04-functions/exercises.gr`

---

[← Previous: Control Flow](03-control-flow.md) | [Next: Collections →](05-collections.md)
