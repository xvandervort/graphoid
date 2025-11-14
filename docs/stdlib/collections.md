# collections - Collection Utilities Module

The `collections` module extends built-in collections with powerful utilities for working with lists and maps. It provides functional programming tools, set operations, and common data manipulation patterns.

## Overview

Working with collections is fundamental to programming. The `collections` module provides utilities that make it easy to transform, combine, filter, and analyze data structures without writing boilerplate code.

**Quick Example:**
```graphoid
import "collections"

# Split data into batches
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
batches = collections.chunk(data, 3)  # [[1,2,3], [4,5,6], [7,8,9], [10]]

# Find common elements
tags_a = ["python", "web", "tutorial"]
tags_b = ["web", "javascript", "tutorial"]
common = collections.intersection(tags_a, tags_b)  # ["web", "tutorial"]

# Remove duplicates
ids = [101, 102, 101, 103, 102]
unique_ids = collections.unique(ids)  # [101, 102, 103]
```

**Module Alias:** Can also be imported as `coll` (when module aliasing is implemented)

## Functions

### List Utilities

#### chunk(items, size)

Splits a list into chunks (sublists) of a specified size.

**Parameters:**
- `items` - List to split
- `size` - Size of each chunk (must be > 0)

**Returns:** List of sublists, where each sublist has at most `size` elements. The last chunk may be smaller if the list length is not evenly divisible by `size`.

**Examples:**
```graphoid
import "collections"

# Split into groups of 3
numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9]
chunks = collections.chunk(numbers, 3)
# Result: [[1, 2, 3], [4, 5, 6], [7, 8, 9]]

# Last chunk may be smaller
data = [1, 2, 3, 4, 5]
chunks = collections.chunk(data, 2)
# Result: [[1, 2], [3, 4], [5]]

# Batch processing
records = [/* 1000 records */]
batches = collections.chunk(records, 100)  # Process 100 at a time
```

**Time Complexity:** O(n) where n is the length of the list

---

#### partition(items, predicate)

Splits a list into two lists based on a predicate: elements that match and elements that don't.

**Parameters:**
- `items` - List to partition
- `predicate` - Symbol indicating the test (:even, :odd, :positive, :negative, :zero)

**Returns:** List containing two sublists: [matching_elements, non_matching_elements]

**Supported Predicates:**
- `:even` - Even numbers (n % 2 == 0)
- `:odd` - Odd numbers (n % 2 == 1)
- `:positive` - Positive numbers (n > 0)
- `:negative` - Negative numbers (n < 0)
- `:zero` - Zero values (n == 0)

**Examples:**
```graphoid
import "collections"

# Separate even and odd numbers
numbers = [1, 2, 3, 4, 5, 6]
result = collections.partition(numbers, :even)
# Result: [[2, 4, 6], [1, 3, 5]]

# Separate positive and negative
mixed = [1, -2, 3, -4, 5, 0, -6]
result = collections.partition(mixed, :positive)
# Result: [[1, 3, 5], [-2, -4, 0, -6]]

# Destructure results
parts = collections.partition(data, :odd)
odds = parts[0]
evens = parts[1]
```

**Time Complexity:** O(n)

---

#### zip(list1, list2)

Combines two lists element-wise into a list of pairs.

**Parameters:**
- `list1` - First list
- `list2` - Second list

**Returns:** List of two-element lists, where each pair contains corresponding elements from both lists. Length equals the shorter of the two input lists.

**Examples:**
```graphoid
import "collections"

# Pair up names and scores
names = ["Alice", "Bob", "Charlie"]
scores = [95, 87, 92]
pairs = collections.zip(names, scores)
# Result: [["Alice", 95], ["Bob", 87], ["Charlie", 92]]

# Different lengths - stops at shorter list
letters = ["a", "b", "c"]
numbers = [1, 2]
zipped = collections.zip(letters, numbers)
# Result: [["a", 1], ["b", 2]]

# Create key-value pairs
keys = ["name", "age", "city"]
values = ["Alice", 30, "NYC"]
pairs = collections.zip(keys, values)
```

**Time Complexity:** O(min(n, m)) where n and m are the list lengths

---

#### unzip(pairs)

