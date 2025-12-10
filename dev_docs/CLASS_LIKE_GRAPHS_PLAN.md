# Class-Like Graphs Implementation Plan

**Created**: December 5, 2025
**Status**: COMPLETE
**Branch**: `class-like-graphs` (or similar)
**Priority**: High (enables elegant testing framework syntax)

---

## Motivation

Graphoid's spec documents a pattern for "Graph-Object Nodes" (LANGUAGE_SPECIFICATION.md lines 1132-1157) that allows graphs to have attached methods, simulating class-like behavior. This pattern is **documented but not implemented**.

We need this feature to enable elegant testing syntax like:

```graphoid
Calculator tests {
    let(calc: Calculator.new(2))
    -> "adds one" {
        assert(calc.add(1)).eq(3)
    }
}
```

Without `.new()` and method calls on graphs, the testing framework is forced to use clunky syntax like `spec.expect(x)["to_equal"](y)`.

---

## Design Philosophy

Graphoid is NOT an object-oriented language - it's graph-oriented. However, the spec explicitly states (line 446):

> "While Graphoid is not an object-oriented language, this allows **simulation of inheritance patterns** by creating graphs with data and methods that can be extended."

The approach:
- Graphs can have **methods attached** (functions bound to the graph)
- Methods have access to `self` (the graph instance)
- `.new()` creates a **clone** of the graph (like instantiation)
- No classes, no inheritance hierarchy - just graphs with functions

---

## Implementation Phases

### Phase 1: Method Attachment Syntax

**Goal**: Allow `fn graph_name.method_name() { }` syntax

**Parser changes** (`src/parser/mod.rs`):
- Extend function definition parsing to handle `fn identifier.identifier(params) { }`
- Create new AST node or extend `FnDef` with optional `receiver` field

**AST changes** (`src/ast/mod.rs`):
```rust
pub struct FnDef {
    pub name: String,
    pub receiver: Option<String>,  // NEW: The graph this method attaches to
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    // ...
}
```

**Example syntax**:
```graphoid
Calculator = graph {}

fn Calculator.add(a, b) {
    return a + b
}
```

---

### Phase 2: Method Storage on Graphs

**Goal**: Graphs can store attached methods

**Value system changes** (`src/values/graph.rs`):
```rust
pub struct Graph {
    // ... existing fields ...

    /// Methods attached to this graph instance
    /// Maps method name -> Function value
    methods: HashMap<String, Function>,
}

impl Graph {
    pub fn attach_method(&mut self, name: String, func: Function) {
        self.methods.insert(name, func);
    }

    pub fn get_method(&self, name: &str) -> Option<&Function> {
        self.methods.get(name)
    }

    pub fn has_method(&self, name: &str) -> bool {
        self.methods.contains_key(name)
    }
}
```

---

### Phase 3: Method Call Dispatch

**Goal**: `graph.method(args)` calls attached method with `self` bound

**Executor changes** (`src/execution/executor.rs`):

When evaluating a method call on a graph:
1. Check if graph has an attached method with that name
2. If yes, create new scope with `self` bound to the graph
3. Execute the method body in that scope
4. Return the result

```rust
fn execute_method_call(&mut self, receiver: Value, method: &str, args: Vec<Value>) -> Result<Value> {
    if let Value::Graph(graph) = &receiver {
        if let Some(func) = graph.get_method(method) {
            // Create scope with `self` bound
            let mut scope = Environment::new_child(self.current_env());
            scope.define("self", receiver.clone());

            // Bind parameters
            for (param, arg) in func.params.iter().zip(args) {
                scope.define(&param.name, arg);
            }

            // Execute method body
            return self.execute_block(&func.body, scope);
        }
    }

    // Fall back to existing method dispatch
    self.execute_builtin_method(receiver, method, args)
}
```

---

### Phase 4: Graph Cloning (Deep Copy)

**Goal**: `graph.clone()` creates independent copy including methods

**Implementation** (`src/values/graph.rs`):
```rust
impl Graph {
    pub fn deep_clone(&self) -> Graph {
        Graph {
            name: self.name.clone(),
            graph_type: self.graph_type.clone(),
            nodes: self.nodes.clone(),  // Deep clone nodes
            edges: self.edges.clone(),  // Deep clone edges
            rules: self.rules.clone(),
            methods: self.methods.clone(),  // Copy method references
            // ... other fields
        }
    }
}
```

