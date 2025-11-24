# list - List Type

Lists in Graphoid are ordered collections of values. They are internally implemented as linked graphs, making them efficient for both sequential access and graph operations.

## List Literals

### Basic Lists

Lists are created using square brackets:

```graphoid
numbers = [1, 2, 3, 4, 5]
names = ["Alice", "Bob", "Charlie"]
mixed = [1, "hello", true, none]
empty = []
```

### Type Constraints (Optional)

Lists can have optional runtime type constraints:

```graphoid
# Single-parameter type constraint (runtime-checked)
list<num> scores = [95, 87, 92]
list<string> names = ["Alice", "Bob"]

# Type inference (recommended)
scores = [95, 87, 92]  # Infers list
```

**Note**: Graphoid does NOT support multi-parameter or nested generics. See [NO_GENERICS_POLICY.md](../../../dev_docs/NO_GENERICS_POLICY.md).

---

## List Operators

### Indexing (`[]`)

Access elements by position (zero-indexed).

**Syntax**: `list[index]`

**Parameters**:
- `index` (num): Element position (0-based)

**Returns**: Element at position

**Examples**:
```graphoid
numbers = [10, 20, 30, 40, 50]
first = numbers[0]    # 10
third = numbers[2]    # 30

# Negative indices count from end
last = numbers[-1]    # 50
second_last = numbers[-2]  # 40
```

**Errors**: Index out of bounds raises error (or returns `none` with `:bounds_lenient`)

**See also**: `get()`, `slice()`

---

### Slicing (`[start:end]`)

Extract a sublist.

**Syntax**: `list[start:end]`

**Parameters**:
- `start` (num): Starting index (inclusive)
- `end` (num): Ending index (exclusive)

**Returns**: (list) Sublist from `start` to `end`

**Examples**:
```graphoid
numbers = [0, 1, 2, 3, 4, 5]

# Basic slicing
subset = numbers[1:4]   # [1, 2, 3]

# Omit start or end
subset = numbers[:3]    # [0, 1, 2] (from beginning)
subset = numbers[3:]    # [3, 4, 5] (to end)

# Negative indices
subset = numbers[-3:]   # [3, 4, 5] (last 3)

# Copy entire list
copy = numbers[:]       # [0, 1, 2, 3, 4, 5]
```

**See also**: `slice()`, `[]`

---

### Concatenation (`+`)

Joins two lists together.

**Syntax**: `list1 + list2`

**Parameters**:
- `list1` (list): First list
- `list2` (list): Second list

**Returns**: (list) Concatenated list (does NOT modify originals)

**Examples**:
```graphoid
a = [1, 2, 3]
b = [4, 5, 6]
result = a + b
print(result)  # [1, 2, 3, 4, 5, 6]

# Original lists unchanged
print(a)  # [1, 2, 3]
print(b)  # [4, 5, 6]

# Chain multiple lists
all = [1] + [2, 3] + [4, 5]  # [1, 2, 3, 4, 5]
```

**See also**: `append()`, `extend()`

---

### Repetition (`*`)

Repeats a list multiple times.

**Syntax**: `list * count`

**Parameters**:
- `list` (list): List to repeat
- `count` (num): Number of repetitions

**Returns**: (list) Repeated list

**Examples**:
```graphoid
zeros = [0] * 5
print(zeros)  # [0, 0, 0, 0, 0]

pattern = [1, 2] * 3
print(pattern)  # [1, 2, 1, 2, 1, 2]

# Create matrix rows
row = [0] * 10
```

**See also**: `list.generate()`

---

### Element-wise Addition (`.+`)

Adds corresponding elements from two lists.

**Syntax**: `list1 .+ list2`

**Parameters**:
- `list1` (list): First list
- `list2` (list): Second list

**Returns**: (list) Element-wise sum

