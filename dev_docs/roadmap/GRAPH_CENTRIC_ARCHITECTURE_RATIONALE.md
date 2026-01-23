# Graph-Centric Architecture Rationale

**Version**: 1.0
**Created**: January 15, 2026
**Status**: Design Document

---

## The Problem

Graphoid claims "everything is a graph" at three levels:

| Level | Claim | Current Reality |
|-------|-------|-----------------|
| **1. Data Structures** | Lists, maps, trees are graphs | ✅ Partially true (Graph-backed) |
| **2. Namespace** | Variables are nodes in a meta-graph | ❌ FALSE (HashMap<String, Value>) |
| **3. Runtime** | Execution is graph traversal | ❌ FALSE (Tree-walking interpreter) |

**This is a fundamental architectural problem.** The language's core philosophy is not implemented.

---

## Why This Matters

### 1. Identity Crisis

If "everything is a graph" is false, why use Graphoid at all? The language loses its unique value proposition.

### 2. Compilation Blocker

Self-hosting and native compilation require understanding HOW the language works. If execution isn't graph-based, the self-hosted compiler would have to implement a traditional interpreter - defeating the purpose.

### 3. Missed Opportunities

Graph-centric execution enables:
- **Graph-based optimization** - Rewrite rules on execution graphs
- **Parallel execution** - Independent subgraphs execute concurrently
- **Debugging as graph inspection** - Visualize program state as graphs
- **Hot code reloading** - Swap graph nodes without restart
- **Distributed execution** - Partition execution graph across nodes

### 4. Philosophical Consistency

A language about graphs should BE graphs, not just HAVE graphs.

---

## The Vision

### Current Architecture (Imperative)

```
┌─────────────────────────────────────────────────────────────────────┐
│                     CURRENT IMPLEMENTATION                           │
│                                                                     │
│  Source Code (.gr)                                                  │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────┐                                                    │
│  │   Lexer     │ ── Tokens                                          │
│  └─────────────┘                                                    │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────┐                                                    │
│  │   Parser    │ ── AST (Rust enum)                                 │
│  └─────────────┘                                                    │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────┐     ┌─────────────────────┐                        │
│  │  Executor   │ ──► │ Environment         │                        │
│  │ (match arms)│     │ HashMap<String,Value>│                        │
│  └─────────────┘     └─────────────────────┘                        │
│                                                                     │
│  Problem: Traditional tree-walking interpreter                      │
│  Problem: HashMap-based variable storage                            │
│  Problem: Graphs are just a data type, not fundamental              │
└─────────────────────────────────────────────────────────────────────┘
```

### Target Architecture (Graph-Centric)

