# Phase 20: Foreign Function Interface (FFI)

**Duration**: 18-24 days
**Priority**: Critical
**Dependencies**: Phase 18 (Complete Graph Model) for full integration; can start core mechanics earlier
**Status**: Ready to Begin

---

## Goal

Enable Graphoid programs to call native code while maintaining graph-centric principles:

1. **Foreign Realm** - Isolated space for foreign entities, connected via bridge nodes
2. **C FFI** - Dynamic loading of C libraries with full provenance tracking
3. **Rust Plugins** - Safe, typed plugins for performance-critical extensions
4. **Effect Integration** - All foreign calls tracked as effects (Phase 18 integration)

**Key Principle**: Foreign objects aren't graphs (just like numbers aren't graphs), but they ARE nodes in graphs. The Foreign Realm model makes the boundary explicit while preserving full graph utility.

---

## Design Philosophy

### Why Foreign Realm?

Graphoid claims "everything is a graph." Foreign objects challenge this:
- We can't introspect C memory like we can Graphoid values
- Foreign code can mutate data without Graphoid knowing
- Pointers can dangle, double-free, corrupt memory

**The honest answer**: Foreign values are NOT graphs - they're opaque binary data. But they can be **nodes** in graphs, just like the number `42` is a node, not a graph.

The Foreign Realm model:
1. **Preserves graph semantics** - Bridge nodes ARE real graph nodes
2. **Makes boundaries explicit** - You can see what's foreign vs. native
3. **Enables auditing** - Track provenance, detect leaks, audit calls
4. **Maintains honesty** - We don't pretend to know what's inside foreign memory

### FFI as Scaffolding

FFI is **temporary scaffolding** toward self-hosting. Every FFI use should have a documented pure Graphoid path:

| Current FFI Use | Pure Graphoid Path |
|-----------------|-------------------|
| libsqlite3 | Graphoid SQLite protocol implementation |
| libpq (PostgreSQL) | Graphoid PostgreSQL protocol |
| zlib | Pure Graphoid DEFLATE |
| OpenSSL | Already done! TLS 1.3 in pure Graphoid |
| libc | Direct syscalls via syscall module |

---

## Security and Safety

### The Fundamental Limitation

Once we call into foreign code, **we lose control**. We cannot:
- See what foreign code does with memory
- Prevent it from accessing the file system
- Stop it from making network connections
- Control its resource consumption

We CAN control:
- **Whether** a library loads
- **Whether** a function is called
- **What arguments** we pass
- **Observe** what happened (via effects and bridge state)

This means FFI is inherently unsafe. Our security model makes risks **visible and opt-in** rather than pretending we can eliminate them.

### Import Warning (Default Behavior)

FFI is considered an **unsafe import**. When a program imports `ffi`, a warning is emitted:

```graphoid
import "ffi"  # WARNING: Unsafe import 'ffi' - foreign code cannot be sandboxed
```

This warning:
- Alerts developers that the program uses foreign code
- Appears at compile/load time, not buried in runtime
- Can be acknowledged explicitly to suppress:

```graphoid
import "ffi" unsafe  # No warning - developer acknowledges the risk
```

**Rationale**: For now, a warning is sufficient. Future frameworks will provide richer configuration (library whitelists, capability grants, etc.). We don't over-specify configuration until frameworks exist.

### Taint Tracking

Data originating from foreign code is **tainted**. This is metadata attached to values, similar to frozen state tracking.

```graphoid
# Foreign call returns tainted data
result = sqlite.sqlite3_column_text(stmt, 0)

# Check taint
reflect.tainted(result)  # true
reflect.taint_source(result)  # bridge:func:sqlite3_column_text

# Tainted data propagates
processed = result.upper()
reflect.tainted(processed)  # true - derived from tainted source

# Certain operations warn on tainted data
fs.write("output.txt", result)
# WARNING: Writing tainted data to filesystem

exec(result)
# ERROR: Cannot execute tainted data (blocked, not just warned)
```

**Taint can be explicitly cleared** when the programmer takes responsibility:

```graphoid
# After validation, clear taint
if is_safe_string(result) {
    clean_result = ffi.trust(result)
    reflect.tainted(clean_result)  # false
}

# Or trust with documented reason
clean_result = ffi.trust(result, reason: "Validated via is_safe_string")
```

**Taint metadata in graph:**
```
value:result
    ├── content ──► "user input"
    ├── tainted ──► true
    ├── taint_source ──► bridge:func:sqlite3_column_text
    ├── taint_chain ──► [bridge:ptr:001, bridge:func:...]
    └── trusted_at ──► none | {timestamp, reason, caller}
```

### Operations Affected by Taint

