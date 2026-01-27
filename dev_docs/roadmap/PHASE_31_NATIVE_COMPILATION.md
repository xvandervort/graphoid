# Phase 31: Native Compilation

**Duration**: 21-30 days
**Priority**: High (Required for self-hosting)
**Dependencies**: Phase 29 (Compilation Strategy), Phase 20 (FFI for syscalls)
**Status**: Future

---

## Goal

Compile Graphoid programs to native machine code, enabling:
1. **Maximum performance** - No interpretation overhead
2. **Standalone executables** - No runtime dependency
3. **Self-hosting foundation** - Graphoid compiler written in Graphoid
4. **System programming** - Direct syscalls, no libc dependency

---

## Why Native Compilation?

| Execution Mode | Startup Time | Peak Performance | Binary Size | Dependencies |
|----------------|--------------|------------------|-------------|--------------|
| AST Interpreter | Fast | 1x (baseline) | N/A | Full runtime |
| Bytecode VM | Fast | 5-10x | Small (.grc) | VM runtime |
| WASM | Medium | 0.5-0.9x native | Medium (.wasm) | WASM runtime |
| **Native** | Instant | 50-100x | Varies | None (or libc) |

### Self-Hosting Path

```
Current:     Graphoid source -> Rust compiler -> Native binary
Future:      Graphoid source -> Graphoid compiler -> Native binary
End state:   Graphoid is entirely self-sufficient
```

---

## Architecture Options

### Option A: LLVM Backend

```
+--------------+     +--------------+     +--------------+     +--------------+
|   Source     | --> |   Bytecode   | --> |   LLVM IR    | --> |   Native     |
|   (.gr)      |     |   (Phase 29) |     |              |     |   Binary     |
+--------------+     +--------------+     +--------------+     +--------------+
```

**Pros**: Industry-standard optimizations, multiple targets, mature tooling
**Cons**: Large dependency, complex API, not written in Graphoid

### Option B: Cranelift Backend

```
+--------------+     +--------------+     +--------------+     +--------------+
|   Source     | --> |   Bytecode   | --> |  Cranelift   | --> |   Native     |
|   (.gr)      |     |   (Phase 29) |     |     IR       |     |   Binary     |
+--------------+     +--------------+     +--------------+     +--------------+
```

**Pros**: Rust-native, fast compilation, used by wasmtime
**Cons**: Fewer optimizations than LLVM, less mature

### Option C: Custom Backend (Future: Self-Hosted)

```
+--------------+     +--------------+     +--------------+     +--------------+
|   Source     | --> |   Bytecode   | --> |  Custom IR   | --> |   Native     |
|   (.gr)      |     |   (Phase 29) |     |  (Graphoid)  |     |   Binary     |
+--------------+     +--------------+     +--------------+     +--------------+
```

**Pros**: No external dependencies, fully self-hosted
**Cons**: Enormous effort, fewer optimizations

### Recommendation

**This Phase**: Use **Cranelift** (Rust-native, reasonable optimizations)
**Future (Self-Hosting)**: Optionally develop custom backend in Graphoid for full self-hosting

---

## Target Platforms

| Platform | Architecture | Priority | Notes |
|----------|--------------|----------|-------|
| Linux | x86_64 | **High** | Primary development platform |
| Linux | aarch64 | Medium | ARM servers, Raspberry Pi |
| macOS | x86_64 | Medium | Intel Macs |
| macOS | aarch64 | Medium | Apple Silicon |
| Windows | x86_64 | Low | Different ABI, different syscalls |

### Initial Focus: Linux x86_64

Simplest target:
- Well-documented syscall interface
- Stable ABI
- Easy testing

---

## Syscall Integration

### Why Direct Syscalls?

For true self-hosting without libc:

```
With libc:       Graphoid -> libc wrapper -> syscall
Direct:          Graphoid -> syscall (no libc)
```

### Linux x86_64 Syscall Convention

```
Syscall Number:  rax
Arguments:       rdi, rsi, rdx, r10, r8, r9
Return Value:    rax (negative = error)
```

### Essential Syscalls

