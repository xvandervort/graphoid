# Class-Like Graphs: Gap Analysis

**Created**: December 5, 2025
**Updated**: December 10, 2025
**Purpose**: Document what's complete vs what's missing for a "full-featured" graph object system

## Related Documents

- **`CLG_DESIGN_ANALYSIS.md`** - Comprehensive comparison with OOP, design decisions, unique graph opportunities
- **`CLG_IMPLEMENTATION_ROADMAP.md`** - Prioritized implementation plan (Phases 13-18)
- **`CLASS_LIKE_GRAPHS_PLAN.md`** - Original implementation plan (Phases 1-6, COMPLETE)
- **`GRAPH_METHODS_NEXT_STEPS.md`** - Phases 7-12 implementation log (COMPLETE)

---

## What Works

### ✅ Core Method System
- `graph Name { fn method() { } }` unified declaration syntax
- Methods are stored as graph nodes in `__methods__` branch
- `self` binding works inside methods (explicit or implicit)
- Mutation persistence works (changes to `self` persist to original variable)
- Method chaining works with explicit assignment: `x = x.method1().method2()`
- Single method calls persist mutations directly

### ✅ Instance Creation
- `graph.clone()` creates deep copy including methods
- `.new()` convention works as documented
- Cloned instances are fully independent

### ✅ Runtime Modification
- Methods can be added to instances at runtime
- Methods can be added to prototypes at runtime
- Method redefinition works
- New instances get new prototype methods

### ✅ Structure Manipulation
- Methods can call `add_node()`, `remove_node()`, `add_edge()`, `remove_edge()`
- Graph structure can be modified from within methods
- Node values can be modified (via remove + add pattern)

---

## What Doesn't Work / Is Missing

### ✅ Rule Enforcement (Phase 7 - COMPLETE)
**Status**: Implemented December 8, 2025

The following graph methods are now available:
- `g.add_rule(:no_cycles)` - Add validation rule
- `g.add_rule(:max_degree, 2)` - Add parameterized rule
- `g.remove_rule(:no_cycles)` - Remove a rule
- `g.has_rule(:no_cycles)` - Check if rule is active

Rules properly scope to data layer only (ignore `__methods__` branch).

See example: `samples/03-advanced/graph_rules.gr`

### ✅ Internal Nodes Hidden (Phase 8 - COMPLETE)
**Status**: Implemented December 8, 2025

Internal nodes are hidden by default, with `:all` option to show everything:

**Default (data layer only):**
- `g.nodes()` - Returns only data nodes
- `g.edges()` - Returns only data edges
- `g.node_count()` - Count of data nodes only
- `g.edge_count()` - Count of data edges only

**With :all (show internal):**
- `g.nodes(:all)` - All nodes including `__methods__` branch
- `g.edges(:all)` - All edges including method branch edges

See example: `samples/03-advanced/graph_layer_visibility.gr`

### ✅ Graph Visualization (Phase 9 - COMPLETE)
**Status**: Implemented December 8, 2025

Three visualization methods available:
- `g.visualize()` / `g.visualize(:all)` - Text representation
- `g.to_dot()` / `g.to_dot(:all)` - Graphviz DOT format
- `g.to_ascii()` / `g.to_ascii(:all)` - ASCII tree display

All support `:all` parameter to include internal nodes.

See example: `samples/03-advanced/graph_visualization.gr`

### ❌ More Visualization Features (Future)
Additional visualization features that could be added:

**Default behavior**: Show only data layer (filter out `__methods__`, `__config__` branches)

**Directives/options** to control what's shown:
```graphoid
# Text-based visualization
g.visualize()                           # Data only (default)
g.visualize(:all)                       # Everything
g.visualize(:data)                      # Explicit data only
g.visualize(:methods)                   # Methods layer
g.visualize(:config)                    # Config layer
g.visualize({ layers: [:data, :methods] })  # Multiple layers

# Output formats
g.to_dot()                              # Graphviz DOT format
g.to_json()                             # JSON structure
g.to_ascii()                            # ASCII art tree/graph
```

**Layer visibility flags**:
- `:data` - User data nodes (default visible)
- `:methods` - `__methods__` branch (hidden by default)
- `:config` - `__config__` branch (hidden by default)
- `:all` - Everything including internal structure

### ❌ Edge Serialization
`edges()` returns `[list, list, list]` which is not human-readable. Should return structured data like:
```
[["A", "B", "connected"], ["B", "C", "connected"]]
```

### ❌ Node Value Mutation
Direct mutations to complex values inside nodes don't persist:
```graphoid
items = g.get_node("items")  # Returns copy
items.append(x)              # Mutates copy
# Original node unchanged!
```

Workaround: Remove and re-add the node. But this is clunky.

