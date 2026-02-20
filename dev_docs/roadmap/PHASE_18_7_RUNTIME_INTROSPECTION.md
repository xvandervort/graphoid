# Phase 18.7: Runtime Introspection

**Status**: Complete (February 20, 2026)
**Priority**: **High**
**Dependencies**: None
**Estimated Duration**: 2-3 days

---

## Overview

Add runtime introspection features that require no concurrency. These are simple query/inspection APIs that work in a single-threaded context.

---

## Features

### 1. Module Listing and Info

Query the module registry for loaded modules.

```graphoid
import "runtime/modules"

# List all loaded modules
mods = modules.list()  # → ["math", "json", "http", ...]

# Get module info
info = modules.info("math")
# → { path: "stdlib/math.gr", type: "stdlib" }
```

**Implementation**: Read from `ModuleManager` registry. No mutation, no concurrency needed.

---

### 2. Runtime Information

```graphoid
import "runtime"

# Runtime version
runtime.version()  # → "0.1.0"

# Uptime in seconds since interpreter start
runtime.uptime()  # → 3600

# Memory usage (from Rust allocator)
mem = runtime.memory()
# → { used: 45000000, peak: 52000000 }

# Loaded module count
runtime.module_count()  # → 12
```

**Implementation**: Version is compiled in. Uptime tracked from `Instant::now()` at startup. Memory from system allocator stats.

---

### 3. Stack Trace Access

Capture call stack information on exceptions.

```graphoid
try {
    risky_operation()
} catch as e {
    print("Error: " + e.message())
    for frame in e.stack() {
        print("  at " + frame["function"] + " in " + frame["file"] + ":" + frame["line"].to_string())
    }
}

# Stack frame structure:
# { "function": "process_data", "file": "app/lib/processor.gr", "line": 42 }
```

**Implementation**: Capture stack at `raise` time by walking the call stack in `GraphExecutor`. Store as list of maps on the exception value.

---

### 4. Current Module Name

```graphoid
# In app/models/player.gr
print(__MODULE__)  # → "models/player"
```

**Implementation**: `__MODULE__` is set as a variable in the execution environment when a module is loaded. Value is the module's import path.

---

## API Summary

| Module | Functions |
|--------|-----------|
| `runtime/modules` | `list()`, `info(name)` |
| `runtime` | `version()`, `uptime()`, `memory()`, `module_count()` |
| Error object | `.stack()` method |
| Global | `__MODULE__` variable |

---

## Test Specification

```graphoid
describe "runtime" {
    it "returns version string" {
        v = runtime.version()
        expect(typeof(v)).to_equal("string")
    }

    it "returns uptime" {
        t = runtime.uptime()
        expect(t).to_be_gte(0)
    }
}

describe "modules" {
    it "lists loaded modules" {
        mods = modules.list()
        expect(typeof(mods)).to_equal("list")
    }
}

describe "error.stack" {
    it "captures stack trace" {
        trace = none
        try {
            raise("test error")
        } catch as e {
            trace = e.stack()
        }
        expect(typeof(trace)).to_equal("list")
    }
}
```

---

## Success Criteria

- [x] `modules.list()` and `modules.info()` implemented
- [x] `runtime.version()`, `runtime.uptime()`, `runtime.memory()`, `runtime.module_count()` implemented
- [x] `error.stack()` captures call stack at raise time
- [x] `__MODULE__` available in module scope
- [x] All features tested in gspec (18 tests)
- [x] Sample file demonstrating features

---

## Related Documents

- [PHASE_18_6_SERVER_CAPABILITIES.md](PHASE_18_6_SERVER_CAPABILITIES.md) — Previous phase
- [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) — Next phase (concurrency adds `modules.reload/unload`)