Splits a list of pairs into two separate lists.

**Parameters:**
- `pairs` - List of two-element lists

**Returns:** List containing two sublists: [all_first_elements, all_second_elements]

**Examples:**
```graphoid
import "collections"

# Split pairs back into separate lists
pairs = [["Alice", 95], ["Bob", 87], ["Charlie", 92]]
result = collections.unzip(pairs)
# Result: [["Alice", "Bob", "Charlie"], [95, 87, 92]]

names = result[0]
scores = result[1]

# Reverse a zip operation
letters = ["a", "b", "c"]
numbers = [1, 2, 3]
zipped = collections.zip(letters, numbers)
unzipped = collections.unzip(zipped)
# unzipped == [letters, numbers]
```

**Time Complexity:** O(n) where n is the number of pairs

---

#### take(items, n)

Returns the first n elements of a list.

**Parameters:**
- `items` - Source list
- `n` - Number of elements to take

**Returns:** New list containing the first n elements. If n > list length, returns copy of entire list. If n <= 0, returns empty list.

**Examples:**
```graphoid
import "collections"

# Get first 3 elements
data = [1, 2, 3, 4, 5]
first_three = collections.take(data, 3)
# Result: [1, 2, 3]

# Request more than available
small = [1, 2]
result = collections.take(small, 5)
# Result: [1, 2]

# Pagination - first page
items = collections.range(1, 101)  # [1..100]
page_1 = collections.take(items, 10)  # [1..10]
```

**Time Complexity:** O(min(n, list length))

---

#### drop(items, n)

Returns all elements except the first n.

**Parameters:**
- `items` - Source list
- `n` - Number of elements to skip

**Returns:** New list with the first n elements removed. If n >= list length, returns empty list. If n <= 0, returns copy of entire list.

**Examples:**
```graphoid
import "collections"

# Skip first 2 elements
data = [1, 2, 3, 4, 5]
rest = collections.drop(data, 2)
# Result: [3, 4, 5]

# Pagination - skip first page
items = collections.range(1, 101)
page_2 = collections.take(collections.drop(items, 10), 10)
# Skip 10, then take 10 = items 11-20

# Remove header row
csv_data = [header_row, row1, row2, row3]
data_rows = collections.drop(csv_data, 1)
```

**Time Complexity:** O(n) where n is the size of the result

---

#### rotate(items, n)

Rotates list elements by n positions.

**Parameters:**
- `items` - Source list
- `n` - Number of positions to rotate (positive = right, negative = left)

**Returns:** New list with elements rotated. Rotation wraps around.

**Examples:**
```graphoid
import "collections"

# Rotate right (positive n)
data = [1, 2, 3, 4, 5]
rotated = collections.rotate(data, 2)
# Result: [4, 5, 1, 2, 3]

# Rotate left (negative n)
left_rotated = collections.rotate(data, -2)
# Result: [3, 4, 5, 1, 2]

# Rotate by list length = no change
full_rotation = collections.rotate(data, 5)
# Result: [1, 2, 3, 4, 5]

# Shift queue
queue = ["task1", "task2", "task3"]
rotated_queue = collections.rotate(queue, 1)
```

**Time Complexity:** O(n)

---

#### unique(items)

Removes duplicate values from a list, preserving the first occurrence of each value.

**Parameters:**
- `items` - Source list

**Returns:** New list with only unique values in the order they first appeared

**Examples:**
```graphoid
import "collections"

# Remove duplicates
numbers = [1, 2, 2, 3, 1, 4, 3, 5]
unique_nums = collections.unique(numbers)
# Result: [1, 2, 3, 4, 5]

# Deduplicate IDs
user_ids = [101, 102, 101, 103, 102, 104]
unique_users = collections.unique(user_ids)
# Result: [101, 102, 103, 104]

# Count unique values
tags = ["python", "web", "python", "tutorial", "web"]
unique_count = collections.unique(tags).length()
# unique_count = 3
```

**Time Complexity:** O(n²) - suitable for small to medium lists

---

#### reverse(items)

Reverses the order of elements in a list.

**Parameters:**
- `items` - Source list

**Returns:** New list with elements in reverse order

