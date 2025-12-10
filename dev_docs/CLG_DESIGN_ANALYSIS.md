# Class-Like Graphs: Design Analysis and Implementation Plan

**Created**: December 10, 2025
**Status**: Design Phase (Pre-Implementation)
**Branch**: class_like_graphs

---

## Executive Summary

This document analyzes how Graphoid's "class-like graphs" compare to traditional OOP, identifies gaps and opportunities, and establishes a prioritized implementation plan. The key insight is that graphs with methods are not a parallel system to traditional objects - they are a **unification** where the object's structure, state, and behavior are all part of the same graph.

### Core Principle: Data Layer is Default

**Critical**: All operations work on the **data layer only** by default. Internal layers (methods, config, meta) are hidden unless explicitly requested. This separation must be maintained throughout all features.

---

## Part 1: Current State Assessment

### What Works Today

| OOP Concept | Graphoid Implementation | Notes |
|-------------|------------------------|-------|
| **Object** | Graph with data nodes | Data layer holds "instance variables" |
| **Instance fields** | Data nodes | `g.add_node("name", value)` |
| **Methods** | Functions in `__methods__` branch | `fn Graph.method() {}` |
| **`self` reference** | `self` keyword | Bound to graph in method context |
| **Constructor** | `new()` convention | User-defined, returns `self.clone()` |
| **Cloning/copying** | `graph.clone()` | Deep copy including methods |
| **Encapsulation** | `:read_only` rule | Methods can't modify graph |
| **Invariants** | `add_rule()`, `add_method_constraint()` | Structural and behavioral |
| **Dynamic methods** | `remove_method()`, redefinition | Runtime modification |
| **Introspection** | `nodes(:all)`, `edges(:all)` | See internal structure |

### The Five-Layer Graph Architecture

Graphoid graphs have a layered architecture (discussed previously, formalized here):

```
┌─────────────────────────────────────────────┐
│ Layer 5: Meta Layer (__meta__)              │  Future
│   - Graph-about-graph information           │
│   - Schema, versioning, provenance          │
├─────────────────────────────────────────────┤
│ Layer 4: Config Layer (__config__)          │  Future (currently via rules)
│   - Graph-wide settings                     │
│   - Behavioral configuration                │
├─────────────────────────────────────────────┤
│ Layer 3: Method Layer (__methods__)         │  ✅ Implemented
│   - Attached functions                      │
│   - Method constraints                      │
├─────────────────────────────────────────────┤
│ Layer 2: Rule Layer (rules field)           │  ✅ Implemented
│   - Structural constraints                  │
│   - Validation rules                        │
├─────────────────────────────────────────────┤
│ Layer 1: Data Layer (user nodes/edges)      │  ✅ Implemented
│   - User data (the "object state")          │
│   - All operations default to this layer    │
└─────────────────────────────────────────────┘
```

**Design Rule**: All user-facing operations default to Layer 1 (data). Accessing other layers requires explicit opt-in (e.g., `:all`, `:methods`).

---

## Part 2: Gap Analysis

### 2.1 Ergonomic Gaps (Developer Experience)

#### Gap 1: Property Access is Verbose (CRITICAL)

**Current** (awkward):
```graphoid
value = self.get_node("count")
self.add_node("count", value + 1)
```

**Desired** (natural):
```graphoid
value = self.count
self.count = value + 1
# Or
self["count"] = self["count"] + 1
```

**Impact**: This is the #1 barrier to CLGs feeling like "real objects."

#### Gap 2: No Computed Properties (Getters/Setters)

**OOP has**:
```python
@property
def area(self):
    return 3.14159 * self.radius ** 2
```

**Graphoid lacks**: No way to define a "property" that computes on access.

#### Gap 3: Static/Class Methods

**OOP has**:
```python
class User:
    @staticmethod
    def hash_password(pw): ...  # No instance needed
```

**Graphoid lacks**: All methods require `self`. Utility functions have no "home" on a graph.

### 2.2 OOP Feature Gaps

