# Phase 15: Namespace as Graph

**Duration**: 7-10 days
**Priority**: Critical (Blocker for compilation)
**Dependencies**: None (can start immediately)
**Status**: Planning

---

## Goal

Replace the HashMap-based `Environment` with a graph-based namespace where:
- Variables are nodes with binding edges to values
- Scopes are subgraphs with parent edges
- Variable lookup is graph traversal
- Closures are captured edges to outer scope nodes

---

## Current Implementation (Problem)

```rust
// src/execution/environment.rs - CURRENT
pub struct Environment {
    variables: HashMap<String, Value>,      // ← NOT a graph!
    parent: Option<Box<Environment>>,       // ← Lexical chain, NOT graph edges
}

impl Environment {
    pub fn get(&self, name: &str) -> Option<&Value> {
        // HashMap lookup, then recurse to parent
        self.variables.get(name).or_else(|| {
            self.parent.as_ref().and_then(|p| p.get(name))
        })
    }
}
```

**Problems**:
1. Variables are HashMap entries, not graph nodes
2. Scopes are struct nesting, not subgraphs
3. Variable lookup is HashMap::get, not traversal
4. No graph operations possible on namespace

---

## Target Implementation

### Core Data Structures

```rust
// src/namespace/mod.rs - NEW

use crate::graph::Graph;
use crate::values::Value;

/// A variable binding in the namespace graph
pub struct VariableNode {
    pub name: String,
    pub scope_id: ScopeId,
}

/// A scope in the namespace graph
pub struct ScopeNode {
    pub id: ScopeId,
    pub name: String,          // "global", "fn:add", "block:123"
    pub scope_type: ScopeType,
}

pub enum ScopeType {
    Global,
    Function(String),  // Function name
    Block,
    Module(String),    // Module name
    Class(String),     // Class name
}

/// The namespace graph
pub struct NamespaceGraph {
    graph: Graph,
    current_scope: ScopeId,
    scope_counter: u64,
}
```

### Graph Structure

```
Namespace Graph Structure:
==========================

Nodes:
  - Scope nodes: Represent lexical scopes
  - Variable nodes: Represent variable names
  - Value nodes: Store actual values

Edges:
  - "contains": Scope → Variable (scope contains this variable)
  - "binds_to": Variable → Value (variable's current value)
  - "parent": Scope → Scope (lexical parent)
  - "captures": Closure → Variable (captured variables)

Example:
--------
fn add(a, b) {
    return a + b
}
x = 5
result = add(x, 3)

Graph:
┌──────────────────────────────────────────────────────────────────┐
│ Scope: "global"                                                  │
│ ├── contains ──► Var: "add" ── binds_to ──► Fn{...}             │
│ ├── contains ──► Var: "x" ── binds_to ──► Num(5)                │
│ └── contains ──► Var: "result" ── binds_to ──► Num(8)           │
│                                                                  │
│ Scope: "fn:add" ── parent ──► Scope: "global"                   │
│ ├── contains ──► Var: "a" ── binds_to ──► Num(5)   [call-time]  │
│ └── contains ──► Var: "b" ── binds_to ──► Num(3)   [call-time]  │
└──────────────────────────────────────────────────────────────────┘
```

### Core Operations

