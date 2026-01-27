# Platform Loader

**Status**: Design
**Priority**: 2 (Foundational)
**Dependencies**: Graphoid module system, Platform Runtime

---

## Overview

The Platform Loader **builds the platform graph** from your code structure. Your files become nodes; your imports become edges. No manual graph construction required.

**Key Principles**:
- Files are nodes, imports are edges
- The graph emerges from code structure
- Convention over configuration
- Put code in the right place; it becomes part of the graph automatically

---

## The Graph Emerges From Code

When you create files and imports, the loader builds a graph:

```
app/
├── models/
│   ├── player.gr     # Node: models/player
│   └── room.gr       # Node: models/room
└── lib/
    └── game.gr       # Node: lib/game
```

```graphoid
# lib/game.gr
import "models/player"   # Edge: lib/game → models/player
import "models/room"     # Edge: lib/game → models/room
```

**Result**: A graph you can traverse:

```graphoid
platform.lib.game.depends_on    # → [models/player, models/room]
platform.models.player.dependents    # → [lib/game]
```

---

## Core Responsibilities

| Responsibility | Description |
|----------------|-------------|
| **Discover** | Find `.gr` files in conventional locations |
| **Load** | Parse and execute modules |
| **Register** | Track loaded modules for access and reload |
| **Resolve** | Handle dependencies between modules |
| **Provide** | Make modules available via `import` |

---

## Conventional Locations

```
project/
├── app.gr              # Entry point (loaded first)
├── app/
│   ├── models/         # Data definitions
│   │   ├── player.gr   # → import "models/player"
│   │   └── game.gr     # → import "models/game"
│   └── lib/            # Application code
│       ├── parser.gr   # → import "lib/parser"
│       └── map.gr      # → import "lib/map"
├── config/
│   └── settings.gr     # → Loaded into config service
└── platform/           # Platform extensions (if any)
```

### Import Mapping

| File Location | Import Statement |
|---------------|------------------|
| `app/models/player.gr` | `import "models/player"` |
| `app/lib/parser.gr` | `import "lib/parser"` |
| `app/lib/utils/helpers.gr` | `import "lib/utils/helpers"` |

No need to specify `app/` prefix — it's assumed.

---

## Loading Process

```
┌─────────────────┐
│    Discover     │  Scan conventional directories
└────────┬────────┘
         ▼
┌─────────────────┐
│  Build Graph    │  Determine load order from imports
└────────┬────────┘
         ▼
┌─────────────────┐
│ Load in Order   │  Leaves first, then dependents
└────────┬────────┘
         ▼
┌─────────────────┐
│   Register      │  Add to module registry
└─────────────────┘
```

### Discovery

```graphoid
# Pseudocode for discovery

fn discover(root) {
    modules = []

    # Scan conventional locations
    for file in glob(root + "/app/models/*.gr") {
        modules.append({ path: file, namespace: "models" })
    }

    for file in glob(root + "/app/lib/**/*.gr") {
        modules.append({ path: file, namespace: "lib" })
    }

    return modules
}
```

### Graph Construction

The loader parses `import` statements to build the platform graph:

```graphoid
# app/lib/game.gr
import "models/player"
import "models/room"
import "lib/map"
```

This creates edges in the graph:
- `lib/game` → `models/player` (`:depends_on`)
- `lib/game` → `models/room` (`:depends_on`)
- `lib/game` → `lib/map` (`:depends_on`)

**Load order is graph traversal**: Topological sort of the dependency graph ensures leaves load first.

```graphoid
# The graph determines load order naturally
for module in platform.nodes.sorted_by(:dependencies) {
    load(module)
}
```

### Circular Dependency Handling

```graphoid
# a.gr imports b.gr
# b.gr imports a.gr
```

Options:
1. **Error**: Fail with clear message
2. **Lazy resolution**: Allow forward references, resolve at runtime

For MVP: Error with helpful message indicating the cycle.

---

## The Platform Graph IS the Registry

There's no separate "registry" — the platform graph itself tracks all modules and their relationships:

```graphoid
# The platform IS a graph
platform.nodes                      # → [models/player, models/room, lib/game, ...]
platform.edges                      # → [{from: lib/game, to: models/player, type: :depends_on}, ...]

# Each module is a node with properties
platform.models.player.path         # → "app/models/player.gr"
platform.models.player.loaded_at    # → timestamp
platform.models.player.functions    # → [create, take_damage, is_alive]

# Relationships through graph traversal
platform.models.player.depends_on   # → [models/room]
platform.models.player.dependents   # → [lib/game]

# Query the graph naturally
platform.unhealthy                  # → modules where health != :healthy
platform.recently_loaded            # → modules loaded in last minute
```

### Graph Traversal API

```graphoid
# Impact analysis is graph reachability
platform.models.player.all_dependents    # → transitive closure

# Startup order is topological sort
platform.load_order                       # → [models/room, models/player, lib/game]

# Find modules matching criteria
for module in platform where module.loaded_at > cutoff {
    log(module.name + " was recently loaded")
}
```

