# JSON Module

The JSON module provides JSON encoding and decoding functionality for data serialization and interchange.

## Importing

```glang
import "json"
```

## Encoding Functions

### encode(value)
Converts a Glang value to a compact JSON string.

```glang
# Encode simple values
json.encode(42)           # "42"
json.encode("hello")      # "\"hello\""
json.encode(true)         # "true"
json.encode(none)         # "null"

# Encode lists
numbers = [1, 2, 3, 4, 5]
json.encode(numbers)      # "[1,2,3,4,5]"

names = ["Alice", "Bob", "Charlie"]
json.encode(names)        # "[\"Alice\",\"Bob\",\"Charlie\"]"

# Encode hashes
user = { "name": "Alice", "age": 25, "active": true }
json.encode(user)         # "{\"name\":\"Alice\",\"age\":25,\"active\":true}"

# Encode nested structures
data = {
    "users": [
        { "id": 1, "name": "Alice" },
        { "id": 2, "name": "Bob" }
    ],
    "count": 2
}
json.encode(data)
# "{\"users\":[{\"id\":1,\"name\":\"Alice\"},{\"id\":2,\"name\":\"Bob\"}],\"count\":2}"
```

### encode_pretty(value)
Converts a Glang value to a formatted JSON string with indentation.

```glang
user = {
    "name": "Alice",
    "age": 25,
    "address": {
        "street": "123 Main St",
        "city": "New York"
    },
    "hobbies": ["reading", "coding", "hiking"]
}

pretty_json = json.encode_pretty(user)
print(pretty_json)
# Output:
# {
#   "name": "Alice",
#   "age": 25,
#   "address": {
#     "street": "123 Main St",
#     "city": "New York"
#   },
#   "hobbies": [
#     "reading",
#     "coding",
#     "hiking"
#   ]
# }
```

## Decoding Functions

### decode(json_string)
Parses a JSON string and returns the corresponding Glang value.

```glang
# Decode simple values
json.decode("42")          # Returns 42
json.decode("\"hello\"")   # Returns "hello"
json.decode("true")        # Returns true
json.decode("null")        # Returns none

# Decode arrays
json_array = "[1, 2, 3, 4, 5]"
numbers = json.decode(json_array)  # Returns list [1, 2, 3, 4, 5]

# Decode objects (returns hash with data nodes)
json_obj = "{\"name\": \"Bob\", \"age\": 30}"
user = json.decode(json_obj)       # Returns hash
name = user["name"].value()        # "Bob"
age = user["age"].value()          # 30

# Decode nested structures
json_data = '''
{
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ],
    "total": 2
}
'''
data = json.decode(json_data)
users = data["users"].value()      # List of hashes
total = data["total"].value()      # 2
```

### is_valid(json_string)
Checks if a string is valid JSON without parsing it.

```glang
# Valid JSON
json.is_valid("{\"key\": \"value\"}")   # Returns true
json.is_valid("[1, 2, 3]")             # Returns true
json.is_valid("\"hello\"")             # Returns true
json.is_valid("42")                    # Returns true

# Invalid JSON
json.is_valid("not json")              # Returns false
json.is_valid("{key: value}")          # Returns false (unquoted key)
json.is_valid("[1, 2, 3,]")            # Returns false (trailing comma)
```

## Type Mapping

### Glang to JSON

| Glang Type | JSON Type | Example |
|------------|-----------|---------|
| string | string | `"hello"` → `"hello"` |
| num | number | `42` → `42` |
| bool | boolean | `true` → `true` |
| none | null | `none` → `null` |
| list | array | `[1, 2, 3]` → `[1, 2, 3]` |
| hash | object | `{"a": 1}` → `{"a": 1}` |
| data node | (converted to value) | `{"key": "val"}` → value only |

### JSON to Glang

| JSON Type | Glang Type | Example |
|-----------|------------|---------|
| string | string | `"hello"` → `"hello"` |
| number | num | `42` → `42` |
| boolean | bool | `true` → `true` |
| null | none | `null` → `none` |
| array | list | `[1, 2, 3]` → `[1, 2, 3]` |
| object | hash (with data nodes) | `{"a": 1}` → hash with data nodes |

## Working with Decoded Data

When decoding JSON objects, the result is a hash where each value is accessed through data nodes:

```glang
json_string = '{"name": "Alice", "age": 25, "scores": [95, 87, 92]}'
data = json.decode(json_string)

# Access simple values through data nodes
name = data["name"].value()        # "Alice"
age = data["age"].value()          # 25

# Access nested arrays
scores = data["scores"].value()    # [95, 87, 92]
first_score = scores[0]            # 95

# Type-safe access pattern
if data.has_key("name") {
    name_node = data["name"]
    if name_node.type() == "data" {
        name = name_node.value()
        print("Name: " + name)
    }
}
```

## Examples

