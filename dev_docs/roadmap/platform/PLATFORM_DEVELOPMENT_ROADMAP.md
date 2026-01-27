# Platform Development Roadmap

**Status**: Planning
**Purpose**: Clear development path with realistic expectations

---

## Overview

This document defines the development roadmap for the Graphoid Platform. The platform's component priorities are fixed by logical necessity. Where language features are missing, **the language roadmap should be adjusted** to unblock platform development.

**Principles**:
- Platform needs drive language priorities, not the reverse
- **The platform IS a graph** — files are nodes, imports are edges
- Design leverages Graphoid's graph-centric nature throughout

---

## Graph-Centric Architecture

The platform embodies Graphoid's core philosophy: **everything is a graph**.

| Platform Concept | Graph Concept |
|-----------------|---------------|
| Module discovery | Build the graph from files |
| Import statements | Create edges between nodes |
| Dependency order | Topological sort |
| Hot reload | Subgraph replacement |
| Impact analysis | Reachability query |
| Health monitoring | Graph introspection |
| Shutdown order | Reverse topological sort |

This isn't a metaphor — the platform literally operates on a graph that emerges from your code structure. See [GRAPHOID_PLATFORM.md](GRAPHOID_PLATFORM.md) for details.

---

## Current Graphoid Capabilities

As of January 2026, Graphoid provides:

| Capability | Status | Notes |
|------------|--------|-------|
| Module system | ✅ Yes | Import, load modules |
| File I/O | ✅ Yes | Read/write files |
| Graph rules | ✅ Yes | Validation, constraints |
| Graph behaviors | ✅ Yes | Attached behaviors |
| Try/catch | ✅ Yes | Error handling |
| JSON encode/decode | ✅ Yes | Serialization |
| Glob patterns | ✅ Yes | File discovery |
| gspec testing | ✅ Yes | Test framework |

---

## Missing Capabilities (Require Language Work)

| Capability | Needed By | Roadmap Phase |
|------------|-----------|---------------|
| Timers | Runtime, Monitor | Not yet planned |
| Signal handling (SIGINT, etc.) | Runtime | Not yet planned |
| Module unload | Loader, Reload | Not yet planned |
| Module reload | Reload | Not yet planned |
| File watching | Reload | Not yet planned |
| Memory introspection | Monitor | Not yet planned |
| Stack trace access | Logger | Not yet planned |
| Current module name | Logger | Not yet planned |
| Concurrency (spawn, channels) | Process | Phase 19 |
| Isolated execution contexts | Process | Not yet planned |

---

## Platform Component Priorities (Fixed)

These priorities are determined by logical dependency, not by what's currently buildable:

| Priority | Component | Depends On | Blocks |
|----------|-----------|------------|--------|
| **1** | Runtime | Language core | Everything |
| **2** | Loader | Runtime | Reload, Monitor |
| **3** | Logger | Runtime | (standalone utility) |
| **4** | Reload | Loader | Development workflow |
| **5** | Monitor | Runtime, Loader | Production observability |
| **6** | Model | Graph rules | Application development |
| **7** | Process | Concurrency | Advanced patterns |

---

## Language Features Required (By Platform Priority)

### For Runtime (Priority 1)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| Basic module loading | Critical | ✅ Exists |
| File I/O | Critical | ✅ Exists |
| Try/catch | Critical | ✅ Exists |
| **Timers** | Critical | ❌ **NEEDED** |
| **Signal handling** | Critical | ❌ **NEEDED** |
| **Event loop / async** | High | ❌ Phase 19 |

### For Loader (Priority 2)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| Glob patterns | Critical | ✅ Exists |
| Module import | Critical | ✅ Exists |
| **Module unload** | Critical | ❌ **NEEDED** |
| **Module reload** | Critical | ❌ **NEEDED** |

### For Logger (Priority 3)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| File I/O | Critical | ✅ Exists |
| String formatting | Critical | ✅ Exists |
| JSON encoding | Critical | ✅ Exists |
| **Stack trace access** | Medium | ❌ **NEEDED** |
| **Current module name** | Medium | ❌ **NEEDED** |

### For Reload (Priority 4)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| Module unload | Critical | ❌ See Loader |
| Module reload | Critical | ❌ See Loader |
| **File watching** | Critical | ❌ **NEEDED** |