---

## Import Resolution

When user code does `import "models/player"`:

1. Check registry — already loaded?
2. If not, resolve path: `app/models/player.gr`
3. Check dependencies — load those first
4. Load and register module
5. Return exports

### Path Resolution Order

```
import "foo"

1. app/models/foo.gr
2. app/lib/foo.gr
3. stdlib/foo.gr       (Graphoid standard library)
4. platform/foo.gr     (Platform modules)
5. Error: module not found
```

### Explicit Namespace

```graphoid
import "models/player"    # Explicitly from models
import "lib/parser"       # Explicitly from lib
import "http"             # From stdlib
import "platform/reload"  # From platform
```

---

## Configuration Loading

`config/settings.gr` is special:

```graphoid
# config/settings.gr

database = {
    host: "localhost",
    port: 5432
}

log = {
    level: :info
}

debug = false
```

Loaded automatically and available via `config` service:

```graphoid
# In user code
host = config.get("database.host")
debug = config.get("debug")
```

### Environment Overrides

```
config/
├── settings.gr           # Base config
├── settings.dev.gr       # Development overrides
└── settings.prod.gr      # Production overrides
```

```bash
GRAPHOID_ENV=prod graphoid run
```

Loader merges: `settings.gr` + `settings.prod.gr`

---

## Hot Reload Support

The loader provides primitives for hot reload:

```graphoid
import "platform/loader"

# Reload a single module
loader.reload("models/player")

# Reload module and its dependents
loader.reload("models/player", { cascade: true })

# Get reload impact
loader.dependents("models/player")  # → ["lib/game", "lib/battle"]
```

### Reload Process

1. Call `on_before_reload()` if defined in module
2. Capture state to preserve
3. Unload module from registry
4. Re-parse and re-execute module file
5. Call `on_after_reload(preserved_state)` if defined
6. Update dependents' references

See [PLATFORM_RELOAD.md](PLATFORM_RELOAD.md) for full details.

---

## Error Handling

### Module Not Found

```
Error: Module not found: 'models/playr'

Did you mean: 'models/player'?

Searched in:
  - app/models/playr.gr
  - app/lib/playr.gr
  - stdlib/playr.gr
```

### Syntax Error in Module

```
Error: Failed to load 'models/player'

Syntax error at line 15:
  fn create(name
            ^
  Expected ')' to close parameter list
```

### Circular Dependency

```
Error: Circular dependency detected

  models/player
    → imports models/room
      → imports models/player  (cycle)

Consider extracting shared code to a separate module.
```

---

## Language Requirements

| Capability | Status | Notes |
|------------|--------|-------|
| File system access | Exists | For discovery |
| Module parsing | Exists | In interpreter |
| Module loading | Exists | Basic `import` works |
| Module unloading | **Needed** | For hot reload |
| Module re-loading | **Needed** | For hot reload |
| Glob patterns | Exists | For discovery |

### Feedback to Graphoid Roadmap

1. **Module unload**: Ability to remove a module from the namespace
2. **Module reload**: Ability to re-parse and re-execute a module
3. **Import introspection**: Get list of what a module imports

---

## Implementation Phases

### Phase 1: Basic Discovery and Loading

- Scan `app/models/` and `app/lib/`
- Load all discovered modules at startup
- Basic import resolution
- Error messages for missing modules

**Milestone**: Auto-discovery works

### Phase 2: Dependency Resolution

- Parse imports to build dependency graph
- Topological sort for load order
- Circular dependency detection
- Clear error messages

**Milestone**: Dependencies load in correct order

### Phase 3: Module Registry

- Track loaded modules
- `loader.modules()`, `loader.info()` API
- Dependency and dependent tracking

**Milestone**: Can query what's loaded

### Phase 4: Configuration Loading

- Auto-load `config/settings.gr`
- `config.get()` API
- Environment-specific overrides

**Milestone**: Configuration works

### Phase 5: Hot Reload Primitives

- `loader.reload()` function
- Module unload/reload capability
- Dependent tracking for cascade reload

**Milestone**: Hot reload possible (full implementation in PLATFORM_RELOAD)

### Phase 6: Package Integration

- Integration with package manager (Phase 21)
- Load from `packages/` directory
- Version resolution

**Milestone**: External packages work

---

## Open Questions

1. **Eager vs lazy loading**: Load everything at startup, or on first import?

2. **Namespace collisions**: What if `models/game.gr` and `lib/game.gr` both exist?

3. **Nested directories**: How deep does auto-discovery go?

4. **Init files**: Should `app/lib/__init__.gr` run when any lib module loads?

5. **Load order guarantees**: Beyond dependency order, is there alphabetical or other ordering?

---

## Related Documents

- [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) — Execution environment
- [PLATFORM_RELOAD.md](PLATFORM_RELOAD.md) — Hot reload mechanism
- [PHASE_21_PACKAGE_MANAGER.md](../PHASE_21_PACKAGE_MANAGER.md) — Package management
