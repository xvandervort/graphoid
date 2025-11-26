# Phase 12: Pure Graphoid Standard Library - Detailed Implementation Plan

**Duration**: 14-18 days
**Status**: Not started
**Goal**: Implement high-level standard library modules in pure Graphoid (.gr files)

---

## Overview

Phase 12 implements standard library modules written entirely in Graphoid. These modules benefit from pattern matching, behaviors, configuration blocks, and other high-level language features. Implementing them in .gr files serves as **dogfooding** - validating that Graphoid is expressive enough to build real libraries.

**Why Pure Graphoid**:
- Demonstrates language expressiveness
- Benefits from pattern matching and behaviors
- Easier to maintain and modify
- Serves as examples for users
- Can use try/catch, configure blocks, etc.

**All Prerequisites Complete**:
- ✅ Pattern matching (Phases 7 & 9)
- ✅ Behaviors (Phase 8)
- ✅ Module system (Phase 10)
- ✅ Try/catch error handling
- ✅ Configure blocks
- ✅ Precision contexts

---

## Module List

**Total Modules**: 9 (7 originally planned + 2 moved from Phase 10)

### 1. Statistics Module (stats)

**File**: `stdlib/statistics.gr`
**Duration**: 2 days
**From Language Spec**: §1813-1826

**API**:
```graphoid
import "statistics"  # alias: stats

data = [1, 2, 3, 4, 5]
mean = stats.mean(data)
median = stats.median(data)
mode = stats.mode(data)
std_dev = stats.std_dev(data)
variance = stats.variance(data)
```

**Functions to Implement**:
- `mean(list)` - Arithmetic mean
- `median(list)` - Middle value
- `mode(list)` - Most frequent value
- `std_dev(list)` - Standard deviation
- `variance(list)` - Variance
- `percentile(list, p)` - Pth percentile
- `quartiles(list)` - Q1, Q2, Q3
- `iqr(list)` - Interquartile range
- `correlation(list1, list2)` - Pearson correlation
- `covariance(list1, list2)` - Covariance

**Key Features**:
- Use `configure { none_handling: :skip }` for missing data
- Pattern matching for error cases
- Behaviors for data cleaning

**Tests**: 20+ tests covering all statistical functions

---

### 2. CSV Module

**File**: `stdlib/csv.gr`
**Duration**: 2 days
**From Language Spec**: §1783-1796

**API**:
```graphoid
import "csv"

# Parse CSV
rows = csv.parse(csv_text)
# Returns: [{"col1": "val1", "col2": "val2"}, ...]

# Generate CSV
csv_text = csv.generate(rows)

# Validate CSV
is_valid = csv.validate(csv_text)

# Parse with options
rows = csv.parse(csv_text, delimiter: ";", has_header: true)
```

**Functions to Implement**:
- `parse(text)` - Parse CSV to list of hashes
- `parse(text, options)` - Parse with custom delimiter, quotes
- `generate(rows)` - Convert list of hashes to CSV
- `generate(rows, options)` - Generate with custom options
- `validate(text)` - Check if valid CSV
- `count_rows(text)` - Count rows without parsing
- `get_headers(text)` - Extract header row

**Options**:
- `delimiter` - Column separator (default: `,`)
- `quote_char` - Quote character (default: `"`)
- `has_header` - First row is header (default: `true`)
- `skip_empty` - Skip empty rows (default: `true`)

**Key Features**:
- Pattern matching for parsing state machine
- Configure blocks for error handling (strict vs lenient)
- Behaviors for type coercion (string to number)

**Tests**: 25+ tests covering parsing, generation, edge cases

---

### 3. SQL Module

**File**: `stdlib/sql.gr`
**Duration**: 2 days
**From Language Spec**: §1798-1811

**API**:
```graphoid
import "sql"

# Query builder (fluent interface)
query = sql.Table("users")
    .select("name", "email", "age")
    .where("age", ">", 18)
    .where("active", "=", true)
    .sort("name")
    .limit(10)

# Generate SQL
sql_string = query.to_sql()
# "SELECT name, email, age FROM users WHERE age > 18 AND active = true ORDER BY name LIMIT 10"

# With joins
query = sql.Table("users")
    .join("orders", "users.id", "orders.user_id")
    .select("users.name", "orders.total")
    .where("orders.status", "=", "completed")
```

