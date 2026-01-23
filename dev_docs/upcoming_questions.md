# Upcoming Roadmap Questions

**Last Updated**: January 21, 2026

---

## Session Summary (January 21, 2026)

Evaluated and updated phases 20-28 for graph-centric alignment:

| Phase | Name | Status | Changes |
|-------|------|--------|---------|
| 19 | Concurrency | ✅ Complete | Already redesigned (actors=nodes, channels=edges) |
| 20 | FFI | ✅ Complete | Already aligned (Bridge Nodes, Foreign Realm Model) |
| 21 | Package Manager | ✅ Complete | Already aligned (graph-based resolution) |
| 22 | Database | ✅ Complete | Already aligned (connections as Bridge Nodes) |
| 23 | Distributed Primitives | ✅ Complete | Already aligned (peers as Bridge Nodes) |
| 24 | Distributed Execution | ✅ Complete | Added Five-Layer Architecture section |
| 25 | Vector Search | ✅ Complete | Added Five-Layer Architecture section |
| 26 | Reflection | ✅ Complete | **Complete rewrite** - reflection IS graph traversal |
| 27 | Debugger | ✅ Complete | **Complete rewrite** - debugging as graph inspection |
| 28 | Stdlib Translation | ✅ Complete | **Complete rewrite** - 9 modules, subprocess deferred |

### Key Decisions

1. **Five-Layer Architecture**: Phases 24-25 updated to explicitly reference DataLayer, BehaviorLayer, ControlLayer, MetadataLayer for distinguishing user data from internal nodes.

2. **Reflection as Graph Traversal**: Phase 26 completely rewritten. `reflect(value)` returns a graph view, type checking is path-finding in type graph.

3. **Debugging as Graph Inspection**: Phase 27 completely rewritten. Commands map to graph operations (step = advance execution edge, locals = query namespace subgraph).

4. **Stdlib Expansion**: Phase 28 expanded to "batteries included" with 9 modules:
   - constants, encoding/, url/, uuid/, path/, logging/, argparse/, compress/, archive/
   - All modules have graph-centric data structures

5. **Subprocess Deferred**: Security concerns (command injection, shell quoting, env leakage) and philosophical misalignment with Graphoid's self-sufficiency mission. May revisit for build tool integration later.

---

## Phases to Evaluate Next Session

Phase: 29
Name: Bytecode VM
Key Question: Compile graph IR? How does bytecode represent graph operations?
────────────────────────────────────────
Phase: 30
Name: Graphoid Platform
Key Question: Standard runtime environment as graphs?
────────────────────────────────────────
Phase: 31
Name: WASM Compilation
Key Question: Graph → WASM? Preserve graph semantics?
────────────────────────────────────────
Phase: 32
Name: Native Compilation
Key Question: Graph → machine code? Graph-aware optimizations?
────────────────────────────────────────
Phase: 33
Name: Self-Hosting
Key Question: Graphoid compiler as graph transformations?

The compilation phases (29-33) will be particularly interesting - if the compiler itself operates on graphs and produces graphs, that would be the ultimate expression of "everything is a graph."
