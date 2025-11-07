# Glang Data Type Operations Plan
*Created: 2025-01-06*

## Vision: Graph-Centric Data Operations

Glang's core philosophy treats all data as graphs. This plan defines comprehensive operations for each data type, viewing them through the lens of graph theory:

- **Strings**: Linear graphs (sequences of character nodes)
- **Numbers**: Atomic values that can form computational graphs  
- **Lists**: Ordered graph structures with rich connectivity
- **Booleans**: Binary state nodes for logical graphs

## Data Type Operations by Category

### 🔢 **Numeric Operations (`num`)**

#### Arithmetic Operations
```glang
# Basic arithmetic - creating computational graphs
num a = 5
num b = 3
num sum = a + b        # Addition
num diff = a - b       # Subtraction  
num product = a * b    # Multiplication
num quotient = a / b   # Division
num remainder = a % b  # Modulo
num power = a ^ b      # Exponentiation
```

#### Comparison Operations
```glang
# Relational operations - graph connectivity analysis
bool greater = a > b
bool less = a < b
bool equal = a == b
bool not_equal = a != b
bool gte = a >= b          # Greater than or equal
bool gte_alt = a !< b      # Alternative: "not less than"
bool lte = a <= b          # Less than or equal  
bool lte_alt = a !> b      # Alternative: "not greater than"
```

#### Mathematical Methods (Type-Safe Operations)
```glang
# Math operations as methods on nums - eliminates type errors!
num value = 123.456

# Rounding methods with levels
num rounded = value.rnd()           # 123 (round to nearest 1)
num rounded_10 = value.rnd(10)      # 120 (round to nearest 10)
num rounded_100 = value.rnd(100)    # 100 (round to nearest 100)
num rounded_cent = value.rnd(0.01)  # 123.46 (round to nearest cent)

num rounded_up = value.rnd_up()     # 124 (round up to next 1)
num rounded_up_10 = value.rnd_up(10)    # 130 (round up to next 10)
num rounded_up_cent = value.rnd_up(0.01) # 123.46 (round up to next cent)

num rounded_down = value.rnd_dwn()      # 123 (round down to 1)
num rounded_down_10 = value.rnd_dwn(10) # 120 (round down to 10)

# Other math methods
num absolute = value.abs()          # Absolute value
num square_root = value.sqrt()      # Square root
num logarithm = value.log()         # Natural logarithm
num power = value.pow(2)            # Power (value^2)
```

#### Precision Operations
```glang
# Precision control - truncate to specified decimal places
num value = 15.1115
num precise = value.to(2)     # 15.11 (truncated, not rounded)
num precise_alt = value.to(0) # 15 (truncated to integer)
num precise_three = value.to(3) # 15.111 (truncated to 3 decimals)
```

### 🔤 **String Operations (`string`)**
*Strings as Linear Character Graphs*

#### Construction & Concatenation
```glang
# String graphs - building character sequences
string greeting = "Hello"
string name = "World" 
string message = greeting + " " + name  # Graph concatenation
string template = "Count: ${count}"     # Template interpolation
```

#### Graph Traversal & Slicing  
```glang
# Navigate the character graph
string text = "Hello World"
string first_char = text[0]           # First node access
string last_char = text[-1]           # Last node access  
string substring = text[1:5]          # Subgraph extraction
string reversed = text[::-1]          # Reverse traversal
```

#### Character Graph Analysis
```glang
# Analyze the string graph structure
num length = text.length()            # Graph size
bool is_empty = text.isEmpty()        # Empty graph check
bool contains = text.contains("World") # Subgraph search
num position = text.indexOf("World")  # Node position finding
```

#### Graph Transformations
```glang
# Transform the character graph
string upper = text.up()              # Case transformation (intuitive)
string upper_alt = text.toUpper()     # Case transformation (alias)
string lower = text.down()            # Case transformation (intuitive)  
string lower_alt = text.toLower()     # Case transformation (alias)
string trimmed = text.trim()          # Edge node removal
list<string> words = text.split(" ")  # Graph decomposition
string joined = words.join("-")       # Graph reconstruction
```

#### Pattern Matching (Graph Pattern Recognition)
```glang
# Advanced string graph analysis
bool matches = text.matches("^Hello.*")     # Pattern matching
string replaced = text.replace("World", "Universe")  # Subgraph replacement
list<string> found = text.findAll("[A-Z]\\w+")      # Pattern extraction
```