```
┌─────────────────────────────────────────────────────────────────────┐
│                     GRAPH-CENTRIC ARCHITECTURE                       │
│                                                                     │
│  Source Code (.gr)                                                  │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────┐                                                    │
│  │   Lexer     │ ── Tokens                                          │
│  └─────────────┘                                                    │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────┐                                                    │
│  │   Parser    │ ── AST Graph (not enum!)                           │
│  └─────────────┘                                                    │
│       │                                                             │
│       ▼                                                             │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │                    UNIVERSE GRAPH                            │    │
│  │  ┌───────────────────────────────────────────────────────┐  │    │
│  │  │  Meta-Graph (Namespace)                               │  │    │
│  │  │  ┌─────────┐    ┌─────────┐    ┌─────────┐           │  │    │
│  │  │  │ var "x" │───►│ Value 5 │    │ var "y" │───►...    │  │    │
│  │  │  └─────────┘    └─────────┘    └─────────┘           │  │    │
│  │  │       ▲                              ▲                │  │    │
│  │  │       │ (parent scope edge)          │                │  │    │
│  │  │  ┌─────────────────────────────────────┐             │  │    │
│  │  │  │         Scope: "global"             │             │  │    │
│  │  │  └─────────────────────────────────────┘             │  │    │
│  │  └───────────────────────────────────────────────────────┘  │    │
│  │                                                             │    │
│  │  ┌───────────────────────────────────────────────────────┐  │    │
│  │  │  Execution Graph (Runtime)                            │  │    │
│  │  │  ┌─────────┐    ┌─────────┐    ┌─────────┐           │  │    │
│  │  │  │ eval(+) │───►│ eval(x) │    │ eval(y) │           │  │    │
│  │  │  └─────────┘    └─────────┘    └─────────┘           │  │    │
│  │  │       │                                               │  │    │
│  │  │       ▼ (execution edge)                              │  │    │
│  │  │  ┌─────────┐                                          │  │    │
│  │  │  │result: 8│                                          │  │    │
│  │  │  └─────────┘                                          │  │    │
│  │  └───────────────────────────────────────────────────────┘  │    │
│  │                                                             │    │
│  │  ┌───────────────────────────────────────────────────────┐  │    │
│  │  │  User Graphs (Data)                                   │  │    │
│  │  │  ┌─────────┐    ┌─────────┐    ┌─────────┐           │  │    │
│  │  │  │ Graph A │    │ Graph B │    │ Graph C │           │  │    │
│  │  │  └─────────┘    └─────────┘    └─────────┘           │  │    │
│  │  └───────────────────────────────────────────────────────┘  │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                                                                     │
│  Executor: Traverses execution graph                                │
│  Variables: Nodes in meta-graph with binding edges                  │
│  User data: Subgraphs within universe                               │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Three-Layer Architecture

### Layer 1: Data Graphs (Mostly Done)

User-created graphs for data structures:

```graphoid
my_graph = graph{}
my_graph.add_node("A", 100)
my_graph.add_edge("A", "B", "connects")
```

**Current status**: ✅ Implemented, graphs work as data structures
**Improvement needed**: Graphs should be nodes in universe graph, not isolated Rust structs

### Layer 2: Namespace Graph (Meta-Graph)

Variables, scopes, and bindings as graph structure:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        NAMESPACE GRAPH                               │
│                                                                     │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Scope: "global"                                              │   │
│  │  ┌─────────┐         ┌─────────┐         ┌─────────┐         │   │
│  │  │ "x"     │──bind──►│ Num(5)  │         │ "add"   │──bind──►│   │
│  │  └─────────┘         └─────────┘         └─────────┘         │   │
│  │                                                │               │   │
│  │                                                ▼               │   │
│  │                                          ┌─────────┐           │   │
│  │                                          │ Fn{...} │           │   │
│  │                                          └─────────┘           │   │
│  └──────────────────────────────────────────────────────────────┘   │
│         │                                                           │
│         │ (child scope edge)                                        │
│         ▼                                                           │
│  ┌──────────────────────────────────────────────────────────────┐   │
│  │  Scope: "fn:add"                                              │   │
│  │  ┌─────────┐         ┌─────────┐                              │   │
│  │  │ "a"     │──bind──►│ Num(?)  │   (parameter, bound at call) │   │
│  │  └─────────┘         └─────────┘                              │   │
│  │  ┌─────────┐         ┌─────────┐                              │   │
│  │  │ "b"     │──bind──►│ Num(?)  │                              │   │
│  │  └─────────┘         └─────────┘                              │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  Variable lookup = Graph traversal up scope chain                   │
│  Closure = Captured edges to outer scope nodes                      │
│  Module = Subgraph with import/export edges                         │
└─────────────────────────────────────────────────────────────────────┘
```

**Benefits**:
- Variable lookup is graph traversal (not HashMap lookup)
- Closures are naturally captured edges
- Scope visualization is built-in
- Debuggers can inspect namespace as graph

### Layer 3: Execution Graph (Runtime)

