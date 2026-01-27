# Phase 29: Compilation Strategy

**Duration**: 14-21 days
**Priority**: High (Foundation for production deployment)
**Dependencies**: Phases 15-18 (Complete Graph Model)
**Status**: Future

---

## Goal

Implement a compilation system for Graphoid that enables production deployment with improved performance and security, while maintaining the interpreter as the default development experience.

**Key principle**: Compilation is a tool for production, not a replacement for interpretation. Graphoid programs can run interpreted during development and compiled for deployment.

---

## Why Compilation?

### Production Applications Need It

| Need | How Compilation Helps |
|------|----------------------|
| **Speed** | 5-10x improvement for CPU-bound code |
| **Security** | Compiled code is harder to reverse-engineer/modify |
| **Distribution** | Ship binaries instead of source |
| **Startup** | Pre-compiled code loads faster than parsing |
| **Optimization** | Ahead-of-time analysis enables better optimization |

### Development Stays Interpreted

| Benefit | Why It Matters |
|---------|----------------|
| Fast iteration | No compile step during development |
| Better errors | Full source context available |
| Easy debugging | Direct inspection of runtime state |
| REPL | Interactive exploration |

### The User Experience

```bash
# Development - interpreted (default)
gr run app.gr              # Fast iteration, great errors
gr repl                    # Interactive exploration
gr test                    # Quick test runs

# Production - compiled
gr build app.gr            # Compile for deployment
gr build --release app.gr  # Optimized build
./app                      # Run compiled binary
```

---

## What Compilation Can and Cannot Do

### Where Compilation Helps (CPU-Bound)

```graphoid
# Tight loops - YES, big improvement
fn fibonacci(n) {
    if n <= 1 { return n }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

# Numeric computation - YES
fn compute_scores(data) {
    total = 0
    for item in data {
        total = total + (item.value * item.weight)
    }
    return total
}

# String processing - YES
fn parse_log_lines(lines) {
    results = []
    for line in lines {
        if line.starts_with("ERROR") {
            results.append(parse_error(line))
        }
    }
    return results
}
```

**Expected improvement**: 5-10x for these patterns

### Where Compilation Has Limited Impact (I/O-Bound)

```graphoid
# Network calls - NO, bottleneck is latency
fn fetch_all(urls) {
    return urls.map(url => http.get(url))
}

# Database queries - NO, bottleneck is database
fn get_users(db) {
    return db.query("SELECT * FROM users")
}

# File operations - NO, bottleneck is disk
fn process_files(paths) {
    return paths.map(p => fs.read(p))
}
```

**Expected improvement**: Minimal (< 1.2x) - the interpreter isn't the bottleneck

### Graph Operations - Depends

```graphoid
# Graph traversal with computation - PARTIAL benefit
fn analyze_network(g) {
    for node in g.nodes() {
        score = compute_centrality(node)  # CPU-bound part benefits
        node.set("score", score)
    }
}

# Pure traversal - LIMITED benefit
fn find_paths(g, start, end) {
    return g.traverse(from: start, to: end)  # Already optimized in runtime
}
```

**Key insight**: Graph operations are often memory-bound or algorithmically-bound, not interpreter-bound. The runtime's graph engine is already written in Rust.

---

## Compilation Target Options

### Option 1: Bytecode VM (Recommended First)

**What**: Compile to portable bytecode, execute on stack-based VM.

```
Source (.gr) → AST → Bytecode → VM Execution
```

| Aspect | Assessment |
|--------|------------|
| Effort | 3-4 weeks |
| Speedup | 5-10x for CPU-bound |
| Portability | Excellent (single .grc file runs anywhere) |
| Debugging | Good (debug info in bytecode) |
| Dynamic features | Full support |

**File format**: `.grc` (Graphoid Compiled)

**Why start here**:
- Proven approach (Python, Lua, Ruby all use this)
- Portable output
- Maintains all dynamic features
- Foundation for other targets

### Option 2: Native Compilation (Future)

**What**: Compile to machine code via LLVM or Cranelift.

```
Source → AST → IR → LLVM → Native Binary
```

| Aspect | Assessment |
|--------|------------|
| Effort | 6-8 weeks (after bytecode) |
| Speedup | 50-100x possible |
| Portability | Must compile per-platform |
| Debugging | Harder (need DWARF) |
| Dynamic features | Some limitations |

**Why later**: Higher effort, bytecode covers most needs

### Option 3: WASM (Future)

**What**: Compile to WebAssembly for browser/edge deployment.

```
Source → AST → IR → WASM
```

| Aspect | Assessment |
|--------|------------|
| Effort | 4-6 weeks (after bytecode) |
| Use case | Browser, edge computing, sandboxed execution |
| Speedup | 10-50x |

**Why later**: Specialized use case, bytecode foundation helps

---

