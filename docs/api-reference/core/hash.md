# hash - Hash/Dictionary Type

Hashes (also called maps or dictionaries) in Graphoid are collections of key-value pairs. Internally, they are implemented as graph structures with edges representing key-value relationships.

## Hash Literals

### Basic Hashes

Hashes are created using curly braces with key-value pairs:

```graphoid
user = {"name": "Alice", "age": 30, "email": "alice@example.com"}
config = {"host": "localhost", "port": 8080, "debug": true}
empty = {}
```

### Type Constraints (Optional)

Hashes can have optional runtime type constraints on values:

```graphoid
# Single-parameter type constraint (values only)
hash<num> scores = {"math": 95, "english": 87}
hash<string> names = {"first": "Alice", "last": "Smith"}

# Type inference (recommended)
scores = {"math": 95, "english": 87}  # Infers hash
```

**Note**: Keys are always strings. Graphoid does NOT support multi-parameter generics like `hash<K,V>`.

---

## Hash Operators

### Indexing (`[]`)

Access values by key.

**Syntax**: `hash[key]`

**Parameters**:
- `key` (string): Key to look up

**Returns**: Value associated with key

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30}
name = user["name"]  # "Alice"
age = user["age"]    # 30

# Access nested structures
data = {"user": {"name": "Bob", "age": 25}}
username = data["user"]["name"]  # "Bob"
```

**Errors**: Accessing non-existent key raises error (or returns `none` with `:lenient`)

**See also**: `get()`, `has_key()`

---

### Assignment (`[]=`)

Sets a value for a key.

**Syntax**: `hash[key] = value`

**Parameters**:
- `key` (string): Key to set
- `value`: Value to associate

**Returns**: none (modifies hash in place)

**Examples**:
```graphoid
user = {}
user["name"] = "Alice"
user["age"] = 30
print(user)  # {"name": "Alice", "age": 30}

# Update existing key
user["age"] = 31

# Create nested structures
user["address"] = {}
user["address"]["city"] = "Boston"
```

**See also**: `set()`, `put()`

---

### Membership (`in`)

Tests if a key exists in the hash.

**Syntax**: `key in hash`

**Parameters**:
- `key` (string): Key to check
- `hash` (hash): Hash to search

**Returns**: (bool) `true` if key exists, `false` otherwise

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30}
result = "name" in user   # true
result = "email" in user  # false

# Check before access
if "email" in user {
    print(user["email"])
}
```

**See also**: `has_key()`, `get()`

---

### Merge (`+`)

Combines two hashes (later values override earlier).

**Syntax**: `hash1 + hash2`

**Parameters**:
- `hash1` (hash): First hash
- `hash2` (hash): Second hash (overrides conflicts)

**Returns**: (hash) Merged hash (does NOT modify originals)

**Examples**:
```graphoid
defaults = {"host": "localhost", "port": 8080, "debug": false}
overrides = {"port": 3000, "debug": true}

config = defaults + overrides
print(config)
# {"host": "localhost", "port": 3000, "debug": true}

# Original hashes unchanged
print(defaults["port"])  # 8080
print(overrides["host"]) # Error: key doesn't exist
```

**See also**: `merge()`

---

## Hash Methods

### length()

Returns the number of key-value pairs.

**Syntax**: `hash.length()`

**Returns**: (num) Number of entries

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30, "email": "alice@example.com"}
count = user.length()
print(count)  # 3

empty = {}
print(empty.length())  # 0
```

**See also**: `is_empty()`, `keys()`

---

### keys()

Returns a list of all keys.

**Syntax**: `hash.keys()`

**Returns**: (list) List of keys

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30, "email": "alice@example.com"}
key_list = user.keys()
print(key_list)  # ["name", "age", "email"]

# Iterate over keys
for key in user.keys() {
    print(key + ": " + user[key].to_string())
}

# Check if any key matches
settings_keys = settings.keys()
has_debug = settings_keys.contains("debug")
```

**See also**: `values()`, `entries()`

---

### values()

Returns a list of all values.

**Syntax**: `hash.values()`

**Returns**: (list) List of values

**Examples**:
```graphoid
scores = {"math": 95, "english": 87, "science": 92}
score_list = scores.values()
print(score_list)  # [95, 87, 92]

# Calculate statistics
average = scores.values().sum() / scores.length()
max_score = scores.values().max()
```

**See also**: `keys()`, `entries()`

---

### entries()

Returns a list of `[key, value]` pairs.

**Syntax**: `hash.entries()`

**Returns**: (list) List of `[key, value]` pairs

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30}
pairs = user.entries()
print(pairs)
# [["name", "Alice"], ["age", 30]]

