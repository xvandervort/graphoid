# tree - Tree Data Structure

Trees in Graphoid are ordered, hierarchical collections. By default, they are binary search trees (BST) that maintain sorted order and provide efficient lookup, insertion, and deletion.

## Tree Literals

### Basic Trees

Trees are created using the `tree{}` literal:

```graphoid
# Empty tree
numbers = tree{}

# Tree with initial values
numbers = tree{5, 3, 7, 1, 9}

# Type-constrained tree
tree<num> scores = tree{95, 87, 92}
tree<string> names = tree{"Alice", "Bob", "Charlie"}
```

### Tree Types

Trees can be configured with different properties:

```graphoid
# Binary search tree (default)
bst = tree { type: :bst }

# Balanced tree (AVL or Red-Black)
balanced = tree { type: :balanced }

# Binary tree (max 2 children, no ordering)
binary = tree { type: :binary }

# N-ary tree (configurable max children)
quad = tree { type: :nary, max_children: 4 }
```

---

## Tree Operations

### insert(value)

Inserts a value into the tree.

**Syntax**: `tree.insert(value)`

**Parameters**:
- `value`: Value to insert

**Returns**: none (modifies tree in place)

**Examples**:
```graphoid
numbers = tree{}
numbers.insert(5)
numbers.insert(3)
numbers.insert(7)
numbers.insert(1)
numbers.insert(9)

# Tree maintains BST property:
#       5
#      / \
#     3   7
#    /     \
#   1       9

# Insert multiple
for num in [4, 6, 8] {
    numbers.insert(num)
}
```

**See also**: `remove()`, `contains()`

---

### remove(value)

Removes a value from the tree.

**Syntax**: `tree.remove(value)`

**Parameters**:
- `value`: Value to remove

**Returns**: (bool) `true` if removed, `false` if not found

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
removed = numbers.remove(3)
print(removed)  # true

removed = numbers.remove(100)
print(removed)  # false

# Remove multiple
to_remove = [1, 9]
for num in to_remove {
    numbers.remove(num)
}
```

**See also**: `insert()`, `clear()`

---

### contains(value)

Tests if a value exists in the tree.

**Syntax**: `tree.contains(value)`

**Parameters**:
- `value`: Value to search for

**Returns**: (bool) `true` if found, `false` otherwise

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
result = numbers.contains(7)   # true
result = numbers.contains(100) # false

# Same as 'in' operator
result = 7 in numbers  # true

# Conditional logic
if numbers.contains(target) {
    print("Found!")
}
```

**See also**: `find()`, `in`

---

### find(value)

Finds a value in the tree (returns the value if found).

**Syntax**: `tree.find(value)`

**Parameters**:
- `value`: Value to search for

**Returns**: Value if found, or `none` if not found

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
result = numbers.find(7)
print(result)  # 7

result = numbers.find(100)
print(result)  # none

# With objects
users = tree{}  # Sorted by some key
user = users.find(user_id)
```

**See also**: `contains()`, `search()`

---

### min()

Returns the minimum value in the tree.

**Syntax**: `tree.min()`

**Returns**: Minimum value

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
minimum = numbers.min()
print(minimum)  # 1

# Empty tree
empty = tree{}
result = empty.min()  # none or error
```

**Errors**: Calling `min()` on empty tree may raise error

**See also**: `max()`

---

### max()

Returns the maximum value in the tree.

**Syntax**: `tree.max()`

**Returns**: Maximum value

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
maximum = numbers.max()
print(maximum)  # 9

# Range queries
min_val = numbers.min()
max_val = numbers.max()
print("Range: ${min_val} to ${max_val}")
```

**Errors**: Calling `max()` on empty tree may raise error

**See also**: `min()`

---

### height()

Returns the height of the tree.

**Syntax**: `tree.height()`

**Returns**: (num) Tree height (levels from root to deepest leaf)

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
#       5
#      / \
#     3   7
#    /     \
#   1       9
# Height = 3

h = numbers.height()
print(h)  # 3

# Empty tree
empty = tree{}
print(empty.height())  # 0
```

**See also**: `size()`, `is_balanced()`

---

### size()

Returns the number of nodes in the tree.

**Syntax**: `tree.size()`

**Returns**: (num) Number of nodes

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
count = numbers.size()
print(count)  # 5

# Same as length()
count = numbers.length()  # 5
```

**See also**: `is_empty()`, `height()`

---

### length()

Alias for `size()`. Returns the number of nodes.

**Syntax**: `tree.length()`

**Returns**: (num) Number of nodes

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
print(numbers.length())  # 5
```

**See also**: `size()`

---

### clear()

Removes all nodes from the tree.

**Syntax**: `tree.clear()`

