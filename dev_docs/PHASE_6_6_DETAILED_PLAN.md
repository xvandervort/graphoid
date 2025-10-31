# Phase 6.6: Edge Weights & Complete Level 4 Graph Querying

**Duration**: 3-4 days
**Status**: Not started - NEW phase
**Goal**: Implement first-class edge weight support and complete Level 4 graph querying

---

## Overview

Phase 6.6 adds **first-class edge weight support** to the graph system and completes the missing Level 4 features. Edge weights are currently missing from the implementation but are specified as a core feature in the language spec (§250, §393-394, §570, §608).

**Why This Phase is Critical**:
- Edge weights are first-class in the spec, not properties
- Required for weighted pathfinding (Dijkstra's algorithm)
- Enables graph rules: `weighted_edges`, `unweighted_edges`
- Completes Level 4 querying (currently 67% done)
- Must be done before stdlib (breaking change to edge structure)

**Current Status**:
- ❌ Edge weights completely missing
- ❌ `add_edge()` signature doesn't match spec
- ✅ Level 4: 6 of 9 methods implemented (67%)
- ❌ Level 4: 3 methods missing (33%)

**Dependencies**: Must come before Phase 7 (stdlib uses graphs)

---

## Part A: Edge Weight Infrastructure (Days 1-2)

### Day 1: Add Weight Field and Update Signatures

#### 1.1 Update Edge Structure

**File**: `src/values/graph.rs`

**Current**:
```rust
pub struct EdgeInfo {
    pub edge_type: String,
    pub properties: HashMap<String, Value>,
}
```

**Updated**:
```rust
pub struct EdgeInfo {
    pub edge_type: String,
    pub weight: Option<f64>,  // NEW: First-class weight field
    pub properties: HashMap<String, Value>,
}

impl EdgeInfo {
    /// Create new edge with no weight
    pub fn new(edge_type: String, properties: HashMap<String, Value>) -> Self {
        EdgeInfo {
            edge_type,
            weight: None,
            properties,
        }
    }

    /// Create new edge with weight
    pub fn new_weighted(edge_type: String, weight: f64, properties: HashMap<String, Value>) -> Self {
        EdgeInfo {
            edge_type,
            weight: Some(weight),
            properties,
        }
    }

    /// Get weight (returns None if unweighted)
    pub fn weight(&self) -> Option<f64> {
        self.weight
    }

    /// Set weight
    pub fn set_weight(&mut self, weight: Option<f64>) {
        self.weight = weight;
    }

    /// Check if edge is weighted
    pub fn is_weighted(&self) -> bool {
        self.weight.is_some()
    }
}
```

#### 1.2 Update add_edge() Signature

**File**: `src/values/graph.rs`

**From Spec** (§250): `add_edge(from, to, edge_type, weight?)`

**New signature**:
```rust
pub fn add_edge(
    &mut self,
    from: &str,
    to: &str,
    edge_type: String,
    weight: Option<f64>,
    properties: HashMap<String, Value>
) -> Result<(), GraphoidError>
```

**Implementation**:
```rust
pub fn add_edge(
    &mut self,
    from: &str,
    to: &str,
    edge_type: String,
    weight: Option<f64>,
    properties: HashMap<String, Value>
) -> Result<(), GraphoidError> {
    // Validate the operation against active rules
    let operation = GraphOperation::AddEdge {
        from: from.to_string(),
        to: to.to_string(),
        edge_type: edge_type.clone(),
        weight,
        properties: properties.clone(),
    };

    match self.validate_rules(operation) {
        ValidationResult::Allowed => {
            // Get or create nodes
            if !self.nodes.contains_key(from) {
                return Err(GraphoidError::runtime_error(
                    format!("Source node '{}' does not exist", from),
                    None
                ));
            }
            if !self.nodes.contains_key(to) {
                return Err(GraphoidError::runtime_error(
                    format!("Target node '{}' does not exist", to),
                    None
                ));
            }

            // Create edge info with weight
            let edge_info = if let Some(w) = weight {
                EdgeInfo::new_weighted(edge_type.clone(), w, properties)
            } else {
                EdgeInfo::new(edge_type.clone(), properties)
            };

            // Add to adjacency list
            let from_node = self.nodes.get_mut(from).unwrap();
            from_node.neighbors.insert(to.to_string(), edge_info.clone());

            // If undirected, add reverse edge
            if self.graph_type == GraphType::Undirected {
                let to_node = self.nodes.get_mut(to).unwrap();
                to_node.neighbors.insert(from.to_string(), edge_info);
            }

            Ok(())
        }
        ValidationResult::Rejected { reason } => {
            Err(GraphoidError::rule_violation(reason, None))
        }
    }
}
```

#### 1.3 Update All Existing add_edge() Calls

**Files to update**:
- `src/values/graph.rs` - Internal calls
- `src/execution/executor.rs` - Executor calls
- `tests/**/*.rs` - All test files

**Migration strategy**:
```rust
// Old call
graph.add_edge("A", "B", "edge".to_string(), HashMap::new())?;

// New call (no weight)
graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new())?;

// New call (with weight)
graph.add_edge("A", "B", "edge".to_string(), Some(5.0), HashMap::new())?;
```

**Automated migration**:
```bash
# Find all add_edge calls
grep -r "add_edge(" src/ tests/

# Update pattern (manual review required)
# Before: .add_edge("A", "B", "type", props)
# After:  .add_edge("A", "B", "type", None, props)
```

**Tests**: 20 tests for weight infrastructure
- Create weighted edge
- Create unweighted edge
- Get weight from weighted edge
- Get weight from unweighted edge (returns None)
- Set weight on existing edge
- Remove weight from edge
- Check is_weighted()
- Weight in edge accessor
- Multiple weighted edges
- Mixed weighted/unweighted graph

---

### Day 2: Edge Weight Rules and Accessors

#### 2.1 Implement weighted_edges Rule

**File**: `src/graph/rules.rs`

```rust
pub struct WeightedEdgesRule;

impl GraphRule for WeightedEdgesRule {
    fn name(&self) -> &str {
        "weighted_edges"
    }

    fn validate_operation(&self, graph: &Graph, operation: &GraphOperation) -> ValidationResult {
        match operation {
            GraphOperation::AddEdge { weight, .. } => {
                if weight.is_none() {
                    ValidationResult::Rejected {
                        reason: "weighted_edges rule requires all edges to have weights".to_string()
                    }
                } else {
                    ValidationResult::Allowed
                }
            }
            _ => ValidationResult::Allowed
        }
    }

    fn validate_structure(&self, graph: &Graph) -> ValidationResult {
        // Check all existing edges have weights
        for node in graph.nodes.values() {
            for edge_info in node.neighbors.values() {
                if !edge_info.is_weighted() {
                    return ValidationResult::Rejected {
                        reason: format!(
                            "weighted_edges rule violated: edge '{}' has no weight",
                            edge_info.edge_type
                        )
                    };
                }
            }
        }
        ValidationResult::Allowed
    }
}
```

#### 2.2 Implement unweighted_edges Rule

```rust
pub struct UnweightedEdgesRule;

impl GraphRule for UnweightedEdgesRule {
    fn name(&self) -> &str {
        "unweighted_edges"
    }

    fn validate_operation(&self, graph: &Graph, operation: &GraphOperation) -> ValidationResult {
        match operation {
            GraphOperation::AddEdge { weight, .. } => {
                if weight.is_some() {
                    ValidationResult::Rejected {
                        reason: "unweighted_edges rule forbids weights on edges".to_string()
                    }
                } else {
                    ValidationResult::Allowed
                }
            }
            _ => ValidationResult::Allowed
        }
    }

    fn validate_structure(&self, graph: &Graph) -> ValidationResult {
        // Check no edges have weights
        for node in graph.nodes.values() {
            for edge_info in node.neighbors.values() {
                if edge_info.is_weighted() {
                    return ValidationResult::Rejected {
                        reason: format!(
                            "unweighted_edges rule violated: edge '{}' has weight",
                            edge_info.edge_type
                        )
                    };
                }
            }
        }
        ValidationResult::Allowed
    }
}
```

#### 2.3 Edge Weight Mutation Methods

**File**: `src/values/graph.rs`

Add methods to get and set weights on existing edges:

```rust
impl Graph {
    /// Get edge weight (returns None if edge doesn't exist or is unweighted)
    pub fn get_edge_weight(&self, from: &str, to: &str) -> Option<f64> {
        self.nodes.get(from)?
            .neighbors.get(to)?
            .weight
    }

    /// Set weight on existing edge
    pub fn set_edge_weight(&mut self, from: &str, to: &str, weight: Option<f64>) -> Result<(), GraphoidError> {
        // Validate against rules first
        if let Some(from_node) = self.nodes.get_mut(from) {
            if let Some(edge_info) = from_node.neighbors.get_mut(to) {
                // Store old weight for rollback
                let old_weight = edge_info.weight;

                // Temporarily set new weight
                edge_info.weight = weight;

                // Validate with rules
                let operation = GraphOperation::UpdateEdge {
                    from: from.to_string(),
                    to: to.to_string(),
                    old_weight,
                    new_weight: weight,
                };

                match self.validate_rules(operation) {
                    ValidationResult::Allowed => {
                        // If undirected, update reverse edge too
                        if self.graph_type == GraphType::Undirected {
                            if let Some(to_node) = self.nodes.get_mut(to) {
                                if let Some(rev_edge) = to_node.neighbors.get_mut(from) {
                                    rev_edge.weight = weight;
                                }
                            }
                        }
                        Ok(())
                    }
                    ValidationResult::Rejected { reason } => {
                        // Rollback
                        edge_info.weight = old_weight;
                        Err(GraphoidError::rule_violation(reason, None))
                    }
                }
            } else {
                Err(GraphoidError::runtime_error(
                    format!("Edge from '{}' to '{}' does not exist", from, to),
                    None
                ))
            }
        } else {
            Err(GraphoidError::runtime_error(
                format!("Node '{}' does not exist", from),
                None
            ))
        }
    }

    /// Remove weight from edge (make it unweighted)
    pub fn remove_edge_weight(&mut self, from: &str, to: &str) -> Result<(), GraphoidError> {
        self.set_edge_weight(from, to, None)
    }

    /// Check if edge is weighted
    pub fn is_edge_weighted(&self, from: &str, to: &str) -> bool {
        self.get_edge_weight(from, to).is_some()
    }
}
```

#### 2.4 Executor Integration

**File**: `src/execution/executor.rs`

Support these operations:
```graphoid
# Get edge weight
weight = graph.get_edge_weight("A", "B")

# Set edge weight
graph.set_edge_weight("A", "B", 5.0)

# Remove weight
graph.remove_edge_weight("A", "B")

# Check if weighted
is_weighted = graph.is_edge_weighted("A", "B")
```

**Tests**: 15 tests for weight rules and mutation
- weighted_edges rule blocks unweighted edges
- unweighted_edges rule blocks weighted edges
- Get weight on existing edge
- Set weight on existing edge
- Remove weight from edge
- Set weight validates rules
- Undirected graph updates both directions
- Error: set weight on non-existent edge
- Error: set weight on non-existent node
- Retroactive rule validation

---

## Part B: Complete Level 4 Querying (Days 3-4)

### Day 3: Weighted Shortest Path (Dijkstra)

#### 3.1 Implement Dijkstra's Algorithm

**File**: `src/values/graph.rs`

```rust
impl Graph {
    /// Weighted shortest path using Dijkstra's algorithm
    fn shortest_path_weighted(&self, from: &str, to: &str) -> Vec<String> {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;

        #[derive(Debug)]
        struct State {
            node: String,
            cost: f64,
        }

        impl Ord for State {
            fn cmp(&self, other: &State) -> Ordering {
                // Min-heap: reverse ordering
                other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &State) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Eq for State {}
        impl PartialEq for State {
            fn eq(&self, other: &State) -> bool {
                self.cost == other.cost && self.node == other.node
            }
        }

        // Initialize distances
        let mut distances: HashMap<String, f64> = HashMap::new();
        let mut previous: HashMap<String, String> = HashMap::new();
        let mut heap = BinaryHeap::new();

        distances.insert(from.to_string(), 0.0);
        heap.push(State {
            node: from.to_string(),
            cost: 0.0,
        });

        while let Some(State { node, cost }) = heap.pop() {
            // Found destination
            if node == to {
                break;
            }

            // Already found better path
            if let Some(&best_cost) = distances.get(&node) {
                if cost > best_cost {
                    continue;
                }
            }

            // Explore neighbors
            if let Some(graph_node) = self.nodes.get(&node) {
                for (neighbor_id, edge_info) in &graph_node.neighbors {
                    // Get edge weight (default to 1.0 if unweighted)
                    let edge_weight = edge_info.weight.unwrap_or(1.0);
                    let new_cost = cost + edge_weight;

                    // Update if better path found
                    if !distances.contains_key(neighbor_id) || new_cost < distances[neighbor_id] {
                        distances.insert(neighbor_id.clone(), new_cost);
                        previous.insert(neighbor_id.clone(), node.clone());
                        heap.push(State {
                            node: neighbor_id.clone(),
                            cost: new_cost,
                        });
                    }
                }
            }
        }

        // Reconstruct path
        if !distances.contains_key(to) {
            return vec![]; // No path found
        }

        let mut path = vec![to.to_string()];
        let mut current = to.to_string();
        while current != from {
            if let Some(prev) = previous.get(&current) {
                path.push(prev.clone());
                current = prev.clone();
            } else {
                return vec![]; // Path broken
            }
        }
        path.reverse();
        path
    }
}
```

#### 3.2 Update shortest_path() with Options

**From Spec** (§565, §571):
```graphoid
# With edge type constraint
path = graph.shortest_path("A", "Z", edge_type: :FRIEND)

# Weighted
path = graph.shortest_path("A", "Z", weighted: true)
```

**New signature**:
```rust
pub fn shortest_path(
    &self,
    from: &str,
    to: &str,
    edge_type: Option<&str>,
    weighted: bool
) -> Vec<String>
```

**Implementation**:
```rust
pub fn shortest_path(
    &self,
    from: &str,
    to: &str,
    edge_type: Option<&str>,
    weighted: bool
) -> Vec<String> {
    // If edge type specified, filter graph first
    if let Some(etype) = edge_type {
        return self.shortest_path_with_edge_type(from, to, etype, weighted);
    }

    // Choose algorithm
    if weighted {
        self.shortest_path_weighted(from, to)
    } else if self.has_rule("no_cycles") {
        self.shortest_path_dag(from, to)
    } else {
        self.shortest_path_bfs(from, to)
    }
}

fn shortest_path_with_edge_type(
    &self,
    from: &str,
    to: &str,
    edge_type: &str,
    weighted: bool
) -> Vec<String> {
    // Filter to only edges of specified type
    // Then run appropriate shortest path algorithm
    // ... implementation ...
}
```

**Tests**: 15 tests for weighted pathfinding
- Basic weighted shortest path
- Dijkstra chooses lower-weight path
- Mixed weighted/unweighted edges
- Negative weights (error or warning)
- Zero weights
- No path (disconnected)
- Self-loop with weight
- Weighted vs unweighted comparison
- Edge type filtering
- Combined: weighted + edge_type
- Performance on large graph

---

### Day 4: nodes_within() and Edge Type Filtering

#### 4.1 Implement nodes_within()

**From Spec** (§580): `nearby = graph.nodes_within("user_123", hops: 3)`

```rust
impl Graph {
    /// Get all nodes within N hops of start node
    pub fn nodes_within(&self, start: &str, hops: usize) -> Vec<String> {
        if !self.nodes.contains_key(start) {
            return vec![];
        }

        let mut result = HashSet::new();
        let mut current_level = HashSet::new();
        current_level.insert(start.to_string());
        result.insert(start.to_string());

        for _ in 0..hops {
            let mut next_level = HashSet::new();

            for node_id in &current_level {
                if let Some(node) = self.nodes.get(node_id) {
                    for neighbor_id in node.neighbors.keys() {
                        if result.insert(neighbor_id.clone()) {
                            next_level.insert(neighbor_id.clone());
                        }
                    }
                }
            }

            if next_level.is_empty() {
                break; // No more nodes to explore
            }

            current_level = next_level;
        }

        result.into_iter().collect()
    }

    /// Get nodes within N hops, filtered by edge type
    pub fn nodes_within_by_edge(
        &self,
        start: &str,
        hops: usize,
        edge_type: &str
    ) -> Vec<String> {
        if !self.nodes.contains_key(start) {
            return vec![];
        }

        let mut result = HashSet::new();
        let mut current_level = HashSet::new();
        current_level.insert(start.to_string());
        result.insert(start.to_string());

        for _ in 0..hops {
            let mut next_level = HashSet::new();

            for node_id in &current_level {
                if let Some(node) = self.nodes.get(node_id) {
                    for (neighbor_id, edge_info) in &node.neighbors {
                        // Filter by edge type
                        if edge_info.edge_type == edge_type {
                            if result.insert(neighbor_id.clone()) {
                                next_level.insert(neighbor_id.clone());
                            }
                        }
                    }
                }
            }

            if next_level.is_empty() {
                break;
            }

            current_level = next_level;
        }

        result.into_iter().collect()
    }
}
```

#### 4.2 Comprehensive Edge Type Filtering

Support edge type filtering in all path methods:
- `shortest_path(from, to, edge_type, weighted)`
- `all_paths(from, to, max_length, edge_type)`
- `has_path(from, to, edge_type)`
- `distance(from, to, edge_type)`

**Tests**: 15 tests for nodes_within and edge type filtering
- nodes_within with hops=1
- nodes_within with hops=3
- nodes_within with hops > graph diameter
- nodes_within on disconnected graph
- nodes_within with edge type filter
- Edge type filtering in shortest_path
- Edge type filtering in all_paths
- Edge type filtering in has_path
- Edge type filtering in distance
- Multiple edge types between same nodes
- Non-existent edge type
- Empty edge type (all edges)
- Performance with large hops value

---

## Integration and Testing

### Executor Updates

**File**: `src/execution/executor.rs`

Support these Graphoid operations:
```graphoid
# Add weighted edge
graph.add_edge("A", "B", "road", 5.0)

# Add unweighted edge
graph.add_edge("A", "C", "path")

# Get/set weight
weight = graph.get_edge_weight("A", "B")
graph.set_edge_weight("A", "B", 7.5)

# Weighted shortest path
path = graph.shortest_path("A", "Z", weighted: true)

# With edge type
path = graph.shortest_path("A", "Z", edge_type: "road")

# Combined
path = graph.shortest_path("A", "Z", edge_type: "road", weighted: true)

# Nodes within
nearby = graph.nodes_within("A", hops: 3)

# Rules
graph.add_rule("weighted_edges")
graph.add_rule("unweighted_edges")
```

### Comprehensive Integration Tests

**File**: `tests/phase_6_6_integration_tests.rs`

```rust
#[test]
fn test_weighted_road_network() {
    let code = r#"
        g = graph { type: :directed }
        g.add_rule("weighted_edges")

        # Add cities
        g.add_node("SF", "San Francisco")
        g.add_node("LA", "Los Angeles")
        g.add_node("SD", "San Diego")

        # Add roads with distances
        g.add_edge("SF", "LA", "highway", 383.0)
        g.add_edge("LA", "SD", "highway", 120.0)
        g.add_edge("SF", "SD", "highway", 500.0)

        # Find shortest path (should go SF->LA->SD, not SF->SD)
        path = g.shortest_path("SF", "SD", weighted: true)
        path.size()
    "#;
    assert_eq!(eval(code), Value::Number(3.0)); // SF, LA, SD
}

#[test]
fn test_social_network_within_hops() {
    let code = r#"
        g = graph { type: :undirected }

        # Build network
        g.add_node("Alice", {})
        g.add_node("Bob", {})
        g.add_node("Carol", {})
        g.add_node("Dave", {})

        g.add_edge("Alice", "Bob", "friend")
        g.add_edge("Bob", "Carol", "friend")
        g.add_edge("Carol", "Dave", "friend")

        # Friends within 2 hops of Alice
        nearby = g.nodes_within("Alice", hops: 2)
        nearby.size()
    "#;
    assert_eq!(eval(code), Value::Number(3.0)); // Alice, Bob, Carol
}

#[test]
fn test_mixed_weighted_unweighted_errors() {
    let code = r#"
        g = graph{}
        g.add_rule("weighted_edges")

        g.add_node("A", 1)
        g.add_node("B", 2)

        # This should error: weighted_edges rule requires weights
        try {
            g.add_edge("A", "B", "link")
            "no_error"
        } catch RuleViolation {
            "error_caught"
        }
    "#;
    assert_eq!(eval(code), Value::String("error_caught".to_string()));
}

#[test]
fn test_edge_weight_mutation() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "link", 5.0)

        # Get initial weight
        w1 = g.get_edge_weight("A", "B")

        # Change weight
        g.set_edge_weight("A", "B", 10.0)
        w2 = g.get_edge_weight("A", "B")

        # Remove weight
        g.remove_edge_weight("A", "B")
        w3 = g.get_edge_weight("A", "B")

        [w1, w2, w3]
    "#;
    let expected = Value::List(List::from_vec(vec![
        Value::Number(5.0),
        Value::Number(10.0),
        Value::None,
    ]));
    assert_eq!(eval(code), expected);
}
```

---

## Migration Guide

### For Existing Code

**Before Phase 6.6**:
```rust
graph.add_edge("A", "B", "edge".to_string(), HashMap::new())?;
```

**After Phase 6.6**:
```rust
// Unweighted edge
graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new())?;

// Weighted edge
graph.add_edge("A", "B", "edge".to_string(), Some(5.0), HashMap::new())?;
```

### Automated Migration Script

```bash
#!/bin/bash
# migrate_edge_calls.sh

# Find all add_edge calls and update them
find src tests -name "*.rs" -exec sed -i \
  's/\.add_edge(\([^,]*\), \([^,]*\), \([^,]*\), \([^)]*\))/\.add_edge(\1, \2, \3, None, \4)/g' {} \;

echo "Migration complete. Review changes with: git diff"
```

---

## Success Criteria

### Edge Weight Infrastructure
- ✅ EdgeInfo has weight field
- ✅ add_edge() accepts weight parameter
- ✅ get_edge_weight(), set_edge_weight(), remove_edge_weight() work
- ✅ is_edge_weighted() works
- ✅ weighted_edges rule implemented
- ✅ unweighted_edges rule implemented
- ✅ All existing tests updated and passing
- ✅ 35+ new tests for weights (20 + 15)

### Complete Level 4 Querying
- ✅ Dijkstra's algorithm implemented
- ✅ shortest_path() with weighted parameter
- ✅ shortest_path() with edge_type parameter
- ✅ nodes_within(node, hops) implemented
- ✅ Edge type filtering in all path methods
- ✅ 30+ new tests for Level 4 (15 + 15)

### General
- ✅ Zero compiler warnings
- ✅ All 1397+ tests still passing (after migration)
- ✅ Documentation complete
- ✅ Migration guide provided
- ✅ REPL support for all new features

**Total New Tests**: 65+ (35 weights + 30 Level 4)

---

## Breaking Changes

### API Changes
1. `add_edge()` signature changed (added weight parameter)
2. EdgeInfo structure changed (added weight field)
3. All existing `add_edge()` calls must be updated

### Migration Required
- All code calling `add_edge()` must be updated
- Serialization/deserialization may need updates
- Documentation must reflect new signatures

### Risk Mitigation
- Provide migration script
- Update all internal code first
- Run full test suite after migration
- Document breaking changes clearly

---

## Next Phase Impact

**Phase 7 (Function Pattern Matching)**: Can start immediately after Phase 6.6 - no dependencies

**Phase 9 (Graph Pattern Matching)**: Will benefit from edge weight support:
- Pattern queries can filter by edge weight
- Weighted path queries in patterns
- Better graph querying capabilities

**Phases 11-12 (Stdlib)**: Clean graph API available for stdlib modules to use

---

## References

- **Language Specification**: §250 (add_edge signature), §393-394 (weight rules), §555-587 (Level 4), §570 (weighted paths), §608 (edge weight accessor)
- **Current Implementation**: `src/values/graph.rs`
- **Gap Analysis**: `EDGE_WEIGHT_GAP_ANALYSIS.md`
- **Level 4 Status**: `LEVEL_4_COMPLETENESS_CHECK.md`

---

## Quick Start Commands

```bash
# Check current implementation
grep -n "add_edge" src/values/graph.rs

# Count add_edge calls to migrate
grep -r "add_edge(" src tests | wc -l

# Run graph tests
~/.cargo/bin/cargo test graph --lib

# After implementation
~/.cargo/bin/cargo test phase_6_6 --lib
```
