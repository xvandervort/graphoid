# Refactor Plan: Option A - Pure Philosophy

**Created**: October 22, 2025
**Status**: REQUIRED - Current implementation violates philosophy
**Estimated Time**: 2-3 days

---

## Problem

The current implementation (Phase 6 Week 1) created separate `Tree` and `Graph` types:
- `src/values/tree.rs` - Separate Tree struct with TreeNode
- `src/values/graph.rs` - Separate Graph struct
- `Value::Tree(Tree)` and `Value::Graph(Graph)` in enum

This **violates the core Graphoid philosophy**: Trees are graphs with rules, not a separate type.

---

## Solution: Option A (Pure Philosophy)

**Goal**: One `Graph` type that handles both graphs and trees. The `tree{}` syntax desugars to `graph{}.with_ruleset(:tree)`.

### Key Principles

1. **No separate Tree type** - Only `Value::Graph(Graph)`
2. **All methods on Graph** - Traversals, insertions work on any graph
3. **Rules enforce constraints** - `:tree` ruleset enforces no_cycles, single_root, etc.
4. **Syntax sugar** - `tree{}` is parsed as `graph{}.with_ruleset(:tree)`

---

## Refactor Steps

### Step 1: Extend Graph with Tree Functionality (1 day)

**File**: `src/values/graph.rs`

Add these methods to the `Graph` struct:

```rust
impl Graph {
    // Existing methods: add_node, add_edge, has_node, etc.

    // NEW: Convenience method for tree-like insertion
    pub fn insert(&mut self, value: Value, parent: Option<&str>) -> String {
        // Generate node ID (e.g., uuid or sequential)
        let node_id = self.generate_node_id();

        // Add the node
        self.add_node(node_id.clone(), value);

        // If parent specified, add edge
        if let Some(parent_id) = parent {
            self.add_edge(parent_id, &node_id, "child".to_string(), HashMap::new());
        }

        node_id
    }

    // NEW: Search for node containing value
    pub fn contains(&self, value: &Value) -> bool {
        self.nodes.values().any(|node| node.value == *value)
    }

    // NEW: BFS traversal
    pub fn bfs(&self, start: &str) -> Vec<String> {
        // Standard BFS implementation
        // Returns node IDs in BFS order
    }

    // NEW: DFS traversal
    pub fn dfs(&self, start: &str) -> Vec<String> {
        // Standard DFS implementation
        // Returns node IDs in DFS order
    }

    // NEW: In-order traversal (assumes binary tree structure)
    pub fn in_order(&self, start: &str) -> Vec<Value> {
        // Assumes each node has at most 2 children with "left" and "right" edges
        // Returns values in in-order (left, root, right)
    }

    // NEW: Pre-order traversal
    pub fn pre_order(&self, start: &str) -> Vec<Value> {
        // Returns values in pre-order (root, left, right)
    }

    // NEW: Post-order traversal
    pub fn post_order(&self, start: &str) -> Vec<Value> {
        // Returns values in post-order (left, right, root)
    }

    // Helper: Generate unique node ID
    fn generate_node_id(&self) -> String {
        // Simple approach: sequential numbers
        format!("node_{}", self.nodes.len())
    }
}
```

**Rationale**:
- `insert()` makes tree-like usage ergonomic
- Traversals work on ANY graph, not just trees
- BST ordering logic can be added later via rules/behaviors

---

### Step 2: Remove Tree Type (30 minutes)

**File**: `src/values/mod.rs`

**BEFORE**:
```rust
pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Graph(Graph),
    Tree(Tree),  // ❌ Remove this!
    // ...
}
```

**AFTER**:
```rust
pub enum Value {
    None,
    Bool(bool),
    Number(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Graph(Graph),  // ✅ Only Graph variant
    // ...
}
```

Update all Value methods:
- Remove Tree pattern matching
- `type_name()` - remove "tree" case
- `to_string_value()` - remove Tree formatting
- `is_truthy()` - remove Tree case

---

### Step 3: Update Parser (1 hour)

**File**: `src/parser/mod.rs`