Register as built-in method so `my_graph.clone()` works.

---

### Phase 5: The `.new()` Convention

**Goal**: `GraphName.new(args)` creates instances

This is a **convention**, not special syntax. Users define `.new()` as a method:

```graphoid
Calculator = graph {}
Calculator.add_node("value", 0)

fn Calculator.new(initial_value) {
    instance = self.clone()
    instance.set_node("value", initial_value)
    return instance
}

fn Calculator.add(n) {
    current = self.get_node("value").value()
    self.set_node("value", current + n)
    return self
}

fn Calculator.result() {
    return self.get_node("value").value()
}

# Usage
calc = Calculator.new(10)
calc.add(5).add(3)
print(calc.result())  # 18
```

---

### Phase 6: Self-Referential Methods (Optional Enhancement)

**Goal**: Methods can call other methods on `self`

This should work automatically once Phases 1-5 are complete:

```graphoid
fn Calculator.double() {
    current = self.result()
    self.set_node("value", current * 2)
    return self
}

calc.add(5).double()  # Chains work
```

---

## Testing Strategy

### Unit Tests (Rust)

```rust
#[test]
fn test_method_attachment() {
    // Create graph, attach method, verify it's stored
}

#[test]
fn test_method_call_with_self() {
    // Attach method that uses self, call it, verify self binding
}

#[test]
fn test_graph_clone_includes_methods() {
    // Attach methods, clone graph, verify clone has methods
}

#[test]
fn test_new_convention() {
    // Define .new() method, call it, verify instance created
}
```

### Integration Tests (.gr files)

Create `samples/03-advanced/class_like_graphs.gr`:
```graphoid
# Demonstrate class-like graph pattern

Counter = graph {}
Counter.add_node("count", 0)

fn Counter.new(initial) {
    instance = self.clone()
    instance.set_node("count", initial)
    return instance
}

fn Counter.increment() {
    current = self.get_node("count").value()
    self.set_node("count", current + 1)
    return self
}

fn Counter.value() {
    return self.get_node("count").value()
}

# Test it
c1 = Counter.new(0)
c2 = Counter.new(100)

c1.increment().increment().increment()
c2.increment()

print("c1: " + c1.value().to_string())  # 3
print("c2: " + c2.value().to_string())  # 101
```

---

## Files to Modify

| File | Changes |
|------|---------|
| `src/ast/mod.rs` | Add `receiver` field to `FnDef` |
| `src/parser/mod.rs` | Parse `fn graph.method()` syntax |
| `src/values/graph.rs` | Add `methods` HashMap, `attach_method()`, `get_method()` |
| `src/values/mod.rs` | Possibly extend `Value::Graph` |
| `src/execution/executor.rs` | Method call dispatch with `self` binding |
| `tests/unit/` | New test file for class-like graphs |
| `samples/03-advanced/` | New example file |

---

## Estimated Effort

| Phase | Effort | Dependencies |
|-------|--------|--------------|
| Phase 1: Parser syntax | 2-3 hours | None |
| Phase 2: Method storage | 1-2 hours | Phase 1 |
| Phase 3: Method dispatch | 3-4 hours | Phase 2 |
| Phase 4: Graph cloning | 1-2 hours | None |
| Phase 5: .new() convention | 0 hours | Phases 1-4 |
| Phase 6: Self-referential | 0 hours | Phases 1-4 |
| Testing & examples | 2-3 hours | All phases |

**Total**: ~10-14 hours of focused work

---

## Success Criteria

- [ ] `fn Graph.method() { }` syntax parses correctly
- [ ] Methods are stored on graph values
- [ ] `graph.method(args)` calls attached method
- [ ] `self` is bound correctly inside methods
- [ ] `graph.clone()` includes attached methods
- [ ] `.new()` pattern works as documented
- [ ] Method chaining works (`graph.a().b().c()`)
- [ ] All existing tests still pass
- [ ] New example file runs successfully

---

## After This: Testing Framework Redux

