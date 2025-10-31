# Phase 11: Pure Graphoid Standard Library - Detailed Implementation Plan

**Duration**: 10-14 days
**Status**: Not started
**Goal**: Implement high-level standard library modules in pure Graphoid (.gr files)

---

## Overview

Phase 11 implements standard library modules written entirely in Graphoid. These modules benefit from pattern matching, behaviors, configuration blocks, and other high-level language features. Implementing them in .gr files serves as **dogfooding** - validating that Graphoid is expressive enough to build real libraries.

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
}, json.stringify({"key": "value"}))

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

---

## Testing Strategy

**Each module must have**:
- 20-30 unit tests
- Integration tests with other modules
- Error case coverage
- Edge case handling

**Total Tests**: 150-200 tests for Phase 11

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

- [ ] ✅ All 7 modules implemented in .gr files
- [ ] ✅ 150+ tests passing
- [ ] ✅ Modules work together (CSV + Stats, HTTP + JSON)
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
