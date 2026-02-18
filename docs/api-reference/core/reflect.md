# reflect - Runtime Introspection

The `reflect` module provides runtime access to Graphoid's internal structures: the module universe, type hierarchy, loaded modules, and pattern introspection. All results are returned as queryable graphs, following Graphoid's "everything is a graph" philosophy.

---

## Methods

### reflect.universe()

Returns the persistent universe graph containing type nodes, module nodes, and import edges.

```graphoid
u = reflect.universe()
print(u.nodes().length())               # Number of nodes in universe
print(u.has_node("type:num"))            # true
print(u.has_path("type:bignum", "type:num"))  # true (bignum is subtype of num)
print(u.has_node("module:math"))         # true (after importing math)
```

**Returns**: `graph` — A clone of the universe graph.

---

### reflect.type_hierarchy()

Returns a subgraph containing only the type nodes and `subtype_of` edges.

```graphoid
th = reflect.type_hierarchy()
print(th.has_path("type:list", "type:collection"))   # true
print(th.has_path("type:string", "type:num"))         # false
```

**Returns**: `graph` — The type hierarchy subgraph.

---

### reflect.loaded_modules()

Returns a list of all currently loaded module names.

```graphoid
import "math"
import "json"
modules = reflect.loaded_modules()
print(modules)  # ["math", "json"]
```

**Returns**: `list` — Module name strings.

---

### reflect.module(name)

Returns information about a loaded module, or `none` if not found.

```graphoid
import "math"
m = reflect.module("math")
if m != none {
    print(m.exports())
    print(m.path())
}
```

**Parameters**:
- `name` (`string`) — The module name to look up.

**Returns**: Module object or `none`.

---

### reflect.current_scope()

Returns a map describing the current execution scope.

```graphoid
scope = reflect.current_scope()
print(scope["type"])       # "main" or "function"
print(scope["depth"])      # Nesting depth
print(scope["variables"])  # List of variable names in scope
```

**Returns**: `map` — Scope information with keys `type`, `depth`, `variables`.

---

### reflect.pattern(expr)
### reflect.pattern(expr, guard)

Introspects an expression as a pattern and returns it as a queryable graph. This is a **special form**: the first argument is not evaluated. Instead, its syntactic structure is walked to produce a pattern graph with binding, literal, wildcard, and guard nodes.

```graphoid
p = reflect.pattern({ name: n, age: a })
print(p.has_node("field:name"))   # true
print(p.has_node("field:age"))    # true

field = p.get_node("field:name")
print(field["type"])   # "binding"
print(field["key"])    # "name"
print(field["name"])   # "n"
```

**Parameters**:
- `expr` — An unevaluated expression to interpret as a pattern. Identifiers become bindings, `_` becomes a wildcard, literals stay as literal match values.
- `guard` (optional) — A lambda representing the guard condition.

**Returns**: `graph` — A pattern graph.

#### Supported expression types

| Expression | Pattern Type | Example |
|------------|-------------|---------|
| `{ key: var }` | Map destructuring | `reflect.pattern({ name: n })` |
| `[a, b, c]` | List destructuring | `reflect.pattern([head, 1, _])` |
| `42`, `"hi"`, `true`, `:sym` | Literal match | `reflect.pattern(42)` |
| `x` | Binding | `reflect.pattern(x)` |
| `_` | Wildcard | `reflect.pattern(_)` |

#### Graph structure

**Map pattern** — `reflect.pattern({ name: n, age: 30 })`:

```
pattern:root  →  { pattern_type: "map", field_count: 2 }
  ├── has_field → field:name  →  { type: "binding", key: "name", name: "n" }
  └── has_field → field:age   →  { type: "literal", key: "age", value: 30 }
```

**List pattern** — `reflect.pattern([x, 42, _])`:

```
pattern:root  →  { pattern_type: "list", element_count: 3 }
  ├── has_element → element:0  →  { type: "binding", index: 0, name: "x" }
  ├── has_element → element:1  →  { type: "literal", index: 1, value: 42 }
  └── has_element → element:2  →  { type: "wildcard", index: 2 }
```

**Literal / binding / wildcard** — `reflect.pattern(42)`:

```
pattern:root  →  { pattern_type: "literal", value: 42 }
```

**With guard** — `reflect.pattern({ age: a }, a => a > 18)`:

```
pattern:root  →  { pattern_type: "map", field_count: 1 }
  ├── has_field → field:age  →  { type: "binding", key: "age", name: "a" }
  └── has_guard → guard:0    →  { type: "guard" }
```

#### Practical example: extract all bindings from a pattern

```graphoid
p = reflect.pattern({ first: f, last: l, email: _ })
for node_id in p.nodes() {
    if node_id != "pattern:root" {
        val = p.get_node(node_id)
        if val["type"] == "binding" {
            print("Binds: " + val["name"])
        }
    }
}
# Output:
# Binds: f
# Binds: l
```

---

### reflect.call_graph()

Returns the function call graph as a queryable graph. Includes function nodes, call edges, and exception propagation edges.

```graphoid
fn greet(name) { return "hello " + name }
greet("world")

cg = reflect.call_graph()
print(cg.has_node("fn:greet"))          # true
node = cg.get_node("fn:greet")
print(node["name"])                      # "greet"
print(node["call_count"])                # 1
```

**Returns**: `graph` — The function call graph.

#### Node structure

Each function that has been called becomes a node with ID `fn:{name}`:

```
fn:greet  →  { name: "greet", call_count: 1 }
```

#### Edge types

| Edge Type | Label | Meaning |
|-----------|-------|---------|
| Call | `calls` | Normal function call (A called B) |
| Exception | `exception` | Exception propagated from A to B |
| Capture | `captures` | Closure captured a variable |

#### Exception propagation

When an exception propagates through function calls, `exception` edges trace the path:

```graphoid
fn boom() { raise ValueError("oops") }
fn middle() { boom() }
fn outer() { middle() }

try { outer() } catch as e { }

cg = reflect.call_graph()
edges = cg.edges()
for edge in edges {
    if edge[2] == "exception" {
        print(edge[0] + " --> " + edge[1])
    }
}
# Output:
# fn:boom --> fn:middle
# fn:middle --> fn:outer
```