### ❌ Property Access Syntax
No shorthand for accessing node values:
```graphoid
# Current (verbose)
value = self.get_node("count")

# Desired (shorthand)
value = self.count  # or self["count"]
```

### ❌ Private Methods
All methods are public. No way to mark methods as internal:
```graphoid
fn Graph._helper() { }  # Would be nice to have private methods
```

### ❌ Constructor Parameters
No type checking on `.new()` parameters. Since `.new()` is just a convention, there's no enforcement of:
- Required parameters
- Parameter types
- Default values

### ❌ Inheritance / Prototype Chain
No way to create a graph that inherits methods from another:
```graphoid
ChildGraph = ParentGraph.extend()  # Doesn't exist
```

Currently must manually clone and add methods.

### ❌ Static Methods
No way to define methods that don't use `self`:
```graphoid
fn Graph.static_helper(x) { }  # Acts like instance method
```

---

## Architectural Principles

### This is NOT a Separate System

**Critical**: Graph objects are NOT a parallel concept - they are simply **graphs with methods attached**. They must integrate seamlessly with all existing graph capabilities:

- Graph rules (no_cycles, single_root, etc.) apply normally
- Pattern matching works on graph objects
- Subgraph extraction works
- All graph algorithms (paths, distance, etc.) work
- Visualization shows the graph structure

The `__methods__` and `__config__` branches are just **reserved namespace conventions**, not special magic.

### Data Layer Can Be Structured Graphs

The data portion of a graph object might itself be a BST, linked list, DAG, or any other graph structure:

```graphoid
BST = graph { type: :directed }
BST.add_rule("no_cycles")
BST.add_rule("max_children_2")

fn BST.insert(value) {
    # Maintains BST invariants in the DATA layer
    # Rules apply to data nodes, not __methods__ branch
}
```

This must NOT conflict with method/config layers. Rules like `max_children_2` should apply to the **data layer only**, not count method nodes.

### ✅ Method Constraints (Phase 11 - COMPLETE)
**Status**: Implemented December 9, 2025

Method constraints are implemented as rules (not config), using the existing `add_rule()` system:

```graphoid
g = graph{}
g.add_rule(:no_node_additions)   # Methods cannot add nodes
g.add_rule(:no_node_removals)    # Methods cannot remove nodes
g.add_rule(:no_edge_additions)   # Methods cannot add edges
g.add_rule(:no_edge_removals)    # Methods cannot remove edges
g.add_rule(:read_only)           # Methods cannot modify the graph at all
```

**Design decision**: Used rules instead of config because:
- Consistent with existing rule system
- Can be queried with `has_rule()` and `rule()`
- Can be removed with `remove_rule()`
- Simpler implementation

**Enforcement**: Constraints are checked in `call_graph_method()` by comparing graph state before and after method execution.

See example: `samples/03-advanced/graph_method_constraints.gr`

### ❌ Custom Rules for Methods (Future)

Power users could define custom validation rules for the method layer:

```graphoid
fn validate_method_purity(graph, method_name, before_state, after_state) {
    # Custom validation that runs after method execution
    # Can check invariants, log changes, etc.
}

MyGraph.add_method_rule(validate_method_purity)
```

This is advanced functionality - most users won't need it.

---

## Architectural Questions

### ✅ Rules Extended with rule() (Phase 10 - COMPLETE)
**Status**: Implemented December 8, 2025

Instead of creating a separate config layer, extended the rules system:
- `g.rule(:max_degree)` - Returns parameter value (e.g., 5)
- `g.rule(:no_cycles)` - Returns true for non-parameterized rules
- `g.rule(:nonexistent)` - Returns none if rule doesn't exist

**Design rationale**: Configuration IS rules. No need for separate systems.

### Should methods be removable?
**Current**: No `remove_method()` exists
**Recommendation**: Add for flexibility

### Should the `__methods__` branch be truly hidden?
**Current**: Visible in `nodes()` output
**Recommendation**: Yes, filter automatically by default, with opt-in visibility

### How do graph rules interact with layers?
**Question**: When `add_rule("max_children_2")` is called, should it:
- Apply to all nodes (including __methods__)? **No**
- Apply only to data nodes? **Yes - this is the expected behavior**

Rules should scope to the data layer by default, with explicit `:all_layers` option if needed.

---

## Priority Recommendations

### High Priority (Core functionality)
1. ~~**Graph rules enforcement**~~ - ✅ DONE (Phase 7)
2. **Hide internal nodes** - Expose `data_nodes()` method (Phase 8)
3. ~~**Fix edge serialization**~~ - ✅ Already returns `[["A", "B", "type"]]` format
4. **Graph visualization** - Essential for debugging and understanding (Phase 9)

