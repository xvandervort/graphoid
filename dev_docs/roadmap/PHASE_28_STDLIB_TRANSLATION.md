# Phase 28: Stdlib Translation & Expansion

**Duration**: 15-20 days
**Priority**: High
**Dependencies**: Phase 14 (Testing Framework), Phase 20 (FFI) for compress/archive
**Status**: Ready to Begin (pure Graphoid modules); FFI modules after Phase 20

---

## Goal

Expand Graphoid's standard library to be "batteries included" like Python and Go. All modules follow graph-centric design - data structures ARE graphs, not wrappers around traditional data types.

---

## Design Principles

1. **Everything is a graph** - URLs, paths, log entries, CLI args are all graph structures
2. **Low-level first** - Foundational modules before high-level
3. **Multi-file namespaces** - Complex modules use folder structure
4. **Pure Graphoid** - No native dependencies (defer to post-FFI)
5. **Batteries included** - Comprehensive coverage for common tasks

---

## Module Structure

Modules can be single files or namespaced folders:

```
stdlib/
├── constants.gr           # Single file module
├── encoding/              # Namespace folder
│   ├── mod.gr             # Main exports
│   ├── base64.gr
│   ├── hex.gr
│   └── percent.gr
├── url/
│   ├── mod.gr
│   ├── parser.gr
│   └── builder.gr
├── path/
│   ├── mod.gr
│   ├── posix.gr
│   └── windows.gr
├── uuid/
│   └── mod.gr
├── logging/
│   ├── mod.gr
│   ├── logger.gr
│   ├── handlers.gr
│   └── formatters.gr
└── argparse/
    ├── mod.gr
    ├── parser.gr
    └── types.gr
```

---

## Immediate Scope (Phase 28)

### 1. Constants (`stdlib/constants.gr`)

**Effort**: 1 day

Trivial translation of values:

```graphoid
# Mathematical Constants
PI = 3.141592653589793
E = 2.718281828459045
TAU = 6.283185307179586
PHI = 1.618033988749895

# Physical Constants (SI)
SPEED_OF_LIGHT = 299792458.0
PLANCK = 6.62607015e-34
GRAVITATIONAL = 6.67430e-11
```

---

### 2. Encoding (`stdlib/encoding/`)

**Effort**: 2 days

Consolidate encoding functions currently scattered in crypto. Graph-centric design for streaming encoders.

#### Graph Structure

```
encoder_graph
├── config/
│   ├── alphabet ──► "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
│   └── padding ──► "="
├── state/
│   ├── buffer ──► [pending bytes]
│   └── position ──► 0
└── output ──► [encoded chunks]
```

#### API

```graphoid
import "encoding"

# Simple functions
encoded = encoding.base64_encode("hello world")
decoded = encoding.base64_decode(encoded)

hex_str = encoding.hex_encode([0x48, 0x65, 0x6c, 0x6c, 0x6f])
bytes = encoding.hex_decode("48656c6c6f")

url_safe = encoding.percent_encode("hello world!")  # "hello%20world%21"
original = encoding.percent_decode("hello%20world%21")

# Streaming encoder (for large data)
encoder = encoding.Base64Encoder {}
encoder.write("first chunk")
encoder.write("second chunk")
result = encoder.finish()

# Inspect encoder state (it's a graph!)
reflect(encoder).data().nodes()  # See internal buffer state
```

#### Files

- `mod.gr` - Public exports
- `base64.gr` - Base64 encoding/decoding
- `hex.gr` - Hexadecimal encoding/decoding
- `percent.gr` - URL percent encoding

---

### 3. URL (`stdlib/url/`)

**Effort**: 2-3 days

URLs ARE graphs. Parsing produces a graph structure, not a record.

#### Graph Structure

