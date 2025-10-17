# Graphoid/Glang: Canonical Language Specification

**Version**: 1.0
**Last Updated**: January 2025
**Status**: Definitive specification for fresh Rust implementation

---

## Core Philosophy: Everything is a Graph

Graphoid is built on the radical principle that **every aspect of computation can and should be represented as a graph**. This isn't just about using graphs as data structures—it's about making graphs the fundamental abstraction at every level.

### The Three Levels of Graph Abstraction

#### Level 1: Data Structures as Graphs
- A list `[1, 2, 3]` is internally `Node(1) → Node(2) → Node(3)`
- A tree is a graph with hierarchical constraints
- Hash tables are graphs with key-value edges
- No artificial boundaries between "graph" and "non-graph" data

#### Level 2: Variable Storage as Graphs (Meta-Graph)
- Variables are nodes in a meta-graph
- Variable names are name nodes: `📛 "fruits"`
- Variable values are value nodes: `📊 LinearGraph([apple, banana])`
- Assignment creates edges: `📛 "fruits" → 📊 LinearGraph`
- The namespace itself IS a graph that can be inspected and manipulated

#### Level 3: Runtime Environment as Graphs (Future)
- Function calls as graph traversals
- Functions are nodes with parameter and return edges
- Call stack as path through function graph
- Modules as subgraphs with import/export edges
- Recursion as cycles in the call graph

---

## Type System

### Primitive Types

#### Numbers (`num`)
- Floating-point by default (f64)
- Display precision context: `precision N { ... }` controls decimal places
- Integer display when fractional part is zero
- Methods:
  - `sqrt()` - Square root
  - `pow(n)` - Raise to power
  - `abs()` - Absolute value
  - `up()` - Ceiling (optionally with decimal places: `up(2)`)
  - `down()` - Floor (optionally with decimal places: `down(2)`)
  - `round()` - Round to nearest (optionally with decimal places: `round(2)`)
  - `log()` - Natural logarithm
  - `log(base)` - Logarithm with specified base

#### Strings (`string`)
- Internally represented as character node graphs
- Unicode-aware grapheme cluster support
- Methods:
  - Case: `upper()`, `lower()`
  - Manipulation: `trim()`, `reverse()`, `substring(start, end)`
  - Splitting: `split(delimiter)`, `split(regex)`
  - Pattern matching: `starts_with(prefix)`, `ends_with(suffix)`
  - Unified semantic methods:
    - `contains(mode, patterns...)` - Check for patterns
    - `extract(pattern)` - Extract matching content
    - `count(pattern)` - Count occurrences
    - `find_first(pattern)` - Find position of first match
  - Regex support: `replace(regex, replacement)`, `find_all(regex)`

**Pattern Types**: `:digits`/`:numbers`, `:letters`, `:uppercase`, `:lowercase`, `:spaces`/`:whitespace`, `:punctuation`, `:symbols`, `:alphanumeric`, `:words`, `:emails`

#### Booleans (`bool`)
- Values: `true`, `false`
- Methods: `to_num()`, `to_string()`
- Truthiness rules:
  - `true`/`false` - obvious
  - Numbers: `0` is false, non-zero is true
  - Strings: empty string is false, non-empty is true
  - Collections: empty is false, non-empty is true

#### Symbols (`symbol`)
- Immutable named constants like `:ok`, `:error`, `:pending`
- Syntax: `:name`
- Interned for efficiency (one instance per unique name)
- Perfect for status codes, flags, pattern matching

#### None (`none`)
- Represents absence of value
- Configurable conversion behavior via configuration context
- Safe propagation through operations
- Detection methods: `is_none()`, `is_some()`

#### Time (`time`)
- Internal representation: UTC timestamp (f64)
- Display: ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`)
- Creation:
  - `time.now()` - Current time
  - `time.today()` - Start of today (00:00:00 UTC)
  - `time.from_components(year, month, day, hour?, minute?, second?)`
  - `time.from_string(iso_string)`
  - `time.from_timestamp(number)`
- Methods:
  - `to_string()` - ISO format
  - `to_num()` - Unix timestamp
  - Casting support to/from numbers and strings
  - `time_components()` - Extract year, month, day, etc.

### Collection Types

#### Lists (`list`)
- Ordered, mutable collections
- Internal representation: linked graph nodes
- Type constraints: `list<num>`, `list<string>`, etc.
- Methods:
  - **Mutation**: `append(value)`, `prepend(value)`, `insert(index, value)`, `remove(value)`, `remove_at_index(index)`, `pop()`, `clear()`, `reverse()`
  - **Inspection**: `size()`, `first()`, `last()`, `contains(value)`, `index_of(value)`, `is_empty()`
  - **Transformation**: `slice(start, end)`, `slice(start, end, step)`, `join(delimiter)`
  - **Functional**:
    - `map(transform)` - Apply transformation to each element
    - `filter(predicate)` - Keep elements matching predicate
    - `select(predicate)` - Alias for filter
    - `reject(predicate)` - Opposite of filter
    - `sort()` - Sort elements
    - `each(func)` - Execute function for each element
  - **Generators**:
    - `generate(start, end, step)` - Range mode
    - `generate(start, end, func)` - Function mode
    - `upto(n)` - Generate from 0 to n
  - **Type Conversion**: `to_bool()`, `to_string()`

**Available Transformations**: `double`, `square`, `negate`, `increment`/`inc`, `decrement`/`dec`, `upper`/`up`, `lower`/`down`, `trim`, `reverse`, `to_string`/`str`, `to_num`/`num`, `to_bool`/`bool`

**Available Predicates**: `positive`/`pos`, `negative`/`neg`, `zero`, `even`, `odd`, `empty`, `non_empty`, `uppercase`, `lowercase`, `alphabetic`/`alpha`, `numeric`/`digit`, `is_string`/`string`, `is_number`/`number`, `is_bool`/`boolean`, `is_list`/`list`, `truthy`, `falsy`

#### Maps (`map`)
- Key-value collections stored as hash graphs
- Type constraints: `map<num>` (value type constraint)
- Multi-line literals supported with trailing commas
- Methods:
  - **Access**: `get(key)`, `has_key(key)`, `keys()`, `values()`
  - **Mutation**: `remove(key)`, `pop(key)`, `merge(other_map)`
  - **Inspection**: `size()`, `is_empty()`
  - **Type Conversion**: `to_bool()`, `to_string()`
- Index access: `map["key"]` returns value
- Index assignment: `map["key"] = value`

#### Trees (`tree`)
- Binary search trees with graph-based implementation
- Created with: `tree{}` literal
- Type constraints: `tree<num>`, `tree<string>`
- Governed by rules: `single_root`, `no_cycles`, `connected`, `max_children_2`
- Methods:
  - `insert(value)` - Insert with BST ordering
  - `search(value)` - Find value in tree
  - `height()` - Tree height
  - Traversals: in-order, pre-order, post-order

#### Graphs (`graph`)
- Universal graph type representing nodes and edges
- Created with: `graph { type: :type_name }` or `graph "name" { ... }`
- Methods:
  - `add_node(id, value)` - Add a node
  - `add_edge(from, to, edge_type, weight?)` - Add an edge
  - `insert(value, parent?)` - Smart insertion using rules
  - `has_path(from, to)` - Check path existence
  - `count_roots()` - Count root nodes
  - `get_children_count(node_id)` - Count children
- Rule-based governance system (see below)
- Edge types: `Child`, `Next`, `Depends`, `Custom(name)`

### Complex Types

#### Functions (`function`)
Defined with `func` keyword:

```glang
func name(param1, param2) {
    # function body
    return value
}

