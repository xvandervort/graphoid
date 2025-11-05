# Phase 9: Graph Pattern Matching - REVISED Implementation Plan

**Date**: November 5, 2025 (Revised)
**Status**: ✅ READY TO START
**Duration**: 5-7 days
**Goal**: Implement Level 3 Pattern-Based Querying with explicit syntax

---

## Revision Summary

### Changes from Original Plan

| Item | Original | Revised | Reason |
|------|----------|---------|--------|
| Duration | 7-10 days | 5-7 days | Removed duplicate work |
| Scope | Level 3 + Level 5 | Level 3 only | Level 5 done in Phase 6.5 |
| Syntax | Compact & Explicit | Explicit only | Compact syntax removed per recent decision |
| Starting status | "Days 1-2 done" | Clear baseline | Accurate assessment |

### What Was Removed

- ❌ **Subgraph operations** (Days 6-8 in original plan)
  - Reason: Already implemented in Phase 6.5
  - `extract_subgraph()` ✅
  - `insert_subgraph()` ✅
  - 16 tests passing ✅

### What Was Reprioritized

- ⭐ **Explicit syntax** → Days 1-5 (PRIMARY - was not in original plan)
- ❌ **Compact syntax** → REMOVED completely per recent decision (was Day 2 in original)

---

## Phase 9 Scope

### IN SCOPE: Level 3 Pattern-Based Querying

**Primary Goal**: Implement `graph.match()` with explicit syntax

```graphoid
# This is what we're building:
results = graph.match(
    node("person", type: "User"),
    edge(type: "FRIEND", direction: :outgoing),
    node("friend", type: "User")
).where(person.age > 18, friend.age > 18)
 .return(person.name, friend.name)
```

**Features**:
- ✅ Pattern objects (`node()`, `edge()`, `path()`) - Already done
- ❌ Graph.match() method - **BUILD THIS**
- ❌ Pattern matching execution - **BUILD THIS**
- ❌ Where clause filtering - **BUILD THIS**
- ❌ Return clause projection - **BUILD THIS**
- ❌ Variable-length paths - **BUILD THIS**

### OUT OF SCOPE

- ❌ **Compact Cypher syntax** - REMOVED per recent decision
- ❌ Subgraph operations (Level 5) - Done in Phase 6.5
- ❌ Graph DSL (Level 5) - Future phase
- ❌ Property graph patterns - Future enhancement

---

## Prerequisites (Already Complete)

From previous phases:

- ✅ **AST types**: GraphPattern, PatternNode, PatternEdge (Phase 7)
- ✅ **Pattern values**: PatternNode, PatternEdge, PatternPath (Phase 7)
- ✅ **Built-in functions**: node(), edge(), path() (Phase 7)
- ✅ **34 pattern object tests** passing
- ✅ **Graph navigation**: neighbors(), predecessors(), has_edge() (Phase 6)
- ✅ **Path algorithms**: shortest_path(), has_path(), distance() (Phase 6)

---

## Day-by-Day Implementation Plan

### Day 1: Graph.match() Method Skeleton (TDD)

**Goal**: Create `graph.match()` method that accepts pattern arguments

**Tasks**:
1. **Write tests first** (TDD - RED phase)
   ```rust
   // tests/unit/graph_pattern_execution_tests.rs (NEW)
   #[test]
   fn test_simple_node_edge_node_pattern() {
       let mut g = Graph::new(GraphType::Directed);
       g.add_node("Alice", Value::number(1));
       g.add_node("Bob", Value::number(2));
       g.add_edge("Alice", "Bob", "FRIEND", None, HashMap::new());

       // Pattern: node() -edge()-> node()
       let results = g.match_pattern(vec![
           node("person"),
           edge(type: "FRIEND"),
           node("friend")
       ]);

       assert_eq!(results.len(), 1);
       // Result should be { "person": "Alice", "friend": "Bob" }
   }
   ```

