# Universal Reflection Methods

Universal reflection methods are available on all Glang values, providing introspection and runtime type information capabilities. These methods enable dynamic programming and meta-programming patterns.

## Core Reflection Methods

These methods are available on every value in Glang, regardless of type:

### type()
Returns the type of any value as a string.

```glang
# Works on all types
"hello".type()        # Returns "string"
42.type()            # Returns "num"
true.type()          # Returns "bool"
[1, 2, 3].type()     # Returns "list"
{ "key": "value" }.type()  # Returns "hash"
{ "single": 1 }.type()     # Returns "data"
(x => x * 2).type()  # Returns "function"
```

### methods()
Returns a list of all available methods for the value's type.

```glang
text = "hello"
text.methods()
# Returns ["type", "methods", "can", "inspect", "size", "upper", "lower", "trim", ...]

numbers = [1, 2, 3]
numbers.methods()
# Returns ["type", "methods", "can", "inspect", "size", "append", "prepend", ...]
```

### can(method_name)
Checks if a specific method is available on the value.

```glang
value = "hello"
value.can("upper")    # Returns true (strings have upper())
value.can("append")   # Returns false (strings don't have append())

list = [1, 2, 3]
list.can("append")    # Returns true
list.can("upper")     # Returns false
```

### inspect()
Returns detailed information about the value, useful for debugging and introspection.

```glang
data = [1, 2, 3]
data.inspect()
# Returns detailed information about the list

config = { "host": "localhost", "port": 8080 }
config.inspect()
# Returns detailed information about the hash
```

### size()
Returns the size of the value. The meaning varies by type:
- **Strings**: Number of characters
- **Lists**: Number of elements
- **Hashes**: Number of key-value pairs
- **Numbers, Booleans**: Always returns 1 (atomic values)
- **Data Nodes**: Always returns 1 (single key-value pair)

```glang
"hello".size()           # Returns 5
[1, 2, 3, 4].size()     # Returns 4
{ "a": 1, "b": 2 }.size()  # Returns 2
42.size()               # Returns 1
true.size()             # Returns 1
```

## Advanced Reflection Patterns

### Dynamic Method Invocation
```glang
# Check if method exists before calling
value = "hello world"
method_name = "upper"

if value.can(method_name) {
    # Safe to call the method
    result = value.upper()  # Would need dynamic invocation in practice
    print(result)
}
```

### Type-Based Dispatch
```glang
func process_value(value) {
    value_type = value.type()
    
    if value_type == "string" {
        return value.upper()
    } else if value_type == "num" {
        return value * 2
    } else if value_type == "list" {
        return value.size()
    } else {
        return value.to_string()
    }
}

print(process_value("hello"))    # "HELLO"
print(process_value(21))         # 42
print(process_value([1, 2, 3]))  # 3
```

### Method Discovery
```glang
# Discover common methods between types
string_methods = "test".methods()
list_methods = [].methods()

# Find methods available on both
common_methods = []
for method in string_methods {
    if list_methods.count(method) > 0 {
        common_methods.append(method)
    }
}
print("Common methods: " + common_methods.to_string())
# Includes: type, methods, can, inspect, size, to_string, to_bool, etc.
```

### Generic Size Checking
```glang
func is_empty(value) {
    # Works for any value with a size
    if value.can("size") {
        return value.size() == 0
    }
    return false
}

print(is_empty(""))         # true
print(is_empty([]))         # true
print(is_empty({}))         # true
print(is_empty("hello"))    # false
print(is_empty([1, 2, 3]))  # false
```

### Runtime Type Validation
```glang
func validate_type(value, expected_type) {
    actual_type = value.type()
    if actual_type != expected_type {
        print("Type mismatch: expected " + expected_type + ", got " + actual_type)
        return false
    }
    return true
}

# Use for input validation
user_input = "42"
if validate_type(user_input, "string") {
    # Safe to use string methods
    number = user_input.to_num()
}
```

### Method Availability Checking
```glang
func safe_transform(value, transformation) {
    # Check if transformation is available
    if not value.can(transformation) {
        print("Warning: " + transformation + " not available on " + value.type())
        return value
    }
    
    # Apply transformation (would need dynamic invocation)
    if transformation == "upper" and value.type() == "string" {
        return value.upper()
    } else if transformation == "double" and value.type() == "num" {
        return value * 2
    }
    # ... other transformations
    
    return value
}
```

### Polymorphic Functions
```glang
func get_length(value) {
    # Works with any value that has size
    if value.can("size") {
        return value.size()
    } else if value.can("length") {
        return value.length()
    } else {
        return 1  # Atomic value
    }
}

print(get_length("hello"))      # 5
print(get_length([1, 2, 3, 4])) # 4
print(get_length(42))           # 1
```

### Type Inspection Utilities
```glang
func describe_value(value) {
    description = "Type: " + value.type()
    
    if value.can("size") {
        description = description + ", Size: " + value.size().to_string()
    }
    
    method_count = value.methods().size()
    description = description + ", Methods: " + method_count.to_string()
    
    return description
}

print(describe_value("hello"))
# "Type: string, Size: 5, Methods: 25"

print(describe_value([1, 2, 3]))
# "Type: list, Size: 3, Methods: 30"
```

### Dynamic Property Access
```glang
func get_info(value) {
    info = {}
    info["type"] = value.type()
    
    # Add size if available
    if value.can("size") {
        info["size"] = value.size()
    }
    
    # Add constraint if it's a list
    if value.type() == "list" and value.can("constraint") {
        constraint = value.constraint()
        if constraint != none {
            info["constraint"] = constraint
        }
    }
    
    return info
}

list<num> scores = [95, 87, 92]
info = get_info(scores)
# info: { "type": "list", "size": 3, "constraint": "num" }
```

## Best Practices

1. **Always check with can()** before calling methods dynamically to avoid runtime errors.

2. **Use type() for dispatch** when implementing polymorphic behavior.

3. **Leverage inspect()** during development for debugging complex data structures.

4. **Cache methods() results** if checking multiple methods on the same value.

5. **Combine with type constraints** to ensure type safety while maintaining flexibility.

## Common Use Cases

### Input Validation
```glang
func validate_input(input, expected_type, min_size) {
    if input.type() != expected_type {
        return false
    }
    
    if input.can("size") and input.size() < min_size {
        return false
    }
    
    return true
}
```

### Generic Containers
```glang
func create_container(item_type) {
    container = {
        "type": item_type,
        "items": [],
        "add": func(item) {
            if item.type() == item_type {
                items.append(item)
            }
        }
    }
    return container
}
```

### Debugging Helpers
```glang
func debug_print(value, label) {
    print(label + ":")
    print("  Type: " + value.type())
    print("  Methods available: " + value.methods().size().to_string())
    print("  Details: " + value.inspect())
}
```