**Examples**:
```graphoid
a = [1, 2, 3]
b = [10, 20, 30]
result = a .+ b
print(result)  # [11, 22, 33]

# Different lengths: result length is minimum
a = [1, 2, 3, 4]
b = [10, 20]
result = a .+ b
print(result)  # [11, 22]
```

**See also**: `.-`, `.*`, `./`

---

### Element-wise Subtraction (`.-`)

Subtracts corresponding elements.

**Syntax**: `list1 .- list2`

**Parameters**:
- `list1` (list): First list
- `list2` (list): Second list

**Returns**: (list) Element-wise difference

**Examples**:
```graphoid
a = [10, 20, 30]
b = [1, 2, 3]
result = a .- b
print(result)  # [9, 18, 27]
```

**See also**: `.+`, `.*`, `./`

---

### Element-wise Multiplication (`.*`)

Multiplies corresponding elements.

**Syntax**: `list1 .* list2`

**Parameters**:
- `list1` (list): First list
- `list2` (list): Second list

**Returns**: (list) Element-wise product

**Examples**:
```graphoid
a = [2, 3, 4]
b = [10, 20, 30]
result = a .* b
print(result)  # [20, 60, 120]
```

**See also**: `.+`, `.-`, `./`

---

### Element-wise Division (`./`)

Divides corresponding elements.

**Syntax**: `list1 ./ list2`

**Parameters**:
- `list1` (list): First list
- `list2` (list): Second list

**Returns**: (list) Element-wise quotient

**Examples**:
```graphoid
a = [20, 40, 60]
b = [2, 4, 6]
result = a ./ b
print(result)  # [10, 10, 10]
```

**Errors**: Division by zero raises error

**See also**: `.+`, `.-`, `.*`

---

### Membership (`in`)

Tests if a value is in the list.

**Syntax**: `value in list`

**Parameters**:
- `value`: Value to search for
- `list` (list): List to search

**Returns**: (bool) `true` if found, `false` otherwise

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
result = 3 in numbers  # true
result = 10 in numbers # false

# In conditionals
if "admin" in roles {
    print("Admin access granted")
}
```

**See also**: `contains()`, `index_of()`

---

## List Methods

### length()

Returns the number of elements in the list.

**Syntax**: `list.length()`

**Returns**: (num) Number of elements

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
len = numbers.length()
print(len)  # 5

empty = []
print(empty.length())  # 0

# Loop over indices
for i in range(list.length()) {
    print(list[i])
}
```

**See also**: `is_empty()`

---

### append(value)

Adds an element to the end of the list.

**Syntax**: `list.append(value)`

**Parameters**:
- `value`: Value to add

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
numbers = [1, 2, 3]
numbers.append(4)
print(numbers)  # [1, 2, 3, 4]

# Build a list
result = []
for i in range(10) {
    result.append(i)
}
```

**See also**: `prepend()`, `insert()`, `+`

---

### prepend(value)

Adds an element to the beginning of the list.

**Syntax**: `list.prepend(value)`

**Parameters**:
- `value`: Value to add

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
numbers = [2, 3, 4]
numbers.prepend(1)
print(numbers)  # [1, 2, 3, 4]

# Build stack
stack = []
stack.prepend(item)  # Push to front
```

**See also**: `append()`, `insert()`

---

### insert(index, value)

Inserts an element at a specific position.

**Syntax**: `list.insert(index, value)`

**Parameters**:
- `index` (num): Position to insert (0-based)
- `value`: Value to insert

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
numbers = [1, 3, 4]
numbers.insert(1, 2)
print(numbers)  # [1, 2, 3, 4]

# Insert at beginning (same as prepend)
list.insert(0, value)

# Insert at end (same as append)
list.insert(list.length(), value)
```

**See also**: `append()`, `prepend()`

---

### remove(value, mode)

Removes element(s) from the list.

**Syntax**: `list.remove(value, mode)`

**Parameters**:
- `value`: Value to remove
- `mode` (symbol, optional): `:first` (default) or `:all`

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
numbers = [1, 2, 3, 2, 4]

# Remove first occurrence (default)
numbers.remove(2)
print(numbers)  # [1, 3, 2, 4]

# Remove all occurrences
numbers = [1, 2, 3, 2, 4]
numbers.remove(2, :all)
print(numbers)  # [1, 3, 4]

# Remove if exists
if numbers.contains(5) {
    numbers.remove(5)
}
```

