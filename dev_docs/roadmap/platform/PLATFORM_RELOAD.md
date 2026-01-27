# Platform Reload

**Status**: Design
**Priority**: 4 (Core Platform Promise)
**Dependencies**: Platform Loader, Module reload primitive

---

## Overview

Hot reload is **subgraph replacement**: swap out a node in the platform graph while the application runs. The graph structure makes this natural — edges tell us what's affected.

**Key Principles**:
- Reload is subgraph replacement
- Edges reveal impact (dependents need updating)
- Preserve state across the swap
- Fail safely — bad reload doesn't crash the app

---

## Basic Usage

### Command Line

```bash
# In another terminal while app is running
graphoid reload models/player
graphoid reload lib/game
graphoid reload --all  # Reload everything (careful!)
```

### From Code

```graphoid
import "platform"

platform.reload("models/player")
platform.reload("lib/game")
```

### Automatic (Watch Mode)

```bash
graphoid run --watch
```

Platform watches for file changes and reloads automatically.

---

## What Gets Reloaded

| Change | Reloaded? | Notes |
|--------|-----------|-------|
| Function body | Yes | New code runs on next call |
| New function | Yes | Available immediately |
| Removed function | Yes | Calls will error |
| Module-level variable | Preserved | Unless `on_after_reload` changes it |
| Imported modules | No | Must reload explicitly or use cascade |

### Cascade Reload (Graph Traversal)

Reload a module and everything that depends on it — just follow the edges:

```graphoid
platform.reload("models/player", { cascade: true })
# Follows dependent edges to reload:
#   models/player → lib/game → lib/battle
```

The graph structure makes impact analysis trivial:

```graphoid
# What will be affected?
platform.models.player.all_dependents    # → [lib/game, lib/battle]

# Reload order is reverse topological sort of affected subgraph
platform.reload_order("models/player")   # → [models/player, lib/game, lib/battle]
```

---

## State Preservation

### Automatic (Default)

Module-level variables are preserved across reload:

```graphoid
# models/player.gr

players = {}  # This survives reload

fn create(name) {
    p = graph { type: :player }
    p.name = name
    players[name] = p
    return p
}
```

After reload, `players` still contains all created players.

### Explicit Control

Use lifecycle hooks for custom state handling:

```graphoid
# lib/game.gr

state = {
    current_room: none,
    score: 0,
    history: []
}

fn on_before_reload() {
    # Return state to preserve
    return {
        score: state.score,
        # Don't preserve history — let it reset
    }
}

fn on_after_reload(preserved) {
    # Restore preserved state
    state.score = preserved.score
    state.current_room = none  # Reset room
    state.history = []         # Fresh history

    log("Game module reloaded, score preserved")
}
```

---

## Reload Process

```
┌─────────────────────┐
│   Reload Request    │
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  Validate Module    │  Does it exist? Any syntax errors?
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│ on_before_reload()  │  Capture state to preserve
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   Unload Module     │  Remove from registry
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   Parse New Code    │  Read and parse updated file
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   Load Module       │  Execute and register
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│ on_after_reload()   │  Restore preserved state
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│  Update References  │  Point dependents to new module
└──────────┬──────────┘
           ▼
┌─────────────────────┐
│   Notify Reload     │  Fire :reload event
└─────────────────────┘
```

---

## Handling Reload Events

Application code can respond to reloads:

```graphoid
# app.gr

fn main() {
    platform.on(:reload, fn(module_name) {
        log("Module reloaded: " + module_name)

        if module_name == "models/player" {
            refresh_player_display()
        }
    })

    platform.loop()
}
```

---

## Error Handling

### Syntax Error in New Code

```
Reload failed: models/player

Syntax error at line 23:
    fn create(name
              ^
    Expected ')' to close parameter list

Module unchanged. Previous version still running.
```

The old code continues to run. Fix the error and try again.

### Runtime Error During Reload

If `on_after_reload` throws:

```
Reload failed: lib/game

Error in on_after_reload:
    KeyError: 'score' not found in preserved state

Rolling back to previous version.
```

Platform rolls back to previous module version.

### Rollback Mechanism

```graphoid
# Conceptual rollback process

fn reload_with_rollback(module_name) {
    # Save current state
    backup = registry.snapshot(module_name)

    try {
        perform_reload(module_name)
    } catch error {
        log.error("Reload failed, rolling back", error)
        registry.restore(module_name, backup)
        raise ReloadError(module_name, error)
    }
}
```

---

## Watch Mode

