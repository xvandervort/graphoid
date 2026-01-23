# Phase 30: WebAssembly Compilation

**Duration**: 14-21 days
**Priority**: High (Enables safe distributed execution)
**Dependencies**: Phase 29 (Compilation Strategy)
**Status**: Future

---

## Goal

Compile Graphoid programs to WebAssembly, enabling:
1. **Sandboxed plugin execution** - Run untrusted code safely
2. **Browser deployment** - Graphoid in web applications
3. **Portable distribution** - Single .wasm works everywhere
4. **Safe distributed execution** - Critical for Phase 23-24

---

## Why WebAssembly?

| Feature | Benefit for Graphoid |
|---------|---------------------|
| **Memory-safe sandbox** | Untrusted plugins can't crash host |
| **Portable** | Same .wasm runs on Linux, macOS, Windows, browsers |
| **Near-native speed** | 0.5-0.9x native performance |
| **Capability-based security** | Explicit permissions via WASI |
| **Industry standard** | Wide tooling support |

### Use Cases

```graphoid
# 1. Run untrusted plugin safely
plugin = wasm.load("community_plugin.wasm")
result = plugin.transform(data)  # Can't access filesystem

# 2. Compile and distribute
graphoid compile --target wasm app.gr -o app.wasm
# Share app.wasm - runs anywhere

# 3. Browser execution
<script type="module">
  import { Graphoid } from './graphoid-runtime.js';
  const result = await Graphoid.run('app.wasm');
</script>

# 4. Distributed execution (Phase 23-24)
cluster.submit("task.wasm", data)  # Safe remote execution
```

---

## Architecture

### Compilation Pipeline

```
+--------------+     +--------------+     +--------------+     +--------------+
|   Source     | --> |   Bytecode   | --> |    WASM      | --> |   .wasm      |
|   (.gr)      |     |   (Phase 29) |     |   Codegen    |     |   binary     |
+--------------+     +--------------+     +--------------+     +--------------+
```

### WASM Module Structure

```
+-------------------------------------------------------------+
|                    Graphoid WASM Module                      |
+-------------------------------------------------------------+
|  Imports                                                     |
|  - wasi_snapshot_preview1 (I/O, clock, random)              |
|  - graphoid_runtime (GC, graph ops)                         |
+-------------------------------------------------------------+
|  Memory                                                      |
|  - Linear memory (heap, stack)                              |
|  - Graphoid values, strings, collections                    |
+-------------------------------------------------------------+
|  Functions                                                   |
|  - User-defined functions                                   |
|  - Runtime support functions                                |
+-------------------------------------------------------------+
|  Exports                                                     |
|  - _start (entry point)                                     |
|  - Public functions                                         |
|  - Memory (for host inspection)                             |
+-------------------------------------------------------------+
```

---

## WASM Runtime Options

### Option A: wasmtime (Recommended)

```rust
// Cargo.toml
[dependencies]
wasmtime = "17"
wasmtime-wasi = "17"
```

**Pros**: Rust-native, fast, excellent WASI support, Cranelift JIT
**Cons**: Larger binary size

### Option B: wasmer

```rust
// Cargo.toml
[dependencies]
wasmer = "4"
wasmer-wasi = "4"
```

**Pros**: Multiple backends (Cranelift, LLVM, Singlepass), universal binaries
**Cons**: More complex API

### Recommendation

Use **wasmtime** for:
- Native Rust integration
- Best-in-class WASI support
- Cranelift backend (same as Rust uses)
- Active Bytecode Alliance development

---

## Value Representation in WASM

WASM has limited types (i32, i64, f32, f64). Graphoid values need encoding:

### Tagged Values (NaN-boxing)

```
64-bit value layout:
+-------------------------------------------------------------+
|  If top 13 bits are 0x7FF8: pointer or special value        |
|  Otherwise: IEEE 754 double (num)                           |
+-------------------------------------------------------------+

Special values:
0x7FF8_0000_0000_0000  = none
0x7FF8_0000_0000_0001  = true
0x7FF8_0000_0000_0002  = false
0x7FF8_XXXX_XXXX_XXXX  = pointer (lower 48 bits)
```