**See also**: `remove_at()`, `filter()`

---

### remove_at(index)

Removes element at a specific position.

**Syntax**: `list.remove_at(index)`

**Parameters**:
- `index` (num): Position to remove (0-based)

**Returns**: Removed value

**Examples**:
```graphoid
numbers = [10, 20, 30, 40]
removed = numbers.remove_at(1)
print(removed)  # 20
print(numbers)  # [10, 30, 40]

# Remove last element
last = list.remove_at(-1)

# Remove first element
first = list.remove_at(0)
```

**See also**: `remove()`, `pop()`

---

### pop()

Removes and returns the last element.

**Syntax**: `list.pop()`

**Returns**: Last element (modifies list)

**Examples**:
```graphoid
stack = [1, 2, 3, 4]
top = stack.pop()
print(top)    # 4
print(stack)  # [1, 2, 3]

# Process all elements
while not stack.is_empty() {
    item = stack.pop()
    process(item)
}
```

**Errors**: Calling `pop()` on empty list raises error

**See also**: `remove_at()`, `append()`

---

### clear()

Removes all elements from the list.

**Syntax**: `list.clear()`

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
numbers.clear()
print(numbers)  # []
print(numbers.length())  # 0
```

**See also**: `remove()`

---

### contains(value)

Tests if list contains a value.

**Syntax**: `list.contains(value)`

**Parameters**:
- `value`: Value to search for

**Returns**: (bool) `true` if found, `false` otherwise

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
result = numbers.contains(3)  # true
result = numbers.contains(10) # false

# Same as 'in' operator
result = 3 in numbers  # true
```

**See also**: `in`, `index_of()`

---

### index_of(value)

Finds the position of a value.

**Syntax**: `list.index_of(value, start)`

**Parameters**:
- `value`: Value to find
- `start` (num, optional): Starting position (default: 0)

**Returns**: (num) Index of first occurrence, or -1 if not found

**Examples**:
```graphoid
numbers = [10, 20, 30, 40, 50]
index = numbers.index_of(30)
print(index)  # 2

index = numbers.index_of(100)
print(index)  # -1

# Find multiple occurrences
items = [1, 2, 3, 2, 4]
first = items.index_of(2)        # 1
second = items.index_of(2, first + 1)  # 3
```

**See also**: `contains()`, `find()`

---

### reverse()

Reverses the list in place.

**Syntax**: `list.reverse()`

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
numbers.reverse()
print(numbers)  # [5, 4, 3, 2, 1]

# Palindrome check (non-destructive)
fn is_palindrome(list) {
    reversed = list[:]  # Copy
    reversed.reverse()
    return list == reversed
}
```

**See also**: `sort()`

---

### sort(comparator)

Sorts the list in place.

**Syntax**: `list.sort(comparator, reverse)`

**Parameters**:
- `comparator` (function, optional): Comparison function
- `reverse` (bool, optional): Sort descending (default: false)

**Returns**: none (modifies list in place)

**Examples**:
```graphoid
# Sort numbers ascending
numbers = [3, 1, 4, 1, 5, 9]
numbers.sort()
print(numbers)  # [1, 1, 3, 4, 5, 9]

# Sort descending
numbers.sort(none, true)
print(numbers)  # [9, 5, 4, 3, 1, 1]

# Sort strings
names = ["Charlie", "Alice", "Bob"]
names.sort()
print(names)  # ["Alice", "Bob", "Charlie"]

