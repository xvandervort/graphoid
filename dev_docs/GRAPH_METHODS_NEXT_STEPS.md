# Graph Methods: Next Steps Plan

**Created**: December 5, 2025
**Status**: Ready for implementation
**Prerequisites**: Phases 1-6 complete (basic method attachment working)

---

## Summary

The basic method attachment system works:
- `fn Graph.method()` syntax ✅
- Methods stored as nodes in `__methods__` branch ✅
- `self` binding ✅
- Mutation persistence ✅
- `clone()` for instances ✅

What's missing: rule enforcement, layer visibility, config system, visualization.

---

## Phase 7: Graph Rule Enforcement ✅ COMPLETE

**Priority**: HIGH - Critical for data integrity
**Completed**: December 8, 2025

### Goal
Implement `graph.add_rule()` that applies rules to the DATA layer only.

### Implementation Summary
- Added `add_rule`, `remove_rule`, `has_rule` methods to executor
- Rules properly scope to data layer (ignore `__methods__` branch)
- `node_count()`, `node_ids()`, `edge_count()` now return data layer only
- Created 14 integration tests in `tests/graph_rules_graphoid_syntax_test.rs`
- Created example: `samples/03-advanced/graph_rules.gr`

### Tasks

1. **Add `add_rule()` method to graphs** in `eval_graph_method()`
   ```rust
   "add_rule" => {
       // Similar to list.add_rule() but scoped to data layer
   }
   ```

2. **Built-in rules to implement**:
   - `"no_cycles"` - Prevent cycles (DAG enforcement)
   - `"single_root"` - Tree constraint (one node with no predecessors)
   - `"max_children_N"` - Limit outgoing edges per node
   - `"connected"` - All nodes must be reachable
   - `"no_orphans"` - No disconnected nodes

3. **Rule scoping** - Rules MUST use `data_node_ids()` not `nodes.keys()`:
   ```rust
   fn validate_no_cycles(&self) -> bool {
       // Only check data nodes, skip __methods__ branch
       for node_id in self.data_node_ids() {
           // cycle detection logic
       }
   }
   ```

4. **Validation timing**:
   - After `add_node()` - validate new node
   - After `add_edge()` - validate new edge doesn't violate rules
   - After method execution - validate final state

### Tests
- `test_graph_rule_no_cycles`
- `test_graph_rule_single_root`
- `test_graph_rule_max_children`
- `test_rules_ignore_methods_branch`
- `test_rule_violation_error_message`

### Example file
Create `samples/03-advanced/graph_rules.gr`

---

## Phase 8: Hide Internal Nodes ✅ COMPLETE

**Priority**: HIGH - UX issue
**Completed**: December 8, 2025

### Goal
`nodes()` and `edges()` return only data layer by default.

### Implementation Summary
- `nodes()` returns data nodes only by default
- `nodes(:all)` returns all nodes including `__methods__` branch
- `edges()` returns data edges only by default
- `edges(:all)` returns all edges including method branch edges
- Added `data_edge_list()` method to graph.rs
- Created 10 integration tests in `tests/graph_layer_visibility_test.rs`
- Created example: `samples/03-advanced/graph_layer_visibility.gr`

### Tasks

1. **Expose `data_nodes()` method** (already exists in Rust as `data_node_ids()`):
   ```graphoid
   g.data_nodes()  # Returns only user data nodes
   g.nodes()       # CHANGE: Same as data_nodes() by default
   g.nodes(:all)   # Returns everything including __methods__
   ```

2. **Similarly for edges**:
   ```graphoid
   g.edges()       # Data layer edges only
   g.edges(:all)   # All edges including method branch edges
   ```

3. **Fix edge serialization** - Return readable format:
   ```graphoid
   g.edges()  # Returns [["A", "B", "connected"], ["B", "C", "child"]]
   # NOT [list, list, list]
   ```

### Tests
- `test_nodes_hides_methods_by_default`
- `test_nodes_all_includes_methods`
- `test_edges_readable_format`

---

## Phase 9: Basic Visualization ✅ COMPLETE

**Priority**: HIGH - Essential for debugging
**Completed**: December 8, 2025

### Goal
Add `visualize()` and export methods for graphs.

### Implementation Summary
- `visualize()` - Text representation with nodes and edges
- `visualize(:all)` - Includes `__methods__` branch
- `to_dot()` - Graphviz DOT format export
- `to_dot(:all)` - Includes internal nodes
- `to_ascii()` - ASCII tree display (works best for tree structures)
- `to_ascii(:all)` - Includes internal nodes
- Created 16 integration tests in `tests/graph_visualization_test.rs`
- Created example: `samples/03-advanced/graph_visualization.gr`

