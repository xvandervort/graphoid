# List Methods

List is a built-in collection type in Glang representing ordered sequences of values. Lists can be homogeneous (type-constrained) or heterogeneous (mixed types).

## Type Information

### type()
Returns the type of the value.
```glang
items = [1, 2, 3]
items.type()  # Returns "list"
```

### methods()
Returns a list of all available methods for lists.
```glang
items = [1, 2, 3]
items.methods()  # Returns ["type", "methods", "can", "inspect", "size", "append", "prepend", ...]
```

### can(method_name)
Checks if a method is available on the list.
```glang
items = [1, 2, 3]
items.can("append")  # Returns true
items.can("invalid")  # Returns false
```

### inspect()
Returns detailed structural information about the list including graph structure, rules, and connections.
```glang
items = [1, 2, 3]
items.names(['red', 'orange', 'yellow'])
items.inspect()
# Returns:
# Graph Structure:
# ========================================
# Type: list
# Nodes: 3
# Edges: 2
# Active Rules: no_list_cycles, same_structure_only
#
# Node Connections:
#   red (1):
#     → orange (2) (0)
#   orange (2):
#     → yellow (3) (1)
#   yellow (3): (no outgoing edges)
```

### visualize()
Shows the shape and flow of the list structure. Names are displayed when available.
```glang
items = [1, 2, 3, 4, 5]
items.visualize()  # Returns "[1 → 2 → 3 → 4 → 5]"

# With names
items.names(['first', 'second', 'third', 'fourth', 'fifth'])
items.visualize()  # Returns "[first → second → third → fourth → fifth]"

# Large lists are abbreviated
big_list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
big_list.visualize()  # Returns "[1 → 2 → 3 → ... → 10 → 11 → 12] (12 elements)"
```

### view()
Shows a clean display of names and values together.
```glang
items = [1, 2, 3]
items.view()  # Returns "[1, 2, 3]"

# With names
items.names(['red', 'orange', 'yellow'])
items.view()  # Returns ["red": 1, "orange": 2, "yellow": 3]
```

## Size and State

### size()
Returns the number of elements in the list.
```glang
items = [1, 2, 3, 4, 5]
items.size()  # Returns 5
```

### empty()
Checks if the list is empty.
```glang
[].empty()  # Returns true
[1, 2, 3].empty()  # Returns false
```

### first()
Returns the first element in the list. Returns `none` for empty lists. This is a universal graph method available on all graph types.
```glang
items = [10, 20, 30, 40]
items.first()  # Returns 10

# Single element
[42].first()   # Returns 42

# Empty list
[].first()     # Returns none

# Works with any element type
mixed = [1, "hello", true]
mixed.first()  # Returns 1
```

### last()
Returns the last element in the list. Returns `none` for empty lists. This is a universal graph method available on all graph types.
```glang
items = [10, 20, 30, 40]
items.last()   # Returns 40

# Single element
[42].last()    # Returns 42

# Empty list
[].last()      # Returns none

# Works with any element type
mixed = [1, "hello", true]
mixed.last()   # Returns true
```

## Type Constraints

### constraint()
Returns the type constraint of the list, if any.
```glang
list<num> scores = [95, 87, 92]
scores.constraint()  # Returns "num"

mixed = [1, "hello", true]
mixed.constraint()  # Returns none
```

### validate_constraint()
Validates that all elements match the constraint.
```glang
list<num> scores = [95, 87, 92]
scores.validate_constraint()  # Returns true
```

### types()
Returns unique types present in the list. Useful for checking homogeneity and type composition.
```glang
# Homogeneous list
numbers = [1, 2, 3]
numbers.types()  # Returns ["num"]

# Mixed list
mixed = [1, "hello", true, 2]
mixed.types()  # Returns ["string", "num", "bool"]

# Empty list
empty = []
empty.types()  # Returns []

# Check if list is homogeneous
is_homogeneous = mixed.types().size() == 1  # false
```

### coerce_to_constraint(type)
Attempts to convert all elements to the specified type.
```glang
strings = ["1", "2", "3"]
strings.coerce_to_constraint("num")  # Returns [1, 2, 3]
```

## Adding Elements

### append(element)
Adds an element to the end of the list.
```glang
items = [1, 2, 3]
items.append(4)  # items becomes [1, 2, 3, 4]
```

### prepend(element)
Adds an element to the beginning of the list.
```glang
items = [2, 3, 4]
items.prepend(1)  # items becomes [1, 2, 3, 4]
```

### insert(index, element)
Inserts an element at the specified index.
```glang
items = [1, 2, 4, 5]
items.insert(2, 3)  # items becomes [1, 2, 3, 4, 5]
```