#### Gap 4: Inheritance

**Planned** (Phase 13): `graph from Parent {}`

**Note**: This is clone-based, not reference-based. Child gets a copy of parent's methods at creation time. This is intentional and graph-appropriate.

#### Gap 5: Super Calls

**Planned** (Phase 14): `super.method()`

Requires storing parent reference for method lookup chain.

#### Gap 6: Private Methods

**Planned** (Phase 15): `_method()` convention

Methods starting with `_` only callable from within the same graph's methods.

#### Gap 7: Type Checking

**Planned** (Phase 17): `is_a()`, `type_of()`

Runtime type interrogation for inheritance chains.

### 2.3 Graph-Unique Opportunities (OOP Can't Do This)

#### Opportunity 1: Structure-Based Polymorphism

Methods can dispatch based on graph structure, not type hierarchy:

```graphoid
fn Shape.area() when self.has_node("radius") {
    r = self.get_node("radius")
    return 3.14159 * r * r
}

fn Shape.area() when self.has_node("width") {
    return self.get_node("width") * self.get_node("height")
}
```

The shape of the data determines behavior, not a class name.

#### Opportunity 2: Relationships as Edges

OOP uses foreign keys or reference fields. Graphs have native edges:

```graphoid
fn User.add_friend(other_user) {
    self.add_edge(self.id, other_user.id, "friend")
}

fn User.friends() {
    return self.neighbors_by_edge_type("friend")
}
```

Relationships ARE the graph, not metadata about it.

#### Opportunity 3: Rules as Invariants

No OOP language has structural invariants built-in:

```graphoid
BinaryTree = graph{}
BinaryTree.add_rule(:no_cycles)
BinaryTree.add_rule(:max_degree, 2)
# Structure physically cannot violate these rules
```

#### Opportunity 4: Queryable Internal Structure

The object's state is a graph you can run algorithms on:

```graphoid
# Inside a method:
self.shortest_path("start", "end")
self.has_path("input", "output")
self.nodes().filter(n => n.value > 100)
```

#### Opportunity 5: Visualization IS Debugging

```graphoid
my_object.visualize()  # See state as graph
my_object.to_dot()     # Export for Graphviz
```

---

## Part 3: Design Decisions

### 3.1 Property Access Syntax

**Decision**: Implement both dot notation and index notation.

```graphoid
# Dot notation for known properties
self.count          # Equivalent to self.get_node("count")
self.count = 5      # Equivalent to self.add_node("count", 5)

# Index notation for dynamic access
key = "count"
self[key]           # Equivalent to self.get_node(key)
self[key] = 5       # Equivalent to self.add_node(key, 5)
```

**Rules**:
1. Only works for **data layer** nodes (not `__methods__`, etc.)
2. Dot notation only valid for valid identifiers
3. Returns `none` if node doesn't exist (not error)
4. Assignment creates node if it doesn't exist

**Conflict Resolution**:
- If graph has a *method* called `count` AND a *data node* called `count`:
  - `self.count()` calls the method
  - `self.count` (no parens) accesses the data node
  - This matches Ruby's convention

### 3.2 Inheritance Model

**Decision**: Clone-based inheritance with parent reference.

```graphoid
Dog = graph from Animal {}
```

Semantics:
1. Clone `Animal` (deep copy of data nodes, methods, rules)
2. Store reference to `Animal` as parent (for `super` calls)
3. Apply any additional initializers from `{}`

**Why clone-based, not prototype-based**:
- Graphs are independent entities
- Changes to parent after child creation don't affect child
- Simpler mental model
- More predictable behavior

### 3.3 Layer Access Convention

**Decision**: Use symbol arguments for layer control.

```graphoid
# Data layer (default)
g.nodes()           # Data nodes only
g.edges()           # Data edges only

# All layers
g.nodes(:all)       # All nodes including internal
g.edges(:all)       # All edges including internal

# Specific layers (future)
g.nodes(:methods)   # Only method nodes
g.nodes(:data)      # Explicit data (same as default)
```