Change tree{} parsing to desugar into graph{}.with_ruleset(:tree):

**Current parsing** (Week 1):
```rust
fn parse_tree(&mut self) -> Result<Expr> {
    // Parses tree{} into Expr::Tree
}
```

**New parsing** (Option A):
```rust
fn parse_tree(&mut self) -> Result<Expr> {
    // tree{} becomes:
    // graph{}.with_ruleset(:tree)

    // Parse tree literal as graph literal
    let graph_expr = self.parse_graph_literal()?;

    // Wrap in method call: .with_ruleset(:tree)
    Ok(Expr::MethodCall {
        receiver: Box::new(graph_expr),
        method: "with_ruleset".to_string(),
        args: vec![Expr::Symbol("tree".to_string())],
        pos: /* ... */
    })
}
```

**Rationale**: Parser desugars `tree{}` immediately, so runtime never sees "tree" - only graph with rules.

---

### Step 4: Update Executor (30 minutes)

**File**: `src/execution/executor.rs`

Remove `eval_tree()` function:

**BEFORE**:
```rust
fn eval_tree(&mut self, tree_expr: &TreeExpr) -> Result<Value> {
    // Creates Value::Tree(Tree::new())
}
```

**AFTER**:
```rust
// Deleted - tree{} desugars to graph{}.with_ruleset(:tree) in parser
```

Update `eval_expr()`:
```rust
fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
    match expr {
        Expr::Graph(g) => self.eval_graph(g),
        // Expr::Tree(t) => self.eval_tree(t),  // ❌ Remove this
        // ...
    }
}
```

---

### Step 5: Delete tree.rs (5 minutes)

```bash
rm src/values/tree.rs
```

Update `src/values/mod.rs`:
```rust
// mod tree;  // ❌ Remove this import
// pub use tree::*;  // ❌ Remove this re-export
```

---

### Step 6: Update Tests (1 day)

**Update integration tests** to reflect new structure:

**BEFORE** (Week 1):
```rust
#[test]
fn test_tree_creation() {
    let source = r#"
        t = tree {}
    "#;
    let t = execute_and_get(source, "t").unwrap();
    assert!(matches!(t, Value::Tree(_)));
}
```

**AFTER** (Option A):
```rust
#[test]
fn test_tree_creation() {
    let source = r#"
        t = tree {}
    "#;
    let t = execute_and_get(source, "t").unwrap();
    // tree{} desugars to graph, so expect Graph variant
    assert!(matches!(t, Value::Graph(_)));
}

#[test]
fn test_tree_has_tree_ruleset() {
    let source = r#"
        t = tree {}
        has_tree = t.has_ruleset(:tree)
    "#;
    let has_tree = execute_and_get(source, "has_tree").unwrap();
    // Note: This test requires ruleset support (Phase 6 Week 2)
    assert_eq!(has_tree, Value::Bool(true));
}
```

**Add new tests** for graph with tree methods:
```rust
#[test]
fn test_graph_insert_method() {
    let source = r#"
        g = graph {}
        g.insert(5, none)
        g.insert(3, some("node_0"))
        count = g.node_count()
    "#;
    let count = execute_and_get(source, "count").unwrap();
    assert_eq!(count, Value::Number(2.0));
}

#[test]
fn test_graph_in_order_traversal() {
    let source = r#"
        g = graph {}
        root = g.insert(5, none)
        g.insert(3, some(root))
        g.insert(7, some(root))
        values = g.in_order(root)
    "#;
    let values = execute_and_get(source, "values").unwrap();
    // Expect [3, 5, 7]
}
```

---

### Step 7: Update AST (if needed) (30 minutes)

**File**: `src/ast/mod.rs`

If `Expr::Tree` exists, consider whether to:

**Option A**: Remove `Expr::Tree` entirely
```rust
pub enum Expr {
    Graph(GraphExpr),
    // Tree(TreeExpr),  // ❌ Remove
    // ...
}
```

**Option B**: Keep `Expr::Tree` but have it desugar in parser
- Rationale: Preserves AST structure for debugging
- Parser transforms Tree → MethodCall before execution

