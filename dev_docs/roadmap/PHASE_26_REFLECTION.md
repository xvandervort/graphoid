# Phase 26: Runtime Reflection

**Duration**: 5-7 days
**Priority**: Low
**Dependencies**: Phase 18 (Complete Graph Model)
**Status**: Ready (can be done anytime after Phase 18)

---

## Goal

Add runtime reflection capabilities that expose Graphoid's graph structure programmatically. Reflection IS graph traversal - inspecting types, fields, and methods means traversing the universe graph and value graphs.

**Key principle**: Reflection is not a separate system bolted onto Graphoid. It's the natural consequence of "everything is a graph" - if everything is a graph, you can traverse and inspect everything.

---

## Core Concept: Reflection as Graph Traversal

```
┌─────────────────────────────────────────────────────────────────┐
│  Traditional Reflection          Graphoid Reflection            │
│                                                                 │
│  reflect(obj) → TypeInfo         reflect(obj) → Graph View      │
│  info.type_name → "Person"       info.type() → type:Person node │
│  info.fields → ["name", "age"]   info.fields() → property edges │
│  info.methods → ["greet"]        info.methods() → method nodes  │
│                                                                 │
│  Reflection IS graph traversal, not a separate API              │
└─────────────────────────────────────────────────────────────────┘
```

---

## Universe Graph Reflection

The universe graph (Phase 18) is fully introspectable:

```graphoid
# Get the universe graph
u = reflect.universe()

# Query system subgraphs
u.types()        # Type hierarchy graph
u.modules()      # Loaded modules
u.packages()     # Installed packages
u.connections()  # Database connections (Phase 22)
u.distribution() # Distribution state (Phase 23)
u.effects()      # Effect log

# Traverse the type graph
type_graph = u.types()
type_graph.has_path("int", "any")        # true - int is subtype of any
type_graph.traverse(from: "collection")  # list, map, graph, tree, hnsw

# Find all graphs instantiated from a template
people = u.query({ has_edge: { label: "instantiated_from", to: "Person" } })

# Find all values with a specific behavior
validated = u.query({ has_edge: { label: "behavior", to: "rule:validate_age" } })
```

### Universe Graph Structure

```
universe
├── types/                    # Type hierarchy (graph)
│   ├── any
│   │   ├── num → int, float
│   │   ├── string
│   │   ├── bool
│   │   └── collection → list, map, graph
│   └── ... user-defined types
│
├── modules/                  # Loaded modules
│   ├── module:math
│   ├── module:json
│   └── module:my_app
│
├── packages/                 # Installed packages (Phase 21)
├── connections/              # Database connections (Phase 22)
├── distribution/             # Distribution state (Phase 23)
└── effects/                  # Effect log (Phase 18)
```

---

## Value Reflection

Every value is a graph (or can be viewed as one). Reflection exposes this structure:

### Basic Value Inspection

```graphoid
person = Person { name: "Alice", age: 30 }

# reflect() returns a graph view of the value
view = reflect(person)

# Type is a traversal to the type graph
view.type()              # Returns node reference to type:Person
view.type().name         # "Person"
view.type().parent()     # Traverses "from" edge to parent type

# Fields are property edges in the graph
view.fields()            # ["name", "age"] - property edge labels
view.field("name")       # Returns the property node
view.field("name").value # "Alice"

# Methods are method nodes
view.methods()           # ["greet"] - method node names
view.method("greet")     # Returns the method node (contains body subgraph)
```

### Five-Layer Inspection

Reflection exposes all five layers of graph-backed values:

```graphoid
# Create a graph with behaviors and rules
temps = [98.6, none, 102.1]
temps.add_rule("none_to_zero")
temps.add_rule("validate_range", 95, 110)

view = reflect(temps)

# DataLayer - the actual data
view.data()              # Graph view of data nodes and edges
view.data().nodes()      # [98.6, 0, 102.1] (after behavior applied)
view.data().edges()      # Sequential edges for list structure

# BehaviorLayer - transformation rules
view.behaviors()         # Graph of active behaviors
view.behaviors().transformations()  # [rule:none_to_zero]
view.behaviors().validators()       # [rule:validate_range]

# ControlLayer - validation rules
view.control()           # Graph of control rules
view.control().active_rules()

# MetadataLayer - history and provenance
view.metadata()          # Graph of metadata
view.metadata().creation_time()
view.metadata().operation_log()
```