## Recommended Approach: Bytecode First

### Why Bytecode

1. **Proven**: Python, Lua, Ruby, Java all use bytecode VMs successfully
2. **Portable**: Single compiled file runs on any platform with Graphoid runtime
3. **Debuggable**: Can include source maps and debug info
4. **Dynamic**: Supports eval, hot reload, reflection
5. **Foundation**: Same IR can target native/WASM later

### Dual-Path Execution Model

Graphoid should use dual-path execution to preserve graph advantages:

```
┌─────────────────────────────────────────────────────────────┐
│  Graph IR                                                   │
│      │                                                      │
│      ├── Scalar code → Bytecode (fast stack operations)     │
│      │   (loops, math, strings)                             │
│      │                                                      │
│      └── Graph code → Graph-native (native traversal)       │
│          (traversals, queries, mutations)                   │
└─────────────────────────────────────────────────────────────┘
```

**Why dual-path**:
- Scalar code: Traditional bytecode is faster (cache-friendly, branch prediction)
- Graph code: Native graph operations avoid translation overhead

The compiler decides which path based on code analysis.

---

## Technical Design

### Compilation Pipeline

```
┌──────────────────────────────────────────────────────────────────┐
│                     COMPILATION PIPELINE                          │
│                                                                  │
│  Source (.gr)                                                    │
│      │                                                           │
│      ▼                                                           │
│  ┌────────────┐                                                  │
│  │   Lexer    │                                                  │
│  └────────────┘                                                  │
│      │                                                           │
│      ▼                                                           │
│  ┌────────────┐                                                  │
│  │   Parser   │                                                  │
│  └────────────┘                                                  │
│      │                                                           │
│      ▼                                                           │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                      Graph IR                               │  │
│  │  - Control Flow Graph (CFG)                                │  │
│  │  - Data Flow Graph (DFG)                                   │  │
│  │  - Optimization passes                                     │  │
│  └────────────────────────────────────────────────────────────┘  │
│      │                                                           │
│      ├─────────────────────┬─────────────────────┐               │
│      │                     │                     │               │
│      ▼                     ▼                     ▼               │
│  ┌──────────┐       ┌────────────┐       ┌────────────┐          │
│  │ Bytecode │       │   Native   │       │    WASM    │          │
│  │   VM     │       │   (LLVM)   │       │            │          │
│  └──────────┘       └────────────┘       └────────────┘          │
│   Phase 29            Phase 32             Phase 31              │
└──────────────────────────────────────────────────────────────────┘
```

### Graph IR

The Graph IR is the intermediate representation used for optimization. All compilation targets start from Graph IR.

```rust
enum IRNode {
    // Control flow
    Block { id: BlockId, ops: Vec<IROp>, terminator: Terminator },

    // Values
    Constant(Value),
    Parameter { index: usize, name: String },
    Local { slot: usize, name: String },

    // Operations
    BinaryOp { op: BinOp, left: NodeId, right: NodeId },
    UnaryOp { op: UnOp, operand: NodeId },
    Call { func: NodeId, args: Vec<NodeId> },

    // Graph-specific (preserved for graph-native execution)
    GraphTraverse { graph: NodeId, from: NodeId, opts: TraverseOpts },
    GraphQuery { graph: NodeId, pattern: Pattern },
    GraphMutate { graph: NodeId, mutation: Mutation },

    // Parallel hint (from independence analysis)
    ParallelRegion { subgraphs: Vec<BlockId> },
}
```

### Optimization Passes

Graph IR enables optimization via graph rewriting:

1. **Constant folding**: `2 + 3` → `5`
2. **Dead code elimination**: Remove unreachable blocks
3. **Common subexpression elimination**: Compute once, reuse
4. **Parallel detection**: Mark independent subgraphs for parallel execution
5. **Inline caching hints**: Mark hot call sites

### Bytecode Instruction Set

```
# Stack Operations
PUSH_CONST <idx>     # Push constant from pool
PUSH_LOCAL <slot>    # Push local variable
PUSH_GLOBAL <idx>    # Push global variable
POP                  # Discard top of stack
DUP                  # Duplicate top of stack

# Arithmetic
ADD, SUB, MUL, DIV, MOD
NEG                  # Negate

# Comparison
EQ, NE, LT, LE, GT, GE

# Logic
AND, OR, NOT

# Control Flow
JUMP <offset>        # Unconditional jump
JUMP_IF_TRUE <off>   # Conditional jump
JUMP_IF_FALSE <off>  # Conditional jump
LOOP <offset>        # Jump backward (for loops)

# Functions
CALL <nargs>         # Call function on stack
RETURN               # Return from function
CLOSURE <idx>        # Create closure

# Variables
STORE_LOCAL <slot>   # Store to local
STORE_GLOBAL <idx>   # Store to global
LOAD_UPVALUE <idx>   # Load from closure

# Collections
NEW_LIST <size>      # Create list
NEW_MAP <size>       # Create map
INDEX                # collection[key]
INDEX_SET            # collection[key] = val

# Objects
GET_FIELD <idx>      # obj.field
SET_FIELD <idx>      # obj.field = val
CALL_METHOD <idx> <nargs>

# Graph mode switching
ENTER_GRAPH_MODE     # Switch to graph-native execution
EXIT_GRAPH_MODE      # Return to bytecode

# Special
HALT                 # End execution
```

