# Phase 18: Complete Graph Model

**Duration**: 10-14 days
**Priority**: Critical (Completes graph-centric foundation)
**Dependencies**: Phase 15, 16, 17
**Status**: In Progress (Sections 1-4 Complete)

---

## Goal

Complete the graph-centric model by making remaining language constructs graph-native:
- Complex graphs (with properties, methods, behaviors)
- Type system as graph
- Patterns as graph templates
- Exceptions as alternative paths
- Iterators as traversable structures
- Effects as tracked nodes

---

## Universe Graph Structure

The **universe graph** is the top-level graph containing all runtime state. This section defines its canonical structure.

```
universe
│
├── types/                    # Type hierarchy (Section 2)
│   ├── any
│   ├── num
│   │   └── bignum
│   ├── string
│   ├── bool
│   └── collection
│       ├── list
│       ├── map
│       └── graph
│
├── modules/                  # Loaded modules (Phase 17)
│   ├── module:math
│   ├── module:json
│   └── module:my_app
│
├── packages/                 # Installed packages (Phase 21)
│   ├── package:better_json@1.2.3
│   ├── package:http_client@1.5.0
│   └── ...
│
├── connections/              # Database connections (Phase 22)
│   ├── pg_main              # Bridge Node → foreign handle
│   ├── sqlite_cache
│   └── ...
│
└── effects/                  # Effect log (Section 6)
    └── (effect nodes with ordering edges)
```

**Note**: There is no `classes/` or `instances/` subgraph. Graphoid has no classes - only graphs. Any graph can have functions, properties, or behaviors attached, making it "class-like" through usage, not through special designation. Graphs that serve as templates are just graphs; graphs instantiated from templates are also just graphs (possibly with an `instantiated_from` edge to the template).

### Querying the Universe

```graphoid
# Get the universe graph
u = reflect.universe()

# Query specific subgraphs
modules = u.modules()
packages = u.packages()
connections = u.connections()
types = u.types()

# Find graphs instantiated from a template
people = u.query({ has_edge: { label: "instantiated_from", to: "Person" } })

# Find all graphs with a specific behavior
validated = u.query({ has_edge: { label: "behavior", to: "rule:validate_age" } })
```

### Design Principles

1. **Namespaced subgraphs** - Related nodes grouped under common prefixes
2. **Edges express relationships** - `from`, `instantiated_from`, `imports`, `depends_on`
3. **Queryable structure** - Can introspect any part of the runtime
4. **Extensible** - Future phases add new subgraphs (connections, distributed nodes, etc.)

---

## 1. Complex Graphs (Partial)

Graphs can have functions, properties, and behaviors attached, making them more complex. There's no special "class" type - these are just graphs with more structure.

### Graph with Properties and Methods

A graph becomes "complex" when it has properties, methods, or behaviors attached:

```
┌─────────────────────────────────────────────────────────────────────┐
│  graph Person {                                                     │
│      name = ""                                                      │
│      age = 0                                                        │
│      fn greet() { return "Hello, " + name }                        │
│  }                                                                  │
│                                                                     │
│  Graph representation:                                              │
│                                                                     │
│  ┌──────────┐                                                       │
│  │ Person   │                                                       │
│  └────┬─────┘                                                       │
│       │                                                             │
│  ┌────┴────┬────────────┬────────────┐                              │
│  │         │            │            │                              │
│  ▼         ▼            ▼            ▼                              │
│ ┌──────┐ ┌──────┐   ┌──────────┐ ┌──────────┐                       │
│ │prop: │ │prop: │   │ method:  │ │configure │                       │
│ │name  │ │age   │   │ greet    │ │ settings │                       │
│ │def:"" │ │def:0 │   └────┬─────┘ └──────────┘                       │
│ └──────┘ └──────┘        │                                          │
│                          ▼                                          │
│                    (body subgraph)                                  │
└─────────────────────────────────────────────────────────────────────┘
```