| Operation | Tainted Behavior |
|-----------|------------------|
| `exec()`, `eval()` | **Blocked** - cannot execute tainted code |
| `fs.write()` | Warning |
| `net.send()` | Warning |
| `sql.query()` with interpolation | Warning (use parameterized queries) |
| Arithmetic, string ops | Propagates taint to result |
| `print()` | Allowed (display only) |
| Comparison | Allowed (doesn't create new tainted values) |

### Resource Limits

Basic resource limits prevent runaway foreign code:

```graphoid
# Global limits (can be configured)
ffi.limits({
    max_bridge_nodes: 10000,     # Total tracked foreign entities
    max_memory_bytes: 100_000_000,  # 100MB foreign allocations
    max_libraries: 20,           # Loaded libraries
    max_pinned_callbacks: 100,   # Callbacks held for C
})

# Per-call timeout
result = ffi.with_timeout(5000) {  # 5 second timeout
    lib.slow_function(data)
}
```

Exceeding limits raises exceptions:
```graphoid
buffer = ffi.alloc(200_000_000)
# ERROR: Foreign allocation would exceed limit (100MB max)
```

### Sandbox Mode (Optional)

For untrusted libraries, an optional sandbox mode runs FFI calls in an isolated subprocess:

```graphoid
# Sandbox isolates at OS level
result = ffi.sandbox {
    lib = ffi.c("untrusted_plugin")
    lib.process(data)
}  # Subprocess terminated after block

# Sandbox limitations:
# - Higher overhead (IPC for every call)
# - Data must be serialized across boundary
# - Foreign pointers cannot escape sandbox
# - Callbacks into Graphoid are restricted
```

**Design note**: Sandbox implementation details deferred. The API is defined; implementation may use seccomp (Linux), sandbox-exec (macOS), or process isolation.

### Distributed Execution (Phase 24 Integration)

In distributed Graphoid, nodes can declare their FFI policy:

```graphoid
# Node configuration
node.configure({
    ffi_policy: :pure_only      # No FFI - only pure Graphoid
    # or
    ffi_policy: :allowed        # FFI permitted
    # or
    ffi_policy: :sandboxed      # FFI only in sandbox mode
})
```

**Implications:**
- Tasks requiring FFI only scheduled on `:allowed` or `:sandboxed` nodes
- Bridge nodes cannot be serialized to `:pure_only` nodes
- Tainted data crossing node boundaries is marked with origin node

```
# Distributed taint tracking
value:result
    ├── tainted ──► true
    ├── taint_source ──► bridge:func:sqlite3_column_text
    ├── origin_node ──► node:worker-3  # Which node created this
    └── crossed_boundary ──► true
```

This allows users to segregate pure computation from impure FFI workloads.

### Security Metadata on Bridge Nodes

Bridge nodes carry security-relevant metadata:

```
bridge:lib:sqlite3
    ├── type ──► type:ForeignLib
    ├── path ──► "/usr/lib/libsqlite3.so"
    ├── loaded_at ──► (timestamp)
    ├── loaded_by ──► (stack trace / caller info)
    ├── import_acknowledged ──► true | false  # Was 'unsafe' keyword used?
    └── calls_made ──► 47  # Number of function calls

bridge:ptr:001
    ├── ... (existing fields)
    ├── taint_propagates ──► true  # Data from this pointer is tainted
    └── trust_cleared_at ──► none | {timestamp, reason}
```

### Auditing

All security-relevant events can be queried:

```graphoid
# What libraries were loaded?
reflect.foreign_realm().libs()

# What tainted data exists?
reflect.values().filter(v => reflect.tainted(v))

# What foreign calls happened?
reflect.effects().filter(e => e.type == :foreign_call)

# Any trust operations?
reflect.effects().filter(e => e.type == :taint_cleared)

# Comprehensive security audit
fn security_audit() {
    report = {
        libraries: reflect.foreign_realm().libs().map(l => l.path),
        foreign_calls: reflect.effects().filter(e => e.type == :foreign_call).length(),
        tainted_values: reflect.values().filter(v => reflect.tainted(v)).length(),
        trust_operations: reflect.effects().filter(e => e.type == :taint_cleared),
        unfreed_allocations: reflect.foreign_realm().query({ state: :allocated }).length()
    }
    return report
}
```

### Summary: Security Principles

1. **Visible risk**: Import warnings make FFI usage obvious
2. **Tracked provenance**: Taint follows data from foreign sources
3. **Blocked danger**: Truly dangerous operations (exec tainted code) are blocked
4. **Warned caution**: Risky operations (write tainted to file) are warned
5. **Explicit trust**: Clearing taint requires programmer acknowledgment
6. **Auditable**: All foreign activity is queryable via reflection
7. **Distributable**: Nodes can declare pure-only policy
8. **Extensible**: Frameworks can add richer configuration later

### Power User Configuration

For users who need more control than warnings but less complexity than a full framework system, Graphoid provides power user escape hatches via environment variables, command-line flags, and runtime API.

**Environment Variables:**
```bash
# Library restrictions
GRAPHOID_FFI_ALLOW=sqlite3,zlib      # Whitelist (if set, others denied)
GRAPHOID_FFI_DENY=dangerous_lib      # Blacklist specific libraries
GRAPHOID_FFI_PATHS=/usr/lib/*        # Allowed library paths

# Behavior
GRAPHOID_FFI_WARNINGS=false          # Suppress import warnings
GRAPHOID_FFI_TAINT=true              # Enable taint tracking (default)
```

**Command-Line Flags:**
```bash
graphoid --ffi-allow=sqlite3,zlib script.gr
graphoid --ffi-deny-all script.gr           # Disable FFI entirely
graphoid --ffi-unsafe script.gr             # Suppress all warnings
graphoid --ffi-paths=/usr/lib/* script.gr
```

**Runtime API:**
```graphoid
import "ffi" unsafe

ffi.configure({
    allow: ["sqlite3", "zlib"],     # Whitelist (if set, others denied)
    deny: ["dangerous_lib"],         # Blacklist
    paths: ["/usr/lib/*"],           # Allowed library paths
    warnings: false,                 # Suppress import warnings
    taint: true,                     # Enable taint tracking
    limits: {
        max_memory_bytes: 50_000_000,
        max_libraries: 5
    }
})

# Configuration can tighten but not loosen after initial setting
ffi.configure({ allow: ["sqlite3"] })  # Restrict to sqlite3 only
ffi.configure({ allow: ["curl"] })     # ERROR: Cannot expand after restricting
```

**Precedence (highest to lowest):**
1. Command-line flags
2. Environment variables
3. Runtime `ffi.configure()`
4. Defaults (warnings on, all libraries allowed)

### Logging Configuration

Effect tracking and logging are separate concerns:
- **Tracking**: In-memory, for `reflect.effects()` queries
- **Logging**: Persistent output to file/stderr for auditing

```graphoid
import "logging"

logging.configure({
    # What to track/log
    effects: true,              # Track all effects in memory
    foreign_calls: true,        # Log FFI calls (default: true)
    taint_operations: true,     # Log taint clear/propagation events

    # Where to log
    destination: :stderr,       # :stderr, :stdout, :file, :null
    file_path: "/var/log/graphoid/ffi.log",

    # Format
    format: :structured,        # :text, :structured (JSON), :compact
    include_args: false,        # Include function arguments (security risk)
    include_stack: true,        # Include call stack

    # Filtering
    level: :warn,               # :debug, :info, :warn, :error
    filter: fn(event) {
        return event.type == :foreign_call
    }
})
```

**Environment Variables for Logging:**
```bash
GRAPHOID_LOG_LEVEL=warn
GRAPHOID_LOG_DEST=file:/var/log/graphoid.log
GRAPHOID_LOG_FORMAT=json
GRAPHOID_LOG_FFI=true
GRAPHOID_LOG_EFFECTS=false     # Disable effect tracking for performance
```

**Performance Modes:**
```graphoid
# Production: minimal overhead
logging.configure({
    effects: false,             # Disable in-memory tracking
    foreign_calls: true,        # Still log FFI calls (lightweight)
    format: :compact
})

# Development: full visibility
logging.configure({
    effects: true,
    foreign_calls: true,
    taint_operations: true,
    include_args: true,
    include_stack: true,
    level: :debug
})

# Sampling for production profiling
logging.configure({
    effects: :sample,
    sample_rate: 0.01           # Track 1% of effects
})
```

**Separate Tracking from Logging:**
```graphoid
# Track for queries but don't log to file
ffi.configure({ track_effects: true })
logging.configure({ log_effects: false })

# Or log without keeping in memory (low-memory environments)
ffi.configure({ track_effects: false })
logging.configure({ log_effects: true, destination: :file })
```

---

## Part 1: Foreign Realm Model

### 1.1 Two Realms

```
┌─────────────────────────────────────────────────────────────────────────────┐
│  UNIVERSE GRAPH                                                             │
│  ─────────────────────────────────────────────────────────────────────────  │
│  The trusted, introspectable world where "everything is a graph" is TRUE.   │
│                                                                             │
│  All native Graphoid values live here:                                      │
│  - Variables, functions, classes, instances                                 │
│  - Full introspection via reflect.*                                         │
│  - Mutation tracking, effect tracking                                       │
│                                                                             │
│       ┌──────────────┐         ┌──────────────┐                             │
│       │ var:sqlite   │────────►│ bridge:lib   │◄─── Bridge nodes live HERE  │
│       └──────────────┘         │ :sqlite3     │     (they ARE graph nodes)  │
│                                └──────┬───────┘                             │
│                                       │                                     │
│  ─────────────────────────────────────┼─────────────────────────────────────│
│                                       │ realm_ref                           │
│                                       ▼                                     │
│  FOREIGN REALM                                                              │
│  ─────────────────────────────────────────────────────────────────────────  │
│  The untrusted, opaque space for foreign entities.                          │
│                                                                             │
│  Foreign values exist here but are NOT introspectable:                      │
│  - We know metadata (type, source, address)                                 │
│  - We DON'T know actual contents                                            │
│  - Changes happen without our knowledge                                     │
│                                                                             │
│       ┌──────────────┐                                                      │
│       │ handle:      │◄─── Raw dlopen handle, opaque                        │
│       │ 0x7f4a...    │                                                      │
│       └──────────────┘                                                      │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 1.2 Bridge Nodes

Bridge nodes are the key abstraction. They:
- **Live in the universe graph** (they ARE graph nodes, fully introspectable)
- **Reference foreign entities** (via the foreign realm)
- **Track provenance** (where did this come from?)
- **Track state** (allocated, freed, error)

```
bridge:ptr:001
    ├── type ──────────► type:ForeignPtr
    ├── target_type ───► bridge:struct:sqlite3
    ├── source_lib ────► bridge:lib:sqlite3
    ├── source_call ───► "sqlite3_open"
    ├── state ─────────► :allocated | :freed | :error
    └── realm_ref ─────► (opaque handle in foreign realm)
```

### 1.3 Foreign Type Hierarchy

Foreign types integrate into Graphoid's type graph:

```
type:any
    ├── type:num
    ├── type:string
    ├── type:list
    ├── ...
    └── type:foreign                    # Abstract base for all foreign types
            ├── type:ForeignLib         # Loaded dynamic library
            ├── type:ForeignPtr         # Opaque pointer (void*)
            ├── type:ForeignTypedPtr    # Typed pointer (T*)
            ├── type:ForeignStruct      # C struct instance
            │       └── (user-defined struct types from cdef)
            └── type:ForeignCallback    # Wrapped Graphoid function
```

All foreign types are subtypes of `type:foreign`, enabling:
```graphoid
if value is foreign {
    # Handle foreign value specially
}

# Or more specific
if value is ForeignPtr {
    # It's a pointer
}
```

### 1.4 Reflection API

```graphoid
import "ffi"
import "reflect"

lib = ffi.c("sqlite3")

# Bridge node access
bridge = reflect.bridge(lib)
bridge.type          # type:ForeignLib
bridge.path          # "/usr/lib/libsqlite3.so"
bridge.declarations  # [bridge:func:sqlite3_open, ...]

# Query all foreign entities
reflect.foreign_realm().libs()      # All loaded libraries
reflect.foreign_realm().pointers()  # All tracked pointers
reflect.foreign_realm().query({ state: :allocated })  # Active allocations

# Provenance tracking
ptr = get_some_pointer()
bridge = reflect.bridge(ptr)
bridge.source_lib    # Which library created this?
bridge.source_call   # Which function call?
bridge.state         # :allocated, :freed, :error
```

---

## Part 2: Library Graphs

### 2.1 Loading Libraries

```graphoid
import "ffi"

# Load by name (finds .so/.dylib/.dll automatically)
sqlite = ffi.c("sqlite3")

# Load by explicit path
custom = ffi.c("/usr/local/lib/libcustom.so")

# Platform-specific loading
if ffi.platform == "windows" {
    lib = ffi.c("kernel32")
} else {
    lib = ffi.c("c")  # libc
}
```

**Graph created:**
```
bridge:lib:sqlite3
    ├── type ──► type:ForeignLib
    ├── path ──► "/usr/lib/libsqlite3.so.0"
    ├── loaded_at ──► (timestamp)
    ├── declares:func ──► []  # Empty until cdef
    ├── declares:struct ──► []
    └── realm_ref ──► (dlopen handle)
```

### 2.2 Function Declarations

**Primary method** - Parse actual C declarations:

```graphoid
sqlite = ffi.c("sqlite3")

# Declare functions using C syntax (copy-paste from headers!)
sqlite.cdef("""
    typedef struct sqlite3 sqlite3;
    typedef struct sqlite3_stmt sqlite3_stmt;

    int sqlite3_open(const char* filename, sqlite3** ppDb);
    int sqlite3_close(sqlite3* db);
    int sqlite3_exec(sqlite3* db, const char* sql,
                     int (*callback)(void*, int, char**, char**),
                     void* arg, char** errmsg);
    const char* sqlite3_errmsg(sqlite3* db);
    void sqlite3_free(void* ptr);
""")
```

**Graph expanded:**
```
bridge:lib:sqlite3
    │
    ├── declares:struct ──► bridge:struct:sqlite3
    │                           └── opaque: true
    │
    ├── declares:struct ──► bridge:struct:sqlite3_stmt
    │                           └── opaque: true
    │
    ├── declares:func ──► bridge:func:sqlite3_open
    │                         ├── params ──► [{name: "filename", type: str},
    │                         │               {name: "ppDb", type: ptr:ptr:sqlite3}]
    │                         └── returns ──► int
    │
    ├── declares:func ──► bridge:func:sqlite3_close
    │                         ├── params ──► [{name: "db", type: ptr:sqlite3}]
    │                         └── returns ──► int
    │
    ├── declares:func ──► bridge:func:sqlite3_exec
    │                         ├── params ──► [{name: "db", type: ptr:sqlite3},
    │                         │               {name: "sql", type: str},
    │                         │               {name: "callback", type: fn},
    │                         │               {name: "arg", type: ptr},
    │                         │               {name: "errmsg", type: ptr:ptr:char}]
    │                         └── returns ──► int
    │
    └── ...
```

**Why C syntax?**
- Programmers already know it
- Copy-paste from header files works
- No translation errors
- Self-documenting

### 2.3 Compact Declaration (Alternative)

For simple cases or when C headers aren't available:

```graphoid
# Compact syntax: name(params) -> return
sqlite.decl("sqlite3_open", [str, ptr] -> int)
sqlite.decl("sqlite3_close", [ptr] -> int)
sqlite.decl("sqlite3_errmsg", [ptr] -> str)
```

### 2.4 Type Mapping

| C Type | FFI Type | Graphoid Type | Notes |
|--------|----------|---------------|-------|
| `int` | `int` | `num` | Platform int (32-bit usually) |
| `int32_t` | `i32` | `num` | Explicit 32-bit |
| `int64_t` | `i64` | `num` | Explicit 64-bit |
| `uint32_t` | `u32` | `num` | Unsigned 32-bit |
| `uint64_t` | `u64` | `num` | Unsigned 64-bit |
| `float` | `f32` | `num` | 32-bit float |
| `double` | `f64` | `num` | 64-bit float |
| `char*` | `str` | `string` | Null-terminated string |
| `void*` | `ptr` | `ForeignPtr` | Opaque pointer |
| `T*` | `ptr:T` | `ForeignTypedPtr` | Typed pointer |
| `size_t` | `usize` | `num` | Platform-sized |
| `bool` | `bool` | `bool` | C99 bool |

---

## Part 3: Foreign Values

### 3.1 Pointers

```graphoid
# Create pointer (for out parameters)
db_ptr = ffi.ptr()
sqlite.sqlite3_open("test.db", db_ptr)
db = db_ptr.get()  # Dereference
```

**Graph created for `db_ptr`:**
```
bridge:ptr:001
    ├── type ──► type:ForeignPtr
    ├── state ──► :allocated
    └── realm_ref ──► (stack-allocated pointer space)
```

**Graph created for `db` (after sqlite3_open):**
```
bridge:ptr:002
    ├── type ──► type:ForeignTypedPtr
    ├── target_type ──► bridge:struct:sqlite3
    ├── source_lib ──► bridge:lib:sqlite3
    ├── source_call ──► "sqlite3_open"
    ├── source_args ──► ["test.db", bridge:ptr:001]
    ├── state ──► :allocated
    └── realm_ref ──► 0x7f4a...
```

### 3.2 Memory Allocation

```graphoid
# Allocate memory
buffer = ffi.alloc(1024)

# Graph:
# bridge:ptr:003
#     ├── type ──► type:ForeignPtr
#     ├── size ──► 1024
#     ├── source_call ──► "ffi.alloc"
#     ├── state ──► :allocated
#     └── realm_ref ──► 0x7f4b...

# Operations
buffer.set(0, 65)           # Write byte at offset
byte = buffer.get(0)        # Read byte at offset
buffer.write("hello", 0)    # Write string at offset
text = buffer.read_str(0)   # Read null-terminated string

# Manual cleanup
ffi.free(buffer)
# State changes: :allocated → :freed
```

### 3.3 Scope-Based Cleanup

```graphoid
ffi.scope {
    buf = ffi.alloc(4096)
    # ... use buf ...
}   # buf automatically freed here

# Graph shows cleanup:
# bridge:ptr:004
#     ├── state ──► :freed
#     ├── freed_by ──► :scope_exit
#     └── scope ──► scope:001
```

**Leak detection:**
```graphoid
# At scope exit, any allocated pointers from this scope are checked
# If not explicitly freed or transferred, a warning is logged
# and they are freed automatically

# Query for leaks:
leaks = reflect.foreign_realm().query({
    state: :freed,
    freed_by: :scope_exit,  # Auto-freed, not explicitly freed
    source_call: { not: "ffi.alloc" }  # Came from C, not our allocation
})
```

### 3.4 Structs

```graphoid
# Define struct using C syntax
sqlite.cdef("""
    struct Point {
        double x;
        double y;
    };

    double distance(struct Point* a, struct Point* b);
""")

# Create struct instance
p1 = ffi.new("Point", {x: 0.0, y: 0.0})
p2 = ffi.new("Point", {x: 3.0, y: 4.0})

# Graph:
# bridge:struct_instance:001
#     ├── type ──► bridge:struct:Point
#     ├── fields ──► {x: 0.0, y: 0.0}  # Cached Graphoid view
#     ├── state ──► :allocated
#     └── realm_ref ──► 0x7f4c...

# Access fields
print(p1.x)  # 0.0
p1.x = 10.0  # Updates both cache and foreign memory

# Call function
d = sqlite.distance(p1, p2)  # 5.0
```

### 3.5 Struct Synchronization

Foreign code can mutate structs without Graphoid knowing:

```graphoid
point = ffi.new("Point", {x: 0.0, y: 0.0})
some_c_function(point)  # C code might change x and y

# Graphoid's cached view may be stale
print(point.x)  # Might show old value!

# Explicit sync updates cache from foreign memory
ffi.sync(point)
print(point.x)  # Now shows actual value

# Or use auto-sync mode (slower, but always accurate)
point = ffi.new("Point", {x: 0.0, y: 0.0}, sync: :auto)
```

**Graph shows sync status:**
```
bridge:struct_instance:001
    ├── fields ──► {x: 0.0, y: 0.0}
    ├── fields_synced_at ──► (timestamp)
    ├── sync_mode ──► :manual | :auto
    └── possibly_stale ──► true | false
```

---

## Part 4: Callbacks

### 4.1 Passing Graphoid Functions to C

```graphoid
# C declaration includes callback signature
sqlite.cdef("""
    int sqlite3_exec(sqlite3* db, const char* sql,
                     int (*callback)(void* data, int ncols,
                                    char** values, char** names),
                     void* arg, char** errmsg);
""")

# Pass Graphoid function directly - signature inferred from cdef
rows = []
err = ffi.ptr()

sqlite.sqlite3_exec(db, "SELECT * FROM users", fn(data, ncols, values, names) {
    row = {}
    for i in range(ncols) {
        row[ffi.str(names, i)] = ffi.str(values, i)
    }
    rows.append(row)
    return 0
}, none, err)
```

**Graph created:**
```
bridge:callback:001
    ├── type ──► type:ForeignCallback
    ├── wraps ──► func:lambda:001  # The Graphoid function
    ├── signature ──► {params: [ptr, int, ptr, ptr], returns: int}
    ├── pinned ──► true  # Prevented from GC during call
    ├── invocation_count ──► 3  # Called 3 times by C
    └── realm_ref ──► (libffi closure)
```

### 4.2 Callback Lifetime

Callbacks are automatically pinned for the duration of the FFI call:

```graphoid
# Safe: callback only needed during sqlite3_exec
sqlite.sqlite3_exec(db, sql, my_callback, none, err)
# After call returns, callback can be GC'd
```

For callbacks that persist (e.g., event handlers), use explicit pinning:

```graphoid
# Pin callback for persistent use
handler = ffi.pin(fn(event) {
    print("Event: " + event)
    return 0
})
lib.register_handler(handler)

# Graph shows pin:
# bridge:callback:002
#     ├── pinned ──► true
#     ├── pinned_at ──► (timestamp)
#     └── pin_reason ──► :explicit

# Later: unpin when done
ffi.unpin(handler)
# State changes to allow GC
```

### 4.3 Callback Effects

Each callback invocation is tracked:

```graphoid
# After sqlite3_exec with 3 rows returned:
effects = reflect.effects().filter(e => e.type == :callback_invoke)

# [
#   effect:callback_invoke {
#       callback: bridge:callback:001,
#       invoked_by: bridge:func:sqlite3_exec,
#       invocation_number: 1,
#       graphoid_effects: [...]  # Effects from lambda body
#   },
#   effect:callback_invoke { invocation_number: 2, ... },
#   effect:callback_invoke { invocation_number: 3, ... }
# ]
```

---

## Part 5: Safety and Effects

### 5.1 Effect Tracking

All FFI calls generate effects (integrating with Phase 18):

```graphoid
fn do_database_work(path) {
    db_ptr = ffi.ptr()
    sqlite.sqlite3_open(path, db_ptr)
    db = db_ptr.get()

    sqlite.sqlite3_exec(db, "SELECT 1", none, none, ffi.ptr())
    sqlite.sqlite3_close(db)
}

# Effects generated:
effect:foreign_call {
    function: bridge:func:sqlite3_open,
    library: bridge:lib:sqlite3,
    args: ["test.db", bridge:ptr:001],
    result: 0,
    created_ptrs: [bridge:ptr:002],
    effect_type: :foreign_io
}
    │
    └── then ──►

effect:foreign_call {
    function: bridge:func:sqlite3_exec,
    ...
}
    │
    └── then ──►

effect:foreign_call {
    function: bridge:func:sqlite3_close,
    freed_ptrs: [bridge:ptr:002],
    ...
}
```

**Effect types:**
- `:foreign_io` - File, network, database operations
- `:foreign_memory` - Allocations, frees
- `:foreign_compute` - Pure computation (rare)
- `:foreign_unknown` - Can't determine (default)

### 5.2 Three Safety Tiers

```graphoid
# Tier 1: SAFE (default)
# - Type checking on all arguments
# - Null pointer checks
# - Bounds checking where possible
# - Full effect tracking
result = lib.some_function(arg1, arg2)

# Tier 2: UNCHECKED
# - Skip type/bounds checking for performance
# - Still tracks effects
# - Still uses safe memory management
result = lib.some_function!(arg1, arg2)  # ! suffix = unchecked

# Tier 3: RAW (explicit unsafe block)
# - Direct pointer arithmetic
# - Manual memory management
# - Minimal effect tracking
# - Use only when necessary
ffi.unsafe {
    ptr2 = ptr.offset(8)
    value = ptr2.read_i32()
    ptr2.write_i32(value + 1)
}
```

### 5.3 Memory Safety

```graphoid
# Use-after-free detection (safe mode)
ptr = ffi.alloc(100)
ffi.free(ptr)
ptr.get(0)  # ERROR: Use after free detected

# Graph shows:
# bridge:ptr:001
#     ├── state ──► :freed
#     └── access_after_free ──► [attempted access record]

# Double-free detection
ffi.free(ptr)  # ERROR: Double free detected
```

### 5.4 Auditing

```graphoid
# What foreign calls happened in this function?
fn analyze_foreign_usage(func) {
    effects = reflect.effects(func).filter(e => e.type == :foreign_call)

    for effect in effects {
        print("Called: " + effect.function.name)
        print("  Library: " + effect.library.path)
        print("  Args: " + effect.args)
    }
}

# What libraries are currently loaded?
for lib in reflect.foreign_realm().libs() {
    print(lib.path + " - " + lib.declarations.length() + " functions")
}

# Any memory leaks?
leaks = reflect.foreign_realm().query({
    type: :ptr,
    state: :allocated,
    source_call: { not: "ffi.alloc" }  # C-allocated, we can't free
})
if leaks.length() > 0 {
    print("WARNING: " + leaks.length() + " potential leaks")
}
```

---

## Part 6: Platform Abstraction

### 6.1 Platform Detection

```graphoid
ffi.platform      # "linux", "macos", "windows"
ffi.arch          # "x86_64", "aarch64"
ffi.pointer_size  # 8 (bytes)
ffi.endian        # "little", "big"
```

### 6.2 Platform-Aware Loading

```graphoid
# Automatic extension handling
lib = ffi.c("sqlite3")
# Finds: libsqlite3.so (Linux), libsqlite3.dylib (macOS), sqlite3.dll (Windows)

# Platform-specific code
if ffi.platform == "windows" {
    user32 = ffi.c("user32")
    user32.cdef('int __stdcall MessageBoxA(void*, char*, char*, int);')
    user32.MessageBoxA(none, "Hello", "Title", 0)
}
```

### 6.3 Calling Conventions

```graphoid
# Default: C calling convention (cdecl)
lib.cdef("int normal_func(int x);")

# Windows stdcall (for Win32 API)
lib.cdef("int __stdcall MessageBoxA(void* hwnd, char* text, char* caption, int type);",
         convention: "stdcall")

# Or specify per-function
lib.decl("MessageBoxA", [ptr, str, str, int] -> int, convention: "stdcall")
```

---

## Part 7: Rust Plugins

For Rust code, use the safer plugin system instead of raw FFI.

### 7.1 Plugin Loading

```graphoid
import "plugin"

# Load a Rust plugin (compiled as cdylib with abi_stable)
math_plugin = plugin.load("fast_math")

# Graph:
# bridge:plugin:fast_math
#     ├── type ──► type:ForeignPlugin  (subtype of ForeignLib)
#     ├── functions ──► [verified at load time]
#     └── safe ──► true  # Rust safety guarantees

# Call functions - types verified at load time
result = math_plugin.matrix_multiply(a, b)
```

### 7.2 Writing Rust Plugins

Plugins use `abi_stable` crate for stable ABI:

```rust
// fast_math/src/lib.rs
use abi_stable::prelude::*;
use graphoid_plugin::*;

#[export_root_module]
pub fn get_module() -> GraphoidPlugin_Ref {
    GraphoidPlugin {
        name: "fast_math".into(),
        functions: vec![
            PluginFunction {
                name: "matrix_multiply".into(),
                func: matrix_multiply,
            },
        ].into(),
    }.leak_into_prefix()
}

#[sabi_extern_fn]
fn matrix_multiply(args: RSlice<Value>) -> RResult<Value, RString> {
    // Implementation with full Rust safety
    let a = args[0].as_list()?;
    let b = args[1].as_list()?;
    // ... matrix multiplication ...
    ROk(Value::list(result))
}
```

### 7.3 Plugin vs C FFI

| Feature | C FFI | Rust Plugin |
|---------|-------|-------------|
| **Safety** | Manual | Rust-level |
| **Types** | Runtime checked | Compile-time verified |
| **Loading** | Any .so/.dll | Must be built for Graphoid |
| **Graph integration** | Bridge nodes | Bridge nodes (safer) |
| **Use case** | External C libs | Custom extensions |
| **Performance** | Native | Native |

---

## Part 8: System Calls

For the self-hosting goal, direct syscall access:

```graphoid
import "syscall"

# POSIX syscalls
fd = syscall.open("/etc/passwd", syscall.O_RDONLY)
data = syscall.read(fd, 1024)
syscall.close(fd)

# Memory mapping
ptr = syscall.mmap(none, 4096, syscall.PROT_READ | syscall.PROT_WRITE,
                   syscall.MAP_PRIVATE | syscall.MAP_ANONYMOUS, -1, 0)
syscall.munmap(ptr, 4096)

# Graph:
# effect:syscall {
#     number: 0 (read),
#     args: [fd, buffer, 1024],
#     result: bytes_read,
#     effect_type: :syscall_io
# }
```

**Note**: Syscall module is optional and platform-specific. Primary use is bootstrapping toward self-hosting.

---

## Part 9: Complete Example

### SQLite Wrapper with Full Graph Integration

```graphoid
import "ffi"
import "reflect"

# Load and declare
sqlite = ffi.c("sqlite3")
sqlite.cdef("""
    typedef struct sqlite3 sqlite3;

    int sqlite3_open(const char* filename, sqlite3** ppDb);
    int sqlite3_exec(sqlite3* db, const char* sql,
                     int (*callback)(void*, int, char**, char**),
                     void* arg, char** errmsg);
    int sqlite3_close(sqlite3* db);
    const char* sqlite3_errmsg(sqlite3* db);
    void sqlite3_free(void* ptr);
""")

# High-level wrapper
class Database {
    _handle = none
    _bridge = none  # Track our bridge node

    fn open(path) {
        db_ptr = ffi.ptr()
        result = sqlite.sqlite3_open(path, db_ptr)
        if result != 0 {
            raise "Failed to open database"
        }
        _handle = db_ptr.get()
        _bridge = reflect.bridge(_handle)
        return self
    }

    fn execute(sql) {
        rows = []
        err = ffi.ptr()

        sqlite.sqlite3_exec(_handle, sql, fn(data, ncols, values, names) {
            row = {}
            for i in range(ncols) {
                row[ffi.str(names, i)] = ffi.str(values, i)
            }
            rows.append(row)
            return 0
        }, none, err)

        if err.get() != none {
            msg = ffi.str(err.get())
            sqlite.sqlite3_free(err.get())
            raise "SQL error: " + msg
        }

        return rows
    }

    fn close() {
        if _handle != none {
            sqlite.sqlite3_close(_handle)
            _handle = none
        }
    }

    # Introspection
    fn bridge() {
        return _bridge
    }

    fn foreign_calls() {
        return reflect.effects().filter(e =>
            e.type == :foreign_call and
            e.involves(_bridge)
        )
    }
}

# Usage
db = Database().open("test.db")
db.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT)")
db.execute("INSERT INTO users VALUES (1, 'Alice')")
users = db.execute("SELECT * FROM users")
print(users)  # [{"id": "1", "name": "Alice"}]