# Optional parameters with defaults
func greet(name, greeting = "Hello") {
    return greeting + ", " + name
}
```

#### Lambdas (`lambda`)
Anonymous functions with arrow syntax:

```glang
double = x => x * 2
add = (x, y) => x + y

# Multi-statement lambdas (future)
process = x => {
    temp = x * 2
    return temp + 1
}
```

#### Regex (`regex`)
- Literals: `/pattern/flags`
- Flags: `i` (case-insensitive), `m` (multiline), `s` (dot matches newline), `x` (ignore whitespace), `g` (global)
- Operators:
  - `=~` - Match operator: `text =~ /pattern/`
  - `!~` - No-match operator: `text !~ /pattern/`
- Capture groups: `$matches` map/list
  - Named captures: `(?<name>pattern)` → `$matches["name"]`
  - Positional captures: `(pattern)` → `$matches[0]`, `$matches[1]`, etc.

#### File Handles (`file_handle`)
- Boundary capabilities for I/O
- Modes: read, write, append
- Auto-close on EOF for read handles
- Methods: `read()`, `read_line()`, `write(text)`, `close()`, `type()`

---

## Graph Rule System

### Built-in Structural Rules

#### Tree Rules
- `no_cycles` - Prevent circular references
- `single_root` - Exactly one root node
- `connected` - All nodes reachable from root
- `max_children_1` - At most 1 child per node
- `max_children_2` - At most 2 children per node (binary tree)
- `max_children_3` - At most 3 children per node
- `max_children_N` - At most N children per node
- `sorted` - Maintain BST ordering
- `balanced` - Keep tree balanced
- `topologically_valid` - Valid topological ordering

#### Graph Rules
- `linear_only` - Enforce sequential structure (linked list)
- `weighted_edges` - Edges must have weights
- `unweighted_edges` - Edges must not have weights
- `bidirectional` - Edges must be two-way
- `acyclic_directed` - DAG structure
- `planar` - Graph must be planar
- `sequential_access` - FIFO/LIFO access patterns
- `fifo_access` - Queue behavior
- `lifo_access` - Stack behavior

#### Size Limits
- `max_nodes(N)` - Maximum number of nodes
- `max_edges(N)` - Maximum number of edges
- `max_degree(N)` - Maximum edges per node

### User-Defined Rules

```glang
# Define custom validation function
func validate_employee_graph(graph) {
    # Return true if valid, false otherwise
    if graph.nodes.size() > 100 {
        return false
    }
    return true
}

# Add to graph (as symbol)
my_graph.add_rule(:validate_employee_graph)

# Or add by function reference
my_graph.add_rule(validate_employee_graph)
```

### Rule Enforcement

- **Validation Timing**: Before operations (add_node, add_edge, insert, remove)
- **Composite Operations**: Rules validated on final state
  - Example: `insert(value, parent)` = `add_node()` + `add_edge()`
  - SingleRoot rule skipped during intermediate state
  - All rules validated on final state
- **Error Handling**:
  - Clear error messages with rule name
  - Specific constraint that was violated
  - Rollback on failure (composite operations)

### Rule Introspection

```glang
# Check if graph has a rule
has_sorted = my_tree.has_rule("sorted")

# Get all active rules
rules = my_graph.get_rules()
```

---

## Intrinsic Behavior System

Data structures can have behaviors attached that automatically transform values during operations (append, insert, set).

### Standard Behaviors

#### Value Transformation
- `none_to_zero` - Convert none to 0
- `none_to_empty` - Convert none to empty string ""
- `positive` - Make negative numbers positive (absolute value)
- `round_to_int` - Round decimal numbers to integers

#### String Transformation
- `uppercase` - Convert strings to UPPERCASE
- `lowercase` - Convert strings to lowercase

#### Validation
- `validate_range(min, max)` - Clamp numbers to specified range

### Generic Mapping Behaviors

Create custom value mappings using hash graphs:

```glang
# Define custom mapping
status_map = { "active": 1, "inactive": 0, "pending": 2 }
user_statuses = ["active", "unknown", "inactive"]

# Apply with default for unmapped keys
user_statuses.add_mapping_rule(status_map, -1)  # -1 for unmapped
# Result: [1, -1, 0]

# Chain mappings for multi-stage transformations
codes = ["a", "b", "c"]
first_map = { "a": "alpha", "b": "beta", "c": "gamma" }
second_map = { "alpha": 1, "beta": 2, "gamma": 3 }
codes.add_mapping_rule(first_map)
codes.add_mapping_rule(second_map)
# Result: [1, 2, 3]
```

### Custom Function Behaviors

```glang
# Define transformation function
func normalize_temp(value) {
    if value < 95 { return 95 }
    if value > 105 { return 105 }
    return value
}

# Attach to collection
temperatures.add_custom_rule(normalize_temp)

# All current and future values are transformed
temperatures.append(110)  # Becomes 105
temperatures.append(85)   # Becomes 95
```

### Conditional Behaviors

Context-aware behaviors that only apply when specific conditions are met:

```glang
# Define condition and transform functions
func is_string(value) {
    return value.get_type() == "string"
}

func to_upper(value) {
    return value.upper()
}

# Apply conditional behavior
mixed_data = [42, "hello", -10, "world"]
mixed_data.add_conditional_rule(is_string, to_upper)
# Result: [42, "HELLO", -10, "WORLD"]

