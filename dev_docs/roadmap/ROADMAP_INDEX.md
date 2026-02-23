# Graphoid Implementation Roadmap

**Version**: 11.0
**Last Updated**: February 18, 2026
**Status**: Phases 0-18.7 Complete. Phase 19 (Concurrency) next.

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

**Total**: 2,400+ Rust tests, 672+ gspec tests

---

## Remaining Phases (15-33)

### Graph-Centric Foundation — ✅ ALL COMPLETE

Phases 15-18 made "everything is a graph" true at all levels: namespace, execution, modules, and the complete graph model (universe graph, templates, reflect.pattern, exception propagation). Specs archived to `dev_docs/archive/completed_phases/`.

### Pre-Concurrency (Phases 18.6-18.7)

Features that require no concurrency and can be implemented immediately.

| Phase | Name | Priority | Duration | Dependencies | Status |
|-------|------|----------|----------|--------------|--------|
| [18.6](PHASE_18_6_SERVER_CAPABILITIES.md) | Server Capabilities | **CRITICAL** | 3-5 days | None | ✅ Complete |
| [18.7](PHASE_18_7_RUNTIME_INTROSPECTION.md) | Runtime Introspection | **High** | 2-3 days | None | ✅ Complete |

**Phase 18.6**: Adds `net.bind()`, `net.accept()` to enable TCP/HTTP servers. Blocking/sequential — no concurrency needed.

**Phase 18.7**: Runtime introspection features that don't require concurrency: `modules.list()`, `modules.info()`, `runtime.memory()`, `runtime.version()`, `runtime.uptime()`, `error.stack()`, `__MODULE__`.

### Unlocks
Once Phase 18.6 is complete, development can begin on **GraphWeb**, a Sinatra-like web framework.
See: [PLAN_WEB_FRAMEWORK.md](PLAN_WEB_FRAMEWORK.md)

### Concurrency (Phase 19 — Sub-Phases)

Built on graph-centric foundation: actors ARE nodes, channels ARE edges. Share-nothing architecture.

See [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) for full specification.

| Phase | Name | Priority | Duration | Dependencies |
|-------|------|----------|----------|--------------|
| 19.1 | Spawn + Channels | **Critical** | 5-7 days | Phase 15, 16 |
| 19.2 | Timers + Signals | **Critical** | 3-5 days | Phase 19.1 |
| 19.3 | Actors as Graph Nodes | **Critical** | 5-7 days | Phase 19.1 |
| 19.4 | Module Hot Reload | **High** | 3-5 days | Phase 19.1 |
| 19.5 | Select + Supervision | **High** | 5-7 days | Phase 19.3 |
| 19.6 | File Watching + Auto-Reload | **High** | 2-3 days | Phase 19.2, 19.4 |

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
│                   GRAPH-CENTRIC FOUNDATION — ✅ ALL COMPLETE                         │
│                                                                                     │
│   Phase 15 (Namespace) ✅ → Phase 16 (Execution) ✅                                 │
│                                      │                                              │
│                            ┌─────────┴─────────┐                                    │
│                            ▼                   ▼                                    │
│                     Phase 17 ✅         Phase 18 ✅                                  │
│                     (Modules)          (Complete Model)                             │
└─────────────────────────────────────────────────────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┼─────────────────────────────────────┐
         │              PRE-CONCURRENCY│                                      │
         │                             │                                      │
         │   Phase 18.6 (Server) ──────┤                                      │
         │   Phase 18.7 (Introspect) ──┤                                      │
         │                             │                                      │
         └─────────────────────────────┼──────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┼─────────────────────────────────────┐
         │              CONCURRENCY (Phase 19 Sub-Phases)                      │
         │                             │                                      │
         │   19.1 (Spawn+Channels) ────┤                                      │
         │         │                   │                                      │
         │         ├── 19.2 (Timers+Signals)                                  │
         │         ├── 19.3 (Actors as Graph Nodes)                           │
         │         ├── 19.4 (Module Hot Reload)                               │
         │         │         │                                                │
         │         │   19.5 (Select+Supervision) ◄── 19.3                     │
         │         │   19.6 (File Watch+Auto-Reload) ◄── 19.2, 19.4          │
         │                             │                                      │
         └─────────────────────────────┼──────────────────────────────────────┘
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
         │         ▼              ▼                 │                         │
         │   Phase 31        Phase 32              │                          │
         │   (WASM)          (Native) ◄────────────┼── Phase 20 (FFI)        │
         │         │              │                 │                         │
         │         └──────┬───────┘                 │                         │
         │                ▼                         │                         │
         │         Phase 33 (Self-Hosting)          │                         │
         │                                                                    │
         └────────────────────────────────────────────────────────────────────┘

         ┌────────────────────────────────────────────────────────────────────┐
         │              INDEPENDENT (Can start anytime)                        │
         │                                                                    │
         │   Phase 26 (Reflection)                                            │
         │   Phase 27 (Debugger)                                              │
         │   Phase 28 (Stdlib Translation)                                    │
         │                                                                    │
         └────────────────────────────────────────────────────────────────────┘
