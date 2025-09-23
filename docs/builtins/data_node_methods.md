# Data Node Methods

Data Node is a built-in type in Glang representing a single key-value pair. Data nodes are the fundamental building blocks of maps and provide a way to associate metadata with values.

## Creating Data Nodes

Data nodes are created using the curly brace syntax with a single key-value pair:

```glang
# Basic data node
user = { "name": "Alice" }

# Type-constrained data node
data<num> score = { "final": 95 }
data<string> label = { "category": "electronics" }
```

## Type Information

### type()
Returns the type of the value.
```glang
user = { "name": "Alice" }
user.type()  # Returns "data"
```

### methods()
Returns a list of all available methods for data nodes.
```glang
item = { "price": 29.99 }
item.methods()  # Returns ["type", "methods", "can", "inspect", "key", "value", ...]
```

### can(method_name)
Checks if a method is available on the data node.
```glang
item = { "price": 29.99 }
item.can("key")  # Returns true
item.can("invalid")  # Returns false
```

### inspect()
Returns detailed information about the data node.
```glang
user = { "name": "Alice" }
user.inspect()  # Returns detailed data node information
```

### size()
For data nodes, returns 1 (single key-value pair).
```glang
item = { "price": 29.99 }
item.size()  # Returns 1
```

## Key-Value Access

### key()
Returns the key of the data node.
```glang
user = { "name": "Alice" }
user.key()  # Returns "name"

score = { "final": 95 }
score.key()  # Returns "final"
```

### value()
Returns the value of the data node.
```glang
user = { "name": "Alice" }
user.value()  # Returns "Alice"

score = { "final": 95 }
score.value()  # Returns 95

config = { "enabled": true }
config.value()  # Returns true
```

## Type Constraints

Data nodes can be type-constrained to ensure the value is of a specific type:

```glang
# Constrained data nodes
data<string> username = { "user": "alice" }
data<num> temperature = { "celsius": 25.5 }
data<bool> flag = { "active": true }
data<list> items = { "products": [1, 2, 3] }

# Attempting to violate constraint throws error
# data<num> invalid = { "count": "five" }  # Error: value must be num
```

## Type Conversion

### to_string()
Converts the data node to a string representation.
```glang
user = { "name": "Alice" }
user.to_string()  # Returns '{ "name": "Alice" }'
```

### to_bool()
Converts the data node to a boolean (always true for non-empty data nodes).
```glang
item = { "price": 29.99 }
item.to_bool()  # Returns true
```

## Immutability Methods

### freeze()
Makes the data node and its value immutable. Returns self for chaining.
```glang
config = { "timeout": 30 }
config.freeze()
config.is_frozen()  # Returns true
# Cannot modify frozen data node
```

### is_frozen()
Checks if the data node is frozen (immutable).
```glang
setting = { "debug": true }
setting.freeze()
setting.is_frozen()  # Returns true
```

### contains_frozen()
Checks if the data node's value is frozen.
```glang
items = [1, 2, 3]
items.freeze()
node = { "list": items }
node.contains_frozen()  # Returns true
```

### can_accept(value)
Checks if a value can be set in the data node (respects frozen state and type constraints).
```glang
data<num> score = { "points": 100 }
score.can_accept(95)  # Returns true
score.can_accept("high")  # Returns false (wrong type)

frozen_value = 50
frozen_value.freeze()
score.can_accept(frozen_value)  # Returns false (can't mix frozen/unfrozen)
```

## Examples

### Configuration Settings
```glang
# Individual configuration items as data nodes
timeout_setting = { "timeout": 30 }
retry_setting = { "retries": 3 }
debug_setting = { "debug": false }

# Access configuration values
timeout = timeout_setting.value()
print("Timeout: " + timeout.to_string() + " seconds")

# Check setting names
if timeout_setting.key() == "timeout" {
    print("Found timeout configuration")
}
```

### Metadata Attachment
```glang
# Attach metadata to values
price_with_currency = { "USD": 29.99 }
temperature_with_unit = { "celsius": 25.5 }
distance_with_unit = { "km": 42.195 }

# Process with metadata awareness
currency = price_with_currency.key()
amount = price_with_currency.value()
print("Price: " + currency + " " + amount.to_string())
```

### Type-Safe Storage
```glang
# Ensure type safety with constraints
data<num> score = { "points": 95 }
data<string> grade = { "letter": "A" }
data<bool> passed = { "result": true }

# Values are guaranteed to be correct type
points = score.value()  # Guaranteed to be num
letter = grade.value()  # Guaranteed to be string
result = passed.value()  # Guaranteed to be bool
```

### Building Maps from Data Nodes
```glang
# Data nodes are the building blocks of maps
name_node = { "name": "Alice" }
age_node = { "age": 25 }
city_node = { "city": "New York" }

# Combine into a map
person = {}
person[name_node.key()] = name_node.value()
person[age_node.key()] = age_node.value()
person[city_node.key()] = city_node.value()
# person: { "name": "Alice", "age": 25, "city": "New York" }
```

### Named Values Pattern
```glang
# Use data nodes to create named values
pi = { "pi": 3.14159 }
e = { "e": 2.71828 }
golden_ratio = { "phi": 1.61803 }

# Use in calculations with clear naming
circle_area = pi.value() * radius * radius
print("Area calculated using " + pi.key() + " = " + pi.value().to_string())
```

### Validation with Data Nodes
```glang
# Validate data with type constraints
func validate_score(score_node) {
    if score_node.type() != "data" {
        return false
    }
    
    score = score_node.value()
    if score.type() != "num" {
        return false
    }
    
    if score < 0 or score > 100 {
        return false
    }
    
    return true
}

test_score = { "exam": 85 }
if validate_score(test_score) {
    print("Valid score: " + test_score.value().to_string())
}
```

### Immutable Data Nodes
```glang
# Create immutable configuration
max_connections = { "max": 100 }
max_connections.freeze()

# Safe to pass around - cannot be modified
configure_server(max_connections)
value = max_connections.value()  # Can still read
# max_connections.value() = 200  # Would throw error
```

### Data Node Collections
```glang
# List of data nodes for structured data
measurements = [
    { "temperature": 25.5 },
    { "humidity": 65.0 },
    { "pressure": 1013.25 }
]

# Process structured data
for measurement in measurements {
    param = measurement.key()
    value = measurement.value()
    print(param + ": " + value.to_string())
}
```

### Conditional Data Nodes
```glang
# Use data nodes for conditional configuration
environment = "production"

if environment == "production" {
    db_config = { "pool_size": 50 }
} else {
    db_config = { "pool_size": 5 }
}

pool_size = db_config.value()
print("Database pool size: " + pool_size.to_string())
```