# Chapter 5: Collections

Collections are the heart of Graphoid. In this chapter, we'll explore lists, hashes, trees, and graphs - all of which are graph-based under the hood!

## Lists

Lists are ordered collections of elements. In Graphoid, lists are internally represented as linked graphs.

### Creating Lists

```graphoid
# Empty list
empty = []

# List with elements
numbers = [1, 2, 3, 4, 5]
names = ["Alice", "Bob", "Charlie"]
mixed = [1, "hello", true, none]  # Lists can hold any type
```

### Accessing Elements

```graphoid
fruits = ["apple", "banana", "orange"]

first = fruits[0]    # "apple"
second = fruits[1]   # "banana"
last = fruits[2]     # "orange"

# Negative indexing
last = fruits[-1]    # "orange"
second_last = fruits[-2]  # "banana"
```

### List Methods

#### Adding Elements

```graphoid
numbers = [1, 2, 3]

# Append to end
numbers.append(4)
print(numbers)  # [1, 2, 3, 4]

# Insert at position
numbers.insert(0, 0)  # Insert 0 at index 0
print(numbers)  # [0, 1, 2, 3, 4]

# Extend with another list
numbers.extend([5, 6, 7])
print(numbers)  # [0, 1, 2, 3, 4, 5, 6, 7]
```

#### Removing Elements

```graphoid
numbers = [1, 2, 3, 4, 5, 3]

# Remove first occurrence of value
numbers.remove(3)
print(numbers)  # [1, 2, 4, 5, 3]

# Remove at index
numbers.remove_at(0)
print(numbers)  # [2, 4, 5, 3]

# Pop (remove and return last element)
last = numbers.pop()
print(last)     # 3
print(numbers)  # [2, 4, 5]

# Clear all elements
numbers.clear()
print(numbers)  # []
```

#### Querying Lists

```graphoid
numbers = [1, 2, 3, 4, 5]

# Length
len = numbers.length()  # 5

# Contains
has_three = numbers.contains(3)  # true
has_ten = numbers.contains(10)   # false

# Index of element
index = numbers.index_of(3)   # 2
index = numbers.index_of(10)  # -1 (not found)

# Count occurrences
counts = [1, 2, 2, 3, 2, 4]
twos = counts.count(2)  # 3
```

#### Slicing

```graphoid
numbers = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

# Slice from index 2 to 5 (exclusive)
slice = numbers.slice(2, 5)  # [2, 3, 4]

# Slice from index 5 to end
slice = numbers.slice(5)     # [5, 6, 7, 8, 9]

# Slice with step
evens = numbers.slice(0, 10, 2)  # [0, 2, 4, 6, 8]
```

### List Transformations

Graphoid provides powerful functional transformations:

#### map - Transform Each Element

```graphoid
numbers = [1, 2, 3, 4, 5]

# Using lambda
squared = numbers.map(x => x * x)
print(squared)  # [1, 4, 9, 16, 25]

# Using named transformation
doubled = numbers.map("double")
print(doubled)  # [2, 4, 6, 8, 10]
```

**Named Transformations**:
- `"double"` - Multiply by 2
- `"square"` - Square the value
- `"negate"` - Negate the value
- `"abs"` - Absolute value
- `"to_string"` - Convert to string

#### filter - Keep Matching Elements

```graphoid
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Using lambda
evens = numbers.filter(x => x % 2 == 0)
print(evens)  # [2, 4, 6, 8, 10]

# Using named predicate
positives = numbers.filter("positive")
print(positives)  # [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
```

**Named Predicates**:
- `"positive"` - Greater than 0
- `"negative"` - Less than 0
- `"even"` - Even number
- `"odd"` - Odd number
- `"zero"` - Equals 0
- `"nonzero"` - Not 0

#### reject - Remove Matching Elements

```graphoid
numbers = [1, 2, 3, 4, 5]

odds = numbers.reject("even")
print(odds)  # [1, 3, 5]
```

#### reduce - Combine All Elements

```graphoid
numbers = [1, 2, 3, 4, 5]

# Sum
sum = numbers.reduce((acc, x) => acc + x, 0)
print(sum)  # 15

# Product
product = numbers.reduce((acc, x) => acc * x, 1)
print(product)  # 120

# Max
max = numbers.reduce((acc, x) => if x > acc { x } else { acc }, numbers[0])
print(max)  # 5
```

#### Chaining Transformations

```graphoid
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

result = numbers
    .filter("even")       # [2, 4, 6, 8, 10]
    .map("square")        # [4, 16, 36, 64, 100]
    .filter(x => x < 50)  # [4, 16, 36]

print(result)  # [4, 16, 36]
```

### Sorting

```graphoid
numbers = [5, 2, 8, 1, 9, 3]

# Sort ascending (modifies in place)
numbers.sort()
print(numbers)  # [1, 2, 3, 5, 8, 9]

# Sort descending
numbers.sort(:desc)
print(numbers)  # [9, 8, 5, 3, 2, 1]

# Custom comparator
words = ["apple", "pie", "zoo", "cat"]
words.sort((a, b) => a.length() - b.length())
print(words)  # ["pie", "cat", "zoo", "apple"]
```