```
url_graph
├── scheme ──► "https"
├── authority/
│   ├── userinfo ──► none (or "user:pass")
│   ├── host ──► "api.example.com"
│   └── port ──► 443
├── path/
│   ├── segment:0 ──► "v1"
│   ├── segment:1 ──► "users"
│   └── segment:2 ──► "123"
├── query/
│   ├── param:name ──► "alice"
│   ├── param:limit ──► "10"
│   └── param:sort ──► ["name", "asc"]  # Multi-value
└── fragment ──► "section1"
```

#### API

```graphoid
import "url"

# Parse URL into graph
u = url.parse("https://api.example.com:8080/v1/users/123?name=alice&limit=10#section1")

# Access components (graph traversal)
u.scheme           # "https"
u.host             # "api.example.com"
u.port             # 8080
u.path_segments()  # ["v1", "users", "123"]
u.query("name")    # "alice"
u.query("missing") # none
u.fragment         # "section1"

# Inspect as graph
u.nodes()          # All nodes in URL graph
u.traverse(from: "authority")  # Authority subgraph

# Build URL (graph construction)
new_url = url.URL {
    scheme: "https",
    host: "example.com",
    path: ["api", "v2", "items"]
}
new_url.add_query("page", "1")
new_url.add_query("sort", "name")
new_url.to_string()  # "https://example.com/api/v2/items?page=1&sort=name"

# Modify URL (graph mutation)
u.set_path(["v2", "accounts", "456"])
u.remove_query("limit")
u.to_string()

# Resolve relative URLs (graph operations)
base = url.parse("https://example.com/docs/guide/")
relative = url.parse("../api/reference")
resolved = base.resolve(relative)  # "https://example.com/docs/api/reference"

# URL validation
url.is_valid("https://example.com")  # true
url.is_valid("not a url")            # false
```

#### Files

- `mod.gr` - Public exports, URL graph type
- `parser.gr` - RFC 3986 compliant parser
- `builder.gr` - URL construction utilities

---

### 4. Path (`stdlib/path/`)

**Effort**: 2-3 days

Paths ARE graphs. A path is a linked sequence of segment nodes.

#### Graph Structure

```
path_graph
├── root ──► "/" (or "C:\" on Windows)
├── segment:0 ──► "home"
│   └── next ──► segment:1
├── segment:1 ──► "user"
│   └── next ──► segment:2
├── segment:2 ──► "documents"
│   └── next ──► segment:3
└── segment:3 ──► "file.txt"
    └── extension ──► "txt"
```

#### API

```graphoid
import "path"

# Parse path into graph
p = path.parse("/home/user/documents/file.txt")

# Access components
p.root()           # "/"
p.segments()       # ["home", "user", "documents", "file.txt"]
p.parent()         # Path graph for "/home/user/documents"
p.filename()       # "file.txt"
p.stem()           # "file"
p.extension()      # "txt"

# Path operations (graph operations)
p.join("subdir", "another.txt")  # Append segments
p.with_extension("md")           # Change extension
p.with_filename("renamed.txt")   # Change filename

# Normalization
messy = path.parse("/home/user/../user/./documents//file.txt")
messy.normalize()  # "/home/user/documents/file.txt"

# Relative paths
from_path = path.parse("/home/user/projects")
to_path = path.parse("/home/user/documents/file.txt")
from_path.relative_to(to_path)  # "../documents/file.txt"

# Platform-aware
path.separator()   # "/" on Unix, "\" on Windows
path.join("dir", "file")  # Uses correct separator

# Inspect as graph
p.nodes()          # All segment nodes
p.traverse(from: "root")  # Walk path graph
```

#### Files

- `mod.gr` - Public exports, platform detection
- `posix.gr` - Unix path handling
- `windows.gr` - Windows path handling

---

### 5. UUID (`stdlib/uuid/`)

**Effort**: 1 day

UUIDs as graph nodes with parsed components.

#### Graph Structure

```
uuid_graph
├── version ──► 4
├── variant ──► "RFC4122"
├── bytes ──► [16 bytes]
├── fields/
│   ├── time_low ──► 0x550e8400
│   ├── time_mid ──► 0xe29b
│   ├── time_hi_version ──► 0x41d4
│   ├── clock_seq ──► 0xa716
│   └── node ──► 0x446655440000
└── string ──► "550e8400-e29b-41d4-a716-446655440000"
```

