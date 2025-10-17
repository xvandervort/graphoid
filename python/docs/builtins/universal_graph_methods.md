# Universal Graph Methods

Universal graph methods are available on all graph types in Glang (lists, maps, trees), providing consistent access patterns regardless of the specific graph structure. These methods enable writing generic code that works with any graph type.

## Core Universal Methods

### first()
Returns the first element or key-value pair from a graph structure. The behavior is type-specific but predictable:

- **Lists**: Returns the first element (index 0)
- **Maps**: Returns a new map containing only the first inserted key-value pair
- **Trees**: Returns `none` (no meaningful first element)
- **Empty collections**: Returns `none` safely

```glang
# Lists - returns the element at index 0
numbers = [10, 20, 30, 40]
numbers.first()  # Returns 10

# Maps - returns map with first inserted key-value pair
config = { "host": "localhost", "port": 8080, "debug": true }
config.first()  # Returns { "host": "localhost" }

# Empty collections safely return none
[].first()    # Returns none
{}.first()    # Returns none

# Trees return none (no meaningful first)
tree = tree{}
tree.first()  # Returns none
```

### last()
Returns the last element or key-value pair from a graph structure:

- **Lists**: Returns the last element (index -1)
- **Maps**: Returns a new map containing only the last inserted key-value pair
- **Trees**: Returns `none` (no meaningful last element)
- **Empty collections**: Returns `none` safely

```glang
# Lists - returns the element at the last index
numbers = [10, 20, 30, 40]
numbers.last()   # Returns 40

# Maps - returns map with last inserted key-value pair
config = { "host": "localhost", "port": 8080, "debug": true }
config.last()   # Returns { "debug": true }

# Empty collections safely return none
[].last()     # Returns none
{}.last()     # Returns none

# Trees return none (no meaningful last)
tree = tree{}
tree.last()   # Returns none
```

## Type-Specific Behavior

### Lists
For lists, `first()` and `last()` provide direct access to boundary elements:

```glang
# Basic usage
items = ["apple", "banana", "cherry", "date"]
items.first()  # Returns "apple"
items.last()   # Returns "date"

# Single element lists
singleton = [42]
singleton.first()  # Returns 42
singleton.last()   # Returns 42

# Works with any element type
mixed = [1, "hello", true, [1, 2, 3]]
mixed.first()  # Returns 1
mixed.last()   # Returns [1, 2, 3]
```

### Maps
For maps, these methods respect insertion order and return new map objects:

```glang
# Insertion order is preserved
settings = {}
settings["theme"] = "dark"     # First insertion
settings["lang"] = "en"        # Second insertion
settings["debug"] = false      # Last insertion

first_setting = settings.first()   # Returns { "theme": "dark" }
last_setting = settings.last()     # Returns { "debug": false }

# Results are proper maps, not data nodes
first_setting.type()          # Returns "map" (not "data")
first_setting.keys()          # Returns ["theme"]
first_setting["theme"]        # Returns "dark"

# Works with complex values
complex_map = {
    "users": ["alice", "bob"],
    "config": { "port": 8080 },
    "stats": { "count": 42 }
}
complex_map.first()   # Returns { "users": ["alice", "bob"] }
complex_map.last()    # Returns { "stats": { "count": 42 } }
```

### Trees and Empty Collections
Trees and empty collections provide safe, predictable behavior:

```glang
# Trees return none (no ordering concept)
binary_tree = tree{}
binary_tree.insert(10)
binary_tree.insert(5)
binary_tree.insert(15)

binary_tree.first()  # Returns none
binary_tree.last()   # Returns none

# Empty collections are safe
[].first()           # Returns none
{}.first()           # Returns none
[].last()            # Returns none
{}.last()            # Returns none
```

## Method Chaining and Composition

Results from `first()` and `last()` can be used in method chains:

```glang
# Chain operations on map results
config = { "servers": ["web1", "web2"], "timeout": 30, "retries": 3 }

# Get keys from first entry
first_keys = config.first().keys()    # Returns ["servers"]

# Get the value from last entry
last_value = config.last()["retries"] # Returns 3

# Complex chaining
server_list = config.first()["servers"]  # Returns ["web1", "web2"]
first_server = server_list.first()       # Returns "web1"

# Works with lists containing maps
data = [
    { "name": "Alice", "age": 30 },
    { "name": "Bob", "age": 25 },
    { "name": "Carol", "age": 35 }
]

first_person = data.first()        # Returns { "name": "Alice", "age": 30 }
first_name = data.first()["name"]  # Returns "Alice"
last_age = data.last()["age"]      # Returns 35
```

## Error Handling

These methods are designed to be safe and never throw errors:

```glang
# No arguments allowed
items = [1, 2, 3]
# items.first(5)   # Error: first() takes no arguments
# items.last(2)    # Error: last() takes no arguments

# Always safe with empty collections
empty_list = []
empty_map = {}

empty_list.first()  # Returns none (never errors)
empty_map.last()    # Returns none (never errors)

# Safe with none values
result = [].first()
if result != none {
    # Safe to use result
    print(result)
}
```

## Generic Programming Patterns

Use these methods to write code that works with any graph type:

```glang
func get_boundary_elements(graph) {
    first_elem = graph.first()
    last_elem = graph.last()

    boundaries = {}

    if first_elem != none {
        boundaries["first"] = first_elem
    }

    if last_elem != none {
        boundaries["last"] = last_elem
    }

    return boundaries
}

# Works with any graph type
list_bounds = get_boundary_elements([1, 2, 3, 4, 5])
# Returns { "first": 1, "last": 5 }

map_bounds = get_boundary_elements({ "a": 1, "b": 2, "c": 3 })
# Returns { "first": { "a": 1 }, "last": { "c": 3 } }

tree_bounds = get_boundary_elements(tree{})
# Returns {} (empty - trees have no boundaries)
```

### Polymorphic Processing

```glang
func process_endpoints(collection, processor) {
    # Handle first element if it exists
    first = collection.first()
    if first != none {
        processor(first, "first")
    }

    # Handle last element if it exists
    last = collection.last()
    if last != none and last != first {  # Avoid duplicate processing
        processor(last, "last")
    }
}

# Works with different graph types
display = func(element, position) {
    print(position + ": " + element.to_string())
}

process_endpoints([10, 20, 30], display)
# Prints: "first: 10" and "last: 30"

process_endpoints({ "x": 100, "y": 200 }, display)
# Prints: "first: {x: 100}" and "last: {y: 200}"
```

### Type-Safe Boundary Access

```glang
func safe_boundary_access(graph, boundary) {
    if not graph.can("first") or not graph.can("last") {
        return none
    }

    if boundary == "start" {
        return graph.first()
    } else if boundary == "end" {
        return graph.last()
    }

    return none
}

# Safe for any type
result = safe_boundary_access([1, 2, 3], "start")  # Returns 1
result = safe_boundary_access(tree{}, "end")       # Returns none
```

## Performance Considerations

- **Lists**: O(1) - direct index access
- **Maps**: O(1) - uses insertion order tracking
- **Trees**: O(1) - immediate none return
- **Memory**: Map methods create new single-entry maps (minimal overhead)

## Best Practices

1. **Check for none**: Always handle the possibility of none return values
2. **Type awareness**: Remember that map methods return maps, not individual values
3. **Empty collection safety**: These methods never throw errors on empty collections
4. **Method chaining**: Leverage the fact that results are proper graph objects
5. **Generic code**: Use these methods to write functions that work with any graph type

## Integration with Graph Architecture

These universal methods are built on Glang's true graph-theoretic foundation:

```glang
# All graph types support these methods
numbers = [1, 2, 3]
numbers.can("first")   # Returns true
numbers.can("last")    # Returns true

settings = { "theme": "dark" }
settings.can("first")  # Returns true
settings.can("last")   # Returns true

binary_tree = tree{}
binary_tree.can("first") # Returns true
binary_tree.can("last")  # Returns true

# They appear in method lists
numbers.methods()      # Includes "first" and "last"
settings.methods()     # Includes "first" and "last"
binary_tree.methods()  # Includes "first" and "last"
```

The universal `first()` and `last()` methods demonstrate Glang's commitment to consistent APIs across all graph types, enabling powerful generic programming while respecting the semantic differences between data structures.