### Other Useful Methods

```graphoid
numbers = [1, 2, 3, 4, 5]

# Reverse
numbers.reverse()
print(numbers)  # [5, 4, 3, 2, 1]

# Join into string
result = numbers.join(", ")
print(result)  # "5, 4, 3, 2, 1"

# Find first matching
evens = [1, 3, 5, 6, 7]
first_even = evens.find(x => x % 2 == 0)
print(first_even)  # 6

# Check if all match
all_positive = numbers.all(x => x > 0)
print(all_positive)  # true

# Check if any match
has_negative = numbers.any(x => x < 0)
print(has_negative)  # false
```

## Hashes

Hashes (also called dictionaries or maps) store key-value pairs. Keys are always strings.

### Creating Hashes

```graphoid
# Empty hash
empty = {}

# Hash with values
person = {
    "name": "Alice",
    "age": 30,
    "city": "Boston"
}

# Nested hashes
config = {
    "database": {
        "host": "localhost",
        "port": 5432
    },
    "cache": {
        "enabled": true
    }
}
```

### Accessing Values

```graphoid
person = {"name": "Alice", "age": 30}

# Access by key
name = person["name"]  # "Alice"
age = person["age"]    # 30

# Non-existent key returns none
email = person["email"]  # none
```

### Modifying Hashes

```graphoid
person = {"name": "Alice"}

# Add/update key
person["age"] = 30
person["name"] = "Alice Smith"

# Delete key
person.delete("age")
print(person)  # {"name": "Alice Smith"}
```

### Hash Methods

```graphoid
person = {"name": "Alice", "age": 30, "city": "Boston"}

# Get keys
keys = person.keys()
print(keys)  # ["name", "age", "city"]

# Get values
values = person.values()
print(values)  # ["Alice", 30, "Boston"]

# Get entries (key-value pairs)
entries = person.entries()
# [["name", "Alice"], ["age", 30], ["city", "Boston"]]

# Check if key exists
has_name = person.has_key("name")  # true
has_email = person.has_key("email")  # false

# Number of entries
size = person.size()  # 3

# Clear all entries
person.clear()
```

### Iterating Over Hashes

```graphoid
person = {"name": "Alice", "age": 30}

# Iterate over keys
for key in person.keys() {
    print(key + ": " + person[key].to_string())
}

# Iterate over entries
for entry in person.entries() {
    key = entry[0]
    value = entry[1]
    print(key + " = " + value.to_string())
}
```

### Hash Transformations

```graphoid
prices = {"apple": 1.50, "banana": 0.75, "orange": 2.00}

# Map over values
discounted = prices.map_values(x => x * 0.9)
print(discounted)
# {"apple": 1.35, "banana": 0.675, "orange": 1.80}

# Filter entries
expensive = prices.filter_entries((k, v) => v > 1.0)
print(expensive)
# {"apple": 1.50, "orange": 2.00}
```

## Trees

Trees are hierarchical data structures. In Graphoid, trees are graphs with tree constraints.

### Creating Trees

```graphoid
# Binary search tree
bst = tree{}

# Tree with initial values
numbers = tree{10, 5, 15, 3, 7}
```

### Tree Operations

```graphoid
tree = tree{}

# Insert values
tree.insert(10)
tree.insert(5)
tree.insert(15)
tree.insert(3)
tree.insert(7)

# Search
found = tree.contains(7)  # true
not_found = tree.contains(20)  # false

# Remove
tree.remove(5)

# Size
size = tree.size()
```

### Tree Traversals

```graphoid
tree = tree{10, 5, 15, 3, 7, 12, 20}

# In-order traversal (sorted)
in_order = tree.traverse("in_order")
print(in_order)  # [3, 5, 7, 10, 12, 15, 20]

# Pre-order traversal
pre_order = tree.traverse("pre_order")
print(pre_order)  # [10, 5, 3, 7, 15, 12, 20]

# Post-order traversal
post_order = tree.traverse("post_order")
print(post_order)  # [3, 7, 5, 12, 20, 15, 10]

# Level-order (breadth-first)
level_order = tree.traverse("level_order")
print(level_order)  # [10, 5, 15, 3, 7, 12, 20]
```

### Tree Properties

```graphoid
tree = tree{10, 5, 15, 3, 7, 12, 20}

# Height
height = tree.height()  # 3

# Is balanced
balanced = tree.is_balanced()

# Min and max
min = tree.min()  # 3
max = tree.max()  # 20
```

## Graphs

Graphs are the fundamental data structure in Graphoid. Everything is a graph!

### Creating Graphs

```graphoid
# Empty directed graph
g = graph{}

# Specify graph type
g = graph { type: :directed }
g = graph { type: :undirected }
g = graph { type: :dag }  # Directed Acyclic Graph
```