### Instantiating a Graph

Creating a new graph based on another graph:

```
┌─────────────────────────────────────────────────────────────────────┐
│  alice = Person { name: "Alice", age: 30 }                          │
│                                                                     │
│  ┌────────┐                                                         │
│  │ alice  │──instantiated_from──► Person                            │
│  └───┬────┘                                                         │
│      │                                                              │
│  ┌───┴───┐                                                          │
│  ▼       ▼                                                          │
│ ┌────────┐ ┌────────┐                                               │
│ │ name:  │ │ age:   │                                               │
│ │"Alice" │ │ 30     │                                               │
│ └────────┘ └────────┘                                               │
│                                                                     │
│  Method call alice.greet():                                         │
│  1. Find "greet" via instantiated_from edge                         │
│  2. Bind "self" to alice                                            │
│  3. Execute method body                                             │
└─────────────────────────────────────────────────────────────────────┘
```

### Graph Extension

A graph can extend another graph, inheriting its structure:

```
┌─────────────────────────────────────────────────────────────────────┐
│  graph Employee from Person {                                       │
│      salary = 0                                                     │
│      fn work() { ... }                                             │
│  }                                                                  │
│                                                                     │
│  ┌──────────┐                                                       │
│  │ Employee │──from──► Person                                       │
│  └────┬─────┘                                                       │
│       │                                                             │
│  ┌────┴────┐                                                        │
│  ▼         ▼                                                        │
│ ┌────────┐ ┌────────┐                                               │
│ │ prop:  │ │method: │                                               │
│ │ salary │ │ work   │                                               │
│ └────────┘ └────────┘                                               │
│                                                                     │
│  Property lookup: check Employee, then traverse from edge to Person │
│  Method override: Employee.greet shadows Person.greet               │
└─────────────────────────────────────────────────────────────────────┘
```

### Behaviors as Graph Attachments

```graphoid
graph ValidatedPerson from Person {
    configure {
        behaviors: [:validate_age]
    }
}

# Behavior is an edge to a rule subgraph:
ValidatedPerson ──behavior──► rule:validate_age
                                  │
                                  ▼
                             (validation subgraph)
```

---

## 2. Type System as Graph (Partial)

**Implemented so far**: Persistent universe graph on GraphExecutor with 15 type nodes + 14 subtype_of edges. Module nodes added when modules load. Import edges added when exec_import runs. `reflect.universe()` returns clone of persistent graph. `reflect.type_hierarchy()` extracts type subgraph. `graph.has_node()` method exposed.

**Still needed**: `is_subtype()` function, `common_type()` function, user-defined graph types in type hierarchy.

### Type Hierarchy

```
┌─────────────────────────────────────────────────────────────────────┐
│  Type Graph                                                         │
│                                                                     │
│                       ┌───────┐                                     │
│                       │  any  │                                     │
│                       └───┬───┘                                     │
│            ┌──────────────┼──────────────┐                          │
│            ▼              ▼              ▼                          │
│        ┌──────┐       ┌──────┐      ┌──────┐                        │
│        │ num  │       │string│      │ bool │                        │
│        └──┬───┘       └──────┘      └──────┘                        │
│           │                                                         │
│           ▼                                                         │
│       ┌────────┐                                                    │
│       │ bignum │                                                    │
│       └────────┘                                                    │
│                                                                     │
│  Collection types:                                                  │
│        ┌──────────┐                                                 │
│        │collection│                                                 │
│        └────┬─────┘                                                 │
│      ┌──────┼──────┐                                                │
│      ▼      ▼      ▼                                                │
│   ┌────┐ ┌────┐ ┌─────┐                                             │
│   │list│ │map │ │graph│                                             │
│   └────┘ └────┘ └─────┘                                             │
│                                                                     │
│  User-defined graphs:                                               │
│   Person ──subtype──► any                                           │
│   Employee ──subtype──► Person                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Type Operations as Graph Operations

```graphoid
# Subtype check = path exists in type graph
is_subtype(:Employee, :Person)  # true (path exists)
is_subtype(:bignum, :num)       # true
is_subtype(:string, :num)       # false (no path)