# Introspection
print("Database bridge: " + db.bridge().source_call)  # "sqlite3_open"
print("Foreign calls made: " + db.foreign_calls().length())

db.close()

# Verify cleanup
print("Handle state: " + db.bridge().state)  # :freed
```

---

## Architecture

### Module Structure

```
src/ffi/
├── mod.rs              # FFI module root, NativeModule impl
├── realm.rs            # Foreign realm management
├── bridge.rs           # Bridge node creation and tracking
├── library.rs          # Dynamic library loading (libloading)
├── parser.rs           # C declaration parser
├── types.rs            # FFI type system and mapping
├── pointer.rs          # Pointer wrapper type
├── memory.rs           # Allocation tracking
├── callback.rs         # Callback wrapping (libffi)
├── calling.rs          # Function invocation (libffi)
├── effects.rs          # Effect generation for FFI calls
├── sync.rs             # Struct synchronization
├── platform.rs         # Platform abstraction
├── taint.rs            # Taint tracking and propagation
├── limits.rs           # Resource limit enforcement
└── sandbox.rs          # Sandbox mode (optional, OS-specific)

src/plugin/
├── mod.rs              # Plugin module root
├── loader.rs           # Plugin loading
└── abi.rs              # Stable ABI definitions
```

### Dependencies

```toml
# Cargo.toml additions
[dependencies]
libloading = "0.8"      # Dynamic library loading
libffi = "3.0"          # Foreign function calls
abi_stable = "0.11"     # Stable ABI for Rust plugins
```

---

## Implementation Plan

### Phase 20a: Foreign Realm Foundation (4-5 days)

| Day | Task |
|-----|------|
| 1-2 | Foreign realm data structure, bridge node types |
| 3 | Bridge node creation, type hierarchy integration |
| 4-5 | Reflection API: `reflect.bridge()`, `reflect.foreign_realm()` |

### Phase 20b: Library Graphs (4-5 days)

| Day | Task |
|-----|------|
| 6-7 | Library loading with bridge node creation, import warnings |
| 8-9 | C declaration parser, populating declaration subgraph |
| 10 | Platform abstraction, library resolution |

### Phase 20c: Foreign Values (3-4 days)

| Day | Task |
|-----|------|
| 11-12 | Pointer bridge nodes, allocation tracking |
| 13-14 | Struct support, field access, synchronization |

### Phase 20d: Callbacks and Effects (3-4 days)

| Day | Task |
|-----|------|
| 15-16 | Callback wrapping, pinning, invocation tracking |
| 17-18 | Effect generation, integration with Phase 18 |

### Phase 20e: Security (2-3 days)

| Day | Task |
|-----|------|
| 19 | Taint tracking: tainting foreign return values, propagation |
| 20 | Taint operations: `reflect.tainted()`, `ffi.trust()`, blocked/warned ops |
| 21 | Resource limits: `ffi.limits()`, enforcement, limit exceeded errors |

### Phase 20f: Configuration and Logging (2-3 days)

| Day | Task |
|-----|------|
| 22 | Environment variables and CLI flags for FFI configuration |
| 23 | Runtime `ffi.configure()` API with tighten-only semantics |
| 24 | Logging configuration: `logging.configure()`, destinations, formats, filtering |

---

## Success Criteria

### Graph Integration
- [ ] Foreign types in type hierarchy under `type:foreign`
- [ ] Bridge nodes created for all foreign entities
- [ ] `reflect.bridge()` returns bridge node for foreign value
- [ ] `reflect.foreign_realm()` queries all foreign entities
- [ ] Provenance tracked (source_lib, source_call)
- [ ] State tracked (allocated, freed, error)

### Core Functionality
- [ ] Load shared libraries on Linux, macOS, Windows
- [ ] Parse C function declarations
- [ ] Call C functions with all basic types
- [ ] Pointer creation, dereferencing, arithmetic
- [ ] Struct definition and field access
- [ ] Callbacks from C into Graphoid

### Safety and Effects
- [ ] Effect generated for every foreign call
- [ ] Scope-based memory cleanup
- [ ] Use-after-free detection in safe mode
- [ ] Double-free detection
- [ ] Leak detection via reflection

### Security
- [ ] Import warning emitted for `import "ffi"`
- [ ] `import "ffi" unsafe` suppresses warning
- [ ] Foreign return values are tainted
- [ ] Taint propagates through operations
- [ ] `reflect.tainted()` checks taint status
- [ ] `ffi.trust()` clears taint with reason
- [ ] `exec()` / `eval()` blocked on tainted data
- [ ] `fs.write()` / `net.send()` warn on tainted data
- [ ] `ffi.limits()` sets resource limits
- [ ] Exceeding limits raises exceptions
- [ ] Security metadata on bridge nodes (import_acknowledged, etc.)

### Configuration
- [ ] `GRAPHOID_FFI_ALLOW` environment variable for library whitelist
- [ ] `GRAPHOID_FFI_DENY` environment variable for library blacklist
- [ ] `GRAPHOID_FFI_PATHS` environment variable for path restrictions
- [ ] `--ffi-allow`, `--ffi-deny-all`, `--ffi-unsafe` CLI flags
- [ ] `ffi.configure()` runtime API
- [ ] Configuration can tighten but not loosen after initial setting
- [ ] Precedence: CLI > env vars > runtime > defaults

### Logging
- [ ] `logging.configure()` API for FFI/effect logging
- [ ] Destination options: `:stderr`, `:stdout`, `:file`, `:null`
- [ ] Format options: `:text`, `:structured`, `:compact`
- [ ] Level filtering: `:debug`, `:info`, `:warn`, `:error`
- [ ] `GRAPHOID_LOG_*` environment variables
- [ ] Separate tracking (in-memory) from logging (persistent)
- [ ] Sampling mode for production profiling

### Platform Support
- [ ] Platform constants (ffi.platform, ffi.arch)
- [ ] Calling convention support (cdecl, stdcall)
- [ ] Automatic library extension handling

### Testing and Documentation
- [ ] At least 80 FFI tests (including graph integration, security, and configuration)
- [ ] Example: Working SQLite wrapper with introspection
- [ ] Example: Callback with effect tracking
- [ ] Example: Memory leak detection
- [ ] Example: Taint tracking and trust clearing
- [ ] Example: Resource limit enforcement
- [ ] Example: Power user configuration (env vars, CLI, runtime)
- [ ] Example: Logging configuration and output formats
- [ ] Documentation complete

---

## Open Questions (Resolved)

| Question | Resolution |
|----------|------------|
| How do foreign objects fit in universe graph? | Bridge nodes in universe graph reference foreign realm |
| What can we introspect? | Metadata (type, provenance, state) but not foreign memory contents |
| How to handle mutable foreign data? | Explicit `ffi.sync()` or `sync: :auto` mode |
| Effect tracking? | All FFI calls generate `foreign_call` effects |
| Serialization for distribution? | Bridge nodes can't serialize (correctly fails) |
| Thread safety? | Single-threaded initially; `ffi.blocking {}` for async (Phase 20b) |
| Security default? | Warning on import; frameworks will add richer config later |
| How to track dangerous data? | Taint tracking with propagation; explicit trust to clear |
| Configuration granularity? | Deferred to frameworks; program-level warnings for now |
| Distributed FFI policy? | Nodes declare :pure_only, :allowed, or :sandboxed |
| Power user configuration? | Env vars, CLI flags, runtime API with tighten-only semantics |
| Logging control? | `logging.configure()` with destinations, formats, levels; separate from tracking |
| Performance overhead? | Tracking can be disabled; logging can use sampling mode |

---

## Related Documents

- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Effect tracking integration
- [PHASE_19_CONCURRENCY.md](PHASE_19_CONCURRENCY.md) - Async runtime for `ffi.blocking {}`
- [PHASE_21_PACKAGE_MANAGER.md](PHASE_21_PACKAGE_MANAGER.md) - Distribution of FFI wrappers
- [PHASE_22_DATABASE.md](PHASE_22_DATABASE.md) - Primary consumer of FFI
- [PHASE_26_REFLECTION.md](PHASE_26_REFLECTION.md) - `reflect.bridge()` API details