**Recommendation**: Option A (remove) for simplicity.

---

## Testing Strategy

### Phase 1: Unit Tests (graph.rs)
```bash
~/.cargo/bin/cargo test --lib graph
```

Test new methods:
- `insert()` with/without parent
- `contains()` search
- `bfs()`, `dfs()` traversals
- `in_order()`, `pre_order()`, `post_order()` traversals

### Phase 2: Integration Tests
```bash
~/.cargo/bin/cargo test
```

Verify:
- `tree{}` creates a graph
- Graph methods work for both `graph{}` and `tree{}`
- All 388 existing tests still pass (may need updates)

### Phase 3: REPL Testing
```bash
~/.cargo/bin/cargo run
> g = graph {}
> g.insert(5, none)
> g.insert(3, some("node_0"))
> g.in_order("node_0")
```

---

## Success Criteria

After refactor, you should be able to:

### Create graphs and trees identically
```graphoid
g = graph {}
t = tree {}  # Both create Graph instances
```

### Use same methods on both
```graphoid
g.add_node("a", 100)
t.add_node("a", 100)  # Same method!

g.in_order("start")
t.in_order("start")    # Same method!
```

### Tree syntax desugars correctly
```graphoid
t = tree {}
# Parser transforms to:
# t = graph {}.with_ruleset(:tree)
```

### Tests pass
- ✅ All existing tests pass (with updates)
- ✅ New graph method tests pass
- ✅ Tree-as-graph tests pass
- ✅ Zero compiler warnings

---

## Estimated Timeline

| Task | Time | Status |
|------|------|--------|
| Step 1: Extend Graph | 1 day | ⬜ Todo |
| Step 2: Remove Tree from Value | 30 min | ⬜ Todo |
| Step 3: Update Parser | 1 hour | ⬜ Todo |
| Step 4: Update Executor | 30 min | ⬜ Todo |
| Step 5: Delete tree.rs | 5 min | ⬜ Todo |
| Step 6: Update Tests | 1 day | ⬜ Todo |
| Step 7: Update AST | 30 min | ⬜ Todo |
| **TOTAL** | **~2-3 days** | |

---

## Notes

### BST Insertion Logic

The current `tree.rs` has BST insertion logic that compares values. This can be:

**Short-term**: Manual BST logic in `insert()` (for now)
```rust
pub fn insert_bst(&mut self, value: Value, root: Option<&str>) -> String {
    // If root not specified, find it
    // Compare value with node values
    // Insert left or right based on comparison
}
```

**Long-term**: Rules/behaviors (Phase 7)
- Add `:bst` ruleset that modifies insert() behavior
- Behavior transforms insert to follow BST ordering

### Ruleset Implementation

The `.with_ruleset(:tree)` method requires:
1. `Graph` struct has a `rules: Vec<Rule>` field
2. `with_ruleset()` method applies a predefined ruleset
3. Rules validate on mutations (add_node, add_edge)

This can be:
- **Minimal** (Phase 6): Store ruleset name, no enforcement yet
- **Full** (Phase 6 Week 2 or Phase 7): Complete rule system

---

## Benefits of Option A

1. **True to philosophy** - Trees ARE graphs, not separate types
2. **Code reuse** - One implementation, not two
3. **Flexibility** - Any graph can use tree methods
4. **Dogfooding** - Uses graph rules to define trees (core feature!)
5. **Simplicity** - Less code, fewer variants

---

## Risks

1. **Test breakage** - Many tests may need updates
2. **Complexity** - BST logic on graphs is more complex than dedicated tree
3. **Performance** - Graph operations may be slower than optimized tree (measure!)

**Mitigation**:
- Update tests incrementally
- Use TDD throughout refactor
- Profile performance after (optimize if needed)

---

## Next Steps

1. Read this document
2. Read updated RUST_IMPLEMENTATION_ROADMAP.md Phase 6
3. Start with Step 1: Extend Graph
4. Follow TDD: write test, implement, verify
5. Keep all tests passing throughout

**Ready to refactor!**