# Common supertype = lowest common ancestor
common_type(:list, :map)        # :collection
common_type(:string, :num)      # :any

# Type inference = constraint propagation
fn add(a, b) { return a + b }
# Infer: a and b must support +, result is return type of +
# This is constraint edges in the type graph
```

### Runtime Type

Type is intrinsic to every value (via `typeof()`). No explicit `has_type` edges are needed — the type hierarchy in the universe graph is for querying subtype relationships, not for tagging individual values.

```graphoid
typeof(42)       # "num"
typeof("hi")     # "string"
typeof(alice)    # "graph"
```

---

## 3. Patterns as Graph Templates (Partial)

### Pattern Structure

```graphoid
match value {
    { name: n, age: a } if a > 18 => ...
}
```

The pattern is a graph template:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Pattern: { name: n, age: a } if a > 18                             │
│                                                                     │
│  ┌───────────────┐                                                  │
│  │ pattern:root  │                                                  │
│  └───────┬───────┘                                                  │
│          │                                                          │
│     ┌────┴────┐                                                     │
│     ▼         ▼                                                     │
│ ┌────────┐ ┌────────┐                                               │
│ │ name   │ │ age    │                                               │
│ │bind: n │ │bind: a │                                               │
│ └────────┘ └───┬────┘                                               │
│                │                                                    │
│                ▼                                                    │
│           ┌─────────┐                                               │
│           │ guard   │                                               │
│           │ a > 18  │                                               │
│           └─────────┘                                               │
│                                                                     │
│  Matching = subgraph isomorphism with binding                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Pattern Matching Algorithm

```rust
fn match_pattern(pattern: &PatternGraph, value: &ValueGraph) -> Option<Bindings> {
    let mut bindings = Bindings::new();

    // Try to find pattern as subgraph of value
    for pattern_node in pattern.nodes() {
        match pattern_node.type {
            PatternType::Literal(v) => {
                // Must match exactly
                if !value.has_node_with_value(v) {
                    return None;
                }
            }
            PatternType::Binding(name) => {
                // Capture value at this position
                let value_node = value.corresponding_node(pattern_node)?;
                bindings.insert(name, value_node.value());
            }
            PatternType::Wildcard => {
                // Always matches
            }
            PatternType::Guard(expr) => {
                // Evaluate guard with current bindings
                if !eval_guard(expr, &bindings) {
                    return None;
                }
            }
        }
    }

    // Check structural match (edges)
    for pattern_edge in pattern.edges() {
        if !value.has_matching_edge(pattern_edge) {
            return None;
        }
    }

    Some(bindings)
}
```

### Graph Pattern Matching

This extends to matching on graphs themselves:

```graphoid
g = graph {
    a -> b -> c
    a -> d
}