### 📋 **List Operations (`list<T>`)**
*Lists as Ordered Graph Collections*

#### Graph Construction
```glang
# Building list graphs
list<num> numbers = [1, 2, 3, 4, 5]
list<string> words = ["apple", "banana", "cherry"]
list<list<num>> matrix = [[1, 2], [3, 4]]  # Nested graph structures
```

#### Graph Navigation & Access
```glang
# Traverse the list graph
num first = numbers[0]                # First node
num last = numbers[-1]                # Last node
list<num> slice = numbers[1:4]        # Subgraph extraction
num length = numbers.length()         # Graph size
```

#### Graph Modification
```glang
# Modify the list graph structure
numbers.append(6)                     # Add node to end
numbers.prepend(0)                    # Add node to beginning
numbers.insert(2, 10)                 # Insert node at position
num removed = numbers.remove(1)       # Remove node by index
bool found = numbers.removeValue(10)  # Remove node by value
numbers.clear()                       # Empty the graph
```

#### Graph Traversal & Iteration
```glang
# Walk through the graph
for item in numbers {
    print(item)                       # Visit each node
}

for index, item in numbers.enumerate() {
    print("${index}: ${item}")        # Indexed traversal
}
```

#### Graph Analysis & Search
```glang
# Analyze list graph properties
bool contains = numbers.contains(5)   # Node existence check
num index = numbers.indexOf(3)        # Find node position
num count = numbers.count(2)          # Count node occurrences
num minimum = numbers.min()           # Find minimum value node
num maximum = numbers.max()           # Find maximum value node
num sum_total = numbers.sum()         # Aggregate all nodes
```

#### Graph Transformations
```glang
# Transform the list graph
list<num> doubled = numbers.map(x -> x * 2)        # Transform each node
list<num> evens = numbers.filter(x -> x % 2 == 0)  # Filter nodes
num total = numbers.reduce(0, (acc, x) -> acc + x) # Reduce graph
numbers.sort()                                      # Reorder nodes
numbers.reverse()                                   # Reverse connectivity
list<num> unique = numbers.distinct()              # Remove duplicate nodes
```

#### Graph Combination
```glang
# Combine list graphs
list<num> other = [10, 11, 12]
list<num> combined = numbers + other           # List concatenation
list<num> intersection = numbers & other       # Graph intersection
list<num> difference = numbers - other         # Graph difference
```

#### List-Scalar Arithmetic (Advanced Graph Operations)
```glang
# Apply operations across entire graph - ADVANCED FEATURE
list<num> base = [1, 2, 3]
list<num> added = base + 5                     # [6, 7, 8] - add 5 to each element
list<num> subtracted = base - 2                # [-1, 0, 1] - subtract 2 from each
list<num> multiplied = base * 3                # [3, 6, 9] - multiply each by 3
list<num> divided = base / 2                   # [0.5, 1, 1.5] - divide each by 2
list<num> modded = base % 2                    # [1, 0, 1] - modulo each by 2

# List-to-list operations  
list<num> other = [10, 20, 30]
list<num> element_sum = base + other           # [11, 22, 33] - element-wise addition
list<num> element_diff = other - base          # [9, 18, 27] - element-wise subtraction
list<num> element_prod = base * other          # [10, 40, 90] - element-wise multiplication
```

### ✅ **Boolean Operations (`bool`)**
*Booleans as Binary Logic Nodes*

#### Logical Operations
```glang
# Boolean graph logic
bool a = true
bool b = false
bool and_result = a && b              # Logical AND
bool or_result = a || b               # Logical OR  
bool not_result = !a                  # Logical NOT
bool xor_result = a ^^ b              # Logical XOR
```

#### Boolean Methods (Type-Safe Operations)
```glang
# Methods on boolean values
bool flag = true
bool flipped = flag.flip()            # false (reverse polarity)
num numeric = flag.numify()           # 1 (true becomes 1)

bool other = false
bool other_flipped = other.flip()     # true (reverse polarity)
num other_numeric = other.numify()    # 0 (false becomes 0)

# Alternative names (aliases)
num as_number = flag.toNum()          # Alias for numify()
bool toggled = flag.toggle()          # Alias for flip()
```