2. **Implement Graph.match() stub** (GREEN phase)
   ```rust
   // src/values/graph.rs
   pub fn match_pattern(&self, pattern_args: Vec<Value>) -> Result<Vec<HashMap<String, String>>> {
       // Parse pattern_args into nodes and edges
       let (nodes, edges) = self.parse_pattern_arguments(pattern_args)?;

       // Create pattern matching engine
       let engine = GraphPatternEngine::new();

       // Execute pattern matching
       engine.execute(self, nodes, edges)
   }

   fn parse_pattern_arguments(&self, args: Vec<Value>) -> Result<(Vec<PatternNode>, Vec<PatternEdge>)> {
       // Extract PatternNode and PatternEdge values from args
       // Return organized structure
   }
   ```

3. **Refactor** (BLUE phase)
   - Clean up code
   - Add error handling
   - Document methods

**Deliverables**:
- ✅ `Graph::match_pattern()` method
- ✅ `parse_pattern_arguments()` helper
- ✅ 5-8 tests (simple patterns)
- ✅ Tests passing

**Time**: 6-8 hours

---

### Day 2: Pattern Matching Execution Engine

**Goal**: Implement core pattern matching algorithm

**Tasks**:
1. **Create pattern engine** (TDD)
   ```rust
   // src/execution/graph_pattern_engine.rs (NEW FILE)
   pub struct GraphPatternEngine;

   impl GraphPatternEngine {
       pub fn new() -> Self { Self }

       /// Execute pattern matching against a graph
       pub fn execute(
           &self,
           graph: &Graph,
           nodes: Vec<PatternNode>,
           edges: Vec<PatternEdge>
       ) -> Result<Vec<HashMap<String, String>>> {
           let mut results = Vec::new();

           // Start with first node in pattern
           if nodes.is_empty() { return Ok(results); }

           // Find all nodes matching first pattern node
           for start_node in self.find_matching_nodes(graph, &nodes[0])? {
               let mut bindings = HashMap::new();
               bindings.insert(nodes[0].variable.clone(), start_node.clone());

               // Try to extend match following pattern
               if self.extend_match(graph, &nodes, &edges, bindings, 0)? {
                   results.push(bindings);
               }
           }

           Ok(results)
       }

       fn find_matching_nodes(&self, graph: &Graph, pattern: &PatternNode) -> Result<Vec<String>> {
           // Find all graph nodes matching pattern criteria
           // Filter by type if pattern has type constraint
       }

       fn extend_match(
           &self,
           graph: &Graph,
           nodes: &[PatternNode],
           edges: &[PatternEdge],
           bindings: &mut HashMap<String, String>,
           edge_index: usize
       ) -> Result<bool> {
           // Recursively extend partial match
           // Follow edges according to pattern
           // Check type constraints
           // Collect bindings
       }
   }
   ```

2. **Write comprehensive tests** (TDD)
   - Single edge pattern
   - Multiple edges (chain)
   - Type-filtered patterns
   - Multiple matches
   - No matches (empty result)

3. **Implement and test**
   - RED → GREEN → REFACTOR cycle
   - Add tests incrementally
   - Ensure all pass before proceeding

**Deliverables**:
- ✅ `GraphPatternEngine` implementation
- ✅ Node matching algorithm
- ✅ Edge following algorithm
- ✅ Variable binding collection
- ✅ 15-20 tests passing

**Time**: 8-10 hours

---

### Day 3: Complex Patterns and Type Matching

**Goal**: Support complex patterns with multiple edges and type constraints

**Tasks**:
1. **Extend pattern engine** (TDD)
   - Multi-edge patterns: `A -e1-> B -e2-> C`
   - Type-filtered nodes: `node("person", type: "User")`
   - Type-filtered edges: `edge(type: "FRIEND")`
   - Bidirectional edges: `edge(type: "FRIEND", direction: :both)`