### Tasks

1. **`visualize()` method** - Text representation:
   ```graphoid
   g.visualize()           # Data layer only (default)
   g.visualize(:all)       # Everything
   g.visualize(:methods)   # Methods layer only
   g.visualize(:config)    # Config layer only
   ```

2. **`to_dot()` method** - Graphviz DOT format:
   ```graphoid
   dot_string = g.to_dot()
   # digraph G {
   #   "A" -> "B" [label="connected"];
   #   "B" -> "C" [label="child"];
   # }
   ```

3. **`to_ascii()` method** - ASCII art for trees:
   ```graphoid
   t.to_ascii()
   # root
   # ├── child1
   # │   └── grandchild
   # └── child2
   ```

4. **Layer visibility in all methods**:
   - Default: data only
   - `:all` - everything
   - `:methods` - `__methods__` branch
   - `:config` - `__config__` branch

### Tests
- `test_visualize_data_only`
- `test_visualize_all_layers`
- `test_to_dot_format`
- `test_to_ascii_tree`

### Example file
Create `samples/03-advanced/graph_visualization.gr`

---

## Phase 10: Extend Rules System ✅ COMPLETE

**Priority**: MEDIUM - Enables config-like functionality
**Completed**: December 8, 2025

### Goal
Extend the existing rules system to support retrieving rule parameters, rather than creating a separate config layer.

### Implementation Summary
- Added `rule()` method to retrieve rule parameter values
- `g.rule(:max_degree)` returns the max degree value (e.g., 5)
- `g.rule(:no_cycles)` returns true (for non-parameterized rules)
- `g.rule(:nonexistent)` returns none
- Created 3 integration tests in `tests/graph_rules_graphoid_syntax_test.rs`

### Design Decision
Instead of creating a separate `__config__` branch, we extend the rules system:
- Rules already store parameterized values (`:max_degree, 5`)
- `rule()` allows retrieving those values
- This avoids code duplication and keeps the design simple
- Configuration IS rules - there's no need for a separate concept

### Usage
```graphoid
g = graph{}
g.add_rule(:max_degree, 5)
val = g.rule(:max_degree)  # Returns 5
exists = g.has_rule(:max_degree)  # Returns true
g.remove_rule(:max_degree)
```

---

## Phase 11: Method Constraints (Advanced) ✅ COMPLETE

**Priority**: LOW - Power user feature
**Completed**: December 9, 2025

### Goal
Allow rules to constrain what methods can do.

### Implementation Summary
Method constraints are implemented as rules (not config), using the existing `add_rule()` system:

```graphoid
g = graph{}
g.add_rule(:no_node_additions)   # Methods cannot add nodes
g.add_rule(:no_node_removals)    # Methods cannot remove nodes
g.add_rule(:no_edge_additions)   # Methods cannot add edges
g.add_rule(:no_edge_removals)    # Methods cannot remove edges
g.add_rule(:read_only)           # Methods cannot modify the graph at all
```

### Design Decision
Used rules instead of config because:
- Consistent with existing rule system
- Can be queried with `has_rule()` and `rule()`
- Can be removed with `remove_rule()`
- Simpler implementation (no new config layer needed)

### Enforcement Mechanism
Constraints are enforced in `call_graph_method()` in executor.rs:
1. Before method execution: capture graph state (node IDs, edge count)
2. After method execution: compare states
3. If any constraint is violated, return an error

### Tests
- 28 integration tests in `tests/graph_method_constraints_test.rs`
- Tests for each constraint type (add, remove, read-only)
- Tests for constraint querying and removal
- Tests for combined constraints

### Example
See `samples/03-advanced/graph_method_constraints.gr`

---

## Phase 12: Remove Method Capability ✅ COMPLETE

**Priority**: LOW - Nice to have
**Completed**: December 9, 2025

### Goal
Allow removing methods at runtime.

### Implementation Summary
- Added `remove_method()` to Graph struct in `src/values/graph.rs`
- Exposed in executor via `g.remove_method("method_name")`
- Returns `true` if method was removed, `false` if it didn't exist
- 10 integration tests in `tests/graph_method_constraints_test.rs`
- Example in `samples/03-advanced/graph_method_constraints.gr`

### Usage
```graphoid
g = graph{}

fn g.my_method() {
    return 42
}

result = g.my_method()   # Returns 42

removed = g.remove_method("my_method")  # Returns true
# g.my_method() would now fail - method no longer exists

# Can re-add with different implementation
fn g.my_method() {
    return 100
}
```