**Classes/Functions**:
- `Table(name)` - Create query builder
- `.select(fields...)` - Select specific fields
- `.where(field, op, value)` - Add WHERE condition
- `.join(table, key1, key2)` - Join tables
- `.left_join(table, key1, key2)` - Left outer join
- `.sort(field)` - ORDER BY
- `.sort(field, :desc)` - ORDER BY DESC
- `.limit(n)` - LIMIT clause
- `.offset(n)` - OFFSET clause
- `.to_sql()` - Generate SQL string

**Key Features**:
- Fluent/builder pattern (chaining)
- Pattern matching for operators
- SQL injection prevention (parameterized)

**Tests**: 30+ tests covering all query types

---

### 4. HTML Module

**File**: `stdlib/html.gr`
**Duration**: 2 days
**From Language Spec**: §1846-1852

**API**:
```graphoid
import "html"

# Parse HTML
doc = html.parse(html_text)

# Query DOM
links = doc.find_all("a")
first_link = doc.find("a")
divs = doc.find_all("div", class: "container")

# Attributes
href = link.get_attribute("href")
classes = div.get_attribute("class")

# Text content
text = element.text()

# Generate HTML
html_str = html.generate(element)
```

**Functions**:
- `parse(html_text)` - Parse HTML to DOM tree
- `.find(selector)` - Find first matching element
- `.find_all(selector)` - Find all matching elements
- `.get_attribute(name)` - Get attribute value
- `.set_attribute(name, value)` - Set attribute
- `.text()` - Get text content
- `.children()` - Get child elements
- `generate(element)` - Convert DOM to HTML string

**Key Features**:
- DOM represented as graph structure (nodes and edges)
- Pattern matching for element queries
- Behaviors for attribute normalization

**Tests**: 25+ tests covering parsing and querying

---

### 5. HTTP Module

**File**: `stdlib/http.gr`
**Duration**: 2 days
**From Language Spec**: §1830-1844

**API**:
```graphoid
import "http"
import "json"

# HTTP GET
response = http.get("https://api.example.com/data")
if response["status"] == 200 {
    data = json.parse(response["body"])
}

# HTTP POST
response = http.post("https://api.example.com/submit", {
    "Content-Type": "application/json"
}, json.to_string({"key": "value"}))

# With options
response = http.get("https://example.com", {
    timeout: 30,
    follow_redirects: true,
    headers: {"User-Agent": "Graphoid/1.0"}
})
```

**Functions**:
- `get(url)` - HTTP GET request
- `get(url, options)` - GET with options
- `post(url, headers, body)` - HTTP POST
- `put(url, headers, body)` - HTTP PUT
- `delete(url)` - HTTP DELETE
- `head(url)` - HTTP HEAD
- `request(method, url, options)` - Generic request

**Response Format**:
```graphoid
{
    "status": 200,
    "headers": {...},
    "body": "...",
    "ok": true
}
```

**Options**:
- `timeout` - Request timeout in seconds (default: 30)
- `follow_redirects` - Follow 3xx redirects (default: true)
- `max_redirects` - Max redirect count (default: 10)
- `headers` - Custom headers hash
- `verify_ssl` - Verify SSL certificates (default: true)

**Key Features**:
- Pattern matching for response handling
- Try/catch for network errors
- Configure blocks for retry logic

**Tests**: 20+ tests (requires mocking/test server)

---

### 6. Pretty-Print Module (pp)

**File**: `stdlib/pretty_print.gr`
**Duration**: 1-2 days
**New Module** (not in original spec)

**API**:
```graphoid
import "pretty_print"  # alias: pp

# Pretty-print any value
pp.print(value)

# Customized output
pp.print(value, indent: 4, color: true, depth: 3)

# Get formatted string
formatted = pp.format(value)

# Inspect with type information
pp.inspect(value)  # Includes types and sizes
```

**Functions**:
- `print(value)` - Pretty-print to stdout
- `print(value, options)` - Print with options
- `format(value)` - Return formatted string
- `inspect(value)` - Detailed inspection with types

**Options**:
- `indent` - Indentation spaces (default: 2)
- `color` - Use ANSI colors (default: false)
- `depth` - Max nesting depth (default: 5)
- `width` - Max line width (default: 80)
- `compact` - Compact arrays/lists (default: false)

**Output Examples**:
```graphoid
# List
[
  1,
  2,
  3
]

# Hash
{
  "name": "Alice",
  "age": 30,
  "active": true
}

# Graph
graph {
  nodes: 5,
  edges: 8,
  type: :dag
}
```

**Key Features**:
- Pattern matching for different value types
- Recursive formatting with depth control
- ANSI color codes for terminal output

**Tests**: 20+ tests covering all data types

---

