# Phase 17: Modules as Graph

**Duration**: 7-10 days
**Priority**: Critical (Part of graph-centric foundation)
**Dependencies**: Phase 15 (Namespace Graph), Phase 16 (Execution Graph)
**Status**: Planning

---

## Goal

Make the module system fully graph-centric:
- Modules are subgraphs in the universe
- Imports create edges between module graphs
- Exports define public traversal points
- Module resolution is graph traversal

---

## Current Implementation

```rust
// Modules are currently structs with HashMaps
struct Module {
    name: String,
    exports: HashMap<String, Value>,
    path: PathBuf,
}

// Import creates a copy in local namespace
fn import_module(name: &str) -> Result<()> {
    let module = load_module(name)?;
    for (name, value) in module.exports {
        self.environment.define(name, value);
    }
}
```

**Problems**:
1. Modules are isolated structs, not part of universe graph
2. Import copies values instead of creating edges
3. No graph-based module resolution
4. Can't inspect module relationships as graph
5. Circular imports are hard to handle

---

## Target Architecture

### Module as Subgraph

```
┌─────────────────────────────────────────────────────────────────────┐
│  Universe Graph                                                     │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ Module: math                                                │    │
│  │                                                             │    │
│  │  ┌──────────────┐                                           │    │
│  │  │ module:math  │                                           │    │
│  │  └──────┬───────┘                                           │    │
│  │         │                                                   │    │
│  │    ┌────┴────┬────────┬────────┬────────┐                   │    │
│  │    │         │        │        │        │                   │    │
│  │    ▼         ▼        ▼        ▼        ▼                   │    │
│  │ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌────────┐              │    │
│  │ │ sin  │ │ cos  │ │ sqrt │ │ PI   │ │ helper │              │    │
│  │ │public│ │public│ │public│ │public│ │private │              │    │
│  │ └──────┘ └──────┘ └──────┘ └──────┘ └────────┘              │    │
│  │    │                                                        │    │
│  │    └─► (function subgraph)                                  │    │
│  │                                                             │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │ Module: my_app                                              │    │
│  │                                                             │    │
│  │  ┌───────────────┐                                          │    │
│  │  │ module:my_app │                                          │    │
│  │  └───────┬───────┘                                          │    │
│  │          │                                                  │    │
│  │     ┌────┴────┐                                             │    │
│  │     │         │                                             │    │
│  │     ▼         ▼                                             │    │
│  │  ┌──────┐  ┌──────┐                                         │    │
│  │  │ main │  │import│──────────────────────┐                  │    │
│  │  └──────┘  │ math │                      │                  │    │
│  │            └──────┘                      │                  │    │
│  │                                          │                  │    │
│  └──────────────────────────────────────────│──────────────────┘    │
│                                             │                       │
│                                             │ imports               │
│                                             ▼                       │
│                                      module:math                    │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### Import Creates Edge, Not Copy

```graphoid
# In my_app.gr
import "math"
x = math.sin(0.5)
```

**Current behavior**: Copy `sin` function into local namespace.

**Graph behavior**: Create edge from `my_app` to `math.sin`. Lookup traverses edge.

```
my_app:scope ──imports──► module:math
                              │
                              ├──exports──► sin
                              ├──exports──► cos
                              └──exports──► sqrt

# Lookup "math.sin":
1. Find "math" in current scope → finds import edge
2. Traverse import edge to module:math
3. Find "sin" in module's exports
4. Return reference (not copy)
```

### Export Visibility

**Everything is public by default** in modules. Use the `priv` keyword to mark items as private:

```graphoid
# In math.gr
fn sin(x) { ... }          # Public by default
fn cos(x) { ... }          # Public

priv fn helper(x) { ... }  # Private (priv keyword)

# Group multiple private items with priv { } blocks
priv {
    fn internal_calc(x) { ... }
    fn normalize(x) { ... }
    INTERNAL_CONST = 42
}
```

**Privacy Rules:**
- Multiple `priv { }` blocks are allowed in a module
- Nesting `priv { }` blocks generates a warning (semantically meaningless)
- Both `priv` and `private` keywords work (synonyms)

```
Module graph structure:
  module:math
      │
      ├──exports──► sin           (traversable from outside)
      ├──exports──► cos           (traversable from outside)
      ├──contains──► helper       (NOT exported, internal only)
      ├──contains──► internal_calc (NOT exported, internal only)
      └──contains──► normalize    (NOT exported, internal only)
```

### Selective Import

```graphoid
import "math" { sin, cos }     # Only import specific items
import "math" as m             # Alias
import "math" { sin as sine }  # Rename
```

```
Import edges:
  my_scope ──imports:sin──► math:sin
  my_scope ──imports:cos──► math:cos

With alias:
  my_scope ──alias:m──► module:math

With rename:
  my_scope ──imports:sine──► math:sin
```

### Circular Import Handling

Graph structure naturally handles circular imports:

```
Module A imports Module B
Module B imports Module A

Graph:
  module:A ──imports──► module:B
  module:B ──imports──► module:A