## Removing Elements

### remove(element)
Removes the first occurrence of an element.
```glang
items = [1, 2, 3, 2, 4]
items.remove(2)  # items becomes [1, 3, 2, 4]
```

### pop()
Removes and returns the last element.
```glang
items = [1, 2, 3, 4]
last = items.pop()  # last is 4, items becomes [1, 2, 3]
```

### pop(index)
Removes and returns the element at the specified index.
```glang
items = [1, 2, 3, 4]
element = items.pop(1)  # element is 2, items becomes [1, 3, 4]
```

### clear()
Removes all elements from the list.
```glang
items = [1, 2, 3, 4]
items.clear()  # items becomes []
```

## Searching and Counting

### index_of(element)
Returns the index of the first occurrence of an element.
```glang
items = [10, 20, 30, 20, 40]
items.index_of(20)  # Returns 1
items.index_of(50)  # Returns -1 (not found)
```

### count(element)
Counts occurrences of an element.
```glang
items = [1, 2, 3, 2, 2, 4]
items.count(2)  # Returns 3
```

### count_values(value)
Counts occurrences of a value (alias for count).
```glang
items = ["a", "b", "a", "c", "a"]
items.count_values("a")  # Returns 3
```

## Element Naming

Lists support semantic naming of elements, similar to R's named vectors. Names are displayed in visualization methods and provide meaningful labels for list elements.

### names()
Gets the current names assigned to list elements.
```glang
items = [1, 2, 3]
items.names()  # Returns [none, none, none] (no names set)

# After setting names
items.names(['first', 'second', 'third'])
items.names()  # Returns ['first', 'second', 'third']
```

### names(list)
Sets names for list elements. The names list must be the same length as the list.
```glang
items = [10, 20, 30]
items.names(['min', 'avg', 'max'])  # Sets names

# Names can be mixed with nil for unnamed elements
items.names(['min', none, 'max'])  # Middle element has no name
```

### has_names()
Checks if any elements in the list have names assigned.
```glang
items = [1, 2, 3]
items.has_names()  # Returns false

items.names(['a', 'b', 'c'])
items.has_names()  # Returns true

# Even partial names return true
items.names(['a', none, none])
items.has_names()  # Returns true
```

### name(index)
Gets the name of a specific element by index.
```glang
items = [10, 20, 30]
items.names(['min', 'avg', 'max'])

items.name(0)  # Returns 'min'
items.name(1)  # Returns 'avg'
items.name(2)  # Returns 'max'
```

### set_name(index, name)
Sets the name for a single element at the specified index.
```glang
items = [10, 20, 30]
items.set_name(0, 'minimum')
items.set_name(2, 'maximum')

items.names()  # Returns ['minimum', none, 'maximum']
```

## Ordering and Sorting

### reverse()
Reverses the list in place.
```glang
items = [1, 2, 3, 4]
items.reverse()  # items becomes [4, 3, 2, 1]
```

### sort()
Sorts the list in ascending order.
```glang
items = [3, 1, 4, 1, 5, 9]
items.sort()  # items becomes [1, 1, 3, 4, 5, 9]
```

## Mathematical Operations (for numeric lists)

### min()
Returns the minimum value in the list.
```glang
numbers = [3, 1, 4, 1, 5, 9]
numbers.min()  # Returns 1
```

### max()
Returns the maximum value in the list.
```glang
numbers = [3, 1, 4, 1, 5, 9]
numbers.max()  # Returns 9
```

### sum()
Returns the sum of all elements.
```glang
numbers = [1, 2, 3, 4, 5]
numbers.sum()  # Returns 15
```

## Functional Programming

### map(transformation)
Applies a transformation to each element.
```glang
numbers = [1, 2, 3, 4]
numbers.map("double")  # Returns [2, 4, 6, 8]
numbers.map("square")  # Returns [1, 4, 9, 16]

names = ["alice", "bob"]
names.map("upper")  # Returns ["ALICE", "BOB"]
```

Available transformations:
- Numeric: `double`, `square`, `negate`, `increment`, `decrement`
- String: `upper`, `lower`, `trim`, `reverse`
- Type conversion: `to_string`, `to_num`, `to_bool`

### filter(predicate)
Filters elements based on a predicate.
```glang
numbers = [1, 2, 3, 4, 5, 6]
numbers.filter("even")  # Returns [2, 4, 6]
numbers.filter("positive")  # Returns [1, 2, 3, 4, 5, 6]
```

Available predicates:
- Numeric: `positive`, `negative`, `zero`, `even`, `odd`
- String/Collection: `empty`, `non_empty`, `uppercase`, `lowercase`
- Type checks: `is_string`, `is_number`, `is_bool`, `is_list`
- General: `truthy`, `falsy`