# Custom comparator
people = [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25},
    {"name": "Charlie", "age": 35}
]
people.sort((a, b) => a["age"] - b["age"])
# Sorted by age
```

**See also**: `reverse()`, `sorted()`

---

### sorted(comparator)

Returns a sorted copy (does not modify original).

**Syntax**: `list.sorted(comparator, reverse)`

**Parameters**:
- `comparator` (function, optional): Comparison function
- `reverse` (bool, optional): Sort descending (default: false)

**Returns**: (list) Sorted copy

**Examples**:
```graphoid
numbers = [3, 1, 4, 1, 5, 9]
sorted_nums = numbers.sorted()
print(sorted_nums)  # [1, 1, 3, 4, 5, 9]
print(numbers)      # [3, 1, 4, 1, 5, 9] (unchanged)
```

**See also**: `sort()`

---

## Functional Methods

### map(function)

Transforms each element using a function.

**Syntax**: `list.map(function)`

**Parameters**:
- `function`: Transformation function or named transformation

**Returns**: (list) Transformed list

**Examples**:
```graphoid
# Named transformations
numbers = [1, 2, 3, 4, 5]
doubled = numbers.map("double")
print(doubled)  # [2, 4, 6, 8, 10]

squared = numbers.map("square")
print(squared)  # [1, 4, 9, 16, 25]

# Lambda functions
numbers = [1, 2, 3]
tripled = numbers.map(x => x * 3)
print(tripled)  # [3, 6, 9]

# Complex transformations
users = [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25}
]
names = users.map(u => u["name"])
print(names)  # ["Alice", "Bob"]
```

**Named Transformations**: `"double"`, `"square"`, `"increment"`, `"decrement"`, `"negate"`, `"abs"`, `"to_string"`

**See also**: `filter()`, `reduce()`

---

### filter(predicate)

Selects elements that match a condition.

**Syntax**: `list.filter(predicate)`

**Parameters**:
- `predicate`: Test function or named predicate

**Returns**: (list) Filtered list

**Examples**:
```graphoid
# Named predicates
numbers = [-2, -1, 0, 1, 2, 3, 4, 5]
positives = numbers.filter("positive")
print(positives)  # [1, 2, 3, 4, 5]

evens = numbers.filter("even")
print(evens)  # [-2, 0, 2, 4]

# Lambda functions
numbers = [1, 2, 3, 4, 5, 6]
large = numbers.filter(x => x > 3)
print(large)  # [4, 5, 6]

# Filter objects
users = [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25},
    {"name": "Charlie", "age": 35}
]
adults = users.filter(u => u["age"] >= 30)
```

**Named Predicates**: `"positive"`, `"negative"`, `"even"`, `"odd"`, `"truthy"`, `"falsy"`

**See also**: `reject()`, `map()`

---

### reject(predicate)

Removes elements that match a condition (opposite of filter).

**Syntax**: `list.reject(predicate)`

**Parameters**:
- `predicate`: Test function or named predicate

**Returns**: (list) Filtered list

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5, 6]
odds = numbers.reject("even")
print(odds)  # [1, 3, 5]

# Same as filter with negated condition
odds = numbers.filter("odd")
```

**See also**: `filter()`

---

### reduce(function, initial)

Reduces list to a single value.

**Syntax**: `list.reduce(function, initial)`

**Parameters**:
- `function`: Accumulator function `(acc, value) => result`
- `initial`: Initial accumulator value

**Returns**: Final accumulated value

**Examples**:
```graphoid
# Sum
numbers = [1, 2, 3, 4, 5]
sum = numbers.reduce((acc, x) => acc + x, 0)
print(sum)  # 15

# Product
product = numbers.reduce((acc, x) => acc * x, 1)
print(product)  # 120

# Maximum
max = numbers.reduce((acc, x) => {
    if x > acc { return x }
    return acc
}, -infinity)

# String joining
words = ["hello", "world"]
sentence = words.reduce((acc, w) => acc + " " + w, "")
print(sentence.trim())  # "hello world"
```

**See also**: `map()`, `filter()`

---

### each(function)

Executes a function for each element.

**Syntax**: `list.each(function)`

