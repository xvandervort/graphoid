# Chapter 3: Control Flow

Control flow determines the order in which your code executes. Graphoid provides familiar control structures with some powerful additions.

## Conditionals: if/else

### Basic if Statement

```graphoid
age = 20

if age >= 18 {
    print("You are an adult")
}
```

### if/else

```graphoid
age = 15

if age >= 18 {
    print("You are an adult")
} else {
    print("You are a minor")
}
```

### else if

```graphoid
score = 85

if score >= 90 {
    print("Grade: A")
} else if score >= 80 {
    print("Grade: B")
} else if score >= 70 {
    print("Grade: C")
} else if score >= 60 {
    print("Grade: D")
} else {
    print("Grade: F")
}
```

### Conditional Expressions

You can use conditionals in expressions:

```graphoid
age = 20
status = if age >= 18 { "adult" } else { "minor" }
print(status)  # "adult"
```

### Multiple Conditions

```graphoid
temperature = 75
is_sunny = true

if temperature > 70 and is_sunny {
    print("Perfect beach weather!")
}

if temperature < 32 or temperature > 100 {
    print("Extreme temperature!")
}
```

## Loops: while

### Basic while Loop

```graphoid
count = 0

while count < 5 {
    print(count)
    count = count + 1
}
# Prints: 0, 1, 2, 3, 4
```

### while with Condition

```graphoid
# Read input until user types "quit"
running = true

while running {
    # In real code, you'd get user input here
    command = get_input()

    if command == "quit" {
        running = false
    } else {
        process_command(command)
    }
}
```

### Infinite Loops

```graphoid
# Infinite loop (use with caution!)
while true {
    print("This runs forever")
    # Use break to exit (see below)
}
```

## Loops: for

### Iterating Over Lists

```graphoid
fruits = ["apple", "banana", "orange"]

for fruit in fruits {
    print(fruit)
}
# Prints:
# apple
# banana
# orange
```

### Iterating Over Ranges

```graphoid
# Print numbers 0 to 4
for i in range(5) {
    print(i)
}
# Prints: 0, 1, 2, 3, 4

# Print numbers 1 to 5
for i in range(1, 6) {
    print(i)
}
# Prints: 1, 2, 3, 4, 5

# Print even numbers from 0 to 10
for i in range(0, 11, 2) {
    print(i)
}
# Prints: 0, 2, 4, 6, 8, 10
```

### Iterating Over Hash Keys

```graphoid
person = {
    "name": "Alice",
    "age": 30,
    "city": "Boston"
}

# Iterate over keys
for key in person.keys() {
    print(key + ": " + person[key].to_string())
}
# Prints:
# name: Alice
# age: 30
# city: Boston
```

### Iterating Over Hash Entries

```graphoid
person = {
    "name": "Alice",
    "age": 30
}

# Iterate over key-value pairs
for entry in person.entries() {
    key = entry[0]
    value = entry[1]
    print(key + " = " + value.to_string())
}
```

### Nested Loops

```graphoid
# Multiplication table
for i in range(1, 6) {
    for j in range(1, 6) {
        product = i * j
        print(i.to_string() + " x " + j.to_string() + " = " + product.to_string())
    }
}
```

## Loop Control: break and continue

### break - Exit Loop Early

```graphoid
# Find first negative number
numbers = [5, 3, -2, 8, -1]

for num in numbers {
    if num < 0 {
        print("Found negative: " + num.to_string())
        break  # Exit the loop
    }
}
# Prints: Found negative: -2
# (stops before reaching -1)
```

### continue - Skip to Next Iteration

```graphoid
# Print only even numbers
for i in range(10) {
    if i % 2 != 0 {
        continue  # Skip odd numbers
    }
    print(i)
}
# Prints: 0, 2, 4, 6, 8
```

### break in Nested Loops

```graphoid
# break only exits the innermost loop
for i in range(3) {
    for j in range(3) {
        if j == 1 {
            break  # Exits inner loop only
        }
        print("i=" + i.to_string() + ", j=" + j.to_string())
    }
}
```

## Pattern Matching: match

Pattern matching is a powerful feature for checking values against patterns.

### Basic match

```graphoid
day = "Monday"

match day {
    "Monday" => print("Start of the week"),
    "Friday" => print("Almost weekend!"),
    "Saturday" | "Sunday" => print("Weekend!"),
    _ => print("Middle of the week")
}
```

### match with Values

```graphoid
status_code = 404

result = match status_code {
    200 => "OK",
    404 => "Not Found",
    500 => "Server Error",
    _ => "Unknown Status"
}

print(result)  # "Not Found"
```

### match with Ranges (future feature)

```graphoid
score = 85

grade = match score {
    90..100 => "A",
    80..89 => "B",
    70..79 => "C",
    60..69 => "D",
    _ => "F"
}
```

### match with Conditions

```graphoid
x = 15

match x {
    0 => print("Zero"),
    n if n < 0 => print("Negative"),
    n if n > 0 and n < 10 => print("Small positive"),
    n if n >= 10 => print("Large positive"),
    _ => print("Unknown")
}
# Prints: "Large positive"
```

### match on Types

```graphoid
value = 42

match value {
    n if n.is_number() => print("It's a number"),
    s if s.is_string() => print("It's a string"),
    l if l.is_list() => print("It's a list"),
    _ => print("Unknown type")
}
```

## Guard Clauses