### VM Architecture

```rust
struct VM {
    // Bytecode execution state
    frames: Vec<CallFrame>,
    stack: Vec<Value>,

    // Global namespace (graph-based per Phase 15)
    globals: NamespaceGraph,

    // Graph execution units (for graph-native code)
    graph_units: Vec<GraphExecutionUnit>,

    // Current mode
    mode: ExecutionMode,
}

enum ExecutionMode {
    Bytecode,
    GraphNative,
}
```

### Bytecode File Format (.grc)

```
+--------------------------------------------+
|  Magic: "GRBC"                             |  4 bytes
|  Version: 1                                |  2 bytes
|  Flags: [has_debug, has_source_map]        |  2 bytes
+--------------------------------------------+
|  Constant Pool                             |
|  - Count (u32)                             |
|  - Constants (type-tagged values)          |
+--------------------------------------------+
|  Function Table                            |
|  - Count (u32)                             |
|  - Functions:                              |
|    - Name, arity, flags                    |
|    - Bytecode (for scalar functions)       |
|    - Graph IR (for graph-native functions) |
+--------------------------------------------+
|  Main Chunk                                |
|  - Bytecode or Graph IR reference          |
+--------------------------------------------+
|  Debug Info (optional)                     |
|  - Line number table                       |
|  - Local variable names                    |
|  - Source map                              |
+--------------------------------------------+
```

---

## Integration with Five-Layer Architecture

The compiler must integrate with Graphoid's Five-Layer graph architecture:

```
┌─────────────────────────────────────────────────────────────────┐
│  Layer 5: Effect Layer                                          │
│  - Bytecode: CALL_NATIVE for I/O                               │
│  - Preserved across compilation                                 │
├─────────────────────────────────────────────────────────────────┤
│  Layer 4: Metadata Layer                                        │
│  - Debug info in .grc file                                     │
│  - Source maps for error messages                              │
├─────────────────────────────────────────────────────────────────┤
│  Layer 3: Control Layer (validation rules)                      │
│  - Bytecode: CHECK_RULE instruction                            │
│  - Rules checked at runtime                                    │
├─────────────────────────────────────────────────────────────────┤
│  Layer 2: Behavior Layer (transformations)                      │
│  - Bytecode: APPLY_BEHAVIOR instruction                        │
│  - Behaviors execute on mutation                               │
├─────────────────────────────────────────────────────────────────┤
│  Layer 1: Data Layer                                            │
│  - Standard bytecode operations                                │
│  - Graph-native for graph operations                           │
└─────────────────────────────────────────────────────────────────┘
```

---

## User Commands

```bash
# Compilation
gr build app.gr                    # Compile to bytecode (.grc)
gr build --release app.gr          # Optimized build
gr build --target=native app.gr    # Native binary (Phase 32)
gr build --target=wasm app.gr      # WebAssembly (Phase 31)

# Running compiled code
gr run app.grc                     # Run compiled bytecode
./app                              # Run native binary

# Development (stays interpreted)
gr run app.gr                      # Interpret source directly
gr repl                            # Interactive REPL

# Debugging compiled code
gr build --debug app.gr            # Include debug info
gr disasm app.grc                  # Show bytecode disassembly
```

---

## Performance Expectations

| Benchmark | Interpreted | Compiled | Improvement |
|-----------|-------------|----------|-------------|
| fibonacci(35) | ~10s | ~1-2s | 5-10x |
| Loop 1M iterations | ~2s | ~0.2s | 10x |
| String concat 100K | ~1s | ~0.15s | 6x |
| Graph traverse 10K | ~0.5s | ~0.3s | 1.5x |
| HTTP request | ~100ms | ~100ms | 1x (I/O bound) |
| DB query | ~50ms | ~50ms | 1x (I/O bound) |

**Key insight**: Compilation helps CPU-bound code significantly. I/O-bound code sees minimal improvement.

---

## Implementation Plan

### Week 1: Graph IR

| Day | Task |
|-----|------|
| 1-2 | Define Graph IR node types and structure |
| 3-4 | Implement AST → Graph IR compilation |
| 5 | Basic optimization passes (constant folding, dead code) |

### Week 2: Bytecode Compiler