# With fallback for non-matching values
func is_negative(value) {
    return value.get_type() == "number" and value < 0
}

func make_positive(value) {
    return -value
}

func mark_zero(value) {
    return 999  # Marker for non-negative
}

numbers.add_conditional_rule(is_negative, make_positive, mark_zero)
# [5, -3, 0] → [999, 3, 999]
```

### Rulesets (Declarative Bundle Application)

Create reusable behavior bundles:

```glang
# Define ruleset
data_cleaning = Rules[
    "none_to_zero",
    "positive",
    "round_to_int"
]

# Apply to multiple containers
temperatures.add_rules(data_cleaning)
blood_pressure.add_rules(data_cleaning)
heart_rate.add_rules(data_cleaning)
```

### Behavior Management

```glang
# Check if behavior exists
has_positive = list.has_rule("positive")

# Get all active behaviors (sorted alphabetically)
behaviors = list.get_rules()

# Remove specific behavior
list.remove_rule("positive")

# Clear all behaviors
list.clear_rules()
```

### Behavior Application

- **Retroactive**: Applied to all existing values when added
- **Proactive**: Applied to all new values during operations
- **Order Matters**: Behaviors apply in the order they were added
- **Type-Safe**: Works with type constraints

---

## Control Flow

### Conditionals

```glang
if condition {
    # execute when true
}

if condition {
    # true branch
} else {
    # false branch
}

# Logical operators: and, or (synonyms: &&, ||)
if a and b {
    # both must be true
}

if a or b {
    # at least one must be true
}

# Complex expressions (use parentheses for clarity)
if (a == 1) or (b == 2) {
    # recommended style
}
```

### Loops

```glang
# While loop
while condition {
    # loop body
}

# For-in loop
for item in items {
    # process each item
}

# Break and continue
for item in items {
    if item == 5 {
        break       # exit loop
    }
    if item % 2 == 0 {
        continue    # skip to next iteration
    }
    print(item)
}

# Nested loops
for row in matrix {
    for item in row {
        if item > threshold {
            result.append(item)
        }
    }
}
```

### Precision Context Blocks

Control decimal places for numeric operations within a scope:

```glang
# Integer arithmetic (0 decimal places)
precision 0 {
    pi = 3.14159  # Result: 3 (integer, no decimal point)
    area = pi * 10 * 10  # Result: 300 (integer)
}

# Financial calculations (2 decimal places)
precision 2 {
    price = 19.99
    tax = price * 0.085  # Result: 1.70 (exactly 2 places)
    total = price + tax  # Result: 21.69 (exactly 2 places)
}

# Scientific calculations (5 decimal places)
precision 5 {
    pi = 3.14159265358979323846  # Result: 3.14159
    circumference = 2 * pi * 10  # Result: 62.83180
}

# Nested precision contexts
precision 3 {
    outer = 22.0 / 7.0  # Result: 3.143

    precision 1 {
        inner = 22.0 / 7.0  # Result: 3.1
    }

    back = 22.0 / 7.0  # Result: 3.143 (restored)
}
```

**Key Features**:
- Decimal places precision (not significant figures)
- Integer mode: `precision 0` gives true integers with no decimal point
- Scoped semantics (localized, not global)
- Automatic restoration on block exit
- Composable (can nest contexts)

### Configuration Blocks

Control language behavior within a scope:

```glang
# File-level configuration (applies to entire file)
configure { skip_none: false, decimal_places: 2 }

# Block-level configuration
configure { skip_none: true } {
    # All operations skip none values in this block
    data = [1, 2, none, 4]
    result = data.mean()  # none is skipped
}

configure { strict_types: true } {
    # No implicit type conversions
    # result = "5" + 3  # Error
}

# Multiple settings
configure {
    skip_none: false,
    decimal_places: 2,
    strict_types: true
} {
    # All operations use these settings
}
```

**Available Settings**:
- `skip_none` - Skip none values in operations
- `decimal_places` - Default precision
- `strict_types` - Disable implicit type conversions
- `edge_validation` - Enable/disable edge rule checking
- `strict_edge_rules` - Strict vs permissive edge rules
- `none_conversions` - Control none-to-type conversions

---

## Module System

### Importing Modules

```glang
# Import from stdlib
import "module_name"

# Import .gr file
import "path/to/file.gr"

# Import with alias (use sparingly)
import "module" as custom_name

# Module access - dot notation preferred
module.function()
module.value

# Index access also works
module["function"]
module["value"]
```

### Creating Modules

```glang
# In my_module.gr
module my_utilities
alias utils       # Built-in alias

# Define module contents
helper_value = 42

func process(data) {
    return data * 2
}

func validate(input) {
    return input > 0
}
```

### Module Aliases

**Standard Library Aliases**:
- `statistics` → `stats`
- `random` → `rand`
- `regex` → `re`
- `constants` → `const`

**Important**: Using `import "module" as alias` is discouraged for stdlib modules that already have built-in aliases. The built-in aliases are always available without explicit import syntax.

### Loading Files

```glang
# Load merges into current namespace
load "config.gr"

# Variables from config.gr are now available
if debug {
    print("Debug mode enabled")
}
```

**Difference**:
- `import` - Creates module namespace
- `load` - Merges into current namespace

---

## Functional Programming

### Method Chaining

All functional operations return new values and can be chained:

```glang
result = numbers
    .filter(:even)
    .map(:double)
    .map(:to_string)
# [1, 2, 3, 4, 5] → [2, 4] → [4, 8] → ["4", "8"]
```

### Transformations

Named transformations for common operations:

```glang
numbers = [1, 2, 3, 4, 5]

# Numeric transformations
numbers.map(:double)      # [2, 4, 6, 8, 10]
numbers.map(:square)      # [1, 4, 9, 16, 25]
numbers.map(:negate)      # [-1, -2, -3, -4, -5]
numbers.map(:increment)   # [2, 3, 4, 5, 6]
numbers.map(:decrement)   # [0, 1, 2, 3, 4]

# String transformations
names = ["alice", "bob"]
names.map(:upper)         # ["ALICE", "BOB"]
names.map(:lower)         # ["alice", "bob"]
names.map(:trim)          # Trim whitespace
names.map(:reverse)       # Reverse strings