### Configuration File Management
```glang
import "json"
import "io"

# Save configuration to JSON file
config = {
    "server": {
        "host": "localhost",
        "port": 8080,
        "ssl": false
    },
    "database": {
        "host": "db.example.com",
        "port": 5432,
        "name": "myapp"
    },
    "features": ["auth", "logging", "caching"],
    "debug": true
}

# Convert to pretty JSON and save
json_config = json.encode_pretty(config)
io.write_file("config.json", json_config)
print("Configuration saved")

# Load configuration from JSON file
if io.exists("config.json") {
    content = io.read_file("config.json")
    if json.is_valid(content) {
        loaded_config = json.decode(content)
        
        # Access configuration values
        server = loaded_config["server"].value()
        db_host = loaded_config["database"]["host"].value()
        features = loaded_config["features"].value()
        
        print("Server host: " + server["host"].value())
        print("Database: " + db_host)
        print("Features: " + features.to_string())
    }
}
```

### API Response Processing
```glang
import "json"

# Simulate API response
api_response = '''
{
    "status": "success",
    "data": {
        "users": [
            {"id": 1, "name": "Alice", "role": "admin"},
            {"id": 2, "name": "Bob", "role": "user"},
            {"id": 3, "name": "Charlie", "role": "user"}
        ],
        "total": 3,
        "page": 1
    }
}
'''

# Parse response
response = json.decode(api_response)
status = response["status"].value()

if status == "success" {
    data = response["data"].value()
    users = data["users"].value()
    total = data["total"].value()
    
    print("Found " + total.to_string() + " users:")
    
    for user_data in users {
        user_id = user_data["id"].value()
        user_name = user_data["name"].value()
        user_role = user_data["role"].value()
        
        print("  [" + user_id.to_string() + "] " + user_name + " (" + user_role + ")")
    }
}
```

### Data Export/Import
```glang
import "json"
import "io"

# Create data for export
students = [
    { "name": "Alice", "grade": 95, "subjects": ["Math", "Science"] },
    { "name": "Bob", "grade": 87, "subjects": ["English", "History"] },
    { "name": "Charlie", "grade": 92, "subjects": ["Math", "English"] }
]

export_data = {
    "school": "Example High",
    "year": 2024,
    "students": students,
    "exported_at": "2024-01-15T10:30:00Z"
}

# Export to JSON
json_export = json.encode_pretty(export_data)
io.write_file("students.json", json_export)
print("Data exported to students.json")

# Import from JSON
if io.exists("students.json") {
    import_json = io.read_file("students.json")
    imported = json.decode(import_json)
    
    school = imported["school"].value()
    year = imported["year"].value()
    students = imported["students"].value()
    
    print("Imported data from " + school + " (" + year.to_string() + ")")
    print("Student count: " + students.size().to_string())
}
```

### Settings Validation
```glang
import "json"

func validate_settings(json_string) {
    # Check if valid JSON
    if not json.is_valid(json_string) {
        print("Error: Invalid JSON format")
        return false
    }
    
    # Parse and validate structure
    settings = json.decode(json_string)
    
    # Check required fields
    required = ["api_key", "endpoint", "timeout"]
    for field in required {
        if not settings.has_key(field) {
            print("Error: Missing required field: " + field)
            return false
        }
    }
    
    # Validate types
    timeout = settings["timeout"].value()
    if timeout.type() != "num" {
        print("Error: timeout must be a number")
        return false
    }
    
    if timeout < 0 or timeout > 300 {
        print("Error: timeout must be between 0 and 300")
        return false
    }
    
    return true
}

# Test validation
valid_json = '{"api_key": "secret", "endpoint": "api.example.com", "timeout": 30}'
invalid_json = '{"api_key": "secret", "endpoint": "api.example.com"}'

if validate_settings(valid_json) {
    print("Settings are valid")
}

if not validate_settings(invalid_json) {
    print("Settings validation failed")
}
```

### Dynamic JSON Building
```glang
import "json"

func build_report(title, data_points) {
    # Build JSON structure dynamically
    report = {
        "title": title,
        "generated": "2024-01-15",
        "summary": {
            "total_points": data_points.size(),
            "min_value": data_points.min(),
            "max_value": data_points.max(),
            "average": data_points.sum() / data_points.size()
        },
        "data": data_points
    }
    
    return json.encode_pretty(report)
}

# Generate report
measurements = [23.5, 24.1, 22.8, 25.3, 24.7]
report_json = build_report("Temperature Report", measurements)
print(report_json)
```

### Error Handling
```glang
import "json"

func safe_parse(json_string) {
    # Validate before parsing
    if not json.is_valid(json_string) {
        return { "error": "Invalid JSON", "data": none }
    }
    
    # Parse and wrap in result
    data = json.decode(json_string)
    return { "error": none, "data": data }
}

# Use safe parsing
test_json = '{"key": "value"}'
result = safe_parse(test_json)

if result["error"].value() == none {
    data = result["data"].value()
    print("Parsed successfully")
} else {
    error = result["error"].value()
    print("Parse error: " + error)
}
```

## Best Practices

1. **Always validate** JSON strings with `is_valid()` before decoding to avoid errors.

2. **Use encode_pretty()** for human-readable output and debugging.

3. **Use encode()** for network transmission and storage to minimize size.

4. **Access decoded values** through `.value()` method on data nodes.

5. **Check key existence** with `has_key()` before accessing hash values.

6. **Handle type conversions** carefully when working with decoded data.

7. **Wrap parsing in error handling** for robust applications.