2. **Write tests for complex scenarios**
   ```rust
   #[test]
   fn test_three_node_chain_pattern() {
       // A -> B -> C pattern
   }

   #[test]
   fn test_type_filtered_nodes() {
       // Only match nodes with specific type
   }

   #[test]
   fn test_bidirectional_edges() {
       // Match edges in both directions
   }

   #[test]
   fn test_multiple_matches_same_pattern() {
       // Pattern matches multiple times in graph
   }
   ```

3. **Implement type checking**
   ```rust
   fn node_matches_type(graph: &Graph, node_id: &str, pattern_type: &Option<String>) -> bool {
       match pattern_type {
           None => true,  // No type constraint
           Some(typ) => graph.get_node_type(node_id) == Some(typ.clone()),
       }
   }
   ```

**Deliverables**:
- ✅ Complex pattern support
- ✅ Type filtering for nodes
- ✅ Type filtering for edges
- ✅ Bidirectional edge matching
- ✅ 15-20 additional tests

**Time**: 8-10 hours

---

### Day 4: Where Clause Filtering

**Goal**: Implement `.where()` method for filtering match results

**Tasks**:
1. **Define result type**
   ```rust
   // src/values/mod.rs
   pub struct PatternMatchResults {
       bindings: Vec<HashMap<String, String>>,
       graph: Graph,  // Reference to source graph
   }

   impl PatternMatchResults {
       pub fn where_filter(&self, predicates: Vec<Expr>) -> Result<Self> {
           // Filter bindings by evaluating predicates
       }

       pub fn return_projection(&self, fields: Vec<Expr>) -> Result<Value> {
           // Project specific fields from bindings
       }
   }
   ```

2. **Implement where filtering** (TDD)
   ```rust
   impl PatternMatchResults {
       pub fn where_filter(&self, predicates: Vec<Expr>) -> Result<Self> {
           let mut filtered = Vec::new();

           for binding in &self.bindings {
               // Create temporary environment with bindings
               let mut env = Environment::new();
               for (var, node_id) in binding {
                   let node_value = self.graph.get_node(node_id)?;
                   env.define(var.clone(), node_value);
               }

               // Evaluate all predicates
               let all_match = predicates.iter().all(|pred| {
                   // Evaluate predicate in environment
                   // Return true if predicate passes
               });

               if all_match {
                   filtered.push(binding.clone());
               }
           }

           Ok(PatternMatchResults {
               bindings: filtered,
               graph: self.graph.clone(),
           })
       }
   }
   ```

3. **Write comprehensive tests**
   ```rust
   #[test]
   fn test_where_clause_numeric_filter() {
       results.where(person.age > 18)
   }

   #[test]
   fn test_where_clause_multiple_predicates() {
       results.where(person.age > 18, friend.age > 18)
   }

   #[test]
   fn test_where_clause_string_comparison() {
       results.where(person.name == "Alice")
   }
   ```

**Deliverables**:
- ✅ PatternMatchResults type
- ✅ .where() method implementation
- ✅ Predicate evaluation in binding scope
- ✅ 10-12 tests passing

**Time**: 6-8 hours

---

### Day 5: Return Clause and Variable-Length Paths

**Goal**: Complete `.return()` projection and variable-length path matching

**Part A: Return Clause** (3-4 hours)

1. **Implement return projection** (TDD)
   ```rust
   pub fn return_projection(&self, fields: Vec<Expr>) -> Result<Value> {
       let mut results = Vec::new();

       for binding in &self.bindings {
           // Create environment with bindings
           let mut env = Environment::new();
           for (var, node_id) in binding {
               let node_value = self.graph.get_node(node_id)?;
               env.define(var.clone(), node_value);
           }

           // Evaluate each field expression
           let mut row = Vec::new();
           for field_expr in &fields {
               let value = evaluate_in_env(field_expr, &env)?;
               row.push(value);
           }

           results.push(Value::list(row));
       }

       Ok(Value::list(results))
   }
   ```

2. **Write tests**
   ```rust
   #[test]
   fn test_return_single_field() {
       results.return(person.name)
   }

   #[test]
   fn test_return_multiple_fields() {
       results.return(person.name, friend.name)
   }
   ```