Program execution as graph traversal:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        EXECUTION GRAPH                               │
│                                                                     │
│  Source: result = add(x, y)                                         │
│                                                                     │
│                    ┌───────────────┐                                │
│                    │ Assign        │                                │
│                    │ target:"result"│                                │
│                    └───────┬───────┘                                │
│                            │ (value edge)                           │
│                            ▼                                        │
│                    ┌───────────────┐                                │
│                    │ Call          │                                │
│                    │ fn: "add"     │                                │
│                    └───────┬───────┘                                │
│                   ┌────────┼────────┐                               │
│            (arg0) │        │        │ (arg1)                        │
│                   ▼        │        ▼                               │
│           ┌───────────┐    │    ┌───────────┐                       │
│           │ Ident "x" │    │    │ Ident "y" │                       │
│           └───────────┘    │    └───────────┘                       │
│                            │                                        │
│                            │ (fn edge - resolved)                   │
│                            ▼                                        │
│                    ┌───────────────┐                                │
│                    │ Fn "add"      │                                │
│                    │ body: ...     │                                │
│                    └───────────────┘                                │
│                                                                     │
│  Execution = Depth-first traversal of graph                         │
│  Optimization = Graph rewrite rules                                 │
│  Parallelism = Independent subgraphs                                │
└─────────────────────────────────────────────────────────────────────┘
```

**Benefits**:
- Execution is literally graph traversal
- Optimization via graph rewriting (constant folding, etc.)
- Parallel execution of independent subgraphs
- Debugging shows execution graph
- Hot code reload = swap nodes

---

## The Universe Graph

All graphs exist within a single "Universe Graph":

```
┌─────────────────────────────────────────────────────────────────────┐
│                         UNIVERSE GRAPH                               │
│                                                                     │
│  ┌────────────────────┐                                             │
│  │ Root               │                                             │
│  └─────────┬──────────┘                                             │
│            │                                                        │
│    ┌───────┼───────┬───────────┬───────────────┐                   │
│    │       │       │           │               │                   │
│    ▼       ▼       ▼           ▼               ▼                   │
│ ┌──────┐┌──────┐┌──────┐ ┌──────────┐   ┌───────────┐              │
│ │Meta- ││Exec- ││User  │ │ Module   │   │ Module    │              │
│ │Graph ││Graph ││Graph1│ │ "math"   │   │ "stdlib"  │              │
│ └──────┘└──────┘└──────┘ └──────────┘   └───────────┘              │
│    │                          │               │                     │
│    ▼                          ▼               ▼                     │
│ (namespace               (fn graph)      (fn graph)                │
│  structure)              exports: sin    exports: print            │
│                          cos, tan...     read, write...            │
│                                                                     │
│  Every graph is a node in the universe                              │
│  Inter-graph references are edges                                   │
│  GC = Unreachable node collection                                   │
└─────────────────────────────────────────────────────────────────────┘
```

### User Access Restrictions

**CRITICAL DESIGN DECISION**: Users should NOT have direct access to:
- The universe graph root
- The meta-graph (namespace internals)
- The execution graph (runtime internals)

**Users CAN access**:
- Their own data graphs
- Module public APIs
- Debugging tools that INSPECT (not modify) internal graphs

```graphoid
# ❌ FORBIDDEN - Direct universe access
universe.add_node(...)           # No!
meta_graph.variables["x"] = 10   # No!

# ✅ ALLOWED - Normal language operations
x = 10                           # Creates node in meta-graph (implicit)
my_graph.add_node("A", 100)      # User data graph