### For Monitor (Priority 5)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| Timers (periodic) | Critical | ❌ See Runtime |
| **Memory introspection** | High | ❌ **NEEDED** |
| **Module introspection** | Medium | ❌ **NEEDED** |

### For Model (Priority 6)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| Graph rules | Critical | ✅ Exists |
| Graph behaviors | Critical | ✅ Exists |
| JSON encoding | Critical | ✅ Exists |
| Database access | High | ❌ Phase 22 |

### For Process (Priority 7)

| Feature | Importance | Current Status |
|---------|------------|----------------|
| Spawn/channels | Critical | ❌ Phase 19 |
| Isolated contexts | Critical | ❌ **NEEDED** |

---

## Development Phases

Development proceeds in platform priority order. Language features are added as needed to unblock each phase.

### Phase 1: Minimal Runtime + Loader

**Platform components**: Runtime v0.1, Loader v0.1

**What we build**:
- Find and execute app.gr
- Call main() function
- Auto-discover modules in app/
- Load modules in dependency order
- Basic error handling

**Language features needed**: None (use existing capabilities)

**User can**:
- Create a project with conventional structure
- Run `graphoid run` to start their app
- Imports work from app/models/, app/lib/

---

### Phase 2: Logger + Config

**Platform components**: Logger v0.1, Runtime config support

**What we build**:
- `log()` function available globally
- Log levels, file output, console output
- Load config/settings.gr
- `config.get()` API

**Language features needed**: None (use existing file I/O)

**User can**:
- Log without any setup
- Configure application via settings.gr

---

### Phase 3: Full Runtime (Requires Language Work)

**Platform components**: Runtime v1.0

**What we build**:
- Event loop with timer support
- Lifecycle hooks (on_start, on_stop)
- Graceful shutdown on signals

**Language features needed**:
- `timer.after(ms, fn)`
- `timer.every(ms, fn)`
- `signal.on(:sigint, fn)`

**User can**:
- Run periodic tasks
- Handle shutdown gracefully
- Build event-driven applications

---

### Phase 4: Hot Reload (Requires Language Work)

**Platform components**: Loader v1.0, Reload v1.0

**What we build**:
- Module unload and reload
- State preservation across reload
- File watching for auto-reload
- Watch mode for development

**Language features needed**:
- `__unload_module__(name)`
- `__reload_module__(name)`
- `fs.watch(path, fn)`

**User can**:
- Reload code without restarting
- Preserve application state
- Use watch mode for rapid iteration

---

### Phase 5: Monitor

**Platform components**: Monitor v1.0

**What we build**:
- Health checks
- Metrics collection
- Alerting
- Status dashboard

**Language features needed**:
- `runtime.memory()` (for memory stats)
- Timers (from Phase 3)

**User can**:
- Monitor application health
- Track custom metrics
- Get alerts on conditions

---

### Phase 6: Model

**Platform components**: Model v1.0

**What we build**:
- Attribute definitions
- Validation (using graph rules)
- Callbacks
- Serialization

**Language features needed**: None (uses existing graph rules)

**User can**:
- Define data models
- Validate automatically
- Serialize to JSON

---

### Phase 7: Model Persistence (Requires Phase 22)

**Platform components**: Model v2.0

**What we build**:
- Database persistence
- Query builder
- Associations
- Migrations runner

**Language features needed**: Phase 22 database connectivity

**User can**:
- Save/load models from database
- Query with SQL-like syntax
- Define relationships

---

### Phase 8: Process (Requires Phase 19)

**Platform components**: Process v1.0

**What we build**:
- Process spawning
- Channels for communication
- Supervision trees
- Resource limits

**Language features needed**: Phase 19 concurrency

**User can**:
- Run concurrent tasks
- Build fault-tolerant systems
- Use worker pools

---

## Milestones

| Milestone | Phase | Description | Language Required |
|-----------|-------|-------------|-------------------|
| **M1: Hello Platform** | 1 | Run app.gr with auto-discovery | None |
| **M2: Logging Works** | 2 | log() available everywhere | None |
| **M3: Event Loop** | 3 | Timers, graceful shutdown | Timers, signals |
| **M4: Hot Reload** | 4 | Reload without restart | Module reload, fs.watch |
| **M5: Observable** | 5 | Health checks, metrics | Memory introspection |
| **M6: Data Models** | 6 | Validation, serialization | None |
| **M7: Persistence** | 7 | Database-backed models | Phase 22 |
| **M8: Concurrent** | 8 | Processes, supervision | Phase 19 |