### 7. Option Parser Module (optparse)

**File**: `stdlib/option_parser.gr`
**Duration**: 2 days
**New Module** (not in original spec)

**API**:
```graphoid
import "option_parser"  # alias: optparse

# Define options
parser = optparse.Parser()
parser.add_option("-v", "--verbose", type: :bool, help: "Verbose output")
parser.add_option("-f", "--file", type: :string, required: true, help: "Input file")
parser.add_option("-n", "--count", type: :int, default: 10, help: "Number of items")

# Parse arguments
args = ["--verbose", "--file", "data.txt", "--count", "20"]
options = parser.parse(args)

# Access parsed values
verbose = options["verbose"]  # true
file = options["file"]        # "data.txt"
count = options["count"]      # 20

# Get remaining args
remaining = parser.remaining_args()
```

**Classes/Functions**:
- `Parser()` - Create option parser
- `.add_option(short, long, options)` - Define option
- `.add_flag(short, long, help)` - Boolean flag
- `.add_argument(name, options)` - Positional argument
- `.parse(args)` - Parse argument list
- `.usage()` - Generate usage string
- `.help()` - Generate help message

**Option Types**:
- `:bool` - Boolean flag
- `:string` - String value
- `:int` - Integer value
- `:float` - Float value
- `:list` - Multiple values (repeatable option)

**Option Properties**:
- `required` - Must be present (default: false)
- `default` - Default value
- `help` - Help text
- `choices` - List of valid values
- `action` - Action to take (:store, :append, :count)

**Error Handling**:
```graphoid
try {
    options = parser.parse(args)
} catch OptionParserError as e {
    print(parser.usage())
    print("Error: " + e.message())
}
```

**Key Features**:
- Pattern matching for argument parsing
- Automatic help generation
- Type validation and conversion
- Unix-style options (short and long forms)

**Tests**: 30+ tests covering all option types and edge cases

---

### 8. String Module

**File**: `stdlib/string.gr`
**Duration**: 1-2 days
**Moved from Phase 10** - String manipulation utilities

**API**:
```graphoid
import "string"

# Padding
left_padded = string.pad_left("hello", 10, " ")      # "     hello"
right_padded = string.pad_right("hello", 10, " ")    # "hello     "
centered = string.center("hello", 10, " ")           # "  hello   "

# Repetition
repeated = string.repeat("abc", 3)                   # "abcabcabc"
repeated_char = string.repeat("*", 10)               # "**********"

# Joining
joined = string.join(["a", "b", "c"], ", ")          # "a, b, c"
path = string.join_paths(["/home", "user", "file"])  # "/home/user/file"

# Splitting
lines = string.lines("line1\nline2\nline3")          # ["line1", "line2", "line3"]
words = string.words("hello world test")             # ["hello", "world", "test"]
chunks = string.chunks("abcdef", 2)                  # ["ab", "cd", "ef"]

# Case conversion
title = string.title_case("hello world")             # "Hello World"
camel = string.camel_case("hello_world")             # "helloWorld"
snake = string.snake_case("HelloWorld")              # "hello_world"
kebab = string.kebab_case("HelloWorld")              # "hello-world"

# Trimming
trimmed = string.trim("  hello  ")                   # "hello"
left_trimmed = string.trim_left("  hello  ")         # "hello  "
right_trimmed = string.trim_right("  hello  ")       # "  hello"

# Character operations
reversed = string.reverse("hello")                   # "olleh"
char_at = string.char_at("hello", 1)                 # "e"
index = string.index_of("hello", "l")                # 2 (first occurrence)
last_index = string.last_index_of("hello", "l")      # 3

# Validation
is_alpha = string.is_alpha("hello")                  # true
is_digit = string.is_digit("12345")                  # true
is_alnum = string.is_alnum("hello123")               # true
is_space = string.is_space("   ")                    # true

# Truncation
truncated = string.truncate("hello world", 8)        # "hello..."
truncated_custom = string.truncate("hello world", 8, ">>")  # "hello>>"
```

