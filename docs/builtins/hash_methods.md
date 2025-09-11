# Hash Methods

Hash is a built-in collection type in Glang representing key-value mappings. Hashes store data nodes and provide efficient lookup by key.

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

### get(key)
Retrieves a data node by key.
```glang
config = { "host": "localhost", "port": 8080 }
host_node = config.get("host")  # Returns { "host": "localhost" }
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

### Index Access
```glang
config = { "host": "localhost", "port": 8080 }
config["host"]  # Returns { "host": "localhost" } (data node)
config["host"].value()  # Returns "localhost"
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
db_config = app_config["database"].value()
db_host = db_config["host"].value()
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
    current_theme = preferences["theme"].value()
    print("Current theme: " + current_theme)
}

# Get all setting names
setting_names = preferences.keys()
for name in setting_names {
    value = preferences[name].value()
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
    score = scores[subject].value()
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