**Parameters**:
- `function`: Function to execute (receives value, index)

**Returns**: none

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
numbers.each(x => print(x))
# Prints: 1 2 3 4 5

# With index
names = ["Alice", "Bob", "Charlie"]
names.each((name, i) => {
    print("${i}: ${name}")
})
# Prints:
# 0: Alice
# 1: Bob
# 2: Charlie
```

**See also**: `map()`, `for` loop

---

### find(predicate)

Finds the first element matching a condition.

**Syntax**: `list.find(predicate)`

**Parameters**:
- `predicate`: Test function

**Returns**: First matching element, or `none` if not found

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
result = numbers.find(x => x > 3)
print(result)  # 4

result = numbers.find(x => x > 10)
print(result)  # none

# Find object
users = [
    {"name": "Alice", "age": 30},
    {"name": "Bob", "age": 25}
]
user = users.find(u => u["name"] == "Alice")
print(user["age"])  # 30
```

**See also**: `filter()`, `index_of()`

---

### all(predicate)

Tests if all elements match a condition.

**Syntax**: `list.all(predicate)`

**Parameters**:
- `predicate`: Test function or named predicate

**Returns**: (bool) `true` if all match, `false` otherwise

**Examples**:
```graphoid
numbers = [2, 4, 6, 8]
result = numbers.all("even")
print(result)  # true

numbers = [2, 4, 5, 8]
result = numbers.all("even")
print(result)  # false

# With lambda
ages = [25, 30, 35, 40]
all_adults = ages.all(age => age >= 18)
print(all_adults)  # true
```

**See also**: `any()`, `filter()`

---

### any(predicate)

Tests if any element matches a condition.

**Syntax**: `list.any(predicate)`

**Parameters**:
- `predicate`: Test function or named predicate

**Returns**: (bool) `true` if any match, `false` otherwise

**Examples**:
```graphoid
numbers = [1, 3, 5, 7]
result = numbers.any("even")
print(result)  # false

numbers = [1, 3, 4, 7]
result = numbers.any("even")
print(result)  # true (4 is even)

# Check for valid items
has_error = results.any(r => r["status"] == "error")
```

**See also**: `all()`, `find()`

---

### sum()

Returns the sum of all elements (must be numbers).

**Syntax**: `list.sum()`

**Returns**: (num) Sum of all elements

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
total = numbers.sum()
print(total)  # 15

# Empty list
empty = []
print(empty.sum())  # 0
```

**See also**: `reduce()`, `average()`

---

### average()

Returns the average of all elements.

**Syntax**: `list.average()`

**Returns**: (num) Average value

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
avg = numbers.average()
print(avg)  # 3

scores = [95, 87, 92, 88]
avg_score = scores.average()
print(avg_score)  # 90.5
```

**Errors**: Empty list raises error

**See also**: `sum()`, `min()`, `max()`

---

### min()

Returns the minimum value.

**Syntax**: `list.min()`

**Returns**: Minimum element

**Examples**:
```graphoid
numbers = [3, 1, 4, 1, 5, 9]
minimum = numbers.min()
print(minimum)  # 1

# Works with strings
names = ["Charlie", "Alice", "Bob"]
first = names.min()
print(first)  # "Alice"
```

**Errors**: Empty list raises error

**See also**: `max()`, `sort()`

---

### max()

Returns the maximum value.

**Syntax**: `list.max()`

**Returns**: Maximum element

**Examples**:
```graphoid
numbers = [3, 1, 4, 1, 5, 9]
maximum = numbers.max()
print(maximum)  # 9

scores = [95, 87, 92, 88, 91]
high_score = scores.max()
```

**Errors**: Empty list raises error

**See also**: `min()`, `sort()`

---

### unique()

Returns a list with duplicate elements removed.

**Syntax**: `list.unique()`

**Returns**: (list) List with unique elements (preserves order)