# Type conversions
numbers.map(:to_string)   # ["1", "2", "3", "4", "5"]
strings.map(:to_num)      # [1, 2, 3]
values.map(:to_bool)      # [true, false, true]
```

**Available Transformations**:
- **Numeric**: `double`, `square`, `negate`, `increment`/`inc`, `decrement`/`dec`
- **String**: `upper`/`up`, `lower`/`down`, `trim`, `reverse`
- **Type Conversion**: `to_string`/`str`, `to_num`/`num`, `to_bool`/`bool`

### Predicates

Named predicates for filtering:

```glang
numbers = [-2, -1, 0, 1, 2, 3, 4, 5]

# Numeric predicates
numbers.filter(:positive)  # [1, 2, 3, 4, 5]
numbers.filter(:negative)  # [-2, -1]
numbers.filter(:zero)      # [0]
numbers.filter(:even)      # [-2, 0, 2, 4]
numbers.filter(:odd)       # [-1, 1, 3, 5]

# String/collection predicates
items.filter(:empty)       # Empty items
items.filter(:non_empty)   # Non-empty items
text.filter(:uppercase)    # Uppercase strings
text.filter(:lowercase)    # Lowercase strings
text.filter(:alphabetic)   # Alphabetic strings
text.filter(:numeric)      # Numeric strings

# Type check predicates
mixed.filter(:is_string)   # String values only
mixed.filter(:is_number)   # Number values only
mixed.filter(:is_bool)     # Boolean values only
mixed.filter(:is_list)     # List values only

# General predicates
values.filter(:truthy)     # Truthy values
values.filter(:falsy)      # Falsy values

# Alias: select (same as filter)
numbers.select(:even)      # [2, 4]

# Opposite: reject
numbers.reject(:even)      # [1, 3, 5]
```

**Available Predicates**:
- **Numeric**: `positive`/`pos`, `negative`/`neg`, `zero`, `even`, `odd`
- **String/Collection**: `empty`, `non_empty`, `uppercase`, `lowercase`, `alphabetic`/`alpha`, `numeric`/`digit`
- **Type Checks**: `is_string`/`string`, `is_number`/`number`, `is_bool`/`boolean`, `is_list`/`list`
- **General**: `truthy`, `falsy`

### Lambda Expressions

Custom transformation functions:

```glang
# With lambdas
numbers.map(x => x * 3)
numbers.filter(x => x > 10)

# With named functions
func is_prime(n) {
    if n <= 1 { return false }
    for i in [2, 3, 4, 5] {
        if i * i > n { break }
        if n % i == 0 { return false }
    }
    return true
}

primes = numbers.filter(is_prime)
```

---

## Edge Governance System (Five-Layer Architecture)

### The Five Layers

Every graph structure has five distinct layers:

1. **Data Layer** (Layer 1)
   - Pure information storage
   - Values and basic connections
   - Structural relationships

2. **Behavior Layer** (Layer 2)
   - Computational capabilities
   - Transformations and validations
   - Intrinsic behaviors (see Behavior System above)

3. **Control Layer** (Layer 3)
   - Rule enforcement and governance
   - Edge validation
   - Constraint checking
   - **This is where edge governance lives**

4. **Metadata Layer** (Layer 4)
   - Graph history and provenance
   - Element names and descriptions
   - Audit trail
   - Timestamps and versioning

5. **System Boundary Layer** (Layer 5)
   - External interfaces
   - File handles
   - Network connections
   - API boundaries

### Edge Governance Philosophy

- **Strict by Default**: Experimental language can afford to be cautious
- **Explicit Opt-Out**: Dangerous operations require clear intent
- **Self-Governing**: Graphs enforce their own rules (intrinsic to data)
- **Transparent**: Users can inspect and modify governance
- **Cross-Layer**: Rules can validate across all five layers

### Standard Edge Rules

#### Structural Integrity
- `no_list_cycles` - Prevent cycles in sequential structures
- `maintain_list_order` - Only forward references in lists
- `preserve_hash_keys` - Custom edges can't interfere with key access

#### Cross-Structure Protection
- `same_structure_only` - Edges only within same data structure
- `compatible_types` - Edge endpoints must have compatible types

### Configuration Syntax

```glang
# Default: strict rules active
people = ["Alice", "Bob", "Charlie"]
people.add_edge(0, 1, "friend")    # ✓ Allowed (same structure)
people.add_edge(2, 0, "circular")  # ❌ BLOCKED (creates cycle)

# Explicit opt-out for specific cases
configure { strict_edge_rules: false } {
    # Dangerous operations allowed with clear intent
    ring_buffer.add_edge(tail, head, "wrap_around")  # ✓ Allowed
}

# Disable specific rules
configure { disabled_edge_rules: ["no_list_cycles"] } {
    # Only this specific danger allowed
    circular_buffer.add_edge(last, 0, "circular")  # ✓ Allowed
}

# Graph-level customization
my_special_list.control_layer.disable_rule("no_list_cycles")
my_special_list.add_edge(3, 0, "wrap")  # ✓ Allowed for this graph
```

### Edge Introspection

```glang
# Check if edge operation is allowed
can_add = my_list.can_add_edge(2, 0, "circular")  # Returns false + reason

# Get current edge rules
rules = my_list.get_edge_rules()  # ["no_list_cycles", "same_structure_only"]
```

---

## Standard Library

### Math & Numbers

#### Constants Module
```glang
import "constants"

# Mathematical constants
pi = constants["pi"]              # 3.141592653589793
e = constants["e"]                # 2.718281828459045
tau = constants["tau"]            # 6.283185307179586 (2π)
phi = constants["phi"]            # 1.618033988749895 (golden ratio)
sqrt2 = constants["sqrt2"]        # √2
sqrt3 = constants["sqrt3"]        # √3

# Angle conversions
deg_to_rad = constants["deg_to_rad"]  # π/180
rad_to_deg = constants["rad_to_deg"]  # 180/π

# Physical constants
c = constants["c"]                # 299792458 m/s (speed of light)
G = constants["G"]                # 6.67430e-11 (gravitational constant)
```

#### Number Methods
All numbers have built-in methods:
- `sqrt()` - Square root
- `pow(n)` - Raise to power
- `abs()` - Absolute value
- `up()` - Ceiling
- `up(n)` - Ceiling to n decimal places
- `down()` - Floor
- `down(n)` - Floor to n decimal places
- `round()` - Round to nearest
- `round(n)` - Round to n decimal places
- `log()` - Natural logarithm
- `log(base)` - Logarithm with base

### Randomness

#### Random Module
```glang
import "random"  # alias: rand