**Returns**: none (modifies tree in place)

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
numbers.clear()
print(numbers.size())  # 0
print(numbers.is_empty())  # true
```

**See also**: `remove()`

---

### is_empty()

Tests if the tree is empty.

**Syntax**: `tree.is_empty()`

**Returns**: (bool) `true` if empty, `false` otherwise

**Examples**:
```graphoid
empty = tree{}
print(empty.is_empty())  # true

numbers = tree{5, 3, 7}
print(numbers.is_empty())  # false

# Guard clause
if tree.is_empty() {
    print("Tree is empty")
    return
}
```

**See also**: `size()`, `length()`

---

## Tree Traversal

### in_order()

Returns values in sorted order (left-root-right).

**Syntax**: `tree.in_order()`

**Returns**: (list) Values in ascending order

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9, 4, 6}
sorted = numbers.in_order()
print(sorted)  # [1, 3, 4, 5, 6, 7, 9]

# For BST, this gives sorted order
names = tree{"Charlie", "Alice", "Bob", "Diana"}
alphabetical = names.in_order()
print(alphabetical)  # ["Alice", "Bob", "Charlie", "Diana"]
```

**See also**: `pre_order()`, `post_order()`, `to_list()`

---

### pre_order()

Returns values in pre-order traversal (root-left-right).

**Syntax**: `tree.pre_order()`

**Returns**: (list) Values in pre-order

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
#       5
#      / \
#     3   7
#    /     \
#   1       9

result = numbers.pre_order()
print(result)  # [5, 3, 1, 7, 9]
```

**See also**: `in_order()`, `post_order()`

---

### post_order()

Returns values in post-order traversal (left-right-root).

**Syntax**: `tree.post_order()`

**Returns**: (list) Values in post-order

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
#       5
#      / \
#     3   7
#    /     \
#   1       9

result = numbers.post_order()
print(result)  # [1, 3, 9, 7, 5]
```

**See also**: `in_order()`, `pre_order()`

---

### level_order()

Returns values in level-order traversal (breadth-first).

**Syntax**: `tree.level_order()`

**Returns**: (list) Values level by level

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9, 4, 6}
#       5
#      / \
#     3   7
#    / \ / \
#   1  4 6  9

result = numbers.level_order()
print(result)  # [5, 3, 7, 1, 4, 6, 9]
```

**See also**: `in_order()`, `bfs()`

---

### to_list()

Converts tree to a sorted list (same as `in_order()`).

**Syntax**: `tree.to_list()`

**Returns**: (list) Sorted list of values

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
list = numbers.to_list()
print(list)  # [1, 3, 5, 7, 9]

# Iterate over sorted values
for num in numbers.to_list() {
    print(num)
}
```

**See also**: `in_order()`, `to_array()`

---

## Range Queries

### range(min, max)

Returns values within a range.

**Syntax**: `tree.range(min, max, inclusive)`

**Parameters**:
- `min`: Minimum value (inclusive)
- `max`: Maximum value (inclusive by default)
- `inclusive` (bool, optional): Include boundaries (default: true)

**Returns**: (list) Values in range

**Examples**:
```graphoid
numbers = tree{1, 3, 5, 7, 9, 11, 13, 15}

# Inclusive range
result = numbers.range(5, 11)
print(result)  # [5, 7, 9, 11]

# Exclusive range
result = numbers.range(5, 11, false)
print(result)  # [7, 9]

# One-sided ranges
less_than_10 = numbers.range(-infinity, 10)
greater_than_10 = numbers.range(10, infinity)
```

**See also**: `filter()`, `between()`

---

### less_than(value)

Returns all values less than a given value.

**Syntax**: `tree.less_than(value, inclusive)`

**Parameters**:
- `value`: Upper bound
- `inclusive` (bool, optional): Include value (default: false)

**Returns**: (list) Values less than value

**Examples**:
```graphoid
numbers = tree{1, 3, 5, 7, 9, 11}

# Less than (exclusive)
result = numbers.less_than(7)
print(result)  # [1, 3, 5]

# Less than or equal (inclusive)
result = numbers.less_than(7, true)
print(result)  # [1, 3, 5, 7]
```

**See also**: `greater_than()`, `range()`

---

### greater_than(value)

Returns all values greater than a given value.

**Syntax**: `tree.greater_than(value, inclusive)`

**Parameters**:
- `value`: Lower bound
- `inclusive` (bool, optional): Include value (default: false)

**Returns**: (list) Values greater than value

**Examples**:
```graphoid
numbers = tree{1, 3, 5, 7, 9, 11}

# Greater than (exclusive)
result = numbers.greater_than(7)
print(result)  # [9, 11]

# Greater than or equal (inclusive)
result = numbers.greater_than(7, true)
print(result)  # [7, 9, 11]
```

