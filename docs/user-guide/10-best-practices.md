# Chapter 10: Best Practices

This final chapter distills the wisdom from all previous chapters into practical guidelines for writing clean, idiomatic, and maintainable Graphoid code.

## The Graphoid Philosophy

Graphoid embraces three core principles:

1. **KISS** - Keep It Simple, Stupid! Despise unnecessary verbiage
2. **No Hidden Side Effects** - Operations never mutate operands unless explicitly requested
3. **Graph-Theoretic Foundation** - Leverage graphs, don't fight them

## Configuration & Directives

### Configure at Program Start

Place all directives and imports at the top of your file, before any code:

```graphoid
# ✅ GOOD: Directives first, then imports, then code
:precision 32
:32bit

import "math"
import "statistics"

# Now your code begins
result = math.sqrt(2)
```

```graphoid
# ❌ BAD: Directives scattered throughout
import "math"

x = math.sqrt(2)

:precision 32  # Too late! Should be at top

y = math.sqrt(3)
```

### Group Imports Logically

Organize imports in a consistent order for readability:

```graphoid
# ✅ GOOD: Logical grouping
# 1. Standard library modules
import "math"
import "io"
import "collections"

# 2. Project/local modules
import "./utils_module"
import "./service_module"

# 3. Relative imports from subdirectories
import "./validators/user_validator"
```

```graphoid
# ❌ BAD: Random order
import "./service_module"
import "math"
import "./utils_module"
import "io"
```

## Code Style

### Keep It Simple

```graphoid
# ✅ GOOD: Simple and clear
fn calculate_total(items) {
    return items.map("price").reduce((a, b) => a + b, 0)
}

# ❌ BAD: Overly complex
fn calculate_total(items) {
    total = 0
    for i in range(items.length()) {
        item = items[i]
        if item.has_key("price") {
            price = item["price"]
            if price.is_number() {
                total = total + price
            }
        }
    }
    return total
}
```

### Prefer Type Inference

```graphoid
# ✅ GOOD: Let Graphoid infer types
numbers = [1, 2, 3, 4, 5]
name = "Alice"
age = 30

# ❌ BAD: Unnecessary type annotations
list<num> numbers = [1, 2, 3, 4, 5]
string name = "Alice"
num age = 30
```

**When to use explicit types:**
- Runtime validation is needed
- API boundaries
- Documenting complex code

### Use Named Transformations

```graphoid
# ✅ GOOD: Clear intent with named transformations
doubled = numbers.map("double")
positives = numbers.filter("positive")
evens = numbers.filter("even")

# ❌ LESS CLEAR: Lambda for simple operations
doubled = numbers.map(x => x * 2)
positives = numbers.filter(x => x > 0)
evens = numbers.filter(x => x % 2 == 0)
```

**When to use lambdas:**
- Complex transformations
- Context-specific logic
- Operations not covered by named transformations

## Function Design

### Keep Functions Small

```graphoid
# ✅ GOOD: Small, focused functions
fn validate_user(user) {
    validate_name(user.name)
    validate_email(user.email)
    validate_age(user.age)
}

fn validate_name(name) {
    if name.length() < 2 {
        return "Name too short"
    }
}

# ❌ BAD: Giant function doing everything
fn validate_user(user) {
    # 100 lines of validation logic
}
```

**Rule of thumb**: If you can't see the entire function on your screen, it's probably too long.

### Single Responsibility

```graphoid
# ✅ GOOD: Each function does one thing
fn read_file(path) { ... }
fn parse_data(content) { ... }
fn validate_data(data) { ... }

# ❌ BAD: Function doing multiple things
fn read_and_parse_and_validate(path) {
    # Reading
    # Parsing
    # Validating
}
```

### Prefer Pure Functions

```graphoid
# ✅ GOOD: Pure function
fn calculate_tax(amount, rate) {
    return amount * rate
}

# ❌ BAD: Impure function with side effects
total_tax = 0

fn calculate_tax(amount, rate) {
    tax = amount * rate
    total_tax = total_tax + tax  # Side effect!
    return tax
}
```