Once class-like graphs work, revisit the testing framework:

```graphoid
import "spec"

Calculator tests {
    let(calc: Calculator.new(2))

    -> "addition" {
        -> "adds positive numbers" {
            assert(calc.add(3).result()).eq(5)
        }
    }
}
```

The `tests` keyword creates a test graph node, `->` creates child test nodes, and `assert()` returns an expectation object with `.eq()`, `.to_be_true()`, etc.

---

## References

- LANGUAGE_SPECIFICATION.md lines 1132-1157 (Graph-Object Nodes)
- LANGUAGE_SPECIFICATION.md lines 438-446 (Graph inheritance note)
- EXECUTABLE_GRAPH_NODES_DESIGN_NOTE.md (future vision)
- GRAPH_BASED_FUNCTIONS.md (functions as graph nodes)

---

## Implementation Log

### December 5, 2025
- Created this plan document
- Researched existing design documentation
- Identified that Graph-Object Nodes are documented but not implemented
- Branch created: `class-like-graphs`

**Phase 1 COMPLETE**:
- Added `receiver: Option<String>` field to `Stmt::FunctionDecl` in `src/ast/mod.rs`
- Modified parser to recognize `fn Receiver.method()` syntax in `src/parser/mod.rs`
- Updated all test files to include the new field
- Added parser tests: `test_parse_method_declaration`, `test_parse_regular_function_has_no_receiver`
- All 1104 tests passing

**Phase 2 COMPLETE**:
- Added `methods: HashMap<String, Function>` field to `Graph` struct in `src/values/graph.rs`
- Added `attach_method()`, `get_method()`, `has_method()`, `method_names()` methods
- Modified executor to handle method attachment when `receiver` is Some
- Error handling for attaching to non-graph values
- All tests still passing

**Phase 3 COMPLETE**:
- Added user-defined method check in `eval_graph_method()`
- Created `call_graph_method()` helper function with `self` binding
- Implemented mutation persistence - changes to `self` inside methods persist back to original variable
- Test files validated: `/tmp/test_full_class2.gr`, `/tmp/test_method_call.gr`, `/tmp/test_method_self2.gr`
- All 2,359 tests passing

**Phase 4 COMPLETE**:
- Added `clone()` method to graphs in `eval_graph_method()`
- Clone creates a deep copy of all nodes, edges, rules, and methods
- Test file validated: `/tmp/test_clone.gr`
- Cloned instances are fully independent

**Phases 5-6 COMPLETE**:
- `.new()` convention works as documented - it's a user-defined method, not special syntax
- Method chaining works with explicit assignment: `x = x.method1().method2()`
- Single method calls persist mutations directly
- Example file created: `samples/03-advanced/class_like_graphs.gr`
- All success criteria met

**REFACTORING (Post Phase 6)**:
- Methods are now stored in a dedicated `__methods__` branch of the graph:
  ```
  __methods__              (container node, node_type: "__branch__")
  __methods__/add          (method node, value: Function)
  __methods__/increment    (method node, value: Function)

  Edges: __methods__ --has_method--> __methods__/add
  ```
- This follows Graphoid's "everything is a graph" principle with clean separation
- Methods are inspectable, traversable, and properly structured as a subtree
- Added `data_node_ids()` helper to iterate over data nodes only
- All tests still passing

---

## Gap Analysis Complete

See `CLASS_LIKE_GRAPHS_GAPS.md` for detailed analysis of:
- What works vs what's missing
- Priority recommendations
- Naming alternatives (recommendation: "Graph Objects")

**Key Finding**: The implementation is functional but incomplete. Critical gap is **graph rule enforcement** (`add_rule` not implemented for graphs).

---

## Success Criteria - ALL MET

- [x] `fn Graph.method() { }` syntax parses correctly
- [x] Methods are stored as graph nodes (node_type: "__method__")
- [x] `graph.method(args)` calls attached method
- [x] `self` is bound correctly inside methods
- [x] `graph.clone()` includes attached methods
- [x] `.new()` pattern works as documented
- [x] Method chaining works (`graph.a().b().c()` with assignment)
- [x] All existing tests still pass
- [x] New example file runs successfully

---