**Extensible**: When we add Layer 4/5, same pattern works:
```graphoid
g.nodes(:config)    # Config layer nodes
g.nodes(:meta)      # Meta layer nodes
```

### 3.4 Computed Properties (Getters)

**Decision**: No-argument methods without parentheses act as getters.

```graphoid
fn Circle.area {    # No parens in definition = getter
    r = self.radius
    return 3.14159 * r * r
}

c.area              # No parens in call = getter access
c.area()            # With parens also works (explicit method call)
```

**Setters**: Use assignment syntax with special method name:

```graphoid
fn Circle.radius=(value) {    # Setter naming convention
    if value < 0 {
        error("Radius cannot be negative")
    }
    self["_radius"] = value   # Store in internal node
}

c.radius = 5        # Calls the setter
```

### 3.5 Static Methods

**Decision**: Methods that don't reference `self` are implicitly static.

```graphoid
fn Math.sqrt(x) {
    # No self reference - this is a static method
    return x ** 0.5
}

Math.sqrt(16)       # Works without instance
```

**Alternative considered**: `static` keyword. Rejected because:
- Adds syntax complexity
- Graphoid prefers convention over configuration
- The method body makes intent clear

### 3.6 Private Methods

**Decision**: Underscore prefix convention with runtime enforcement.

```graphoid
fn Secret._internal_helper() {
    # Private - only callable from within Secret's methods
}

fn Secret.public_method() {
    self._internal_helper()  # OK
}

Secret._internal_helper()    # ERROR: private method
```

**Scope**: Private means "same graph instance", not "same class." A cloned instance can call its own private methods but not the original's.

### 3.7 Interfaces/Protocols

**Decision**: Defer formal interfaces. Use duck typing with `responds_to()`.

```graphoid
fn requires_drawable(obj) {
    if not obj.responds_to("draw") {
        error("Object must have draw() method")
    }
    obj.draw()
}
```

**Future**: Could add formal interfaces as graph schemas:
```graphoid
Drawable = interface {
    required_methods: ["draw", "get_bounds"]
}
obj.implements(Drawable)  # true/false
```

---

## Part 4: Implementation Plan

### All Features To Implement

All of the following features are part of the implementation plan. None are deferred.

**Core OOP Features:**
1. Property access syntax (dot and index notation)
2. Inheritance with `from`
3. Private methods (`_method`)
4. Super calls
5. Computed properties (getters)
6. Setters
7. Type checking (`is_a`, `type_of`)
8. Static method detection

**Graph-Unique Features:**
9. Structure-based method dispatch (`when` guards)
10. Mixins (`include`)
11. `responds_to()` introspection

**Note on Five-Layer System**: Layers 4-5 (Config, Meta) are architectural extensions that may be implemented as part of this work or as immediate follow-on, depending on need. The infrastructure for layer access (`:all`, `:methods`, etc.) is already in place.

### Phase 13: Property Access Syntax

**Priority**: CRITICAL
**Effort**: 4-6 hours

#### 13.1 Dot Notation for Data Nodes

**Syntax**:
```graphoid
self.name           # Get data node "name"
self.name = "Alice" # Set data node "name"
```

**Parser Changes** (`src/parser/mod.rs`):
- Currently `expr.identifier` parses as method call prep
- Need to distinguish: `expr.identifier()` = method call, `expr.identifier` = property access
- Add `Expr::PropertyAccess { object: Box<Expr>, property: String }`

**Executor Changes** (`src/execution/executor.rs`):
- `PropertyAccess` evaluation:
  - If object is a graph, return `graph.get_node(property)` or `none`
  - If object is a hash/map, return `hash.get(property)` or `none`
- Assignment to `PropertyAccess`:
  - If object is a graph, call `graph.add_node(property, value)`
  - If object is a hash/map, call `hash.set(property, value)`

**Conflict with methods**:
- `self.foo` where `foo` is both a method and a data node:
  - Return the data node value
  - User must use `self.foo()` to call method