**Examples:**
```graphoid
import "collections"

# Reverse order
forward = [1, 2, 3, 4, 5]
backward = collections.reverse(forward)
# Result: [5, 4, 3, 2, 1]

# Reverse string characters
chars = ["H", "e", "l", "l", "o"]
reversed_chars = collections.reverse(chars)
# Result: ["o", "l", "l", "e", "H"]

# Process in reverse chronological order
events = get_recent_events()
oldest_first = collections.reverse(events)
```

**Time Complexity:** O(n)

---

#### interleave(list1, list2)

Interleaves elements from two lists, alternating between them.

**Parameters:**
- `list1` - First list
- `list2` - Second list

**Returns:** New list with alternating elements [list1[0], list2[0], list1[1], list2[1], ...]

**Examples:**
```graphoid
import "collections"

# Interleave two lists
numbers = [1, 2, 3]
letters = ["a", "b", "c"]
result = collections.interleave(numbers, letters)
# Result: [1, "a", 2, "b", 3, "c"]

# Different lengths
short = [1, 2]
long = ["a", "b", "c", "d"]
result = collections.interleave(short, long)
# Result: [1, "a", 2, "b", "c", "d"]

# Merge two streams
stream_a = [/* data */]
stream_b = [/* data */]
merged = collections.interleave(stream_a, stream_b)
```

**Time Complexity:** O(n + m) where n and m are the list lengths

---

#### repeat(value, n)

Creates a list by repeating a value n times.

**Parameters:**
- `value` - Value to repeat
- `n` - Number of times to repeat (must be >= 0)

**Returns:** New list containing `value` repeated `n` times

**Examples:**
```graphoid
import "collections"

# Create list of zeros
zeros = collections.repeat(0, 5)
# Result: [0, 0, 0, 0, 0]

# Initialize with default value
default_scores = collections.repeat(100, 3)
# Result: [100, 100, 100]

# Create separator pattern
separators = collections.repeat("-", 10)
# Result: ["-", "-", "-", "-", "-", "-", "-", "-", "-", "-"]

# Empty list (repeat 0 times)
empty = collections.repeat("x", 0)
# Result: []
```

**Time Complexity:** O(n)

---

#### range(start, end)

Creates a list of consecutive numbers from start to end (exclusive).

**Parameters:**
- `start` - Starting number (inclusive)
- `end` - Ending number (exclusive)

**Returns:** List of numbers [start, start+1, ..., end-1]

**Examples:**
```graphoid
import "collections"

# Basic range
numbers = collections.range(0, 5)
# Result: [0, 1, 2, 3, 4]

# Offset range
range_5_10 = collections.range(5, 10)
# Result: [5, 6, 7, 8, 9]

# Generate IDs
ids = collections.range(1, 101)  # 1 to 100

# Empty range (start == end)
empty = collections.range(5, 5)
# Result: []
```

**Time Complexity:** O(end - start)

---

#### range(start, end, step)

Creates a list of numbers from start to end with a specified step.

**Parameters:**
- `start` - Starting number (inclusive)
- `end` - Ending number (exclusive)
- `step` - Increment value (positive or negative, must not be 0)

**Returns:** List of numbers with specified step

**Examples:**
```graphoid
import "collections"

# Even numbers
evens = collections.range(0, 10, 2)
# Result: [0, 2, 4, 6, 8]

# Odd numbers
odds = collections.range(1, 10, 2)
# Result: [1, 3, 5, 7, 9]

# Countdown (negative step)
countdown = collections.range(10, 0, -1)
# Result: [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]

# Larger steps
multiples_of_5 = collections.range(0, 50, 5)
# Result: [0, 5, 10, 15, 20, 25, 30, 35, 40, 45]
```

**Time Complexity:** O(|end - start| / |step|)

---

### Map Utilities

#### merge(dict1, dict2)

Merges two maps into a new map. Values from dict2 overwrite values from dict1 for duplicate keys.

**Parameters:**
- `dict1` - First map
- `dict2` - Second map

**Returns:** New map containing all keys from both maps. If a key exists in both, dict2's value wins.

