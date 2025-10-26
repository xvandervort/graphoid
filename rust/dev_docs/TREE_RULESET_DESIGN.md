# Tree Ruleset Hierarchy Design

**Date**: January 2025
**Status**: APPROVED - Partial Implementation
**Decision**: Hierarchical tree rulesets with syntactic sugar

**⚠️ IMPLEMENTATION STATUS**:
- ✅ Ruleset **storage** implemented (graph.rulesets field)
- ✅ `with_ruleset()` and `has_ruleset()` methods work
- ✅ Parser desugaring works (tree{} → graph{}.with_ruleset(:tree))
- ❌ Ruleset **enforcement** NOT YET implemented
- ❌ Rule validation NOT YET implemented
- 📅 **Scheduled**: Phase 6 Week 2 (per RUST_IMPLEMENTATION_ROADMAP.md)

---

## Problem

The original `tree.rs` implementation was a **Binary Search Tree** (BST), but we need to support multiple tree types while maintaining the philosophy that "trees are graphs with rules."

## Solution: Hierarchical Rulesets

### Tree Type Hierarchy

```
graph
  └─ :tree (basic tree)
      └─ :binary_tree (max 2 children)
          └─ :bst (ordered insertion)
```

### Syntactic Sugar

| Syntax | Desugars To | Ruleset | Behavior |
|--------|-------------|---------|----------|
| `tree{}` | `graph{}.with_ruleset(:tree)` | `:tree` | Basic tree: no cycles, single root, connected |
| `BinaryTree{}` | `graph{}.with_ruleset(:binary_tree)` | `:binary_tree` | Binary tree: includes `:tree` + max 2 children |
| `BST{}` | `graph{}.with_ruleset(:bst)` | `:bst` | BST: includes `:binary_tree` + ordered insertion |

### Ruleset Definitions

#### `:tree` (Basic Tree)
Rules enforced:
- `no_cycles` - No circular paths
- `single_root` - Exactly one node with no parent
- `connected` - All nodes reachable from root

Example:
```graphoid
t = tree{}
# Creates a graph that can have any branching structure
# as long as it's acyclic and connected
```

#### `:binary_tree` (Binary Tree)
Includes all `:tree` rules plus:
- `max_children_2` - Each node has at most 2 children

Example:
```graphoid
bt = BinaryTree{}
# Can have left/right children, but no ordering requirement
```

#### `:bst` (Binary Search Tree)
Includes all `:binary_tree` rules plus:
- `bst_ordering` - Left child < parent < right child
- Automatic insertion ordering on `.insert()`

Example:
```graphoid
bst = BST{}
bst.insert(5)
bst.insert(3)
bst.insert(7)
# Automatically maintains BST property
```

---

## Implementation Plan

### Phase 1: Basic Tree (Current Refactor)
- ✅ Step 1: Add tree methods to Graph
- ✅ Step 2: Remove Tree from Value enum
- ⏳ Step 3: Parser desugars `tree{}` to `graph{}.with_ruleset(:tree)`
- ⏳ Step 5: Delete tree.rs
- ⏳ Step 6: Rewrite tests for basic tree behavior
- ⏳ Step 7: Remove Expr::Tree from AST

### Phase 2: Binary Tree (Future)
- Add `Expr::BinaryTree` to parser
- Desugar `BinaryTree{}` to `graph{}.with_ruleset(:binary_tree)`
- Implement `:binary_tree` ruleset validation
- Add tests

### Phase 3: BST (Future)
- Add `Expr::BST` to parser
- Desugar `BST{}` to `graph{}.with_ruleset(:bst)`
- Implement `:bst` ruleset validation
- Port old BST tests from tree.rs
- Add ordered insertion behavior

### Phase 4: Advanced Trees (Future)
Potential additional tree types:
- `AVLTree{}` → `:avl` (self-balancing)
- `RedBlackTree{}` → `:red_black` (self-balancing)
- `NaryTree{ max_children: 3 }` → `:nary_tree` (configurable branching)

---

## Benefits

1. **Flexibility**: Users can choose the right tree type for their needs
2. **Clarity**: Syntax makes the tree type explicit
3. **Extensibility**: Easy to add new tree types as rulesets
4. **Philosophy**: Reinforces "trees are graphs with rules"
5. **Performance**: Each ruleset can optimize for its constraints

---

## Migration from Old tree.rs

The old `tree.rs` was a BST. After refactor:

**Before** (old code):
```graphoid
t = tree{}
t.insert(5)  # BST insertion
```

**After** (new code):
```graphoid
# Basic tree (new default)
t = tree{}
t.insert(5, none)  # Manual parent specification

# BST (explicit)
bst = BST{}
bst.insert(5)  # Automatic BST ordering
```

---

## Notes

- All tree types can use the same Graph methods: `bfs()`, `dfs()`, `in_order()`, etc.
- Rulesets are enforced at mutation time (add_node, add_edge, insert)
- `tree{}` is the most general form - principle of least surprise
- More specialized types require explicit opt-in via syntax