**Part B: Variable-Length Paths** (3-4 hours)

1. **Implement path matching** (TDD)
   ```rust
   fn match_variable_length_path(
       graph: &Graph,
       start: &str,
       edge_type: &Option<String>,
       min_length: usize,
       max_length: usize
   ) -> Result<Vec<Vec<String>>> {
       // BFS/DFS to find paths of length min..=max
       // Collect all qualifying paths
       // Return list of node sequences
   }
   ```

2. **Write tests**
   ```rust
   #[test]
   fn test_variable_length_path_min_max() {
       // path(edge_type: "FOLLOWS", min: 1, max: 3)
   }

   #[test]
   fn test_variable_length_path_single_hop() {
       // min: 1, max: 1 should match single edge
   }
   ```

**Deliverables**:
- ✅ .return() method
- ✅ Variable-length path matching
- ✅ 12-15 tests passing

**Time**: 6-8 hours

---

### Day 6-7: Integration, Testing, and Polish

**Goal**: Integration tests, documentation, and performance optimization

**Day 6: Integration Tests** (6-8 hours)

1. **End-to-end integration tests**
   ```rust
   #[test]
   fn test_complete_social_network_query() {
       // Build social graph
       // Query with match().where().return()
       // Verify complete workflow
   }

   #[test]
   fn test_pattern_reuse() {
       // Create pattern objects
       // Reuse in multiple queries
       // Verify consistency
   }

   #[test]
   fn test_chained_filters() {
       // Multiple where clauses
       // Multiple return projections
       // Complex query chains
   }
   ```

2. **Edge case testing**
   - Empty graph queries
   - No matches found
   - Malformed patterns
   - Invalid type constraints
   - Circular path detection

**Day 7: Documentation and Performance** (6-8 hours)

1. **Update documentation**
   - Update LANGUAGE_SPECIFICATION.md examples
   - Create usage guide in dev_docs/
   - Document GraphPatternEngine API
   - Add examples to docstrings

2. **Performance testing**
   - Benchmark pattern matching on large graphs (100+ nodes)
   - Profile hot paths
   - Optimize if needed
   - Add performance regression tests

3. **Final polish**
   - Review error messages for clarity
   - Clean up any warnings
   - Code review and refactoring
   - Ensure 100% test pass rate

**Deliverables**:
- ✅ 15-20 integration tests
- ✅ Edge case coverage
- ✅ Updated documentation
- ✅ Performance benchmarks
- ✅ Zero warnings, all tests passing

---

## Test Plan

### Test Categories

| Category | Target | Priority |
|----------|--------|----------|
| Simple patterns | 8-10 | ⭐ Critical |
| Complex patterns | 15-20 | ⭐ Critical |
| Type filtering | 10-12 | ⭐ Critical |
| Where clauses | 10-12 | ⭐ High |
| Return clauses | 8-10 | ⭐ High |
| Variable-length | 10-12 | ⭐ High |
| Integration | 15-20 | ⭐ Critical |
| Edge cases | 8-10 | ⭐ High |
| **Total** | **84-106** | |

### Test Distribution by Day

- Day 1: 5-8 tests (simple patterns)
- Day 2: 15-20 tests (pattern engine)
- Day 3: 15-20 tests (complex patterns)
- Day 4: 10-12 tests (where clauses)
- Day 5: 12-15 tests (return + paths)
- Day 6: 15-20 tests (integration + edge cases)
- Day 7: 0 tests (documentation + performance)

**Total**: 72-95 new tests

---

## Acceptance Criteria

### Phase 9 Complete When:

#### Must Have (Critical)
- ✅ `graph.match()` method works
- ✅ Pattern matching returns correct bindings
- ✅ Simple patterns work: node() -edge()-> node()
- ✅ Complex multi-edge patterns work
- ✅ Type filtering works for nodes and edges
- ✅ `.where()` clause filters matches
- ✅ `.return()` clause projects fields
- ✅ Variable-length paths work
- ✅ 60+ new tests passing
- ✅ All existing tests still pass
- ✅ Zero compiler warnings
- ✅ Documentation updated