### Type Checking via Graph Traversal

```graphoid
value = get_some_value()
view = reflect(value)

# Type checking is path-finding in type graph
if view.type().name == "list" {
    print("It's a list")
}

# Subtype checking is path existence
if reflect.universe().types().has_path(view.type().name, "collection") {
    print("It's a collection type")
}

# Check if value has a method (method node exists)
if view.has_method("to_json") {
    json = value.to_json()
}

# Check if value has a field (property edge exists)
if view.has_field("name") {
    print(value.name)
}
```

---

## Dynamic Invocation

Dynamic method calls work through the graph structure:

```graphoid
obj = Calculator { x: 10 }
method_name = "add"

# invoke() finds the method node and executes its body subgraph
result = obj.invoke(method_name, [5])  # 15

# This is equivalent to traversing:
# obj → method:add → body_subgraph → execute
```

### Field Access by Name

```graphoid
person = Person { name: "Alice", age: 30 }

# Dynamic field access traverses property edges
name = person.get_field("name")      # "Alice"
person.set_field("age", 31)          # Updates property node value

# Iterate all fields via reflection
for field_name in reflect(person).fields() {
    value = person.get_field(field_name)
    print(field_name + ": " + value.to_string())
}
```

---

## Implementation Plan

### Day 1-2: Universe Graph Accessors

```rust
// reflect.universe() returns the universe graph
fn builtin_reflect_universe() -> Value {
    Value::Graph(UNIVERSE_GRAPH.clone())
}

// Subgraph accessors
impl UniverseGraph {
    fn types(&self) -> Graph { self.subgraph("types") }
    fn modules(&self) -> Graph { self.subgraph("modules") }
    fn packages(&self) -> Graph { self.subgraph("packages") }
    fn connections(&self) -> Graph { self.subgraph("connections") }
    fn distribution(&self) -> Graph { self.subgraph("distribution") }
    fn effects(&self) -> Graph { self.subgraph("effects") }
}
```

### Day 3-4: Value Reflection

```rust
// reflect(value) returns a graph view
fn builtin_reflect(args: &[Value]) -> Result<Value> {
    let value = &args[0];
    Ok(Value::ReflectionView(ReflectionView::new(value)))
}

struct ReflectionView {
    target: Value,
}

impl ReflectionView {
    // Type traversal
    fn type_node(&self) -> Value {
        // Find type node in universe:types/
        let type_name = self.target.type_name();
        universe().types().get_node(&format!("type:{}", type_name))
    }

    // Field enumeration (property edges)
    fn fields(&self) -> Vec<String> {
        match &self.target {
            Value::Graph(g) => g.edges()
                .filter(|e| e.edge_type == "property")
                .map(|e| e.label.clone())
                .collect(),
            _ => vec![],
        }
    }

    // Method enumeration (method nodes)
    fn methods(&self) -> Vec<String> {
        match &self.target {
            Value::Graph(g) => g.edges()
                .filter(|e| e.edge_type == "method")
                .map(|e| e.to.clone())
                .collect(),
            _ => self.builtin_methods(),
        }
    }

    // Five-layer access
    fn data(&self) -> Option<&DataLayer> { ... }
    fn behaviors(&self) -> Option<&BehaviorLayer> { ... }
    fn control(&self) -> Option<&ControlLayer> { ... }
    fn metadata(&self) -> Option<&MetadataLayer> { ... }
}
```

### Day 5: Dynamic Invocation

```rust
impl Value {
    fn invoke(&self, method_name: &str, args: &[Value]) -> Result<Value> {
        // Find method node via graph traversal
        let method_node = self.find_method_node(method_name)?;

        // Execute the method's body subgraph
        method_node.execute_with_args(self, args)
    }

    fn get_field(&self, name: &str) -> Result<Value> {
        // Traverse property edge to find value
        match self {
            Value::Graph(g) => {
                let prop_node = g.traverse_edge("property", name)?;
                Ok(prop_node.value.clone())
            }
            Value::Map(m) => Ok(m.get(name).cloned().unwrap_or(Value::None)),
            _ => Err(RuntimeError::NotAnObject),
        }
    }

    fn set_field(&mut self, name: &str, value: Value) -> Result<()> {
        // Update property node value
        match self {
            Value::Graph(g) => g.set_property(name, value),
            Value::Map(m) => { m.insert(name.to_string(), value); Ok(()) },
            _ => Err(RuntimeError::NotAnObject),
        }
    }
}
```

