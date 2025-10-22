# START HERE - Next Session Quick Start

**Last Updated**: October 22, 2025
**Current Status**: 🔧 REFACTOR REQUIRED - Option A Implementation
**Tests Passing**: 388/388 (100%)
**What's Next**: Execute refactor plan to align with philosophy

---

## 📖 What Happened Last Session

**Last session was a DESIGN REVIEW session**, not a coding session.

### What Was Discovered
The previous implementation (Phase 6 Week 1) created separate `Tree` and `Graph` types, which **violates the core Graphoid philosophy** that "trees are graphs with rules."

### What Was Done
- ✅ Identified the design inconsistency
- ✅ Chose Option A: Pure Philosophy approach
- ✅ Updated ALL documentation to reflect Option A
- ✅ Created detailed refactor plan (REFACTOR_PLAN_OPTION_A.md)

### What Was NOT Done
- ❌ No code changes made
- ❌ Implementation still has separate Tree/Graph types
- ❌ Refactor not yet executed

**Result**: Documentation is ready, code needs to be updated to match.

---

## ⚠️ CRITICAL: Why This Refactor Matters

### Current State (WRONG)
```rust
// src/values/tree.rs exists (separate file)
pub struct Tree { ... }

// src/values/mod.rs
pub enum Value {
    Graph(Graph),
    Tree(Tree),  // ❌ Separate variant
}

// Graphoid code
t = tree {}      // Creates Value::Tree
g = graph {}     // Creates Value::Graph
// These are DIFFERENT types!
```

### Required State (CORRECT - Option A)
```rust
// NO src/values/tree.rs file!

// src/values/mod.rs
pub enum Value {
    Graph(Graph),  // ✅ Only Graph variant
}

// Graphoid code
t = tree {}      // Desugars to: graph{}.with_ruleset(:tree)
g = graph {}     // Creates Value::Graph
// Both create the SAME type!
```

### Core Philosophy
1. **Trees are NOT a separate type** - Only `Value::Graph` exists
2. **tree{} is syntactic sugar** - Parser transforms to `graph{}.with_ruleset(:tree)`
3. **All methods on Graph** - Traversals, insert, etc. work on any graph
4. **Rules enforce constraints** - `:tree` ruleset provides tree behavior

---

## 🎯 This Session's Goal

**Execute REFACTOR_PLAN_OPTION_A.md to align implementation with philosophy**

### Timeline
- **2-3 days** total
- **Day 1**: Extend Graph with tree functionality
- **Day 2**: Remove Tree type, update tests
- **Day 3**: Polish and verify

### Success Criteria
- ✅ No `src/values/tree.rs` file exists
- ✅ No `Value::Tree` variant exists
- ✅ tree{} desugars to graph{}.with_ruleset(:tree)
- ✅ All tests pass (with updates)
- ✅ Zero compiler warnings

---

## 📚 MUST READ Before Starting (15 minutes)

### 1. The Refactor Plan (10 min) - PRIMARY GUIDE
```bash
less /home/irv/work/grang/rust/REFACTOR_PLAN_OPTION_A.md
```

This is your **step-by-step implementation guide**:
- Problem explanation
- Solution approach (Option A)
- **7 detailed steps** with code examples
- Testing strategy
- Timeline and success criteria

**Read this first!**

### 2. Updated Roadmap (3 min)
```bash
less /home/irv/work/grang/dev_docs/RUST_IMPLEMENTATION_ROADMAP.md
# Search for "Phase 6" (press / then type "Phase 6")
```

Shows the correct Phase 6 approach:
- Week 1: Graph only (no separate Tree)
- Week 2: Rules and tree{} syntax sugar

### 3. Last Session Summary (2 min)
```bash
less /home/irv/work/grang/rust/SESSION_SUMMARY.md
```

Explains what was done last session (doc updates, not code).

---

## 🗺️ The Refactor Plan (Overview)

Here are the 7 steps from REFACTOR_PLAN_OPTION_A.md:

### Step 1: Extend Graph with Tree Functionality (1 day)
**File**: `src/values/graph.rs`

Add these methods to Graph:
- `insert(value, parent?)` - Tree-like insertion
- `contains(value)` - Search for value
- `bfs(start)` - Breadth-first traversal
- `dfs(start)` - Depth-first traversal
- `in_order(start)` - In-order traversal
- `pre_order(start)` - Pre-order traversal
- `post_order(start)` - Post-order traversal

**Use TDD**: Write test → Implement → Verify

### Step 2: Remove Tree from Value Enum (30 min)
**File**: `src/values/mod.rs`

Remove `Tree(Tree)` variant from Value enum.
Update all Value methods (type_name, to_string_value, etc.).

### Step 3: Update Parser (1 hour)
**File**: `src/parser/mod.rs`

Change tree{} parsing to desugar into:
```rust
tree{} → graph{}.with_ruleset(:tree)
```

### Step 4: Update Executor (30 min)
**File**: `src/execution/executor.rs`

Remove `eval_tree()` function.
Remove Expr::Tree match arm from eval_expr().

