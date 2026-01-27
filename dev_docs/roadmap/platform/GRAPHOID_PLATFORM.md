# Graphoid Platform

**Status**: Separate Project (Not Part of Numbered Roadmap)
**Priority**: High (after core language stabilizes)
**Dependencies**: Package Manager (Phase 21), Concurrency (Phase 19)

---

## Overview

The **Graphoid Platform** is a pure Graphoid framework that extends what Graphoid programs can do. It provides runtime management capabilities for building robust, manageable applications.

**Key Principles**:
- Written entirely in pure Graphoid
- For Graphoid developers
- Minimal boilerplate — modules ARE components
- Capability gaps feed back to the language roadmap

---

## Core Components

Components are ordered by logical priority. See [PLATFORM_DEVELOPMENT_ROADMAP.md](PLATFORM_DEVELOPMENT_ROADMAP.md) for development phases and language requirements.

| Priority | Component | Document | Purpose | Language Needs |
|----------|-----------|----------|---------|----------------|
| **1** | Runtime | [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) | Execution environment, main loop | Timers, signals |
| **2** | Loader | [PLATFORM_LOADER.md](PLATFORM_LOADER.md) | Auto-discovery, module loading | Module reload |
| **3** | Logger | [PLATFORM_LOGGER.md](PLATFORM_LOGGER.md) | Always-on logging | Stack traces |
| **4** | Reload | [PLATFORM_RELOAD.md](PLATFORM_RELOAD.md) | Hot reload mechanism | fs.watch |
| **5** | Monitor | [PLATFORM_MONITOR.md](PLATFORM_MONITOR.md) | Health, metrics, alerting | Memory introspection |
| **6** | Model | [PLATFORM_MODEL.md](PLATFORM_MODEL.md) | Data, validation, persistence | Phase 22 DB |
| **7** | Process | [PLATFORM_PROCESS.md](PLATFORM_PROCESS.md) | Isolation, supervision | Phase 19 concurrency |

**Component Structure**: See [PLATFORM_COMPONENT_STRUCTURE.md](PLATFORM_COMPONENT_STRUCTURE.md)

**Development Approach**: Build components in priority order. Language features are added as needed to unblock each component. See [PLATFORM_DEVELOPMENT_ROADMAP.md](PLATFORM_DEVELOPMENT_ROADMAP.md) for the proposed language additions.

---

## Running User Code

The platform's primary job is to run user programs. Put your code in the conventional locations; the platform loads and runs it.

### Example: Text Adventure Game

```graphoid
# app.gr — Entry point

import "models/player"
import "models/room"
import "lib/map"
import "lib/parser"
import "lib/menu"
import "lib/stats"

fn main() {
    while true {
        choice = menu.show_main()

        if choice == :play {
            world = map.generate()
            p = player.create("Hero")
            result = run_game(world, p)
            stats.record(result)
        } else if choice == :stats {
            stats.display()
        } else if choice == :quit {
            break
        }
    }
}
```

### Running It

```bash
graphoid run
```

The platform:
1. Discovers all modules in `app/`
2. Loads them in dependency order
3. Calls `main()` in `app.gr`
4. Keeps running until `main()` returns or user quits

### Growing the Application

Because it's running on the platform, you can later add:
- **Database**: `import "database"` — persist games and stats
- **Networking**: `import "network"` — add multiplayer
- **Hot reload**: Fix bugs without restarting

The structure doesn't change. You just import more capabilities.

---

## Modules Are Nodes

A platform component is just a regular Graphoid module. No special syntax, no registration.

### Defining a Component

```graphoid
# app/monitor.gr — just a regular module

metrics = {}
alerts = []

fn check_health(target) {
    # implementation
}

fn get_metrics() {
    return metrics
}
```

That's it. Put it in `app/`, it becomes a node.

### Running the Platform

```graphoid
import "platform"

platform.run()   # Discover graph from app/, run it
```

Or from command line:
```bash
graphoid run
```

### Platform API

```graphoid
platform.run()              # Discover and run
platform.stop()             # Shutdown
platform.reload("monitor")  # Replace a node

# Everything else is graph traversal
platform.nodes              # All components
platform.monitor            # Access a specific node
platform.monitor.dependents # Who depends on monitor?
```

Minimal surface area. The graph provides the rest.

---

## Configuration

Defaults are sensible. Only specify what differs from defaults.

### Multiple Instances

```graphoid
app = platform.app([
    "monitor",
    { name: "worker", instances: 4 }
])
```

### Restart Behavior

```graphoid
app = platform.app([
    "monitor",
    { name: "worker", restart: :on_failure }
])
```

### Available Options

| Option | Default | Description |
|--------|---------|-------------|
| `instances` | 1 | Number of concurrent instances |
| `restart` | `:never` | Restart policy: `:never`, `:on_failure`, `:always` |
| `timeout` | `none` | Kill if running longer than specified |
| `depends_on` | `[]` | Components that must start first |