### select(predicate)
Alias for filter. Selects elements matching the predicate.
```glang
numbers = [1, 2, 3, 4, 5]
numbers.select("odd")  # Returns [1, 3, 5]
```

### reject(predicate)
Returns elements that don't match the predicate.
```glang
numbers = [1, 2, 3, 4, 5, 6]
numbers.reject("even")  # Returns [1, 3, 5]
```

### each(action)
Applies an action to each element (for side effects).
```glang
names = ["alice", "bob", "charlie"]
names.each("print")  # Prints each name
```

## Type Conversion

### to_string()
Converts the list to a string representation.
```glang
items = [1, 2, 3]
items.to_string()  # Returns "[1, 2, 3]"
```

### to_bool()
Converts the list to a boolean. Empty lists are false, non-empty are true.
```glang
[1, 2, 3].to_bool()  # Returns true
[].to_bool()  # Returns false
```

## Immutability Methods

### freeze()
Makes the list and all its elements immutable. Returns self for chaining.
```glang
items = [1, 2, 3]
items.freeze()
items.is_frozen()  # Returns true
# items.append(4)  # Would throw error
```

### is_frozen()
Checks if the list is frozen (immutable).
```glang
items = [1, 2, 3]
items.freeze()
items.is_frozen()  # Returns true
```

### contains_frozen()
Checks if the list contains any frozen elements.
```glang
items = [1, 2, 3]
frozen_item = "hello"
frozen_item.freeze()
items.append(frozen_item)  # Error: can't mix frozen/unfrozen
```

### can_accept(value)
Checks if a value can be added to the list (respects frozen state).
```glang
items = [1, 2, 3]
frozen_value = 4
frozen_value.freeze()
items.can_accept(frozen_value)  # Returns false (can't mix)
```

## List Arithmetic

Lists support arithmetic operations with scalars and other lists:

### Scalar Addition
```glang
numbers = [1, 2, 3]
result = numbers + 5  # Returns [6, 7, 8]
```

### Scalar Multiplication
```glang
numbers = [1, 2, 3]
result = numbers * 2  # Returns [2, 4, 6]
```

### List Concatenation
```glang
list1 = [1, 2, 3]
list2 = [4, 5, 6]
result = list1 + list2  # Returns [1, 2, 3, 4, 5, 6]
```

### Element-wise Operations (dot operators)
```glang
list1 = [1, 2, 3]
list2 = [4, 5, 6]
result = list1 +. list2  # Returns [5, 7, 9] (element-wise addition)
result = list1 *. list2  # Returns [4, 10, 18] (element-wise multiplication)
```

## Indexing and Slicing

### Index Access
```glang
items = [10, 20, 30, 40, 50]
items[0]  # Returns 10
items[2]  # Returns 30
items[-1]  # Returns 50 (last element)
```

### Index Assignment
```glang
items = [10, 20, 30]
items[1] = 25  # items becomes [10, 25, 30]
```

### Slice Access
```glang
items = [1, 2, 3, 4, 5]
items[1:3]  # Returns [2, 3]
items[2:]  # Returns [3, 4, 5]
items[:3]  # Returns [1, 2, 3]
items[::2]  # Returns [1, 3, 5] (every 2nd element)
```

## Examples

### Processing Numeric Data
```glang
scores = [95, 87, 92, 76, 88, 91]

# Find statistics
average = scores.sum() / scores.size()
highest = scores.max()
lowest = scores.min()

# Filter and transform
high_scores = scores.filter(s => s > 90)
curved_scores = scores.map(s => s + 5)
```

### Type-Constrained Lists
```glang
# Enforce numeric elements only
list<num> temperatures = [72.5, 68.2, 75.1]
temperatures.append(70.0)  # OK
# temperatures.append("hot")  # Would throw error

# Validate constraint
if temperatures.validate_constraint() {
    print("All temperatures are numbers")
}
```

### Functional Pipeline
```glang
# Complex data processing
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
result = data.filter("even")
             .map("square")
             .filter(x => x > 10)
             .sort()
print(result)  # [16, 36, 64, 100]
```

### List Building
```glang
# Build a list dynamically
results = []
for i in [1, 2, 3, 4, 5] {
    if i % 2 == 0 {
        results.append(i * i)
    }
}
print(results)  # [4, 16]
```

## List Generators

List generators provide convenient ways to create lists with specific patterns of values.