#### API

```graphoid
import "uuid"

# Generate UUIDs
id = uuid.v4()              # Random UUID
id.to_string()              # "550e8400-e29b-41d4-a716-446655440000"
id.version                  # 4
id.bytes                    # [16 bytes]

# Parse UUID
parsed = uuid.parse("550e8400-e29b-41d4-a716-446655440000")
parsed.version              # 4
parsed.variant              # "RFC4122"

# Validation
uuid.is_valid("550e8400-e29b-41d4-a716-446655440000")  # true
uuid.is_valid("not-a-uuid")                             # false

# Comparison
id1 = uuid.v4()
id2 = uuid.v4()
id1 == id2                  # false (different UUIDs)
id1 == uuid.parse(id1.to_string())  # true

# Nil UUID
uuid.nil()                  # "00000000-0000-0000-0000-000000000000"
uuid.nil().is_nil()         # true
```

---

### 6. Logging (`stdlib/logging/`)

**Effort**: 3-4 days

Log entries ARE graph nodes. The log itself IS a graph - an append-only sequence with edges for filtering, grouping, and querying.

#### Graph Structure

```
logger_graph
├── config/
│   ├── level ──► :info
│   ├── format ──► :json
│   └── handlers ──► [handler_1, handler_2]
├── entries/
│   ├── entry:1
│   │   ├── timestamp ──► 1706000000000
│   │   ├── level ──► :info
│   │   ├── message ──► "Server started"
│   │   ├── source ──► "main.gr:45"
│   │   ├── context/
│   │   │   ├── port ──► 8080
│   │   │   └── host ──► "localhost"
│   │   └── next ──► entry:2
│   └── entry:2
│       ├── timestamp ──► 1706000001000
│       ├── level ──► :error
│       ├── message ──► "Connection failed"
│       ├── error ──► (error graph)
│       └── next ──► none
└── indices/
    ├── by_level/
    │   ├── :info ──► [entry:1]
    │   └── :error ──► [entry:2]
    └── by_source/
        └── "main.gr" ──► [entry:1, entry:2]
```

#### API

```graphoid
import "logging"

# Create logger
log = logging.Logger {
    level: :info,
    format: :json
}

# Add handlers (output destinations)
log.add_handler(logging.ConsoleHandler {})
log.add_handler(logging.FileHandler { path: "/var/log/app.log" })

# Log messages (creates entry nodes)
log.debug("Debug message")           # Filtered out (below :info)
log.info("Server started", { port: 8080 })
log.warn("Slow query", { duration_ms: 1500, query: "SELECT..." })
log.error("Connection failed", { host: "db.example.com" })

# Structured logging with context
request_log = log.with_context({ request_id: "abc123", user_id: 456 })
request_log.info("Processing request")   # Includes request_id, user_id
request_log.info("Request complete", { status: 200 })

# Query log entries (graph traversal!)
errors = log.entries().filter(e => e.level == :error)
recent = log.entries().filter(e => e.timestamp > time.now() - 3600000)
by_source = log.entries().filter(e => e.source.starts_with("api/"))

# Log is a graph - can traverse and inspect
log.entries().nodes()
log.indices.by_level[:error]  # All error entries

# Child loggers (subgraphs)
api_log = log.child("api")
api_log.info("Request received")  # source: "api"

db_log = log.child("db")
db_log.warn("Slow query")  # source: "db"
```

#### Handlers

```graphoid
# Console handler with colors
logging.ConsoleHandler {
    colors: true,
    format: :text  # or :json
}

# File handler with rotation
logging.FileHandler {
    path: "/var/log/app.log",
    max_size: 10 * 1024 * 1024,  # 10MB
    max_files: 5
}

# Custom handler (it's a graph!)
graph MyHandler {
    fn handle(entry) {
        # entry is a graph node
        if entry.level == :error {
            # Send alert
            http.post("https://alerts.example.com", {
                message: entry.message,
                context: entry.context
            })
        }
    }
}
```