**Examples:**
```graphoid
import "collections"

# Merge configurations
defaults = {"host": "localhost", "port": 8080, "debug": false}
overrides = {"port": 3000, "debug": true}
config = collections.merge(defaults, overrides)
# Result: {"host": "localhost", "port": 3000, "debug": true}

# Combine data
user_info = {"name": "Alice", "age": 30}
user_prefs = {"theme": "dark", "language": "en"}
complete_profile = collections.merge(user_info, user_prefs)

# Later values override
map1 = {"a": 1, "b": 2}
map2 = {"b": 3, "c": 4}
merged = collections.merge(map1, map2)
# Result: {"a": 1, "b": 3, "c": 4}
```

**Time Complexity:** O(n + m) where n and m are the map sizes

---

#### get_keys(dict)

Extracts all keys from a map as a list.

**Parameters:**
- `dict` - Source map

**Returns:** List of all keys in the map

**Examples:**
```graphoid
import "collections"

# Get all keys
person = {"name": "Alice", "age": 30, "city": "NYC"}
keys = collections.get_keys(person)
# Result: ["name", "age", "city"] (order may vary)

# Iterate over keys
config = {"host": "localhost", "port": 8080}
for key in collections.get_keys(config) {
    value = config[key]
    print(key + ": " + value.to_string())
}

# Check key count
key_count = collections.get_keys(data).length()
```

**Note:** This is equivalent to calling `.keys()` on the map directly, but provides a consistent API.

**Time Complexity:** O(n) where n is the map size

---

#### get_values(dict)

Extracts all values from a map as a list.

**Parameters:**
- `dict` - Source map

**Returns:** List of all values in the map

**Examples:**
```graphoid
import "collections"

# Get all values
scores = {"Alice": 95, "Bob": 87, "Charlie": 92}
all_scores = collections.get_values(scores)
# Result: [95, 87, 92] (order may vary)

# Calculate statistics on values
import "statistics"
average_score = statistics.mean(collections.get_values(scores))

# Check if any value meets condition
values = collections.get_values(data)
has_negative = false
for v in values {
    if v < 0 {
        has_negative = true
    }
}
```

**Time Complexity:** O(n) where n is the map size

---

#### invert(dict)

Swaps keys and values in a map.

**Parameters:**
- `dict` - Source map

**Returns:** New map where original keys become values and original values become keys (converted to strings)

**Behavior:**
- Values are converted to strings to use as keys
- If multiple keys have the same value, the last one processed wins

**Examples:**
```graphoid
import "collections"

# Invert a lookup table
color_codes = {"red": 1, "green": 2, "blue": 3}
code_colors = collections.invert(color_codes)
# Result: {"1": "red", "2": "green", "3": "blue"}

# Reverse mapping
error_messages = {"E001": "Not found", "E002": "Invalid"}
message_codes = collections.invert(error_messages)
# Result: {"Not found": "E001", "Invalid": "E002"}

# Note: Duplicate values
grades = {"Alice": "A", "Bob": "A", "Charlie": "B"}
inverted = collections.invert(grades)
# Result: {"A": "Bob", "B": "Charlie"}  (Alice lost to Bob)
```

**Time Complexity:** O(n) where n is the map size

---

#### has_key(dict, key)

Checks if a map contains a specific key.

**Parameters:**
- `dict` - Source map
- `key` - Key to check for

**Returns:** `true` if the key exists in the map, `false` otherwise

**Examples:**
```graphoid
import "collections"

# Check for key existence
config = {"host": "localhost", "port": 8080}

if collections.has_key(config, "host") {
    print("Host is configured")
}

if collections.has_key(config, "timeout") == false {
    print("Timeout not set, using default")
}

# Conditional access
settings = get_user_settings()
theme = "light"  # default
if collections.has_key(settings, "theme") {
    theme = settings["theme"]
}
```

**Time Complexity:** O(n) where n is the number of keys

---

### Set Operations

#### union(list1, list2)

Returns all unique elements from both lists (set union).

**Parameters:**
- `list1` - First list
- `list2` - Second list

**Returns:** List containing all unique elements from both lists