- `self.foo` where `foo` is only a method:
  - Return the method as a value (first-class function)
  - This enables `callback = obj.method` patterns

**Tests**:
- `test_property_access_get_existing_node`
- `test_property_access_get_missing_returns_none`
- `test_property_access_set_creates_node`
- `test_property_access_set_updates_node`
- `test_property_access_with_method_conflict`
- `test_property_access_returns_method_if_no_node`
- `test_property_access_only_data_layer`

#### 13.2 Index Notation for Dynamic Access

**Syntax**:
```graphoid
self["name"]            # Get data node "name"
self["name"] = "Alice"  # Set data node "name"
key = "name"
self[key]               # Dynamic key
```

**Parser Changes**:
- Already have `Expr::Index` for list/hash access
- Extend to work with graphs

**Executor Changes**:
- When indexing a graph with a string key:
  - Get: `graph.get_node(key)`
  - Set: `graph.add_node(key, value)`

**Layer access via index** (future consideration):
```graphoid
self["__methods__"]     # Should this work?
```
**Decision**: No. Index notation only accesses data layer. Use `nodes(:all)` for internal access.

**Tests**:
- `test_index_access_get_node`
- `test_index_access_set_node`
- `test_index_access_dynamic_key`
- `test_index_access_rejects_internal_nodes`

#### 13.3 Example File

Create `samples/03-advanced/property_access.gr`:
```graphoid
# Property Access Demo

Person = graph{}
Person.add_node("name", "")
Person.add_node("age", 0)

fn Person.new(name, age) {
    p = self.clone()
    p.name = name       # Property syntax!
    p.age = age
    return p
}

fn Person.birthday() {
    self.age = self.age + 1   # Clean mutation
}

fn Person.describe() {
    return self.name + " is " + self.age.to_string() + " years old"
}

# Usage
alice = Person.new("Alice", 30)
print(alice.describe())      # "Alice is 30 years old"
alice.birthday()
print(alice.describe())      # "Alice is 31 years old"

# Dynamic access
field = "name"
print(alice[field])          # "Alice"
```

---

### Phase 14: Inheritance with `from`

**Priority**: CRITICAL
**Effort**: 4-6 hours
**Depends on**: Nothing (can parallelize with Phase 13)

#### 14.1 Syntax

```graphoid
Animal = graph{}
Animal.add_node("name", "unknown")

fn Animal.speak() {
    return "..."
}

Dog = graph from Animal {}
Dog.name = "dog"  # Override data node

fn Dog.speak() {
    return "Woof!"
}
```

#### 14.2 Parser Changes

Extend `GraphLiteral` AST node:
```rust
Expr::GraphLiteral {
    entries: Vec<(Expr, Expr)>,
    graph_type: Option<String>,
    parent: Option<Box<Expr>>,  // NEW: for "graph from X"
}
```

Parse `graph from <expr> {}`:
- After `graph` keyword, check for `from`
- Parse parent expression
- Parse `{}` block as normal

#### 14.3 Executor Changes

When evaluating `graph from Parent {}`:
1. Evaluate parent expression to get parent graph
2. Clone the parent graph completely (data, methods, rules)
3. Store parent reference in new graph (for `super` calls)
4. Apply any `{}` initializers
5. Return new graph

**Graph struct addition**:
```rust
pub struct Graph {
    // ... existing fields ...
    parent: Option<Box<Graph>>,  // For super call chain
}
```

#### 14.4 Tests

- `test_from_inherits_data_nodes`
- `test_from_inherits_methods`
- `test_from_inherits_rules`
- `test_from_method_override`
- `test_from_chained` (A from B from C)
- `test_from_parent_unchanged`
- `test_from_with_initializers`

---

### Phase 15: Private Methods

**Priority**: CRITICAL
**Effort**: 3-4 hours

#### 15.1 Syntax

```graphoid
fn Secret._helper() {
    return "internal"
}

fn Secret.public() {
    return self._helper()  # OK
}

Secret._helper()  # ERROR
```