| Syscall | Number | Purpose |
|---------|--------|---------|
| read | 0 | Read from file descriptor |
| write | 1 | Write to file descriptor |
| open | 2 | Open file |
| close | 3 | Close file descriptor |
| mmap | 9 | Map memory |
| munmap | 11 | Unmap memory |
| brk | 12 | Adjust heap |
| exit | 60 | Exit process |
| exit_group | 231 | Exit all threads |

### Syscall Wrapper Generation

```rust
// Generated code for syscall
fn sys_write(fd: u64, buf: *const u8, count: u64) -> i64 {
    let result: i64;
    unsafe {
        asm!(
            "syscall",
            in("rax") 1u64,      // syscall number
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rax") result,
            out("rcx") _,
            out("r11") _,
        );
    }
    result
}
```

---

## Memory Management

### Option A: Conservative GC with Boehm

Use libgc for automatic memory management:

```rust
// Link against libgc
extern "C" {
    fn GC_malloc(size: usize) -> *mut u8;
    fn GC_init();
}
```

**Pros**: Easy, battle-tested
**Cons**: External dependency

### Option B: Reference Counting

Same as WASM approach:

```rust
struct RcValue {
    refcount: u32,
    value: Value,
}
```

**Pros**: Predictable, no pauses
**Cons**: Cycles leak (need cycle detector)

### Option C: Custom Tracing GC (Self-Hosted Goal)

Write GC in Graphoid:

```graphoid
# In Graphoid (future)
fn gc_collect() {
    mark_roots()
    sweep_heap()
}
```

**Pros**: Full control, self-hosted
**Cons**: Complex, significant effort

### Recommendation

**This Phase**: Reference counting (simple, works for most cases)
**Future (Self-Hosting)**: Cycle detector or tracing GC written in Graphoid

---

## Code Generation

### Cranelift Example

```rust
use cranelift::prelude::*;
use cranelift_module::{Module, Linkage};

fn compile_function(func: &Function) -> CompiledCode {
    let mut ctx = codegen::Context::new();
    let mut func_ctx = FunctionBuilderContext::new();

    // Create function signature
    let mut sig = Signature::new(CallConv::SystemV);
    for _ in 0..func.arity {
        sig.params.push(AbiParam::new(types::I64));
    }
    sig.returns.push(AbiParam::new(types::I64));

    ctx.func = ir::Function::with_name_signature(
        ExternalName::user(0, 0),
        sig,
    );

    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);
    let entry_block = builder.create_block();
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    // Generate code from bytecode
    for instruction in &func.bytecode {
        match instruction {
            Op::Add => {
                let b = builder.ins().pop();
                let a = builder.ins().pop();
                let result = builder.ins().iadd(a, b);
                builder.ins().push(result);
            }
            // ... other instructions
        }
    }

    builder.finalize();

    // Compile to machine code
    let code = ctx.compile(&*isa).unwrap();
    code
}
```

---

## Graph Operations in Native Code

Graphoid's graph-centric nature requires efficient native code generation for graph operations.

### Graph Representation

Graphs are heap-allocated structures with the same logical layout as WASM, but using native pointers:

```c
// Native graph structure (generated by compiler)
struct Graph {
    uint32_t graph_type;
    uint32_t node_count;
    uint32_t edge_count;
    uint32_t flags;
    NodeTable* nodes;
    EdgeTable* edges;
    IndexStructure* index;
};

struct Node {
    char* id;
    Value value;
    EdgeList* outgoing;
    EdgeList* incoming;
};

struct Edge {
    char* from_id;
    char* to_id;
    char* label;
    Value data;
};
```

### Graph Runtime Functions

Graph operations compile to calls to runtime functions:

```rust
// Cranelift codegen for graph operations
fn compile_graph_traverse(builder: &mut FunctionBuilder, graph: Value, from: Value, opts: Value) -> Value {
    // Call runtime function
    let func_ref = builder.import_function(ExternalName::user(0, FUNC_GRAPH_TRAVERSE));
    let call = builder.ins().call(func_ref, &[graph, from, opts]);
    builder.inst_results(call)[0]
}
```

Runtime library (eventually in Graphoid):