Resolution:
1. Start loading A
2. A needs B, start loading B
3. B needs A, but A already exists (partially) in graph
4. B gets edge to A (even though A not fully loaded)
5. Finish loading B
6. Finish loading A
7. Both modules now fully connected
```

No copying = no infinite loop. Edges to partially-loaded modules just work.

---

## Module Resolution as Graph Traversal

### Resolution Graph

```
┌─────────────────────────────────────────────────────────────────────┐
│  Module Resolution Graph                                            │
│                                                                     │
│  ┌──────────────┐                                                   │
│  │ search:paths │                                                   │
│  └──────┬───────┘                                                   │
│         │                                                           │
│    ┌────┴────┬────────────┬────────────┐                            │
│    ▼         ▼            ▼            ▼                            │
│ ┌──────┐ ┌────────┐ ┌──────────┐ ┌──────────┐                       │
│ │stdlib│ │project/│ │ vendor/ │ │ ~/.gr/  │                        │
│ │      │ │  lib/  │ │         │ │ modules │                        │
│ └──┬───┘ └───┬────┘ └────┬────┘ └────┬────┘                         │
│    │         │           │           │                              │
│    ▼         ▼           ▼           ▼                              │
│  (stdlib   (local      (vendored   (global                          │
│   modules)  modules)    deps)       installs)                       │
│                                                                     │
│  import "math" resolution:                                          │
│  1. Check stdlib → found → return stdlib/math.gr                    │
│                                                                     │
│  import "mylib" resolution:                                         │
│  1. Check stdlib → not found                                        │
│  2. Check project/lib → found → return project/lib/mylib.gr         │
└─────────────────────────────────────────────────────────────────────┘
```

### Dependency Graph

```graphoid
# Package dependencies form a graph
dependencies = module.dependency_graph()

# Visualize
dependencies.to_dot()  # Export as GraphViz

# Analyze
cycles = dependencies.find_cycles()
if cycles.length() > 0 {
    warn("Circular dependencies detected: " + cycles.to_string())
}

# Topological sort for load order
load_order = dependencies.topological_sort()
```

---

## Data Structures

```rust
// src/modules/graph_module.rs

pub struct ModuleGraph {
    /// The universe graph containing all modules
    universe: Arc<RwLock<Graph>>,

    /// Module registry: name → node_id
    modules: HashMap<String, NodeId>,

    /// Resolution paths
    search_paths: Vec<PathBuf>,

    /// Currently loading (for cycle detection)
    loading: HashSet<String>,
}

pub struct ModuleNode {
    pub name: String,
    pub path: PathBuf,
    pub scope: ScopeId,  // Module's namespace subgraph
    pub exports: Vec<String>,
    pub state: ModuleState,
}

pub enum ModuleState {
    Loading,
    Loaded,
    Failed(String),
}

pub enum ImportEdge {
    /// import "math" - full module
    Full { alias: Option<String> },

    /// import "math" { sin, cos }
    Selective { items: Vec<ImportItem> },
}

pub struct ImportItem {
    pub name: String,
    pub alias: Option<String>,
}
```

---

## Core Operations

```rust
impl ModuleGraph {
    /// Load a module (or get existing)
    pub fn load_module(&mut self, name: &str) -> Result<NodeId> {
        // Check if already loaded
        if let Some(node_id) = self.modules.get(name) {
            return Ok(*node_id);
        }

        // Check for circular load
        if self.loading.contains(name) {
            // Not an error - return partial module
            return self.get_partial_module(name);
        }

        self.loading.insert(name.to_string());

        // Resolve path
        let path = self.resolve_path(name)?;

        // Create module node
        let module_id = self.universe.write().add_node(
            format!("module:{}", name),
            Value::Module(ModuleNode {
                name: name.to_string(),
                path: path.clone(),
                scope: ScopeId::new(),
                exports: vec![],
                state: ModuleState::Loading,
            })
        );

        self.modules.insert(name.to_string(), module_id);

        // Parse and execute module
        let source = std::fs::read_to_string(&path)?;
        let ast = parse(&source)?;

        // Execute in module's scope
        let exports = self.execute_module(module_id, ast)?;

        // Mark as loaded
        self.update_module_state(module_id, ModuleState::Loaded);
        self.loading.remove(name);

        Ok(module_id)
    }

    /// Create import edge
    pub fn import(
        &mut self,
        from_scope: ScopeId,
        module_name: &str,
        import_type: ImportEdge,
    ) -> Result<()> {
        let module_id = self.load_module(module_name)?;

        match import_type {
            ImportEdge::Full { alias } => {
                // Create edge from scope to module
                let edge_name = alias.unwrap_or(module_name.to_string());
                self.universe.write().add_edge(
                    from_scope.to_string(),
                    module_id.to_string(),
                    format!("imports:{}", edge_name),
                    None,
                    HashMap::new(),
                );
            }
            ImportEdge::Selective { items } => {
                // Create edge for each imported item
                for item in items {
                    let export_id = self.get_export(module_id, &item.name)?;
                    let local_name = item.alias.unwrap_or(item.name);
                    self.universe.write().add_edge(
                        from_scope.to_string(),
                        export_id.to_string(),
                        format!("imports:{}", local_name),
                        None,
                        HashMap::new(),
                    );
                }
            }
        }

        Ok(())
    }

