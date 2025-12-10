# Class-Like Graphs: Implementation Roadmap

**Created**: December 10, 2025
**Status**: Ready for Implementation
**Branch**: class_like_graphs
**Reference**: See `CLG_DESIGN_ANALYSIS.md` for full design rationale

---

## Overview

This roadmap prioritizes **property access syntax** as the first implementation, based on the design analysis. Property access is the single highest-impact improvement for making CLGs feel like "real objects."

---

## Implementation Order

All features below are part of the implementation plan. No items are deferred.

| Phase | Feature | Effort | Depends On |
|-------|---------|--------|------------|
| **13** | Property Access (dot & index) | 4-6 hrs | - |
| **14** | Inheritance (`from`) | 4-6 hrs | - |
| **15** | Private Methods (`_method`) | 3-4 hrs | - |
| **16** | Super Calls | 3-4 hrs | Phase 14 |
| **17** | Computed Properties (getters) | 2-3 hrs | Phase 13 |
| **18** | Type Checking (`is_a`, `type_of`) | 3-4 hrs | Phase 14 |
| **19** | Setters | 2-3 hrs | Phase 17 |
| **20** | Static Methods | 2-3 hrs | - |
| **21** | Structure-based Dispatch | 4-6 hrs | - |
| **22** | Mixins (`include`) | 3-4 hrs | Phase 14 |
| **23** | `responds_to()` method | 1-2 hrs | - |

**Phases 13-15, 20, 23 can be parallelized** (no dependencies between them).

**Total estimated effort**: 35-45 hours

---

## Phase 13: Property Access Syntax

**Goal**: Enable `self.property` and `self["key"]` for data node access.

### Before/After

```graphoid
# BEFORE (current)
value = self.get_node("count")
self.add_node("count", value + 1)

# AFTER (Phase 13)
value = self.count
self.count = value + 1
# Or
self["count"] = self["count"] + 1
```

### 13.1 Dot Notation

**Syntax**: `object.property`

**Semantics**:
- Read: `self.name` → `self.get_node("name")` or `none`
- Write: `self.name = x` → `self.add_node("name", x)`

**Method vs Property conflict**:
- `self.foo()` → calls method
- `self.foo` → accesses data node (even if method exists)
- If only method exists, returns method as value (for callbacks)

### 13.2 Index Notation

**Syntax**: `object["key"]` or `object[variable]`

**Semantics**: Same as dot notation but allows dynamic keys.

**Restriction**: Only accesses data layer. `self["__methods__"]` returns `none`.

### 13.3 Files to Modify

| File | Changes |
|------|---------|
| `src/ast/mod.rs` | Add `PropertyAccess` expr variant |
| `src/parser/mod.rs` | Parse `expr.identifier` as property access |
| `src/execution/executor.rs` | Evaluate property access and assignment |
| `tests/property_access_test.rs` | New test file |
| `samples/03-advanced/property_access.gr` | Example file |

### 13.4 Test Cases

```rust
// Read tests
test_property_access_get_existing_node
test_property_access_get_missing_returns_none
test_property_access_only_data_layer
test_property_access_with_method_same_name
test_property_access_returns_method_if_no_node

// Write tests
test_property_access_set_creates_node
test_property_access_set_updates_node
test_property_access_set_respects_rules

// Index notation tests
test_index_access_get_node
test_index_access_set_node
test_index_access_dynamic_key
test_index_access_rejects_internal_prefix

// Integration tests
test_property_access_in_method
test_property_access_chained
test_property_access_with_mutation_operator
```

### 13.5 Example File

`samples/03-advanced/property_access.gr`:

```graphoid
# Property Access Syntax Demo
# Phase 13 of Class-Like Graphs

print("=== Property Access Demo ===")
print("")

# -----------------------------------------------------------------------------
# Basic Property Access
# -----------------------------------------------------------------------------
print("--- Basic Property Access ---")

person = graph{}
person.add_node("name", "Alice")
person.add_node("age", 30)

# Dot notation read
print("Name:", person.name)
print("Age:", person.age)

# Dot notation write
person.age = 31
print("After birthday:", person.age)

# Index notation (dynamic)
field = "name"
print("Dynamic access:", person[field])

print("")

# -----------------------------------------------------------------------------
# Property Access in Methods
# -----------------------------------------------------------------------------
print("--- Property Access in Methods ---")

Counter = graph{}
Counter.add_node("count", 0)
Counter.add_node("step", 1)

fn Counter.increment() {
    self.count = self.count + self.step
}

fn Counter.set_step(n) {
    self.step = n
}

fn Counter.value() {
    return self.count
}

c = Counter.clone()
print("Initial:", c.value())
c.increment()
c.increment()
print("After 2 increments:", c.value())
c.set_step(5)
c.increment()
print("After step=5 increment:", c.value())

print("")

# -----------------------------------------------------------------------------
# Constructor Pattern with Properties
# -----------------------------------------------------------------------------
print("--- Constructor with Properties ---")

Point = graph{}
Point.add_node("x", 0)
Point.add_node("y", 0)

fn Point.new(x, y) {
    p = self.clone()
    p.x = x
    p.y = y
    return p
}

fn Point.distance_from_origin() {
    return (self.x * self.x + self.y * self.y) ** 0.5
}

fn Point.describe() {
    return "(" + self.x.to_string() + ", " + self.y.to_string() + ")"
}

origin = Point.new(0, 0)
p1 = Point.new(3, 4)
print("Origin:", origin.describe())
print("P1:", p1.describe())
print("P1 distance:", p1.distance_from_origin())

print("")

# -----------------------------------------------------------------------------
# Properties with Rules
# -----------------------------------------------------------------------------
print("--- Properties with Rules ---")

PositiveCounter = graph{}
PositiveCounter.add_node("value", 0)
PositiveCounter.add_rule(:positive)  # Values must be positive

fn PositiveCounter.set(n) {
    self.value = n  # Will fail if n is negative
}

pc = PositiveCounter.clone()
pc.set(10)
print("Set to 10:", pc.value)
# pc.set(-5)  # Would fail due to :positive rule

print("")
print("=== Demo Complete ===")
```

---

## Phase 14: Inheritance with `from`

**Goal**: Enable `ChildGraph = graph from ParentGraph {}`.

### Syntax

```graphoid
Animal = graph{}
Animal.name = "unknown"

fn Animal.speak() {
    return "..."
}

Dog = graph from Animal {}
Dog.name = "dog"

fn Dog.speak() {
    return "Woof!"
}

# Dog has: name (overridden), speak() (overridden)
# Dog inherits any other methods/nodes from Animal
```

### Semantics

1. Evaluate parent expression
2. Clone parent completely (data, methods, rules)
3. Store parent reference (for `super` calls in Phase 16)
4. Apply `{}` initializers (if any)
5. Return new graph

### Files to Modify

| File | Changes |
|------|---------|
| `src/ast/mod.rs` | Add `parent: Option<Box<Expr>>` to GraphLiteral |
| `src/lexer/mod.rs` | Add `from` keyword |
| `src/parser/mod.rs` | Parse `graph from X {}` |
| `src/values/graph.rs` | Add `parent: Option<Box<Graph>>` field |
| `src/execution/executor.rs` | Implement inheritance evaluation |
| `tests/clg_inheritance_test.rs` | New test file |
| `samples/03-advanced/clg_inheritance.gr` | Example file |

### Test Cases

```rust
test_from_inherits_data_nodes
test_from_inherits_methods
test_from_inherits_rules
test_from_method_override
test_from_data_node_override
test_from_chained_inheritance  // A from B from C
test_from_parent_unchanged
test_from_with_initializers
test_from_clone_has_same_parent
```

---

## Phase 15: Private Methods

**Goal**: Methods starting with `_` are only callable from within the same graph's methods.

### Syntax