**Examples:**
```graphoid
import "collections"

# Combine unique elements
set_a = [1, 2, 3, 4]
set_b = [3, 4, 5, 6]
all_elements = collections.union(set_a, set_b)
# Result: [1, 2, 3, 4, 5, 6]

# Merge tag lists
tags_post_a = ["python", "programming", "web"]
tags_post_b = ["javascript", "web", "tutorial"]
all_tags = collections.union(tags_post_a, tags_post_b)
# Result: ["python", "programming", "web", "javascript", "tutorial"]

# Combine user permissions
user_perms = ["read", "write"]
group_perms = ["write", "execute"]
total_perms = collections.union(user_perms, group_perms)
```

**Time Complexity:** O(n * m) due to duplicate checking

---

#### intersection(list1, list2)

Returns elements that appear in both lists (set intersection).

**Parameters:**
- `list1` - First list
- `list2` - Second list

**Returns:** List containing elements common to both lists (duplicates removed)

**Examples:**
```graphoid
import "collections"

# Find common elements
set_a = [1, 2, 3, 4]
set_b = [3, 4, 5, 6]
common = collections.intersection(set_a, set_b)
# Result: [3, 4]

# Find common tags
tags_a = ["python", "programming", "web", "tutorial"]
tags_b = ["programming", "web", "javascript", "tutorial"]
shared_tags = collections.intersection(tags_a, tags_b)
# Result: ["programming", "web", "tutorial"]

# Find users in both groups
group_a_users = [101, 102, 103, 104]
group_b_users = [103, 104, 105, 106]
users_in_both = collections.intersection(group_a_users, group_b_users)
```

**Time Complexity:** O(n * m) where n and m are the list lengths

---

#### difference(list1, list2)

Returns elements in list1 that are not in list2 (set difference).

**Parameters:**
- `list1` - First list
- `list2` - Second list

**Returns:** List containing elements from list1 that don't appear in list2

**Examples:**
```graphoid
import "collections"

# Find elements only in first set
set_a = [1, 2, 3, 4]
set_b = [3, 4, 5, 6]
only_in_a = collections.difference(set_a, set_b)
# Result: [1, 2]

# Find unique tags
all_tags = ["python", "web", "tutorial", "beginner"]
used_tags = ["web", "tutorial"]
unused_tags = collections.difference(all_tags, used_tags)
# Result: ["python", "beginner"]

# Find removed items
old_items = [1, 2, 3, 4, 5]
new_items = [2, 3, 4, 5, 6]
removed = collections.difference(old_items, new_items)
# Result: [1]
```

**Time Complexity:** O(n * m)

---

#### symmetric_difference(list1, list2)

Returns elements that appear in either list but not both (symmetric difference).

**Parameters:**
- `list1` - First list
- `list2` - Second list

**Returns:** List containing elements that appear in exactly one of the two lists

**Examples:**
```graphoid
import "collections"

# Find elements in either but not both
set_a = [1, 2, 3, 4]
set_b = [3, 4, 5, 6]
diff = collections.symmetric_difference(set_a, set_b)
# Result: [1, 2, 5, 6]

# Find changed items
old_features = ["login", "profile", "search"]
new_features = ["login", "search", "chat"]
changed = collections.symmetric_difference(old_features, new_features)
# Result: ["profile", "chat"]

# XOR-like operation
group_a = [101, 102, 103]
group_b = [102, 103, 104]
unique_to_one_group = collections.symmetric_difference(group_a, group_b)
# Result: [101, 104]
```

**Time Complexity:** O(n * m)

---

#### is_subset(list1, list2)

Checks if all elements of list1 are in list2.

**Parameters:**
- `list1` - Potential subset
- `list2` - Potential superset

**Returns:** `true` if list1 is a subset of list2, `false` otherwise

**Examples:**
```graphoid
import "collections"

# Check subset relationship
small = [1, 2]
large = [1, 2, 3, 4]
is_sub = collections.is_subset(small, large)
# Result: true

# Verify permissions
required_perms = ["read", "write"]
user_perms = ["read", "write", "execute"]
has_all_required = collections.is_subset(required_perms, user_perms)
# Result: true

# Check if all tags present
required_tags = ["python", "tutorial"]
post_tags = ["python", "web", "tutorial", "beginner"]
has_required = collections.is_subset(required_tags, post_tags)
# Result: true

# Not a subset
subset = [1, 5]
set = [1, 2, 3, 4]
collections.is_subset(subset, set)  # false (5 not in set)
```

