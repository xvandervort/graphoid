# Graphoid: Canonical Language Specification

**Version**: 1.0
**Last Updated**: January 2025
**Status**: Definitive specification for fresh Rust implementation

---

## Core Philosophy: Everything is a Graph

Graphoid is built on the radical principle that **every aspect of computation can and should be represented as a graph**. This isn't just about using graphs as data structures—it's about making graphs the fundamental abstraction at every level.

### The Three Levels of Graph Abstraction

#### Level 1: Data Structures as Graphs
- A list `[1, 2, 3]` is internally `Node(1) → Node(2) → Node(3)`
- A tree is a graph with hierarchical constraints (`:tree` ruleset)
- Hash tables are graphs with key-value edges
- No artificial boundaries between "graph" and "non-graph" data
- Collections can transform between forms by adding/removing rules

#### Level 2: Variable Storage as Graphs (Meta-Graph)
- Variables are nodes in a meta-graph
- Variable names are name nodes: `📛 "fruits"`
- Variable values are value nodes: `📊 LinearGraph([apple, banana])`
- Assignment creates edges: `📛 "fruits" → 📊 LinearGraph`
- The namespace itself IS a graph that can be inspected and manipulated

#### Level 3: Runtime Environment as Graphs (Future)
- function calls as graph traversals
- functions are nodes with parameter and return edges
- Call stack as path through function graph
- Modules as subgraphs with import/export edges
- Recursion as cycles in the call graph

---

## Type System

### 🚫 The "No Generics" Policy

**IMPORTANT**: Graphoid has a strict "No Generics" policy. This is a fundamental, non-negotiable design decision.

**See**: `dev_docs/NO_GENERICS_POLICY.md` for complete details.

#### What This Means

✅ **Allowed** (simple, runtime type assertions):
- `list<num>` - Single type parameter, runtime-checked, built-in collections only
- `hash<string>` - Single type parameter for value constraints
- `tree<num>`, `graph<num>` - Single parameter on built-in types

❌ **NEVER Allowed**:
- Multiple type parameters: `HashMap<K, V>` - **FORBIDDEN**
- User-defined generics: `class Container<T>` - **FORBIDDEN**
- Generic functions: `fn process<T>(x: T)` - **FORBIDDEN**
- Nested constraints: `list<list<num>>` - **FORBIDDEN**
- Type bounds: `<T: Trait>` - **FORBIDDEN**

#### Why No Generics?

Generics add unnecessary complexity. Instead, Graphoid provides:
1. **Duck typing** - Works on any compatible value
2. **Graph rules** - Structural constraints at runtime
3. **Runtime type checks** - Explicit validation when needed
4. **Optional type hints** - Documentation without enforcement

#### The One Rule

**"One type parameter, runtime-checked, built-in collections only"**

This is as far as type constraints go. No exceptions.

---

### Primitive Types

#### Numbers (`num`)
- Floating-point by default (f64)
- Display precision context: `precision N { ... }` controls decimal places
- Integer display when fractional part is zero
- **Arithmetic Operators**:
  - `+`, `-`, `*`, `/` - Standard arithmetic (division returns float)
  - `//` - **Integer division** (returns integer, truncates toward zero)
  - `%` - Modulo (remainder)
  - `^` - Exponentiation
- **Integer Division Examples**:
  ```graphoid
  10 / 3      # 3.333333 (float division)
  10 // 3     # 3 (integer division)
  -10 // 3    # -3 (truncates toward zero)
  10.5 // 2   # 5 (works on floats too)
  ```
- Methods:
  - `sqrt()` - Square root
  - `abs()` - Absolute value
  - `up()` - Ceiling (no decimal places)
  - `up(n)` - Ceiling with n decimal places
  - `up(:nearest_ten)` - Ceiling to nearest 10, 100, 1000, etc.
  - `down()` - Floor (no decimal places)
  - `down(n)` - Floor with n decimal places
  - `down(:nearest_ten)` - Floor to nearest 10, 100, 1000, etc.
  - `round()` - Round to nearest (no decimal places)
  - `round(n)` - Round to n decimal places
  - `round(:nearest_ten)` - Round to nearest 10, 100, 1000, etc.
  - `log()` - Natural logarithm
  - `log(base)` - Logarithm with specified base

#### Strings (`string`)
- Internally represented as character node graphs
- Unicode-aware grapheme cluster support
- **Mutation Convention**: Methods ending with `!` mutate in place and return `none`. Methods without `!` return new strings.
- Methods:
  - **Case Conversion** (two versions):
    - `upper()` / `upper!()` - Convert to uppercase (returns new string / mutates in place)
    - `lower()` / `lower!()` - Convert to lowercase (returns new string / mutates in place)
  - **Whitespace** (two versions):
    - `trim()` / `trim!()` - Remove leading/trailing whitespace (returns new string / mutates in place)
  - **Transformation** (two versions):
    - `reverse()` / `reverse!()` - Reverse character order (returns new string / mutates in place)
  - **Extraction** (immutable):
    - `substring(start, end)` - Extract substring by character indices
  - **Splitting** (immutable):
    - `split(delimiter)` - Split into list by delimiter
    - `split(regex)` - Split into list by regex pattern
  - **Pattern Matching** (immutable):
    - `starts_with(prefix)` - Check if starts with prefix
    - `ends_with(suffix)` - Check if ends with suffix
    - `contains(substring)` - Check if contains substring
  - **Unified Semantic Methods** (immutable):
    - `contains(mode, patterns...)` - Check for patterns
    - `extract(pattern)` - Extract matching content
    - `count(pattern)` - Count occurrences
    - `find(pattern)` - Find position of matches (optional second param limits how many matches)
  - **Regex Support** (immutable):
    - `replace(regex, replacement)` - Replace matches with replacement
    - `find_all(regex)` - Find all regex matches

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
- Predefined directives to the interpreter, not a user-defined data type
- Syntax: `:name`
- Interned for efficiency (one instance per unique name)
- Used for method parameters, predicates, transformations, and configuration
- **Users cannot create symbols** - they may only use predefined symbols provided by the language and standard library
- Examples: `:double`, `:even`, `:words`, `:nearest_hundred`, `:all`
- Common contexts:
  - Transformations: `list.map(:double)`
  - Predicates: `list.filter(:even)`
  - String patterns: `text.extract(:words)`
  - Rounding modes: `num.round(:nearest_hundred)`
  - Method options: `list.remove(value, :all)`

#### None (`none`)
- Represents absence of value
- Configurable conversion behavior via configuration context
- Safe propagation through operations
- Detection methods: `is_none()`, `is_some()`
- Methods: 
  - `to_num()` => 0
  - `to_string()` => "" (empty string)

#### Time (`time`)
- Internal representation: UTC timestamp (f64)
- Display: ISO 8601 format (`YYYY-MM-DDTHH:MM:SSZ`)
- Creation:
  - `time.now()` - Current time
  - `time.today()` - Start of today (00:00:00 UTC)
  - `time.from_numbers(year, month, day, hour?, minute?, second?)`
  - `time.from_string(iso_string)`
  - `time.from_timestamp(number)`
- Methods:
  - `to_string()` - ISO format
    - May take format string such as '%Y-%m-%d'
  - `to_num()` - Unix timestamp
  - Casting support to/from numbers and strings
  - `time_numbers()` - Extract year, month, day, etc.

### Collection Types

#### Lists (`list`)
- Ordered, mutable collections
- Internal representation: linked graph nodes
- Type constraints: `list<num>`, `list<string>`, etc.
- **Mutation Convention**: Methods ending with `!` mutate in place and return `none`. Methods without `!` return new lists.
- Methods:
  - **Inspection** (read-only): `size()`, `first()`, `last()`, `contains(value)`, `index_of(value)`, `is_empty()`
  - **Freeze Queries**: `is_frozen()`, `has_frozen()`, `has_frozen(:verbose)`
  - **Transformation** (immutable): `slice(start, end)`, `slice(start, end, step)`, `join(delimiter)`
  - **Mutation Methods** (two versions):
    - `append(value)` / `append!(value)` - Add element (returns new list / mutates in place)
    - `prepend(value)` / `prepend!(value)` - Add to beginning (returns new list / mutates in place)
    - `insert(index, value)` / `insert!(index, value)` - Insert at position (returns new list / mutates in place)
    - `remove(value)` / `remove!(value)` - Remove first occurrence (returns new list / mutates in place)
    - `remove_at_index(index)` / `remove_at_index!(index)` - Remove at index (returns new list / mutates in place)
    - `pop()` - Returns last element without mutation (use with `last()` for read-only access)
    - `pop!()` - Mutates list, removes and returns last element
    - `clear()` / `clear!()` - Remove all elements (returns new empty list / mutates in place)
  - **Functional** (two versions):
    - `map(transform)` / `map!(transform)` - Transform each element (new list / in place)
    - `filter(predicate)` / `filter!(predicate)` - Keep matching elements (new list / in place)
    - `select(predicate)` / `select!(predicate)` - Alias for filter
    - `reject(predicate)` / `reject!(predicate)` - Remove matching elements (new list / in place)
    - `sort()` / `sort!()` - Sort elements (new sorted list / sort in place)
    - `reverse()` / `reverse!()` - Reverse order (new reversed list / reverse in place)
    - `uniq()` / `uniq!()` - Remove duplicates (new list / remove in place)
    - `compact()` / `compact!()` - Remove none values (new list / remove in place)
    - `each(fn)` - Execute function for each element (always iterates, no mutation)
    - `reduce(initial, fn)` - Reduce to single value
  - **Generators**:
    - `generate(start, end, step)` - Range mode
    - `generate(start, end, fn)` - function mode
    - `upto(n)` - Generate from 0 to n
  - **Type Conversion**: `to_bool()`, `to_string()`