# Cryptographically secure
rand.random()                    # Float [0.0, 1.0)
rand.randint(min, max)          # Integer [min, max] inclusive
rand.uniform(min, max)          # Float [min, max)
rand.normal(mean, std_dev)      # Normal distribution
rand.exponential(lambda)        # Exponential distribution
rand.gamma(alpha, beta)         # Gamma distribution

# Deterministic (seeded)
rand.seed(42)                   # Seed RNG
rand.det_random()               # Deterministic random
rand.det_randint(min, max)      # Deterministic integer
rand.det_uniform(min, max)      # Deterministic uniform
rand.det_normal(mean, std_dev)  # Deterministic normal
rand.reset()                    # Reset to unseeded state

# UUID generation
rand.uuid4()                    # Random UUID v4
rand.uuid1()                    # Time-based UUID v1

# Secure tokens
rand.token(length)              # Hex token (length bytes)
```

### Time

#### Time Module
```glang
import "time"

# Create time values
current = time.now()                          # Current time
today = time.today()                         # Start of today
birthday = time.from_components(1990, 12, 25)
meeting = time.from_components(2025, 1, 15, 14, 30, 0)
parsed = time.from_string("2025-01-15T14:30:00Z")
from_ts = time.from_timestamp(1704067200)

# Time value methods
current.to_string()     # ISO format
current.to_num()        # Unix timestamp
current.get_type()      # "time"

# Extract components
components = time.time_components(current)
# Returns map: { "year": 2025, "month": 1, "day": 15,
#                "hour": 14, "minute": 30, "second": 0,
#                "weekday": 2, "day_of_year": 15 }

# Casting
timestamp = current.to_num()
time_from_num = timestamp.to_time()
time_from_str = "2025-01-15T14:30:00".to_time()
```

### String Processing

#### Regex Module
```glang
import "regex"  # alias: re

# Regex literals
pattern = /hello/i          # Case-insensitive
email_pattern = /\w+@\w+/

# Operators
if text =~ /pattern/ {      # Match
    print("Found!")
}

if text !~ /pattern/ {      # No match
    print("Not found!")
}

# Capture groups
text = "John Doe, 30"
if text =~ /(\w+) (\w+), (\d+)/ {
    first = $matches[0]     # "John"
    last = $matches[1]      # "Doe"
    age = $matches[2]       # "30"
}

# Named captures
if text =~ /(?<first>\w+) (?<last>\w+), (?<age>\d+)/ {
    first = $matches["first"]
    last = $matches["last"]
    age = $matches["age"]
}

# String methods with regex
parts = text.split(/\s+/)              # Split by whitespace
cleaned = text.replace(/\d+/, "X")     # Replace numbers
emails = text.find_all(/\w+@\w+/)      # Find all emails
```

#### String Semantic Methods
```glang
text = "Hello World 123! Contact support@example.com"

# Unified contains
text.contains("any", "digits")          # true (has digits)
text.contains("all", "letters", "digits") # true (has both)
text.contains("only", "letters", "spaces") # false

# Unified extract
numbers = text.extract("numbers")       # ["123"]
words = text.extract("words")           # ["Hello", "World", "Contact"]
emails = text.extract("emails")         # ["support@example.com"]

# Unified count
digit_count = text.count("digits")      # 3
word_count = text.count("words")        # 3

# Find first
pos = text.find_first("digits")         # Position of first digit
```

**Available Patterns**: `:digits`/`:numbers`, `:letters`, `:uppercase`, `:lowercase`, `:spaces`/`:whitespace`, `:punctuation`, `:symbols`, `:alphanumeric`, `:words`, `:emails`

### File I/O

#### IO Module
```glang
import "io"

# Read operations (auto-close on EOF)
read_handle = io.open("data.txt", "r")
content = read_handle.read()        # Read all, auto-closes
# read_handle.read()                # Error: exhausted

# Incremental reading
input = io.open("large.txt", "r")
while true {
    line = input.read_line()
    if line == "" { break }         # EOF, auto-closed
    process(line)
}

# Write operations (manual close)
write_handle = io.open("output.txt", "w")
write_handle.write("Line 1\n")
write_handle.write("Line 2\n")
write_handle.close()                # Must manually close

# Append mode
append_handle = io.open("log.txt", "a")
append_handle.write("New entry\n")
append_handle.close()
```

### Data Formats

#### JSON Module
```glang
import "json"

# Parse JSON
data = json.parse('{"name": "Alice", "age": 30}')
name = data["name"]

# Stringify to JSON
json_str = json.stringify(data)
```

#### CSV Module
```glang
import "csv"

# Parse CSV
rows = csv.parse(csv_text)
# Returns: [{"col1": "val1", "col2": "val2"}, ...]

# Generate CSV
csv_text = csv.generate(rows)

# Validate CSV
is_valid = csv.validate(csv_text)
```

#### SQL Module
```glang
import "sql"

# Query builder
query = sql.Table("users")
    .where("age", ">", 18)
    .sort("name")
    .limit(10)

# Generate SQL
sql_string = query.to_sql()
# "SELECT * FROM users WHERE age > 18 ORDER BY name LIMIT 10"
```

### Statistics

#### Statistics Module
```glang
import "statistics"  # alias: stats

# Descriptive statistics
data = [1, 2, 3, 4, 5]
mean = stats.mean(data)
median = stats.median(data)
mode = stats.mode(data)
std_dev = stats.std_dev(data)
variance = stats.variance(data)
```

### Network

#### HTTP Module
```glang
import "http"

# HTTP GET
response = http.get("https://api.example.com/data")
if response["status"] == 200 {
    data = json.parse(response["body"])
}

# HTTP POST
response = http.post("https://api.example.com/submit", {
    "Content-Type": "application/json"
}, json.stringify({"key": "value"}))
```

#### HTML Module
```glang
import "html"

# Parse HTML
doc = html.parse(html_text)

# Query elements
titles = doc.find_all("h1")
links = doc.find_all("a")

# Extract attributes
href = link.get_attribute("href")
```

### Cryptography

#### Crypto Module
```glang
import "crypto"

# SHA-256 hashing
hash = crypto.sha256("data")

# Ed25519 signatures
keypair = crypto.ed25519_keygen()
signature = crypto.ed25519_sign(keypair["private"], "message")
valid = crypto.ed25519_verify(keypair["public"], "message", signature)

# AES encryption
key = crypto.aes_keygen()
encrypted = crypto.aes_encrypt(key, "plaintext")
decrypted = crypto.aes_decrypt(key, encrypted)
```

---

## Syntax Features

### Optional Parentheses for Zero-Argument Methods

```glang
# Both forms are valid
size = items.size()     # With parentheses
size = items.size       # Without (property-like access)

