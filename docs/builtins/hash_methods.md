# Hash Methods

Hash is a built-in collection type in Glang representing key-value mappings. Hashes store data nodes and provide efficient lookup by key.

## ✨ Recent Changes

**Hash Access Improvement**: Hash indexing now returns values directly for intuitive use:
- **NEW**: `hash[key]` returns the value directly (e.g., `"localhost"`, `8080`)
- **NEW**: `hash.node(key)` for explicit data node access when needed
- **DEPRECATED**: `hash.get(key)` issues deprecation warnings - use the above instead

This makes hash usage much more intuitive while preserving access to Glang's graph-theoretic features when explicitly needed.

## Type Information

### type()
Returns the type of the value.
```glang
config = { "host": "localhost", "port": 8080 }
config.type()  # Returns "hash"
```

### methods()
Returns a list of all available methods for hashes.
```glang
config = { "debug": true }
config.methods()  # Returns ["type", "methods", "can", "inspect", "get", "set", "has_key", ...]
```

### can(method_name)
Checks if a method is available on the hash.
```glang
config = { "key": "value" }
config.can("get")  # Returns true
config.can("invalid")  # Returns false
```

### inspect()
Returns detailed information about the hash.
```glang
config = { "host": "localhost", "port": 8080 }
config.inspect()  # Returns detailed hash information
```

### size()
Returns the number of key-value pairs in the hash.
```glang
settings = { "theme": "dark", "lang": "en", "debug": true }
settings.size()  # Returns 3
```

## Key-Value Operations

### get(key) ⚠️ DEPRECATED
**DEPRECATED**: Use `hash[key]` for direct value access or `hash.node(key)` for data node access.

Retrieves a data node by key.
```glang
config = { "host": "localhost", "port": 8080 }
# DEPRECATED - Issues deprecation warning
host_node = config.get("host")  # Returns { "host": "localhost" }
host_node.value()  # Returns "localhost"

# PREFERRED - Direct value access
host = config["host"]  # Returns "localhost" directly

# PREFERRED - Explicit data node access when needed
host_node = config.node("host")  # Returns { "host": "localhost" }
```

### node(key) ✅ NEW
Explicitly retrieves a data node by key. Use this when you need access to the graph-theoretic data node structure.
```glang
config = { "host": "localhost", "port": 8080 }
host_node = config.node("host")  # Returns { "host": "localhost" } (data node)
host_node.key()    # Returns "host"
host_node.value()  # Returns "localhost"
```

### set(key, value)
Sets or updates a key-value pair.
```glang
config = { "host": "localhost" }
config.set("port", 8080)  # config becomes { "host": "localhost", "port": 8080 }
config.set("host", "127.0.0.1")  # Updates existing key
```

### has_key(key)
Checks if a key exists in the hash.
```glang
config = { "host": "localhost", "port": 8080 }
config.has_key("host")  # Returns true
config.has_key("debug")  # Returns false
```

### remove(key)
Removes a key-value pair from the hash.
```glang
config = { "host": "localhost", "port": 8080, "debug": true }
config.remove("debug")  # config becomes { "host": "localhost", "port": 8080 }
```

### pop(key)
Removes and returns a data node by key.
```glang
config = { "host": "localhost", "port": 8080 }
port_node = config.pop("port")  # Returns { "port": 8080 }
# config now only has { "host": "localhost" }
```

### push(key, value)
Alias for set(). Adds or updates a key-value pair.
```glang
config = {}
config.push("theme", "dark")  # config becomes { "theme": "dark" }
```

## Collection Operations

### keys()
Returns a list of all keys in the hash.
```glang
config = { "host": "localhost", "port": 8080, "debug": true }
config.keys()  # Returns ["host", "port", "debug"]
```

### values()
Returns a list of all values in the hash.
```glang
config = { "host": "localhost", "port": 8080, "debug": true }
config.values()  # Returns ["localhost", 8080, true]
```

### empty()
Checks if the hash is empty.
```glang
{}.empty()  # Returns true
{ "key": "value" }.empty()  # Returns false
```

### count_values(value)
Counts occurrences of a specific value in the hash.
```glang
settings = { "color1": "blue", "color2": "red", "color3": "blue" }
settings.count_values("blue")  # Returns 2
settings.count_values("green")  # Returns 0
```

### merge(other_hash)
Merges another hash into this one.
```glang
config1 = { "host": "localhost", "port": 8080 }
config2 = { "debug": true, "port": 9000 }
config1.merge(config2)  
# config1 becomes { "host": "localhost", "port": 9000, "debug": true }
```

