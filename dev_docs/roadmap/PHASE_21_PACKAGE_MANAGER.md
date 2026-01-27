# Phase 21: Package Manager

**Duration**: 16-24 days
**Priority**: High
**Dependencies**: Phase 17 (Modules Graph) for universe integration
**Status**: Ready to Begin

---

## Goal

Graph-based dependency management system enabling code sharing, versioning, and reproducible builds. The package manager dogfoods Graphoid's graph algorithms for dependency resolution and integrates packages into the universe graph.

---

## Design Principles

1. **Consistent with Modules** - Packages work like modules (public by default, same import syntax)
2. **Graph-Centric** - Dependencies are a graph, resolution uses graph algorithms
3. **Simple Names** - Package names are valid identifiers (no hyphens, no conversion)
4. **Default Aliases** - Packages define short aliases, reducing need for manual aliasing
5. **Clean Privacy** - Use `priv` keyword/blocks, not ugly underscore prefixes
6. **Universe Integration** - Packages live in `universe:packages` subgraph

---

## Package Import and Namespace

### Basic Import

```graphoid
# Import a package - name becomes namespace
import "better_json"
better_json.parse(text)

# Package defines alias "bj" in manifest - works automatically
bj.parse(text)
```

### Selective Import

```graphoid
# Import only specific items into local scope
import "better_json", only: [parse, stringify]
parse(text)           # In local scope
stringify(data)       # In local scope
better_json.validate(x)  # Full namespace still works

# Exclude specific items (rare, but available)
import "better_json", except: [deprecated_func]
```

### Custom Alias (Discouraged)

```graphoid
# Override the default alias (rarely needed)
import "better_json" as json
json.parse(text)
```

Discouraged because packages should define good default aliases.

### Package Naming Rules

Package names must be **valid Graphoid identifiers**:

```
Valid:
  better_json     ✓
  graph_db        ✓
  http_client     ✓
  mypackage       ✓

Invalid:
  better-json     ✗  (hyphens not allowed)
  123package      ✗  (can't start with number)
  my.package      ✗  (dots not allowed)
```

**Rationale**: No conversion logic needed. The package name IS the namespace directly.

### Accessing Package Contents

```graphoid
import "graph_db"

# Functions
result = graph_db.query("...")

# Classes
db = graph_db.Database("localhost")

# Graph types defined by package
kg = graph_db.KnowledgeGraph()
kg.add_entity("Alice")

# Constants
port = graph_db.DEFAULT_PORT
```

---

## Privacy Model

Privacy works the same in packages and modules: **public by default**, with explicit `priv` marking.

### Single Item Privacy

```graphoid
# Public (default)
fn parse(text) { ... }
fn stringify(data) { ... }
MAX_DEPTH = 100

# Private - single item
priv fn tokenize(text) { ... }
priv BUFFER_SIZE = 4096
```

### Private Blocks

**Note**: `priv { }` block syntax is implemented in Phase 17 (Modules Graph) and works identically for packages.

```graphoid
# Group multiple private items
priv {
    fn helper1() { ... }
    fn helper2() { ... }
    fn validate_internal(x) { ... }
    INTERNAL_CONST = 42
}

# Public items outside the block
fn parse(text) {
    tokens = tokenize(text)  # Can call priv fn internally
    return process(tokens)
}
```

### Multiple Private Blocks

Multiple priv blocks are allowed (useful for organizing code):

```graphoid
fn parse(text) { ... }

priv {
    fn tokenize(text) { ... }
    fn validate(tokens) { ... }
}

fn stringify(data) { ... }

priv {
    fn format_output(tree) { ... }
    fn escape_string(s) { ... }
}
```

### Nested Private Blocks (Warning)

Nesting priv blocks is redundant and generates a warning:

```graphoid
priv {
    fn outer() { ... }

    priv {  # WARNING: Nested priv block is redundant
        fn inner() { ... }
    }
}
```

### Access Rules

```graphoid
import "better_json"

better_json.parse(text)      # OK - public
better_json.tokenize(text)   # ERROR: 'tokenize' is private in better_json
```

---

## Package Structure

```
my_package/
├── graphoid.toml          # Package manifest
├── src/
│   ├── lib.gr             # Main entry point
│   ├── parser.gr          # Additional module
│   └── utils.gr           # Additional module
├── tests/
│   └── parser.spec.gr     # Tests
└── README.md
```

### Entry Point (lib.gr)