```graphoid
fn Secret._internal() {
    return "secret"
}

fn Secret.public() {
    return self._internal()  # OK
}

Secret._internal()  # ERROR: private method
```

### Semantics

- Method name starts with `_` = private
- Private methods callable only from methods of the same graph instance
- Inherited private methods ARE accessible to child (child is "same graph" after clone)

### Implementation

1. Track "current method graph" during method execution
2. Before calling method:
   - If name starts with `_`
   - AND we're not inside a method of the same graph
   - → Error

### Files to Modify

| File | Changes |
|------|---------|
| `src/execution/executor.rs` | Add call context tracking, privacy check |
| `src/values/graph.rs` | Add graph identity (UUID or counter) |
| `tests/clg_privacy_test.rs` | New test file |
| `samples/03-advanced/clg_privacy.gr` | Example file |

### Test Cases

```rust
test_private_callable_internally
test_private_blocked_externally
test_private_from_other_graph_blocked
test_private_inherited_callable
test_public_always_callable
test_underscore_data_node_unaffected
test_nested_private_calls
```

---

## Phase 16: Super Calls

**Goal**: Overriding methods can call parent's implementation via `super.method()`.

### Syntax

```graphoid
fn Animal.describe() {
    return "I am an animal"
}

Dog = graph from Animal {}

fn Dog.describe() {
    base = super.describe()
    return base + " and I am a dog"
}
```

### Semantics

- `super.method()` looks up method in parent graph, not current
- Walks up inheritance chain if not in immediate parent
- Error if used outside a method or no parent exists

### Files to Modify

| File | Changes |
|------|---------|
| `src/lexer/mod.rs` | Add `super` keyword |
| `src/ast/mod.rs` | Add `SuperMethodCall` expr variant |
| `src/parser/mod.rs` | Parse `super.method(args)` |
| `src/execution/executor.rs` | Implement super call resolution |
| `tests/clg_super_test.rs` | New test file |

### Test Cases

```rust
test_super_calls_parent_method
test_super_with_arguments
test_super_chained_inheritance
test_super_outside_method_fails
test_super_method_not_found
test_super_no_parent_fails
```

---

## Phase 17: Computed Properties (Getters)

**Goal**: No-arg methods without parens act as getters.

### Syntax

```graphoid
fn Circle.area {    # No parens = getter
    return 3.14159 * self.radius * self.radius
}

c.area              # Calls getter
```

### Semantics

- Method defined without `()` is a getter
- Access without `()` calls the getter
- Data node takes priority over getter if same name exists

### Files to Modify

| File | Changes |
|------|---------|
| `src/ast/mod.rs` | Add `is_getter: bool` to FunctionDef |
| `src/parser/mod.rs` | Parse method without parens |
| `src/execution/executor.rs` | Handle getter invocation |
| `tests/clg_getters_test.rs` | New test file |

### Test Cases

```rust
test_getter_definition
test_getter_access_without_parens
test_getter_access_with_parens_also_works
test_getter_vs_data_node_priority
test_getter_caching  // Future: memoization
```

---

## Phase 18: Type Checking

**Goal**: `is_a()` and `type_of()` for runtime type interrogation.

### Syntax

```graphoid
fido = Dog.new()
fido.is_a(Dog)      # true
fido.is_a(Animal)   # true (parent)
fido.is_a(Cat)      # false
fido.type_of()      # "Dog"
```

### Semantics

- `is_a(Type)` walks inheritance chain
- `type_of()` returns type name string
- Type name captured at assignment time

### Files to Modify

| File | Changes |
|------|---------|
| `src/values/graph.rs` | Add `type_name: Option<String>` |
| `src/execution/executor.rs` | Track type on assignment, implement methods |
| `tests/clg_types_test.rs` | New test file |

### Test Cases

```rust
test_is_a_self_type
test_is_a_parent
test_is_a_grandparent
test_is_a_unrelated_false
test_is_a_on_clone
test_type_of_returns_name
test_type_of_anonymous_graph
```