# Pattern match on graph structure
g.match({ from: x, edge: _, to: y } where x.value > y.value) { match =>
    print(match.x + " > " + match.y)
}
```

Pattern is a graph, value is a graph, matching is subgraph isomorphism.

---

## 4. Exceptions as Alternative Paths (Partial)

### Exception Path in Execution Graph

```
┌─────────────────────────────────────────────────────────────────────┐
│  try { risky() } catch e { recover(e) }                             │
│                                                                     │
│  ┌─────────┐                                                        │
│  │  try    │                                                        │
│  └────┬────┘                                                        │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────┐                                                        │
│  │ risky() │                                                        │
│  └────┬────┘                                                        │
│       │                                                             │
│  ┌────┴──────────────────┐                                          │
│  │                       │                                          │
│  ▼ (normal)              ▼ (exception)                              │
│ ┌────────┐          ┌─────────────┐                                 │
│ │ next   │          │ catch       │                                 │
│ │ stmt   │          │ bind: e     │                                 │
│ └────────┘          └──────┬──────┘                                 │
│                            │                                        │
│                            ▼                                        │
│                       ┌─────────┐                                   │
│                       │recover(e)│                                  │
│                       └─────────┘                                   │
│                                                                     │
│  Exception = take alternative edge instead of normal edge           │
└─────────────────────────────────────────────────────────────────────┘
```

### Exception Propagation

```
┌─────────────────────────────────────────────────────────────────────┐
│  Call stack with exception:                                         │
│                                                                     │
│  main() ──calls──► foo() ──calls──► bar() ──calls──► boom()        │
│                                                                     │
│  boom() raises exception:                                           │
│                                                                     │
│  boom() ──exception──► bar()   (no catch, propagate)                │
│         ──exception──► foo()   (has catch, handle)                  │
│                            │                                        │
│                            ▼                                        │
│                       (catch handler)                               │
│                                                                     │
│  Propagation = traverse "caller" edges until finding catch          │
└─────────────────────────────────────────────────────────────────────┘
```

### Exception Types as Graph

```
┌─────────────────────────────────────────────────────────────────────┐
│  Exception Hierarchy                                                │
│                                                                     │
│              ┌───────────┐                                          │
│              │ Exception │                                          │
│              └─────┬─────┘                                          │
│         ┌──────────┼──────────┐                                     │
│         ▼          ▼          ▼                                     │
│   ┌──────────┐ ┌────────┐ ┌─────────┐                               │
│   │ValueError│ │IOError │ │KeyError │                               │
│   └──────────┘ └────┬───┘ └─────────┘                               │
│                     │                                               │
│              ┌──────┴──────┐                                        │
│              ▼             ▼                                        │
│        ┌──────────┐  ┌───────────┐                                  │
│        │FileError │  │ NetError  │                                  │
│        └──────────┘  └───────────┘                                  │
│                                                                     │
│  catch IOError = catches FileError and NetError too                 │
│  (traverse subtype edges)                                           │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 5. Iterators as Traversable Structures

### Iterator as Graph Node

```graphoid
for x in items { ... }
```

```
┌─────────────────────────────────────────────────────────────────────┐
│  Iterator over [1, 2, 3]                                            │
│                                                                     │
│  ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────┐        │
│  │ iter     │────►│ iter     │────►│ iter     │────►│ done │        │
│  │ pos: 0   │next │ pos: 1   │next │ pos: 2   │next │      │        │
│  │ val: 1   │     │ val: 2   │     │ val: 3   │     └──────┘        │
│  └──────────┘     └──────────┘     └──────────┘                     │
│                                                                     │
│  for loop = follow "next" edges until "done"                        │
└─────────────────────────────────────────────────────────────────────┘
```

### Lazy/Infinite Iterators

```graphoid
# Infinite sequence
naturals = iterate(0, n => n + 1)

# Graph representation:
┌──────────┐     ┌──────────────────────────────────┐
│ iter     │────►│ generator                        │
│ val: 0   │next │ fn: n => n + 1                   │
└──────────┘     │ (generates next node on demand)  │
                 └──────────────────────────────────┘
```

### Iterator Combinators as Graph Transformations

```graphoid
items.filter(x => x > 0).map(x => x * 2)
```