```rust
impl NamespaceGraph {
    /// Create a new namespace with global scope
    pub fn new() -> Self {
        let mut graph = Graph::new(GraphType::Directed);
        let global_scope = ScopeId(0);

        // Add global scope node
        graph.add_node(
            global_scope.to_string(),
            Value::from(ScopeNode {
                id: global_scope,
                name: "global".to_string(),
                scope_type: ScopeType::Global,
            })
        );

        NamespaceGraph {
            graph,
            current_scope: global_scope,
            scope_counter: 1,
        }
    }

    /// Define a variable in current scope
    pub fn define(&mut self, name: &str, value: Value) {
        let var_id = format!("var:{}:{}", self.current_scope.0, name);
        let val_id = format!("val:{}", self.next_id());

        // Add variable node
        self.graph.add_node(var_id.clone(), Value::from(VariableNode {
            name: name.to_string(),
            scope_id: self.current_scope,
        }));

        // Add value node
        self.graph.add_node(val_id.clone(), value);

        // Scope contains variable
        self.graph.add_edge(
            &self.current_scope.to_string(),
            &var_id,
            "contains".to_string(),
            None,
            HashMap::new()
        );

        // Variable binds to value
        self.graph.add_edge(
            &var_id,
            &val_id,
            "binds_to".to_string(),
            None,
            HashMap::new()
        );
    }

    /// Look up a variable (traverse scope chain)
    pub fn get(&self, name: &str) -> Option<&Value> {
        let mut scope = self.current_scope;

        loop {
            // Find variable in this scope
            let var_id = format!("var:{}:{}", scope.0, name);
            if let Some(var_node) = self.graph.get_node(&var_id) {
                // Follow binds_to edge to get value
                if let Some(edges) = self.graph.edges_from(&var_id) {
                    for edge in edges {
                        if edge.label == "binds_to" {
                            return self.graph.get_node(&edge.to);
                        }
                    }
                }
            }

            // Traverse to parent scope
            let scope_id = scope.to_string();
            if let Some(edges) = self.graph.edges_from(&scope_id) {
                let parent = edges.iter().find(|e| e.label == "parent");
                if let Some(parent_edge) = parent {
                    scope = ScopeId::from_string(&parent_edge.to);
                } else {
                    // No parent - variable not found
                    return None;
                }
            } else {
                return None;
            }
        }
    }

    /// Assign to existing variable
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Find variable (traverse scope chain)
        let var_id = self.find_variable(name)?;

        // Remove old binds_to edge
        self.graph.remove_edges_from(&var_id, Some("binds_to"));

        // Add new value and edge
        let val_id = format!("val:{}", self.next_id());
        self.graph.add_node(val_id.clone(), value);
        self.graph.add_edge(&var_id, &val_id, "binds_to".to_string(), None, HashMap::new());

        Ok(())
    }

    /// Enter a new scope
    pub fn push_scope(&mut self, name: &str, scope_type: ScopeType) -> ScopeId {
        let new_scope = ScopeId(self.scope_counter);
        self.scope_counter += 1;

        // Add scope node
        self.graph.add_node(
            new_scope.to_string(),
            Value::from(ScopeNode {
                id: new_scope,
                name: name.to_string(),
                scope_type,
            })
        );

        // Parent edge to current scope
        self.graph.add_edge(
            &new_scope.to_string(),
            &self.current_scope.to_string(),
            "parent".to_string(),
            None,
            HashMap::new()
        );

        self.current_scope = new_scope;
        new_scope
    }

    /// Exit current scope
    pub fn pop_scope(&mut self) {
        // Find parent scope
        let current = self.current_scope.to_string();
        if let Some(edges) = self.graph.edges_from(&current) {
            if let Some(parent) = edges.iter().find(|e| e.label == "parent") {
                self.current_scope = ScopeId::from_string(&parent.to);
            }
        }

        // Note: Old scope nodes are NOT immediately removed
        // GC will clean up unreachable scopes
    }

    /// Create a closure with captured variables
    pub fn create_closure(&self, fn_value: Value, captured_names: &[String]) -> Value {
        // For each captured name, find the variable node and add capture edge
        let closure_id = format!("closure:{}", self.next_id());

        for name in captured_names {
            if let Ok(var_id) = self.find_variable(name) {
                self.graph.add_edge(
                    &closure_id,
                    &var_id,
                    "captures".to_string(),
                    None,
                    HashMap::new()
                );
            }
        }

        // Wrap function with closure reference
        Value::Closure {
            function: Box::new(fn_value),
            closure_id,
        }
    }
}
```

---

## Migration Strategy

### Step 1: Create New Module