#### Files

- `mod.gr` - Public exports, Logger graph type
- `logger.gr` - Core logger implementation
- `handlers.gr` - ConsoleHandler, FileHandler, etc.
- `formatters.gr` - Text, JSON, custom formatters

---

### 7. Argparse (`stdlib/argparse/`)

**Effort**: 3-4 days

CLI argument definitions ARE a graph. Parsed arguments ARE a graph.

#### Graph Structure

```
parser_graph
├── name ──► "myapp"
├── description ──► "My application"
├── arguments/
│   ├── arg:verbose
│   │   ├── names ──► ["-v", "--verbose"]
│   │   ├── type ──► :flag
│   │   ├── help ──► "Enable verbose output"
│   │   └── default ──► false
│   ├── arg:config
│   │   ├── names ──► ["-c", "--config"]
│   │   ├── type ──► :string
│   │   ├── required ──► true
│   │   └── help ──► "Config file path"
│   └── arg:port
│       ├── names ──► ["-p", "--port"]
│       ├── type ──► :int
│       ├── default ──► 8080
│       └── help ──► "Port number"
├── subcommands/
│   ├── cmd:serve
│   │   ├── description ──► "Start server"
│   │   └── arguments ──► (subgraph)
│   └── cmd:build
│       ├── description ──► "Build project"
│       └── arguments ──► (subgraph)
└── positional/
    └── pos:files
        ├── nargs ──► :many
        └── help ──► "Input files"
```

#### API

```graphoid
import "argparse"

# Define parser (builds argument graph)
parser = argparse.Parser {
    name: "myapp",
    description: "My application"
}

# Add arguments (adds nodes to graph)
parser.add_argument("-v", "--verbose", {
    type: :flag,
    help: "Enable verbose output"
})

parser.add_argument("-c", "--config", {
    type: :string,
    required: true,
    help: "Config file path"
})

parser.add_argument("-p", "--port", {
    type: :int,
    default: 8080,
    help: "Port number"
})

# Positional arguments
parser.add_positional("files", {
    nargs: :many,
    help: "Input files"
})

# Subcommands (subgraphs)
serve_cmd = parser.add_subcommand("serve", {
    description: "Start server"
})
serve_cmd.add_argument("--host", { default: "localhost" })

build_cmd = parser.add_subcommand("build", {
    description: "Build project"
})
build_cmd.add_argument("--release", { type: :flag })

# Parse arguments (returns result graph)
args = parser.parse(os.args())

# Access parsed values (graph traversal)
args.verbose           # true/false
args.config            # "/path/to/config"
args.port              # 8080
args.files             # ["file1.txt", "file2.txt"]
args.subcommand        # "serve" or "build" or none

# Subcommand args
if args.subcommand == "serve" {
    args.serve.host    # "localhost"
}

# Auto-generated help
parser.print_help()
# Output:
# myapp - My application
#
# Usage: myapp [OPTIONS] [FILES...]
#
# Options:
#   -v, --verbose    Enable verbose output
#   -c, --config     Config file path (required)
#   -p, --port       Port number [default: 8080]
#
# Subcommands:
#   serve            Start server
#   build            Build project

# Inspect parser structure (it's a graph!)
parser.arguments.nodes()
parser.subcommands.nodes()
```

#### Files

- `mod.gr` - Public exports, Parser graph type
- `parser.gr` - Argument parsing logic
- `types.gr` - Type coercion (string, int, float, flag, etc.)

---

### 8. Compression (`stdlib/compress/`)

**Effort**: 3-4 days
**Requires**: Phase 20 FFI (for zlib bindings)

Compression and decompression. Compressor state as graph.

#### Graph Structure

```
compressor_graph
├── algorithm ──► :gzip
├── level ──► 6
├── state/
│   ├── bytes_in ──► 10240
│   ├── bytes_out ──► 3072
│   └── ratio ──► 0.30
└── buffer ──► [compressed chunks]
```