**Time Complexity:** O(n * m) where n is subset size, m is superset size

---

#### is_superset(list1, list2)

Checks if list1 contains all elements of list2.

**Parameters:**
- `list1` - Potential superset
- `list2` - Potential subset

**Returns:** `true` if list1 is a superset of list2, `false` otherwise

**Examples:**
```graphoid
import "collections"

# Check superset relationship
large = [1, 2, 3, 4]
small = [1, 2]
is_super = collections.is_superset(large, small)
# Result: true

# Equivalent to reversed is_subset
collections.is_superset(large, small) == collections.is_subset(small, large)
# true

# Verify capabilities
admin_perms = ["read", "write", "delete", "admin"]
user_perms = ["read", "write"]
admin_has_all = collections.is_superset(admin_perms, user_perms)
# Result: true
```

**Time Complexity:** O(n * m)

---

## Common Use Cases

### 1. Batch Processing
```graphoid
import "collections"

# Process data in batches to avoid overwhelming system
records = load_records()  # 10,000 records
batch_size = 100

batches = collections.chunk(records, batch_size)
for batch in batches {
    process_batch(batch)
    save_results(batch)
}
```

### 2. Pagination
```graphoid
import "collections"

# Implement pagination
all_items = get_all_items()  # 1000 items
page_size = 20
page_number = 3

# Skip first (page_number - 1) * page_size items
skip_count = (page_number - 1) * page_size
page_items = collections.take(collections.drop(all_items, skip_count), page_size)

# Or helper function
fn paginate(items, page_num, size) {
    skip = (page_num - 1) * size
    return collections.take(collections.drop(items, skip), size)
}
```

### 3. Data Deduplication
```graphoid
import "collections"

# Remove duplicate user IDs
user_actions = [
    {"user_id": 101, "action": "click"},
    {"user_id": 102, "action": "view"},
    {"user_id": 101, "action": "purchase"},
    {"user_id": 103, "action": "click"}
]

# Extract and deduplicate IDs
user_ids = []
for action in user_actions {
    user_ids = user_ids.append(action["user_id"])
}
unique_users = collections.unique(user_ids)
# Result: [101, 102, 103]

print("Total actions: " + user_actions.length().to_string())
print("Unique users: " + unique_users.length().to_string())
```

### 4. Tag Analysis
```graphoid
import "collections"

# Find common and unique tags between posts
post_a_tags = ["python", "programming", "web", "tutorial"]
post_b_tags = ["programming", "web", "javascript", "tutorial"]

# Common tags (for suggesting related content)
common_tags = collections.intersection(post_a_tags, post_b_tags)
print("Common: " + common_tags.to_string())

# All unique tags (for search index)
all_tags = collections.union(post_a_tags, post_b_tags)
print("All tags: " + all_tags.to_string())

# Tags unique to post A
unique_to_a = collections.difference(post_a_tags, post_b_tags)
print("Only in post A: " + unique_to_a.to_string())
```

### 5. Configuration Management
```graphoid
import "collections"

# Merge configurations with precedence
system_defaults = {
    "host": "localhost",
    "port": 8080,
    "debug": false,
    "timeout": 30
}

user_config = {
    "port": 3000,
    "debug": true
}

env_overrides = {
    "host": "production.example.com",
    "timeout": 60
}

# Apply in order: defaults < user < environment
config = collections.merge(system_defaults, user_config)
final_config = collections.merge(config, env_overrides)

# Result: {
#   "host": "production.example.com",
#   "port": 3000,
#   "debug": true,
#   "timeout": 60
# }
```

### 6. Data Transformation
```graphoid
import "collections"

# Transform parallel arrays into structured data
names = ["Alice", "Bob", "Charlie"]
ages = [30, 25, 35]
cities = ["NYC", "LA", "Chicago"]

people = []
zipped = collections.zip(names, collections.zip(ages, cities))
for pair in zipped {
    name = pair[0]
    age_city = pair[1]
    person = {
        "name": name,
        "age": age_city[0],
        "city": age_city[1]
    }
    people = people.append(person)
}
```