```graphoid
# runtime/graph.gr - Graph runtime functions

fn __gr_graph_new(graph_type) {
    ptr = alloc(sizeof_graph)
    ptr.graph_type = graph_type
    ptr.node_count = 0
    ptr.edge_count = 0
    ptr.nodes = node_table_new()
    ptr.edges = edge_table_new()
    ptr.index = index_new()
    return ptr
}

fn __gr_graph_traverse(graph, from, opts) {
    results = []
    visited = set{}
    queue = [from]

    while queue.length() > 0 {
        current = queue.shift()
        if visited.contains(current) { continue }
        visited.add(current)
        results.append(current)

        for edge in graph.edges_from(current) {
            if opts.matches_edge(edge) {
                queue.append(edge.to)
            }
        }
    }

    return results
}

fn __gr_graph_query(graph, pattern) {
    # Pattern matching on graph structure
    matches = []
    for node in graph.nodes() {
        if pattern.matches_node(node) {
            matches.append(node)
        }
    }
    return matches
}
```

### Dual-Path in Native Compilation

Phase 29's dual-path model applies to native compilation:

| Code Type | Native Strategy |
|-----------|-----------------|
| Scalar (loops, math) | Direct Cranelift IR → machine instructions |
| Graph operations | Calls to graph runtime functions |

```
┌─────────────────────────────────────────────────────────────────────┐
│  Graph IR (from Phase 29)                                           │
│      │                                                              │
│      ├── Scalar code → Cranelift IR → Native instructions           │
│      │   (i64.add, branches, etc.)                                  │
│      │                                                              │
│      └── Graph code → Calls to __gr_graph_* functions               │
│          (traverse, query, mutate)                                  │
│                                                                     │
│  Graph runtime is:                                                  │
│  - Linked into executable (static)                                  │
│  - Or shared library (dynamic)                                      │
│  - Eventually written in Graphoid itself (self-hosting)             │
└─────────────────────────────────────────────────────────────────────┘
```

### Optimization Opportunities

Native compilation enables graph-specific optimizations:

1. **Inline small graphs** - Graphs with < N nodes can be stack-allocated
2. **Specialize traversals** - Known traversal patterns can be unrolled
3. **Cache-friendly layout** - Node/edge data laid out for cache efficiency
4. **SIMD for batch operations** - Process multiple nodes in parallel

```rust
// Example: Specialized traversal for known depth
fn compile_traverse_depth_1(builder: &mut FunctionBuilder, graph: Value, from: Value) -> Value {
    // Instead of calling generic traverse, inline the single-hop case
    let edges = builder.ins().call(get_edges_from, &[graph, from]);
    let destinations = builder.ins().call(extract_destinations, &[edges]);
    destinations
}
```

---

## Five-Layer Architecture in Native Code

Graphoid's Five-Layer graph architecture compiles to native code structures.

### Layer Compilation Strategy

```
┌─────────────────────────────────────────────────────────────────────┐
│  Five-Layer Integration in Native Code                              │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 5: Effect Layer                                      │    │
│  │  - Direct syscalls (no libc wrapper)                        │    │
│  │  - Effect logging via runtime functions                     │    │
│  │  - I/O operations as inlined syscall sequences              │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 4: Metadata Layer                                    │    │
│  │  - DWARF debug info for source mapping                      │    │
│  │  - Optional: operation history in debug builds              │    │
│  │  - Stripped in release builds for size                      │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 3: Control Layer (validation rules)                  │    │
│  │  - Rules compiled as native functions                       │    │
│  │  - Called before mutations (inlined when possible)          │    │
│  │  - Rule table for dynamic rules                             │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 2: Behavior Layer (transformations)                  │    │
│  │  - Behaviors compiled as native functions                   │    │
│  │  - Function pointer table for dynamic behaviors             │    │
│  │  - Inlined for known static behaviors                       │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 1: Data Layer                                        │    │
│  │  - Native memory layout for values                          │    │
│  │  - Graph structures as described above                      │    │
│  │  - Direct register/memory operations for scalars            │    │
│  └─────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

### Behavior Compilation

Behaviors compile to native functions with a function pointer table:

```rust
// Behavior table structure
struct BehaviorTable {
    count: u32,
    behaviors: [*fn(Value) -> Value],  // Function pointers
}