#### API

```graphoid
import "compress"

# Simple compression/decompression
compressed = compress.gzip("hello world")
original = compress.gunzip(compressed)

compressed = compress.deflate(data)
original = compress.inflate(compressed)

# Streaming compression (for large data)
compressor = compress.GzipCompressor { level: 9 }
compressor.write(chunk1)
compressor.write(chunk2)
result = compressor.finish()

# Streaming decompression
decompressor = compress.GzipDecompressor {}
for chunk in compressed_stream {
    decompressed = decompressor.write(chunk)
    process(decompressed)
}
remaining = decompressor.finish()

# Compress files
compress.gzip_file("/path/to/file.txt", "/path/to/file.txt.gz")
compress.gunzip_file("/path/to/file.txt.gz", "/path/to/file.txt")

# Compression stats (graph inspection)
compressor = compress.GzipCompressor {}
compressor.write(large_data)
compressor.state.bytes_in    # Original size
compressor.state.bytes_out   # Compressed size
compressor.state.ratio       # Compression ratio
```

#### Files

- `mod.gr` - Public exports
- `gzip.gr` - Gzip compression
- `deflate.gr` - Raw deflate
- `zlib.gr` - Zlib format

---

### 9. Archive (`stdlib/archive/`)

**Effort**: 3-4 days
**Requires**: `compress/` module

Archive handling (tar, zip). Archive as graph of entries.

#### Graph Structure

```
archive_graph
├── format ──► :tar_gz
├── path ──► "/path/to/archive.tar.gz"
├── entries/
│   ├── entry:0
│   │   ├── name ──► "dir/"
│   │   ├── type ──► :directory
│   │   ├── mode ──► 0o755
│   │   └── mtime ──► 1706000000
│   ├── entry:1
│   │   ├── name ──► "dir/file.txt"
│   │   ├── type ──► :file
│   │   ├── size ──► 1024
│   │   ├── mode ──► 0o644
│   │   └── content ──► (lazy loaded)
│   └── entry:2
│       ├── name ──► "dir/link"
│       ├── type ──► :symlink
│       └── target ──► "file.txt"
└── metadata/
    ├── total_size ──► 2048
    └── entry_count ──► 3
```

#### API

```graphoid
import "archive"

# Create tar.gz archive
arc = archive.create("/path/to/archive.tar.gz", :tar_gz)
arc.add_file("local/file.txt", "archive/path/file.txt")
arc.add_directory("local/dir", "archive/dir")
arc.add_string("content here", "archive/generated.txt")
arc.close()

# Extract archive
arc = archive.open("/path/to/archive.tar.gz")
arc.extract_all("/destination/")

# Or extract specific files
arc = archive.open("/path/to/archive.tar.gz")
for entry in arc.entries() {
    if entry.name.ends_with(".txt") {
        entry.extract("/destination/" + entry.name)
    }
}

# List contents (graph traversal)
arc = archive.open("/path/to/archive.zip")
for entry in arc.entries() {
    print(entry.name + " (" + entry.size.to_string() + " bytes)")
}

# Read file from archive without extracting
arc = archive.open("/path/to/archive.tar.gz")
content = arc.read("path/in/archive/file.txt")

# Archive metadata (graph inspection)
arc.metadata.total_size
arc.metadata.entry_count
arc.entries().filter(e => e.type == :file).length()

# Supported formats
archive.create("file.tar", :tar)
archive.create("file.tar.gz", :tar_gz)
archive.create("file.tar.bz2", :tar_bz2)
archive.create("file.zip", :zip)
```

#### Files

- `mod.gr` - Public exports, format detection
- `tar.gr` - Tar format handling
- `zip.gr` - Zip format handling
- `entry.gr` - Archive entry graph type

---

## Implementation Plan

### Week 1: Pure Graphoid Modules