### generate(start, end, step)
Creates a list of numbers from `start` to `end` (inclusive) with the given `step`.
```glang
nums = []
sequence = nums.generate(1, 10, 2)  # [1, 3, 5, 7, 9]
decimals = nums.generate(0, 1, 0.25)  # [0, 0.25, 0.5, 0.75, 1]
countdown = nums.generate(10, 0, -2)  # [10, 8, 6, 4, 2, 0]
```

### upto(end)
Creates a list of integers from 0 to `end` (inclusive).
```glang
nums = []
count = nums.upto(5)  # [0, 1, 2, 3, 4, 5]
single = nums.upto(0)  # [0]
```

### from_function(count, func)
Creates a list by calling a function with indices from 0 to count-1.
```glang
items = []

# With lambda
double = x => x * 2
evens = items.from_function(5, double)  # [0, 2, 4, 6, 8]

# With function
func square(x) { return x * x }
squares = items.from_function(4, square)  # [0, 1, 4, 9]

# Generating strings
make_label = x => "Item " + x.to_string()
labels = items.from_function(3, make_label)  # ["Item 0", "Item 1", "Item 2"]
```

### Generator Chaining
```glang
# Generators work seamlessly with functional operations
nums = []
result = nums.generate(1, 20, 1)
    .filter("even")      # Keep only even numbers
    .map("double")       # Double each value
print(result)  # [4, 8, 12, 16, 20, 24, 28, 32, 36, 40]

# Generate test data
random_score = x => 70 + (x * 3) % 30
scores = [].from_function(10, random_score)
```

#### Immutable Data Structures
```glang
# Create immutable configuration
config_values = [8080, "localhost", true]
config_values.freeze()

# Safe to share - cannot be modified
process_config(config_values)
```

## Edge Governance and Graph Operations

Lists in Glang are true graph structures with nodes and edges. The edge governance system provides safe graph operations with configurable rules.

### Adding Edges
```glang
numbers = [10, 20, 30, 40]

# Add edge from index 0 to index 1
numbers.add_edge(0, 1, "next")

# Add edge with custom relationship
numbers.add_edge(1, 2, "flows_to")
```

### Edge Inspection
```glang
# Get all edges as [from_index, to_index, relationship] lists
edges = numbers.edges()
print(edges)  # [[0, 1, "next"], [1, 2, "flows_to"]]

# Count nodes and edges
node_count = numbers.count_nodes()
print(node_count)  # 4

edge_count = numbers.count_edges()
print(edge_count)  # 2

# Check if edge can be added (returns true/false)
can_add = numbers.can_add_edge(0, 2, "skip")
print(can_add)  # true

# Try invalid edge (would create cycle)
can_add = numbers.can_add_edge(3, 0, "cycle")
print(can_add)  # false - blocked by governance rules
```

### Graph Visualization

Lists provide three levels of visualization:

```glang
numbers = [1, 2, 3]
numbers.names(['alpha', 'beta', 'gamma'])
numbers.add_edge(0, 1, "connects")

# Technical inspection - detailed structure
numbers.inspect()
# Graph Structure:
# ========================================
# Type: list
# Nodes: 3
# Edges: 1
# Active Rules: no_list_cycles, same_structure_only
#
# Node Connections:
#   alpha (1):
#     → beta (2) (connects)
#   beta (2): (no outgoing edges)
#   gamma (3): (no outgoing edges)

# Quick shape overview
numbers.visualize()
# [alpha → beta → gamma]

# Clean semantic display
numbers.view()
# ["alpha": 1, "beta": 2, "gamma": 3]

# Get structured summary
summary = numbers.get_graph_summary()
print(summary["type"])        # "list"
print(summary["node_count"])  # 3
print(summary["edge_count"])  # 1
```

### Rule Management
```glang
# View active governance rules
active = numbers.get_active_rules()
print(active)  # ["no_list_cycles", "same_structure_only"]

# Check specific rule status
status = numbers.get_rule_status("no_list_cycles")
print(status)  # "active", "disabled", or "unknown"

# Temporarily disable cycle prevention
numbers.disable_rule("no_list_cycles")

# Now cycles are allowed
numbers.add_edge(2, 0, "cycle")  # Works!

# Re-enable the rule
numbers.enable_rule("no_list_cycles")
```

### Configuration Modes
```glang
# Maximum safety (default)
numbers.configure_for_safe_mode()

# No restrictions for experimentation
numbers.configure_for_experimental_mode()
numbers.add_edge(2, 0, "cycle")  # Now allowed

# Optimized for list processing
numbers.configure_for_list_processing()

# Strict hierarchy for tree structures
numbers.configure_for_tree_structures()
```

See the [Edge Governance Guide](../language_features/edge_governance.md) for complete documentation on graph operations and safety rules.