| Day | Task |
|-----|------|
| 6-7 | Define bytecode instruction set |
| 8-9 | Implement Graph IR → Bytecode (scalar paths) |
| 10 | Graph-native path preservation |

### Week 3: VM Implementation

| Day | Task |
|-----|------|
| 11-12 | Basic VM execution loop |
| 13 | Mode switching (bytecode ↔ graph-native) |
| 14 | Five-Layer integration |

### Week 4: Integration & Testing

| Day | Task |
|-----|------|
| 15-16 | All existing tests pass under compilation |
| 17-18 | Performance benchmarking |
| 19-20 | Debug info and disassembler |
| 21 | Documentation and `gr build` command |

---

## Success Criteria

### Functional
- [ ] Graph IR defined and documented
- [ ] AST → Graph IR compilation works
- [ ] Optimization passes: constant folding, dead code elimination
- [ ] Bytecode instruction set defined
- [ ] Graph IR → Bytecode compilation works
- [ ] VM executes bytecode correctly
- [ ] Dual-path (bytecode + graph-native) works
- [ ] Five-Layer integration complete
- [ ] All existing tests pass when compiled
- [ ] `gr build` command implemented

### Performance
- [ ] 5-10x improvement on CPU-bound benchmarks
- [ ] No regression for I/O-bound code
- [ ] Compilation time < 1s for typical programs

### Developer Experience
- [ ] Debug info preserved in compiled output
- [ ] Error messages show original source locations
- [ ] Bytecode disassembler available
- [ ] `gr build --release` produces optimized output

### Testing
- [ ] 50+ VM-specific unit tests
- [ ] 20+ optimization pass tests
- [ ] 10+ dual-path integration tests
- [ ] Benchmark suite comparing interpreted vs compiled

---

## Limitations to Document

These limitations should be clearly documented for users:

1. **I/O-bound code won't speed up** - If your bottleneck is network/disk, compilation won't help
2. **Dynamic features may differ** - `eval()` in compiled code may fall back to interpreter
3. **Debug builds are slower** - Use `--release` for production performance
4. **Graph operations have modest gains** - The graph engine is already optimized Rust

---

## Open Questions

1. **Compilation heuristics**: How do we decide scalar vs graph-native path? Static analysis? Annotations?

2. **Hot reload**: Can we support hot-reloading compiled code for long-running servers?

3. **Incremental compilation**: Can we recompile only changed functions?

4. **Cross-compilation**: Can we compile on one platform for another?

5. **Size vs speed tradeoffs**: Should `--release` prioritize speed or binary size?

---

## Future Work

### Phase 31: WASM Compilation
- Browser deployment
- Edge computing
- Sandboxed execution

### Phase 32: Native Compilation
- Maximum performance
- Standalone binaries (no runtime needed)
- System programming

### Phase 33: Self-Hosting
- Graphoid compiler written in Graphoid
- Bootstrap complete

---

## Appendix: Design Discussion Context

### The Strategic Question

> "Is bytecode really the right way to go? I see compilation as something more for production applications that require speed and security than as a full time requirement."

### Key Insights

1. **Compilation is for production**: Development stays interpreted for fast iteration and debugging.

2. **Not all code benefits equally**: CPU-bound code sees 5-10x improvement; I/O-bound code sees minimal improvement.

3. **Graphoid is general-purpose**: Like Python (strong in ML) or Perl (strong in text), Graphoid is general-purpose with strengths in graph-centric applications. Both interpreted and compiled modes must work well.

4. **Bytecode first, then native**: Bytecode provides portable speedup with moderate effort. Native compilation is future work for maximum performance.

5. **Dual-path preserves graph advantages**: Scalar code compiles to traditional bytecode; graph code preserves structure for native traversal.

---

## Related Documents

- [GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md](GRAPH_CENTRIC_ARCHITECTURE_RATIONALE.md) - Architecture context
- [PHASE_15_NAMESPACE_GRAPH.md](PHASE_15_NAMESPACE_GRAPH.md) - Namespace as graph (dependency)
- [PHASE_16_EXECUTION_GRAPH.md](PHASE_16_EXECUTION_GRAPH.md) - Execution as graph (dependency)
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Five-Layer architecture (dependency)
- [PHASE_30_WASM_COMPILATION.md](PHASE_30_WASM_COMPILATION.md) - WASM target (future)
- [PHASE_31_NATIVE_COMPILATION.md](PHASE_31_NATIVE_COMPILATION.md) - Native target (future)
- [PHASE_32_SELF_HOSTING.md](PHASE_32_SELF_HOSTING.md) - Self-hosting (future)

---

## References

- Crafting Interpreters (Bob Nystrom) - Bytecode VM design
- Lua 5.x VM - Simple, fast, proven design
- CPython internals - Real-world bytecode VM
- V8 TurboFan - Sea of Nodes IR (graph-based)