# Iterate with destructuring
for entry in user.entries() {
    key = entry[0]
    value = entry[1]
    print("${key} = ${value}")
}

# Transform to different format
list = hash.entries().map(pair => {
    return {"key": pair[0], "value": pair[1]}
})
```

**See also**: `keys()`, `values()`

---

### has_key(key)

Tests if a key exists.

**Syntax**: `hash.has_key(key)`

**Parameters**:
- `key` (string): Key to check

**Returns**: (bool) `true` if key exists, `false` otherwise

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30}
result = user.has_key("name")   # true
result = user.has_key("email")  # false

# Same as 'in' operator
result = "name" in user  # true

# Conditional access
if user.has_key("email") {
    send_email(user["email"])
}
```

**See also**: `in`, `get()`

---

### get(key, default)

Gets a value with a default fallback.

**Syntax**: `hash.get(key, default)`

**Parameters**:
- `key` (string): Key to look up
- `default` (optional): Value to return if key doesn't exist

**Returns**: Value for key, or `default` (or `none` if no default)

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30}

# Get with default
email = user.get("email", "unknown@example.com")
print(email)  # "unknown@example.com"

# Get without default
email = user.get("email")
print(email)  # none

# Safe access
config = {"host": "localhost"}
port = config.get("port", 8080)
print(port)  # 8080
```

**See also**: `has_key()`, `[]`

---

### set(key, value)

Sets a key-value pair (alternative to `[]=`).

**Syntax**: `hash.set(key, value)`

**Parameters**:
- `key` (string): Key to set
- `value`: Value to associate

**Returns**: none (modifies hash in place)

**Examples**:
```graphoid
user = {}
user.set("name", "Alice")
user.set("age", 30)

# Same as
user["name"] = "Alice"
user["age"] = 30
```

**See also**: `[]=`, `put()`

---

### remove(key)

Removes a key-value pair.

**Syntax**: `hash.remove(key)`

**Parameters**:
- `key` (string): Key to remove

**Returns**: Removed value, or `none` if key didn't exist

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30, "email": "alice@example.com"}
removed = user.remove("email")
print(removed)  # "alice@example.com"
print(user)     # {"name": "Alice", "age": 30}

# Remove non-existent key
result = user.remove("phone")
print(result)  # none

# Conditional removal
if user.has_key("temp_token") {
    user.remove("temp_token")
}
```

**See also**: `clear()`, `filter()`

---

### clear()

Removes all key-value pairs.

**Syntax**: `hash.clear()`

**Returns**: none (modifies hash in place)

**Examples**:
```graphoid
cache = {"user_1": {...}, "user_2": {...}}
cache.clear()
print(cache)  # {}
print(cache.length())  # 0

# Reset state
session.clear()
```

**See also**: `remove()`

---

### merge(other)

Merges another hash into this one (modifies in place).

**Syntax**: `hash.merge(other)`

**Parameters**:
- `other` (hash): Hash to merge (overrides conflicts)

**Returns**: none (modifies hash in place)

**Examples**:
```graphoid
config = {"host": "localhost", "port": 8080}
overrides = {"port": 3000, "debug": true}

config.merge(overrides)
print(config)
# {"host": "localhost", "port": 3000, "debug": true}

# Merge multiple
config.merge(defaults).merge(user_prefs).merge(cli_args)
```

**See also**: `+` operator

---

### update(key, function)

Updates a value using a function.

**Syntax**: `hash.update(key, function, default)`

**Parameters**:
- `key` (string): Key to update
- `function`: Update function `(old_value) => new_value`
- `default` (optional): Default value if key doesn't exist

**Returns**: none (modifies hash in place)

**Examples**:
```graphoid
counter = {"clicks": 0, "views": 5}

# Increment
counter.update("clicks", c => c + 1)
print(counter["clicks"])  # 1

# With default (key doesn't exist)
stats = {}
stats.update("count", c => c + 1, 0)
print(stats["count"])  # 1

# Complex update
user.update("login_count", count => {
    user["last_login"] = time.now()
    return count + 1
}, 0)
```

**See also**: `set()`, `get()`

---

### map(function)

Transforms values using a function.

**Syntax**: `hash.map(function)`

**Parameters**:
- `function`: Transform function `(key, value) => new_value`

**Returns**: (hash) New hash with transformed values

**Examples**:
```graphoid
prices = {"apple": 1.0, "banana": 0.5, "orange": 0.75}

# Apply discount
discounted = prices.map((k, v) => v * 0.9)
print(discounted)
# {"apple": 0.9, "banana": 0.45, "orange": 0.675}

# Transform values
scores = {"alice": 95, "bob": 87}
grades = scores.map((name, score) => {
    if score >= 90 { return "A" }
    if score >= 80 { return "B" }
    return "C"
})
# {"alice": "A", "bob": "B"}
```