```
┌─────────────────────────────────────────────────────────────────────┐
│  Iterator Pipeline                                                  │
│                                                                     │
│  ┌────────┐     ┌────────────┐     ┌───────────┐     ┌────────┐     │
│  │ source │────►│ filter     │────►│ map       │────►│ result │     │
│  │ items  │pipe │ x > 0      │pipe │ x * 2     │     │        │     │
│  └────────┘     └────────────┘     └───────────┘     └────────┘     │
│                                                                     │
│  Each stage is a node, pipe is edge                                 │
│  Lazy: only compute when result is consumed                         │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 6. Effects as Tracked Nodes

### Effect Tracking

```graphoid
fn read_file(path) {
    # This has IO effect
    content = fs.read(path)  # effect: io
    return content
}
```

```
┌─────────────────────────────────────────────────────────────────────┐
│  Execution with effect tracking                                     │
│                                                                     │
│  ┌─────────┐     ┌─────────────────┐     ┌─────────┐                │
│  │ call    │────►│ effect:io       │────►│ result  │                │
│  │ fs.read │     │ type: read      │     │ content │                │
│  └─────────┘     │ target: path    │     └─────────┘                │
│                  └─────────────────┘                                │
│                                                                     │
│  Effect node records what happened                                  │
│  Can be used for: logging, replay, undo, sandboxing                │
└─────────────────────────────────────────────────────────────────────┘
```

### Effect Types

```
┌─────────────────────────────────────────────────────────────────────┐
│  Effect Graph                                                       │
│                                                                     │
│                  ┌────────┐                                         │
│                  │ effect │                                         │
│                  └───┬────┘                                         │
│         ┌────────────┼────────────┐                                 │
│         ▼            ▼            ▼                                 │
│     ┌──────┐    ┌────────┐   ┌─────────┐                            │
│     │  io  │    │ state  │   │ network │                            │
│     └──┬───┘    └────────┘   └─────────┘                            │
│    ┌───┴───┐                                                        │
│    ▼       ▼                                                        │
│ ┌─────┐ ┌──────┐                                                    │
│ │read │ │write │                                                    │
│ └─────┘ └──────┘                                                    │
│                                                                     │
│  Effect analysis = traverse effect edges                            │
│  Pure function = no effect edges                                    │
└─────────────────────────────────────────────────────────────────────┘
```

### Effect Ordering

```graphoid
fn do_stuff() {
    x = read_input()    # effect 1
    write_output(x)     # effect 2
    log("done")         # effect 3
}
```

```
┌─────────────────────────────────────────────────────────────────────┐
│  Effect ordering as edges                                           │
│                                                                     │
│  ┌──────────────┐     ┌───────────────┐     ┌─────────────┐         │
│  │ effect:read  │────►│ effect:write  │────►│ effect:log  │         │
│  │ input        │then │ output        │then │ "done"      │         │
│  └──────────────┘     └───────────────┘     └─────────────┘         │
│                                                                     │
│  "then" edges define happens-before ordering                        │
│  Enables: replay, undo, distributed consistency                     │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Implementation Plan

### Day 1: Universe Graph Structure

- [x] Define canonical universe graph structure
- [x] Implement `reflect.universe()` returning universe graph
- [ ] Implement subgraph accessors: `.modules()`, `.packages()`, `.connections()`, `.types()`
- [x] Tests for universe structure queries

### Week 1: Complex Graphs (Days 2-5)

- [x] Graphs with properties and methods (pre-existing)
- [ ] Graph instantiation with `instantiated_from` edge
- [x] Graph extension via `from` edge (pre-existing)
- [ ] Method lookup via `instantiated_from` edge traversal
- [x] `configure { behaviors: [...] }` applies rules to graph declarations
- [x] Graph templates registered as `graph:Name` nodes in universe
- [ ] Tests for instantiated_from and method lookup

### Week 1-2: Types and Patterns (Days 5-8)

- [x] Type hierarchy as graph (15 nodes + 14 subtype_of edges)
- [~] `is_subtype()` function — SKIPPED: not useful to users; type hierarchy already queryable via `reflect.type_hierarchy()` + `has_path()`
- [~] `common_type()` function — SKIPPED: academic; no practical use case in a duck-typed language
- [~] User-defined graph types get `subtype_of` edge — SKIPPED: tied to above
- [x] `reflect.pattern()` returning pattern as queryable graph
- [x] `g.match()` graph pattern matching (pre-existing)
- [x] Tests for pattern reflection