    /// Resolve module.item lookup
    pub fn resolve(&self, scope: ScopeId, path: &[&str]) -> Result<NodeId> {
        let mut current = scope.to_string();

        for (i, segment) in path.iter().enumerate() {
            // Look for import edge with this name
            let edges = self.universe.read().edges_from(&current);

            let import_edge = edges.iter().find(|e| {
                e.label.starts_with("imports:") &&
                e.label.ends_with(segment)
            });

            if let Some(edge) = import_edge {
                current = edge.to.clone();
            } else if i == 0 {
                // Not an import, try local lookup
                return Err(format!("'{}' not found", segment));
            } else {
                // Looking for export in module
                let export_edge = edges.iter().find(|e| {
                    e.label == format!("exports:{}", segment)
                });

                if let Some(edge) = export_edge {
                    current = edge.to.clone();
                } else {
                    return Err(format!("'{}' not exported from module", segment));
                }
            }
        }

        Ok(NodeId::from_string(&current))
    }
}
```

---

## Module Introspection

```graphoid
import "math"

# Inspect module as graph
math_graph = reflect.module("math")

# List exports
math_graph.exports()  # ["sin", "cos", "sqrt", "PI", ...]

# Check dependencies
math_graph.imports()  # ["constants"] (if math imports constants)

# Full dependency tree
math_graph.dependency_tree()

# Visualize
math_graph.to_dot() | save("math_deps.dot")
```

### Debug Module Graph

```graphoid
# See all loaded modules
modules = reflect.loaded_modules()
for m in modules {
    print(m.name + " from " + m.path)
    print("  exports: " + m.exports().join(", "))
    print("  imports: " + m.imports().join(", "))
}

# Dependency analysis
dep_graph = reflect.module_graph()
print("Modules: " + dep_graph.nodes().length())
print("Dependencies: " + dep_graph.edges().length())

cycles = dep_graph.find_cycles()
if cycles.length() > 0 {
    print("Warning: circular dependencies")
    for cycle in cycles {
        print("  " + cycle.join(" -> "))
    }
}
```

---

## Implementation Plan

### Day 1-2: Module Graph Structure

- [ ] Create `ModuleGraph` struct
- [ ] Define `ModuleNode`, `ImportEdge` types
- [ ] Integrate with universe graph
- [ ] Unit tests for structure

### Day 3-4: Module Loading

- [ ] Implement `load_module()` with graph creation
- [ ] Handle circular imports via partial modules
- [ ] Module state tracking
- [ ] Path resolution as graph traversal

### Day 5-6: Import Edge Creation

- [ ] Full module import (`import "math"`)
- [ ] Selective import (`import "math" { sin }`)
- [ ] Aliased import (`import "math" as m`)
- [ ] Renamed items (`import "math" { sin as sine }`)

### Day 7-8: Lookup Resolution

- [ ] Implement `resolve()` for `module.item` paths
- [ ] Handle nested modules
- [ ] Export visibility enforcement
- [ ] Error messages for missing exports

### Day 9: Privacy Blocks

**Note**: Implement `priv { }` blocks for modules here. The same parser/semantic changes apply to packages in Phase 21.

- [ ] Parse `priv { ... }` block syntax
- [ ] Mark all items in block as private
- [ ] Allow multiple `priv { }` blocks per module
- [ ] Emit warning for nested `priv { }` blocks
- [ ] Unit tests for privacy blocks

### Day 10: Integration & Testing

- [ ] Integrate with executor
- [ ] Run all existing module tests
- [ ] Add graph-specific module tests
- [ ] Documentation

---

## Success Criteria

- [ ] All existing module tests pass
- [ ] Modules are nodes in universe graph
- [ ] Imports create edges, not copies
- [ ] Circular imports work correctly
- [ ] Selective imports work
- [ ] Module introspection via `reflect.module()`
- [ ] Dependency graph analysis works
- [ ] Performance: import ≤1.5x current overhead
- [ ] `priv { }` blocks work in modules (shared implementation with packages)

---

## Test Cases

```graphoid
# test_module_graph.gr

# Module exists in universe
import "math"
universe = reflect.universe()
assert(universe.has_node("module:math"), "math module in universe")

# Import creates edge
my_scope = reflect.current_scope()
edges = my_scope.edges({ label_prefix: "imports:" })
assert(edges.any(e => e.to == "module:math"), "import edge exists")

# Selective import
import "math" { sin, cos }
assert(my_scope.has_edge({ to: "math:sin" }), "sin imported")
assert(my_scope.has_edge({ to: "math:cos" }), "cos imported")
assert(not my_scope.has_edge({ to: "math:sqrt" }), "sqrt not imported")

# Circular import (should not crash)
# Assuming a.gr imports b.gr and b.gr imports a.gr
import "test_circular_a"
assert(true, "circular import succeeded")
```

---

## Related Documents

- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Namespace foundation
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Execution foundation
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Remaining concepts
- [PHASE_21_PACKAGE_MANAGER.md](PHASE_21_PACKAGE_MANAGER.md) - Package-level modules