---

## Phase 19: Setters

**Goal**: Allow custom setter methods that are called on assignment.

### Syntax

```graphoid
fn Circle.radius=(value) {
    if value < 0 {
        error("Radius cannot be negative")
    }
    self["_radius"] = value
}

c.radius = 5    # Calls the setter
```

### Semantics

- Method named `property=` is a setter
- `obj.property = value` calls the setter with `value` as argument
- If no setter exists, falls back to data node assignment

### Files to Modify

| File | Changes |
|------|---------|
| `src/parser/mod.rs` | Parse `fn X.name=(param)` syntax |
| `src/execution/executor.rs` | Check for setter on property assignment |
| `tests/clg_setters_test.rs` | New test file |

### Test Cases

```rust
test_setter_definition
test_setter_called_on_assignment
test_setter_with_validation
test_setter_fallback_to_data_node
test_setter_and_getter_together
```

---

## Phase 20: Static Methods

**Goal**: Methods that don't use `self` work without an instance.

### Syntax

```graphoid
fn Math.sqrt(x) {
    return x ** 0.5
}

Math.sqrt(16)    # Works - no instance needed
```

### Semantics

- Methods that don't reference `self` are implicitly static
- Can be called on the graph directly without `.new()`
- Also callable on instances (for uniformity)

### Implementation

- During method execution, detect if `self` is referenced
- If not, method can be called without instance binding
- Alternative: analyze function body at attachment time

### Files to Modify

| File | Changes |
|------|---------|
| `src/execution/executor.rs` | Allow method calls without instance when no self reference |
| `tests/clg_static_test.rs` | New test file |

### Test Cases

```rust
test_static_method_no_self
test_static_callable_on_graph
test_static_callable_on_instance
test_non_static_requires_instance
test_mixed_static_and_instance_methods
```

---

## Phase 21: Structure-Based Dispatch

**Goal**: Methods can have guard clauses that dispatch based on graph structure.

### Syntax

```graphoid
fn Shape.area() when self.has_node("radius") {
    r = self.radius
    return 3.14159 * r * r
}

fn Shape.area() when self.has_node("width") {
    return self.width * self.height
}

fn Shape.area() {
    error("Unknown shape")
}
```

### Semantics

- Multiple method definitions with same name but different `when` guards
- Guards evaluated in definition order
- First matching guard wins
- Unguarded definition is fallback

### Implementation

- Store multiple method implementations per name
- Each has optional guard expression
- On call, evaluate guards until one matches

### Files to Modify

| File | Changes |
|------|---------|
| `src/ast/mod.rs` | Add `guard: Option<Expr>` to method definition |
| `src/parser/mod.rs` | Parse `when` clause after method signature |
| `src/values/graph.rs` | Store method as list of (guard, function) pairs |
| `src/execution/executor.rs` | Evaluate guards on method dispatch |
| `tests/clg_dispatch_test.rs` | New test file |

### Test Cases

```rust
test_dispatch_first_matching_guard
test_dispatch_fallback_no_guard
test_dispatch_multiple_guards
test_dispatch_guard_with_complex_condition
test_dispatch_no_match_error
```

---

## Phase 22: Mixins (`include`)

**Goal**: Copy methods from one graph to another without inheritance.

### Syntax

```graphoid
Walkable = graph{}
fn Walkable.walk() {
    return "walking..."
}

Swimmable = graph{}
fn Swimmable.swim() {
    return "swimming..."
}

Duck = graph{}
Duck.include(Walkable)
Duck.include(Swimmable)

fn Duck.speak() {
    return "Quack!"
}

# Duck now has: walk(), swim(), speak()
```

### Semantics

- `include(other)` copies all methods from `other` to `self`
- Does NOT establish parent reference (no `super` for included methods)
- Later includes override earlier ones on name conflict
- Source graph is not modified

### Implementation

- New graph method `include(other_graph)`
- Iterate other's methods and copy to self
- No parent chain modification