### Alternative: Boxed Values

```rust
// Every value is a pointer to heap allocation
struct Value {
    tag: u8,      // Type tag
    data: [u8],   // Type-specific data
}
```

---

## Memory Management

### Option A: Linear Allocator (Simple)

```rust
// Bump allocator - fast but no deallocation
static mut HEAP_PTR: u32 = HEAP_START;

fn alloc(size: u32) -> u32 {
    let ptr = HEAP_PTR;
    HEAP_PTR += align(size);
    ptr
}
```

### Option B: Reference Counting

```rust
struct RcValue {
    refcount: u32,
    value: Value,
}

fn incref(ptr: u32) {
    let rc = ptr as *mut RcValue;
    (*rc).refcount += 1;
}

fn decref(ptr: u32) {
    let rc = ptr as *mut RcValue;
    (*rc).refcount -= 1;
    if (*rc).refcount == 0 {
        free(ptr);
    }
}
```

### Option C: Tracing GC (Complex)

Mark-and-sweep or copying collector. More complex but handles cycles.

### Recommendation

Start with **reference counting** (Option B):
- Simple to implement
- Predictable performance
- Works for most cases
- Can add cycle detection later

---

## WASI Integration

WASI (WebAssembly System Interface) provides portable system access:

```graphoid
# These work via WASI imports
print("hello")           # fd_write to stdout
content = fs.read(path)  # path_open, fd_read
time = time.now()        # clock_time_get
r = random.random()      # random_get
```

### WASI Capability Model

```graphoid
# Host controls what WASM module can access
runtime = wasm.runtime({
    stdin: true,
    stdout: true,
    stderr: true,
    filesystem: ["/data:ro", "/output:rw"],  # Mapped directories
    network: false,                           # No network access
    env: ["DEBUG=1"],                         # Environment variables
})

result = runtime.run("untrusted.wasm")
```

---

## Code Generation

### Bytecode to WASM Translation

```
Graphoid Bytecode          WASM Instructions
-----------------          -----------------
PUSH_CONST 5               i64.const <encoded-5>

ADD                        call $__gr_add

PUSH_LOCAL 0               local.get 0

STORE_LOCAL 1              local.set 1

CALL 2                     call $func_name

JUMP_IF_FALSE 10           call $__gr_is_falsy
                           br_if 0

NEW_LIST 3                 i32.const 3
                           call $__gr_list_new
```

### Runtime Functions (Imported or Embedded)

```wat
;; Graphoid runtime functions in WASM
(func $__gr_add (param $a i64) (param $b i64) (result i64)
  ;; Type check, then add
  ...
)

(func $__gr_list_new (param $size i32) (result i64)
  ;; Allocate list, return pointer as tagged value
  ...
)

(func $__gr_list_get (param $list i64) (param $idx i64) (result i64)
  ;; Bounds check, return element
  ...
)
```

---

## Graph Operations in WASM

Graphoid's graph-centric nature requires special consideration for WASM compilation.

### Graph Representation in WASM

Graphs are represented as heap-allocated structures in WASM linear memory:

```
Graph Layout in WASM Memory:
+------------------------------------------+
|  Header (16 bytes)                       |
|  - graph_type: u32                       |
|  - node_count: u32                       |
|  - edge_count: u32                       |
|  - flags: u32                            |
+------------------------------------------+
|  Node Table (ptr)                        |
|  - Array of node entries                 |
|  - Each: id_ptr, value_ptr, edge_list    |
+------------------------------------------+
|  Edge Table (ptr)                        |
|  - Array of edge entries                 |
|  - Each: from_id, to_id, label_ptr, data |
+------------------------------------------+
|  Index Structures (ptr)                  |
|  - Hash table for node lookup            |
|  - Adjacency lists for traversal         |
+------------------------------------------+
```