# Multi-argument methods always need parentheses
items.insert(0, "first")
```

### Type Inference

```glang
# Inferred types (recommended when obvious)
name = "Alice"              # string
age = 25                    # num
active = true               # bool
items = [1, 2, 3]           # list
config = {"host": "localhost"}  # map

# Explicit types (when needed for clarity or constraints)
string username = "Bob"
num max_age = 100
bool is_valid = false
list<num> scores = [95, 87, 92]
map<string> settings = {"theme": "dark"}
tree<num> numbers = tree{}
```

### Multi-line Literals

```glang
# Maps with trailing commas
config = {
    "host": "localhost",
    "port": 8080,
    "debug": true,    # Trailing comma OK
}

# Lists with trailing commas
items = [
    "apple",
    "banana",
    "cherry",    # Trailing comma OK
]

# Improves readability and reduces diff noise
```

### Index Access and Assignment

```glang
# Lists
first = items[0]        # Get first element
last = items[-1]        # Get last element (if supported)
items[0] = 99           # Set element

# Maps
host = config["host"]   # Get value by key
config["port"] = 8080   # Set or create key

# Strings
char = text[0]          # Get character at index
```

### Pattern Matching (Future)

```glang
# Implicit pattern functions
func factorial(n) {
    0 => 1
    1 => 1
    x => x * factorial(x - 1)
}

# Automatic fallthrough (returns none)
func get_sound(animal) {
    "dog" => "woof"
    "cat" => "meow"
    "cow" => "moo"
    # No explicit wildcard needed
}
```

**Key Features**:
- Zero ceremony (no `match` keyword)
- Automatic fallthrough to none
- Perfect for recursive functions
- Functional elegance with imperative practicality

---

## Design Principles

### KISS Principle (Keep It Simple, Stupid!)
- Despises unnecessary verbiage and redundant syntax
- One clear, obvious way to do things
- Minimal boilerplate
- Natural programming constructs

### No Semantic Markers
All code must be fully implemented with real enforcement. Never resort to "semantic markers" or placeholder implementations. Rules are rules—they enforce constraints, not document intentions.

### No Method Proliferation
**Critical Principle**: Simplicity argues against proliferating almost-identical methods.

- ❌ **BAD**: `io.print("message")` and `io.print_nonewline("message")`
- ✅ **GOOD**: `io.print("message")` and `io.print("message", false)`

- ❌ **BAD**: `list.remove_first(element)` and `list.remove_all(element)`
- ✅ **GOOD**: `list.remove(element)` and `list.remove(element, :all)`

**Rationale**: Each new method pollutes the namespace and increases the learning curve. Parameters with symbols (`:all`, `:nonewline`) provide clear intent without creating dozens of methods.

### Self-Hosting Vision
- Write as much as possible in Graphoid itself
- Target: 90%+ of standard library in pure Graphoid
- Only core runtime in Rust/native code
- Enables language to evolve through itself

### Developer Experience First
- **Helpful error messages** with source positions (file, line, column)
- **Rich REPL** with tab completion and multiline support
- **Clear, predictable behavior** (Principle of Least Surprise)
- **Excellent documentation** with examples
- **No surprises** - code behaves exactly as you'd expect

### Clean Import Syntax
**NEVER use `import "module" as alias` for stdlib modules** - they have built-in aliases that make this redundant.

- ❌ **BAD**: `import "random" as rand` (redundant - rand is already available!)
- ✅ **GOOD**: `import "random"` (gives you both `random` and `rand`)

**When aliases ARE appropriate**:
- Only for user-defined modules with more descriptive names
- Never for standard library modules with built-in aliases

---

## Implementation Requirements

### Core Runtime (Must be in Native Code - Rust)

The following components MUST be implemented in Rust for performance, safety, and fundamental language semantics:

1. **Lexer & Parser**
   - Tokenization with position tracking
   - AST construction with typed nodes
   - Error recovery and reporting

2. **Type System**
   - Type inference engine
   - Constraint checking and validation
   - Type casting rules

3. **Memory Model**
   - Graph node allocation and lifecycle
   - Garbage collection (reference counting or tracing)
   - Memory safety guarantees

4. **Execution Engine**
   - AST interpretation or compilation
   - Value system (tagged union or trait objects)
   - Method dispatch and resolution

5. **Module Loader**
   - .gr file loading and caching
   - Module namespace management
   - Dependency resolution

6. **Graph Infrastructure**
   - Node/edge storage and indexing
   - Rule validation engine
   - Graph traversal primitives

### Native Libraries (Performance/Security Critical)

The following should be implemented in Rust but exposed as modules:

1. **File I/O**
   - Low-level file operations
   - Buffered reading/writing
   - Async I/O (future)

2. **Network Stack**
   - TCP/UDP sockets
   - HTTP client/server
   - TLS/SSL (future)

3. **Cryptography**
   - SHA-256, SHA-512
   - Ed25519 signatures
   - AES encryption
   - Constant-time guarantees

4. **JSON**
   - Fast parsing
   - Serialization
   - Streaming support (future)

5. **Regex**
   - Pattern compilation
   - Matching engine
   - Capture group extraction

6. **Time**
   - System time access
   - Date/time arithmetic
   - Timezone support (future)

### Pure Graphoid (Standard Library)

The following should be implemented in pure .gr files:

1. **Statistics**
   - mean, median, mode
   - std_dev, variance
   - percentiles, quartiles

2. **CSV**
   - Parsing (using string methods)
   - Generation
   - Validation

3. **SQL**
   - Query builder
   - WHERE clause construction
   - JOIN logic

4. **Graph Algorithms**
   - BFS, DFS
   - Shortest path (Dijkstra)
   - Topological sort
   - Cycle detection

5. **Logging**
   - Structured logging
   - Log levels
   - Formatters

6. **Simulation**
   - Event scheduler
   - Discrete event simulation
   - Tick system

7. **Distributions**
   - Built on random primitives
   - Normal, beta, binomial
   - Box-Muller transform

---

## Value System Architecture

### Design Philosophy
- **Tagged Union**: Single enum type for all values
- **Inline Primitives**: No heap allocation for Number, String, Boolean
- **Boxed Collections**: ListData, MapData, GraphValue on heap
- **Behavior Integration**: Collections include BehaviorRules
- **Type Safety**: Strong typing with pattern matching

### Core Value Types

```rust
enum Value {
    // Primitives (inline, no heap allocation)
    Number(NumberData),      // f64 + optional display precision
    String(String),          // UTF-8 string
    Boolean(bool),           // true/false
    None,                    // Absence of value
    Symbol(String),          // :symbol
    Time(f64),               // UTC timestamp