#### Should Have (High Priority)
- ✅ Integration tests demonstrate end-to-end workflows
- ✅ Error messages are clear and helpful
- ✅ Performance is acceptable (< 100ms for typical queries)
- ✅ Edge cases handled gracefully
- ✅ Performance benchmarks established

#### Future Enhancements (Not Phase 9)
- ⭕ Pattern .bind() method
- ⭕ Advanced pattern composition
- ⭕ Query optimization hints

---

## Risk Management

### Risk 1: Performance on Large Graphs
**Mitigation**: Profile early, optimize hot paths, add indexing if needed, benchmark on Day 7

### Risk 2: Scope Creep
**Mitigation**: Strict scope (Level 3 explicit syntax only), NO compact syntax, defer all other features

### Risk 3: Test Debt
**Mitigation**: TDD throughout, write tests before code, maintain 100% pass rate

### Risk 4: Complex Pattern Edge Cases
**Mitigation**: Comprehensive edge case testing on Day 6, validate all boundary conditions

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| New tests | 70+ | `cargo test pattern` |
| Pass rate | 100% | All tests green |
| Coverage | 80%+ | Core engine covered |
| Performance | < 100ms | Simple query benchmark |
| Edge cases | 10+ | Boundary conditions tested |
| Documentation | Complete | All APIs documented |
| Warnings | 0 | Clean build |

---

## Architecture Decisions

### Decision 1: Explicit Syntax ONLY ✅
**Rationale**: Compact Cypher syntax removed per recent decision, only explicit syntax will be implemented

### Decision 2: Separate Pattern Engine ✅
**Rationale**: Separation of concerns, easier to test, cleaner architecture

### Decision 3: TDD Throughout ✅
**Rationale**: Complex feature, tests prevent regressions, validates correctness

### Decision 4: Incremental Implementation ✅
**Rationale**: Simple → complex, validate each step, build confidence progressively

### Decision 5: NO Compact Syntax ❌
**Rationale**: Recent decision to remove Cypher-style syntax completely from language, focus on explicit pattern objects only

---

## Comparison with Original Plan

### Original Plan Issues
- ❌ Started with compact syntax (now removed completely)
- ❌ Included subgraph operations (duplicate - done in Phase 6.5)
- ❌ Unclear implementation status (claimed "Days 1-2 complete")
- ❌ 7-10 days for both Level 3 & 5

### Revised Plan Fixes
- ✅ **Explicit syntax ONLY** - No compact syntax at all
- ✅ Remove subgraph operations (already complete)
- ✅ Clear starting point documented (40% infrastructure done)
- ✅ 5-7 days for Level 3 explicit syntax only
- ✅ Focused scope with no feature creep

---

## Next Steps

### Immediate (Day 1 Start)
1. Create `tests/unit/graph_pattern_execution_tests.rs`
2. Write first test for simple pattern
3. Implement `Graph::match_pattern()` stub
4. Make test pass (GREEN)
5. Refactor (BLUE)

### Command to Start
```bash
cd /home/irv/work/grang/rust
mkdir -p tests/unit
touch tests/unit/graph_pattern_execution_tests.rs
# Begin Day 1 implementation
```

---

## Appendix: Language Spec References

- **Level 3 Spec**: LANGUAGE_SPECIFICATION.md §509-658
- **Pattern Objects**: §595-658
- **Explicit Syntax**: §577-588 (PRIMARY - ONLY syntax to implement)
- **Compact Syntax**: §589-594 (REMOVED - not implementing)
- **Subgraph Operations**: §768-818 (Phase 6.5 - already complete)

---

**Plan Status**: ✅ READY TO IMPLEMENT
**Created**: November 5, 2025
**Approved**: Pending user review