### Graph Runtime Functions

```wat
;; Graph operations implemented as WASM functions

(func $__gr_graph_new (param $type i32) (result i64)
  ;; Allocate graph structure, return tagged pointer
)

(func $__gr_graph_add_node (param $graph i64) (param $id i64) (param $value i64) (result i64)
  ;; Add node to graph, return node reference
)

(func $__gr_graph_add_edge (param $graph i64) (param $from i64) (param $to i64) (param $label i64) (result i64)
  ;; Add edge to graph
)

(func $__gr_graph_traverse (param $graph i64) (param $from i64) (param $opts i64) (result i64)
  ;; Traverse graph, return result list
  ;; This is the core graph operation - must be efficient
)

(func $__gr_graph_query (param $graph i64) (param $pattern i64) (result i64)
  ;; Pattern matching query on graph
)
```

### Dual-Path Consideration

Phase 29 introduced dual-path execution (scalar bytecode vs graph-native). In WASM:

| Code Type | WASM Strategy |
|-----------|---------------|
| Scalar (loops, math) | Standard WASM instructions |
| Graph operations | Runtime function calls |

**Key insight**: WASM itself is scalar (no native graph support), so graph operations compile to runtime function calls. However, the runtime functions are optimized for graph operations.

```
┌─────────────────────────────────────────────────────────────────────┐
│  Graphoid Source                                                    │
│      │                                                              │
│      ▼                                                              │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Graph IR (Phase 29)                                        │    │
│  └─────────────────────────────────────────────────────────────┘    │
│      │                                                              │
│      ├── Scalar code → WASM instructions (i32.add, local.set, etc.) │
│      │                                                              │
│      └── Graph code → WASM calls to $__gr_graph_* functions         │
│                                                                     │
│  The graph runtime library is either:                               │
│  - Embedded in the .wasm module (standalone)                        │
│  - Imported from host (when running under Graphoid runtime)         │
└─────────────────────────────────────────────────────────────────────┘
```

### Standalone vs Hosted Mode

| Mode | Graph Runtime | Use Case |
|------|---------------|----------|
| **Standalone** | Embedded in .wasm | Browser, portable distribution |
| **Hosted** | Imported from host | Plugins, distributed execution |

```graphoid
# Standalone: graph runtime compiled into WASM
graphoid compile --target wasm --standalone app.gr -o app.wasm

# Hosted: graph runtime provided by host
graphoid compile --target wasm --hosted app.gr -o app.wasm
# Smaller .wasm, but requires Graphoid runtime to execute
```

---

## Five-Layer Architecture in WASM

Graphoid's Five-Layer graph architecture must be preserved in WASM modules.

### Layer Compilation Strategy

```
┌─────────────────────────────────────────────────────────────────────┐
│  Five-Layer Integration in WASM                                     │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 5: Effect Layer                                      │    │
│  │  - WASI calls for I/O                                       │    │
│  │  - Host imports for external effects                        │    │
│  │  - Effect logging via runtime functions                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 4: Metadata Layer                                    │    │
│  │  - Debug info in custom WASM sections                       │    │
│  │  - Source maps for error messages                           │    │
│  │  - Optional: operation history (if enabled)                 │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 3: Control Layer (validation rules)                  │    │
│  │  - Rules compiled as WASM functions                         │    │
│  │  - Called before mutations: $__gr_check_rules               │    │
│  │  - Rule violations trap or return error                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 2: Behavior Layer (transformations)                  │    │
│  │  - Behaviors compiled as WASM functions                     │    │
│  │  - Called after mutations: $__gr_apply_behaviors            │    │
│  │  - Transformation chain in behavior table                   │    │
│  └─────────────────────────────────────────────────────────────┘    │
│                              │                                      │
│  ┌─────────────────────────────────────────────────────────────┐    │
│  │  Layer 1: Data Layer                                        │    │
│  │  - Values in WASM linear memory                             │    │
│  │  - Graph structures as described above                      │    │
│  │  - Standard WASM operations for scalars                     │    │
│  └─────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

### Behavior Compilation

Behaviors are compiled to WASM functions and stored in a behavior table:

```wat
;; Behavior table in WASM
(table $behaviors 100 funcref)

