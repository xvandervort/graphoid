# Graphoid Implementation Roadmap

**Version**: 9.2
**Last Updated**: January 28, 2026
**Status**: Phases 0-15 Complete, Phase 16 (Execution Graph) In Progress

---

## Completed Phases (0-14)

All core language features are complete and working:

| Phase | Name | Tests | Status |
|-------|------|-------|--------|
| 0 | Project Setup | - | ✅ Complete |
| 1 | Lexer | 54+ | ✅ Complete |
| 2 | Parser & AST | 446+ | ✅ Complete |
| 3 | Value System & Basic Execution | 133+ | ✅ Complete |
| 4 | Functions & Lambdas | 521+ | ✅ Complete |
| 5 | Collections & Methods | - | ✅ Complete |
| 6 | Graph Types & Rules | - | ✅ Complete |
| 6.5 | Foundational Gaps | 132+ | ✅ Complete |
| 7 | Function Pattern Matching | 186+ | ✅ Complete |
| 8 | Behavior System | - | ✅ Complete |
| 9 | Graph Pattern Matching | 69+ | ✅ Complete |
| 10 | Module System | 40+ | ✅ Complete |
| 11 | Pure Graphoid Stdlib | 24+ | ✅ Complete |
| 12 | Native Stdlib Modules | - | ✅ Complete |
| 13 | Bitwise Operators | - | ✅ Complete |
| 13.5 | Exception Handling | - | ✅ Complete |
| 14 | gspec Testing Framework | 621+ | ✅ Complete |

**Total**: 2,400+ Rust tests, 621+ gspec tests

---

## Remaining Phases (15-33)

### Graph-Centric Foundation (CRITICAL - Must Complete First)

The claim "everything is a graph" is currently FALSE at two levels:
- **Namespace**: Variables stored in HashMap, not graph nodes
- **Runtime**: Traditional tree-walking interpreter, not graph traversal

These phases fix this fundamental architectural gap.

| Phase | Name | Priority | Duration | Dependencies | Status |
|-------|------|----------|----------|--------------|--------|
| [15](PHASE_15_NAMESPACE_GRAPH.md) | Namespace as Graph | **BLOCKER** | 7-10 days | None | ✅ Complete |
| [16](PHASE_16_EXECUTION_GRAPH.md) | Execution as Graph | **BLOCKER** | 14-21 days | Phase 15 | 🔄 Next |
| [17](PHASE_17_MODULES_GRAPH.md) | Modules as Graph | **BLOCKER** | 7-10 days | Phase 15, 16 | |
| [18](PHASE_18_COMPLETE_GRAPH_MODEL.md) | Complete Graph Model | **BLOCKER** | 10-14 days | Phase 15, 16 | |

**Total Graph Foundation**: 38-55 days

### Platform Support (Planned After Graph Foundation)

These features are required by the Graphoid Platform. They will be implemented **in sequence after completing the graph-centric foundation** (Phases 16-18).

| Phase | Name | Priority | Duration | Dependencies | Status |
|-------|------|----------|----------|--------------|--------|
| [18.5](PHASE_18_5_PLATFORM_SUPPORT.md) | Platform Support | **CRITICAL** | 5-7 days | None | 📋 Planned |
| [18.6](PHASE_18_6_SERVER_CAPABILITIES.md) | Server Capabilities | **CRITICAL** | 3-5 days | Phase 18.5 | 📋 Planned |

### Unlocks
Once Phase 18.6 is complete, development can begin on **GraphWeb**, a Sinatra-like web framework.
See: [PLAN_WEB_FRAMEWORK.md](PLAN_WEB_FRAMEWORK.md)

**Features**: Timers, signal handling, module reload, file watching, stack traces, runtime introspection.

**Execution Plan**: Complete graph-centric phases (16-18) first, then implement 18.5 → 18.6 in sequence.

**See**: [GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md](GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md) for detailed justification.

### Concurrency & Parallelism

Built on graph-centric foundation: actors ARE nodes, channels ARE edges.

| Phase | Name | Priority | Duration | Dependencies |
|-------|------|----------|----------|--------------|
| [19](PHASE_19_CONCURRENCY.md) | Concurrency & Parallelism | **Critical** | 14-18 days | Phase 15, 16 |

### Ecosystem & Interop