### How It Works

```bash
graphoid run --watch
```

1. Platform starts file watcher on project directory
2. On file change, determine affected module
3. Reload module (with cascade if configured)
4. Report result to console

### Watch Configuration

```graphoid
# config/settings.gr

watch = {
    paths: ["app/"],           # What to watch
    ignore: ["*.tmp", ".git"], # What to skip
    cascade: false,            # Auto-cascade on change?
    debounce: 100              # ms to wait for multiple saves
}
```

### Console Output

```
[watch] Detected change: app/models/player.gr
[watch] Reloading models/player...
[watch] ✓ Reloaded in 23ms

[watch] Detected change: app/lib/game.gr
[watch] Reloading lib/game...
[watch] ✗ Syntax error at line 45 (see log/platform.log)
```

---

## In-Flight Operations

What happens to code currently executing when reload occurs?

### Function Calls

```graphoid
# Old version running
fn process(data) {
    step1(data)       # Old code
    # <-- reload happens here
    step2(data)       # Still old code (same invocation)
}

# Next call uses new version
process(new_data)     # New code
```

Current invocations complete with old code. New invocations use new code.

### Long-Running Operations

For operations that shouldn't be interrupted:

```graphoid
fn critical_operation() {
    platform.pause_reload()  # Block reloads

    try {
        # Do critical work
        save_to_database(data)
    } finally {
        platform.resume_reload()  # Allow reloads again
    }
}
```

---

## Reload Scope

### What Can Be Reloaded

| Item | Reloadable | Notes |
|------|------------|-------|
| User modules (`app/`) | Yes | Primary use case |
| Config (`config/settings.gr`) | Yes | Triggers config refresh |
| Platform modules | Limited | Some internals can reload |
| Graphoid stdlib | No | Requires restart |

### What Cannot Change

Some changes require restart:

- Adding new language features
- Changing platform core behavior
- Modifying native (Rust) code
- Changing module load order fundamentally

---

## Performance

### Reload Speed Target

| Project Size | Target Reload Time |
|--------------|-------------------|
| Small (< 20 modules) | < 50ms |
| Medium (20-100 modules) | < 200ms |
| Large (100+ modules) | < 500ms |

### Optimization Strategies

1. **Incremental parsing**: Only re-parse changed files
2. **Lazy cascade**: Only reload dependents when they're next used
3. **Parallel reload**: Reload independent modules in parallel

---

## Language Requirements

| Capability | Status | Notes |
|------------|--------|-------|
| Module unload | **Needed** | Remove module from namespace |
| Module reload | **Needed** | Re-parse and re-execute |
| File watching | **Needed** | Detect file changes |
| Reference update | **Needed** | Point imports to new module |

### Feedback to Graphoid Roadmap

1. **Module unload primitive**: `__unload_module__(name)`
2. **Module reload primitive**: `__reload_module__(name)`
3. **File watcher**: `fs.watch(path, callback)` or similar
4. **Import reference update**: Mechanism to update all references to a module

---

## Implementation Phases

### Phase 1: Basic Reload

- `platform.reload(module_name)` function
- Unload and reload single module
- Basic error handling (syntax errors)

**Milestone**: Can reload a module manually

### Phase 2: State Preservation

- `on_before_reload` / `on_after_reload` hooks
- Automatic state preservation for module variables
- Rollback on reload failure

**Milestone**: State survives reload

### Phase 3: Cascade Reload

- Track module dependencies
- `cascade: true` option
- Reload dependents in correct order

**Milestone**: Reload with dependencies

### Phase 4: Watch Mode

- File system watcher
- Automatic reload on change
- Debouncing for rapid saves
- Console feedback

**Milestone**: `graphoid run --watch` works

### Phase 5: Production Hardening

- Reload pause/resume for critical sections
- Parallel reload for performance
- Comprehensive error recovery

**Milestone**: Production-ready hot reload

---

## Open Questions

1. **Closure behavior**: What happens to closures that captured old function references?

2. **Timer/event handlers**: How to update handlers registered with old code?

3. **Partial reload**: Can we reload just one function, not whole module?

4. **Version tracking**: Should we track module versions for debugging?

---

## Related Documents

- [PLATFORM_LOADER.md](PLATFORM_LOADER.md) — Module loading and registry
- [PLATFORM_RUNTIME.md](PLATFORM_RUNTIME.md) — Lifecycle and events
- [PLATFORM_LOGGER.md](PLATFORM_LOGGER.md) — Reload logging
