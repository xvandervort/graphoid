# Graph Declaration Syntax Design

**Created**: December 10, 2025
**Updated**: December 12, 2025
**Status**: Phases 1-6 COMPLETE
**Purpose**: Define proper class-like graph declaration syntax with intrinsic naming

---

## Motivation

Graphs with class-like properties need **intrinsic names**, not names derived from variable assignment. A class/type must know its own identity for:

- **Type reflection**: `type(dog)` should return `"Dog"`, not the variable name
- **Error messages**: "Expected Animal, got String" requires knowing the type name
- **Debugging**: Stack traces need meaningful type names
- **Identity**: Reassigning to a different variable shouldn't change what something *is*

The old assignment-based syntax (`Animal = graph {}`) conflated "binding a value to a name" with "declaring a type with identity". These are fundamentally different operations.

---

## Core Syntax

### Named Graph Declaration (Class-Like)

Named graphs are **declarations**, not expressions assigned to variables:

```graphoid
graph Animal {
    sound: none

    fn speak() {
        print(sound)
    }
}
```

This creates a graph type named `Animal` that:
- Knows its own name (`Animal`)
- Is automatically bound to the identifier `Animal` in the current scope
- Can be used as a parent for inheritance
- Reports its type correctly via `type()`

### Anonymous Graph (Instance)

Anonymous graphs are **expressions** that create instances:

```graphoid
# Anonymous graph assigned to variable
origin = graph { x: 0, y: 0 }

# Typed anonymous graph
dag = graph(:dag) { nodes: [] }
```

Anonymous graphs:
- Have no intrinsic name (or a generated name like `<anonymous>`)
- Are typically one-off instances, not reusable types
- Can still have properties and methods

### The Key Distinction

| Syntax | Semantics | Use Case |
|--------|-----------|----------|
| `graph Name { }` | Declaration with intrinsic identity | Defining reusable types |
| `x = graph { }` | Anonymous expression | Creating one-off instances |

---

## Properties

Properties use `:` assignment inside the graph body (same as hash literals):

```graphoid
graph Point {
    x: 0
    y: 0
}
```

Properties are mutable by default. Use directives for access control.

---

## Methods

Methods are defined with `fn` inside the body. No receiver prefix needed—we're inside the graph:

```graphoid
graph Point {
    x: 0
    y: 0

    fn distance(other) {
        dx = x - other.x
        dy = y - other.y
        return (dx*dx + dy*dy).sqrt()
    }
}
```

---

## Implicit Self

Inside a graph body, bare identifiers resolve to graph properties/methods. No `self.` prefix needed:

```graphoid
graph Counter {
    count: 0

    fn increment() {
        count = count + 1  # Reads and writes the property
    }

    fn reset() {
        count = 0
    }
}
```

If you need to pass the graph itself, `self` is still available:

```graphoid
graph Builder {
    value: 0

    fn add(n) {
        value = value + n
        return self  # For chaining
    }
}
```

---

## Inheritance

Use `from` keyword after the graph name:

```graphoid
graph Animal {
    sound: none

    fn speak() {
        print(sound)
    }
}

graph Dog from Animal {
    sound: "woof"

    fn fetch() {
        return "fetching"
    }
}
```

---

## Graph Types (DAG, Tree, etc.)

Use parentheses after `graph` keyword for typed graphs:

```graphoid
# Named typed graph
graph TaskGraph(:dag) {
    tasks: []

    fn add_dependency(from, to) {
        add_edge(from, to)
    }
}

# Anonymous typed graph
workflow = graph(:dag) { nodes: [] }
```

---

## Rules

Rules go in the body using `rule` keyword:

```graphoid
graph Tree {
    rule :no_cycles
    rule :single_root

    root: none
}

# Or as a list property
graph DAG {
    rules: [:no_cycles]

    fn connect(a, b) {
        add_edge(a, b)
    }
}
```

---

## Access Control

Use a `configure` block for auto-generating getters/setters:

```graphoid
graph Rectangle {
    configure {
        readable: [:width, :height]    # Generates width() and height() getters
        writable: :color               # Generates set_color(value) setter
        accessible: :size              # Generates both getter and setter
    }

    width: 0
    height: 0
    color: "red"
    size: 0

    fn area() {
        return width * height
    }

    fn perimeter() {
        return 2 * (width + height)
    }

    # Private methods use priv fn, not configure
    priv fn helper() {
        return 42
    }
}
```

**Configure options:**
- `readable: :prop` or `readable: [:prop1, :prop2]` - Generate getter methods
- `writable: :prop` or `writable: [:prop1, :prop2]` - Generate setter methods (`set_prop(value)`)
- `accessible: :prop` - Both readable and writable

**Private methods:**
- Use `priv fn method_name() { }` syntax
- Methods are renamed with underscore prefix internally (`_method_name`)
- Call internally via `_method_name()` or externally blocked

---

## Complete Example

```graphoid
graph Vehicle {
    rules: [:no_cycles]
    readonly :vin

    wheels: 4
    speed: 0
    vin: none
    _engine_on: false

    fn start() {
        _engine_on = true
    }

    fn stop() {
        _engine_on = false
        speed = 0
    }

    fn accelerate(amount) {
        if _engine_on {
            speed = speed + amount
        }
    }
}

graph Car from Vehicle {
    doors: 4

    fn honk() {
        print("beep!")
    }
}

# Create an instance
my_car = Car.clone()
my_car.start()
my_car.accelerate(50)
print(type(my_car))  # "Car"
```

---

## What This Replaces

The old scattered syntax is **replaced**, not supplemented:

| Old (DEPRECATED) | New |
|------------------|-----|
| `Animal = graph {}` | `graph Animal { }` |
| `Animal.x = 1` after declaration | `x: 1` inside body |
| `fn Animal.method() { self.x }` | `fn method() { x }` inside body |
| `Animal.add_rule(:no_cycles)` | `rule :no_cycles` inside body |

The external `fn Graph.method()` syntax goes away. Methods belong inside the graph definition.

---

## Resolution Order

Inside a graph method body, identifiers resolve:

1. Function parameters
2. Local variables (defined with `=` in current scope)
3. Graph properties
4. Graph methods
5. Enclosing scope

---

## Static Methods

Use `static fn` prefix for methods that don't operate on an instance:

```graphoid
graph Counter {
    _total: 0

    static fn count() {
        return _total
    }

    static fn reset_all() {
        _total = 0
    }
}
```

---

## Design Decisions

1. **Named vs Anonymous**: `graph Name { }` declares a type; `x = graph { }` creates an instance
   - This is the critical distinction that enables proper type identity

2. **Separators**: Flexible. Commas, newlines, or mix—doesn't matter while brace is open.
   ```graphoid
   # All valid:
   graph Point { x: 0, y: 0 }
   graph Point { x: 0; y: 0 }
   graph Point {
       x: 0
       y: 0
   }
   ```

3. **Directive syntax**: `directive :symbol` with comma-separated lists allowed.
   ```graphoid
   readable :x, :y, :z
   readonly :id
   private :_helper, :_internal
   ```

4. **Static methods**: Use `static fn` prefix—more natural, reads like English.

5. **Graph type config**: Parentheses after `graph` keyword.
   ```graphoid
   graph TaskGraph(:dag) { }
   workflow = graph(:dag) { }
   ```

---

## Implementation Phases

### Phase 1: Named Graph Declaration ✅ COMPLETE
- Parse `graph Name { prop: value }` as a declaration statement
- Store intrinsic name in the graph value
- Bind to identifier in current scope
- Parse `fn name() { }` inside graph body

### Phase 2: Implicit Self Resolution ✅ COMPLETE
- Inside method bodies, resolve bare identifiers to graph properties
- Maintain resolution order: params → locals → properties → methods → outer scope
- `self` still available explicitly when needed

### Phase 3: Inheritance Syntax ✅ COMPLETE
- Parse `graph Name from Parent { }`
- Inherit properties and methods from parent
- Override support

### Phase 4: Configure Block ✅ COMPLETE (Updated from "Directives")
- Parse `configure { readable: :x, writable: [:y, :z] }`
- `readable: :prop` - generates getter method
- `writable: :prop` - generates `set_prop(value)` setter
- `accessible: :prop` - generates both getter and setter
- Private methods use `priv fn`, NOT configure block

### Phase 5: Static Methods, Graph Types & Rules ✅ COMPLETE
- Parse `static fn name() { }` inside graph body
- Parse `graph Name(:type) { }` for typed graphs
- Parse `rule :name` or `rule :name, param` inside body

### Phase 6: Remove Old Syntax 🔲 NEXT
- Remove support for `fn Graph.method()` external definitions
- Remove support for `get Graph.prop()` external getter definitions
- Remove support for `static fn Graph.method()` external static definitions
- Possibly deprecate `Graph = graph {}` assignment form (keep `x = graph {}` anonymous)
- Update all samples and tests
- Update documentation

---

## Summary

The new unified syntax with intrinsic naming:

```graphoid
graph Animal {
    sound: none

    fn speak() {
        print(sound)
    }
}

graph Dog from Animal {
    sound: "woof"

    fn fetch() {
        return "fetching"
    }
}

graph TaskGraph(:dag) {
    rule :no_cycles
    readonly :id

    id: none
    tasks: []

    static fn create(id) {
        g = clone()
        g.id = id
        return g
    }

    fn add_task(name) {
        add_node(name)
    }
}

# Usage
fido = Dog.clone()
fido.speak()           # "woof"
print(type(fido))      # "Dog"

workflow = TaskGraph.create("build")
print(type(workflow))  # "TaskGraph"
```

**Key insight**: Named graphs are declarations that establish type identity. Anonymous graphs are expressions that create instances. This distinction is fundamental to having a proper type system.

---

## File Conventions

A "class-like graph" (CLG) isn't a special construct—it's just a graph that uses more features. The same `graph Name { }` syntax works whether you're defining a full-featured type or a simple data structure:

```graphoid
# Full-featured CLG
graph Animal {
    sound: none
    fn speak() { print(sound) }
}

# Simple graph type - no methods, just structure
graph Point {
    x: 0
    y: 0
}

# Even simpler - just a typed empty graph
graph EmptyDAG(:dag) {
}

# Data-only with rules
graph Config {
    readonly :version
    version: "1.0"
    settings: {}
}
```

### Naming Convention

The primary convention is **one graph per file, named to match**:

| File | Contains |
|------|----------|
| `animal.gr` | `graph Animal { }` |
| `point.gr` | `graph Point { }` |
| `task_graph.gr` | `graph TaskGraph { }` |
| `method_less.gr` | `graph MethodLess { }` |

This helps the system locate graph definitions by name. When you reference `Animal`, the system can look for `animal.gr`.

### Flexibility

The convention is a guideline, not a requirement:

- **Multiple related graphs in one file**: `shapes.gr` could contain `graph Point { }`, `graph Rectangle { }`, `graph Circle { }`
- **Folder organization**: `models/user.gr` for `graph User { }`
- **Utility collections**: `validators.gr` with several small validation graphs

The system uses filename hints to help locate graphs, but doesn't enforce strict one-to-one mapping. Use your judgment for what makes code readable and maintainable.