**See also**: `filter()`, `reduce()`

---

### filter(predicate)

Selects entries that match a condition.

**Syntax**: `hash.filter(predicate)`

**Parameters**:
- `predicate`: Test function `(key, value) => bool`

**Returns**: (hash) Filtered hash

**Examples**:
```graphoid
scores = {"alice": 95, "bob": 65, "charlie": 87, "diana": 72}

# Filter by value
passing = scores.filter((name, score) => score >= 70)
print(passing)
# {"alice": 95, "charlie": 87, "diana": 72}

# Filter by key
admins = users.filter((name, user) => name.starts_with("admin_"))

# Complex conditions
active = users.filter((id, user) => {
    return user["active"] and user["verified"]
})
```

**See also**: `map()`, `reject()`

---

### reject(predicate)

Removes entries that match a condition (opposite of filter).

**Syntax**: `hash.reject(predicate)`

**Parameters**:
- `predicate`: Test function `(key, value) => bool`

**Returns**: (hash) Filtered hash

**Examples**:
```graphoid
data = {"name": "Alice", "age": 30, "temp": 123, "_internal": 456}

# Remove temporary fields
clean = data.reject((key, value) => key.starts_with("_") or key == "temp")
print(clean)
# {"name": "Alice", "age": 30}
```

**See also**: `filter()`

---

### reduce(function, initial)

Reduces hash to a single value.

**Syntax**: `hash.reduce(function, initial)`

**Parameters**:
- `function`: Accumulator function `(acc, key, value) => result`
- `initial`: Initial accumulator value

**Returns**: Final accumulated value

**Examples**:
```graphoid
scores = {"alice": 95, "bob": 87, "charlie": 92}

# Sum all scores
total = scores.reduce((acc, name, score) => acc + score, 0)
print(total)  # 274

# Find maximum
max_entry = scores.reduce((acc, name, score) => {
    if score > acc["score"] {
        return {"name": name, "score": score}
    }
    return acc
}, {"name": none, "score": -infinity})

# Build string
summary = scores.reduce((acc, name, score) => {
    return acc + name + ": " + score.to_string() + ", "
}, "")
```

**See also**: `map()`, `filter()`

---

### each(function)

Executes a function for each entry.

**Syntax**: `hash.each(function)`

**Parameters**:
- `function`: Function to execute `(key, value) => none`

**Returns**: none

**Examples**:
```graphoid
user = {"name": "Alice", "age": 30, "email": "alice@example.com"}

# Print all entries
user.each((key, value) => {
    print("${key}: ${value}")
})

# Output:
# name: Alice
# age: 30
# email: alice@example.com

# Side effects
cache.each((key, value) => {
    save_to_disk(key, value)
})
```

**See also**: `map()`, `for` loop

---

### invert()

Swaps keys and values.

**Syntax**: `hash.invert()`

**Returns**: (hash) Inverted hash

**Examples**:
```graphoid
colors = {"red": 1, "green": 2, "blue": 3}
inverted = colors.invert()
print(inverted)
# {1: "red", 2: "green", 3: "blue"}

# ID to name lookup
name_to_id = {"alice": 1, "bob": 2, "charlie": 3}
id_to_name = name_to_id.invert()
print(id_to_name[1])  # "alice"
```

**Note**: If multiple keys have the same value, later ones override earlier ones.

**See also**: `entries()`

---

### select(keys)

Creates a new hash with only specified keys.

**Syntax**: `hash.select(keys)`

**Parameters**:
- `keys` (list): List of keys to include

**Returns**: (hash) Hash with selected keys

**Examples**:
```graphoid
user = {
    "name": "Alice",
    "age": 30,
    "email": "alice@example.com",
    "password_hash": "...",
    "internal_id": 12345
}

# Select public fields
public = user.select(["name", "email"])
print(public)
# {"name": "Alice", "email": "alice@example.com"}

# API response
api_data = user.select(["name", "age", "email"])
```

**See also**: `filter()`, `omit()`

---

### omit(keys)

Creates a new hash without specified keys.

**Syntax**: `hash.omit(keys)`

**Parameters**:
- `keys` (list): List of keys to exclude

**Returns**: (hash) Hash without omitted keys

**Examples**:
```graphoid
user = {
    "name": "Alice",
    "age": 30,
    "email": "alice@example.com",
    "password_hash": "...",
    "internal_id": 12345
}

# Remove sensitive fields
safe = user.omit(["password_hash", "internal_id"])
print(safe)
# {"name": "Alice", "age": 30, "email": "alice@example.com"}
```

