# Filter-Based Subgraph Operations

## Overview

Phase 9 Level 5 filter-based subgraph operations are **fully implemented** at both Rust API and .gr file levels.

**Status**: ✅ Rust API complete | ✅ .gr file syntax complete

## Available Methods

### 1. `extract_filtered()` - Extract subgraph by filter

Extract a subgraph containing only nodes and edges matching the provided filters.

**Rust API**:
```rust
pub fn extract_filtered(
    &self,
    node_filter: Option<Box<dyn Fn(&str, &Value) -> bool>>,
    edge_filter: Option<Box<dyn Fn(&str, &str, &str, Option<f64>, &HashMap<String, Value>) -> bool>>,
    include_orphans: bool,
) -> Result<Graph, GraphoidError>
```

**Example**:
```rust
// Extract nodes with value > 2
let node_filter = Box::new(|_id: &str, val: &Value| -> bool {
    if let ValueKind::Number(n) = &val.kind {
        *n > 2.0
    } else {
        false
    }
});

let result = graph.extract_filtered(Some(node_filter), None, true)?;
```

**.gr Syntax** (fully implemented):

**Positional Syntax**:
```graphoid
# Extract nodes with specific properties
node_filter = n => n > 100
active_users = graph.extract(node_filter, none, true)

# Extract with both node and edge filters
node_filter = n => n > 50
edge_filter = (from, to, edge_type) => edge_type == "follows"
social_network = graph.extract(node_filter, edge_filter, false)

# Extract by edges only (exclude orphan nodes)
edge_filter = (from, to, edge_type) => edge_type == "friend"
connected_only = graph.extract(none, edge_filter, false)
```

**Inline Block Syntax** (NEW - fully working):
```graphoid
# Inline with node filter
active_users = graph.extract({
    nodes: n => n > 100,
    include_orphans: true
})

# Inline with edge filter
friends_only = graph.extract({
    edges: (from, to, etype) => etype == "friend",
    include_orphans: false
})

# Inline with both filters
filtered = graph.extract({
    nodes: n => n.active == true,
    edges: (from, to, etype) => etype == "follows",
    include_orphans: false
})
```

### 2. `delete_filtered()` - Delete subgraph by filter

Returns a new graph WITHOUT elements matching the filters (inverse of extract).

**Rust API**:
```rust
pub fn delete_filtered(
    &self,
    node_filter: Option<Box<dyn Fn(&str, &Value) -> bool>>,
    edge_filter: Option<Box<dyn Fn(&str, &str, &str, Option<f64>, &HashMap<String, Value>) -> bool>>,
) -> Result<Graph, GraphoidError>
```

**Example**:
```rust
// Delete nodes where value < 2
let node_filter = Box::new(|_id: &str, val: &Value| -> bool {
    if let ValueKind::Number(n) = &val.kind {
        *n < 2.0
    } else {
        false
    }
});

let cleaned = graph.delete_filtered(Some(node_filter), None)?;
```

**.gr Syntax** (fully implemented):

**Positional Syntax**:
```graphoid
# Delete low-value nodes
node_filter = n => n < 50
cleaned = graph.delete(node_filter, none)

# Remove specific edge types
edge_filter = (from, to, edge_type) => edge_type == "temporary"
simplified = graph.delete(none, edge_filter)
```

**Inline Block Syntax** (NEW - fully working):
```graphoid
# Inline delete with node filter
cleaned = graph.delete({
    nodes: n => n.deleted == true
})

# Inline delete with edge filter
simplified = graph.delete({
    edges: (from, to, etype) => etype == "temporary"
})
```

### 3. `add_subgraph()` - Merge graphs with conflict resolution

Merges two graphs with configurable conflict resolution strategies.

**Rust API**:
```rust
pub fn add_subgraph(
    &self,
    other: &Graph,
    on_conflict: Option<String>,
) -> Result<Graph, GraphoidError>
```

**Conflict Strategies**:
- `"keep_original"` (default): Keep existing node values on conflict
- `"overwrite"`: Replace with new values on conflict
- `"merge"`: Combine attributes (currently same as overwrite)

