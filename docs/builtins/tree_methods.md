# Tree Methods

Binary trees in Glang are graph-based data structures that implement binary search tree (BST) semantics with automatic ordering and efficient lookup operations.

## Tree Creation

```glang
# Create an empty tree
my_tree = tree{}

# Create a type-constrained tree
numbers = tree<num>{}
names = tree<string>{}
```

## Basic Operations

### `insert(value)` -> `none`
Insert a value into the binary search tree maintaining BST ordering.

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)
tree.insert(3)
tree.insert(7)

print(tree.size())  # 5
```

### `search(value)` -> `bool`
Search for a value in the tree using BST lookup.

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)

found = tree.search(5)    # true
missing = tree.search(8)  # false
```

### `size()` -> `num`
Get the number of nodes in the tree.

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)

count = tree.size()  # 2
```

### `empty()` -> `bool`
Check if the tree is empty.

```glang
tree = tree{}
print(tree.empty())  # true

tree.insert(42)
print(tree.empty())  # false
```

### `first()` -> `none`
Always returns `none` for trees. Trees have no meaningful concept of "first" element since they are hierarchical structures organized by value relationships rather than insertion order. This is a universal graph method available on all graph types.

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)

result = tree.first()  # Always returns none
print(result == none)  # true

# Use in_order() for sorted access instead
sorted_values = tree.in_order()  # [5, 10, 15]
first_value = sorted_values.first()  # 5
```

### `last()` -> `none`
Always returns `none` for trees. Trees have no meaningful concept of "last" element since they are hierarchical structures organized by value relationships rather than insertion order. This is a universal graph method available on all graph types.

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)

result = tree.last()   # Always returns none
print(result == none)  # true

# Use in_order() for sorted access instead
sorted_values = tree.in_order()   # [5, 10, 15]
last_value = sorted_values.last()  # 15
```

### `height()` -> `num`
Get the height of the tree (maximum depth from root to leaf).

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)

h = tree.height()  # 2
```

## Tree Traversals

### `in_order()` -> `list`
Return values in in-order traversal (left, root, right). For BSTs, this produces sorted order.

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)
tree.insert(3)
tree.insert(7)

sorted_values = tree.in_order()  # [3, 5, 7, 10, 15]
```

### `pre_order()` -> `list`
Return values in pre-order traversal (root, left, right).

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)

values = tree.pre_order()  # [10, 5, 15]
```

### `post_order()` -> `list`
Return values in post-order traversal (left, right, root).

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)

values = tree.post_order()  # [5, 15, 10]
```

## Type Constraints

Trees support type constraints to ensure all values are of the same type:

```glang
# Only numbers allowed
numbers = tree<num>{}
numbers.insert(42)
numbers.insert(17)
# numbers.insert("hello")  # Error: type constraint violation

# Only strings allowed
words = tree<string>{}
words.insert("apple")
words.insert("banana")
# words.insert(123)  # Error: type constraint violation

# Unconstrained tree accepts any types
mixed = tree{}
mixed.insert(42)
mixed.insert("hello")
mixed.insert(true)  # All allowed
```

## Binary Search Tree Properties

Trees automatically maintain BST ordering:

```glang
tree = tree{}

# Insert in random order
tree.insert(50)
tree.insert(30)
tree.insert(70)
tree.insert(20)
tree.insert(40)
tree.insert(60)
tree.insert(80)

# In-order traversal gives sorted sequence
sorted = tree.in_order()  # [20, 30, 40, 50, 60, 70, 80]

# Search is efficient (O(log n) on average)
found = tree.search(40)   # true
missing = tree.search(35) # false
```

## Edge Governance Integration

Trees inherit all edge governance features for advanced graph operations:

### Rule Management

```glang
tree = tree{}

# Check active governance rules
rules = tree.get_active_rules()
# ["no_list_cycles", "same_structure_only", "max_children_two", "tree_hierarchy", "no_tree_cycles"]

# Check specific rule status
status = tree.get_rule_status("max_children_two")  # "active"

# Temporarily disable rules (advanced usage)
tree.disable_rule("tree_hierarchy")
tree.enable_rule("tree_hierarchy")
```

### Graph Analysis

```glang
tree = tree{}
tree.insert(10)
tree.insert(5)
tree.insert(15)

# Get graph structure summary
summary = tree.get_graph_summary()
print(summary["type"])        # "tree"
print(summary["node_count"])  # 3
print(summary["edge_count"])  # 2

# Visualize tree structure
text_view = tree.visualize_structure("text")
dot_graph = tree.visualize_structure("dot")      # For Graphviz
summary = tree.visualize_structure("summary")
```

## Tree-Specific Governance Rules

Trees have specialized governance rules that enforce tree constraints:

- **`max_children_two`**: Each node can have at most 2 children (left and right)
- **`tree_hierarchy`**: Enforces proper parent-child relationships in trees
- **`no_tree_cycles`**: Prevents any cycles in tree structures (stricter than list cycles)

These rules ensure tree integrity and prevent invalid graph structures.

## Performance Characteristics

- **Insert**: O(log n) average, O(n) worst case
- **Search**: O(log n) average, O(n) worst case
- **Traversal**: O(n) for all nodes
- **Space**: O(n) for storage

## Practical Examples

### Building a Dictionary

```glang
dictionary = tree<string>{}

# Add words in random order
dictionary.insert("banana")
dictionary.insert("apple")
dictionary.insert("cherry")
dictionary.insert("date")

# Get alphabetical listing
alphabetical = dictionary.in_order()  # ["apple", "banana", "cherry", "date"]

# Quick word lookup
has_apple = dictionary.search("apple")    # true
has_zebra = dictionary.search("zebra")    # false
```

### Numerical Analysis

```glang
data = tree<num>{}

# Insert measurements
data.insert(23.5)
data.insert(19.2)
data.insert(31.7)
data.insert(15.8)
data.insert(27.3)

# Get sorted data for analysis
sorted_data = data.in_order()  # [15.8, 19.2, 23.5, 27.3, 31.7]

# Tree structure analysis
print("Dataset size:", data.size())     # 5
print("Tree height:", data.height())    # Tree depth
```

### Comparison with Lists

```glang
# Trees maintain order automatically
tree = tree{}
tree.insert(3)
tree.insert(1)
tree.insert(2)
sorted_tree = tree.in_order()  # [1, 2, 3] - automatically sorted

# Lists maintain insertion order
list = [3, 1, 2]               # [3, 1, 2] - insertion order preserved
```

Trees are ideal when you need:
- Automatic sorting
- Efficient search operations
- Ordered data retrieval
- Logarithmic performance characteristics

Lists are better when you need:
- Insertion order preservation
- Index-based access
- Constant-time append operations
- Sequential data processing