**Functions**:
- `pad_left(str, width, fill)` - Left-pad string
- `pad_right(str, width, fill)` - Right-pad string
- `center(str, width, fill)` - Center string with padding
- `repeat(str, count)` - Repeat string n times
- `join(list, separator)` - Join list of strings
- `join_paths(list)` - Join path components (platform-aware)
- `lines(str)` - Split by newlines
- `words(str)` - Split by whitespace
- `chunks(str, size)` - Split into fixed-size chunks
- `title_case(str)` - Convert to Title Case
- `camel_case(str)` - Convert to camelCase
- `snake_case(str)` - Convert to snake_case
- `kebab_case(str)` - Convert to kebab-case
- `trim(str)` - Trim whitespace from both ends
- `trim_left(str)` - Trim left whitespace
- `trim_right(str)` - Trim right whitespace
- `reverse(str)` - Reverse string
- `char_at(str, index)` - Get character at index
- `index_of(str, substr)` - Find first occurrence
- `last_index_of(str, substr)` - Find last occurrence
- `is_alpha(str)` - Check if all alphabetic
- `is_digit(str)` - Check if all digits
- `is_alnum(str)` - Check if alphanumeric
- `is_space(str)` - Check if all whitespace
- `truncate(str, max_length)` - Truncate with "..."
- `truncate(str, max_length, suffix)` - Truncate with custom suffix

**Key Features**:
- Pure Graphoid implementation using pattern matching
- Handles Unicode correctly
- Platform-aware path joining
- No dependencies on native modules

**Tests**: 30+ tests covering all string operations

---

### 9. List Module

**File**: `stdlib/list.gr`
**Duration**: 1-2 days
**Moved from Phase 10** - Advanced list utilities

**API**:
```graphoid
import "list"

# Flattening
nested = [[1, 2], [3, 4], [5, 6]]
flat = list.flatten(nested)                          # [1, 2, 3, 4, 5, 6]

deep_nested = [1, [2, [3, [4]]]]
flat_deep = list.flatten_deep(deep_nested)           # [1, 2, 3, 4]

# Zipping
names = ["Alice", "Bob", "Charlie"]
ages = [25, 30, 35]
zipped = list.zip(names, ages)                       # [["Alice", 25], ["Bob", 30], ["Charlie", 35]]

# Multiple lists
list1 = [1, 2, 3]
list2 = [10, 20, 30]
list3 = [100, 200, 300]
zipped_multi = list.zip_multi([list1, list2, list3]) # [[1, 10, 100], [2, 20, 200], [3, 30, 300]]

# Unzipping
pairs = [["Alice", 25], ["Bob", 30]]
[names, ages] = list.unzip(pairs)                    # names = ["Alice", "Bob"], ages = [25, 30]

# Range generation
range1 = list.range(5)                               # [0, 1, 2, 3, 4]
range2 = list.range(1, 6)                            # [1, 2, 3, 4, 5]
range3 = list.range(0, 10, 2)                        # [0, 2, 4, 6, 8]
range4 = list.range(10, 0, -2)                       # [10, 8, 6, 4, 2]

# Repetition
repeated = list.repeat([1, 2, 3], 3)                 # [1, 2, 3, 1, 2, 3, 1, 2, 3]
repeated_val = list.repeat_value(0, 5)               # [0, 0, 0, 0, 0]

# Partitioning
numbers = [1, 2, 3, 4, 5, 6]
[evens, odds] = list.partition(numbers, n => n % 2 == 0)  # evens = [2, 4, 6], odds = [1, 3, 5]

# Chunking
items = [1, 2, 3, 4, 5, 6, 7, 8]
chunks = list.chunk(items, 3)                        # [[1, 2, 3], [4, 5, 6], [7, 8]]

# Windowing
nums = [1, 2, 3, 4, 5]
windows = list.window(nums, 3)                       # [[1, 2, 3], [2, 3, 4], [3, 4, 5]]

# Unique values
duplicates = [1, 2, 2, 3, 3, 3, 4]
unique = list.unique(duplicates)                     # [1, 2, 3, 4]

# Frequency counting
values = ["a", "b", "a", "c", "b", "a"]
freq = list.frequencies(values)                      # {"a": 3, "b": 2, "c": 1}

# Sorting with key
people = [{"name": "Bob", "age": 30}, {"name": "Alice", "age": 25}]
sorted_by_age = list.sort_by(people, p => p["age"])  # Sorted by age

# Grouping
nums = [1, 2, 3, 4, 5, 6]
grouped = list.group_by(nums, n => n % 3)            # {0: [3, 6], 1: [1, 4], 2: [2, 5]}

# Intersperse
items = ["a", "b", "c"]
with_sep = list.intersperse(items, "|")              # ["a", "|", "b", "|", "c"]

# Take/Drop
items = [1, 2, 3, 4, 5]
first_3 = list.take(items, 3)                        # [1, 2, 3]
last_3 = list.take_last(items, 3)                    # [3, 4, 5]
drop_2 = list.drop(items, 2)                         # [3, 4, 5]
drop_last_2 = list.drop_last(items, 2)               # [1, 2, 3]
```