**Examples**:
```graphoid
numbers = [1, 2, 2, 3, 1, 4, 3, 5]
unique = numbers.unique()
print(unique)  # [1, 2, 3, 4, 5]

# Remove duplicate names
names = ["Alice", "Bob", "Alice", "Charlie"]
unique_names = names.unique()
print(unique_names)  # ["Alice", "Bob", "Charlie"]
```

**See also**: `count()`

---

### count(value)

Counts occurrences of a value.

**Syntax**: `list.count(value)`

**Parameters**:
- `value` (optional): Value to count. If omitted, returns length.

**Returns**: (num) Number of occurrences

**Examples**:
```graphoid
numbers = [1, 2, 2, 3, 1, 4, 1]
count = numbers.count(1)
print(count)  # 3

count = numbers.count(5)
print(count)  # 0

# Count matching predicate
votes = ["yes", "no", "yes", "yes", "no"]
yes_count = votes.count("yes")
print(yes_count)  # 3
```

**See also**: `unique()`, `filter()`

---

### flatten(depth)

Flattens nested lists.

**Syntax**: `list.flatten(depth)`

**Parameters**:
- `depth` (num, optional): Depth to flatten (default: 1)

**Returns**: (list) Flattened list

**Examples**:
```graphoid
# Flatten one level
nested = [[1, 2], [3, 4], [5, 6]]
flat = nested.flatten()
print(flat)  # [1, 2, 3, 4, 5, 6]

# Deeply nested
deep = [1, [2, [3, [4]]]]
flat = deep.flatten()
print(flat)  # [1, 2, [3, [4]]]

flat = deep.flatten(infinity)
print(flat)  # [1, 2, 3, 4]
```

**See also**: `map()`

---

### zip(other_list)

Combines two lists into pairs.

**Syntax**: `list1.zip(list2)`

**Parameters**:
- `other_list` (list): List to zip with

**Returns**: (list) List of pairs `[value1, value2]`

**Examples**:
```graphoid
names = ["Alice", "Bob", "Charlie"]
ages = [30, 25, 35]
pairs = names.zip(ages)
print(pairs)
# [["Alice", 30], ["Bob", 25], ["Charlie", 35]]

# Create hash from pairs
hash = {}
for pair in pairs {
    hash[pair[0]] = pair[1]
}
# {"Alice": 30, "Bob": 25, "Charlie": 35}

# Different lengths: stops at shorter
a = [1, 2, 3, 4]
b = [10, 20]
result = a.zip(b)
print(result)  # [[1, 10], [2, 20]]
```

**See also**: `map()`

---

### chunk(size)

Splits list into chunks of a given size.

**Syntax**: `list.chunk(size)`

**Parameters**:
- `size` (num): Chunk size

**Returns**: (list) List of chunks

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5, 6, 7, 8]
chunks = numbers.chunk(3)
print(chunks)
# [[1, 2, 3], [4, 5, 6], [7, 8]]

# Process in batches
items.chunk(100).each(batch => {
    process_batch(batch)
})
```

**See also**: `slice()`

---

### take(n)

Returns the first n elements.

**Syntax**: `list.take(n)`

**Parameters**:
- `n` (num): Number of elements to take

**Returns**: (list) First n elements

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
first_three = numbers.take(3)
print(first_three)  # [1, 2, 3]

# Take more than available
result = numbers.take(10)
print(result)  # [1, 2, 3, 4, 5]
```

**See also**: `drop()`, `[:]`

---

### drop(n)

Removes the first n elements and returns the rest.

**Syntax**: `list.drop(n)`

**Parameters**:
- `n` (num): Number of elements to drop

**Returns**: (list) Remaining elements

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
rest = numbers.drop(2)
print(rest)  # [3, 4, 5]