### 7. Circular Buffer / Queue Rotation
```graphoid
import "collections"

# Rotate tasks in a queue
task_queue = ["task1", "task2", "task3", "task4"]

# Move first task to end (like round-robin scheduling)
rotated = collections.rotate(task_queue, 1)
# Before: ["task1", "task2", "task3", "task4"]
# After:  ["task4", "task1", "task2", "task3"]

# Process and rotate
current_task = task_queue[0]
process_task(current_task)
task_queue = collections.rotate(task_queue, 1)
```

### 8. Permission Verification
```graphoid
import "collections"

# Check if user has required permissions
required_permissions = ["read", "write", "delete"]
user_permissions = ["read", "write", "execute", "admin"]

if collections.is_subset(required_permissions, user_permissions) {
    print("User has all required permissions")
    perform_sensitive_operation()
} else {
    missing = collections.difference(required_permissions, user_permissions)
    print("Missing permissions: " + missing.to_string())
}
```

---

## Edge Cases and Behavior

### Empty Lists
```graphoid
import "collections"

empty = []

collections.chunk(empty, 2)           # []
collections.unique(empty)             # []
collections.reverse(empty)            # []
collections.rotate(empty, 5)          # []
collections.union(empty, [1, 2])      # [1, 2]
collections.intersection(empty, [1])   # []
```

### Single Element
```graphoid
import "collections"

single = [42]

collections.chunk(single, 2)          # [[42]]
collections.reverse(single)           # [42]
collections.rotate(single, 10)        # [42]
collections.unique(single)            # [42]
```

### Negative or Zero Parameters
```graphoid
import "collections"

data = [1, 2, 3]

collections.take(data, 0)             # []
collections.take(data, -1)            # []
collections.drop(data, 0)             # [1, 2, 3]
collections.repeat("x", 0)            # []
collections.range(5, 5)               # []
```

### Mismatched List Sizes
```graphoid
import "collections"

short = [1, 2]
long = [10, 20, 30, 40]

collections.zip(short, long)          # [[1, 10], [2, 20]]
collections.interleave(short, long)   # [1, 10, 2, 20, 30, 40]
```

---

## Performance Characteristics

| Function | Time Complexity | Space Complexity | Notes |
|----------|----------------|------------------|-------|
| `chunk()` | O(n) | O(n) | Creates new sublists |
| `partition()` | O(n) | O(n) | Single pass |
| `zip()` | O(min(n,m)) | O(min(n,m)) | Limited by shorter list |
| `unzip()` | O(n) | O(n) | - |
| `take()` | O(min(n,k)) | O(min(n,k)) | k = number to take |
| `drop()` | O(n-k) | O(n-k) | k = number to drop |
| `rotate()` | O(n) | O(n) | Creates new list |
| `unique()` | O(n²) | O(n) | Suitable for small/medium lists |
| `reverse()` | O(n) | O(n) | Creates new list |
| `interleave()` | O(n+m) | O(n+m) | - |
| `repeat()` | O(n) | O(n) | n = repeat count |
| `range()` | O(|end-start|/step) | O(|end-start|/step) | - |
| `merge()` | O(n+m) | O(n+m) | Combines both maps |
| `get_keys()` | O(n) | O(n) | - |
| `get_values()` | O(n) | O(n) | - |
| `invert()` | O(n) | O(n) | - |
| `has_key()` | O(n) | O(1) | Linear search |
| `union()` | O(n*m) | O(n+m) | Due to duplicate checking |
| `intersection()` | O(n*m) | O(n) | Nested iteration |
| `difference()` | O(n*m) | O(n) | Nested iteration |
| `symmetric_difference()` | O(n*m) | O(n+m) | Nested iteration |
| `is_subset()` | O(n*m) | O(1) | Early termination possible |
| `is_superset()` | O(n*m) | O(1) | Calls is_subset |

**Notes:**
- Set operations (union, intersection, etc.) use O(n*m) time due to duplicate checking
- For very large datasets, consider specialized data structures
- Most functions create new collections (immutable pattern)

---

## Tips and Best Practices