| Day | Task |
|-----|------|
| 1 | `constants.gr` - translate values |
| 2-3 | `encoding/` - base64, hex, percent encoding |
| 3-4 | `url/` - URL parsing and building |
| 5 | `uuid/` - UUID generation and parsing |

### Week 2: Pure Graphoid Modules (cont.)

| Day | Task |
|-----|------|
| 1-2 | `path/` - path manipulation |
| 3-4 | `logging/` - structured logging |
| 5 | `logging/` - handlers and formatters |

### Week 3: Pure + FFI Modules

| Day | Task |
|-----|------|
| 1-2 | `argparse/` - argument parsing and help |
| 3-4 | `compress/` - gzip, deflate (uses FFI for zlib) |
| 5 | `archive/` - tar format handling |

### Week 4: Completion

| Day | Task |
|-----|------|
| 1-2 | `archive/` - zip format handling |
| 3-5 | Integration testing, documentation |

---

## Success Criteria

- [ ] `constants.gr` - all math/physics constants
- [ ] `encoding/` - base64, hex, percent encode/decode
- [ ] `url/` - parse, build, resolve URLs as graphs
- [ ] `uuid/` - generate and parse UUIDs
- [ ] `path/` - cross-platform path manipulation as graphs
- [ ] `logging/` - structured logging with graph-based entries
- [ ] `argparse/` - CLI parsing with graph-based definitions
- [ ] `compress/` - gzip/deflate compression as graphs
- [ ] `archive/` - tar/zip handling as graphs
- [ ] All modules have gspec tests
- [ ] All modules have documentation
- [ ] All existing tests pass
- [ ] Graph-centric design verified for each module

---

## Metrics

### After Phase 28

| Category | Count |
|----------|-------|
| Pure Graphoid stdlib | 31 modules (+9) |
| Native Rust stdlib | 5 modules (syscall wrappers only) |
| Namespace modules | 8 (encoding, url, path, uuid, logging, argparse, compress, archive) |

---

## Intentionally Deferred: Subprocess

The `subprocess` module was considered but intentionally deferred. This section documents the rationale.

### Security Concerns

Subprocess execution is a persistent source of security vulnerabilities:

1. **Command Injection** - User input in shell commands is OWASP Top 10
2. **Shell Quoting Complexity** - Even experienced developers get it wrong
3. **Environment Leakage** - Child processes inherit potentially sensitive env vars
4. **Path Manipulation** - Attacks via PATH or relative command paths
5. **Resource Exhaustion** - Fork bombs, zombie processes, descriptor leaks

### Philosophical Misalignment

Graphoid's mission is graph-theoretic computing with eventual self-hosting:

| Use Case | Subprocess Approach | Graphoid Approach |
|----------|--------------------|--------------------|
| HTTP requests | `curl` command | `http.get()` (already exists) |
| JSON processing | `jq` command | `json` module (already exists) |
| Git operations | `git` command | Git library via FFI |
| Image processing | `ffmpeg` command | libavcodec via FFI |

Shelling out to external tools is antithetical to self-sufficiency. The preferred pattern is libraries/FFI, not subprocess wrappers.

### When Subprocess Might Be Needed

The primary legitimate use case is **build tool integration** - running compilers, test runners, and other development tools. If Graphoid develops a package manager or build system, subprocess may become necessary.

### Decision

**Defer subprocess until a clear, unavoidable use case emerges.**

If subprocess is eventually implemented, it should:
- Default to direct execution (no shell)
- Require arguments as array (never string concatenation)
- Log all calls to the effect graph for auditing
- Potentially support allowlists for permitted commands
- Require explicit opt-in for shell mode with clear warnings

For users who need subprocess today, FFI provides direct access to `fork`/`exec` syscalls - making the danger explicit and intentional.

---

## Related Documents

- [PHASE_20_FFI.md](PHASE_20_FFI.md) - For compress, archive (and subprocess if needed later)
- [PHASE_18_COMPLETE_GRAPH_MODEL.md](PHASE_18_COMPLETE_GRAPH_MODEL.md) - Graph structure patterns
