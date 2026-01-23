# Platform Runtime

**Status**: Design
**Priority**: 1 (Foundational)
**Dependencies**: Graphoid interpreter, Phase 19 Concurrency (partial)

---

## Overview

The Platform Runtime is the execution environment for Graphoid applications. **Execution is graph traversal** — the runtime walks the platform graph to start, run, and manage your application.

**Key Principles**:
- The platform IS a graph (files are nodes, imports are edges)
- Execution follows graph structure
- Lifecycle management through graph operations
- You write Graphoid code; the platform handles everything else

---

## Architecture

The runtime operates on a graph that emerges from your code structure:

```
┌─────────────────────────────────────────────┐
│     Platform Graph (emerges from code)      │
│                                             │
│   app.gr ──→ lib/game ──→ models/player     │
│                 │              │            │
│                 └────→ models/room ←────┘   │
├─────────────────────────────────────────────┤
│             Platform Runtime                │
│   Traverses the graph to: load, execute,    │
│   monitor, reload, and shutdown             │
├─────────────────────────────────────────────┤
│            Graphoid Interpreter             │
└─────────────────────────────────────────────┘
```

**Everything is a graph operation**:
- **Startup**: Topological sort of dependency edges
- **Execution**: Walk from entry point through function calls
- **Shutdown**: Reverse topological order
- **Reload**: Subgraph replacement

---

## Starting the Platform

### Command Line

```bash
graphoid run              # Run app.gr in current directory
graphoid run game.gr      # Run specific entry point
graphoid run --reload     # Run with hot reload enabled
```

### Programmatic

```graphoid
import "platform"

platform.run()            # Run app.gr
platform.run("game.gr")   # Run specific entry point
```

---

## Application Entry Point

The runtime looks for an entry point in this order:

1. `app.gr` with `main()` function
2. `app.gr` with top-level code
3. Specified file via `graphoid run <file>`

### Simple Application

```graphoid
# app.gr

fn main() {
    print("Hello, Graphoid!")
}
```

### Continuous Application

```graphoid
# app.gr

fn main() {
    while true {
        input = io.readline("> ")
        if input == "quit" {
            break
        }
        process(input)
    }
}
```

### Event-Driven Application

```graphoid
# app.gr

fn main() {
    platform.on(:input, handle_input)
    platform.on(:timer, handle_tick)
    platform.loop()  # Run until terminated
}

fn handle_input(data) {
    # ...
}

fn handle_tick() {
    # ...
}
```

---

## Platform Services

Services available to all user code without explicit import:

| Service | Access | Description |
|---------|--------|-------------|
| Logging | `log()`, `log.info()`, etc. | Always-on logging |
| Config | `config.get("key")` | Application configuration |
| Platform | `platform.reload()`, etc. | Platform control |

### Logging (Built-in)

```graphoid
# Available everywhere, no import needed
log("Simple message")
log.debug("Detailed info")
log.info("Normal operation")
log.warn("Something unusual")
log.error("Something failed", error)
```

### Configuration

```graphoid
# Reads from config/settings.gr
db_host = config.get("database.host")
debug_mode = config.get("debug", false)  # With default
```

### Platform Control (Graph Operations)

```graphoid
# The platform IS the graph — control it naturally
platform.reload("models/player")    # Subgraph replacement
platform.status()                   # Graph introspection

# Shutdown follows reverse dependency order
platform.shutdown()                 # Traverse leaves-to-roots

# Query the running platform
platform.nodes                      # All loaded modules
platform.models.player.dependents   # Who uses this module?
platform.unhealthy                  # Modules with problems
```

---

## Lifecycle

```
┌─────────────┐
│   Start     │
└──────┬──────┘
       ▼
┌─────────────┐
│ Initialize  │  Load config, start logger, init services
└──────┬──────┘
       ▼
┌─────────────┐
│    Load     │  Discover and load user code (via Loader)
└──────┬──────┘
       ▼
┌─────────────┐
│    Run      │  Call main() or enter event loop
└──────┬──────┘
       ▼
┌─────────────┐
│  Shutdown   │  Cleanup, flush logs, exit
└─────────────┘
```

### Lifecycle Hooks

User code can hook into lifecycle events:

```graphoid
# app.gr

fn on_platform_start() {
    log.info("Application starting")
}

fn on_platform_shutdown() {
    log.info("Application shutting down")
    save_state()
}

fn main() {
    # ...
}
```

---

## Event Loop

For applications that need to handle multiple event sources:

```graphoid
fn main() {
    # Register handlers
    platform.on(:stdin, fn(line) {
        process_command(line)
    })

    platform.every(1.second, fn() {
        update_display()
    })

    # Run the loop
    platform.loop()
}
```

### Event Types

| Event | Description |
|-------|-------------|
| `:stdin` | User input from terminal |
| `:timer` | Scheduled timer fired |
| `:signal` | OS signal (SIGINT, etc.) |
| `:reload` | Module reload completed |
| `:message` | Inter-component message (future) |

---

## Project Structure

The runtime expects this structure by convention:

```
project/
├── app.gr              # Entry point
├── app/
│   ├── models/         # Data definitions
│   └── lib/            # Application code
├── config/
│   └── settings.gr     # Configuration
├── db/
│   └── migrations/     # Schema evolution
├── log/                # Log output (created automatically)
└── tests/
    └── spec/           # Tests
```

### Auto-Discovery

The runtime (via Loader) automatically discovers:

| Path | Loaded As |
|------|-----------|
| `app/models/*.gr` | `import "models/name"` |
| `app/lib/*.gr` | `import "lib/name"` |
| `config/settings.gr` | Available via `config` |

---

## Error Handling

### Application Errors

Uncaught errors in user code:

```graphoid
fn main() {
    result = risky_operation()  # Throws an error
}
# Runtime catches it, logs it, exits with status 1
```

### Graceful Error Handling

```graphoid
fn main() {
    while true {
        try {
            process_next()
        } catch error {
            log.error("Failed to process", error)
            # Continue running
        }
    }
}
```

### Platform Errors

If platform itself fails:
- Log the error
- Attempt graceful shutdown
- Exit with error status

---

## Language Requirements

The runtime needs capabilities from Graphoid core:

| Capability | Status | Notes |
|------------|--------|-------|
| Module loading | Exists | Used by Loader |
| Try/catch | Exists | Error handling |
| File I/O | Exists | Logging, config |
| Timers | **Needed** | For `platform.every()` |
| Signal handling | **Needed** | For graceful shutdown |
| Event loop | **Needed** | For `platform.loop()` |
| Module reload | **Needed** | For hot reload |

### Feedback to Graphoid Roadmap

These should be added to the language:

1. **Timer primitives**: `timer.after(duration, callback)`, `timer.every(interval, callback)`
2. **Signal handling**: `signal.on(:sigint, callback)`
3. **Event loop**: Core event dispatch mechanism (may be part of Phase 19)
4. **Module reload**: `__reload_module__(name)` intrinsic

---

## Implementation Phases

### Phase 1: Basic Execution

- Find and run `app.gr`
- Call `main()` if present
- Basic error handling
- Exit when main returns

**Milestone**: Can run Hunt the Wumpus

### Phase 2: Logging

- Built-in `log()` function
- Log levels (debug, info, warn, error)
- Write to `log/app.log`
- Console output in development

**Milestone**: Logs visible without setup

### Phase 3: Configuration

- Load `config/settings.gr`
- `config.get()` API
- Environment-specific overrides

**Milestone**: Configurable applications

### Phase 4: Lifecycle Hooks

- `on_platform_start()` / `on_platform_shutdown()`
- Graceful shutdown on SIGINT
- Cleanup handling

**Milestone**: Clean application lifecycle

### Phase 5: Event Loop

- `platform.on(event, handler)`
- `platform.every(interval, handler)`
- `platform.loop()`
- Timer support

**Milestone**: Event-driven applications

### Phase 6: Integration with Reload

- Coordinate with hot reload system
- Preserve event handlers across reload
- `:reload` event for user code

**Milestone**: Hot reload works with running applications

---

## Open Questions

1. **Single-threaded or concurrent?** Does Phase 1 runtime assume single-threaded? When does concurrency come in?

2. **Event loop design**: Custom implementation or leverage Phase 19 async?

3. **Global services**: Is `log()` truly global (no import) or `platform.log()`?

4. **REPL integration**: Can you attach a REPL to a running platform for debugging?

---

## Related Documents

- [PLATFORM_LOADER.md](PLATFORM_LOADER.md) — How code gets loaded
- [PLATFORM_LOGGER.md](PLATFORM_LOGGER.md) — Logging infrastructure
- [PLATFORM_RELOAD.md](PLATFORM_RELOAD.md) — Hot reload mechanism
- [PHASE_19_CONCURRENCY.md](../PHASE_19_CONCURRENCY.md) — Concurrency primitives