### 1. Chain Operations
```graphoid
import "collections"

# Good: Chain operations for readability
data = [1, 2, 2, 3, 4, 4, 5, 6, 7, 8, 9]
result = collections.take(
    collections.unique(data),
    5
)

# Better: Use intermediate variables for complex chains
unique_data = collections.unique(data)
top_five = collections.take(unique_data, 5)
```

### 2. Prefer Specific Functions
```graphoid
import "collections"

# Good: Use chunk for batching
collections.chunk(data, 100)

# Avoid: Manual batching
batches = []
i = 0
while i < data.length() {
    # Complex manual logic...
}
```

### 3. Understand Set Operation Semantics
```graphoid
import "collections"

# Union: All unique elements
union = collections.union([1, 2], [2, 3])  # [1, 2, 3]

# Intersection: Only common elements
intersection = collections.intersection([1, 2], [2, 3])  # [2]

# Difference: Elements in first but not second
diff = collections.difference([1, 2], [2, 3])  # [1]

# Symmetric difference: Elements in either but not both
sym_diff = collections.symmetric_difference([1, 2], [2, 3])  # [1, 3]
```

### 4. Use has_key for Safe Access
```graphoid
import "collections"

# Good: Check before accessing
if collections.has_key(config, "timeout") {
    timeout = config["timeout"]
} else {
    timeout = 30  # default
}

# Or use pattern with default
timeout = 30
if collections.has_key(config, "timeout") {
    timeout = config["timeout"]
}
```

### 5. Pagination Pattern
```graphoid
import "collections"

# Reusable pagination function
fn get_page(items, page_number, page_size) {
    skip = (page_number - 1) * page_size
    return collections.take(
        collections.drop(items, skip),
        page_size
    )
}

# Use it
page_1 = get_page(all_items, 1, 20)
page_2 = get_page(all_items, 2, 20)
```

---

## API Reference Summary

### List Utilities (15 functions)

| Function | Parameters | Description |
|----------|------------|-------------|
| `chunk(items, size)` | 2 | Split into chunks |
| `partition(items, predicate)` | 2 | Split by predicate |
| `zip(list1, list2)` | 2 | Combine into pairs |
| `unzip(pairs)` | 1 | Split pairs |
| `take(items, n)` | 2 | First n elements |
| `drop(items, n)` | 2 | All except first n |
| `rotate(items, n)` | 2 | Rotate elements |
| `unique(items)` | 1 | Remove duplicates |
| `reverse(items)` | 1 | Reverse order |
| `interleave(list1, list2)` | 2 | Alternate elements |
| `repeat(value, n)` | 2 | Repeat value |
| `range(start, end)` | 2 | Number sequence |
| `range(start, end, step)` | 3 | Number sequence with step |
| `flatten(items)` | 1 | Flatten one level |

### Map Utilities (5 functions)

| Function | Parameters | Description |
|----------|------------|-------------|
| `merge(dict1, dict2)` | 2 | Merge maps |
| `get_keys(dict)` | 1 | Extract keys |
| `get_values(dict)` | 1 | Extract values |
| `invert(dict)` | 1 | Swap keys/values |
| `has_key(dict, key)` | 2 | Check key existence |

### Set Operations (6 functions)

| Function | Parameters | Description |
|----------|------------|-------------|
| `union(list1, list2)` | 2 | All unique elements |
| `intersection(list1, list2)` | 2 | Common elements |
| `difference(list1, list2)` | 2 | Elements only in first |
| `symmetric_difference(list1, list2)` | 2 | Elements in either but not both |
| `is_subset(list1, list2)` | 2 | Check subset |
| `is_superset(list1, list2)` | 2 | Check superset |

---

## See Also

- **statistics module** - For analyzing numeric collections
- **functional module** (planned) - Higher-order functions like map, filter, reduce
- **List methods** - Built-in `.append()`, `.length()`, `.first()`, `.last()`
- **Map methods** - Built-in `.keys()`, `.values()`, `.has_key()`

---

## Implementation Note

The `collections` module is implemented entirely in pure Graphoid (not Rust), demonstrating the language's capability for self-implementation and complex data manipulation. You can view the source at `stdlib/collections.gr`.

**Total:** 30+ functions in 580+ lines of pure Graphoid code, showcasing the language's expressiveness for real-world collection operations.
