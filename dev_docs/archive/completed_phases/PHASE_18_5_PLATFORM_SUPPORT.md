# Phase 18.5: Platform Support Features

> **SUPERSEDED**: This phase was split in February 2026. Concurrency-dependent features (timers, signals, file watching, module hot reload) moved to Phase 19 sub-phases. Non-concurrency features (runtime introspection, module listing, error.stack, __MODULE__) moved to Phase 18.7. Server capabilities remain in Phase 18.6.

**Status**: Proposed
**Priority**: **CRITICAL** (Blocks Platform Development)
**Dependencies**: None (can start immediately)
**Estimated Duration**: 5-7 days

---

## Overview

This phase adds language features required by the Graphoid Platform. These are fundamental capabilities that enable the platform to provide runtime management, hot reload, and observability.

**Rationale**: The Graphoid Platform is a pure-Graphoid framework for building robust applications. It cannot be built without these language features. Platform development should not be blocked waiting for full concurrency (Phase 19).

---

## Features

### 1. Timers

Enable time-based callbacks for event loops and periodic tasks.

```graphoid
import "timer"

# One-shot timer
handle = timer.after(1000, fn() {
    log("1 second elapsed")
})

# Recurring timer
handle = timer.every(5000, fn() {
    log("Every 5 seconds")
})

# Cancel a timer
timer.cancel(handle)

# Sleep (blocking)
timer.sleep(500)  # Sleep 500ms
```

**Implementation Notes**:
- Use Rust's `tokio::time` or `std::thread::sleep` internally
- Return a handle that can be cancelled
- Callbacks execute in order when multiple timers fire

---

### 2. Signal Handling

Enable graceful shutdown and process control.

```graphoid
import "signal"

# Register signal handler
signal.on(:sigint, fn() {
    log("Received SIGINT, shutting down...")
    cleanup()
    exit(0)
})

signal.on(:sigterm, fn() {
    log("Received SIGTERM")
    graceful_shutdown()
})

# Available signals
# :sigint  - Ctrl+C
# :sigterm - Termination request
# :sighup  - Hangup (optional)
```

**Implementation Notes**:
- Use Rust's `signal-hook` or `ctrlc` crate
- Allow multiple handlers per signal
- Handlers run in registration order

---

### 3. Module Unload/Reload

Enable hot reloading of modules at runtime.

```graphoid
import "runtime/modules"

# List loaded modules
modules.list()  # → ["models/player", "lib/game", ...]

# Check if module is loaded
modules.loaded?("models/player")  # → true

# Unload a module (removes from namespace)
modules.unload("models/player")

# Reload a module (unload + re-parse + re-execute)
modules.reload("models/player")

# Get module info
modules.info("models/player")
# → { path: "app/models/player.gr", loaded_at: timestamp }
```