#### Conditional Logic
```glang
# Boolean-driven control flow
bool condition = age >= 18
string status = condition ? "adult" : "minor"  # Ternary operator

if condition {
    print("Eligible")
} else {
    print("Not eligible")  
}
```

## Implementation Plan

### Phase 1: Core Arithmetic & String Operations
**Priority: HIGH - Essential for basic programs**

**Target Operations:**
- ✅ Basic arithmetic: `+`, `-`, `*`, `/`, `%` for numbers
- ✅ Comparison operators: `>`, `<`, `==`, `!=`, `>=`, `<=`, `!>`, `!<` for numbers  
- ✅ String concatenation with `+` operator
- ✅ String/list indexing with `[n]` syntax
- ✅ String/list slicing with `[start:end]` syntax
- ✅ Core string methods: `length()`, `contains()`, `up()`, `down()` (with `toUpper()`, `toLower()` aliases)
- ✅ Precision operations: `num.to(digits)` for truncation

### Phase 2: List Graph Operations + Basic Reflection ✅ **COMPLETED**
**Priority: HIGH - Core data structure manipulation & discoverability**

**Target Operations:**
- ✅ List modification: `append()`, `prepend()`, `insert()`, `remove()`
- ✅ List analysis: `indexOf()`, `count()`, `min()`, `max()`, `sum()`
- ✅ List transformations: `sort()`, `reverse()` (immutable operations)
- ⚠️ List transformations: `map()`, `filter()`, `reduce()` (deferred - require lambda support)
- ✅ List combinations: union (`+`), intersection (`&`), difference (`-`)

**Basic Reflection (All Types):**
- ✅ `value.type()` - get type name ("num", "string", "bool", "list")
- ✅ `value.methods()` - list all available method names
- ✅ `value.can(method_name)` - check if method exists
- ✅ `value.inspect()` - human-readable representation with type info

**Implementation Notes:**
- Established hybrid immutability model: transformations return new values, modifications change in place
- All new methods integrated with universal reflection system
- 25 comprehensive tests added in `test_list_graph_operations.py`
- All 333 tests passing with 68% coverage

### Phase 3: Advanced String Graph Operations + REPL Reflection ✅ **COMPLETED**
**Priority: MEDIUM - Rich text processing & enhanced discoverability**

**Target Operations:**
- ✅ String transformations: `trim()` (whitespace removal), `split()` (enhanced), `join()` (list to string)  
- ✅ Pattern operations: `matches()` (regex matching), `replace()` (regex replacement), `findAll()` (extract matches)
- ✅ Advanced indexing: negative indices (`text[-1]`), step slicing (`text[::2]`, `list[::-1]`)

**REPL Reflection Commands:**
- ✅ `/methods variable_name` - show all methods for a variable
- ✅ `/type variable_name` - show detailed type information
- ✅ `/inspect variable_name` - detailed value inspection
- ✅ `/can variable_name method` - check method availability

**Implementation Notes:**
- All string transformations are immutable (return new strings)
- Pattern operations use Python's `re` module with full regex support
- Enhanced slicing works for both strings and lists with consistent behavior
- REPL reflection commands provide comprehensive introspection capabilities
- 9 comprehensive tests added in `test_phase3_advanced_strings.py`
- All 350 tests passing with 68% coverage

### Phase 4: Advanced List-Scalar Arithmetic ✅ **COMPLETED**
**Priority: MEDIUM - Graph-theoretic list operations**

**Target Operations:**
- ✅ List-scalar arithmetic: `[1,2,3] +. 5` → `[6,7,8]` (using dot operators)
- ✅ List-list element-wise operations: `[1,2,3] +. [10,20,30]` → `[11,22,33]` (using dot operators)
- ✅ All element-wise arithmetic operators: `+.`, `-.`, `*.`, `/.`, `%.`
- ✅ Clear operator separation: `+` always concatenates, `+.` does element-wise arithmetic
- ✅ Helpful error messages: "use *. for element-wise operations"