### Use Descriptive Names

```graphoid
# ✅ GOOD: Clear, descriptive names
fn calculate_shipping_cost(weight, distance) { ... }
fn validate_email_address(email) { ... }
fn find_users_by_age_range(min_age, max_age) { ... }

# ❌ BAD: Vague names
fn calc(w, d) { ... }
fn check(e) { ... }
fn find(a, b) { ... }
```

## Collection Best Practices

### Choose the Right Collection

```graphoid
# ✅ GOOD: List for ordered sequence
scores = [95, 87, 92, 88, 91]

# ✅ GOOD: Hash for key-value lookup
user = {"name": "Alice", "age": 30, "email": "alice@example.com"}

# ✅ GOOD: Tree for sorted data
ages = tree{30, 25, 35, 28, 32}

# ✅ GOOD: Graph for relationships
social_network = graph{}
```

### Chain Transformations

```graphoid
# ✅ GOOD: Elegant chaining
result = data
    .filter("positive")
    .map("square")
    .filter(x => x < 100)
    .reduce((a, b) => a + b, 0)

# ❌ BAD: Temporary variables
temp1 = data.filter("positive")
temp2 = temp1.map("square")
temp3 = temp2.filter(x => x < 100)
result = temp3.reduce((a, b) => a + b, 0)
```

### Don't Modify While Iterating

```graphoid
# ✅ GOOD: Create new collection
evens = numbers.filter("even")

# ❌ BAD: Modify during iteration
for num in numbers {
    if num % 2 != 0 {
        numbers.remove(num)  # Don't do this!
    }
}
```

## Graph Best Practices

### Use Rules for Constraints

```graphoid
# ✅ GOOD: Declarative constraints
dag = graph { type: :dag }
dag.add_rule("no_cycles")
dag.add_rule("max_degree", 3)

# ❌ BAD: Manual validation everywhere
# Every time you add an edge, manually check for cycles
```

### Use Behaviors for Transformations

```graphoid
# ✅ GOOD: Automatic transformation
temperatures.add_rule("none_to_zero")
temperatures.add_rule("validate_range", 95, 105)

# ❌ BAD: Manual transformation
for i in range(temperatures.length()) {
    if temperatures[i] == none {
        temperatures[i] = 0
    }
    if temperatures[i] < 95 {
        temperatures[i] = 95
    }
    if temperatures[i] > 105 {
        temperatures[i] = 105
    }
}
```

### Use the Right Graph Type

```graphoid
# ✅ GOOD: Directed graph for one-way relationships
dependencies = graph { type: :directed }

# ✅ GOOD: Undirected for two-way relationships
friendships = graph { type: :undirected }

# ✅ GOOD: DAG when cycles are forbidden
tasks = graph { type: :dag }
```

## Error Handling

### Fail Fast

```graphoid
# ✅ GOOD: Check preconditions early
fn process_order(order) {
    if order == none { return "Error: No order provided" }
    if not order.has_key("items") { return "Error: No items" }
    if order["items"].length() == 0 { return "Error: Empty order" }

    # Main logic here
    return calculate_total(order["items"])
}

# ❌ BAD: Deep nesting
fn process_order(order) {
    if order != none {
        if order.has_key("items") {
            if order["items"].length() > 0 {
                return calculate_total(order["items"])
            }
        }
    }
    return "Error"
}
```

### Use Meaningful Error Messages

```graphoid
# ✅ GOOD: Specific error message
if age < 0 {
    return "Error: Age cannot be negative (got " + age.to_string() + ")"
}

# ❌ BAD: Vague error
if age < 0 {
    return "Invalid input"
}
```

## Module Organization

### One Concept Per Module

```graphoid
# ✅ GOOD: Focused modules
user_validation.gr    # User validation only
user_model.gr         # User data model only
user_service.gr       # User business logic only

# ❌ BAD: Everything in one place
user.gr               # Validation, model, service, API, etc.
```

### Use Folders for Related Modules