// Generated code for applying behaviors
fn __gr_apply_behaviors(value: Value, mutation_type: u32) -> Value {
    let table = get_behavior_table(value);
    let mut current = value;

    for i in 0..table.count {
        let behavior_fn = table.behaviors[i];
        current = behavior_fn(current);
    }

    current
}
```

When behaviors are statically known, they can be inlined:

```rust
// Before (dynamic):
let result = __gr_apply_behaviors(value, MUTATION_APPEND);

// After optimization (static behaviors known):
let result = behavior_none_to_zero(behavior_validate_range(value));
```

### Rule Validation

Rules are checked before mutations:

```rust
// Rule checking function
fn __gr_check_rules(value: Value, mutation: u32, new_value: Value) -> bool {
    let table = get_rule_table(value);

    for i in 0..table.count {
        let rule_fn = table.rules[i];
        if !rule_fn(value, mutation, new_value) {
            return false;  // Rule violation
        }
    }

    true
}

// Generated mutation code
fn list_append(list: Value, item: Value) -> Value {
    // Check rules
    if !__gr_check_rules(list, MUTATION_APPEND, item) {
        panic_rule_violation();
    }

    // Perform mutation
    list_append_unchecked(list, item);

    // Apply behaviors
    __gr_apply_behaviors(list, MUTATION_APPEND)
}
```

### Debug Info (DWARF)

For debugging support, generate DWARF info:

```rust
// Cranelift debug info generation
fn emit_debug_info(func: &CompiledFunction, source_map: &SourceMap) -> DwarfInfo {
    let mut dwarf = DwarfInfo::new();

    // Map native addresses to source locations
    for (addr, loc) in &func.address_map {
        if let Some(source_loc) = source_map.get(loc) {
            dwarf.add_line_info(addr, source_loc.file, source_loc.line);
        }
    }

    dwarf
}
```

---

## Executable Format

### ELF (Linux)

```
+---------------------------------------------+
|  ELF Header                                 |
|  - Magic: 0x7F "ELF"                        |
|  - Class: 64-bit                            |
|  - Endianness: Little                       |
|  - Entry point                              |
+---------------------------------------------+
|  Program Headers                            |
|  - PT_LOAD (code)                           |
|  - PT_LOAD (data)                           |
+---------------------------------------------+
|  .text (code)                               |
|  - _start                                   |
|  - Compiled functions                       |
+---------------------------------------------+
|  .rodata (constants)                        |
|  - String literals                          |
|  - Numeric constants                        |
+---------------------------------------------+
|  .data (initialized globals)                |
+---------------------------------------------+
|  .bss (uninitialized globals)               |
+---------------------------------------------+
```

### Minimal _start

```asm
; Entry point - no libc
_start:
    ; Set up stack frame
    xor rbp, rbp

    ; Get argc, argv from stack
    pop rdi                 ; argc
    mov rsi, rsp            ; argv

    ; Call main
    call graphoid_main

    ; Exit with return value
    mov rdi, rax
    mov rax, 60             ; sys_exit
    syscall
```

---

## Runtime Library

Even without libc, some runtime support is needed:

### Minimal Runtime (in Assembly/Rust, eventually Graphoid)

```
runtime/
+-- _start.s          # Entry point
+-- syscalls.s        # Syscall wrappers
+-- memory.gr         # Allocator (in Graphoid!)
+-- strings.gr        # String operations
+-- io.gr             # I/O primitives
+-- panic.gr          # Panic handler
```

### Runtime Functions

```graphoid
# runtime/memory.gr (written in Graphoid)
heap_start = 0
heap_end = 0

fn init_heap() {
    heap_start = syscall.brk(0)
    heap_end = heap_start
}

fn alloc(size) {
    aligned = align(size, 8)
    ptr = heap_end
    heap_end = syscall.brk(heap_end + aligned)
    return ptr
}

fn free(ptr) {
    # Reference counting handles this
}
```

---

## CLI Commands

```bash
# Compile to native executable
graphoid compile --target native program.gr -o program