---

## Lifecycle Hooks

Components can optionally define hooks for custom behavior:

```graphoid
# monitor.gr

metrics = {}

fn check_health(target) { ... }

# Optional: called before reload, return state to preserve
fn on_before_reload() {
    return { metrics: metrics }
}

# Optional: called after reload with preserved state
fn on_after_reload(preserved) {
    metrics = preserved.metrics
}

# Optional: called on startup
fn on_start() {
    log("Monitor starting")
}

# Optional: called on shutdown
fn on_stop() {
    log("Monitor stopping")
}
```

If hooks aren't defined, the platform uses sensible defaults.

---

## Models and Migrations

Like Rails, the platform provides conventional locations for data definitions and schema evolution.

### Models (app/models/)

Data definitions with structure, validation, and methods:

```graphoid
# app/models/player.gr

fn create(name) {
    p = graph { type: :player }
    p.name = name
    p.health = 100
    p.position = [0, 0]
    p.add_rule("health_range", 0, 100)
    return p
}

fn player.take_damage(p, amount) {
    p.health = p.health - amount
}

fn player.is_alive(p) {
    return p.health > 0
}
```

Models leverage Graphoid's graph rules for validation — no separate validation layer needed.

### Migrations (db/migrations/)

Schema changes over time, numbered for ordering:

```graphoid
# db/migrations/001_create_games.gr

fn up(db) {
    db.create_table("games", {
        id: :auto,
        player_name: :string,
        result: :string,
        score: :integer,
        played_at: :timestamp
    })
}

fn down(db) {
    db.drop_table("games")
}
```

Run migrations:

```bash
graphoid migrate        # Run pending migrations
graphoid migrate:down   # Rollback last
graphoid migrate:status # Show state
```

### Model Component

Full model support (validations, callbacks, querying, persistence) is a long-term project documented in PLATFORM_MODEL.md (to be created).

---

## Platform IS a Graph

The platform doesn't just "use" graphs internally — **the platform IS a graph** that emerges naturally from your code:

### The Graph Emerges From Code

**Files are nodes:**
```
app/
├── monitor.gr     # node: monitor
├── worker.gr      # node: worker
└── logger.gr      # node: logger
```

**Imports are edges:**
```graphoid
# app/worker.gr
import "monitor"   # edge: worker -> monitor (:depends_on)
import "logger"    # edge: worker -> logger (:depends_on)
```

**No manual graph construction.** The platform discovers the graph from your code structure.

### Graph Operations Are Natural

```graphoid
import "platform"

# The platform IS the graph
platform.nodes              # → [monitor, worker, logger]
platform.edges              # → [{worker, monitor, :depends_on}, ...]

# Traverse relationships through property access
worker.depends_on           # → [monitor, logger]
monitor.dependents          # → [worker]

# Query by state
platform.unhealthy          # → [database] (nodes where health != :healthy)
platform.running            # → [monitor, worker, logger]

# Pattern matching on the graph
for node in platform where node.health == :degraded {
    node.restart()
}
```

### Configuration as Node Properties

```graphoid
# config/settings.gr

# These become properties on platform nodes
database.host = "localhost"
database.port = 5432

worker.instances = 4
worker.restart = :on_failure
```

Access naturally:
```graphoid
platform.database.host      # → "localhost"
platform.worker.instances   # → 4
```

### Why This Matters

Because the platform IS a graph:
- **Dependency analysis** is graph traversal
- **Impact analysis** is reachability queries
- **Health propagation** follows edges
- **Startup order** is topological sort
- **Hot reload** is subgraph replacement
- **Monitoring** is graph introspection
- **Supervision** is hierarchical graph structure
- **Model relationships** are edges

The graph isn't a metaphor — it's the actual runtime structure.

### Graph-Centric Components

Each platform component embodies the graph-centric philosophy:

| Component | Graph Concept |
|-----------|---------------|
| **Loader** | Builds the graph from files/imports |
| **Runtime** | Execution is graph traversal |
| **Reload** | Hot reload is subgraph replacement |
| **Monitor** | Monitoring is graph introspection |
| **Model** | Models ARE graphs, relationships ARE edges |
| **Process** | Supervision trees ARE graphs |
| **Logger** | Module context from graph position |

---

## Language Requirements

The platform requires capabilities that Graphoid may not yet have. These feed back to the language roadmap:

### Required from Phase 19 (Concurrency)

| Capability | Purpose |
|------------|---------|
| Spawn concurrent tasks | Run multiple component instances |
| Crash propagation | Detect when a component fails |
| Message passing / channels | Communication between components |

### Required: Module Reload Primitive