## Type Constraints

Hashes can be type-constrained to ensure all values are of the same type:

```glang
# Type-constrained hash
hash<string> settings = { "theme": "dark", "lang": "en" }
settings["font"] = "Arial"  # OK
# settings["size"] = 12  # Would throw error (not a string)

# Mixed-type hash (no constraint)
config = { "host": "localhost", "port": 8080, "debug": true }
```

## Type Conversion

### to_string()
Converts the hash to a string representation.
```glang
config = { "host": "localhost", "port": 8080 }
config.to_string()  # Returns '{"host": "localhost", "port": 8080}'
```

### to_bool()
Converts the hash to a boolean. Empty hashes are false, non-empty are true.
```glang
{ "key": "value" }.to_bool()  # Returns true
{}.to_bool()  # Returns false
```

## Immutability Methods

### freeze()
Makes the hash and all its contents immutable. Returns self for chaining.
```glang
config = { "host": "localhost", "port": 8080 }
config.freeze()
config.is_frozen()  # Returns true
# config["debug"] = true  # Would throw error
```

### is_frozen()
Checks if the hash is frozen (immutable).
```glang
config = { "key": "value" }
config.freeze()
config.is_frozen()  # Returns true
```

### contains_frozen()
Checks if the hash contains any frozen values.
```glang
config = { "host": "localhost" }
frozen_value = "frozen"
frozen_value.freeze()
# config["key"] = frozen_value  # Would throw error (can't mix frozen/unfrozen)
```

### can_accept(value)
Checks if a value can be added to the hash (respects frozen state).
```glang
config = { "host": "localhost" }
frozen_value = 8080
frozen_value.freeze()
config.can_accept(frozen_value)  # Returns false (can't mix)
```

## Index Operations

Hashes support bracket notation for accessing and setting values:

### Index Access ✅ PREFERRED
```glang
config = { "host": "localhost", "port": 8080 }
# NEW BEHAVIOR - Returns values directly (intuitive!)
config["host"]  # Returns "localhost" directly
config["port"]  # Returns 8080 directly

# For explicit data node access when needed:
config.node("host")  # Returns { "host": "localhost" } (data node)
```

### Index Assignment
```glang
config = { "host": "localhost" }
config["port"] = 8080  # Adds new key
config["host"] = "127.0.0.1"  # Updates existing key
```

## Examples

### Configuration Management
```glang
# Application configuration
app_config = {
    "database": {
        "host": "localhost",
        "port": 5432,
        "name": "myapp"
    },
    "cache": {
        "enabled": true,
        "ttl": 3600
    },
    "debug": false
}

# Access nested configuration
db_config = app_config["database"]  # Direct value access
db_host = db_config["host"]  # Direct value access
print("Database host: " + db_host)

# Update configuration
app_config["debug"] = true
app_config.set("version", "1.0.0")
```

### User Settings
```glang
# User preferences
preferences = {
    "theme": "dark",
    "language": "en",
    "notifications": true,
    "font_size": 14
}

# Check and update settings
if preferences.has_key("theme") {
    current_theme = preferences["theme"]  # Direct value access
    print("Current theme: " + current_theme)
}

# Get all setting names
setting_names = preferences.keys()
for name in setting_names {
    value = preferences[name]  # Direct value access
    print(name + ": " + value.to_string())
}
```

### Data Aggregation
```glang
# Student scores by subject
scores = {
    "math": 95,
    "science": 87,
    "english": 92,
    "history": 88
}

# Calculate statistics
subjects = scores.keys()
all_scores = scores.values()
total = 0

for score in all_scores {
    total = total + score
}

average = total / scores.size()
print("Average score: " + average.to_string())

# Find highest score
max_score = 0
best_subject = ""
for subject in subjects {
    score = scores[subject]  # Direct value access
    if score > max_score {
        max_score = score
        best_subject = subject
    }
}
print("Best subject: " + best_subject + " (" + max_score.to_string() + ")")
```

### Hash Merging
```glang
# Default settings
defaults = {
    "timeout": 30,
    "retries": 3,
    "logging": true
}

# User overrides
user_settings = {
    "timeout": 60,
    "verbose": true
}

# Merge settings (user overrides defaults)
defaults.merge(user_settings)
# defaults now: { "timeout": 60, "retries": 3, "logging": true, "verbose": true }
```

