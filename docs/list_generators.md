# List Generators

List generators provide convenient ways to create lists with specific patterns of values. These methods are essential for creating sequences, ranges, and programmatically generated lists.

## Available Methods

### `generate(start, end, step)`

Creates a list of numbers from `start` to `end` (inclusive) with the given `step`.

```glang
nums = []
sequence = nums.generate(1, 10, 2)
print(sequence)  # [1, 3, 5, 7, 9]

# Works with floats too
decimals = nums.generate(0, 1, 0.25)
print(decimals)  # [0, 0.25, 0.5, 0.75, 1]

# Negative steps for descending sequences
countdown = nums.generate(10, 0, -2)
print(countdown)  # [10, 8, 6, 4, 2, 0]
```

**Parameters:**
- `start` (number): The starting value
- `end` (number): The ending value (inclusive)
- `step` (number): The increment between values (cannot be zero)

**Returns:** A new list of numbers with constraint `list<num>`

### `upto(end)`

Creates a list of integers from 0 to `end` (inclusive). This is a convenience method for common counting sequences.

```glang
nums = []

# Count from 0 to 5
count = nums.upto(5)
print(count)  # [0, 1, 2, 3, 4, 5]

# Single element for upto(0)
single = nums.upto(0)
print(single)  # [0]

# Empty list for negative values
empty = nums.upto(-1)
print(empty)  # []
```

**Parameters:**
- `end` (number): The ending value (inclusive)

**Returns:** A new list of numbers with constraint `list<num>`

### `from_function(count, func)`

Creates a list by calling a function with indices from 0 to count-1.

```glang
items = []

# Generate using lambda
double = x => x * 2
evens = items.from_function(5, double)
print(evens)  # [0, 2, 4, 6, 8]

# Generate using regular function
func square(x) {
    return x * x
}
squares = items.from_function(4, square)
print(squares)  # [0, 1, 4, 9]

# Generate strings
make_label = x => "Item " + x.to_string()
labels = items.from_function(3, make_label)
print(labels)  # ["Item 0", "Item 1", "Item 2"]
```

**Parameters:**
- `count` (integer): Number of elements to generate (must be >= 0)
- `func` (function): Function that takes an index (0-based) and returns a value

**Returns:** A new list with inferred type constraint based on generated values

## Common Patterns

### Creating Ranges

```glang
# Traditional range
nums = []
range = nums.generate(1, 100, 1)

# Even numbers
evens = nums.generate(0, 100, 2)

# Powers of 2 using from_function
power_of_2 = x => 2.pow(x)
powers = nums.from_function(10, power_of_2)  # [1, 2, 4, 8, 16, 32, 64, 128, 256, 512]
```

### Combining with Functional Operations

List generators work seamlessly with functional operations like `map`, `filter`, and `each`:

```glang
nums = []

# Generate, filter, and transform
result = nums.generate(1, 20, 1)
    .filter("even")      # Keep only even numbers
    .map("double")       # Double each value
print(result)  # [4, 8, 12, 16, 20, 24, 28, 32, 36, 40]

# Generate and process
nums.upto(10)
    .filter("positive")
    .map("square")
    .each("print")       # Prints: 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
```

### Creating Test Data

```glang
# Generate test scores
random_score = x => 70 + (x * 3) % 30
scores = [].from_function(10, random_score)

# Generate user IDs
make_id = x => "user_" + (1000 + x).to_string()
user_ids = [].from_function(5, make_id)
print(user_ids)  # ["user_1000", "user_1001", "user_1002", "user_1003", "user_1004"]

# Generate coordinates
make_point = x => { "x": x, "y": x * 2 }
points = [].from_function(3, make_point)
```

## Edge Cases and Errors

### Step Cannot Be Zero
```glang
nums = []
result = nums.generate(1, 10, 0)  # Error: generate() step cannot be zero
```

### Type Checking
```glang
nums = []
result = nums.generate("1", 10, 1)  # Error: generate() start must be a number
result = nums.upto("5")             # Error: upto() argument must be a number
result = nums.from_function(-1, x => x)  # Error: from_function() count cannot be negative
result = nums.from_function(5, 42)       # Error: second argument must be a function
```

### Empty Sequences
```glang
nums = []

# Empty when start > end with positive step
empty1 = nums.generate(10, 1, 1)   # []

# Empty when start < end with negative step
empty2 = nums.generate(1, 10, -1)  # []

# Empty with count = 0
empty3 = nums.from_function(0, x => x)  # []
```

## Performance Considerations

- `generate()` and `upto()` create all elements immediately (eager evaluation)
- For very large sequences, consider using `from_function()` with lazy patterns
- All three methods have O(n) time complexity where n is the number of generated elements

## See Also

- [List Methods](list_methods.md) - Complete list method reference
- [Functional Operations](functional_operations.md) - Map, filter, and other transformations
- [Type Constraints](type_constraints.md) - Understanding list type constraints