### Tests
- `test_remove_method_basic`
- `test_remove_method_returns_true_when_found`
- `test_remove_method_returns_false_when_not_found`
- `test_remove_method_makes_method_uncallable`
- `test_remove_method_preserves_other_methods`
- `test_remove_method_does_not_affect_data_nodes`
- `test_remove_method_can_readd_method`
- `test_remove_method_on_clone_does_not_affect_original`
- `test_remove_method_wrong_arg_type_fails`
- `test_remove_method_wrong_arg_count_fails`

---

## Implementation Order

| Phase | Description | Priority | Depends On | Status |
|-------|-------------|----------|------------|--------|
| 7 | Graph rule enforcement | HIGH | - | ✅ COMPLETE |
| 8 | Hide internal nodes | HIGH | - | ✅ COMPLETE |
| 9 | Basic visualization | HIGH | Phase 8 | ✅ COMPLETE |
| 10 | Config layer foundation | MEDIUM | - | ✅ COMPLETE |
| 11 | Method constraints | LOW | Phase 10 | ✅ COMPLETE |
| 12 | Remove method | LOW | - | ✅ COMPLETE |

**All phases complete!** The class-like graphs feature is fully implemented.

---

## Success Criteria

After all phases:

- [x] `g.add_rule(:no_cycles)` works and enforces on data layer ✅ Phase 7
- [x] `g.nodes()` hides `__methods__` by default ✅ Phase 7/8
- [x] `g.nodes(:all)` shows everything ✅ Phase 8
- [x] `g.edges()` returns readable `[["A", "B", "type"]]` format ✅ Phase 8
- [x] `g.edges(:all)` shows all edges ✅ Phase 8
- [x] `g.visualize()` shows text representation ✅ Phase 9
- [x] `g.to_dot()` exports Graphviz format ✅ Phase 9
- [x] `g.to_ascii()` shows ASCII tree display ✅ Phase 9
- [x] `g.rule(:max_degree)` returns parameter value ✅ Phase 10
- [x] Rules can constrain method behavior ✅ Phase 11
- [x] `g.remove_method()` works ✅ Phase 12
- [x] All existing tests still pass ✅ 2200+ tests passing
- [x] Example files ✅ graph_rules.gr, graph_layer_visibility.gr, graph_visualization.gr, graph_method_constraints.gr

---

## Files to Modify

| File | Changes |
|------|---------|
| `src/values/graph.rs` | Rule validation, config storage, remove_method |
| `src/execution/executor.rs` | add_rule, visualize, to_dot, config methods |
| `tests/unit/graph_tests.rs` | New tests for all phases |
| `samples/03-advanced/` | New example files |
| `dev_docs/CLASS_LIKE_GRAPHS_GAPS.md` | Update as gaps are filled |

---

## Notes

- Rules scope to data layer by default (use `data_node_ids()`)
- Reserved prefixes: `__methods__`, `__config__`, `__meta__`
- This is NOT a separate system - it's an extension of graphs
- Visualization defaults to data layer only
- User data should avoid `__` prefixes

---

## Future Work: Pure Graphoid Rules

**Priority**: Future (after self-hosting progress)

All built-in graph rules (`:no_cycles`, `:single_root`, `:no_duplicates`, etc.) are currently implemented in Rust. As part of the self-hosting goal, these should eventually be converted to pure Graphoid implementations.

The pattern established by `add_method_constraint()` shows how this can work:
- User defines a constraint function in Graphoid
- Function receives graph state and returns true/false
- No Rust code changes required

**Target**: All rules definable in pure Graphoid, with Rust implementations only as optional performance optimizations (not requirements).

---

## Quick Start Next Session

```bash
cd /home/irv/work/grang
git status  # Should be on 'class_like_graphs' branch

# ALL PHASES COMPLETE! ✅
# Phases 7-12 of class-like graphs are done.
#
# Summary of implemented features:
# - Phase 7: Graph rule enforcement (add_rule, remove_rule, has_rule)
# - Phase 8: Layer visibility (nodes(), edges() hide __methods__ by default)
# - Phase 9: Visualization (visualize(), to_dot(), to_ascii())
# - Phase 10: Rule querying (rule() returns parameter values)
# - Phase 11: Method constraints (:read_only, :no_node_removals, :no_edge_removals, custom)
# - Phase 12: Remove method (remove_method())
#
# The class-like graphs feature branch is complete and ready for merge.

# Run tests:
~/.cargo/bin/cargo test --test graph_method_constraints_test  # 39 tests
~/.cargo/bin/cargo test --test graph_visualization_test
~/.cargo/bin/cargo test --test graph_layer_visibility_test
~/.cargo/bin/cargo test --test graph_rules_graphoid_syntax_test
```