# Compile with optimizations
graphoid compile --target native -O2 program.gr -o program

# Compile without libc (direct syscalls)
graphoid compile --target native --no-libc program.gr -o program

# Cross-compile
graphoid compile --target native --arch aarch64 program.gr -o program

# Generate assembly (for debugging)
graphoid compile --target native --emit asm program.gr -o program.s

# Static vs dynamic linking
graphoid compile --target native --static program.gr -o program
```

---

## Implementation Plan

### Week 1-2: Cranelift Integration

| Day | Task |
|-----|------|
| 1-2 | Integrate Cranelift, basic setup |
| 3-4 | Value representation in native code |
| 5-6 | Bytecode -> Cranelift IR (expressions) |
| 7 | Arithmetic, comparisons |

### Week 2-3: Control Flow & Functions

| Day | Task |
|-----|------|
| 8-9 | Control flow (if, while, for) |
| 10-11 | Functions, calling convention |
| 12-13 | Closures, upvalues |
| 14 | Stack management |

### Week 3-4: Memory & Collections

| Day | Task |
|-----|------|
| 15-16 | Memory allocator (reference counting) |
| 17-18 | Collections (list, map) |
| 19-20 | Strings |
| 21 | Graph basics |

### Week 4-5: System Integration

| Day | Task |
|-----|------|
| 22-23 | ELF output generation |
| 24-25 | Syscall wrappers (Linux) |
| 26-27 | Runtime library |
| 28 | CLI integration |

### Week 5: Polish

| Day | Task |
|-----|------|
| 29 | Testing, debugging |
| 30 | Documentation |

---

## Success Criteria

### Core Functionality
- [ ] Compile to native x86_64 executable
- [ ] All core language features work
- [ ] Reference counting memory management
- [ ] Direct syscall support (Linux)
- [ ] No libc dependency option
- [ ] ELF executable generation

### Graph Operations
- [ ] Graph creation and manipulation in native code
- [ ] Graph traversal via runtime functions
- [ ] Graph queries compile and execute correctly
- [ ] Dual-path compilation (scalar → native, graph → runtime calls)

### Five-Layer Architecture
- [ ] Behaviors compile to native functions
- [ ] Rules checked before mutations
- [ ] Behavior/rule function pointer tables work
- [ ] DWARF debug info generation

### Quality
- [ ] Performance within 10% of hand-written C
- [ ] At least 70 native compilation tests
- [ ] Cross-compilation support (at least x86_64 + aarch64)
- [ ] Documentation complete

---

## Performance Targets

| Benchmark | Bytecode VM | Native | Target |
|-----------|-------------|--------|--------|
| Fibonacci(40) | ~1s | ~0.01s | 100x faster |
| Loop 100M | ~5s | ~0.1s | 50x faster |
| String ops | ~2s | ~0.2s | 10x faster |
| Graph traversal | ~1s | ~0.1s | 10x faster |

---

## Open Questions

1. **Optimization level**: How much optimization in this phase vs defer to later?
2. **Debug info**: DWARF generation for debugger support? (Now addressed in Five-Layer section)
3. **Dynamic linking**: Support shared libraries?
4. **Windows support**: Different ABI, different syscalls - defer?
5. **Graph runtime**: Link statically or dynamically? Written in Rust initially, Graphoid later?

---

## Related Documents

- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Compilation strategy, Graph IR, dual-path model
- [PHASE_30_WASM_COMPILATION.md](PHASE_30_WASM_COMPILATION.md) - Alternative WASM target
- [PHASE_32_SELF_HOSTING.md](PHASE_32_SELF_HOSTING.md) - Compiler in Graphoid (uses this phase)
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Five-Layer architecture
- [PHASE_20_FFI.md](PHASE_20_FFI.md) - Syscall access

---

## References

- [Cranelift Documentation](https://cranelift.dev/)
- [System V AMD64 ABI](https://refspecs.linuxbase.org/elf/x86_64-abi-0.99.pdf)
- [Linux Syscall Table](https://filippo.io/linux-syscall-table/)
- [ELF Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
- [Writing a Compiler Backend (Tutorial)](https://norasandler.com/2017/11/29/Write-a-Compiler.html)