```
src/namespace/
├── mod.rs           # NamespaceGraph
├── scope.rs         # ScopeNode, ScopeId, ScopeType
├── variable.rs      # VariableNode
└── closure.rs       # Closure handling
```

### Step 2: Implement Core Operations

1. `new()` - Create namespace with global scope
2. `define()` - Define variable in current scope
3. `get()` - Look up variable (traverse scope chain)
4. `assign()` - Assign to existing variable
5. `push_scope()` - Enter new scope
6. `pop_scope()` - Exit scope
7. `create_closure()` - Create closure with captures

### Step 3: Add Feature Flag

```rust
// src/lib.rs
#[cfg(feature = "graph_namespace")]
pub mod namespace;

// src/execution/executor.rs
impl Executor {
    pub fn new() -> Self {
        #[cfg(feature = "graph_namespace")]
        let env = NamespaceGraph::new();

        #[cfg(not(feature = "graph_namespace"))]
        let env = Environment::new();

        // ...
    }
}
```

### Step 4: Migrate Executor

Replace all Environment usage with NamespaceGraph:

```rust
// Before
self.environment.define(name, value);
let val = self.environment.get(name);

// After
self.namespace.define(name, value);
let val = self.namespace.get(name);
```

### Step 5: Test and Validate

Run all existing tests with graph namespace enabled.

### Step 6: Remove Old Code

Once validated, remove `Environment` struct and feature flag.

---

## Implementation Plan

### Day 1-2: Core Data Structures

- [ ] Create `src/namespace/mod.rs`
- [ ] Define `NamespaceGraph`, `ScopeNode`, `VariableNode`
- [ ] Implement `new()`, basic graph structure
- [ ] Unit tests for data structures

### Day 3-4: Variable Operations

- [ ] Implement `define()` - add variable to scope
- [ ] Implement `get()` - traverse scope chain
- [ ] Implement `assign()` - update existing variable
- [ ] Unit tests for variable operations

### Day 5-6: Scope Operations

- [ ] Implement `push_scope()` - enter new scope
- [ ] Implement `pop_scope()` - exit scope
- [ ] Implement parent edge traversal
- [ ] Unit tests for scope operations

### Day 7-8: Closures

- [ ] Implement `create_closure()` - capture variables
- [ ] Implement closure variable lookup
- [ ] Test closures with various capture patterns

### Day 9-10: Integration

- [ ] Add feature flag for graph namespace
- [ ] Migrate Executor to use NamespaceGraph
- [ ] Run all existing tests
- [ ] Performance benchmarking
- [ ] Documentation

---

## Success Criteria

### Functional

- [ ] All variable operations work via graph
- [ ] Scope nesting works correctly
- [ ] Closures capture variables correctly
- [ ] All existing tests pass

### Performance

- [ ] Variable lookup ≤2x HashMap overhead
- [ ] Memory overhead ≤30% increase
- [ ] No regression on existing benchmarks

### Architecture

- [ ] Variables are graph nodes
- [ ] Scopes are subgraphs
- [ ] Lookup is graph traversal
- [ ] Closures are capture edges

---

## Debugging Support

### Inspect Namespace

```graphoid
# Future: Debug API
debug.namespace()
# Returns graph visualization of current namespace

# Example output:
# Scope: "global"
# ├── x: 5
# ├── y: 10
# └── add: fn(a, b)
#     └── Scope: "fn:add" (current)
#         ├── a: 5
#         └── b: 3
```

### Visualize Scope Chain

```graphoid
debug.scope_chain()
# Returns: ["fn:add", "global"]
# Shows current scope and all parents
```

---

## Open Questions

1. **GC Strategy**: When to collect unreachable scope nodes?
2. **Caching**: Should we cache frequent lookups?
3. **Node IDs**: String IDs vs numeric IDs?
4. **Value Sharing**: Should multiple bindings share value nodes?

---

## Related Documents

- [GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md](GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md) - Why this matters
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Next phase
- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Depends on this (if compilation pursued)