---

## Proposed Language Roadmap Changes

The platform requires language features that should be prioritized ahead of or alongside existing roadmap phases.

### Immediate Priority (Blocks Runtime, Loader, Reload)

These features block the top 4 platform components and should be added to the language as soon as possible:

| Feature | API | Blocks | Proposed Phase |
|---------|-----|--------|----------------|
| **Timers** | `timer.after(ms, fn)`, `timer.every(ms, fn)` | Runtime event loop | **New: Pre-19** |
| **Module unload** | `__unload_module__(name)` | Loader, Reload | **New: Pre-19** |
| **Module reload** | `__reload_module__(name)` | Reload | **New: Pre-19** |
| **File watching** | `fs.watch(path, fn)` | Reload watch mode | **New: Pre-19** |
| **Signal handling** | `signal.on(:sigint, fn)` | Runtime shutdown | **New: Pre-19** |

### High Priority (Blocks Features)

| Feature | API | Blocks | Proposed Phase |
|---------|-----|--------|----------------|
| Stack trace access | `error.stack()` | Logger error details | **New: Pre-19** |
| Current module name | `__MODULE__` | Logger context | **New: Pre-19** |
| Memory introspection | `runtime.memory()` | Monitor memory stats | **New: Pre-19** |

### Already Planned (Confirm Priority)

| Feature | Blocks | Current Phase |
|---------|--------|---------------|
| Concurrency (spawn, channels) | Process component | Phase 19 |
| Database connectivity | Model persistence | Phase 22 |

### Added: Phase 18.5 Platform Support

**[Phase 18.5: Platform Support](../PHASE_18_5_PLATFORM_SUPPORT.md)** has been added to the language roadmap:

- Timers (`timer.after`, `timer.every`, `timer.cancel`, `timer.sleep`)
- Signal handling (`signal.on`)
- Module management (`modules.list`, `modules.reload`, `modules.unload`)
- File watching (`fs.watch`)
- Stack trace access (`error.stack()`)
- Current module (`__MODULE__`)
- Runtime introspection (`runtime.memory()`, `runtime.uptime()`)

**Duration**: 5-7 days
**Dependencies**: None (can start immediately)

This phase unblocks platform development without waiting for graph-centric foundation or concurrency.

---

## Getting Started

### Immediate Actions (No Language Work Required)

1. **Create platform/ directory structure**
2. **Implement Runtime v0.1** — find and run app.gr
3. **Implement Loader v0.1** — auto-discover modules
4. **Implement Logger v0.1** — basic logging

### First Working Platform (Milestone M2)

With Phases 1-2 complete, users can:

```graphoid
# app.gr
import "lib/game"

fn main() {
    log("Game starting")
    game.run()
    log("Game ended")
}
```

```bash
graphoid run
# → [2026-01-23 12:00:00] INFO Game starting
# → ...game runs...
# → [2026-01-23 12:05:00] INFO Game ended
```

### Next Priority: Language Features for Phase 3

To proceed beyond M2, the language needs:
- Timers
- Signal handling

These should be prioritized in the Graphoid roadmap.

---

## Open Questions

1. **Command name**: Is `graphoid run` correct, or should it be `gr run`?

2. **Platform distribution**: Is platform bundled with Graphoid or separate package?

3. **Version coupling**: Does platform version track Graphoid version?

4. **Backward compatibility**: What's the stability promise for platform API?

---

## Related Documents

- [GRAPHOID_PLATFORM.md](GRAPHOID_PLATFORM.md) — Platform overview
- [PLATFORM_COMPONENT_STRUCTURE.md](PLATFORM_COMPONENT_STRUCTURE.md) — Component structure
- [PHASE_19_CONCURRENCY.md](../PHASE_19_CONCURRENCY.md) — Concurrency features
- [PHASE_22_DATABASE.md](../PHASE_22_DATABASE.md) — Database connectivity