# Same as slice
rest = numbers[2:]  # [3, 4, 5]
```

**See also**: `take()`, `[:]`

---

## Static Methods

### list.generate(start, end) - Range Mode

Generates a list of numbers from start to end.

**Syntax**: `list.generate(start, end, step)`

**Parameters**:
- `start` (num): Starting value (inclusive)
- `end` (num): Ending value (inclusive)
- `step` (num, optional): Increment (default: 1)

**Returns**: (list) Generated list

**Examples**:
```graphoid
# Basic range
numbers = list.generate(1, 5)
print(numbers)  # [1, 2, 3, 4, 5]

# With step
evens = list.generate(0, 10, 2)
print(evens)  # [0, 2, 4, 6, 8, 10]

# Countdown
countdown = list.generate(10, 1, -1)
print(countdown)  # [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]

# Decimal steps
fractions = list.generate(0, 1, 0.25)
print(fractions)  # [0, 0.25, 0.5, 0.75, 1.0]
```

**See also**: `range()`, `string.generate()`

---

### list.generate(value, count) - Repetition Mode

Generates a list by repeating a value.

**Syntax**: `list.generate(value, count)`

**Parameters**:
- `value`: Value to repeat
- `count` (num): Number of repetitions

**Returns**: (list) Generated list

**Examples**:
```graphoid
# Create zeros
zeros = list.generate(0, 5)
print(zeros)  # [0, 0, 0, 0, 0]

# Same as repetition operator
zeros = [0] * 5

# Initialize with value
defaults = list.generate(none, 10)
```

**See also**: `*` operator

---

## Graph Operations

Lists are internally graphs, so they support graph operations:

### neighbors(index)

Returns adjacent elements in the graph representation.

**Syntax**: `list.neighbors(index)`

**Parameters**:
- `index` (num): Element position

**Returns**: (list) Adjacent elements

**Examples**:
```graphoid
numbers = [1, 2, 3, 4, 5]
adjacent = numbers.neighbors(2)  # [2, 4] (neighbors of 3)
```

---

### add_edge(from, to, label)

Adds a graph edge between elements.

**Syntax**: `list.add_edge(from_index, to_index, label)`

**See also**: Graph operations chapter

---

## Type Checking

### is_list()

Tests if a value is a list.

**Syntax**: `value.is_list()`

**Returns**: (bool) `true` if list, `false` otherwise

**Examples**:
```graphoid
result = [1, 2, 3].is_list()  # true
result = "hello".is_list()    # false

# Type validation
if not value.is_list() {
    print("Expected list")
}
```

**See also**: `is_string()`, `is_number()`

---

### is_empty()

Tests if list is empty.

**Syntax**: `list.is_empty()`

**Returns**: (bool) `true` if empty, `false` otherwise

**Examples**:
```graphoid
empty = []
result = empty.is_empty()  # true

numbers = [1, 2, 3]
result = numbers.is_empty()  # false

# Guard clause
if list.is_empty() {
    print("No items to process")
    return
}
```

**See also**: `length()`

---

## Common Patterns

### Sum of filtered values

```graphoid
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
even_sum = numbers.filter("even").sum()
print(even_sum)  # 30
```

### Chaining operations

```graphoid
result = numbers
    .filter("positive")
    .map("square")
    .filter(x => x < 100)
    .sum()
```

### Creating a frequency map

```graphoid
items = ["a", "b", "a", "c", "a", "b"]
freq = {}
for item in items {
    if freq.has_key(item) {
        freq[item] = freq[item] + 1
    } else {
        freq[item] = 1
    }
}
# {"a": 3, "b": 2, "c": 1}
```

### Matrix transpose

```graphoid
fn transpose(matrix) {
    if matrix.is_empty() { return [] }
    cols = matrix[0].length()
    result = list.generate([], cols)
    for row in matrix {
        for i in range(cols) {
            result[i].append(row[i])
        }
    }
    return result
}
```

---

## See Also

- [hash](hash.md) - Hash/dictionary type
- [tree](tree.md) - Tree data structure
- [graph](graph.md) - Graph data structure
- [operators](../operators.md) - Complete operator reference
- [User Guide: Collections](../../user-guide/05-collections.md) - Collections tutorial