### Type-Constrained Hashes
```glang
# Ensure all values are strings
hash<string> labels = {
    "name": "Product Name",
    "description": "Product Description",
    "category": "Electronics"
}

labels["brand"] = "BrandName"  # OK
# labels["price"] = 99.99  # Would throw error

# Ensure all values are numbers
hash<num> metrics = {
    "cpu_usage": 45.2,
    "memory_usage": 67.8,
    "disk_usage": 82.1
}

metrics["network_usage"] = 23.5  # OK
```

### Building Hashes Dynamically
```glang
# Build hash from parallel lists
keys = ["name", "age", "city"]
values = ["Alice", 25, "New York"]

person = {}
for i in [0, 1, 2] {
    key = keys[i]
    value = values[i]
    person[key] = value
}
# person: { "name": "Alice", "age": 25, "city": "New York" }
```

### Immutable Configuration
```glang
# Create immutable settings
system_config = {
    "max_connections": 100,
    "timeout": 30,
    "buffer_size": 4096
}
system_config.freeze()

# Safe to share - cannot be modified
process_with_config(system_config)
# system_config["timeout"] = 60  # Would throw error
```

## Edge Governance and Graph Operations

Hashes in Glang are true graph structures where keys connect to value nodes. The edge governance system provides safe graph operations with configurable rules.

### Adding Value Edges
```glang
config = { "host": "localhost", "port": 8080, "ssl": true }

# Add edge between values (beyond key-based structure)
config.add_value_edge("host", "port", "connects_to")
config.add_value_edge("port", "ssl", "requires")
```

### Edge Inspection
```glang
# Get all edges as [from_key, to_key, relationship] lists
edges = config.get_edges()
print(edges)  # [["host", "port", "connects_to"], ["port", "ssl", "requires"]]

# Count total edges between values
count = config.get_edge_count()
print(count)  # 2

# Check if edge can be added between two keys
can_add = config.can_add_edge("ssl", "host", "back_reference")
print(can_add)  # true or false, depending on governance rules

# Get keys whose values are connected to a specific key
connected = config.get_connected_keys("port", "connects_to")
print(connected)  # ["ssl"] (if port connects to ssl with "connects_to")
```

### Graph Visualization
```glang
config = { "db": "mysql", "cache": "redis", "queue": "rabbitmq" }
config.add_value_edge("db", "cache", "uses")

# Get structured summary
summary = config.get_graph_summary()
print(summary["type"])        # "hash"
print(summary["node_count"])  # 3 (plus root node)
print(summary["edge_count"])  # 1

# Visualize structure (text format)
viz = config.visualize_structure("text")
print(viz)
# Graph Structure:
# ========================================
# Type: hash
# Nodes: 4
# Edges: 4 (3 key-value + 1 custom)
# Active Rules: no_list_cycles, same_structure_only

# DOT format for Graphviz
dot = config.visualize_structure("dot")

# Compact summary
summary = config.visualize_structure("summary")
print(summary)  # [HASH] 4 nodes, 4 edges
```

### Rule Management
```glang
# View active governance rules
active = config.get_active_rules()
print(active)  # ["no_list_cycles", "same_structure_only"]

# Check specific rule status
status = config.get_rule_status("same_structure_only")
print(status)  # "active", "disabled", or "unknown"

# Temporarily disable cross-structure protection
config.disable_rule("same_structure_only")

# Now cross-hash edges might be allowed (depending on implementation)
# config.add_external_edge(other_hash)

# Re-enable the rule
config.enable_rule("same_structure_only")
```

### Configuration Modes
```glang
# Maximum safety (default)
config.configure_for_safe_mode()

# No restrictions for experimentation
config.configure_for_experimental_mode()

# Optimized for list processing
config.configure_for_list_processing()

# Strict hierarchy for tree structures
config.configure_for_tree_structures()
```

### Value Relationships
```glang
# Model configuration dependencies
services = {
    "web": "nginx",
    "app": "python",
    "db": "postgresql",
    "cache": "redis"
}

# Define service dependencies
services.add_value_edge("web", "app", "proxies_to")
services.add_value_edge("app", "db", "connects_to")
services.add_value_edge("app", "cache", "uses")

# Query relationships
db_dependencies = services.get_connected_keys("app", "connects_to")
print(db_dependencies)  # ["db"]

all_edges = services.get_edges()
print(all_edges)  # [["web", "app", "proxies_to"], ["app", "db", "connects_to"], ...]
```

See the [Edge Governance Guide](../language_features/edge_governance.md) for complete documentation on graph operations and safety rules.