```
myapp/
  user/
    model.gr
    validation.gr
    service.gr
  post/
    model.gr
    validation.gr
    service.gr
```

### Export Only Public API

```graphoid
# File: calculator.gr

# Public API - explicitly exported
export fn add(a, b) { ... }
export fn subtract(a, b) { ... }

# Private - use priv keyword for internal helpers
priv fn validate_input(x) {
    if x == none { return false }
    return true
}

priv fn internal_helper(x) { ... }
```

**Why use `priv`?** It signals intent to other developers (and your future self) that these functions are implementation details, not part of the module's contract.

### Use Meaningful Module Aliases

```graphoid
# ✅ GOOD: Short, clear alias
module user_validation alias uv

# ✅ GOOD: Descriptive for complex names
module business_analytics_service alias analytics

# ❌ BAD: Cryptic alias
module user_validation alias x1
```

### Include Module Version

Add a version at the end of your modules for tracking:

```graphoid
module my_utilities alias utils

# ... all your functions ...

fn helper_one() { ... }
fn helper_two() { ... }

# Version at module end
__version = "1.2.0"
```

### Configure Module Error Handling

Set error handling behavior appropriate for your module's purpose:

```graphoid
# Strict mode for critical modules (fail immediately on errors)
module payment_processor alias pay {
    error_mode: :strict
}

# Lenient mode for data processing (continue on non-fatal errors)
module data_importer alias importer {
    error_mode: :lenient
}

# Collect mode for batch operations (gather all errors, report at end)
module bulk_validator alias validator {
    error_mode: :collect
}
```

**Guidelines:**
- Use `:strict` for financial, security, or safety-critical code
- Use `:lenient` when partial results are acceptable
- Use `:collect` for batch validation or migration scripts

## Performance

### Avoid Premature Optimization

```graphoid
# ✅ GOOD: Write clear code first
result = numbers.filter("positive").map("square")

# ❌ BAD: Premature optimization
# Don't micro-optimize until you've profiled!
result = []
for num in numbers {
    if num > 0 {
        result.append(num * num)
    }
}
```

**Optimize only when:**
1. You've profiled and found a bottleneck
2. The code is correct and tested
3. The optimization makes a measurable difference

### Use Appropriate Data Structures

```graphoid
# ✅ GOOD: Hash for lookups
user_map = {"alice": {...}, "bob": {...}}
user = user_map["alice"]  # O(1)

# ❌ BAD: List for lookups
users = [{"name": "alice", ...}, {"name": "bob", ...}]
user = users.find(u => u["name"] == "alice")  # O(n)
```

### Cache Expensive Computations

```graphoid
# ✅ GOOD: Memoization for expensive functions
cache = {}

fn expensive_calculation(n) {
    key = n.to_string()
    if cache.has_key(key) {
        return cache[key]
    }

    result = # ... expensive work ...
    cache[key] = result
    return result
}
```

## Testing

### Write Testable Code

```graphoid
# ✅ GOOD: Pure, testable function
fn calculate_total(items) {
    return items.map("price").reduce((a, b) => a + b, 0)
}

# Test:
# assert(calculate_total([{price: 10}, {price: 20}]) == 30)

# ❌ BAD: Hard to test
fn calculate_total() {
    items = get_global_items()  # Side effect
    # Hard to control inputs for testing
}
```

### Use Descriptive Test Names

```graphoid
# ✅ GOOD: Clear test names
fn test_calculate_total_with_empty_list() { ... }
fn test_calculate_total_with_multiple_items() { ... }
fn test_calculate_total_handles_none_prices() { ... }

# ❌ BAD: Vague names
fn test1() { ... }
fn test2() { ... }
```

## Documentation

### Comment Why, Not What

```graphoid
# ✅ GOOD: Explains reasoning
# Use binary search because data is already sorted
index = binary_search(data, target)

# ❌ BAD: States the obvious
# Call binary_search function
index = binary_search(data, target)
```

### Document Complex Algorithms