### Step 5: Delete tree.rs (5 min)
```bash
rm src/values/tree.rs
```
Remove imports from mod.rs.

### Step 6: Update Tests (1 day)
**File**: `tests/integration_tests.rs`

Update all tree tests to expect Graph instead of Tree.
Add new tests for tree-as-graph functionality.

### Step 7: Update AST (30 min)
**File**: `src/ast/mod.rs`

Remove `Expr::Tree` variant (if present).

---

## 🚀 Quick Start Commands

### Verify Current State
```bash
cd /home/irv/work/grang/rust

# Should show 388 tests passing
~/.cargo/bin/cargo test 2>&1 | grep "test result:"

# Should build with zero warnings
~/.cargo/bin/cargo build 2>&1 | grep -i warning
```

**Expected**: 388/388 tests, zero warnings

### Start Refactor - Step 1
```bash
# Read the refactor plan
less REFACTOR_PLAN_OPTION_A.md

# Open graph.rs to add tree methods
# Start with insert() method

# TDD approach for each method:
# 1. Write test in tests/unit/graph_tests.rs
# 2. Run: ~/.cargo/bin/cargo test test_graph_insert
# 3. Implement method in src/values/graph.rs
# 4. Run: ~/.cargo/bin/cargo test test_graph_insert (should pass)
# 5. Run: ~/.cargo/bin/cargo test (all should pass)
# 6. Commit
```

### Ask Claude Code

**If using Claude Code, say:**

> "Execute Step 1 of REFACTOR_PLAN_OPTION_A.md: Extend Graph with tree functionality (insert, contains, traversals). Follow TDD. Keep all tests passing."

Claude will:
1. Read REFACTOR_PLAN_OPTION_A.md
2. Add tree methods to Graph (insert, contains, traversals)
3. Write tests for each method (TDD)
4. Keep all 388 tests passing throughout
5. Maintain zero warnings

After Step 1 is complete, continue with:

> "Execute Step 2 of REFACTOR_PLAN_OPTION_A.md: Remove Tree from Value enum."

And so on through all 7 steps.

---

## 🎯 Step 1 Details (Start Here)

### Goal
Add tree functionality to Graph struct so it can handle both graphs and trees.

### Methods to Add

1. **insert(value, parent?) → String**
   ```rust
   pub fn insert(&mut self, value: Value, parent: Option<&str>) -> String {
       // Generate node ID
       // Add node
       // If parent specified, add edge
       // Return node ID
   }
   ```

2. **contains(value) → bool**
   ```rust
   pub fn contains(&self, value: &Value) -> bool {
       // Search all nodes for matching value
   }
   ```

3. **bfs(start) → Vec<String>**
   ```rust
   pub fn bfs(&self, start: &str) -> Vec<String> {
       // Standard BFS - return node IDs in order
   }
   ```

4. **dfs(start) → Vec<String>**
   ```rust
   pub fn dfs(&self, start: &str) -> Vec<String> {
       // Standard DFS - return node IDs in order
   }
   ```

5. **in_order(start) → Vec<Value>**
   ```rust
   pub fn in_order(&self, start: &str) -> Vec<Value> {
       // Assumes binary tree (left/right edges)
       // Return values in in-order: left, root, right
   }
   ```

6. **pre_order(start) → Vec<Value>**
   ```rust
   pub fn pre_order(&self, start: &str) -> Vec<Value> {
       // Return values in pre-order: root, left, right
   }
   ```

7. **post_order(start) → Vec<Value>**
   ```rust
   pub fn post_order(&self, start: &str) -> Vec<Value> {
       // Return values in post-order: left, right, root
   }
   ```

### TDD Workflow

For **each method** above:

1. **RED**: Write failing test
   ```rust
   // tests/unit/graph_tests.rs
   #[test]
   fn test_graph_insert() {
       let mut g = Graph::new(GraphType::Directed);
       let node_id = g.insert(Value::Number(5.0), None);
       assert_eq!(g.node_count(), 1);
       assert!(g.has_node(&node_id));
   }
   ```

2. **RUN**: `~/.cargo/bin/cargo test test_graph_insert` (should FAIL)

3. **GREEN**: Implement method in src/values/graph.rs

4. **RUN**: `~/.cargo/bin/cargo test test_graph_insert` (should PASS)

5. **VERIFY**: `~/.cargo/bin/cargo test` (all 388+ should PASS)

6. **BUILD**: `~/.cargo/bin/cargo build` (zero warnings)

7. **COMMIT**: Small commit for this one method

### Tips

- **Look at tree.rs** for reference implementations
  - Contains BST insertion logic
  - Contains traversal implementations
  - Extract and adapt for Graph structure

- **Start simple**: Begin with insert() and contains()

- **Test incrementally**: Don't implement all methods before testing

- **Keep tests passing**: All 388 tests should pass after each method

---

## 📊 Progress Tracking

Use this checklist to track refactor progress:

### Step 1: Extend Graph (Day 1)
- [ ] Add insert() method + test
- [ ] Add contains() method + test
- [ ] Add bfs() method + test
- [ ] Add dfs() method + test
- [ ] Add in_order() method + test
- [ ] Add pre_order() method + test
- [ ] Add post_order() method + test
- [ ] All 388+ tests passing
- [ ] Zero warnings

### Step 2-5: Remove Tree Type (Day 2, Morning)
- [ ] Remove Tree from Value enum
- [ ] Update Value methods
- [ ] Update parser (desugar tree{})
- [ ] Update executor (remove eval_tree)
- [ ] Delete src/values/tree.rs
- [ ] All tests passing (with updates)

### Step 6: Update Tests (Day 2, Afternoon)
- [ ] Update tree creation tests
- [ ] Add tree-as-graph tests
- [ ] Add traversal tests
- [ ] All tests passing

### Step 7: Polish (Day 3)
- [ ] Update AST if needed
- [ ] Final test verification
- [ ] REPL testing
- [ ] Documentation check

---

## 🔍 Verification Commands

After each step, run these to verify correctness:

```bash
# All tests pass
~/.cargo/bin/cargo test

# Zero warnings
~/.cargo/bin/cargo build 2>&1 | grep -i warning

# Count tests passing
~/.cargo/bin/cargo test 2>&1 | grep "test result:"

# Try in REPL
~/.cargo/bin/cargo run
> g = graph {}
> # (whatever works at this step)
```

---

## 🎓 Key Insights

### Why This Refactor Is Important

1. **Philosophy First**: Graphoid's value is "everything is a graph"
2. **Dogfooding**: Using rules to define trees proves the rule system works
3. **Code Quality**: One implementation is simpler than two
4. **Flexibility**: Any graph can use tree methods, not just "trees"
5. **Teaching**: Code should exemplify the principles

### What Makes This Safe

1. **Tests**: 388 passing tests catch regressions
2. **TDD**: Test-first approach for new functionality
3. **Incremental**: Small steps with verification
4. **Reversible**: Git history preserves working state
5. **Clear Plan**: Detailed guide reduces uncertainty

---

## ⚠️ Common Pitfalls to Avoid

### Don't Do This

1. ❌ **Implement all methods before testing**
   - ✅ Do: One method at a time with TDD

2. ❌ **Let tests break and fix later**
   - ✅ Do: Keep all tests passing throughout

3. ❌ **Skip reading REFACTOR_PLAN_OPTION_A.md**
   - ✅ Do: Read the plan first (has code examples!)

4. ❌ **Try to keep Tree type "for now"**
   - ✅ Do: Complete refactor to eliminate it

5. ❌ **Forget to delete tree.rs**
   - ✅ Do: Delete in Step 5 (not before!)

---

## 📁 Key Files Reference

### Will Read
- `rust/REFACTOR_PLAN_OPTION_A.md` - **PRIMARY GUIDE**
- `rust/src/values/tree.rs` - Reference for BST/traversal logic
- `rust/src/values/graph.rs` - Where tree methods will be added
- `dev_docs/RUST_IMPLEMENTATION_ROADMAP.md` - Updated Phase 6 spec

### Will Modify
- `src/values/graph.rs` - Add tree methods (Step 1)
- `src/values/mod.rs` - Remove Tree variant (Step 2)
- `src/parser/mod.rs` - Desugar tree{} (Step 3)
- `src/execution/executor.rs` - Remove eval_tree (Step 4)
- `tests/unit/graph_tests.rs` - Add tests
- `tests/integration_tests.rs` - Update tests

### Will Delete
- `src/values/tree.rs` - Remove in Step 5

---

## 💡 Success Looks Like

### After Step 1 (Day 1)
```rust
// Graph has all tree methods
let mut g = Graph::new(GraphType::Directed);
let root = g.insert(Value::Number(5.0), None);
g.insert(Value::Number(3.0), Some(&root));
g.insert(Value::Number(7.0), Some(&root));

assert!(g.contains(&Value::Number(5.0)));
let values = g.in_order(&root);
// [3, 5, 7]
```

### After Complete Refactor (Day 3)
```graphoid
# In Graphoid code:
t = tree {}              # Creates a Graph
g = graph {}             # Creates a Graph

# Same methods work on both!
t.insert(5, none)
g.insert(5, none)

t.in_order(start)
g.in_order(start)

# tree{} has rules enforced
t.add_node("orphan", 10)  # May fail with rule violation
```

---

## 🚦 Ready to Start?

**You have everything you need:**
- ✅ Clear problem statement
- ✅ Detailed 7-step plan
- ✅ Code examples for each step
- ✅ TDD workflow defined
- ✅ Success criteria clear
- ✅ Timeline realistic (2-3 days)

**Next command:**
```bash
cd /home/irv/work/grang/rust
less REFACTOR_PLAN_OPTION_A.md
```

**Or ask Claude Code:**
> "Execute Step 1 of REFACTOR_PLAN_OPTION_A.md: Extend Graph with tree functionality. Follow TDD."

---

**Let's align the implementation with the philosophy! 🚀**