    // Collections (boxed, includes behaviors)
    List(Box<ListData>),     // Vec<Value> + BehaviorRules
    Map(Box<MapData>),       // HashMap<String, Value> + BehaviorRules

    // Complex types (boxed)
    Graph(Box<GraphValue>),  // Nodes, edges, RuleBundle
    Lambda(Box<LambdaData>), // Params, body, captures
    Function(Box<FunctionData>), // Name, params, body, captures
    Regex(Box<RegexData>),   // Pattern, compiled, flags
    FileHandle(Box<FileHandleData>), // Path, mode, state
}
```

### Collection Data Structures

```rust
struct ListData {
    elements: Vec<Value>,
    behaviors: BehaviorRules,
}

struct MapData {
    entries: HashMap<String, Value>,
    behaviors: BehaviorRules,
}

struct GraphValue {
    nodes: HashMap<String, GraphNode>,
    rule_bundle: RuleBundle,
    metadata: HashMap<String, String>,
    root_node: Option<String>,
}
```

### Behavior System Integration

```rust
struct BehaviorRules {
    transformations: Vec<Transformation>,
    validators: Vec<Validator>,
    mappings: Vec<MappingRule>,
    conditionals: Vec<ConditionalRule>,
}

impl BehaviorRules {
    fn apply_to_value(&self, value: &mut Value) {
        // Apply all transformations in order
        for transform in &self.transformations {
            *value = transform.apply(value);
        }
    }
}
```

### Universal Methods

All values must support:
- `get_type()` → &str
- `to_display_string()` → String
- `universal_size()` → usize
- `is_truthy()` → bool
- Arithmetic: `add()`, `subtract()`, `multiply()`, `divide()`, `modulo()`, `negate()`
- Comparison: `equals()`, `less_than()`, `greater_than()`
- Casting: `to_num()`, `to_string()`, `to_bool()`

---

## Error Handling

### Error Categories

1. **Parse Errors**
   - Unexpected token
   - Missing closing delimiter
   - Invalid syntax

2. **Type Errors**
   - Type mismatch in operation
   - Type constraint violation
   - Invalid type conversion

3. **Runtime Errors**
   - Division by zero
   - Index out of bounds
   - Key not found
   - File not found

4. **Rule Violations**
   - Graph rule violated
   - Edge governance violated
   - Behavior constraint violated

### Error Message Format

```
Error: [Category] [Specific Error]
  at line X, column Y in file.gr

[Context lines with highlighting]

[Explanation of what went wrong]

[Suggestion for fix (if available)]
```

### Example Error Messages

```
Error: Type constraint violation
  at line 15, column 8 in process.gr

  13 | list<num> scores = [95, 87, 92]
  14 | scores.append(100)
  15 | scores.append("invalid")
               ^^^^^^^^^^^^^^^

Cannot append string to list<num>
Expected: num
Received: string

Suggestion: Convert to number or use a list without type constraint
```

```
Error: Graph rule violated
  at line 42, column 5 in tree.gr

  41 | my_tree.insert(50)
  42 | my_tree.add_node("invalid", 25)
           ^^^^^^^^^^^

Rule 'single_root' violated: Operation would create 2 roots
(single_root requires exactly 1)

Suggestion: Use insert() with parent parameter instead of add_node()
```

---

## Built-In Testing Framework

Graphoid includes a comprehensive **behavior-driven testing framework** (RSpec-style) as part of the language, not an external library.

### Spec File Convention

Test files use `.spec.gr` extension and are automatically discovered by `graphoid spec`:

```glang
# In calculator.spec.gr
import "spec"
import "../src/calculator"

describe "Calculator" {

    describe "add" {
        it "adds two positive numbers" {
            result = calculator.add(2, 3)
            expect(result).to_equal(5)
        }

        it "handles negative numbers" {
            result = calculator.add(-2, -3)
            expect(result).to_equal(-5)
        }
    }

    describe "divide" {
        context "when dividing by zero" {
            it "raises an error" {
                expect(func() {
                    calculator.divide(10, 0)
                }).to_raise("RuntimeError")
            }
        }

        context "when dividing valid numbers" {
            it "returns the quotient" {
                expect(calculator.divide(10, 2)).to_equal(5)
            }
        }
    }
}
```

### Expectation API

Built-in `spec` module provides expressive expectations:

```glang
import "spec"

# Equality
expect(value).to_equal(expected)
expect(value).not_to_equal(expected)

# Truthiness
expect(value).to_be_true()
expect(value).to_be_false()
expect(value).to_be_truthy()

# Comparison
expect(value).to_be_greater_than(threshold)
expect(value).to_be_less_than(threshold)
expect(value).to_be_between(min, max)

# Type checks
expect(value).to_be_a("num")
expect(value).to_be_a_string()
expect(value).to_be_a_list()

# Collection matchers
expect(collection).to_contain(element)
expect(collection).to_be_empty()
expect(collection).to_have_size(expected_size)

# Approximate equality (floats)
expect(3.14159).to_be_close_to(3.14, 0.01)
expect(value).to_be_within(0.001).of(expected)

# Exception matchers
expect(func() { risky_op() }).to_raise("RuntimeError")
expect(func() { safe_op() }).not_to_raise()

# Deep equality
expect(nested).to_deeply_equal(expected)

# Regex matchers
expect(text).to_match(/pattern/)
```

### Test Organization

```glang
describe "Feature" {
    # Hooks
    before_all {
        # Run once before all specs in this describe
    }

    after_all {
        # Run once after all specs
    }

    before_each {
        # Run before each spec
    }

    after_each {
        # Run after each spec
    }

    # Nested describe blocks
    describe "sub-feature" {
        it "behaves correctly" {
            expect(result).to_be_true()
        }
    }

    # Context blocks for scenarios
    context "when condition is met" {
        it "produces expected result" {
            expect(output).to_equal(expected)
        }
    }
}
```

### Running Specs

```bash
# Run all specs
graphoid spec

# Run specific file
graphoid spec specs/calculator.spec.gr

# Run with documentation format
graphoid spec --format documentation