### Files to Modify

| File | Changes |
|------|---------|
| `src/execution/executor.rs` | Implement `include()` method |
| `tests/clg_mixins_test.rs` | New test file |
| `samples/03-advanced/clg_mixins.gr` | Example file |

### Test Cases

```rust
test_include_copies_methods
test_include_multiple_sources
test_include_conflict_last_wins
test_include_does_not_modify_source
test_include_no_super_for_included
test_include_preserves_existing_methods
```

---

## Phase 23: `responds_to()` Method

**Goal**: Check if a graph has a specific method.

### Syntax

```graphoid
Dog = graph{}
fn Dog.bark() { return "Woof!" }

Dog.responds_to("bark")     # true
Dog.responds_to("meow")     # false

# Use in guards
fn pet(animal) {
    if animal.responds_to("bark") {
        print("It's a dog!")
    }
}
```

### Semantics

- `responds_to(name)` returns true if graph has method with that name
- Includes inherited methods
- Does not check data nodes, only methods

### Implementation

- Simple method that checks `has_method(name)`
- Should check parent chain for inherited methods

### Files to Modify

| File | Changes |
|------|---------|
| `src/execution/executor.rs` | Implement `responds_to()` method |
| `tests/clg_responds_to_test.rs` | New test file |

### Test Cases

```rust
test_responds_to_own_method
test_responds_to_inherited_method
test_responds_to_missing_returns_false
test_responds_to_data_node_returns_false
test_responds_to_private_method
```

---

## Summary: What Each Phase Delivers

| After Phase | User Can Now... |
|-------------|-----------------|
| 13 | Use `self.prop` and `self["key"]` for clean data access |
| 14 | Create inherited graphs with `graph from Parent {}` |
| 15 | Define private methods with `_method()` convention |
| 16 | Call parent methods via `super.method()` |
| 17 | Define computed properties as getters |
| 18 | Check types with `is_a()` and `type_of()` |
| 19 | Define setter methods with validation |
| 20 | Call methods that don't need `self` without an instance |
| 21 | Define multiple method implementations with structural guards |
| 22 | Compose behaviors from multiple graphs with `include()` |
| 23 | Check method availability with `responds_to()` |

---

## Quick Reference: Design Decisions

1. **Property access**: Data layer only, method takes priority for calls
2. **Inheritance**: Clone-based (not prototype-based)
3. **Private**: Underscore convention with runtime enforcement
4. **Super**: Walks parent chain, error if no parent
5. **Getters**: No-parens methods, data node takes priority
6. **Setters**: `name=` convention, called on assignment
7. **Static**: Methods without `self` reference are implicitly static
8. **Dispatch**: `when` guards, first match wins, unguarded is fallback
9. **Mixins**: Copy methods, no inheritance chain
10. **Types**: Name captured at assignment, chain walked for `is_a`

See `CLG_DESIGN_ANALYSIS.md` for full rationale on each decision.

---

## Implementation Notes

### Suggested Order

1. **Phase 13** (Property Access) - Start here, highest impact
2. **Phases 14, 15, 20, 23** - Can be done in parallel (no dependencies)
3. **Phase 16** - Requires Phase 14
4. **Phase 17** - Requires Phase 13
5. **Phase 18, 22** - Require Phase 14
6. **Phase 19** - Requires Phase 17
7. **Phase 21** - Can be done anytime, most complex

### Dependency Graph

```
    ┌─────────────────────────────────────────┐
    │                                         │
    │  13 ──────► 17 ──────► 19               │
    │  (props)   (getters)  (setters)         │
    │                                         │
    │  14 ──┬──► 16 (super)                   │
    │  (from)│                                │
    │        ├──► 18 (types)                  │
    │        │                                │
    │        └──► 22 (mixins)                 │
    │                                         │
    │  15, 20, 21, 23  (independent)          │
    │                                         │
    └─────────────────────────────────────────┘
```

**Total estimated effort**: 35-45 hours for all phases