;; Apply behaviors after mutation
(func $__gr_apply_behaviors (param $value i64) (param $mutation i32) (result i64)
  ;; Get behavior list for this value
  ;; Call each behavior function in order
  ;; Return transformed value
  (local $i i32)
  (local $behavior_count i32)
  (local $current i64)

  (local.set $current (local.get $value))
  (local.set $behavior_count (call $__gr_get_behavior_count (local.get $value)))

  (block $done
    (loop $next
      (br_if $done (i32.ge_u (local.get $i) (local.get $behavior_count)))

      ;; Call behavior function from table
      (local.set $current
        (call_indirect (type $behavior_sig)
          (local.get $current)
          (call $__gr_get_behavior (local.get $value) (local.get $i))))

      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br $next)
    )
  )

  (local.get $current)
)
```

### Rule Validation

Rules are checked before mutations:

```wat
(func $__gr_check_rules (param $value i64) (param $mutation i32) (param $new_value i64) (result i32)
  ;; Returns 1 if allowed, 0 if rejected
  ;; Similar structure to behavior application
)

;; Example: validated mutation
(func $__gr_list_append (param $list i64) (param $item i64) (result i64)
  ;; Check rules first
  (if (i32.eqz (call $__gr_check_rules (local.get $list) (i32.const 1) (local.get $item)))
    (then
      ;; Rule violation - trap or return error
      (call $__gr_raise_rule_violation)
      (unreachable)
    )
  )

  ;; Perform mutation
  (call $__gr_list_append_unchecked (local.get $list) (local.get $item))

  ;; Apply behaviors
  (call $__gr_apply_behaviors (local.get $list) (i32.const 1))
)
```

---

## Browser Runtime

### JavaScript Glue

```javascript
// graphoid-runtime.js
export class Graphoid {
  static async load(wasmPath) {
    const response = await fetch(wasmPath);
    const bytes = await response.arrayBuffer();

    const imports = {
      wasi_snapshot_preview1: {
        fd_write: (fd, iovs, iovsLen, nwritten) => { /* ... */ },
        // ... other WASI functions
      },
      graphoid: {
        print: (ptr, len) => {
          const str = this.readString(ptr, len);
          console.log(str);
        },
      },
    };

    const { instance } = await WebAssembly.instantiate(bytes, imports);
    return new Graphoid(instance);
  }

  run() {
    this.instance.exports._start();
  }

  call(name, ...args) {
    const fn = this.instance.exports[name];
    return fn(...args);
  }
}
```

### Usage in Browser

```html
<!DOCTYPE html>
<html>
<head>
  <script type="module">
    import { Graphoid } from './graphoid-runtime.js';

    async function main() {
      const gr = await Graphoid.load('app.wasm');
      const result = gr.call('process_data', inputData);
      document.getElementById('output').textContent = result;
    }

    main();
  </script>
</head>
<body>
  <div id="output"></div>
</body>
</html>
```

---

## CLI Commands

```bash
# Compile to WASM
graphoid compile --target wasm program.gr -o program.wasm

# Compile with optimization
graphoid compile --target wasm -O2 program.gr -o program.wasm

# Run WASM module
graphoid run program.wasm

# Run with WASI permissions
graphoid run --allow-read=/data --allow-write=/output program.wasm