# Run with coverage
graphoid spec --coverage
```

**Output (documentation format)**:
```
Calculator
  add
    ✓ adds two positive numbers (0.2ms)
    ✓ handles negative numbers (0.1ms)
  divide
    when dividing by zero
      ✓ raises an error (0.3ms)
    when dividing valid numbers
      ✓ returns the quotient (0.1ms)

Specs: 4 passed, 0 failed, 4 total
Time: 0.7s
Coverage: 92%
```

See [Production Tooling Specification](PRODUCTION_TOOLING_SPECIFICATION.md) for complete testing framework documentation.

---

## Testing Philosophy (Implementation)

### Test Coverage Goals
- **Core Language**: 100% coverage of language features
- **Standard Library**: 90%+ coverage
- **Edge Cases**: Comprehensive edge case testing
- **Error Paths**: All error conditions tested

### Test Organization (Implementation)
```
tests/
  unit/           # Unit tests for individual features (Rust)
    lexer/
    parser/
    execution/
    types/
    collections/
    behaviors/
    rules/
  integration/    # Integration tests for workflows (Rust)
    file_io/
    modules/
    stdlib/
  samples/        # Example programs (.gr files)
  *.test.gr       # User-facing Graphoid tests
```

### Testing Principles
- **Fast Feedback**: Unit tests run in milliseconds
- **Isolated**: Tests don't depend on each other
- **Comprehensive**: Every feature has tests
- **Documented**: Tests serve as examples
- **Regression**: Bug fixes include regression tests

---

## Future Directions

### Near-Term (Next 6 Months)
- Complete Rust implementation with Python parity
- Achieve 90%+ self-hosting
- FFI (Foreign Function Interface) for native code access
- Performance optimization (JIT compilation)

### Mid-Term (6-12 Months)
- Concurrency primitives (threads, async/await, channels)
- Advanced pattern matching
- Unit-safe numerics (compile-time unit checking)
- Self-modification capabilities with governance

### Long-Term (1-2 Years)
- Distributed graph systems
- Network programming as graph partitioning
- Self-aware computational systems
- Graph-based memory management
- Compiler optimizations as graph rewrites

---

## Appendices

### Appendix A: Complete Syntax Grammar (BNF-like)

```
Program ::= Statement*

Statement ::=
    | VariableDeclaration
    | Assignment
    | FunctionDeclaration
    | IfStatement
    | WhileLoop
    | ForLoop
    | ReturnStatement
    | BreakStatement
    | ContinueStatement
    | ImportStatement
    | LoadStatement
    | ModuleDeclaration
    | ConfigureBlock
    | PrecisionBlock
    | ExpressionStatement

VariableDeclaration ::= Type? Identifier "=" Expression
Type ::= "num" | "string" | "bool" | "list" | "map" | "tree" | "graph"
       | "list" "<" Type ">"
       | "map" "<" Type ">"
       | "tree" "<" Type ">"

Assignment ::= Identifier "=" Expression
             | IndexAccess "=" Expression

FunctionDeclaration ::= "func" Identifier "(" Parameters? ")" Block
Parameters ::= Parameter ("," Parameter)*
Parameter ::= Identifier ("=" Expression)?

IfStatement ::= "if" Expression Block ("else" Block)?

WhileLoop ::= "while" Expression Block

ForLoop ::= "for" Identifier "in" Expression Block

ReturnStatement ::= "return" Expression?

ImportStatement ::= "import" StringLiteral ("as" Identifier)?

LoadStatement ::= "load" StringLiteral

ModuleDeclaration ::= "module" Identifier ("alias" Identifier)?

ConfigureBlock ::= "configure" "{" ConfigSettings "}" Block?
PrecisionBlock ::= "precision" Number Block

Expression ::=
    | Literal
    | Identifier
    | FunctionCall
    | MethodCall
    | IndexAccess
    | BinaryOp
    | UnaryOp
    | Lambda
    | ListLiteral
    | MapLiteral
    | TreeLiteral
    | GraphLiteral

Literal ::= Number | String | Boolean | Symbol | Regex | "none"

FunctionCall ::= Identifier "(" Arguments? ")"
MethodCall ::= Expression "." Identifier ("(" Arguments? ")")?
IndexAccess ::= Expression "[" Expression "]"

BinaryOp ::= Expression Operator Expression
Operator ::= "+" | "-" | "*" | "/" | "%" | "^"
           | "==" | "!=" | "<" | ">" | "<=" | ">="
           | "and" | "or" | "&&" | "||"
           | "=~" | "!~"

UnaryOp ::= "-" Expression | "not" Expression

Lambda ::= Identifier "=>" Expression
         | "(" Parameters ")" "=>" Expression

ListLiteral ::= "[" (Expression ("," Expression)* ","?)? "]"
MapLiteral ::= "{" (MapEntry ("," MapEntry)* ","?)? "}"
MapEntry ::= String ":" Expression

TreeLiteral ::= "tree" "{" "}"
              | "tree" "<" Type ">" "{" "}"

GraphLiteral ::= "graph" "{" GraphSpec "}"
               | "graph" String "{" GraphSpec "}"
GraphSpec ::= (GraphProperty ("," GraphProperty)*)?
GraphProperty ::= "type" ":" Symbol
                | "inherits" ":" Identifier
                | "rules" ":" ListLiteral
```

### Appendix B: Operator Precedence

From highest to lowest:

1. Method call, index access: `.`, `[]`
2. Unary: `-`, `not`
3. Power: `^`
4. Multiplicative: `*`, `/`, `%`
5. Additive: `+`, `-`
6. Comparison: `<`, `>`, `<=`, `>=`
7. Equality: `==`, `!=`
8. Regex match: `=~`, `!~`
9. Logical AND: `and`, `&&`
10. Logical OR: `or`, `||`

### Appendix C: Reserved Keywords

```
and, or, not, if, else, while, for, in, break, continue,
return, func, import, load, module, alias, configure,
precision, true, false, none, num, string, bool, list,
map, tree, graph, data
```

### Appendix D: Standard Library Module Index

- **constants** - Mathematical and physical constants
- **random** (rand) - Random number generation
- **time** - Date/time handling
- **regex** (re) - Regular expressions
- **io** - File I/O
- **json** - JSON parsing/serialization
- **csv** - CSV handling
- **sql** - SQL query builder
- **html** - HTML parsing
- **http** - HTTP client
- **crypto** - Cryptography
- **statistics** (stats) - Statistical functions

---

**End of Specification**

This document serves as the canonical, implementation-independent specification for the Graphoid/Glang programming language. All implementations should strive to conform to this specification.