### Week 2: Exceptions (Days 9-10)

- [x] Error type hierarchy in universe graph (6 types + 2 IOError subtypes)
- [x] `catch IOError` catches FileError/NetError via `has_path()`
- [x] Exception propagation via caller edges in execution graph
- [x] Tests for error hierarchy and catch subtypes

### ~~Week 2: Iterators as Graph (Days 11-12)~~ — KILLED

Not needed. Iterators are not in the language spec, and wrapping conventional iteration
in graph nodes doesn't advance the "everything is a graph" philosophy.

### ~~Week 2: Effects (Days 13-14)~~ — KILLED

Not needed. Effect tracking is not in the language spec and doesn't aid the
graph-theoretic re-architecture.

---

## Success Criteria

- [x] Universe graph structure defined with namespaced subgraphs
- [x] `reflect.universe()` returns queryable universe graph
- [x] Complex graphs have properties, methods, behaviors as subgraph structure
- [x] Graph instantiation creates `instantiated_from` edges
- [x] Graph extension works via `from` edges
- [x] Type hierarchy is a graph
- [~] `is_subtype()` and `common_type()` — SKIPPED (redundant; graph is already queryable)
- [x] Pattern reflection via `reflect.pattern()`
- [x] `g.match()` uses graph pattern matching
- [x] Exceptions propagate via caller edges
- [~] Iterators as graph nodes — KILLED (not in language spec, doesn't advance graph philosophy)
- [~] Effects as tracked nodes — KILLED (not in language spec, doesn't advance graph philosophy)
- [x] All existing tests pass
- [x] New graph-model tests pass

---

## Test Cases

```graphoid
# test_complex_graph.gr
graph Animal {
    name = ""
    fn speak() { return "..." }
}

graph Dog from Animal {
    fn speak() { return "woof" }
}

# Graphs exist (they're just values, not in a special subgraph)
assert(Animal != none, "Animal graph exists")
assert(Dog != none, "Dog graph exists")

# Extension edge exists
assert(Dog.has_edge({ label: "from", to: Animal }), "from edge")

# Instantiated graph has instantiated_from edge
fido = Dog { name: "Fido" }
assert(fido.has_edge({ label: "instantiated_from", to: Dog }), "instantiated_from edge")

# test_type_graph.gr
type_graph = reflect.type_hierarchy()
assert(type_graph.has_path("type:bignum", "type:any"), "bignum is subtype of any")
assert(not type_graph.has_path("type:string", "type:num"), "string not subtype of num")

# test_pattern_graph.gr
pattern = reflect.pattern({ name: n, age: a } if a > 18)
assert(pattern.has_node({ type: :binding, name: "n" }), "binding node")
assert(pattern.has_node({ type: :guard }), "guard node")

# test_exception_graph.gr
fn risky() { raise ValueError("oops") }
fn safe() {
    try { risky() } catch e { return "caught" }
}
# Exception path should exist in execution graph
# (This would require execution tracing to verify)

# test_iterator_graph.gr
iter = [1, 2, 3].iter()
iter_graph = reflect.iterator(iter)
assert(iter_graph.has_edge({ label: "next" }), "has next edge")

# test_effect_graph.gr
fn effectful() {
    x = io.read("file.txt")
    io.write("out.txt", x)
}
effects = reflect.effects(effectful)
assert(effects.length() == 2, "two effects")
assert(effects[0].type == :io_read, "first is read")
assert(effects[1].type == :io_write, "second is write")
```

---

## Related Documents

- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Namespace foundation
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Execution foundation
- [PHASE_17_MODULES_GRAPH.md](PHASE_17_MODULES_GRAPH.md) - Module graphs
- [GRAPH_RUNTIME_TEST_SPECIFICATION.md](GRAPH_RUNTIME_TEST_SPECIFICATION.md) - Testing framework