```graphoid
# src/lib.gr - Package entry point

# Import internal modules
import "./parser"
import "./utils"

# Public API
fn parse(text) {
    tokens = parser.tokenize(text)
    return build_ast(tokens)
}

fn stringify(data) {
    return utils.format(data)
}

class JsonParser {
    fn new(options = {}) { ... }
    fn parse(text) { ... }
}

MAX_DEPTH = 100
VERSION = "1.2.3"

# Private helpers
priv {
    fn build_ast(tokens) { ... }
    fn validate(ast) { ... }
}
```

---

## Package Manifest (graphoid.toml)

```toml
[package]
name = "better_json"
alias = "bj"                    # Short alias (optional, recommended)
version = "1.2.3"
description = "Fast JSON parsing for Graphoid"
authors = ["Alice <alice@example.com>"]
license = "MIT"
repository = "https://github.com/alice/better_json"

# Entry points
main = "src/main.gr"            # For executables
lib = "src/lib.gr"              # For libraries

# Minimum Graphoid version
graphoid_version = ">=0.5.0"

[dependencies]
string_utils = "^2.0.0"         # Caret: 2.x.x compatible
data_structs = "~1.4.0"         # Tilde: 1.4.x only
exact_version = "1.0.0"         # Exact version

# From git
internal_lib = { git = "https://github.com/org/lib", tag = "v1.0.0" }

# From local path (development)
local_module = { path = "../local_module" }

[dev_dependencies]
test_helpers = "^1.0.0"
mock_server = "^2.0.0"

[scripts]
test = "gr spec tests/"
docs = "gr doc generate"
build = "gr build --release"
```

---

## Lock File (graphoid.lock)

```toml
# Auto-generated, do not edit

[[package]]
name = "better_json"
version = "1.2.3"
source = "registry+https://packages.graphoid.org"
checksum = "sha256:abc123..."
dependencies = ["string_utils 2.1.0"]

[[package]]
name = "string_utils"
version = "2.1.0"
source = "registry+https://packages.graphoid.org"
checksum = "sha256:def456..."
dependencies = []
```

---

## Universe Graph Integration

Packages live in the `universe:packages` subgraph, enabling graph queries.

### Package Graph Structure

```
universe
└── packages (subgraph)
    ├── package:better_json@1.2.3
    │   ├── type ──► type:Package
    │   ├── version ──► "1.2.3"
    │   ├── alias ──► "bj"
    │   ├── depends_on ──► [package:string_utils@2.1.0]
    │   ├── depended_by ──► [package:my_app@1.0.0]
    │   ├── exports ──► [parse, stringify, JsonParser, MAX_DEPTH]
    │   ├── registry ──► "packages.graphoid.org"
    │   ├── checksum ──► "sha256:abc123..."
    │   └── installed_at ──► "/home/user/.graphoid/packages/..."
    │
    ├── package:string_utils@2.1.0
    │   └── ...
    │
    └── package:my_app@1.0.0
        └── ...
```

### Querying Packages

```graphoid
# Get all installed packages
packages = reflect.universe().packages()
for pkg in packages {
    print(pkg.name + "@" + pkg.version)
}

# Get specific package info
bj = reflect.universe().packages()["better_json"]
bj.version       # "1.2.3"
bj.exports       # [parse, stringify, JsonParser, MAX_DEPTH]
bj.dependencies  # [package:string_utils@2.1.0]

# Traverse dependency graph
deps = reflect.universe().packages().traverse(
    from: "my_app",
    edge: :depends_on,
    depth: :all
)

# Impact analysis: what depends on this package?
dependents = reflect.universe().packages().traverse(
    from: "string_utils",
    edge: :depended_by,
    depth: :all
)

# Find packages with security issues
vulnerable = reflect.universe().packages().query({
    has_edge: { to: "known_vulnerable_pkg" }
})
```

---

## Graph-Based Dependency Resolution

The resolver uses Graphoid's graph algorithms (dogfooding!).

### Resolution Algorithm