| Phase | Name | Priority | Duration | Dependencies |
|-------|------|----------|----------|--------------|
| [20](PHASE_20_FFI.md) | Foreign Function Interface | **Critical** | 12-16 days | None |
| [21](PHASE_21_PACKAGE_MANAGER.md) | Package Manager | **High** | 14-21 days | None |
| [22](PHASE_22_DATABASE.md) | Database Connectivity | **High** | 7-10 days | Phase 20, 21 |

### Distributed Computing

| Phase | Name | Priority | Duration | Dependencies |
|-------|------|----------|----------|--------------|
| [23](PHASE_23_DISTRIBUTED_PRIMITIVES.md) | Distribution Primitives | **High** | 12-16 days | Phase 15, 20 |
| [24](PHASE_24_DISTRIBUTED_EXECUTION.md) | Distributed Execution | **High** | 24-30 days | Phase 23 |
| [25](PHASE_25_VECTOR_SEARCH.md) | Vector Search & HNSW | **High** | 14-18 days | Phase 19, 23 |

**Note**: Phase 23 provides language-level primitives (serialization, remote refs, routing hooks) that enable multiple distribution models. The actual distributed computing patterns (Pregel, Actors, MapReduce, CRDTs) are implemented in **Phase 30: Graphoid Platform** - a separate project written in Graphoid.

### Developer Experience

| Phase | Name | Priority | Duration | Dependencies |
|-------|------|----------|----------|--------------|
| [26](PHASE_26_REFLECTION.md) | Runtime Reflection | Medium | 5-7 days | None |
| [27](PHASE_27_DEBUGGER.md) | Debugger | Medium | 10-14 days | None |
| [28](PHASE_28_STDLIB_TRANSLATION.md) | Stdlib Translation | Medium | 7-10 days | None |

### Compilation & Self-Hosting

| Phase | Name | Priority | Duration | Dependencies |
|-------|------|----------|----------|--------------|
| [29](PHASE_29_COMPILATION_STRATEGY.md) | Compilation Strategy | **High** | 14-21 days | Phase 15-18 |
| [30](PHASE_30_WASM_COMPILATION.md) | WASM Compilation | **High** | 14-21 days | Phase 29 |
| [31](PHASE_31_NATIVE_COMPILATION.md) | Native Compilation | **High** | 21-30 days | Phase 29, 20 |
| [32](PHASE_32_SELF_HOSTING.md) | Self-Hosting | **Ultimate** | 30-45 days | Phase 29, 30 or 31 |

---

## Phase Dependencies

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                          GRAPH-CENTRIC FOUNDATION (MUST DO FIRST)                    │
│                                                                                     │
│   Phase 15 (Namespace) ───► Phase 16 (Execution)                                    │
│                                      │                                              │
│                            ┌─────────┴─────────┐                                    │
│                            │                   │                                    │
│                            ▼                   ▼                                    │
│                     Phase 17           Phase 18                                     │
│                     (Modules)          (Complete Model)                             │
│                            │                   │                                    │
│                            └─────────┬─────────┘                                    │
│                                      │ (enables concurrency)                        │
│                                      ▼                                              │
│                               Phase 19 (Concurrency)                                │
│                                      │                                              │
└──────────────────────────────────────┼──────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────────────┐
│                    PLATFORM SUPPORT (PARALLEL - NO DEPENDENCIES)                     │
│                                                                                     │
│   Phase 18.5 (Platform Support) ───► Graphoid Platform                              │
│   - Timers                           - Runtime                                      │
│   - Signals                          - Loader                                       │
│   - Module reload                    - Logger                                       │
│   - File watching                    - Reload                                       │
│   - Introspection                    - Monitor                                      │
│                                                                                     │
│   (Can start immediately, enables Platform development)                             │
└─────────────────────────────────────────────────────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┼─────────────────────────────────────┐
         │              ECOSYSTEM TRACK│                                      │
         │                             │                                      │
         │ Phase 19 (Concurrency) ─────┼──► Phase 23 (Distributed) ──► 24    │
         │                             │                │                     │
         │                             │                ▼                     │
         │                             │       Phase 25 (Vector Search)      │
         │                             │                                      │
         │ Phase 20 (FFI) ──┬──────────┼──► Phase 22 (Database)              │
         │                  │          │         ▲                            │
         │                  ▼          │         │                            │
         │ Phase 21 (Package Mgr) ─────┼─────────┘                            │
         │                             │                                      │
         └─────────────────────────────┼──────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┼──────────────────────────────────────┐
         │              COMPILATION TRACK                                      │
         │                             │                                      │
         │   Phase 29 (Bytecode VM) ◄──┘                                      │
         │         │                                                          │
         │         ├──────────────┬─────────────────┐                         │
         │         │              │                 │                         │
         │         ▼              ▼                 │                         │
         │   Phase 31        Phase 32              │                          │
         │   (WASM)          (Native) ◄────────────┼── Phase 20 (FFI)        │
         │         │              │                 │                         │
         │         └──────┬───────┘                 │                         │
         │                │                         │                         │
         │                ▼                         │                         │
         │         Phase 33 (Self-Hosting)          │                         │
         │                                                                    │
         └────────────────────────────────────────────────────────────────────┘

         ┌────────────────────────────────────────────────────────────────────┐
         │              INDEPENDENT (Can start after Phase 16)                 │
         │                                                                    │
         │   Phase 26 (Reflection)                                            │
         │   Phase 27 (Debugger)                                              │
         │   Phase 28 (Stdlib Translation)                                    │
         │                                                                    │
         └────────────────────────────────────────────────────────────────────┘