# ✅ ALLOWED - Debug inspection (read-only)
debug.inspect_namespace()        # See current scope as graph
debug.inspect_call_stack()       # See execution graph
```

---

## Performance Considerations

### Concern: Graph Overhead

Graph traversal may be slower than direct HashMap lookup or switch dispatch.

### Mitigations

1. **Internal Optimizations** (Hidden from users)
   - Use adjacency lists (efficient for sparse graphs)
   - Cache frequently accessed paths
   - Use compact node IDs, not string keys internally

2. **Just-In-Time Optimization**
   - Detect hot paths and optimize them
   - Constant folding via graph rewriting
   - Dead code elimination via unreachable node removal

3. **Hybrid Approach** (Acceptable Compromise)
   - Internal representation uses graphs
   - But execution may use optimized dispatch for hot paths
   - The SEMANTICS are graph-based even if implementation takes shortcuts

4. **Benchmarking Targets**
   - Namespace lookup: ≤2x HashMap overhead acceptable
   - Execution: ≤1.5x tree-walking overhead acceptable
   - User graphs: No regression from current implementation

### The Philosophy

> "We might accept some little compromising for efficiency."

The goal is **semantic graph-centricity**. If the runtime behaves AS IF everything is a graph, internal optimizations are acceptable. The key is:

1. Debugging shows graphs
2. Graph operations work on all levels
3. The mental model is consistent
4. Compilation assumes graph semantics

---

## Implementation Phases

### Phase A: Namespace Graph (Medium Complexity)

Replace Environment HashMap with graph structure:

1. `Variable` = Node with name and binding edge to value
2. `Scope` = Subgraph containing variable nodes
3. Lookup = Traverse parent edges to find binding
4. Closure = Captured edges to outer scope

**Estimated effort**: 7-10 days
**Risk**: Medium (localized change)

### Phase B: Execution Graph (High Complexity)

Replace AST enum with graph representation:

1. `AstNode` = Graph node with edges to children
2. `Executor` = Graph traverser
3. Result = Value attached to node
4. Optimization = Graph rewrite rules

**Estimated effort**: 14-21 days
**Risk**: High (touches everything)

### Phase C: Universe Integration (Medium Complexity)

Connect all graphs into universe:

1. Every graph becomes a node
2. Inter-graph references become edges
3. GC based on graph reachability
4. Debug inspection tools

**Estimated effort**: 5-7 days
**Risk**: Medium

---

## Migration Strategy

### Phase 1: Parallel Implementation

Build new graph-centric structures alongside existing code:
- `GraphEnvironment` alongside `Environment`
- `AstGraph` alongside `Ast`
- Feature flag to switch between them

### Phase 2: Gradual Transition

One component at a time:
1. First: Namespace (Environment → GraphEnvironment)
2. Second: Parser (Ast enum → AstGraph)
3. Third: Executor (match arms → graph traversal)

### Phase 3: Cleanup

Remove old code:
- Delete `Environment` struct
- Delete `Ast` enums
- Delete match-based execution

---

## Success Criteria

### Functional Requirements

- [ ] Variables stored as graph nodes
- [ ] Scopes stored as subgraphs with parent edges
- [ ] Variable lookup via graph traversal
- [ ] AST represented as graph
- [ ] Execution via graph traversal
- [ ] All graphs connected in universe
- [ ] User graphs are nodes in universe
- [ ] All existing tests pass

### Performance Requirements

- [ ] Namespace lookup ≤2x HashMap overhead
- [ ] Execution ≤1.5x tree-walking overhead
- [ ] Memory overhead ≤30% increase

### Debugging Requirements

- [ ] Namespace viewable as graph
- [ ] Call stack viewable as graph
- [ ] Execution trace as graph traversal path

### User Experience Requirements

- [ ] No changes to .gr file syntax
- [ ] No access to internal graphs (universe, meta, exec)
- [ ] Existing programs work unchanged
- [ ] Better error messages (with graph context)

---

## Related Documents

- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Namespace implementation
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Execution implementation
- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Depends on graph-centric arch
- [PHASE_32_SELF_HOSTING.md](PHASE_32_SELF_HOSTING.md) - Depends on graph-centric arch

---

## Conclusion

Graph-centric architecture is not a nice-to-have. It is the **raison d'être** of Graphoid. Without it, the language is just another scripting language with graph data structures.

The investment is significant (3-5 weeks) but the payoff is:
1. Philosophical consistency ("everything is a graph" becomes TRUE)
2. Compilation foundation (self-hosting assumes graph semantics)
3. Unique capabilities (graph-based debugging, optimization, distribution)
4. Language identity (clear differentiation from alternatives)

**Recommendation**: Implement as Phases 15 and 16, before starting compilation track (Phases 29-32).