**Available Transformations**: `double`, `square`, `negate`, `increment`/`inc`, `decrement`/`dec`, `upper`/`up`, `lower`/`down`, `trim`, `reverse`, `to_string`/`str`, `to_num`/`num`, `to_bool`/`bool`
  - Example: `[1, 2, 3].map(:double)` returns new list `[2, 4, 6]`

**Available Predicates**: `positive`/`pos`, `negative`/`neg`, `zero`, `even`, `odd`, `empty`, `non_empty`, `uppercase`, `lowercase`, `alphabetic`/`alpha`, `numeric`/`digit`, `is_string`/`string`, `is_number`/`number`, `is_bool`/`boolean`, `is_list`/`list`, `truthy`, `falsy`, `frozen`, `unfrozen`/`mutable` (All use symbols, eg :pos, :frozen)

#### Hashes (`hash`)
- Key-value collections stored as hash graphs
- Type constraints: `hash<num>` (value type constraint)
- Multi-line literals supported with trailing commas
- **Mutation**: Hash mutation occurs through index assignment: `hash["key"] = value`
- Methods:
  - **Inspection**: `has_key(key)`, `keys()`, `values()`, `size()`, `is_empty()`
  - **Type Conversion**: `to_bool()`, `to_string()`
- Index access: `hash["key"]` returns value (returns `none` in lenient mode if key not found)
- Index assignment: `hash["key"] = value` creates or updates key-value pair

#### Graphs (`graph`)
- Universal graph type representing nodes and edges
- Created with: `graph { type: :type_name }` or `graph "name" { ... }`
- **Index access**: `graph["node_id"]` returns node (consistent with hash/list syntax)
- **Index assignment**: `graph["node_id"] = value` adds or updates node
- Methods:
  - **Access** (hash-like): `keys()`, `values()`, `has_key(node_id)`
  - **Nodes/Edges**: `nodes()`, `edges()`
  - **Mutation**: `add_node(id, value)`, `add_edge(from, to, edge_type, weight?)`, `insert(value, parent?)`
  - **Querying**: `has_path(from, to)`, `count_roots()`, `get_children_count(node_id)`
  - **Type Conversion**: `size()`, `is_empty()`
- **Method Details**:
  - `keys()` - Returns list of node IDs (same as hash.keys())
  - `values()` - Returns list of node objects
  - `nodes()` - Returns list of node objects (alias for values())
  - `edges()` - Returns list of edge objects
- Rule-based governance system (see below)
- Edge types: `Child`, `Next`, `Depends`, `Custom(name)`
- **Inheritance**: Graphs can inherit characteristics from other graphs:
  ```graphoid
  base_graph = graph "base" { type: :dag }
  # ... add nodes and methods to base_graph ...

  derived = graph { inherits: base_graph }
  # derived graph inherits structure and methods from base_graph
  ```
  While Graphoid is not an object-oriented language, this allows simulation of inheritance patterns by creating graphs with data and methods that can be extended.

**Trees**: Trees are not a distinct data type—they are graphs with tree-specific rules applied. For convenience, the syntax `tree{}` creates a graph with the `:tree` ruleset:

```graphoid
# These are equivalent:
my_tree = tree{}
my_tree = graph{}.with_ruleset(:tree)

# With type constraint:
numbers = tree<num>{}
numbers = graph<num>{}.with_ruleset(:tree)
```

The `:tree` ruleset includes rules like `no_cycles`, `single_root`, and `connected`.

### Complex Types

#### functions (`function`)
Defined with `fn` keyword:

```Graphoid
fn name(param1, param2) {
    # function body
    return value
}

# Optional parameters with defaults
fn greet(name, greeting = "Hello") {
    return greeting + ", " + name
}
```

#### Lambdas (`lambda`)
Anonymous functions with arrow syntax:

```Graphoid
double = x => x * 2
add = (x, y) => x + y

# Multi-statement lambdas (block bodies)
process = x => {
    temp = x * 2
    return temp + 1
}
```

#### Trailing-block syntax for lambdas

Graphoid provides Ruby/Smalltalk-like trailing block syntax as syntactic sugar for a function's last lambda parameter. This improves ergonomics for iteration, transactions, and DSL-style APIs. Trailing blocks are ordinary lambdas.

**Syntax (desugaring)**:
```Graphoid
list.each { |x, i| print(i.to_string() + ": " + x) }
# == list.each((x, i) => { print(i.to_string() + ": " + x) })

numbers.map { |x| x * 2 }.filter { |x| x > 10 }
# == numbers.map((x) => x * 2).filter((x) => x > 10)

# Empty parameter list
5.times { || print("Hello") }
# == 5.times(() => { print("Hello") })
```

**Function usage**:
```Graphoid
fn times(n, block: lambda = () => {}) {
    i = 0
    while i < n {
        block(i)
        i = i + 1
    }
}
5.times { |i| print(i) }

fn with_transaction(graph, block: lambda) {
  begin_tx(graph)
  try { yield(); commit_tx(graph) } catch Error { rollback_tx(graph); raise }
}
# yield() == block()
```

**Semantics**:
- No nonlocal returns: `return` inside the block returns from the block only
- Arity follows error modes: `:strict` errors on mismatch; `:lenient` ignores extras, fills missing with `none`
- Parser: trailing `{ ... }` after a call is parsed as a lambda argument
- Parameter list: `|params|` declares block parameters, `||` for empty parameter list
- Disambiguation: not confused with pattern matching since that form doesn't appear as call argument

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

```Graphoid
# Define custom validation function
fn validate_employee_graph(graph) {
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

```Graphoid
# Check if graph has a rule
has_sorted = my_tree.has_rule("sorted")

# Get all active rules
rules = my_graph.rules()
```

---

## Graph Querying and Traversal

**Critical Feature**: Graph querying is fundamental to Graphoid's identity. The language provides five levels of sophistication, from simple navigation to complex pattern matching.

### Level 1: Simple Navigation

Basic node and edge access for straightforward queries:

```Graphoid
# Node access (same syntax as hash/list indexing)
user = graph["user_123"]                # Get node by ID
value = user.value()                    # Get node's value
attrs = user.attributes()               # Get all attributes
age = user.get_attribute("age")         # Get specific attribute

# Edge access
outgoing = user.outgoing()              # All outgoing edges
incoming = user.incoming()              # All incoming edges
friends = user.outgoing(:FRIEND)        # Edges of specific type
followers = user.incoming(:FOLLOWS)     # Incoming edges of type

# Neighbor access
neighbors = user.neighbors()            # All adjacent nodes
friends = user.neighbors(:FRIEND)       # Neighbors via FRIEND edges

# Get all nodes/edges
all_nodes = graph.nodes()               # List of all nodes
all_edges = graph.edges()               # List of all edges
```

### Level 2: Method Chaining

Fluent API for filtering and traversal:

```Graphoid
# Filter nodes
adults = graph.nodes()
    .filter(n => n.get_attribute("age") > 18)

# Filter by type
users = graph.nodes()
    .filter(n => n.type == "User")

# Traverse and filter
friends_of_friends = user
    .outgoing(:FRIEND)
    .map(edge => edge.to_node())
    .map(friend => friend.outgoing(:FRIEND))
    .flatten()
    .uniq()

# Complex chaining
active_adult_users = graph.nodes()
    .filter(n => n.type == "User")
    .filter(n => n.get_attribute("age") > 18)
    .filter(n => n.get_attribute("status") == "active")
    .map(n => n.value())
```

### Level 3: Pattern-Based Querying

Declarative pattern matching inspired by Cypher (graph query language):

```Graphoid
# Simple pattern: user with friends
results = graph.match(
    (person:User) -[:FRIEND]-> (friend:User)
)

# With predicates
adult_friendships = graph.match(
    (person:User) -[:FRIEND]-> (friend:User)
).where(person.age > 18, friend.age > 18)

# Variable-length paths
influencers = graph.match(
    (user:User) -[:FOLLOWS*1..3]-> (influencer:User)
).where(influencer.follower_count > 1000)

# Bidirectional patterns
mutual_friends = graph.match(
    (a:User) -[:FRIEND]- (b:User)
).where(a.id != b.id)