### Day 6-7: Testing & Documentation

- Unit tests for universe graph traversal
- Unit tests for value reflection
- Integration tests with real use cases
- Documentation and examples

---

## Success Criteria

- [ ] `reflect.universe()` returns queryable universe graph
- [ ] `reflect.universe().types()` returns type hierarchy graph
- [ ] `reflect.universe().modules()` returns module graph
- [ ] `reflect(value)` returns graph view of value
- [ ] `view.type()` traverses to type node
- [ ] `view.fields()` returns property edge labels
- [ ] `view.methods()` returns method node names
- [ ] `view.data()`, `.behaviors()`, `.control()`, `.metadata()` expose five layers
- [ ] `has_field()` and `has_method()` work via graph queries
- [ ] `invoke(method_name, args)` works via graph traversal
- [ ] `get_field()` and `set_field()` work via property edges
- [ ] At least 30 reflection tests
- [ ] Example: Generic serializer using graph traversal
- [ ] Example: Plugin system using method discovery
- [ ] Documentation complete

---

## Example: Generic Serializer via Graph Traversal

```graphoid
# Serialize any object by traversing its graph structure
fn to_map(obj) {
    view = reflect(obj)
    result = {}

    result["__type__"] = view.type().name

    # Traverse property edges to get fields
    for field_name in view.fields() {
        value = obj.get_field(field_name)

        # Recursively serialize graph values
        if reflect(value).type().is_subtype_of("graph") {
            value = to_map(value)
        }

        result[field_name] = value
    }

    return result
}

# Usage
person = Person { name: "Alice", age: 30 }
data = to_map(person)
# { "__type__": "Person", "name": "Alice", "age": 30 }
```

---

## Example: Plugin System via Method Discovery

```graphoid
# Load and invoke plugins by discovering methods in their graphs
fn run_plugins(plugins, event, data) {
    results = []
    handler_name = "on_" + event

    for plugin in plugins {
        # Check if plugin graph has handler method node
        if reflect(plugin).has_method(handler_name) {
            result = plugin.invoke(handler_name, [data])
            results.append(result)
        }
    }

    return results
}

# Plugins are just graphs with method nodes
graph LoggingPlugin {
    fn on_request(data) {
        print("Request: " + data.to_string())
        return data
    }
}

graph ValidationPlugin {
    fn on_request(data) {
        if !data.has("user_id") {
            raise "Missing user_id"
        }
        return data
    }
}

# Run all plugins that have on_request method nodes
plugins = [LoggingPlugin {}, ValidationPlugin {}]
run_plugins(plugins, "request", { user_id: 123 })
```

---

## Example: Type Introspection via Type Graph

```graphoid
# Explore the type hierarchy
type_graph = reflect.universe().types()

# Find all subtypes of "collection"
collection_types = type_graph.traverse(
    from: "type:collection",
    direction: :outgoing,
    edge: "subtype"
)
print(collection_types)  # [type:list, type:map, type:graph, type:tree, type:hnsw]

# Check inheritance chain
fn inheritance_chain(type_name) {
    chain = [type_name]
    current = type_graph.get_node("type:" + type_name)

    while current.has_edge("from") {
        parent = current.traverse_edge("from")
        chain.append(parent.name)
        current = parent
    }

    return chain
}

print(inheritance_chain("hnsw"))  # ["hnsw", "graph", "collection", "any"]
```

---

## Open Questions

1. **Performance** - Should reflection views be cached?
2. **Mutability** - Can reflection modify the universe graph directly?
3. **Security** - Should some parts of the universe graph be hidden?

---

## Related Documents

- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Universe graph structure
- [ARCHITECTURE_DESIGN.md](../ARCHITECTURE_DESIGN.md) - Five-layer architecture