### Adding Nodes

```graphoid
g = graph{}

# Add nodes with values
g.add_node("A", 100)
g.add_node("B", 200)
g.add_node("C", 300)
```

### Adding Edges

```graphoid
g = graph{}
g.add_node("A", 1)
g.add_node("B", 2)
g.add_node("C", 3)

# Add edge from A to B
g.add_edge("A", "B")

# Add labeled edge
g.add_edge("A", "C", "depends_on")

# Add weighted edge
g.add_edge("B", "C", "road", 10.5)  # Weight = 10.5
```

### Graph Queries

```graphoid
# Get all nodes
nodes = g.nodes()

# Get all edges
edges = g.edges()

# Get neighbors of a node
neighbors = g.neighbors("A")

# Check if edge exists
has_edge = g.has_edge("A", "B")

# Get edge weight
weight = g.get_weight("B", "C")
```

### Graph Traversal

```graphoid
# Depth-first search
dfs = g.traverse("A", :dfs)
print(dfs)  # Nodes in DFS order

# Breadth-first search
bfs = g.traverse("A", :bfs)
print(bfs)  # Nodes in BFS order
```

We'll explore more graph algorithms in Chapter 6!

## Type Constraints on Collections

You can optionally specify element types for collections:

```graphoid
# List of numbers only
list<num> scores = [95, 87, 92]
# scores.append("hello")  # Error: Type mismatch

# Hash with number values
hash<num> settings = {"volume": 80, "brightness": 60}

# Tree of strings
tree<string> words = tree{"apple", "banana", "orange"}
```

**Note**: Type constraints are runtime-checked, not compile-time.

## Collection Generators

### List Generators

```graphoid
# Generate range
numbers = list.generate(1, 10)  # [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Generate with step
evens = list.generate(0, 20, 2)  # [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]

# Repeat value
zeros = list.generate(0, 5)  # [0, 0, 0, 0, 0] (if only two args)
```

### String Generators

```graphoid
# Repeat character
padding = string.generate(" ", 10)  # "          "

# Character sequence
alphabet = string.generate("a", "z")  # "abcdefghijklmnopqrstuvwxyz"
```

## Immutability

By default, collections are mutable. To make a collection immutable:

```graphoid
numbers = [1, 2, 3]
numbers.freeze()

# numbers.append(4)  # Error: Cannot modify frozen collection

# Check if frozen
is_frozen = numbers.is_frozen()  # true
```

## Collection Best Practices

### Choose the Right Collection

- **List**: Ordered sequence, frequent access by index
- **Hash**: Key-value lookup, unique keys
- **Tree**: Sorted data, range queries
- **Graph**: Relationships, networks, dependencies

### Use Named Transformations

```graphoid
# ✅ GOOD: Clear and concise
doubled = numbers.map("double")

# ❌ Less clear (but more flexible)
doubled = numbers.map(x => x * 2)
```

### Chain Transformations

```graphoid
result = data
    .filter("positive")
    .map("square")
    .reduce((a, b) => a + b, 0)
```

### Avoid Modifying During Iteration

```graphoid
# ❌ BAD
for item in list {
    if condition {
        list.remove(item)  # Don't modify while iterating!
    }
}

# ✅ GOOD
filtered = list.filter(item => not condition)
```

## Summary

In this chapter, you learned:

- ✅ **Lists**: Creation, access, methods, transformations
- ✅ **Hashes**: Key-value storage and operations
- ✅ **Trees**: Hierarchical data and traversals
- ✅ **Graphs**: Nodes, edges, and basic operations
- ✅ **Transformations**: map, filter, reduce, chaining
- ✅ **Named operations**: "double", "square", "even", "positive"
- ✅ **Type constraints**: Optional runtime type checking
- ✅ **Best practices**: Choosing collections, chaining, immutability

---

## Quick Reference

```graphoid
# Lists
list = [1, 2, 3]
list.append(4)
list.map(x => x * 2)
list.filter(x => x > 0)
list.reduce((a, b) => a + b, 0)

# Hashes
hash = {"key": "value"}
hash["key"]
hash.keys()
hash.values()

# Trees
tree = tree{10, 5, 15}
tree.insert(7)
tree.traverse("in_order")

# Graphs
g = graph{}
g.add_node("A", 100)
g.add_edge("A", "B", "label", weight)
g.neighbors("A")
```

---

## Exercises

1. **Unique Elements**: Write a function that returns unique elements from a list

2. **Merge Hashes**: Write a function that merges two hashes (second overrides first)

3. **Flatten List**: Write a recursive function to flatten a nested list

4. **Word Frequency**: Count word frequency in a string using a hash

5. **Tree Height**: Implement a function to calculate tree height recursively

6. **List Intersection**: Find common elements between two lists

**Solutions** are available in `examples/05-collections/exercises.gr`

---

[← Previous: Functions](04-functions.md) | [Next: Graph Operations →](06-graph-operations.md)