#### 15.2 Implementation

**Executor changes**:
1. Add call context tracking: `current_method_graph_id: Option<u64>`
2. Before calling a graph method:
   - Check if method name starts with `_`
   - If private and not called from same graph's method → error
3. Push graph ID onto context stack before method call
4. Pop after method returns

**Graph identity**: Need unique ID per graph for comparison.

#### 15.3 Tests

- `test_private_callable_internally`
- `test_private_blocked_externally`
- `test_private_from_other_graph_blocked`
- `test_private_inherited_callable` (subclass can use inherited private)
- `test_underscore_data_node_unaffected`

---

### Phase 16: Super Calls

**Priority**: IMPORTANT
**Effort**: 3-4 hours
**Depends on**: Phase 14 (inheritance)

#### 16.1 Syntax

```graphoid
fn Dog.speak() {
    base = super.speak()
    return base + " (but also Woof!)"
}
```

#### 16.2 Implementation

**Parser**: Add `super` keyword, parse `super.method(args)` as special expr.

**Executor**:
1. Track current graph and its parent chain during method calls
2. `super.method()` looks up method in parent, not current graph
3. Walk up chain if not found in immediate parent

#### 16.3 Tests

- `test_super_calls_parent`
- `test_super_chain` (grandparent)
- `test_super_with_args`
- `test_super_outside_method_fails`
- `test_super_method_not_found`

---

### Phase 17: Computed Properties (Getters)

**Priority**: IMPORTANT
**Effort**: 2-3 hours

#### 17.1 Syntax

```graphoid
fn Circle.area {      # No parens = getter
    return 3.14159 * self.radius * self.radius
}

c.area                # Access like property
```

#### 17.2 Implementation

**Parser**: Method definition without `()` creates getter.

**Executor**: When accessing `obj.property`:
1. Check for data node → return if exists
2. Check for no-arg method → call if exists
3. Return `none`

#### 17.3 Tests

- `test_getter_definition`
- `test_getter_access`
- `test_getter_vs_data_node_priority`

---

### Phase 18: Type Checking

**Priority**: IMPORTANT
**Effort**: 3-4 hours
**Depends on**: Phase 14 (inheritance)

#### 18.1 Syntax

```graphoid
fido.is_a(Dog)        # true
fido.is_a(Animal)     # true (parent)
fido.type_of()        # "Dog" or reference
```

#### 18.2 Implementation

**Graph additions**:
- `type_name: Option<String>` - set when assigned to variable
- Walk parent chain for `is_a()` comparison

**Challenge**: Graph identity for comparison. Use combination of:
- Type name
- Structure hash (optional, for unnamed graphs)

#### 18.3 Tests

- `test_is_a_self_type`
- `test_is_a_parent_type`
- `test_is_a_grandparent`
- `test_is_a_unrelated_false`
- `test_type_of`

---

## Part 5: Additional Considerations

### 5.1 Five-Layer System (Layers 4-5)

Currently implemented:
- Layer 1: Data (user nodes) ✅
- Layer 2: Rules (rules field) ✅
- Layer 3: Methods (`__methods__` branch) ✅

To be added as needed:
- Layer 4: Config (`__config__` branch)
- Layer 5: Meta (`__meta__` branch)

**Access pattern** (infrastructure already exists):
```graphoid
g.nodes()           # Layer 1 only (default)
g.nodes(:all)       # All layers
g.nodes(:data)      # Explicit Layer 1
g.nodes(:methods)   # Layer 3 only
g.nodes(:config)    # Layer 4 only (when implemented)
g.nodes(:meta)      # Layer 5 only (when implemented)
g.nodes([:data, :methods])  # Multiple specific layers
```

Layers 4-5 may be added during this implementation if use cases arise, or as immediate follow-on.

### 5.2 Observer Pattern (Potential Future Enhancement)

Graph-native event system:

```graphoid
g.on_change("count", fn(old, new) {
    print("Count changed from " + old + " to " + new)
})
```