**Implementation Notes:**
- Resolved operator collision by introducing dot operators (`.` suffix) for element-wise operations
- `+` now always does list concatenation: `[1,2,3] + [10,20,30]` → `[1,2,3,10,20,30]`
- Element-wise operations require explicit dot operators: `[1,2,3] +. [10,20,30]` → `[11,22,33]`
- Added 5 new token types and comprehensive executor methods (~280 lines)
- 16 comprehensive tests added in `test/test_dot_operators.py`
- All 367 tests passing with clean, unambiguous operator behavior

### Phase 5: Mathematical Methods & Advanced Reflection
**Priority: MEDIUM - Scientific computing & graph introspection**

**Target Operations:**
- Num methods: `value.abs()`, `value.sqrt()`, `value.log()`, `value.pow(n)`
- Rounding methods: `value.rnd(level)`, `value.rnd_up(level)`, `value.rnd_dwn(level)`
- Level examples: `1` (default), `10`, `100`, `0.01`, `0.1`
- Boolean methods: `flag.flip()`, `flag.numify()`, `flag.toggle()`, `flag.toNum()`
- Trigonometric methods: `value.sin()`, `value.cos()`, `value.tan()`
- Constants: `PI`, `E`

**Advanced Graph Reflection:**
- `value.graph_size()` - count of nodes in the graph structure
- `value.graph_type()` - "atomic", "linear", "tree", or "graph"
- `list.element_type()` - type of elements in lists
- `list.is_uniform()` - whether all elements are same type
- `string.encoding()` - character encoding information

### Phase 6: Boolean Logic & Control Flow
**Priority: LOW - Already mostly implemented**

**Target Operations:**
- Logical operators: `&&`, `||`, `!`, `^^`
- Ternary operator: `condition ? true_value : false_value`

### Phase 7: CLI Program Execution
**Priority: MEDIUM - Developer Experience Enhancement**

**Target Features:**
- Direct program execution: `glang program.gr` or `glang ./program.gr`
- Support for .gr file execution without entering REPL
- Proper exit codes for script success/failure
- Command-line argument passing to programs
- Shebang support for executable .gr files

**Implementation Notes:**
- Extend existing CLI interface (`src/glang/cli.py`) to handle file arguments
- Reuse ExecutionSession infrastructure for file execution
- Provide clean error reporting for script execution failures
- Consider supporting both `.gr` and `.gl` file extensions
- Integration with existing file loading system

## Graph Theory Integration

Each operation is designed with graph principles:

1. **Connectivity**: Operations preserve or transform node relationships
2. **Traversal**: Algorithms for walking data structures 
3. **Analysis**: Understanding graph properties (size, connectivity, patterns)
4. **Transformation**: Converting between graph representations
5. **Composition**: Combining graphs to create new structures

## Success Metrics

**Phase 1 Complete When:**
- Can write basic calculator programs: `num result = (a + b) * c / 2`
- Can perform string manipulation: `string name = first + " " + last.up()`
- Can access/slice sequences: `string first_char = name[0]`, `string part = name[1:5]`
- Can use intuitive comparisons: `bool valid = age !< 18` (not less than 18)
- Can control precision: `num price = total.to(2)` (truncate to 2 decimals)
- Can discover capabilities: `list<string> ops = value.methods()`, `bool can_do = value.can("rnd")`

**Full Implementation Complete When:**
- Can write rich data processing programs
- Full mathematical computing capability with flexible rounding
- Complex string processing and pattern matching  
- Sophisticated list manipulation and analysis
- Advanced list-scalar arithmetic for vector operations

**Type-Safe Method Examples:**
```glang
# Numbers know how to do number operations
num price = 127.89
num to_dollar = price.rnd_up()       # 128 (next dollar)  
num to_five = price.rnd_up(5)        # 130 (next $5)
num to_ten = price.rnd_up(10)        # 130 (next $10)
num to_cent = price.rnd_dwn(0.01)    # 127.89 (to cent)
num to_nickel = price.rnd_up(0.05)   # 127.90 (next nickel)
num to_quarter = price.rnd_up(0.25)  # 128.00 (next quarter)

# Strings know how to do string operations  
string name = "Alice"
string upper = name.up()             # "ALICE"
string first = name[0]               # "A"

# Booleans know how to do boolean operations
bool active = true
bool inactive = active.flip()        # false
num binary = active.numify()         # 1

# Everything can reflect on itself
string type_info = price.type()      # "num"
list<string> available = price.methods() # ["rnd", "rnd_up", "abs", "sqrt", ...]
bool can_round = price.can("rnd")    # true
string debug = price.inspect()       # "127.89 (num)"

# No more type errors like round("string", 10) - impossible!
```