**See also**: `select()`, `filter()`

---

## Type Checking

### is_hash()

Tests if a value is a hash.

**Syntax**: `value.is_hash()`

**Returns**: (bool) `true` if hash, `false` otherwise

**Examples**:
```graphoid
result = {}.is_hash()           # true
result = {"a": 1}.is_hash()     # true
result = [1, 2, 3].is_hash()    # false

# Type validation
if not value.is_hash() {
    print("Expected hash")
}
```

**See also**: `is_list()`, `is_string()`

---

### is_empty()

Tests if hash is empty.

**Syntax**: `hash.is_empty()`

**Returns**: (bool) `true` if empty, `false` otherwise

**Examples**:
```graphoid
empty = {}
result = empty.is_empty()  # true

user = {"name": "Alice"}
result = user.is_empty()   # false

# Guard clause
if cache.is_empty() {
    populate_cache()
}
```

**See also**: `length()`

---

## Common Patterns

### Default values

```graphoid
config = {
    "host": settings.get("host", "localhost"),
    "port": settings.get("port", 8080),
    "debug": settings.get("debug", false)
}
```

### Counting occurrences

```graphoid
words = ["apple", "banana", "apple", "cherry", "banana", "apple"]
counts = {}
for word in words {
    counts[word] = counts.get(word, 0) + 1
}
# {"apple": 3, "banana": 2, "cherry": 1}
```

### Grouping by property

```graphoid
users = [
    {"name": "Alice", "role": "admin"},
    {"name": "Bob", "role": "user"},
    {"name": "Charlie", "role": "admin"}
]

by_role = {}
for user in users {
    role = user["role"]
    if not by_role.has_key(role) {
        by_role[role] = []
    }
    by_role[role].append(user)
}
# {"admin": [...], "user": [...]}
```

### Deep merge

```graphoid
fn deep_merge(h1, h2) {
    result = h1[:]  # Copy h1
    for entry in h2.entries() {
        key = entry[0]
        value = entry[1]
        if result.has_key(key) and result[key].is_hash() and value.is_hash() {
            result[key] = deep_merge(result[key], value)
        } else {
            result[key] = value
        }
    }
    return result
}
```

### Convert to query string

```graphoid
fn to_query_string(params) {
    pairs = params.entries().map(entry => {
        key = entry[0]
        value = entry[1]
        return key + "=" + value.to_string()
    })
    return "?" + string.join(pairs, "&")
}

params = {"page": 1, "limit": 10, "sort": "name"}
query = to_query_string(params)
# "?page=1&limit=10&sort=name"
```

---

## Nested Structures

Hashes can contain other hashes and lists:

```graphoid
# Complex nested structure
user = {
    "name": "Alice",
    "age": 30,
    "address": {
        "street": "123 Main St",
        "city": "Boston",
        "zip": "02101"
    },
    "scores": {
        "math": 95,
        "english": 87,
        "science": 92
    },
    "tags": ["admin", "verified", "premium"]
}

# Access nested values
city = user["address"]["city"]  # "Boston"
math_score = user["scores"]["math"]  # 95
first_tag = user["tags"][0]  # "admin"

# Modify nested values
user["address"]["city"] = "Cambridge"
user["scores"]["math"] = 98
user["tags"].append("featured")
```

---

## Graph Operations

Hashes are internally graphs, so they support graph operations:

### as_graph()

Converts hash to explicit graph representation.

**Examples**:
```graphoid
data = {"a": 1, "b": 2, "c": 3}
g = data.as_graph()

# Graph operations available
nodes = g.nodes()  # ["a", "b", "c", 1, 2, 3]
```

**See also**: [Graph Operations](../../user-guide/06-graph-operations.md)

---

## JSON Compatibility

Hashes are directly compatible with JSON:

```graphoid
import "json"

# Hash to JSON
user = {"name": "Alice", "age": 30}
json_str = json.stringify(user)
# '{"name":"Alice","age":30}'

# JSON to hash
data = json.parse('{"name":"Bob","age":25}')
print(data["name"])  # "Bob"
```

**See also**: [json stdlib](../stdlib/json.md)

---

## Performance Notes

- **Key lookups**: O(1) average case (hash table)
- **Iteration**: O(n) where n is number of entries
- **Memory**: Hashes are graph-backed, may use more memory than traditional hash tables
- **Keys**: Always strings; numeric keys are converted to strings

---

## See Also

- [list](list.md) - List/array type
- [tree](tree.md) - Tree data structure
- [graph](graph.md) - Graph data structure
- [json](../stdlib/json.md) - JSON parsing and serialization
- [User Guide: Collections](../../user-guide/05-collections.md) - Collections tutorial