Use early returns to avoid deep nesting:

```graphoid
fn process_user(user) {
    # Guard clause: check preconditions first
    if user == none {
        return "Error: No user provided"
    }

    if not user.has_key("name") {
        return "Error: Missing name"
    }

    if not user.has_key("email") {
        return "Error: Missing email"
    }

    # Main logic - only reached if all checks pass
    return "Welcome, " + user["name"]
}

result = process_user(none)
print(result)  # "Error: No user provided"
```

## Ternary-like Expressions

Graphoid doesn't have a ternary operator, but you can use conditional expressions:

```graphoid
age = 20

# Instead of: status = age >= 18 ? "adult" : "minor"
status = if age >= 18 { "adult" } else { "minor" }
```

## Loop Patterns

### Enumerate Pattern

```graphoid
fruits = ["apple", "banana", "orange"]

# Using enumerate (index and value)
for i in range(fruits.length()) {
    print(i.to_string() + ": " + fruits[i])
}
# Prints:
# 0: apple
# 1: banana
# 2: orange
```

### While with Counter

```graphoid
count = 0
items = ["a", "b", "c", "d", "e"]

while count < items.length() {
    print(items[count])
    count = count + 1
}
```

### Loop Until Pattern

```graphoid
# Loop until condition becomes true
found = false
index = 0
items = [1, 2, 3, 4, 5]

while not found and index < items.length() {
    if items[index] == 3 {
        found = true
        print("Found at index " + index.to_string())
    }
    index = index + 1
}
```

## Common Pitfalls

### Infinite Loops

```graphoid
# ❌ BAD: Forgot to update counter
count = 0
while count < 5 {
    print(count)
    # Missing: count = count + 1
}
# This runs forever!
```

```graphoid
# ✅ GOOD: Update counter
count = 0
while count < 5 {
    print(count)
    count = count + 1
}
```

### Modifying List While Iterating

```graphoid
# ❌ BAD: Modifying list during iteration
numbers = [1, 2, 3, 4, 5]
for num in numbers {
    if num % 2 == 0 {
        numbers.remove(num)  # Don't do this!
    }
}
```

```graphoid
# ✅ GOOD: Create new list or iterate in reverse
numbers = [1, 2, 3, 4, 5]
odd_numbers = []

for num in numbers {
    if num % 2 != 0 {
        odd_numbers.append(num)
    }
}
```

## Best Practices

### Use Meaningful Loop Variables

```graphoid
# ❌ BAD: Unclear variable names
for x in y {
    print(x)
}

# ✅ GOOD: Clear, descriptive names
for student in students {
    print(student)
}
```

### Prefer for Over while When Iterating

```graphoid
# ❌ Less clear
i = 0
while i < items.length() {
    print(items[i])
    i = i + 1
}

# ✅ More clear
for item in items {
    print(item)
}
```

### Keep Conditionals Simple

```graphoid
# ❌ BAD: Complex nested conditions
if x > 0 {
    if y > 0 {
        if z > 0 {
            print("All positive")
        }
    }
}

# ✅ GOOD: Combine conditions
if x > 0 and y > 0 and z > 0 {
    print("All positive")
}
```

### Use Early Returns

```graphoid
# ❌ BAD: Deep nesting
fn check_value(x) {
    if x != none {
        if x > 0 {
            if x < 100 {
                return "Valid"
            } else {
                return "Too large"
            }
        } else {
            return "Not positive"
        }
    } else {
        return "No value"
    }
}

# ✅ GOOD: Early returns
fn check_value(x) {
    if x == none { return "No value" }
    if x <= 0 { return "Not positive" }
    if x >= 100 { return "Too large" }
    return "Valid"
}
```

## Summary

In this chapter, you learned:

- ✅ `if`/`else` conditionals and conditional expressions
- ✅ `while` loops for condition-based iteration
- ✅ `for` loops for iterating over collections
- ✅ `break` and `continue` for loop control
- ✅ `match` expressions for pattern matching
- ✅ Guard clauses and early returns
- ✅ Common pitfalls and best practices

---

## Quick Reference

```graphoid
# Conditionals
if condition { ... }
if condition { ... } else { ... }
if cond1 { ... } else if cond2 { ... } else { ... }

# Loops
while condition { ... }
for item in collection { ... }
for i in range(n) { ... }

# Loop control
break      # Exit loop
continue   # Skip to next iteration

# Pattern matching
match value {
    pattern1 => result1,
    pattern2 => result2,
    _ => default
}
```

---

## Exercises

1. **FizzBuzz**: Write a program that prints numbers from 1 to 100, but:
   - For multiples of 3, print "Fizz"
   - For multiples of 5, print "Buzz"
   - For multiples of both 3 and 5, print "FizzBuzz"

2. **Find Maximum**: Write a function that finds the maximum value in a list without using built-in max()

3. **Count Vowels**: Write a program that counts how many vowels (a, e, i, o, u) are in a string

4. **Palindrome Checker**: Write a function that checks if a string is a palindrome (reads the same forwards and backwards)

5. **Prime Numbers**: Write a program that prints all prime numbers from 1 to 100

6. **Sum of Evens**: Use a for loop to calculate the sum of all even numbers from 1 to 100

**Solutions** are available in `examples/03-control-flow/exercises.gr`

---

[← Previous: Basics](02-basics.md) | [Next: Functions →](04-functions.md)