```graphoid
# ✅ GOOD: Algorithm explanation
# Dijkstra's algorithm for shortest path
# Time complexity: O((V + E) log V)
# Space complexity: O(V)
fn shortest_path(graph, start, end) {
    # Implementation
}
```

### Use Module-Level Documentation

```graphoid
# File: validation.gr
# User validation utilities
#
# This module provides validation functions for user data:
#   - validate_email(email)
#   - validate_phone(phone)
#   - validate_age(age)
#
# All functions return true on success or error message on failure.

export fn validate_email(email) { ... }
```

## Common Anti-Patterns

### Avoid Magic Numbers

```graphoid
# ✅ GOOD: Named constants
MAX_RETRIES = 3
TIMEOUT_SECONDS = 30

if retries > MAX_RETRIES { ... }

# ❌ BAD: Magic numbers
if retries > 3 { ... }
```

### Avoid Deep Nesting

```graphoid
# ✅ GOOD: Early returns (guard clauses)
fn process(data) {
    if data == none { return "Error" }
    if data.length() == 0 { return "Error" }

    # Main logic at outer level
    return transform(data)
}

# ❌ BAD: Deep nesting
fn process(data) {
    if data != none {
        if data.length() > 0 {
            # Main logic buried deep
        }
    }
}
```

### Avoid String Concatenation in Loops

```graphoid
# ✅ GOOD: Build list then join
parts = []
for item in items {
    parts.append(item.to_string())
}
result = parts.join(",")

# ❌ BAD: String concatenation in loop
result = ""
for item in items {
    result = result + item.to_string() + ","
}
```

### Don't Repeat Yourself (DRY)

```graphoid
# ✅ GOOD: Reusable function
fn validate_range(value, min, max, name) {
    if value < min or value > max {
        return name + " must be between " + min.to_string() + " and " + max.to_string()
    }
    return true
}

age_valid = validate_range(age, 0, 120, "Age")
score_valid = validate_range(score, 0, 100, "Score")

# ❌ BAD: Duplicated logic
if age < 0 or age > 120 {
    return "Age must be between 0 and 120"
}

if score < 0 or score > 100 {
    return "Score must be between 0 and 100"
}
```

## Summary

Key principles for writing great Graphoid code:

- ✅ **Configure at the top** - Directives and imports before code
- ✅ **Group imports logically** - Stdlib first, then project modules
- ✅ **Keep it simple** - Simplicity beats cleverness
- ✅ **Use named transformations** - Clear intent
- ✅ **Write small functions** - Single responsibility
- ✅ **Prefer pure functions** - No side effects
- ✅ **Chain transformations** - Functional style
- ✅ **Use graph features** - Rules, behaviors, algorithms
- ✅ **Fail fast** - Early validation
- ✅ **One concept per module** - Focused organization
- ✅ **Use `priv` for internals** - Signal implementation details
- ✅ **Version your modules** - Track with `__version`
- ✅ **Set appropriate error modes** - Strict, lenient, or collect
- ✅ **Don't optimize early** - Correct first, fast second
- ✅ **Comment why, not what** - Explain reasoning
- ✅ **Avoid anti-patterns** - No magic numbers, deep nesting, duplication

---

## The Graphoid Way

Remember: **Everything is a graph!**

When in doubt, think about your problem as a graph:
- Variables are nodes in a namespace graph
- Functions form a call graph
- Modules create a dependency graph
- Data structures are graph-backed collections

Embrace the graph-theoretic foundation and let Graphoid's unique features work for you!

---

## Congratulations!

You've completed the Graphoid User Guide! You now have the knowledge to:

- Write clean, idiomatic Graphoid code
- Leverage collections and graph operations
- Organize code into maintainable modules
- Use directives for fine control
- Take advantage of the standard library

**Next Steps:**
- Read the [API Reference](../api-reference/) for detailed function docs
- Explore [Examples](../../examples/) for real-world code
- Join the [Community Forum](https://discuss.graphoid.org)
- Contribute to [Graphoid on GitHub](https://github.com/yourusername/graphoid)

Happy coding with Graphoid! 🚀

---

[← Previous: Standard Library](09-standard-library.md)