**Implementation Notes**:
- `unload` removes module from namespace registry
- `reload` re-reads file, re-parses, re-executes
- Existing references to old module functions remain (caller's responsibility)
- Internal primitive: `__unload_module__`, `__reload_module__`

---

### 4. File System Watching

Enable automatic reload on file changes.

```graphoid
import "fs"

# Watch a file for changes
watcher = fs.watch("app/models/player.gr", fn(event) {
    log("File changed: " + event.path)
    log("Event type: " + event.type)  # :modified, :created, :deleted
})

# Watch a directory
watcher = fs.watch("app/", fn(event) {
    if event.path.ends_with(".gr") {
        modules.reload(path_to_module(event.path))
    }
}, { recursive: true })

# Stop watching
watcher.stop()
```

**Implementation Notes**:
- Use Rust's `notify` crate
- Debounce rapid changes (configurable, default 100ms)
- Support recursive directory watching

---

### 5. Stack Trace Access

Enable detailed error logging.

```graphoid
try {
    risky_operation()
} catch error {
    log.error("Operation failed")
    log.error("Message: " + error.message)
    log.error("Stack trace:")
    for frame in error.stack() {
        log.error("  at " + frame.function + " in " + frame.file + ":" + frame.line)
    }
}

# Stack frame structure
# {
#     function: "process_data",
#     file: "app/lib/processor.gr",
#     line: 42
# }
```

**Implementation Notes**:
- Capture stack at error creation time
- Store as list of frame maps
- Available via `error.stack()` method

---

### 6. Current Module Name

Enable contextual logging.

```graphoid
# In app/models/player.gr

log(__MODULE__)  # → "models/player"

fn create(name) {
    log(__MODULE__ + ": Creating player " + name)
}
```

**Implementation Notes**:
- `__MODULE__` is a compile-time constant
- Value is the module's import path (not file path)
- Available in all code within the module

---

### 7. Runtime Introspection

Enable monitoring and debugging.

```graphoid
import "runtime"

# Memory usage
mem = runtime.memory()
# → { used: 45000000, peak: 52000000, limit: 100000000 }

# Loaded modules count
runtime.module_count()  # → 12

# Runtime version
runtime.version()  # → "0.1.0"

# Uptime
runtime.uptime()  # → 3600 (seconds)
```

**Implementation Notes**:
- Memory stats from Rust's allocator
- Module count from module registry
- Uptime tracked from interpreter start

---

## API Summary

| Module | Functions |
|--------|-----------|
| `timer` | `after(ms, fn)`, `every(ms, fn)`, `cancel(handle)`, `sleep(ms)` |
| `signal` | `on(signal, fn)` |
| `runtime/modules` | `list()`, `loaded?(name)`, `unload(name)`, `reload(name)`, `info(name)` |
| `fs` | `watch(path, fn, options)` (extends existing fs module) |
| `runtime` | `memory()`, `module_count()`, `version()`, `uptime()` |
| Error object | `.stack()` method |
| Global | `__MODULE__` constant |

---

## Test Specification

### Timer Tests

```graphoid
describe "timer" {
    it "fires after specified delay" {
        fired = false
        timer.after(100, fn() { fired = true })
        timer.sleep(150)
        expect(fired).to_equal(true)
    }

    it "can be cancelled" {
        fired = false
        handle = timer.after(100, fn() { fired = true })
        timer.cancel(handle)
        timer.sleep(150)
        expect(fired).to_equal(false)
    }

    it "fires repeatedly with every" {
        count = 0
        handle = timer.every(50, fn() { count = count + 1 })
        timer.sleep(175)
        timer.cancel(handle)
        expect(count).to_be_gte(3)
    }
}
```

### Module Tests

```graphoid
describe "runtime/modules" {
    it "lists loaded modules" {
        mods = modules.list()
        expect(mods).to_include("runtime/modules")
    }

    it "reloads a module" {
        # Assumes test module exists
        result = modules.reload("test/fixtures/reloadable")
        expect(result).to_equal(true)
    }
}
```

### Signal Tests

```graphoid
describe "signal" {
    it "registers a handler" {
        # Can't easily test signal delivery in unit tests
        # Just verify registration doesn't error
        signal.on(:sigint, fn() { })
        expect(true).to_equal(true)
    }
}
```

---

## Implementation Plan

### Day 1-2: Timers

1. Add `timer` native module
2. Implement `after`, `every`, `cancel`, `sleep`
3. Write tests
4. Create sample file

### Day 2-3: Signals

1. Add `signal` native module
2. Implement `on` with handler registration
3. Wire up Rust signal handling
4. Write tests

### Day 3-4: Module Management

1. Add `runtime/modules` native module
2. Implement `list`, `loaded?`, `info`
3. Implement `unload` (remove from registry)
4. Implement `reload` (unload + re-parse + execute)
5. Write tests

### Day 4-5: File Watching

1. Add `fs.watch` to fs module
2. Implement watcher using `notify` crate
3. Add debouncing
4. Write tests

### Day 5-6: Introspection

1. Add `error.stack()` method
2. Capture stack trace on error creation
3. Add `__MODULE__` constant
4. Add `runtime.memory()`, etc.
5. Write tests

### Day 6-7: Integration & Polish

1. Integration tests
2. Sample files demonstrating all features
3. Documentation
4. Edge case handling

---

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Timer accuracy on busy system | Document as approximate; use monotonic clock |
| Signal handler reentrancy | Queue signals, process one at a time |
| Module reload breaking references | Document behavior; provide lifecycle hooks |
| File watcher resource usage | Limit watched paths; provide explicit cleanup |

---

## Success Criteria

- [ ] All features implemented and tested
- [ ] Sample files demonstrating each feature
- [ ] Platform can use these features to build Runtime v1.0
- [ ] No regressions in existing functionality

---

## Related Documents

- [GRAPHOID_PLATFORM.md](platform/GRAPHOID_PLATFORM.md) — Platform overview
- [PLATFORM_DEVELOPMENT_ROADMAP.md](platform/PLATFORM_DEVELOPMENT_ROADMAP.md) — What this unblocks
- [PLATFORM_RUNTIME.md](platform/PLATFORM_RUNTIME.md) — Uses timers, signals
- [PLATFORM_RELOAD.md](platform/PLATFORM_RELOAD.md) — Uses module reload, file watching
- [PLATFORM_LOGGER.md](platform/PLATFORM_LOGGER.md) — Uses stack traces, __MODULE__
- [PLATFORM_MONITOR.md](platform/PLATFORM_MONITOR.md) — Uses runtime introspection