```

---

## Recommended Implementation Order

### Immediate (Two Parallel Tracks)

**Track A: Graph-Centric Foundation (BLOCKER for concurrency)**

1. **Phase 15: Namespace Graph** - Variables as graph nodes, scopes as subgraphs
2. **Phase 16: Execution Graph** - AST as graph, functions as subgraphs
3. **Phase 17: Modules Graph** - Modules as subgraphs, imports as edges (can parallel with 18)
4. **Phase 18: Complete Graph Model** - Classes, types, patterns, exceptions (can parallel with 17)

**Track B: Platform Support (CRITICAL - unblocks Platform development)**

1. **Phase 18.5: Platform Support** - Timers, signals, module reload, file watching, introspection

*Track B has no dependencies and can start immediately. This enables platform development to proceed while graph-centric work continues.*

### Near-Term (Concurrency + Ecosystem)

5. **Phase 19: Concurrency** - Actors as nodes, channels as edges, graph-native messaging
6. **Phase 20: FFI** - C interop, Rust plugins, syscalls
7. **Phase 21: Package Manager** - Ecosystem enablement
8. **Phase 22: Database** - PostgreSQL, SQLite, Redis

### Medium-Term (Distributed + Compilation)

9. **Phase 23: Distributed Primitives** - Serialization, remote refs, routing hooks
10. **Phase 29: Bytecode VM** - 5-10x performance, foundation for compilation
11. **Phase 31: WASM Compilation** - Sandboxed execution, browser target
12. **Phase 24: Distributed Execution** - Safe remote execution (uses WASM)

### Long-Term (Self-Hosting)

13. **Phase 32: Native Compilation** - Maximum performance
14. **Phase 33: Self-Hosting** - Graphoid compiles itself
15. **Phase 25: Vector Search** - AI/ML capabilities
16. **Phases 26-28** - Developer experience polish

---

## Three Development Tracks

### Foundation: Graph-Centric Architecture (Phases 15-18)
**Goal**: Make "everything is a graph" TRUE at all levels

```
15 (Namespace) → 16 (Execution) → 17 (Modules) + 18 (Complete)
                                           ↓
                                    19 (Concurrency)
```

**This must be completed before starting concurrency or compilation tracks.**

### Track 1: Ecosystem (Phases 19-25)
**Goal**: Make Graphoid useful for real-world applications

```
19 (Concurrency) → 20 (FFI) → 21 (Package) → 22 (Database)
       │
       └──────────→ 23 (Distributed) → 24 (Distributed Exec) → 25 (Vector)
```

### Track 2: Compilation (Phases 29, 31-33)
**Goal**: Self-hosting, independence from Rust

```
29 (Bytecode VM) → 31 (WASM) ──────┐
                                   ├──→ 33 (Self-Hosting)
                 → 32 (Native) ────┘