# Inspect WASM module
graphoid wasm inspect program.wasm
```

---

## Implementation Plan

### Week 1: WASM Foundation

| Day | Task |
|-----|------|
| 1-2 | Integrate wasmtime, basic module loading |
| 3-4 | Value representation (NaN-boxing or boxed) |
| 5 | Memory allocator (reference counting) |

### Week 2: Code Generation

| Day | Task |
|-----|------|
| 6-7 | Bytecode -> WASM translation (expressions) |
| 8-9 | Control flow, functions |
| 10 | Collections (list, map basics) |

### Week 3: Runtime & Integration

| Day | Task |
|-----|------|
| 11-12 | WASI integration (I/O, filesystem) |
| 13-14 | CLI commands (compile, run) |
| 15 | Browser runtime, JavaScript glue |

### Week 4: Polish

| Day | Task |
|-----|------|
| 16-17 | Graph operations in WASM |
| 18-19 | Testing, optimization |
| 20-21 | Documentation, examples |

---

## Success Criteria

### Core Functionality
- [ ] Compile Graphoid source to .wasm files
- [ ] Run .wasm modules via CLI
- [ ] All core language features work in WASM
- [ ] WASI integration (stdin/stdout, filesystem, clock)
- [ ] Browser runtime with JavaScript glue
- [ ] Capability-based sandboxing
- [ ] Plugin loading from WASM

### Graph Operations
- [ ] Graph creation and manipulation in WASM
- [ ] Graph traversal via runtime functions
- [ ] Graph queries compile and execute correctly
- [ ] Standalone and hosted modes both work

### Five-Layer Architecture
- [ ] Behaviors compile to WASM functions
- [ ] Rules checked before mutations
- [ ] Behavior/rule tables work correctly
- [ ] Debug info in custom WASM sections

### Quality
- [ ] At least 60 WASM-specific tests
- [ ] Performance within 2x of native bytecode VM
- [ ] Documentation complete

---

## Security Model

### Sandboxing Guarantees

| Threat | Protection |
|--------|------------|
| Memory corruption | WASM linear memory isolation |
| Filesystem access | WASI capability tokens |
| Network access | Explicit permission required |
| Infinite loops | Fuel metering (instruction limits) |
| Memory exhaustion | Memory limits configurable |

### Example: Untrusted Plugin

```graphoid
# Load untrusted plugin with minimal permissions
plugin = wasm.load("untrusted.wasm", {
    memory_limit: 10_mb,
    fuel: 1_000_000,        # Instruction limit
    filesystem: false,
    network: false,
})

try {
    result = plugin.process(data)
} catch WasmTrap as e {
    print("Plugin failed: " + e.message)
}
```

---

## Future Extensions

### WASM GC (Garbage Collection)

When WASM GC proposal stabilizes:
- Native reference types
- Struct and array types
- Host-managed GC

### WASM Threads

When threading is needed:
- SharedArrayBuffer for browser
- atomics.* operations
- Parallel graph algorithms

### Component Model

When WASM Component Model stabilizes:
- Better inter-module communication
- Interface types
- Virtualization

---

## Related Documents

- [PHASE_29_COMPILATION_STRATEGY.md](PHASE_29_COMPILATION_STRATEGY.md) - Compilation strategy, Graph IR, dual-path model
- [PHASE_31_NATIVE_COMPILATION.md](PHASE_31_NATIVE_COMPILATION.md) - Alternative native target
- [PHASE_32_SELF_HOSTING.md](PHASE_32_SELF_HOSTING.md) - Self-hosting goal
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Five-Layer architecture
- [PHASE_23_DISTRIBUTED_PRIMITIVES.md](PHASE_23_DISTRIBUTED_PRIMITIVES.md) - Uses WASM for safe remote execution
- [PHASE_24_DISTRIBUTED_EXECUTION.md](PHASE_24_DISTRIBUTED_EXECUTION.md) - Distributed graph algorithms

---

## References

- [WebAssembly Specification](https://webassembly.github.io/spec/)
- [WASI Specification](https://wasi.dev/)
- [wasmtime Documentation](https://docs.wasmtime.dev/)
- [Bytecode Alliance](https://bytecodealliance.org/)
