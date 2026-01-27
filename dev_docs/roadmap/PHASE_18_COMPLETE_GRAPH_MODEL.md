# Phase 18: Complete Graph Model

**Duration**: 10-14 days
**Priority**: Critical (Completes graph-centric foundation)
**Dependencies**: Phase 15, 16, 17
**Status**: Planning

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
│   │   ├── int
│   │   └── float
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

## 1. Complex Graphs

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

## 2. Type System as Graph

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
│       ┌───┴───┐                                                     │
│       ▼       ▼                                                     │
│   ┌──────┐ ┌──────┐                                                 │
│   │ int  │ │float │                                                 │
│   └──────┘ └──────┘                                                 │
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
is_subtype(Employee, Person)  # true (path exists)
is_subtype(int, num)          # true
is_subtype(string, num)       # false (no path)

# Common supertype = lowest common ancestor
common_type(int, float)       # num
common_type(string, int)      # any

# Type inference = constraint propagation
fn add(a, b) { return a + b }
# Infer: a and b must support +, result is return type of +
# This is constraint edges in the type graph
```

### Runtime Type as Node Property

```
Every value node has type edge:
  value:42 ──has_type──► type:int
  value:"hi" ──has_type──► type:string
  alice ──has_type──► Person
```

---

## 3. Patterns as Graph Templates

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

## 4. Exceptions as Alternative Paths

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

- [ ] Define canonical universe graph structure
- [ ] Implement `reflect.universe()` returning universe graph
- [ ] Implement subgraph accessors: `.modules()`, `.packages()`, `.connections()`, `.types()`
- [ ] Tests for universe structure queries

### Week 1: Complex Graphs (Days 2-5)

- [ ] Graphs with properties and methods
- [ ] Graph instantiation with instantiated_from edge
- [ ] Graph extension via from edge
- [ ] Method lookup via graph traversal
- [ ] Tests for complex graph model

### Week 1-2: Types and Patterns (Days 5-8)

- [ ] Type hierarchy as graph
- [ ] Subtype check as path finding
- [ ] Pattern as graph template
- [ ] Match as subgraph isomorphism
- [ ] Tests for type and pattern graphs

### Week 2: Exceptions and Iterators (Days 9-12)

- [ ] Exception paths in execution graph
- [ ] Propagation via caller edges
- [ ] Iterator as node with next edges
- [ ] Lazy generators
- [ ] Tests for exceptions and iterators

### Week 2: Effects (Days 13-14)

- [ ] Effect nodes in execution graph
- [ ] Effect type hierarchy
- [ ] Effect ordering edges
- [ ] Integration and final tests

---

## Success Criteria

- [ ] Universe graph structure defined with namespaced subgraphs
- [ ] `reflect.universe()` returns queryable universe graph
- [ ] Complex graphs have properties, methods, behaviors as subgraph structure
- [ ] Graph instantiation creates instantiated_from edges
- [ ] Graph extension works via from edges
- [ ] Type hierarchy is a graph
- [ ] Pattern matching uses subgraph isomorphism
- [ ] Exceptions use alternative execution paths
- [ ] Iterators are traversable node chains
- [ ] Effects are tracked as nodes
- [ ] All existing tests pass
- [ ] New graph-model tests pass

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
type_graph = reflect.type_graph()
assert(type_graph.has_path("int", "any"), "int is subtype of any")
assert(not type_graph.has_path("string", "num"), "string not subtype of num")

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