**See also**: `less_than()`, `range()`

---

## Tree Properties

### is_balanced()

Tests if the tree is height-balanced.

**Syntax**: `tree.is_balanced()`

**Returns**: (bool) `true` if balanced, `false` otherwise

**Examples**:
```graphoid
# Balanced tree
balanced = tree{5, 3, 7, 1, 9}
#       5
#      / \
#     3   7
#    /     \
#   1       9
print(balanced.is_balanced())  # true

# Unbalanced tree (degenerate to linked list)
unbalanced = tree{}
for i in range(10) {
    unbalanced.insert(i)
}
print(unbalanced.is_balanced())  # false
```

**See also**: `height()`, `rebalance()`

---

### is_bst()

Tests if the tree maintains BST property.

**Syntax**: `tree.is_bst()`

**Returns**: (bool) `true` if valid BST, `false` otherwise

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
print(numbers.is_bst())  # true

# This should always be true for trees created via insert()
# Useful for verifying manually constructed trees
```

**See also**: `validate()`

---

### rebalance()

Rebalances the tree to minimize height.

**Syntax**: `tree.rebalance()`

**Returns**: none (modifies tree in place)

**Examples**:
```graphoid
# Create unbalanced tree
numbers = tree{}
for i in range(10) {
    numbers.insert(i)
}
print("Height before: " + numbers.height().to_string())  # 10

# Rebalance
numbers.rebalance()
print("Height after: " + numbers.height().to_string())   # 4

print(numbers.is_balanced())  # true
```

**See also**: `is_balanced()`, `height()`

---

## Tree Rules

Trees support validation rules:

### add_rule(rule)

Adds a validation rule to the tree.

**Syntax**: `tree.add_rule(rule, ...args)`

**Parameters**:
- `rule` (string or function): Rule name or validation function
- `args`: Additional arguments for the rule

**Returns**: none

**Examples**:
```graphoid
# Enforce single root
tree.add_rule("single_root")

# Enforce maximum children
binary = tree{}
binary.add_rule("max_children", 2)

# Custom validation
fn validate_positive(tree) {
    for value in tree.to_list() {
        if value < 0 {
            return false
        }
    }
    return true
}

numbers = tree{}
numbers.add_rule(validate_positive)
numbers.insert(5)   # OK
# numbers.insert(-1)  # Error: violates rule
```

**Built-in Rules**:
- `"single_root"` - Tree must have one root
- `"max_children"` - Limit children per node
- `"no_cycles"` - Prevent cycles (trees shouldn't have cycles anyway)
- `"balanced"` - Maintain balance on every insertion

**See also**: [Graph Rules](../../user-guide/06-graph-operations.md#graph-rules)

---

## Functional Methods

### map(function)

Transforms each value using a function.

**Syntax**: `tree.map(function)`

**Parameters**:
- `function`: Transformation function

**Returns**: (tree) New tree with transformed values

**Examples**:
```graphoid
numbers = tree{1, 2, 3, 4, 5}
doubled = numbers.map(x => x * 2)
print(doubled.to_list())  # [2, 4, 6, 8, 10]

# Named transformations
squared = numbers.map("square")
print(squared.to_list())  # [1, 4, 9, 16, 25]
```

**Note**: The resulting tree maintains sorted order if values remain comparable.

**See also**: `filter()`, `reduce()`

---

### filter(predicate)

Selects values that match a condition.

**Syntax**: `tree.filter(predicate)`

**Parameters**:
- `predicate`: Test function or named predicate

**Returns**: (tree) Filtered tree

**Examples**:
```graphoid
numbers = tree{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}

# Named predicates
evens = numbers.filter("even")
print(evens.to_list())  # [2, 4, 6, 8, 10]

# Lambda
large = numbers.filter(x => x > 5)
print(large.to_list())  # [6, 7, 8, 9, 10]
```

**See also**: `map()`, `range()`

---

### reduce(function, initial)

Reduces tree to a single value.

**Syntax**: `tree.reduce(function, initial)`

**Parameters**:
- `function`: Accumulator function `(acc, value) => result`
- `initial`: Initial accumulator value

**Returns**: Final accumulated value

**Examples**:
```graphoid
numbers = tree{1, 2, 3, 4, 5}

# Sum
sum = numbers.reduce((acc, x) => acc + x, 0)
print(sum)  # 15

# Product
product = numbers.reduce((acc, x) => acc * x, 1)
print(product)  # 120
```

**Note**: Traverses in in-order (sorted order).

**See also**: `map()`, `filter()`

---

### each(function)

Executes a function for each value (in sorted order).

**Syntax**: `tree.each(function)`

**Parameters**:
- `function`: Function to execute

**Returns**: none

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}
numbers.each(x => print(x))
# Prints in sorted order: 1 3 5 7 9

# With index
numbers.each((value, index) => {
    print("${index}: ${value}")
})
```