Could be implemented as edges in a `__observers__` branch. Not part of initial implementation but architecture supports it.

### 5.3 Relationships as Edges (Potential Future Enhancement)

First-class support for inter-object relationships:

```graphoid
fn User.befriend(other) {
    # Create edge between graphs
    graph_link(self, other, "friend")
}

fn User.friends() {
    return linked_graphs(self, "friend")
}
```

Graphs already support edges; this would be a convenience layer on top.

---

## Part 6: Success Criteria

### Complete CLG System (Phases 13-23)

All features to be implemented:

**Property Access (Phase 13):**
- [ ] `self.property` syntax works for data nodes
- [ ] `self["key"]` syntax works for dynamic access
- [ ] Data layer only (internal nodes not accessible this way)

**Inheritance (Phases 14, 16):**
- [ ] `graph from Parent {}` creates inherited graph
- [ ] Methods can call parent via `super.method()`
- [ ] Chained inheritance works (A from B from C)

**Encapsulation (Phase 15):**
- [ ] `_private()` methods enforced at runtime
- [ ] Private methods callable from within same graph

**Computed Properties (Phases 17, 19):**
- [ ] Getters defined with `fn X.prop {}` (no parens)
- [ ] Setters defined with `fn X.prop=(val) {}`
- [ ] Accessed naturally: `x.prop`, `x.prop = val`

**Type System (Phase 18):**
- [ ] `is_a(Type)` walks inheritance chain
- [ ] `type_of()` returns type name

**Static Methods (Phase 20):**
- [ ] Methods without `self` reference callable without instance

**Structure-Based Dispatch (Phase 21):**
- [ ] `when` guards on method definitions
- [ ] Multiple implementations of same method name
- [ ] First matching guard wins

**Composition (Phase 22):**
- [ ] `include(Mixin)` copies methods from another graph
- [ ] No inheritance chain established (no super)

**Introspection (Phase 23):**
- [ ] `responds_to("method_name")` checks method availability

**Quality:**
- [ ] All existing tests still pass
- [ ] New tests for each feature
- [ ] Example files for each feature
- [ ] Documentation updated

---

## Part 7: Open Questions

1. **Should property access work on non-graph objects?**
   - Currently: Hashes use `hash["key"]`
   - Could: `hash.key` work too?
   - Decision: Defer, focus on graphs first

2. **How do setters interact with rules?**
   - `self.count = -5` where `count` has `:positive` rule?
   - Decision: Rules still apply, assignment fails if rule violated

3. **Can private methods be inherited?**
   - Decision: Yes, child can call inherited `_private()` methods

4. **Should `clone()` preserve parent reference?**
   - Decision: Yes, clone has same parent as original

5. **What happens with `graph from graph from X {}`?**
   - Decision: Allow chained inheritance in single expression

---

## Appendix: Comparison Table

| Feature | Java/C# | Python | Ruby | JavaScript | Graphoid CLG |
|---------|---------|--------|------|------------|--------------|
| Instance fields | `this.x` | `self.x` | `@x` | `this.x` | `self.x` (new) |
| Private | `private` | `_x` convention | `private` | `#x` | `_x` enforced |
| Inheritance | `extends` | `class X(Y)` | `< Parent` | `extends` | `from Parent` |
| Super | `super.m()` | `super().m()` | `super` | `super.m()` | `super.m()` |
| Type check | `instanceof` | `isinstance()` | `is_a?` | `instanceof` | `is_a()` |
| Mixins | interfaces | multiple inherit | `include` | mixins | `include()` |
| Computed props | getter methods | `@property` | methods | get/set | `fn X.prop {}` |
| Invariants | assertions | N/A | N/A | N/A | `add_rule()` ✨ |
| Queryable state | reflection | reflection | reflection | reflection | Graph algorithms ✨ |
| Visualization | debugger | debugger | debugger | debugger | `visualize()` ✨ |

✨ = Unique to Graphoid

---

## Document History

- 2025-12-10: Initial creation, comprehensive design analysis