**Example**:
```rust
// Merge with keep_original strategy
let merged = graph_a.add_subgraph(&graph_b, Some("keep_original".to_string()))?;

// Merge with overwrite strategy
let merged = graph_a.add_subgraph(&graph_b, Some("overwrite".to_string()))?;

// Merge with default strategy (keep_original)
let merged = graph_a.add_subgraph(&graph_b, None)?;
```

**.gr Syntax** (implemented):
```graphoid
# Basic merge (keep_original is default)
combined = graph_a.add_subgraph(graph_b, "keep_original")

# Overwrite conflicting nodes
merged = graph_a.add_subgraph(graph_b, "overwrite")

# Merge strategy (combines attributes)
merged = graph_a.add_subgraph(graph_b, "merge")
```

## Test Coverage

✅ **16 comprehensive tests** covering:
- Node-only filtering
- Edge-only filtering
- Combined node and edge filtering
- Orphan node handling
- Conflict resolution strategies
- Edge cases (empty graphs, no matches, all match)

See: `tests/filter_based_subgraph_tests.rs`

## Implementation Status

| Feature | Rust API | .gr Syntax | Tests | Integration Tests |
|---------|----------|------------|-------|-------------------|
| `extract_filtered()` | ✅ Complete | ✅ Complete | ✅ 9 tests | ✅ Verified |
| `delete_filtered()` | ✅ Complete | ✅ Complete | ✅ 4 tests | ✅ Verified |
| `add_subgraph()` | ✅ Complete | ✅ Complete | ✅ 3 tests | ✅ Verified |
| `node_count()` | ✅ Complete | ✅ Complete | N/A | ✅ Verified |
| `edge_count()` | ✅ Complete | ✅ Complete | N/A | ✅ Verified |

## Implementation Details

The .gr syntax supports **two fully-working approaches**:

### 1. Positional Arguments
- **Node filters**: Single-parameter lambdas `n => expression` operating on node values
- **Edge filters**: Three-parameter lambdas `(from, to, edge_type) => expression`
- **Arguments**: Positional (node_filter, edge_filter, include_orphans)
- **None values**: Use `none` keyword for optional parameters

### 2. Inline Block Syntax ✅ NEW!
- **Inline literal syntax**: `graph.extract({ nodes: ..., edges: ..., include_orphans: ... })`
- **Lambdas as map values**: Fully supported
- **Multiline formatting**: Supported (newlines handled correctly)
- **Parser enhancement**: Complete (lambdas + newline handling)

The executor intelligently detects whether arguments are positional or block-style and handles both.

### Parser Enhancements Completed
- ✅ Map values now support lambda expressions
- ✅ Map literals support multiline formatting
- ✅ Both `hash{...}` and `{...}` map syntaxes updated

## Existing Subgraph Operations

The existing root/depth-based subgraph operations remain available:

```rust
// Extract from root node with optional depth limit
let subgraph = graph.extract_subgraph("root_id", Some(2))?;

// Insert subgraph with attachment point
graph.insert_subgraph(&other, "attach_to", "link_type")?;
```

**Tests**: `tests/unit/subgraph_operations_tests.rs` (17 tests)

## Usage Guide

Users can choose between two APIs depending on their needs:

### Rust API (Internal/Advanced)
- **Use when**: Building Rust applications, need programmatic control
- **Filters**: Rust closures with full type safety
- **Example**: `graph.extract_filtered(Some(node_filter), None, true)?`

### .gr Syntax (User-Facing)
- **Use when**: Writing .gr programs, need simple filtering
- **Filters**: Lambda expressions in .gr syntax
- **Example**: `filtered = graph.extract(n => n > 100, none, true)`

Both APIs are production-ready and fully supported.

## Example Files

See working examples in `/home/irv/work/grang/tmp/`:
- `test_extract.gr` - Extract nodes by value filter
- `test_delete.gr` - Delete nodes by value filter
- `test_add_subgraph.gr` - Merge graphs with conflict resolution
- `filter_subgraph_demo.gr` - Comprehensive demonstration of all features