**See also**: `map()`, `in_order()`

---

## Type Checking

### is_tree()

Tests if a value is a tree.

**Syntax**: `value.is_tree()`

**Returns**: (bool) `true` if tree, `false` otherwise

**Examples**:
```graphoid
result = tree{}.is_tree()       # true
result = tree{1, 2, 3}.is_tree()  # true
result = [1, 2, 3].is_tree()    # false

# Type validation
if not value.is_tree() {
    print("Expected tree")
}
```

**See also**: `is_list()`, `is_hash()`, `is_graph()`

---

## Advanced Operations

### successor(value)

Finds the next larger value.

**Syntax**: `tree.successor(value)`

**Parameters**:
- `value`: Value to find successor of

**Returns**: Next larger value, or `none` if none exists

**Examples**:
```graphoid
numbers = tree{1, 3, 5, 7, 9}
next = numbers.successor(5)
print(next)  # 7

next = numbers.successor(9)
print(next)  # none (no successor)
```

**See also**: `predecessor()`, `greater_than()`

---

### predecessor(value)

Finds the previous smaller value.

**Syntax**: `tree.predecessor(value)`

**Parameters**:
- `value`: Value to find predecessor of

**Returns**: Previous smaller value, or `none` if none exists

**Examples**:
```graphoid
numbers = tree{1, 3, 5, 7, 9}
prev = numbers.predecessor(5)
print(prev)  # 3

prev = numbers.predecessor(1)
print(prev)  # none (no predecessor)
```

**See also**: `successor()`, `less_than()`

---

### kth_smallest(k)

Finds the k-th smallest element (1-indexed).

**Syntax**: `tree.kth_smallest(k)`

**Parameters**:
- `k` (num): Position (1-indexed)

**Returns**: k-th smallest value

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}

first = numbers.kth_smallest(1)   # 1
second = numbers.kth_smallest(2)  # 3
median = numbers.kth_smallest(3)  # 5

# Median of tree
median_index = (numbers.size() + 1) / 2
median = numbers.kth_smallest(median_index)
```

**Errors**: k out of range raises error

**See also**: `kth_largest()`

---

### kth_largest(k)

Finds the k-th largest element (1-indexed).

**Syntax**: `tree.kth_largest(k)`

**Parameters**:
- `k` (num): Position (1-indexed)

**Returns**: k-th largest value

**Examples**:
```graphoid
numbers = tree{5, 3, 7, 1, 9}

largest = numbers.kth_largest(1)  # 9
second = numbers.kth_largest(2)   # 7
```

**Errors**: k out of range raises error

**See also**: `kth_smallest()`

---

## Common Patterns

### Sorted set operations

```graphoid
# Union
a = tree{1, 3, 5}
b = tree{2, 3, 4}
union = tree{}
for x in a.to_list() { union.insert(x) }
for x in b.to_list() { union.insert(x) }
# union = tree{1, 2, 3, 4, 5}

# Intersection
intersection = tree{}
for x in a.to_list() {
    if b.contains(x) {
        intersection.insert(x)
    }
}
# intersection = tree{3}
```

### Priority queue (min-heap behavior)

```graphoid
pq = tree{}

# Insert with priorities
pq.insert(5)
pq.insert(3)
pq.insert(7)
pq.insert(1)

# Get minimum (highest priority)
while not pq.is_empty() {
    min_val = pq.min()
    print(min_val)
    pq.remove(min_val)
}
# Output: 1 3 5 7
```

### Range statistics

```graphoid
scores = tree{65, 72, 85, 90, 95, 78, 88}

# Students who passed (>= 70)
passing = scores.greater_than(70, true)
pass_rate = passing.length() / scores.size()

# Grade distribution
a_students = scores.greater_than(90, true).length()
b_students = scores.range(80, 89).length()
c_students = scores.range(70, 79).length()
```

---

## Performance Characteristics

| Operation | Average | Worst Case |
|-----------|---------|------------|
| Insert | O(log n) | O(n)* |
| Remove | O(log n) | O(n)* |
| Search | O(log n) | O(n)* |
| Min/Max | O(log n) | O(n)* |
| Range | O(log n + k) | O(n) |
| Traversal | O(n) | O(n) |

\* Worst case for unbalanced tree. Use `type: :balanced` or call `rebalance()` for guaranteed O(log n).

---

## See Also

- [list](list.md) - List/array type
- [hash](hash.md) - Hash/dictionary type
- [graph](graph.md) - Graph data structure
- [User Guide: Collections](../../user-guide/05-collections.md) - Collections tutorial
- [User Guide: Graph Operations](../../user-guide/06-graph-operations.md) - Graph algorithms