```

---

## Recommended Implementation Order

### Immediate (Pre-Concurrency)

1. ~~**Phase 18.6: Server Capabilities**~~ ✅ Complete (Feb 19, 2026)
2. ~~**Phase 18.7: Runtime Introspection**~~ ✅ Complete (Feb 20, 2026)

### Near-Term (Concurrency)

3. **Phase 19.1: Spawn + Channels** - Core concurrency foundation (share-nothing)
4. **Phase 19.2: Timers + Signals** - Timer channels, signal channels
5. **Phase 19.3: Actors as Graph Nodes** - Actors ARE nodes, graph-native messaging
6. **Phase 19.4: Module Hot Reload** - Erlang-style per-task reload
7. **Phase 19.5: Select + Supervision** - Channel multiplexing, restart strategies
8. **Phase 19.6: File Watching + Auto-Reload** - `fs.watch()` triggers hot reload

### Medium-Term (Ecosystem + Compilation)

9. **Phase 20: FFI** - C interop, Rust plugins, syscalls
10. **Phase 21: Package Manager** - Ecosystem enablement
11. **Phase 22: Database** - PostgreSQL, SQLite, Redis
12. **Phase 23: Distributed Primitives** - Serialization, remote refs, routing hooks
13. **Phase 29: Bytecode VM** - 5-10x performance

### Long-Term (Distributed + Self-Hosting)

14. **Phase 24: Distributed Execution** - Safe remote execution
15. **Phase 25: Vector Search** - AI/ML capabilities
16. **Phase 31: WASM Compilation** - Sandboxed execution, browser target
17. **Phase 32: Native Compilation** - Maximum performance
18. **Phase 33: Self-Hosting** - Graphoid compiles itself
19. **Phases 26-28** - Developer experience polish

---

## Timeline Estimates

### Pre-Concurrency (Phases 18.6-18.7)
**Estimated**: 1 week
**Can Start**: Immediately

| Milestone | Features | Duration |
|-----------|----------|----------|
| Server Caps | `net.bind`, `net.accept`, `http.Server` | 3-5 days |
| Introspection | `modules.list/info`, `runtime.*`, `error.stack()`, `__MODULE__` | 2-3 days |

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
5. **Share-Nothing Tasks** - Isolated namespaces, communicate via channels
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

### Archived (Completed)
- Phases 15-18, design rationale docs → `dev_docs/archive/completed_phases/`

### Pre-Concurrency
- [PHASE_18_6_SERVER_CAPABILITIES.md](PHASE_18_6_SERVER_CAPABILITIES.md) - TCP/HTTP server (net.bind, net.accept)
- [PHASE_18_7_RUNTIME_INTROSPECTION.md](PHASE_18_7_RUNTIME_INTROSPECTION.md) - modules.list/info, runtime.*, error.stack(), __MODULE__

### Concurrency (Phase 19 Sub-Phases)
- Concurrency syntax defined in `dev_docs/LANGUAGE_SPECIFICATION.md` § Concurrency
- Sub-phase breakdown in this file (see Concurrency section above)

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
| 11.0 | 2026-02-18 | Phases 15-18 complete. Phase 18.5 split: concurrency parts into Phase 19 sub-phases, non-concurrency into 18.7. Phase 19 broken into 6 sub-phases (19.1-19.6). Concurrency syntax added to language spec. BigNum cleanup complete. |
| 10.0 | 2026-02-09 | Phases 15-17 complete. Phase 18 in progress. |
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