The platform needs an internal capability to reload a module at runtime. This should be:
- Hidden from normal users (not a public API)
- Available to the platform library
- Handles: unload old module, re-parse file, load new module

**Proposed**: A `__reload_module__(name)` intrinsic that only platform internals use.

### Required: Isolated Execution Contexts

For true process isolation, the platform needs:
- Separate namespaces per component instance
- No shared mutable state between instances
- Communication only via explicit channels

This may require runtime support beyond Phase 19 concurrency.

---

## Dependency: Package Manager

The `platform/loader` module depends on the Package Manager (Phase 21).

For initial development:
- Design the loader interface assuming packages exist
- Use simple module loading as a stand-in
- Full functionality when Phase 21 completes

---

## Project Structure (Convention)

The platform expects user projects to follow this structure:

```
my_project/
├── app.gr              # Entry point (main function)
├── app/
│   ├── models/         # Data definitions
│   │   ├── player.gr   # → import "models/player"
│   │   └── game.gr     # → import "models/game"
│   └── lib/            # Application code
│       ├── parser.gr   # → import "lib/parser"
│       └── map.gr      # → import "lib/map"
├── config/
│   └── settings.gr     # Configuration
├── db/
│   └── migrations/     # Schema evolution
├── log/                # Log output (created automatically)
│   ├── app.log
│   └── platform.log
└── tests/
    └── spec/           # gspec tests
```

## Platform Source Structure

```
platform/
├── runtime.gr      # Execution environment, lifecycle
├── loader.gr       # Discovery, loading, registry
├── logger.gr       # Logging infrastructure
├── reload.gr       # Hot reload system
├── monitor.gr      # Health and metrics
├── model.gr        # Data modeling support
└── config.gr       # Configuration handling
```

---

## Implementation Phases

### Phase A: Foundation
- `platform.app()` creation
- `app.start()` and `app.stop()`
- Basic component loading
- Status reporting

### Phase B: Hot Reload
- `app.reload()` implementation
- Lifecycle hooks (`on_before_reload`, `on_after_reload`)
- State preservation across reloads

### Phase C: Process Management
- Multiple instances per component
- Restart policies
- Crash detection and handling
- Requires: Phase 19 concurrency

### Phase D: Monitoring
- Health checks
- Metrics collection
- Alerting
- Status dashboard (optional)

---

## Prior Work

Extensive prior thinking exists in `dev_docs/archive/sims/platform/`. This work explored advanced use cases (space probes, medical devices, autonomous systems) and remains valuable reference material for future expansion:

| Document | Focus |
|----------|-------|
| `GRAPHOID_PLATFORM_ARCHITECTURE.md` | Self-healing architecture |
| `PLATFORM_FOUNDATION.md` | Process management, hot reloading |
| `SELF_HEALING_ARCHITECTURE.md` | Diagnostic and healing engines |
| `SELF_HEALING_USE_CASES.md` | Application scenarios |
| `ETHICS_MODULE_AND_CONFIGURATION.md` | Ethical decision frameworks |

These advanced capabilities (self-healing, ethics, hardware integration) can be built as optional packages on top of the core platform once the foundation is solid.

---

## Open Questions

1. **Namespace isolation**: How do we isolate component instances from each other?
2. **Channel design**: How do components communicate?
3. **Reload atomicity**: How do we handle in-flight operations during reload?
4. **Dependency ordering**: How does the platform determine start/stop order?

---

## Related Documents

### Platform Planning
- [PLATFORM_DEVELOPMENT_ROADMAP.md](PLATFORM_DEVELOPMENT_ROADMAP.md) — What to build when
- [PLATFORM_COMPONENT_STRUCTURE.md](PLATFORM_COMPONENT_STRUCTURE.md) — How components are organized

### Platform Components
- [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) — Execution environment, main loop
- [PLATFORM_LOADER.md](PLATFORM_LOADER.md) — Auto-discovery, module loading
- [PLATFORM_LOGGER.md](PLATFORM_LOGGER.md) — Always-on logging
- [PLATFORM_RELOAD.md](PLATFORM_RELOAD.md) — Hot reload mechanism
- [PLATFORM_MONITOR.md](PLATFORM_MONITOR.md) — Health, metrics, alerting
- [PLATFORM_MODEL.md](PLATFORM_MODEL.md) — Data modeling, validation, persistence
- [PLATFORM_PROCESS.md](PLATFORM_PROCESS.md) — Isolation, supervision, concurrency

### Language Dependencies
- [PHASE_19_CONCURRENCY.md](../PHASE_19_CONCURRENCY.md) — Concurrency primitives
- [PHASE_21_PACKAGE_MANAGER.md](../PHASE_21_PACKAGE_MANAGER.md) — Package management

### Prior Work (Archived)
- [dev_docs/archive/sims/platform/](../../archive/sims/platform/) — Advanced features (self-healing, ethics)