```

**Tracks 1 and 2 can be developed in parallel** after the foundation is complete.

---

## Timeline Estimates

### Platform Support (Phase 18.5 - 18.6)
**Estimated**: 2 weeks
**Can Start**: Immediately (no dependencies)

| Milestone | Features | Duration |
|-----------|----------|----------|
| Timers & Signals | `timer.after`, `timer.every`, `signal.on` | 2-3 days |
| Module Management | `modules.reload`, `modules.unload` | 1-2 days |
| Server Caps | `net.bind`, `net.accept`, `http.Server` | 3-5 days |
| Introspection | `error.stack()`, `__MODULE__`, `runtime.memory()` | 1-2 days |

### Foundation: Graph-Centric (Phases 15-18)
**Estimated**: 5-8 weeks

| Milestone | Phases | Duration |
|-----------|--------|----------|
| Namespace is Graph | 15 | 1-2 weeks |
| Execution is Graph | 16 | 2-3 weeks |
| Complete Graph Model | 17, 18 | 2-3 weeks |

### Track 1: Ecosystem (Phases 19-25)
**Estimated**: 14-18 weeks

| Milestone | Phases | Duration |
|-----------|--------|----------|
| Concurrent Graphoid | 19 | 2-3 weeks |
| Interoperable Graphoid | 20 | 2 weeks |
| Ecosystem-Ready | 21, 22 | 3-4 weeks |
| Distributed Graphoid | 23, 24 | 6-8 weeks |
| AI-Ready | 25 | 2-3 weeks |

### Track 2: Compilation (Phases 29, 31-33)
**Estimated**: 12-18 weeks

| Milestone | Phases | Duration |
|-----------|--------|----------|
| Fast Graphoid | 29 | 2 weeks |
| Portable Graphoid | 31 | 3 weeks |
| Native Graphoid | 32 | 4-5 weeks |
| Self-Hosted Graphoid | 33 | 5-7 weeks |

### Full Completion
**Estimated**: 8-12 months (if sequential)
**Estimated**: 6-8 months (if parallel tracks after foundation)

---

## Design Principles

1. **Everything IS a Graph** - Namespace, runtime, and data are all graphs
2. **Actors ARE Nodes** - Concurrency built on graph primitives, not bolted on
3. **Channels ARE Edges** - Message passing is graph traversal
4. **Functions ARE Subgraphs** - Composition is graph connection
5. **M:N Green Threads** - Lightweight tasks scheduled across CPU cores
6. **FFI is Scaffolding** - Every native dependency has a pure Graphoid path
7. **WASM for Safety** - Sandboxed execution for untrusted code
8. **Self-Hosting is the Goal** - Graphoid must eventually compile itself
9. **Syscalls for Independence** - Direct kernel interface, no libc required
10. **Bytecode as IR** - Common representation for all backends

---

## Success Metrics

| Milestone | Criteria |
|-----------|----------|
| **Graph-Centric Graphoid** | Namespace is a graph, execution is graph traversal |
| **Concurrent Graphoid** | Actors as nodes, channels as edges, graph-native messaging |
| **Interoperable Graphoid** | Can call C/Rust libraries from .gr files |
| **Ecosystem-Ready Graphoid** | Package manager, registry, dependencies |
| **Database-Connected Graphoid** | PostgreSQL, SQLite, Redis + third-party |
| **Distributed Graphoid** | Graphs spanning multiple nodes |
| **Self-Executing Distributed Graphs** | Programs as distributed graph traversals |
| **AI-Ready Graphoid** | HNSW vector search, embeddings, similarity |
| **Fast Graphoid** | Bytecode VM, 5-10x interpreter speedup |
| **Portable Graphoid** | WASM compilation, browser deployment |
| **Native Graphoid** | Native compilation, 50-100x interpreter speedup |
| **Self-Hosted Graphoid** | Graphoid compiles itself, Rust deleted |

---

## File Index

### Design Rationale
- [GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md](GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md) - Why graph-centric execution matters
- [GRAPH_RUNTIME_TEST_SPECIFICATION.md](GRAPH_RUNTIME_TEST_SPECIFICATION.md) - Tests for runtime experiments
- [CONCURRENCY_MODEL_RATIONALE.md](CONCURRENCY_MODEL_RATIONALE.md) - Why actors-as-nodes

### Graph-Centric Foundation (BLOCKER)
- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Variables as graph nodes
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Execution via graph traversal
- [PHASE_17_MODULES_GRAPH.md](PHASE_17_MODULES_GRAPH.md) - Modules as subgraphs
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Classes, types, patterns, etc.

### Platform Support (CRITICAL - Can Start Immediately)
- [PHASE_18_5_PLATFORM_SUPPORT.md](PHASE_18_5_PLATFORM_SUPPORT.md) - Timers, signals, module reload, file watching, introspection

### Concurrency (Built on Graph Foundation)
- [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) - Actors as nodes, channels as edges

### Ecosystem (High Priority)
- [PHASE_20_FFI.md](PHASE_20_FFI.md) - C FFI, Rust plugins, syscalls
- [PHASE_21_PACKAGE_MANAGER.md](PHASE_21_PACKAGE_MANAGER.md) - Dependency management
- [PHASE_22_DATABASE.md](PHASE_22_DATABASE.md) - Database connectivity
- [PHASE_23_DISTRIBUTED_PRIMITIVES.md](PHASE_23_DISTRIBUTED_PRIMITIVES.md) - Remote references, partitioning
- [PHASE_24_DISTRIBUTED_EXECUTION.md](PHASE_24_DISTRIBUTED_EXECUTION.md) - Distributed scheduling
- [PHASE_25_VECTOR_SEARCH.md](PHASE_25_VECTOR_SEARCH.md) - HNSW graph type, similarity

### Developer Experience (Medium Priority)
- [PHASE_26_REFLECTION.md](PHASE_26_REFLECTION.md) - Runtime type introspection
- [PHASE_27_DEBUGGER.md](PHASE_27_DEBUGGER.md) - Interactive debugging
- [PHASE_28_STDLIB_TRANSLATION.md](PHASE_28_STDLIB_TRANSLATION.md) - Pure Graphoid stdlib

### Compilation & Self-Hosting (High Priority)
- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Bytecode VM, dual-path execution
- [PHASE_30_WASM_COMPILATION.md](PHASE_30_WASM_COMPILATION.md) - WebAssembly target
- [PHASE_31_NATIVE_COMPILATION.md](PHASE_31_NATIVE_COMPILATION.md) - Native x86_64/ARM64
- [PHASE_32_SELF_HOSTING.md](PHASE_32_SELF_HOSTING.md) - Graphoid compiler in Graphoid

### Separate Projects (Not Numbered)
- [Graphoid Platform](platform/GRAPHOID_PLATFORM.md) - Application framework (Rails/Django for Graphoid) - after core language stabilizes

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 9.2 | 2026-01-28 | Phase 15 (Namespace as Graph) complete. Phases 18.5-18.6 planned for after graph-centric foundation. Phase 16 next. |
| 9.1 | 2026-01-28 | Added Phase 18.6: Server Capabilities (bind/accept/listen) to enable interactive web simulations immediately, replacing the wait for WASM. |
| 9.0 | 2026-01-23 | Added Phase 18.5: Platform Support (timers, signals, module reload, file watching, introspection). This phase has no dependencies and unblocks Graphoid Platform development. |
| 8.0 | 2026-01-22 | Phase 29 rewritten as Compilation Strategy (dual-path, interpreter-first for dev, compiled for production). Moved Graphoid Platform out of numbered sequence (now GRAPHOID_PLATFORM.md). Renumbered: 31-33 → 30-32. |
| 7.0 | 2026-01-20 | Phase 23 rewritten as Distribution Primitives (serialization, remote refs, routing hooks). Added Phase 30: Graphoid Platform (separate project for Pregel, Actors, MapReduce, CRDTs). Renumbered compilation phases: 30-32 → 31-33. |
| 6.0 | 2026-01-16 | Renumbered phases: 14A-D → 15-18, old 15-28 → 19-32. Concurrency redesigned for graph-centric primitives. |
| 5.0 | 2026-01-15 | Added graph-centric architecture phases (14A-14D) as blockers |
| 4.0 | 2026-01-14 | Added Phase 21 (Vector Search), renumbered 21-23 → 22-24 |

---

## The End State Vision

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│                        GRAPHOID: THE GRAPH LANGUAGE                         │
│                                                                             │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐           │
│  │   Concurrent    │   │   Distributed   │   │   Self-Hosted   │           │
│  │ (Actors=Nodes)  │   │   (Clusters)    │   │   (No Rust)     │           │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘           │
│                                                                             │
│  ┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐           │
│  │   Native Speed  │   │   WASM Target   │   │   AI-Ready      │           │
│  │   (Compiled)    │   │   (Portable)    │   │   (Vectors)     │           │
│  └─────────────────┘   └─────────────────┘   └─────────────────┘           │
│                                                                             │
│  Everything is a graph. Actors are nodes. Channels are edges.               │
│  Zero dependencies. Pure Graphoid.                                          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```