```graphoid
# Written in Graphoid - will be self-hosted eventually

fn resolve_dependencies(manifest) {
    # Phase 1: Build constraint graph
    constraint_graph = graph { type: :directed }

    for dep in manifest.dependencies {
        add_package_constraints(constraint_graph, dep)
    }

    # Phase 2: Check for cycles
    if constraint_graph.has_cycle() {
        cycle = constraint_graph.find_cycle()
        raise DependencyError {
            message: "Circular dependency detected",
            cycle: cycle
        }
    }

    # Phase 3: Resolve versions (hybrid: graph + constraint solving)
    resolution = resolve_versions(constraint_graph)

    if resolution == none {
        conflicts = find_conflicts(constraint_graph)
        raise DependencyError {
            message: "Version conflict",
            conflicts: conflicts,
            graph: conflicts.to_subgraph()  # Subgraph showing the conflict
        }
    }

    # Phase 4: Determine install order via topological sort
    install_order = resolution.topological_sort()

    # Phase 5: Identify parallel install groups via graph coloring
    parallel_groups = resolution.chromatic_partition()

    return {
        packages: resolution.nodes(),
        order: install_order,
        parallel_groups: parallel_groups
    }
}

priv {
    fn add_package_constraints(graph, dep) {
        versions = registry.get_versions(dep.name)
        compatible = versions.filter(v => dep.constraint.satisfies(v))

        for version in compatible {
            graph.add_node(dep.name + "@" + version, {
                package: dep.name,
                version: version
            })

            # Add edges for this version's dependencies
            pkg_manifest = registry.get_manifest(dep.name, version)
            for sub_dep in pkg_manifest.dependencies {
                graph.add_edge(
                    dep.name + "@" + version,
                    sub_dep.name,
                    { constraint: sub_dep.constraint }
                )
            }
        }
    }

    fn resolve_versions(constraint_graph) {
        # Find subgraph where:
        # - Exactly one version per package name
        # - All constraint edges satisfied

        # This uses constraint propagation + backtracking
        # Graph structure guides the search

        packages = constraint_graph.nodes().group_by(n => n.package)

        return backtrack_solve(packages, constraint_graph, {})
    }

    fn find_conflicts(graph) {
        # Return minimal unsatisfiable core as subgraph
        # Helps users understand why resolution failed
        return graph.find_unsatisfiable_core()
    }
}
```

### Graph Algorithms Used

| Task | Algorithm | Graph Operation |
|------|-----------|-----------------|
| Cycle detection | DFS with back-edges | `graph.has_cycle()`, `graph.find_cycle()` |
| Install order | Topological sort | `graph.topological_sort()` |
| Parallel groups | Graph coloring | `graph.chromatic_partition()` |
| Conflict analysis | Subgraph extraction | `graph.find_unsatisfiable_core()` |
| Impact analysis | Reachability | `graph.traverse(edge: :depended_by)` |
| Dependency tree | Tree extraction | `graph.as_tree(root: pkg)` |

### Conflict Explanation

When resolution fails, show the conflict as a graph:

```graphoid
# gr install output on conflict:

ERROR: Dependency conflict detected

  my_app@1.0.0
      │
      ├── depends_on ──► json_parser@^2.0.0
      │                      │
      │                      └── needs json_parser >= 2.0.0
      │
      └── depends_on ──► http_client@1.5.0
                             │
                             └── depends_on ──► json_parser@^1.0.0
                                                    │
                                                    └── needs json_parser < 2.0.0

  CONFLICT: json_parser >= 2.0.0 AND json_parser < 2.0.0 cannot both be satisfied

  Suggestions:
  - Update http_client to version >= 2.0.0 (supports json_parser@^2.0.0)
  - Pin json_parser to version 1.9.0 (compatible with both constraints)
```

---

## CLI Commands

```bash
# Create new project
gr new my_project
gr new my_library --lib

# Add dependencies
gr add string_utils
gr add string_utils@^2.0.0
gr add --dev test_helpers

# Remove dependencies
gr remove string_utils

# Install all dependencies
gr install

# Update dependencies
gr update
gr update string_utils

# Show dependency graph (ASCII rendering)
gr tree
gr tree --graph  # Full graph view, not just tree

# Run scripts
gr run test
gr run build

# Publish to registry
gr publish

# Search packages
gr search "json"

# Package info
gr info string_utils
```

### Dependency Tree Output

```bash
$ gr tree

my_app v1.0.0
├── better_json v1.2.3 (alias: bj)
│   └── string_utils v2.1.0
├── http_client v1.5.0
│   ├── socket_lib v0.5.0
│   └── string_utils v2.1.0  (shared)
└── [dev] test_helpers v1.0.0
```

### Dependency Graph Output

```bash
$ gr tree --graph

my_app@1.0.0
    │
    ├───────────────┬─────────────────┐
    ▼               ▼                 ▼
better_json     http_client      [dev] test_helpers
  @1.2.3          @1.5.0              @1.0.0
    │               │
    │          ┌────┴────┐
    ▼          ▼         ▼
string_utils  socket_lib  string_utils
  @2.1.0       @0.5.0      @2.1.0
    │                        │
    └────────────────────────┘
           (same package)
```

---

## Registry Integration

### API Endpoints