## Architecture Notes

**Implementation Strategy:**
- **Type-safe methods**: Each data type implements its own operations
- **No global functions**: Operations belong to the types that support them
- **Error elimination**: `round("string", 10)` becomes syntactically impossible
- Use elegant keyword registry for operator definitions
- Extend AST nodes for method calls and binary/unary operations
- Add operator precedence parsing  
- Implement operations as methods on GlangValue types
- Maintain graph paradigm throughout execution

**Key Files to Modify:**
- `src/glang/language/keyword_registry.py` - Add operator definitions
- `src/glang/parser/ast_parser.py` - Add expression parsing for operators
- `src/glang/ast/nodes.py` - Add operation AST nodes
- `src/glang/execution/values.py` - Implement operations on value types
- `src/glang/execution/executor.py` - Execute operation nodes

## Reflection & Discoverability

### Universal Reflection Methods
*Every value has these introspection capabilities*

```glang
# Type introspection
num value = 42
string type_name = value.type()          # "num"
bool is_number = value.is("num")         # true
bool is_string = value.is("string")      # false

# Method discovery
list<string> methods = value.methods()   # ["abs", "rnd", "rnd_up", "rnd_dwn", "sqrt", "log", "pow", "to", ...]
bool can_round = value.can("rnd")        # true
bool can_flip = value.can("flip")        # false (nums can't flip)

# Value inspection
string display = value.inspect()         # "42 (num)"
string debug = value.debug()             # Detailed internal representation

# Graph properties (advanced)
num size = value.graph_size()            # 1 (atomic value)
string structure = value.graph_type()    # "atomic" | "linear" | "tree" | "graph"
```

### Type-Specific Reflection
```glang
# String reflection
string text = "Hello World"
num char_count = text.length()           # 11
list<string> chars = text.chars()        # ["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d"]
string encoding = text.encoding()        # "UTF-8"

# List reflection  
list<num> numbers = [1, 2, 3, 4, 5]
num item_count = numbers.length()        # 5
string element_type = numbers.element_type()  # "num"
bool is_homogeneous = numbers.is_uniform()    # true (all same type)

# Boolean reflection
bool flag = true
string truth_value = flag.state()       # "true"
num numeric_value = flag.numify()       # 1
```

### REPL Introspection Commands
```glang
# Interactive exploration
/methods value        # Show all methods available on value
/type value          # Show type information
/inspect value       # Detailed value inspection
/can value method    # Check if value supports method
```

### Implementation Phases

**Phase 2 Extension: Basic Reflection**
- `value.type()` - get type name
- `value.methods()` - list available methods
- `value.can(method)` - check method availability
- `value.inspect()` - human-readable representation

**Phase 3 Extension: Advanced Reflection**  
- `value.graph_size()` - graph node count
- `value.graph_type()` - graph structure type
- Type-specific inspectors (encoding, element types, etc.)

**Phase 4 Extension: REPL Integration**
- `/methods`, `/type`, `/inspect`, `/can` commands
- IDE integration hooks for autocomplete
- Runtime method documentation

### Benefits

1. **Developer Experience**: Discover available operations interactively
2. **Debugging**: Inspect values and their capabilities at runtime
3. **IDE Integration**: Perfect autocomplete and method suggestions
4. **Learning**: Explore language capabilities naturally
5. **Dynamic Programming**: Check capabilities before using them

### Graph-Theoretic Reflection
```glang
# Advanced: Reflect on graph structure
string text = "Hello"
string graph_info = text.graph_type()   # "linear" (sequence of chars)
num nodes = text.graph_size()           # 5 (5 character nodes)
list<string> connections = text.graph_edges() # Adjacency information

list<num> data = [1, 2, 3]
string structure = data.graph_type()    # "linear" (ordered sequence)
bool is_connected = data.is_connected() # true (all elements linked)
```

This reflection system would make Glang incredibly discoverable while maintaining its graph-theoretic elegance!

This plan positions Glang to support real-world programming while maintaining its unique graph-theoretic foundation! 🚀