# Complex patterns with multiple relationships
results = graph.match(
    (buyer:User) -[:PURCHASED]-> (product:Product),
    (product) -[:CATEGORY]-> (cat:Category)
).where(cat.name == "Electronics")

# Return specific fields
purchases = graph.match(
    (user:User) -[:PURCHASED]-> (product:Product)
).where(user.id == "user_123")
 .return(product.name, product.price)
```

**Pattern Syntax**:
- `(node:Type)` - Node with type
- `-[:EDGE_TYPE]->` - Directed edge
- `-[:EDGE_TYPE]-` - Bidirectional edge
- `-[:EDGE*min..max]->` - Variable-length path
- `.where()` - Filter predicates
- `.return()` - Select specific fields

### Level 4: Path Queries

Specialized algorithms for path finding:

```Graphoid
# Shortest path
path = graph.shortest_path("user_A", "user_Z")
# Returns: list of node IDs forming the path

# Shortest path with edge type constraint
path = graph.shortest_path("A", "Z", edge_type: :FRIEND)

# All paths (up to max length)
all_paths = graph.all_paths("A", "Z", max_length: 5)

# Weighted shortest path (Dijkstra's algorithm)
path = graph.shortest_path("A", "Z", weighted: true)

# Path exists check
connected = graph.has_path("A", "Z")

# Distance between nodes
distance = graph.distance("A", "Z")

# All nodes within N hops
nearby = graph.nodes_within("user_123", hops: 3)

# Breadth-first traversal
bfs_order = graph.bfs("start_node")

# Depth-first traversal
dfs_order = graph.dfs("start_node")
```

### Level 5: Subgraph Operations

Extract, manipulate, and compose subgraphs:

```Graphoid
# Extract subgraph by node filter
active_users = graph.extract {
    nodes: n => n.type == "User" and n.get_attribute("active") == true
}

# Extract subgraph by edge filter
social_network = graph.extract {
    nodes: n => n.type == "User",
    edges: e => e.type == :FRIEND or e.type == :FOLLOWS
}

# Extract subgraph by multiple criteria
relevant_data = graph.extract {
    nodes: n => n.get_attribute("created_at") > cutoff_date,
    edges: e => e.weight > 0.5,
    include_orphans: false  # Exclude nodes with no edges
}

# Delete subgraph (returns new graph without matching nodes/edges)
cleaned = graph.delete {
    nodes: n => n.get_attribute("deleted") == true
}

# Delete edges only
simplified = graph.delete {
    edges: e => e.weight < 0.1
}

# Add subgraph (merge two graphs)
combined = graph_a.add_subgraph(graph_b)

# Add with conflict resolution
merged = graph_a.add_subgraph(graph_b, on_conflict: :keep_original)
merged = graph_a.add_subgraph(graph_b, on_conflict: :overwrite)

# Clone subgraph (deep copy)
backup = graph.clone()
partial_backup = graph.extract { nodes: n => n.critical == true }.clone()
```

**Subgraph Operation Options**:
- `nodes:` - Node filter predicate
- `edges:` - Edge filter predicate
- `include_orphans:` - Include/exclude isolated nodes (default: true)
- `on_conflict:` - Conflict resolution strategy (`:keep_original`, `:overwrite`, `:merge`)

### Graph-Object Nodes

Graph nodes can themselves contain graphs with callable methods:

```Graphoid
# Create a graph node that contains another graph as its value
person_data = graph {}
person_data.add_node("name", "Alice")
person_data.add_node("age", 30)
person_data.add_node("email", "alice@example.com")

# Attach method to the person data graph
fn person_data.get_name() {
    return person_data["name"].value()
}

# Add person as a node in the main graph
social_graph.add_node("person_123", person_data)

# Access nested graph-object and call methods
person = social_graph["person_123"].value()
name = person.get_name()                    # Calls method on nested graph
age = person.get_attribute("age")           # Access nested attribute
```

This enables **composition of graph structures** where nodes can be complex objects with their own behavior and structure.

### Graph Performance & Auto-Optimization

**Philosophy**: Graphs optimize themselves transparently. Users don't need to think about performance tuning.

#### Automatic Optimization (v1.0)

Graphoid graphs learn from usage patterns and automatically create optimizations:

**Auto-Indexing**:
```graphoid
social = graph { type: :directed }

# First few property lookups are slower (no index yet)
social.find_node(email: "alice@example.com")  # ~100μs
social.find_node(email: "bob@example.com")    # ~100μs

# After pattern detected (10+ lookups), index auto-created
social.find_node(email: "charlie@example.com")  # ~1μs (100x faster!)

# Check what happened
social.stats()
# => {
#   auto_indices: ["email"],
#   rationale: "Frequent property lookups (10+ times)"
# }
```

**Rule-Aware Optimization**:
```graphoid
# Declaring rules enables automatic algorithm selection
dag = graph { rules: ["no_cycles"] }

# Shortest path automatically uses topological algorithm instead of general BFS
# No manual hints needed!
path = dag.shortest_path("A", "B")
```

**Built-in Performance Features**:
- **Index-free adjacency**: O(1) neighbor lookups (nodes directly point to neighbors)
- **Auto-indexing**: Property indices created after repeated lookups (default: 10x threshold)
- **Edge type indices**: Fast traversal by edge type
- **Rule-aware algorithms**: Smarter algorithms based on declared constraints
- **Lazy evaluation**: Queries return iterators when possible

#### Explain & Statistics

**Transparency via Explain**:
```graphoid
plan = dag.explain { dag.shortest_path("A", "B") }
plan.show()

# Output:
# Execution Plan:
#   1. Topological sort (uses rule: no_cycles)
#   2. BFS from A
#   3. Path reconstruction
# Estimated cost: 50 operations
# Rules applied: ["no_cycles" → "enabled topological algorithms"]
```

**Statistics Tracking**:
```graphoid
stats = graph.stats()
# => {
#   nodes: 10000,
#   edges: 45000,
#   avg_degree: 4.5,
#   max_degree: 127,
#   auto_indices: ["user_id", "email"],
#   auto_optimizations: [
#     "user_id indexed after 15 lookups",
#     "email indexed after 12 lookups"
#   ]
# }
```

#### Manual Performance Tuning (v2.0+ Only)

**NOT IN v1.0**: Advanced manual controls deferred to future versions:
- Manual index management (create/drop indices)
- Query hints (prefer_bfs, parallelize)
- Optimization strategies (optimize_for reads vs writes)
- Query plan manipulation
- Advanced explain with suggestions

See `dev_docs/FUTURE_FEATURES.md` for v2.0+ roadmap.

**v1.0 Design**: Automatic optimization handles 90% of use cases. Manual tuning only needed for expert-level edge cases.

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

#### Freeze Control
- `no_frozen` - Reject frozen elements (throws error on attempt to add)
- `copy_elements` - Copy all elements on insertion (copies are unfrozen)
- `shallow_freeze_only` - Freeze operations only freeze the collection, not nested elements

### Generic Mapping Behaviors

Create custom value mappings using hash graphs:

```Graphoid
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

### Custom function Behaviors

```Graphoid
# Define transformation function
fn normalize_temp(value) {
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

```Graphoid
# Define condition and transform functions
fn is_string(value) {
    return value.get_type() == "string"
}

fn to_upper(value) {
    return value.upper()
}

# Apply conditional behavior
mixed_data = [42, "hello", -10, "world"]
mixed_data.add_rule(is_string, to_upper)
# Result: [42, "HELLO", -10, "WORLD"]

# With fallback for non-matching values
fn is_negative(value) {
    return value.type() == "number" and value < 0
}

fn make_positive(value) {
    return -value  # Convert negative to positive
}

fn leave_unchanged(value) {
    return value  # Fallback: non-negative values stay as-is
}

numbers.add_rule(:is_negative, :make_positive, :leave_unchanged)
# [5, -3, 0] → [5, 3, 0]  # Only -3 was transformed
```

### Rulesets (Declarative Bundle Application)

Create reusable behavior bundles:

```Graphoid
# Define ruleset
data_cleaning = [
    :none_to_zero,
    :positive,
    :round_to_int
]

# Apply to multiple containers
temperatures.add_rules(data_cleaning)
blood_pressure.add_rules(data_cleaning)
heart_rate.add_rules(data_cleaning)
```

### Behavior Management

```Graphoid
# Check if behavior exists
has_positive = list.has_rule(:positive)

# Get all active behaviors (sorted alphabetically)
behaviors = list.rules()

# Remove specific behavior
list.remove_rule(:positive)

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

#### Block-Level Conditionals

```Graphoid
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

# alternative with same precedence:
if a && b {
    # if both true
}

if a or b {
    # at least one must be true
}

# alternative with same precedence:
if a || b {
  # if either is true
}

# Complex expressions (use parentheses for clarity)
if (a == 1 or (a == 2 && b == 2) {
  # must have parens for correct evaluation
}
```

#### Inline Conditionals

Graphoid supports inline conditional expressions for concise conditional assignment:

```Graphoid
# if-then-else expression
status = if age >= 18 then "adult" else "minor"
value = if is_valid then compute() else default_value
result = if x > 0 then x * 2 else 0

# Suffix if (when condition is true)
value = compute() if condition
result = zed.round(2) if zippy > pinhead
items.append(new_item) if should_add

# Suffix unless (when condition is false)
message = "OK" unless error_occurred
process_data() unless cache.has_key("result")
status = "active" unless paused

# Nested inline conditionals (use sparingly - prefer blocks for clarity)
level = if score >= 90 then "A" else (if score >= 80 then "B" else "C")
```

**Key Features**:
- **if-then-else**: Returns the `then` value when true, `else` value when false
- **Suffix if**: Executes or evaluates only when condition is true, otherwise returns `none`
- **Suffix unless**: Executes or evaluates only when condition is false, otherwise returns `none`
- **Expressions**: Can be used anywhere an expression is expected
- **Clarity**: Prefer block conditionals for complex logic; use inline for simple cases

### Loops

```Graphoid
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
      next    # skip to next iteration
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

```Graphoid
# Integer arithmetic (0 decimal places)
precision 0 {
  # alternative  precision :int {
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
- Integer mode: `precision 0` (or `precision :int`) gives true integers with no decimal point
- Scoped semantics (localized, not global)
- Automatic restoration on block exit
- Composable (can nest contexts)

### Configuration Blocks

Control language behavior within a scope:

```Graphoid
# File-level configuration (applies to entire file)
configure { skip_none: false, decimal_places: 2 }

# Block-level configuration
configure { skip_none: true } {
    # All operations skip none values in this block
    data = [1, 2, none, 4]
    result = data.mean()  # none is skipped
}

configure { strict_types: true } {  # strict types is default
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

```Graphoid
# Import from stdlib
import "module_name"

# Import .gr file
import "path/to/file.gr"

# Import with non-standard alias (use sparingly)
import "module" as custom_name

# Module access - dot notation preferred
module.function()
module.value
```

### Creating Modules

```Graphoid
# In my_module.gr
module my_utilities
alias utils       # Built-in alias

# Define module contents
helper_value = 42

fn process(data) {
    return data * 2
}

fn validate(input) {
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

```Graphoid
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

## Project Structure and Organization

### Project Manifest (`graphoid.toml`)

Multi-file Graphoid projects use a **manifest file** (`graphoid.toml`) to define project metadata, dependencies, and build settings:

```toml
[project]
name = "my_application"
version = "1.0.0"
authors = ["Alice Developer <alice@example.com>"]
description = "A sample Graphoid application"
license = "MIT"
graphoid_version = "1.0"

[dependencies]
# External packages (future - package manager)
# http_client = "2.1.0"
# graph_algorithms = "^1.5"

[dev_dependencies]
# Dependencies only for testing/development
# test_helpers = "1.0"

[build]
entry_point = "src/main.gr"
output_dir = "build/"
include = ["src/**/*.gr", "lib/**/*.gr"]
exclude = ["**/*.spec.gr", "samples/**"]

[test]
test_pattern = "**/*.spec.gr"
coverage_threshold = 80
```

### Standard Project Layout

```
my_project/
├── graphoid.toml           # Project manifest
├── README.md               # Project documentation
├── src/                    # Source code
│   ├── main.gr             # Entry point
│   ├── app/                # Application modules
│   │   ├── server.gr
│   │   ├── routes.gr
│   │   └── handlers.gr
│   ├── models/             # Data models
│   │   ├── user.gr
│   │   └── product.gr
│   └── utils/              # Utilities
│       ├── logging.gr
│       └── validation.gr
├── lib/                    # Reusable libraries
│   └── custom_graphs/
│       └── social_network.gr
├── specs/                  # Test files
│   ├── app/
│   │   ├── server.spec.gr
│   │   └── routes.spec.gr
│   └── models/
│       └── user.spec.gr
├── samples/                # Example programs
│   └── demo.gr
├── docs/                   # Documentation
│   └── api.md
└── build/                  # Compiled output (generated)
    └── main.grc
```

### Module Resolution

Graphoid resolves imports using a **clear precedence order**:

1. **Relative paths**: `import "./utils/logging.gr"` or `import "../shared/config.gr"`
2. **Project modules**: `import "app/server"` (looks in `src/app/server.gr`)
3. **Standard library**: `import "json"`, `import "http"` (built-in modules)
4. **External packages**: `import "graph_algorithms"` (from dependencies, future)

```Graphoid
# Relative import (from same directory)
import "./helpers.gr"

# Relative import (from parent directory)
import "../config.gr"

# Project module (searches src/, lib/)
import "models/user"        # src/models/user.gr

# Standard library
import "json"               # Built-in JSON module

# External package (future)
import "http_client"        # From dependencies in graphoid.toml
```

### Multi-File Project Example

**src/main.gr**:
```Graphoid
import "app/server"
import "utils/logging"

fn main() {
    logging.info("Starting application...")
    server.start(port: 8080)
}
```

**src/app/server.gr**:
```Graphoid
module server

import "app/routes"
import "../utils/logging"

fn start(port) {
    logging.info("Server starting on port " + port.to_string())
    # ... server implementation ...
}
```

**src/app/routes.gr**:
```Graphoid
module routes

import "../models/user"

fn setup_routes() {
    # ... route definitions ...
}
```

**src/models/user.gr**:
```Graphoid
module user

fn create(name, email) {
    return {
        "id": generate_id(),
        "name": name,
        "email": email,
        "created_at": time.now()
    }
}

fn validate(user_data) {
    # ... validation logic ...
}
```

### Build Commands

```bash
# Run the project (executes entry_point)
graphoid run

# Run specific file
graphoid run src/main.gr

# Run tests
graphoid spec

# Run specific test file
graphoid spec specs/models/user.spec.gr

# Build project (compile/bundle)
graphoid build

# Clean build artifacts
graphoid clean

# Check syntax without running
graphoid check
```

### Namespace Organization

**Best Practices**:
- Keep related code in the same directory
- Use descriptive module names matching file paths
- One primary export per file (single responsibility)
- Group tests with source files or in parallel `specs/` directory
- Avoid deeply nested directories (3-4 levels max)

**File Naming Conventions**:
- Source files: `snake_case.gr`
- Test files: `snake_case.spec.gr`
- Module names: match file name (e.g., `user_model.gr` → `module user_model`)

---

## functional Programming

### Method Chaining

All functional operations return new values and can be chained:

```Graphoid
result = numbers
    .filter(:even)
    .map(:double)
    .map(:to_string)
# [1, 2, 3, 4, 5] → [2, 4] → [4, 8] → ["4", "8"]
```

### Transformations

Named transformations for common operations:

```Graphoid
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

```Graphoid
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
- **State**: `frozen`, `unfrozen`/`mutable`
- **General**: `truthy`, `falsy`

### Lambda Expressions

Custom transformation functions:

```Graphoid
# With lambdas
numbers.map(x => x * 3)
numbers.filter(x => x > 10)

# With named functions
fn is_prime(n) {
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

```Graphoid
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

```Graphoid
# Check if edge operation is allowed
can_add = my_list.can_add_edge(2, 0, "circular")  # Returns false + reason

# Get current edge rules
rules = my_list.get_edge_rules()  # ["no_list_cycles", "same_structure_only"]
```

---

## Standard Library

### Math & Numbers

#### Constants Module
```Graphoid
import "constants"

# Mathematical constants
pi = constants.pi              # 3.141592653589793
e = constants.e                # 2.718281828459045
tau = constants.tau            # 6.283185307179586 (2π)
phi = constants.phi            # 1.618033988749895 (golden ratio)
sqrt2 = constants.sqrt2        # √2
sqrt3 = constants.sqrt3        # √3

# Angle conversions
deg_to_rad = constants.deg_to_rad  # π/180
rad_to_deg = constants.rad_to_deg  # 180/π

# Physical constants
c = constants.c                # 299792458 m/s (speed of light)
G = constants.G                # 6.67430e-11 (gravitational constant)
```

#### Number Methods
All numbers have built-in methods:

**Basic Math**:
- `sqrt()` - Square root
- `abs()` - Absolute value
- `log()` - Natural logarithm
- `log(base)` - Logarithm with base

**Rounding with Decimal Places**:
```graphoid
pi = 3.14159
pi.up()         # 4 (ceiling, no decimals)
pi.up(2)        # 3.15 (ceiling to 2 decimal places)
pi.down()       # 3 (floor, no decimals)
pi.down(2)      # 3.14 (floor to 2 decimal places)
pi.round()      # 3 (round to nearest integer)
pi.round(2)     # 3.14 (round to 2 decimal places)
```

**Rounding to Powers of 10**:
```graphoid
345.round(:nearest_ten)        # 350
345.round(:nearest_hundred)    # 300
3456.round(:nearest_thousand)  # 3000

345.up(:nearest_hundred)       # 400 (ceiling to nearest 100)
345.down(:nearest_hundred)     # 300 (floor to nearest 100)
```

**Available Symbols**: `:nearest_ten`, `:nearest_hundred`, `:nearest_thousand`, `:nearest_million`

### Randomness

#### Random Module
```Graphoid
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
```Graphoid
import "time"

# Create time values
current = time.now()                          # Current time
today = time.today()                         # Start of today
birthday = time.from_numbers(1990, 12, 25)
meeting = time.from_numbers(2025, 1, 15, 14, 30, 0)
parsed = time.from_string("2025-01-15T14:30:00Z")
from_ts = time.from_timestamp(1704067200)

# Time value methods
current.to_string()     # ISO format
current.to_num()        # Unix timestamp
current.type()      # "time"

# Extract components
components = time.time_numbers(current)
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
```Graphoid
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
```Graphoid
text = "Hello World 123! Contact support@example.com"

# Unified contains
text.contains(:any, :digits)          # true (has digits)
text.contains(:all, :letters, :digits) # true (has both)
text.contains(:only, :letters, :spaces) # false

# Unified extract
numbers = text.extract(:numbers)       # ["123"]
words = text.extract(:words)           # ["Hello", "World", "Contact"]
emails = text.extract(:emails)         # ["support@example.com"]

# Unified count
digit_count = text.count(:digits)      # 3
word_count = text.count(:words)        # 3

# Find first
pos = text.find_first(:digits)         # Position of first digit
```

**Available Patterns**: `:digits`/`:numbers`, `:letters`, `:uppercase`, `:lowercase`, `:spaces`/`:whitespace`, `:punctuation`, `:symbols`, `:alphanumeric`, `:words`, `:emails`

### File I/O

#### IO Module
```Graphoid
import "io"

# Read operations (auto-close on EOF)
read_handle = io.open("data.txt", "r")
content = read_handle.read()        # Read all, auto-closes
# read_handle.read()                # Error: exhausted

# Incremental reading
input = io.open("large.txt", "r")
while true {
    line = input.read_line()
    if line == :EOF { break }       # EOF, auto-closed
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
```Graphoid
import "json"

# Parse JSON
data = json.parse('{"name": "Alice", "age": 30}')
name = data["name"]

# Stringify to JSON
json_str = json.stringify(data)
```

#### CSV Module
```Graphoid
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
```Graphoid
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
```Graphoid
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
```Graphoid
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
```Graphoid
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
```Graphoid
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

### Element-Wise Operations

Graphoid supports **element-wise (vectorized) operations** on lists using the dot-prefix operator syntax:

```Graphoid
l = [1, 2, 3]

# Element-wise arithmetic
l .* 3         # [3, 6, 9]     - multiply each element by 3
l .+ 10        # [11, 12, 13]  - add 10 to each element
l .- 1         # [0, 1, 2]     - subtract 1 from each element
l ./ 2         # [0.5, 1, 1.5] - divide each element by 2
l .// 2        # [0, 1, 1]     - integer division each element
l .% 2         # [1, 0, 1]     - modulo each element
l .^ 2         # [1, 4, 9]     - square each element

# Element-wise with two lists (zip operation)
a = [1, 2, 3]
b = [4, 5, 6]
a .+ b         # [5, 7, 9]     - element-wise addition
a .* b         # [4, 10, 18]   - element-wise multiplication
a .^ b         # [1, 32, 729]  - element-wise exponentiation

# Element-wise comparison (returns list of booleans)
values = [10, 20, 30]
values .> 15   # [false, true, true]
values .< 25   # [true, true, false]
values .== 20  # [false, true, false]

# Chaining element-wise operations
result = [1, 2, 3]
    .* 2        # [2, 4, 6]
    .+ 10       # [12, 14, 16]
    ./ 2        # [6, 7, 8]
```

**Key Features**:
- **Concise syntax**: Dot-prefix (`.`) before operator enables element-wise behavior
- **Scalar operations**: `list .op scalar` applies operation to each element
- **Vector operations**: `list1 .op list2` applies operation pairwise (zip)
- **All operators**: Works with all binary operators (`+`, `-`, `*`, `/`, `//`, `%`, `^`, comparisons)
- **Returns new list**: Element-wise operations always return a new list (immutable)
- **Length mismatch**: For two-list operations, result length is the shorter list

**Precedence**: Element-wise operators have the same precedence as their non-element-wise equivalents.

**Common Use Cases**:
```Graphoid
# Scale values
temperatures_celsius = [0, 10, 20, 30]
temperatures_fahrenheit = temperatures_celsius .* 1.8 .+ 32
# [32, 50, 68, 86]

# Normalize data
values = [100, 200, 300]
normalized = values ./ 100    # [1, 2, 3]

# Boolean masking (filter by condition)
data = [5, 15, 25, 35]
mask = data .> 20             # [false, false, true, true]

# Combine with filter
high_values = data.filter((data .> 20))
```

### Mutation Operator Convention

Graphoid uses a **two-version method system** for operations that could mutate data:

- **Immutable methods** (default, no suffix): Return a new copy, leave original unchanged
- **Mutating methods** (`!` suffix): Modify in place, return none or the modified collection

This convention provides clarity at the call site and encourages immutable-first programming.

```Graphoid
# Immutable (best practice - returns new list)
sorted = original.sort()      # original unchanged, sorted is new list
reversed = items.reverse()    # items unchanged, reversed is new list
unique = values.uniq()        # values unchanged, unique is new list

# Mutating (explicit intent with ! suffix)
original.sort!()              # original is now sorted, returns none
items.reverse!()              # items is now reversed, returns none
values.uniq!()                # values now has duplicates removed, returns none

# Works for all collection transformations
capitalized = names.map(:upper)    # New list with uppercase names
names.map!(:upper)                 # names now contains uppercase values

filtered = numbers.filter(:even)   # New list with even numbers only
numbers.filter!(:even)             # numbers now contains only even values

# Works with behavior/rule operations
copy = data.freeze()          # New immutable copy
data.freeze!()                # data is now immutable
```

**Key Principles**:
- **Immutable by default**: Prefer non-mutating methods for functional style
- **Explicit mutation**: The `!` makes mutation visible and intentional
- **Absolute consistency**: ALL methods that could mutate follow this pattern
- **Return value**: Mutating methods typically return `none` to discourage chaining
- **Safety**: Easier to reason about data flow when mutation is explicit

**Which Methods Have Both Versions?**

Any method that transforms or reorders a collection:
- `sort` / `sort!`
- `reverse` / `reverse!`
- `uniq` / `uniq!` (remove duplicates)
- `map` / `map!`
- `filter` / `filter!`
- `reject` / `reject!`
- `compact` / `compact!` (remove none values)
- `freeze` / `freeze!` (make immutable)

Methods that only add/remove elements use mutation by nature:
- `append(value)`, `prepend(value)`, `insert(index, value)` - always mutate
- `remove(value)`, `remove_at_index(index)`, `pop()` - always mutate
- `clear()` - always mutates

### Immutability with `.freeze()`

Any collection can be made **permanently immutable** using the freeze mechanism:

```Graphoid
# Create frozen copy (immutable version)
data = [1, 2, 3]
frozen = data.freeze()      # Returns new immutable list (deep freeze)
data.append(4)              # OK - original is still mutable
frozen.append(5)            # ERROR: Cannot mutate frozen collection

# Freeze in place (make original immutable)
config = {"host": "localhost", "port": 8080}
config.freeze!()            # config is now immutable (deep freeze)
config["debug"] = true      # ERROR: Cannot mutate frozen collection

# Shallow freeze (only freeze the collection, not nested elements)
data = [[1, 2], [3, 4]]
frozen = data.freeze(shallow: true)
frozen.append([5, 6])       # ERROR - list structure is frozen
frozen[0].append(3)         # OK - nested lists are not frozen

# Query freeze status
is_frozen = data.is_frozen()      # Is this collection frozen?
has_frozen = data.has_frozen()    # Does this contain frozen elements?

# Works on all collections
frozen_list = items.freeze()
frozen_hash = settings.freeze()
frozen_graph = network.freeze()
```

#### Two Levels of Freezing

**Collection-Level Freeze**: The collection structure cannot change
- Can't add, remove, or replace elements
- Can't change the edges in the underlying graph
- Index assignment blocked: `frozen_list[0] = x` → ERROR

**Element-Level Freeze**: Individual elements are frozen
- Elements themselves cannot be modified
- But can be replaced if collection is not frozen

```Graphoid
# Mixed frozen and unfrozen elements
frozen_item = [1, 2, 3].freeze()
mutable_item = [4, 5, 6]

my_list = [frozen_item, mutable_item]  # List is mutable

# List structure operations - OK (list not frozen)
my_list.append([7, 8, 9])              # OK - adding element
my_list[0] = [10, 11, 12]              # OK - replacing frozen element

# Element modification - depends on element
my_list[0].append(4)                   # ERROR - element is frozen
my_list[1].append(7)                   # OK - element is mutable

# Now freeze the list
my_list.freeze!()

# Structure operations - blocked
my_list.append([13, 14])               # ERROR - list is frozen
my_list[0] = [15, 16]                  # ERROR - list is frozen

# Element modification - depends on element
my_list[0].append(4)                   # ERROR - element is frozen
my_list[1].append(7)                   # ALSO ERROR - list is frozen, blocks access
```

**Important**: When a collection is frozen, you cannot modify elements through it, even if the elements themselves are mutable. The frozen collection blocks all mutation paths.

#### Deep Freeze vs Shallow Freeze

**Deep Freeze (default)**:
```Graphoid
inner = [1, 2, 3]
outer = [inner, 4, 5]
frozen = outer.freeze()    # Deep freeze

# Both outer and inner are now frozen
frozen.append(6)           # ERROR - outer is frozen
frozen[0].append(4)        # ERROR - inner is frozen
```

**Shallow Freeze (explicit)**:
```Graphoid
inner = [1, 2, 3]
outer = [inner, 4, 5]
frozen = outer.freeze(shallow: true)

# Only outer is frozen, inner is not
frozen.append(6)           # ERROR - outer is frozen
frozen[0].append(4)        # OK - inner is mutable
```

#### Freeze Control with Rules

Collections can use rules to control freeze behavior:

**`:no_frozen` Rule** - Reject frozen elements:
```Graphoid
my_list = []
my_list.add_rule(:no_frozen)

frozen_item = [1, 2, 3].freeze()
my_list.append(frozen_item)   # ERROR - rule violation
# Error: Rule violation: no_frozen
# Cannot add frozen elements to this collection
```

**`:copy_elements` Rule** - Always copy on insert:
```Graphoid
my_list = []
my_list.add_rule(:copy_elements)

frozen_item = [1, 2, 3].freeze()
my_list.append(frozen_item)   # OK - a copy is added

# The copy is NOT frozen
my_list[0].append(4)          # OK - [1, 2, 3, 4]

# Original is unchanged and still frozen
frozen_item.append(4)         # ERROR - original is frozen
```

**`:shallow_freeze_only` Rule** - Prevent deep freeze:
```Graphoid
inner = [1, 2, 3]
outer = [inner, 4, 5]
outer.add_rule(:shallow_freeze_only)

frozen = outer.freeze()       # Shallow freeze despite no parameter

# Outer is frozen, inner is not
frozen.append(6)              # ERROR - outer is frozen
frozen[0].append(4)           # OK - inner is mutable
```

#### Query Methods

```Graphoid
# Check if collection itself is frozen
if data.is_frozen() {
    print("Cannot modify")
}

# Check if collection contains any frozen elements (boolean)
if data.has_frozen() {
    print("Contains frozen elements")
}

# Get detailed freeze information
info = data.has_frozen(:verbose)
# Returns hash: {
#   "has_frozen": true,
#   "frozen_count": 5,
#   "frozen_collections": 2,
#   "frozen_primitives": 3
# }

# Use existing methods with :frozen predicate
frozen_items = data.filter(:frozen)        # Get all frozen elements
unfrozen_items = data.filter(:unfrozen)    # Get all unfrozen elements

# Find frozen elements
first_frozen_idx = data.index_of(:frozen)  # Index of first frozen element
all_frozen = data.filter(:frozen)          # List of frozen elements

# Count frozen elements
frozen_count = data.filter(:frozen).size()

# Check if specific element is frozen
if data[3].is_frozen() {
    print("Element at index 3 is frozen")
}
```

#### Interaction with Behaviors

When a collection has behaviors and you try to add a frozen element:

**Without `:copy_elements`**:
```Graphoid
my_list = []
my_list.add_rule(:uppercase)  # Transform strings to uppercase

frozen_item = "hello".freeze()
my_list.append(frozen_item)   # ERROR - can't transform frozen element
# Error: Cannot apply behavior to frozen element
# Suggestion: Use :copy_elements rule to transform copies
```

**With `:copy_elements`**:
```Graphoid
my_list = []
my_list.add_rule(:uppercase)
my_list.add_rule(:copy_elements)

frozen_item = "hello".freeze()
my_list.append(frozen_item)   # OK - copy is transformed

print(my_list[0])             # "HELLO" - copy was transformed
print(frozen_item)            # "hello" - original unchanged
```

#### Use Cases

```Graphoid
# Protect configuration from accidental changes
config = load_config("app.conf")
config.freeze!()

# Create immutable constants
PRIMES = [2, 3, 5, 7, 11, 13].freeze()

# Freeze after initialization
cache = {}
# ... populate cache ...
cache.freeze!()    # No further modifications allowed

# Controlled sharing with partial freezing
shared_cache = {}
shared_cache.add_rule(:no_frozen)  # Don't accept frozen data
# ... use shared_cache safely ...

# Thread-safe data sharing (future)
shared_data = compute_results()
shared_data.freeze!()
# Can now safely share across threads

# Copy-on-write pattern
master_config = load_defaults().freeze()
user_config = []
user_config.add_rule(:copy_elements)
user_config.append(master_config)  # Gets mutable copy
user_config[0]["custom_setting"] = true  # OK
```

#### Frozen Collection Errors

```Graphoid
frozen = [1, 2, 3].freeze()
frozen.append(4)
# Error: FrozenCollectionError
#   at line 2, column 8 in example.gr
#
#   2 | frozen.append(4)
#           ^^^^^^
#
# Cannot mutate frozen collection
# Collection was frozen at line 1, column 20
#
# Suggestion: Create a mutable copy with .clone() first
```

```Graphoid
frozen_item = [1, 2, 3].freeze()
my_list = []
my_list.add_rule(:no_frozen)
my_list.append(frozen_item)
# Error: Rule violation: no_frozen
#   at line 4, column 9 in example.gr
#
#   4 | my_list.append(frozen_item)
#              ^^^^^^
#
# Cannot add frozen element to collection with :no_frozen rule
# Element was frozen at line 1, column 28
#
# Suggestion: Use :copy_elements rule to add a mutable copy
```

#### Freeze Behavior Summary

| Scenario | Collection Frozen? | Element Frozen? | Can Replace Element? | Can Modify Element? |
|----------|-------------------|-----------------|---------------------|---------------------|
| Mutable collection, mutable element | No | No | ✅ Yes | ✅ Yes |
| Mutable collection, frozen element | No | Yes | ✅ Yes | ❌ No |
| Frozen collection, mutable element | Yes | No | ❌ No | ❌ No* |
| Frozen collection, frozen element | Yes | Yes | ❌ No | ❌ No |

\* Frozen collection blocks all mutation paths, even to mutable elements

### Optional Parentheses for Zero-Argument Methods

```Graphoid
# Both forms are valid
size = items.size()     # With parentheses
size = items.size       # Without (property-like access)

# Multi-argument methods always need parentheses
items.insert(0, "first")
```

### Type Inference

```Graphoid
# Inferred types (recommended when obvious)
name = "Alice"              # string
age = 25                    # num
active = true               # bool
items = [1, 2, 3]           # list
config = {"host": "localhost"}  # hash

# Explicit types (when needed for clarity or constraints)
string username = "Bob"
num max_age = 100
bool is_valid = false
list<num> scores = [95, 87, 92]           # Runtime type assertion (optional)
hash<string> settings = {"theme": "dark"}  # Runtime type assertion (optional)
tree<num> numbers = tree{}                 # Runtime type assertion (optional)

# ❌ FORBIDDEN - See NO_GENERICS_POLICY.md
# hash<string, num> data = {}              # Multiple params - NEVER ALLOWED
# fn process<T>(x: T) { ... }              # Generic functions - NEVER ALLOWED
# class Container<T> { ... }               # Generic classes - NEVER ALLOWED
```

### Multi-line Literals

```Graphoid
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
    "cherry",    # Trailing comma OK but not required
]

# reduces diff noise
```

### Index Access and Assignment

```Graphoid
# Lists
first = items[0]        # Get first element
last = items[-1]        # Get last element
items[0] = 99           # Set element

# Hashes
host = config["host"]   # Get value by key
config["port"] = 8080   # Set or create key

# Graphs
user = graph["user_123"] # Get node by ID
graph["user_456"] = user_data  # Add or update node (equivalent to add_node)

# Strings
char = text[0]          # Get character at index
```

### Pattern Matching

Functions can use pattern matching with pipe syntax for concise, expressive case handling:

```Graphoid
# Pattern matching with pipes
fn factorial(n) {
    |0| => 1
    |1| => 1
    |x| => x * factorial(x - 1)
}

# String patterns
fn get_sound(animal) {
    |"dog"| => "woof"
    |"cat"| => "meow"
    |"cow"| => "moo"
    # Automatic fallthrough returns none
}

# Multiple conditions with guards
fn classify(n) {
    |x| if x < 0 => "negative"
    |0| => "zero"
    |x| if x > 0 => "positive"
}
```

**Key Features**:
- Pipe syntax `|pattern| =>` clearly distinguishes pattern matching from lambdas
- Pattern guards with `if` conditions: `|x| if x > 0 => ...`
- Automatic fallthrough to `none` if no pattern matches
- Perfect for recursive functions
- Functional elegance with imperative practicality

**Disambiguation**: The `|pattern| =>` syntax distinguishes pattern matching from lambda expressions (`x => x * 2`), preventing parser ambiguity.

---

## Design Principles

### KISS Principle (Keep It Simple, Stupid!)
- Despises unnecessary verbiage and redundant syntax
- One clear, obvious way to do things
  - But sometimes with reasonable alternatives for developer happiness
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
- Long-term goal is to have all code written in Graphoid.

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

### Numerous well-annotated example files
- In 'samples' folder
- Show patterns, practices and correct syntax that help users learn
- Demonstrate best practices with clear examples.
- Give clear explanations both for users and for AI training purposes.
- When changes make sample files obsolete, update or prune them.

---

## Implementation Requirements

### Core Runtime (Must be in Native Code - Rust - for now)

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

The following may initially be implemented in Rust but exposed as modules:

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

## REPL and CLI Parity

**Critical Requirement**: The REPL (interactive shell) and CLI (file execution) **must have complete feature parity**. Anything that works in one must work in the other.

### Parity Principles

1. **No REPL-Only Features**: All language features must work identically in files
2. **No CLI-Only Features**: All file-based code must work in the REPL
3. **Consistent Behavior**: Same code produces same results regardless of execution mode
4. **Unified Semantics**: REPL is not a separate "mode" - it's the same language

### What This Means

**✅ ALLOWED**:
```Graphoid
# REPL conveniences that don't change semantics
graphoid> x = 5           # Automatic printing of expression result
=> 5

graphoid> [1, 2, 3].map(:double)
=> [2, 4, 6]

# But the same code works in files (just without auto-printing)
x = 5
result = [1, 2, 3].map(:double)
```

**✅ ALLOWED**:
- Multiline input in REPL (with proper continuation detection)
- Command history and tab completion (UI features)
- Auto-printing of expression results (presentation, not semantics)
- Variable inspection commands (e.g., `.vars`, `.type x`)

**❌ NOT ALLOWED**:
- Special REPL-only syntax that doesn't work in files
- Different scoping rules in REPL vs files
- Magic variables only available in REPL
- Different import resolution in REPL vs CLI

### Examples of Parity

**Function Definitions**:
```Graphoid
# Works in REPL:
graphoid> fn add(a, b) { return a + b }
graphoid> add(2, 3)
=> 5

# Works identically in file:
fn add(a, b) { return a + b }
result = add(2, 3)
print(result)  # 5
```

**Module Imports**:
```Graphoid
# Works in REPL:
graphoid> import "json"
graphoid> data = json.parse('{"key": "value"}')

# Works identically in file:
import "json"
data = json.parse('{"key": "value"}')
```

**Multi-Statement Blocks**:
```Graphoid
# Works in REPL (multiline mode):
graphoid> if x > 10 {
    ...>     print("Large")
    ...> } else {
    ...>     print("Small")
    ...> }

# Works identically in file:
if x > 10 {
    print("Large")
} else {
    print("Small")
}
```

### Implementation Guidelines

1. **Shared Parser**: REPL and CLI use the **exact same parser**
2. **Shared Execution Engine**: No separate REPL interpreter
3. **State Management**: REPL maintains state across lines, but uses same semantics
4. **Error Handling**: Identical error messages and behavior
5. **Module Loading**: Same import resolution rules

### REPL-Specific Commands

REPL **meta-commands** (starting with `.`) are allowed for inspection and control, but they don't affect language semantics:

```Graphoid
graphoid> .help         # Show help
graphoid> .vars         # List all variables
graphoid> .type x       # Show type of x
graphoid> .clear        # Clear REPL state
graphoid> .exit         # Exit REPL
graphoid> .load file.gr # Load and execute file
```

**Important**: Meta-commands are REPL UI features, not language features. They don't appear in `.gr` files.

### Testing Parity

Every language feature must be tested in both modes:

```rust
// In implementation tests
#[test]
fn test_feature_in_file() {
    let result = run_file("test.gr");
    assert_eq!(result, expected);
}

#[test]
fn test_feature_in_repl() {
    let mut repl = Repl::new();
    let result = repl.eval("same code as test.gr");
    assert_eq!(result, expected);
}
```

### Why This Matters

- **Consistency**: Users don't have to learn two different modes
- **Predictability**: Code works the same everywhere
- **Copy-Paste Friendly**: REPL sessions can be saved as files directly
- **Teaching**: Examples in REPL translate directly to production code
- **Debugging**: Debugging in REPL reflects actual program behavior

**Bad Example (Other Languages)**:
```python
# Python REPL has auto-printing:
>>> [1, 2, 3]
[1, 2, 3]

# But in a file, this does nothing (no print):
[1, 2, 3]  # Silent
```

**Good Example (Graphoid)**:
```Graphoid
# REPL shows result:
graphoid> [1, 2, 3]
=> [1, 2, 3]

# File with explicit print produces same output:
print([1, 2, 3])  # [1, 2, 3]

# But both are valid Graphoid - just different presentation
```

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
    function(Box<functionData>), // Name, params, body, captures
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

### Philosophy

**Core Principles**:
1. **Errors are exceptional** - Not for expected outcomes or control flow
2. **Safe defaults** - Operations choose sensible fallbacks when possible
3. **Explicit handling** - Traditional try/catch for exceptional cases
4. **Configurable strictness** - Different contexts need different behavior
5. **No magic operators** - No `&.` safe navigation or similar tricks
6. **Clear over clever** - Predictable behavior beats brevity
7. **No throw for control flow** - Use `return` and pattern matching instead

### Return Values vs Exceptions

Some operations naturally return success/failure indicators (not exceptions):

```Graphoid
# File operations return handle or none
handle = io.open("config.txt", "r")
if handle == none {
    print("File not found")
} else {
    content = handle.read()
}

# Hash access returns value or none
config = {"host": "localhost"}
port = config["port"]  # Returns none if key doesn't exist
if port == none {
    port = 8080  # Default
}

# List find returns index or none
items = ["a", "b", "c"]
idx = items.index_of("d")  # Returns none if not found

# Type conversions return none on failure (lenient by default)
num = "abc".to_num()  # Returns none (not an error)
```

**When to use return values** (not exceptions):
- Optional values (hash keys, find operations)
- Expected failures (file not found, conversion failures)
- Safe defaults make sense

**When to use exceptions**:
- Contract violations (type constraints, graph rules)
- Programming errors (index out of bounds in strict mode)
- Unexpected failures (network errors, disk full)

### Exception Handling Syntax

Traditional try/catch/finally:

```Graphoid
try {
    data = json.parse(user_input)
    result = process(data)
} catch JSONParseError as e {
    print("Invalid JSON: " + e.message())
    result = default_value
} catch ProcessingError as e {
    print("Processing failed: " + e.message())
    result = none
} finally {
    cleanup_resources()
}

# Catch any error
try {
    risky_operation()
} catch Error as e {
    print("Something went wrong: " + e.message())
}

# No catch - just finally
try {
    operation()
} finally {
    cleanup()
}
```

**No `throw` keyword** - Errors are raised by runtime, not user code (see Raising Errors below).

### Configurable Error Modes

Error behavior can be configured at multiple scopes:

```Graphoid
# File-level configuration (top of file)
configure {
    error_mode: :strict,
    bounds_checking: :strict
}

# Block-level configuration
configure { error_mode: :lenient } {
    # Operations in this block use lenient error handling
    list = [1, 2, 3]
    x = list[10]  # Returns none instead of error

    y = "invalid".to_num()  # Returns none instead of error
}

# Nested configurations
configure { bounds_checking: :lenient } {
    data = [1, 2, 3]

    configure { bounds_checking: :strict } {
        x = data[10]  # ERROR - strict mode restored
    }

    y = data[10]  # Returns none - lenient mode again
}
```

**Available Configuration Options**:

**`error_mode`**:
- `:strict` (default) - Errors stop execution unless caught
- `:lenient` - Use safe defaults (none, skip, etc.)
- `:collect` - Collect errors, continue execution, return error list

**`bounds_checking`**:
- `:strict` (default) - Out of bounds access raises error
- `:lenient` - Out of bounds returns none

**`type_coercion`**:
- `:strict` (default) - Type mismatches raise errors
- `:lenient` - Attempt conversions, return none on failure

**`none_handling`**:
- `:propagate` (default) - none values pass through operations
- `:skip` - Skip none values in operations
- `:error` - Treat none as an error

### Error Collection Mode

Useful for processing multiple items where some may fail:

```Graphoid
configure { error_mode: :collect } {
    results = []

    for file in file_list {
        data = process_file(file)  # Errors don't stop loop
        results.append(data)
    }

    # Get collected errors
    errors = get_errors()

    if errors.size() > 0 {
        print("Processing completed with " + errors.size().to_string() + " errors:")
        for err in errors {
            print("  " + err.file() + ": " + err.message())
        }
    }
}

# Alternative: clear errors after checking
configure { error_mode: :collect } {
    batch_1 = process_batch(items_1)
    errors_1 = get_errors()
    clear_errors()

    batch_2 = process_batch(items_2)
    errors_2 = get_errors()
}
```

### Safe Defaults in Standard Library

Many stdlib operations have sensible defaults that avoid errors:

**CSV Module** - Lenient by default:
```Graphoid
import "csv"

# Invalid fields become none (not an error)
data = csv.parse("name,age\nAlice,30\nBob,invalid\nCarol,25\n")
# data[1]["age"] == none

# Strict mode if needed
configure { error_mode: :strict } {
    data = csv.parse(text)  # Raises error on invalid data
}

# Or per-operation
data = csv.parse(text, strict: true)  # Raises error
```

**JSON Module** - Strict by default (malformed JSON is always an error):
```Graphoid
import "json"

# Malformed JSON raises error
try {
    data = json.parse(user_input)
} catch JSONParseError as e {
    print("Invalid JSON")
    data = {}
}
```

**Type Conversions** - Lenient by default:
```Graphoid
# Returns none on failure
num = "abc".to_num()      # none
num = "123".to_num()      # 123

# Strict mode
configure { type_coercion: :strict } {
    num = "abc".to_num()  # ERROR: Cannot convert string to num
}
```

**List/Hash Access**:
```Graphoid
# Lenient mode (returns none)
list = [1, 2, 3]
configure { bounds_checking: :lenient } {
    x = list[10]  # none
}

# Strict mode (default - raises error)
x = list[10]  # ERROR: Index out of bounds
```

### Raising Errors

User code can raise errors explicitly:

```Graphoid
fn validate_age(age) {
    if age < 0 {
        raise ValueError("Age cannot be negative")
    }
    if age > 150 {
        raise ValueError("Age unreasonably large: " + age.to_string())
    }
    return true
}

# Usage
try {
    validate_age(-5)
} catch ValueError as e {
    print("Validation failed: " + e.message())
}
```

**Available Error Types**:
- `Error` - Base error type
- `ValueError` - Invalid value
- `TypeError` - Type mismatch
- `RuntimeError` - Runtime failure
- `IOError` - I/O operation failed
- `NetworkError` - Network operation failed
- `ParseError` - Parsing failed

**Custom Error Types** (future):
```Graphoid
# Define custom error
error_type ValidationError extends ValueError

fn validate_user(data) {
    if !data.has_key("email") {
        raise ValidationError("Missing required field: email")
    }
}
```

### Error Categories

1. **Parse Errors** (always fatal - cannot continue)
   - Unexpected token
   - Missing closing delimiter
   - Invalid syntax

2. **Type Errors** (catchable)
   - Type mismatch in operation
   - Type constraint violation
   - Invalid type conversion (strict mode)

3. **Runtime Errors** (catchable)
   - Division by zero
   - Index out of bounds (strict mode)
   - Key not found (strict mode)
   - File not found (if not using safe return values)

4. **Rule Violations** (catchable)
   - Graph rule violated
   - Edge governance violated
   - Behavior constraint violated
   - Frozen collection mutation

5. **I/O Errors** (catchable)
   - File not found
   - Permission denied
   - Network timeout

6. **User Errors** (catchable)
   - Raised via `raise` keyword

### Error Objects

All errors provide:

```Graphoid
try {
    operation()
} catch Error as e {
    print(e.type())        # "TypeError"
    print(e.message())     # "Cannot append string to list<num>"
    print(e.file())        # "process.gr"
    print(e.line())        # 15
    print(e.column())      # 8
    print(e.stack_trace()) # Full stack trace as string
}
```

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

```
Error: Index out of bounds
  at line 23, column 12 in data.gr

  22 | items = [1, 2, 3]
  23 | x = items[10]
                  ^^

Index 10 is out of bounds for list of size 3
Valid indices: 0 to 2 (or -3 to -1)

Suggestion: Use bounds_checking: :lenient to return none instead
```

### No Safe Navigation Operator

**We explicitly reject** Ruby's `&.` (safe navigation) operator:

```Graphoid
# ❌ BAD (like Ruby)
# value = a&.method1()&.method2()&.method3()

# ✅ GOOD - Explicit checking
if a != none {
    result = a.method1()
    if result != none {
        value = result.method2()
    }
}

# ✅ GOOD - Use configure block for none handling
configure { none_handling: :skip } {
    # Operations automatically skip none
    value = a.method1().method2()  # Stops at first none
}

# ✅ GOOD - Use pattern matching
value = match a {
    |none| => default_value
    |obj| => obj.method1().method2()
}
```

**Rationale**: Safe navigation hides control flow and makes debugging harder. Explicit checks or configuration blocks are clearer.

### Best Practices

**Use return values for expected cases**:
```Graphoid
# ✅ GOOD
port = config["port"]
if port == none {
    port = 8080
}

# ❌ BAD - error for expected case
try {
    port = config["port"]
} catch KeyNotFoundError {
    port = 8080
}
```

**Use exceptions for contract violations**:
```Graphoid
# ✅ GOOD - type constraint violation is an error
list<num> scores = [95, 87, 92]
try {
    scores.append("invalid")  # ERROR
} catch TypeError as e {
    handle_error(e)
}
```

**Configure per context**:
```Graphoid
# Strict for critical operations
configure { error_mode: :strict } {
    validate_financial_data(data)
}

# Lenient for user input
configure { type_coercion: :lenient } {
    age = user_input["age"].to_num()  # Returns none if invalid
}

# Collect errors for batch processing
configure { error_mode: :collect } {
    results = process_all_files(file_list)
    errors = get_errors()
}
```

---

## Built-In Testing Framework

Graphoid includes a comprehensive **behavior-driven testing framework** (RSpec-style) as part of the language, not an external library.

### Spec File Convention

Test files use `.spec.gr` extension and are automatically discovered by `graphoid spec`:

```Graphoid
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
                expect(fn() {
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

```Graphoid
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
expect(fn() { risky_op() }).to_raise("RuntimeError")
expect(fn() { safe_op() }).not_to_raise()

# Deep equality
expect(nested).to_deeply_equal(expected)

# Regex matchers
expect(text).to_match(/pattern/)
```

### Test Organization

```Graphoid
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
- Achieve 90%+ self-hosting
- FFI (Foreign function Interface) for native code access
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
    | functionDeclaration
    | IfStatement
    | WhileLoop
    | ForLoop
    | ReturnStatement
    | BreakStatement
    | NextStatement
    | ImportStatement
    | LoadStatement
    | ModuleDeclaration
    | ConfigureBlock
    | PrecisionBlock
    | ExpressionStatement

VariableDeclaration ::= Type? Identifier "=" Expression
Type ::= "num" | "string" | "bool" | "list" | "hash" | "tree" | "graph"
       | "list" "<" Type ">"
       | "hash" "<" Type ">"
       | "tree" "<" Type ">"

Assignment ::= Identifier "=" Expression
             | IndexAccess "=" Expression

functionDeclaration ::= "fn" Identifier "(" Parameters? ")" Block
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
    | functionCall
    | MethodCall
    | IndexAccess
    | BinaryOp
    | UnaryOp
    | Lambda
    | ListLiteral
    | HashLiteral
    | TreeLiteral
    | GraphLiteral

Literal ::= Number | String | Boolean | Symbol | Regex | "none"

functionCall ::= Identifier "(" Arguments? ")"
MethodCall ::= Expression "." Identifier ("(" Arguments? ")")?
IndexAccess ::= Expression "[" Expression "]"

BinaryOp ::= Expression Operator Expression
Operator ::= "+" | "-" | "*" | "/" | "//" | "%" | "^"
           | "==" | "!=" | "<" | ">" | "<=" | ">="
           | "and" | "or" | "&&" | "||"
           | "=~" | "!~"

UnaryOp ::= "-" Expression | "not" Expression

Lambda ::= Identifier "=>" Expression
         | "(" Parameters ")" "=>" Expression

ListLiteral ::= "[" (Expression ("," Expression)* ","?)? "]"
HashLiteral ::= "{" (HashEntry ("," HashEntry)* ","?)? "}"
HashEntry ::= String ":" Expression

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
4. Multiplicative: `*`, `/`, `//`, `%`
5. Additive: `+`, `-`
6. Comparison: `<`, `>`, `<=`, `>=`, `!<` (equiv to `>=`), `!>` (equiv to `<=`)
7. Equality: `==`, `!=`
8. Regex match: `=~`, `!~`
9. Logical AND: `and`, `&&`
10. Logical OR: `or`, `||`

**Notes**:
- `//` is integer division (truncates toward zero), while `/` is float division
- `!<` (not less than) is syntactic sugar for `>=`, and `!>` (not greater than) is syntactic sugar for `<=`

### Appendix C: Reserved Keywords

```
and, or, not, if, else, while, for, in, break, next,
return, fn, import, load, module, alias, configure,
precision, true, false, none, num, string, bool, list,
hash, tree, graph
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

This document serves as the canonical, implementation-independent specification for the Graphoid programming language. All implementations should strive to conform to this specification.