### Medium Priority (Developer experience)
5. **Property access syntax** - `self.property` shorthand
6. **Remove method capability** - For dynamic systems
7. **Better node value mutation** - Direct mutation persistence

### Low Priority (Nice to have)
8. **Private methods** - Encapsulation
9. **Static methods** - Utility functions
10. **Prototype chain** - Inheritance patterns

---

## Naming Alternatives

"Class-like graphs" is misleading - it implies a separate system. These are just **graphs with methods attached**.

Options:

1. **No special name** - Just call them "graphs" - methods are a graph feature, not a graph type
2. **Graph Objects** - Familiar to OOP developers, clear meaning
3. **Behavioral Graphs** - Emphasizes methods as behaviors
4. **Method Graphs** - Technical but precise
5. **Object Graphs** - Most familiar to OOP developers

**Recommendation**: **No special name needed**.

A graph can have:
- Data nodes (the "data layer")
- Method nodes (the `__methods__` branch)
- Config nodes (the `__config__` branch)

This is just what graphs ARE in Graphoid. The phrase "graph with methods" is sufficient when distinguishing. Calling it something special implies it's different from "regular graphs" - but it's not.

---

## Implementation Notes

### Method Storage Structure

The current implementation stores methods correctly as graph nodes:
```
__methods__              (container, type: "__branch__")
__methods__/add          (method node, value: Function, type: "__method__")
__methods__/remove       (method node, value: Function, type: "__method__")

Edges: __methods__ --has_method--> __methods__/add
```

This follows "everything is a graph" - methods ARE part of the graph structure, just in a dedicated branch.

### Rule Scoping Implementation

When implementing `add_rule()` for graphs, rules MUST scope to data layer by default:

```rust
// In rule validation, skip internal branches
fn validate_rule(&self, rule: &Rule) -> bool {
    for node_id in self.data_node_ids() {  // NOT self.nodes.keys()
        // Validate only data nodes
    }
}
```

The `data_node_ids()` helper already exists - it filters out `__methods__/*` nodes.

### Reserved Namespaces

These prefixes are reserved for internal graph structure:
- `__methods__` - Method storage branch ✅ Implemented
- `__meta__` - Metadata storage (future)

Note: `__config__` was considered but rejected in favor of extending the rules system.

User data nodes should NOT use double-underscore prefixes.

---

## Test File Summary

Test files created during analysis:
- `/tmp/test_method_data_rules.gr` - Data manipulation (partial success)
- `/tmp/test_structure_manipulation.gr` - Structure manipulation (works)
- `/tmp/test_runtime_modification.gr` - Runtime changes (works)
- `/tmp/test_rules_enforcement.gr` - Rules (FAILS - not implemented)

---

## Conclusion

The class-like graphs implementation is **COMPLETE** through Phase 23. All planned features are implemented and working.

### Implementation Status Summary

**Phases 1-6** (Core CLG): ✅ COMPLETE
- Method attachment, storage, dispatch, cloning, .new() convention

**Phases 7-12** (Enhancements): ✅ COMPLETE
- Rule enforcement, layer visibility, visualization, rule querying, method constraints, remove_method

**Phases 13-23** (Advanced Features): ✅ COMPLETE (December 10, 2025)
- Phase 13: Property access syntax (`self.property`)
- Phase 14: Inheritance with `from` keyword
- Phase 15: Private methods (underscore prefix)
- Phase 16: Super calls (`super.method()`)
- Phase 17: Computed properties (getters)
- Phase 18: Type checking (`is_a()`, `type_of()`)
- Phase 19: Setters
- Phase 20: Static methods
- Phase 21: Structure-based dispatch (`when` guards)
- Phase 22: Mixins (`include()`)
- Phase 23: Duck typing (`responds_to()`)

### Unified Graph Declaration Syntax (December 12, 2025)

**Phases 1-5 of GRAPH_DECLARATION_SYNTAX.md: ✅ COMPLETE**

The new unified syntax is now working:

```graphoid
graph Animal {
    configure { readable: :sound }
    rule :no_cycles

    sound: none

    fn speak() {
        print(sound)  # Implicit self!
    }
}

graph Dog from Animal {
    sound: "woof"
}
```

**What's implemented:**
- Phase 1: Named graph declaration `graph Name { }`
- Phase 2: Implicit self resolution
- Phase 3: Inheritance with `from` keyword
- Phase 4: Configure block (`readable`, `writable`, `accessible`)
- Phase 5: Static methods in body, graph types, `rule` keyword

**Phase 6: ✅ COMPLETE** - Old external syntax removed (`fn Graph.method()` etc.)

See `GRAPH_DECLARATION_SYNTAX.md` for the complete design and implementation plan.