**Functions**:
- `flatten(list)` - Flatten one level
- `flatten_deep(list)` - Recursively flatten all levels
- `zip(list1, list2)` - Zip two lists
- `zip_multi(lists)` - Zip multiple lists
- `unzip(pairs)` - Unzip pairs into separate lists
- `range(end)` - Range from 0 to end-1
- `range(start, end)` - Range from start to end-1
- `range(start, end, step)` - Range with step
- `repeat(list, count)` - Repeat list n times
- `repeat_value(value, count)` - Repeat value n times
- `partition(list, predicate)` - Split by predicate
- `chunk(list, size)` - Split into fixed-size chunks
- `window(list, size)` - Sliding window
- `unique(list)` - Remove duplicates
- `frequencies(list)` - Count occurrences
- `sort_by(list, key_fn)` - Sort by key function
- `group_by(list, key_fn)` - Group by key function
- `intersperse(list, separator)` - Insert separator between elements
- `take(list, n)` - Take first n elements
- `take_last(list, n)` - Take last n elements
- `drop(list, n)` - Drop first n elements
- `drop_last(list, n)` - Drop last n elements

**Key Features**:
- Pure Graphoid implementation
- Leverages pattern matching and lambdas
- Works with Graphoid's built-in list operations
- Functional programming patterns

**Tests**: 35+ tests covering all list operations

---

## Implementation Strategy

### Day 1-2: Statistics Module
- Implement mean, median, mode
- Standard deviation and variance
- Percentiles and quartiles
- Handle missing data with configure blocks

### Day 3-4: CSV Module
- CSV parser with state machine
- CSV generator
- Validator
- Options handling

### Day 5-6: SQL Module
- Query builder class
- Fluent interface (method chaining)
- WHERE, JOIN, ORDER BY, LIMIT
- SQL generation

### Day 7-8: HTML Module
- HTML parser (basic subset)
- DOM tree structure
- Element queries
- Attribute access

### Day 9-10: HTTP Module
- HTTP request functions
- Response handling
- Error handling with try/catch
- Options processing

### Day 11-12: Pretty-Print Module
- Value formatting
- Recursive pretty-printing
- Color output
- Inspection mode

### Day 13-14: Option Parser Module
- Option definition
- Argument parsing
- Help generation
- Type validation

### Day 15-16: String Module
- Padding, trimming, case conversion
- Joining, splitting, chunking
- Character operations
- Validation functions

### Day 17-18: List Module
- Flattening, zipping, unzipping
- Range generation, repetition
- Partitioning, chunking, windowing
- Grouping, sorting, unique operations

---

## Testing Strategy

**Each module must have**:
- 20-35 unit tests per module
- Integration tests with other modules
- Error case coverage
- Edge case handling

**Total Tests**: 220-260 tests for Phase 11

---

## Documentation Requirements

**For each module**:
- API reference with examples
- Usage patterns
- Error handling guide
- Integration examples

**File**: `stdlib/README.md` - Overview of all stdlib modules

---

## Success Criteria

- [ ] ✅ All 9 modules implemented in .gr files
- [ ] ✅ 220+ tests passing
- [ ] ✅ Modules work together (CSV + Stats, HTTP + JSON, List + String)
- [ ] ✅ Error handling robust (try/catch in modules)
- [ ] ✅ Pattern matching used effectively
- [ ] ✅ Behaviors demonstrated where appropriate
- [ ] ✅ Documentation complete
- [ ] ✅ REPL works with all modules
- [ ] ✅ Zero compiler warnings

---

## Dependencies

**Requires**:
- Phase 10 (Module System) complete
- Native stdlib modules (Phase 12) for:
  - JSON parsing (needed by HTTP module)
  - Regex (needed by HTML/CSV parsing)
  - I/O (for file operations in CSV)

**Note**: Some modules may have partial implementations in Phase 11 and be enhanced in Phase 12 after native dependencies are available.

---

## Dogfooding Benefits

Implementing stdlib in Graphoid proves:
- ✅ Pattern matching is practical
- ✅ Behaviors work for real use cases
- ✅ Module system enables code organization
- ✅ Try/catch handles errors gracefully
- ✅ Configure blocks provide flexible configuration
- ✅ Language is expressive enough for real libraries

---

## References

- **Language Specification**: §1556-1852 "Standard Library"
- **Testing**: Use patterns from Phase 13 testing framework
- **Module System**: Phase 10 provides import/export mechanism
