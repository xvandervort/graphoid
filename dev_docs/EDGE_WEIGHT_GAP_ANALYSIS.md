# Edge Weight Gap Analysis

**Critical Finding**: Edge weights are underspecified in the current implementation

---

## What the Spec Says

**Line 250**: `add_edge(from, to, edge_type, weight?)`
- Weight is an **optional direct parameter**, not a property

**Lines 393-394**: Graph Rules
- `weighted_edges` - Edges must have weights
- `unweighted_edges` - Edges must not have weights

**Line 570**:
```graphoid
# Weighted shortest path (Dijkstra's algorithm)
path = graph.shortest_path("A", "Z", weighted: true)
```

**Lines 608, 619**: Edge filtering examples
```graphoid
edges: e => e.weight > 0.5
edges: e => e.weight < 0.1
```

**Conclusion**: Edge weights are a **first-class feature**, with:
- Direct parameter in `add_edge()`
- `.weight` accessor on edges
- Validation rules
- Weighted pathfinding algorithms

---

## What the Implementation Has

**Current signature**:
```rust
pub fn add_edge(&mut self, from: &str, to: &str, edge_type: String, properties: HashMap<String, Value>)
```

**Edge structure**:
```rust
pub struct EdgeInfo {
    pub edge_type: String,
    pub properties: HashMap<String, Value>,
}
```

**Findings**:
- ❌ No `weight` field on edges
- ❌ No weight parameter in `add_edge()`
- ❌ No `.weight` accessor
- ❌ Rules `weighted_edges` and `unweighted_edges` don't exist
- ⚠️ Could theoretically store weight in `properties` hash, but spec doesn't work this way

**Grep results**: Zero mentions of "weight" or "Weight" in `src/values/graph.rs`

---

## The Gap

Edge weights are **not implemented** as a first-class feature. Current implementation only has generic `properties` hash, which is insufficient because:

1. **Spec syntax won't work**: `e.weight` accessor doesn't exist
2. **add_edge() signature wrong**: Missing `weight?` parameter
3. **Rules don't exist**: Can't enforce `weighted_edges` or `unweighted_edges`
4. **Weighted pathfinding impossible**: Dijkstra needs edge weights to work

---

## What Needs to Be Implemented

### 1. Edge Structure Update
```rust
pub struct EdgeInfo {
    pub edge_type: String,
    pub weight: Option<f64>,  // NEW: First-class weight field
    pub properties: HashMap<String, Value>,
}
```

### 2. add_edge() Signature Update
```rust
pub fn add_edge(
    &mut self,
    from: &str,
    to: &str,
    edge_type: String,
    weight: Option<f64>,  // NEW: Optional weight parameter
    properties: HashMap<String, Value>
) -> Result<(), GraphoidError>
```

**Or overloaded versions**:
```rust
// Simple version (no weight, no properties)
pub fn add_edge(&mut self, from: &str, to: &str, edge_type: String)

// With weight
pub fn add_edge_weighted(&mut self, from: &str, to: &str, edge_type: String, weight: f64)

// Full version
pub fn add_edge_full(&mut self, from: &str, to: &str, edge_type: String, weight: Option<f64>, properties: HashMap<String, Value>)
```

### 3. Edge Accessor
```rust
impl Edge {
    pub fn weight(&self) -> Option<f64> {
        self.weight
    }
}
```

### 4. Graph Rules
- Implement `weighted_edges` rule (validates all edges have weights)
- Implement `unweighted_edges` rule (validates no edges have weights)

### 5. Weighted Pathfinding
- Implement Dijkstra's algorithm
- Support `graph.shortest_path(from, to, weighted: true)` syntax

---

## Impact on Level 4 Completion

**Original assessment**: Missing 3 methods
**Revised assessment**: Missing edge weight infrastructure + 3 methods

**Work required**:
- Add weight field to edge structure (breaking change!)
- Update all existing `add_edge()` calls in codebase
- Implement weight accessor
- Implement weighted/unweighted rules
- Implement Dijkstra's algorithm
- Implement 3 missing Level 4 methods

**Estimated effort**: 3-4 days (not 2 days as originally thought)

---

## Implemented Solution: Phase 6.6

**Decision**: Created Phase 6.6 (3-4 days) before Phase 7

### Phase 6.6 Structure

**Duration**: 3-4 days
**Location**: Before Phase 7 (Function Pattern Matching)

**Part A (Days 1-2)**: Edge Weight Infrastructure
- Add weight field to EdgeInfo structure
- Update add_edge() signature to include weight parameter
- Implement weight mutation methods (get, set, remove)
- Implement weight rules (weighted_edges, unweighted_edges)
- Migrate all existing add_edge() calls throughout codebase
- 35 tests for weight infrastructure

**Part B (Days 3-4)**: Complete Level 4
- Implement Dijkstra's weighted shortest path algorithm
- Add edge_type and weighted parameters to shortest_path()
- Implement nodes_within(node, hops: N) method
- Add edge type filtering to all path methods
- 30 tests for Level 4 completion

**Total**: 65+ new tests, breaking changes properly handled

**Why This Approach**:
- Clean separation of concerns (infrastructure first, then algorithms)
- Breaking changes done before stdlib work
- Level 4 complete before pattern matching (Phase 9)
- All graph querying levels complete before Levels 3 & 5 implementation

---

## Breaking Changes Note

Adding the weight field to EdgeInfo is a **breaking change** that requires:
1. Updating all existing `add_edge()` calls throughout codebase
2. Updating serialization/deserialization
3. Updating tests
4. Migration path for existing graphs (if any)

This is why it should be done **before** stdlib work (Phases 11-12) where graphs might be used extensively.

---

## Conclusion

**Answer to original question**: No, edges do NOT currently support weights in a way that matches the specification. While edges have a generic `properties` hash that could theoretically hold a weight, the spec treats weights as a first-class feature with:
- Direct parameter syntax
- Dedicated accessor
- Validation rules
- Specialized algorithms

Implementing weighted shortest path requires first implementing the edge weight infrastructure.

**Resolution**: ✅ Created Phase 6.6 (3-4 days) to properly implement edge weights and complete Level 4 before attempting pattern matching work.

**See**: `PHASE_6_6_DETAILED_PLAN.md` for complete implementation details including:
- Edge weight infrastructure specification
- Weight mutation API
- Weighted pathfinding algorithms
- Migration guide for existing code
- 65+ test specifications