```
packages.graphoid.org/api/

GET  /packages?q={query}           # Search
GET  /packages/{name}              # Package info
GET  /packages/{name}/{version}    # Specific version
GET  /packages/{name}/{version}/download  # Download tarball
PUT  /packages/{name}              # Publish (authenticated)
```

### Authentication

```bash
$ gr login
Opening browser for authentication...
Logged in as alice@example.com
Token stored in ~/.graphoid/credentials

$ gr publish
Publishing my_package v1.0.0...
Authenticated as alice@example.com
Published to packages.graphoid.org!
```

### Package Storage

```
packages.graphoid.org/
├── api/
│   └── packages/
│       └── {name}/
│           └── {version}/
│               ├── metadata.json
│               └── package.tar.gz
├── index/
│   └── {first_two_chars}/
│       └── {name}.json
└── search/
    └── index.json
```

---

## Implementation Plan

### Phase 21a: Core Infrastructure (Days 1-5)

| Day | Task |
|-----|------|
| 1-2 | Package manifest (graphoid.toml) parsing |
| 3-4 | Lock file (graphoid.lock) generation and parsing |
| 5 | Package naming validation, alias resolution |

### Phase 21b: Dependency Resolution (Days 6-10)

| Day | Task |
|-----|------|
| 6-7 | Constraint graph construction |
| 8 | Cycle detection, topological sort |
| 9 | Version resolution (constraint solving) |
| 10 | Conflict detection and explanation |

### Phase 21c: Universe Integration (Days 11-13)

| Day | Task |
|-----|------|
| 11 | Package nodes in universe:packages subgraph |
| 12 | Dependency edges (depends_on, depended_by) |
| 13 | Query API: `reflect.universe().packages()` |

### Phase 21d: CLI and Registry (Days 14-19)

| Day | Task |
|-----|------|
| 14-15 | CLI commands: new, add, remove, install |
| 16-17 | Registry client: search, download, publish |
| 18-19 | Authentication, caching |

### Phase 21e: Visualization and Polish (Days 20-24)

| Day | Task |
|-----|------|
| 20-21 | `gr tree` with ASCII graph rendering |
| 22-23 | Parallel installation using graph coloring |
| 24 | Documentation, examples, testing |

---

## Success Criteria

### Core Functionality
- [ ] `graphoid.toml` manifest parsing with alias support
- [ ] `graphoid.lock` generation and parsing
- [ ] Package names must be valid identifiers (enforced)
- [ ] Semantic version resolution
- [ ] `priv { }` blocks work in packages (implemented in Phase 17, shared with modules)

### Graph Integration
- [ ] Packages in `universe:packages` subgraph
- [ ] `depends_on` and `depended_by` edges
- [ ] `reflect.universe().packages()` query API
- [ ] Dependency traversal via graph operations

### Resolution
- [ ] Constraint graph construction
- [ ] Cycle detection via graph algorithm
- [ ] Topological sort for install order
- [ ] Graph-based conflict explanation
- [ ] Parallel groups via graph coloring

### CLI
- [ ] `gr new`, `gr add`, `gr remove` commands
- [ ] `gr install` with caching
- [ ] `gr update` with lock file update
- [ ] `gr tree` with ASCII graph rendering
- [ ] `gr tree --graph` for full dependency graph
- [ ] `gr publish` to registry
- [ ] `gr search` and `gr info`

### Testing and Documentation
- [ ] At least 60 package manager tests
- [ ] Example: Multi-package project
- [ ] Example: Conflict resolution
- [ ] Example: Graph queries on packages
- [ ] Documentation complete

---

## Open Questions (Resolved)

| Question | Resolution |
|----------|------------|
| Import syntax? | `import "pkg"` and `import "pkg", only: [...]` |
| Package naming? | Valid identifiers only (no hyphens) |
| Alias handling? | Default alias in manifest, automatic |
| Privacy model? | `priv` keyword/blocks, public by default |
| `priv { }` blocks? | Implemented in Phase 17, shared by modules and packages |
| Universe integration? | `universe:packages` subgraph |
| Resolution algorithm? | Hybrid: graph structure + constraint solving |
| Conflict display? | ASCII graph showing conflict subgraph |

---

## Related Documents

- [PHASE_17_MODULES_GRAPH.md](PHASE_17_MODULES_GRAPH.md) - Module system foundation
- [PHASE_20_FFI.md](PHASE_20_FFI.md) - FFI wrappers as packages
- [PHASE_22_DATABASE.md](PHASE_22_DATABASE.md) - Database drivers as packages
