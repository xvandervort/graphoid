# Phase 9: Graph Pattern Matching & Advanced Querying - Detailed Implementation Plan

**Duration**: 7-10 days
**Status**: 🔄 IN PROGRESS - Days 1-2 complete
**Goal**: Implement declarative graph pattern matching with explicit, readable syntax

---

## Overview

This phase implements **Level 3 (Pattern-Based Querying)** and **Level 5 (Subgraph Operations)** from the language specification, filling the critical capability gap in graph querying.

**Why This is Critical**:
- Makes Graphoid credible as a graph-theoretic language
- Enables declarative graph querying with readable syntax
- Provides powerful subgraph manipulation
- "Make or break" feature per language specification §452

**Current Status**:
- ✅ Level 1: Basic navigation (neighbors, has_node) - COMPLETE
- ✅ Level 2: Filtering/traversal (bfs, dfs, filter) - COMPLETE
- 🔄 Level 3: Pattern-based querying - **IN PROGRESS** (this phase)
- ✅ Level 4: Path algorithms (shortest_path, distance) - COMPLETE
- ❌ Level 5: Subgraph operations - **PENDING** (this phase)

**Implementation Progress**:
- ✅ AST types (GraphPattern, PatternNode, PatternEdge, EdgeDirection, EdgeLength)
- ✅ Parser for compact syntax `(node:Type) -[:EDGE]-> (other:Type)`
- ✅ Pattern value types (PatternNode, PatternEdge, PatternPath)
- ✅ Built-in functions (node(), edge(), path())
- ✅ Documentation in LANGUAGE_SPECIFICATION.md
- 🔄 Pattern object methods (.bind(), property access)
- ⏳ Parser for explicit syntax
- ⏳ Pattern matching execution engine
- ⏳ Subgraph operations

**Major Design Decisions**:
1. **Explicit Syntax PRIMARY**: Readable `node("var", type: "Type")` syntax is recommended
2. **Compact Syntax OPTIONAL**: Cypher-style `(var:Type)` syntax supported for power users
3. **Pattern Objects as First-Class Values**: Can be stored, passed, inspected, and composed
4. **No Range Literals**: Variable-length paths use `*{min: N, max: M}` explicit syntax

**Dependencies**:
- Phase 7 (Function Pattern Matching) - shares parser/engine
- Phase 8 (Behavior System) - for result transformation

---

## Part A: Level 3 Pattern-Based Querying (Days 1-5)

### Architecture Overview

**From Language Specification §509-553**:

Pattern syntax inspired by Cypher graph query language:
- `(node:Type)` - Node with type
- `-[:EDGE_TYPE]->` - Directed edge
- `-[:EDGE_TYPE]-` - Bidirectional edge
- `-[:EDGE*{min: N, max: M}]->` - Variable-length path
- `.where()` - Filter predicates
- `.return()` - Select specific fields

### Day 1: AST Nodes for Graph Patterns

**Goal**: Define AST for graph pattern syntax

**File**: `src/ast/mod.rs`

```rust
/// Graph pattern match expression
#[derive(Debug, Clone, PartialEq)]
pub struct GraphPattern {
    pub nodes: Vec<PatternNode>,
    pub edges: Vec<PatternEdge>,
    pub where_clause: Option<Vec<Expr>>,
    pub return_clause: Option<Vec<Expr>>,
    pub position: SourcePosition,
}

/// Node in a pattern: (node:Type)
#[derive(Debug, Clone, PartialEq)]
pub struct PatternNode {
    pub variable: String,           // Variable name (e.g., "person")
    pub node_type: Option<String>,  // Optional type (e.g., "User")
    pub position: SourcePosition,
}

/// Edge in a pattern: -[:TYPE]->
#[derive(Debug, Clone, PartialEq)]
pub struct PatternEdge {
    pub from: String,                      // Source node variable
    pub to: String,                        // Target node variable
    pub edge_type: Option<String>,         // Optional edge type
    pub direction: EdgeDirection,          // Directed or bidirectional
    pub length: EdgeLength,                // Fixed or variable-length
    pub position: SourcePosition,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeDirection {
    Directed,       // ->
    Bidirectional,  // -
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeLength {
    Fixed,                      // Single edge
    Variable { min: usize, max: usize },  // Variable-length path
}

/// Update Expr enum
pub enum Expr {
    // ... existing variants ...
    GraphMatch {
        pattern: GraphPattern,
        position: SourcePosition,
    },
}
```

### Day 2: Parser for Graph Patterns

**Goal**: Parse Cypher-style pattern syntax

**File**: `src/parser/mod.rs`

```rust
impl Parser {
    /// Parse graph.match(...) expression
    fn parse_graph_match(&mut self) -> Result<Expr, GraphoidError> {
        // Expecting: graph.match((node:Type) -[:EDGE]-> (other:Type))

        self.expect(Token::Symbol("match".to_string()))?;
        self.expect(Token::LeftParen)?;

        let pattern = self.parse_graph_pattern()?;

        self.expect(Token::RightParen)?;

        Ok(Expr::GraphMatch {
            pattern,
            position: self.position(),
        })
    }

    /// Parse graph pattern: (node:Type) -[:EDGE]-> (other:Type)
    fn parse_graph_pattern(&mut self) -> Result<GraphPattern, GraphoidError> {
        let mut nodes = vec![];
        let mut edges = vec![];

        // Parse first node
        nodes.push(self.parse_pattern_node()?);

        // Parse edges and subsequent nodes
        while self.check(Token::Minus) {
            let edge = self.parse_pattern_edge()?;
            let node = self.parse_pattern_node()?;
            edges.push(edge);
            nodes.push(node);
        }

        // Optional where clause
        let where_clause = if self.check_symbol("where") {
            self.advance();
            self.expect(Token::LeftParen)?;
            let mut conditions = vec![];
            loop {
                conditions.push(self.parse_expression()?);
                if !self.check(Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect(Token::RightParen)?;
            Some(conditions)
        } else {
            None
        };

        // Optional return clause
        let return_clause = if self.check_symbol("return") {
            self.advance();
            self.expect(Token::LeftParen)?;
            let mut fields = vec![];
            loop {
                fields.push(self.parse_expression()?);
                if !self.check(Token::Comma) {
                    break;
                }
                self.advance();
            }
            self.expect(Token::RightParen)?;
            Some(fields)
        } else {
            None
        };

        Ok(GraphPattern {
            nodes,
            edges,
            where_clause,
            return_clause,
            position: self.position(),
        })
    }

    /// Parse pattern node: (variable:Type)
    fn parse_pattern_node(&mut self) -> Result<PatternNode, GraphoidError> {
        self.expect(Token::LeftParen)?;

        let variable = self.expect_symbol()?;

        let node_type = if self.check(Token::Colon) {
            self.advance();
            Some(self.expect_symbol()?)
        } else {
            None
        };

        self.expect(Token::RightParen)?;

        Ok(PatternNode {
            variable,
            node_type,
            position: self.position(),
        })
    }

    /// Parse pattern edge: -[:TYPE]-> or -[:TYPE*{min: N, max: M}]->
    fn parse_pattern_edge(&mut self) -> Result<PatternEdge, GraphoidError> {
        self.expect(Token::Minus)?;

        let edge_type = if self.check(Token::LeftBracket) {
            self.advance();
            self.expect(Token::Colon)?;
            let edge_type = self.expect_symbol()?;

            // Check for variable length: *{min: N, max: M}
            let length = if self.check(Token::Star) {
                self.advance();
                self.expect(Token::LeftBrace)?;
                // Parse min: N
                self.expect_identifier("min")?;
                self.expect(Token::Colon)?;
                let min = self.expect_number()? as usize;
                self.expect(Token::Comma)?;
                // Parse max: M
                self.expect_identifier("max")?;
                self.expect(Token::Colon)?;
                let max = self.expect_number()? as usize;
                self.expect(Token::RightBrace)?;
                EdgeLength::Variable { min, max }
            } else {
                EdgeLength::Fixed
            };

            self.expect(Token::RightBracket)?;
            Some(edge_type)
        } else {
            None
        };

        // Direction: -> or just -
        let direction = if self.check(Token::GreaterThan) {
            self.advance();
            EdgeDirection::Directed
        } else {
            EdgeDirection::Bidirectional
        };

        Ok(PatternEdge {
            from: "".to_string(),  // Will be filled in by pattern builder
            to: "".to_string(),
            edge_type,
            direction,
            length,
            position: self.position(),
        })
    }
}
```

**Tests**: `tests/unit/parser_tests.rs`

```rust
#[test]
fn test_parse_simple_graph_pattern() {
    let code = "graph.match((person:User) -[:FRIEND]-> (friend:User))";
    let ast = parse(code).unwrap();
    // Assert pattern structure
}

#[test]
fn test_parse_pattern_with_where() {
    let code = r#"
        graph.match((person:User) -[:FRIEND]-> (friend:User))
            .where(person.age > 18, friend.age > 18)
    "#;
    let ast = parse(code).unwrap();
    // Assert where clause
}

#[test]
fn test_parse_variable_length_path() {
    let code = "graph.match((user:User) -[:FOLLOWS*{min: 1, max: 3}]-> (influencer:User))";
    let ast = parse(code).unwrap();
    // Assert variable-length edge
}

#[test]
fn test_parse_bidirectional_edge() {
    let code = "graph.match((a:User) -[:FRIEND]- (b:User))";
    let ast = parse(code).unwrap();
    // Assert bidirectional
}
```

### Day 3-4: Pattern Matching Engine for Graphs

**Goal**: Execute graph pattern matching

**File**: `src/execution/graph_pattern_matcher.rs` (NEW)

```rust
use crate::ast::{GraphPattern, PatternNode, PatternEdge, EdgeDirection, EdgeLength};
use crate::values::{Value, Graph};
use crate::error::{GraphoidError, Result};
use std::collections::HashMap;

pub struct GraphPatternMatcher;

impl GraphPatternMatcher {
    pub fn new() -> Self {
        GraphPatternMatcher
    }

    /// Match a pattern against a graph, return all matches
    pub fn match_pattern(
        &self,
        graph: &Graph,
        pattern: &GraphPattern
    ) -> Result<Vec<HashMap<String, String>>> {
        // Returns list of variable bindings for each match
        // e.g., [{"person": "node_1", "friend": "node_2"}, ...]

        let mut results = vec![];

        // Start with first node in pattern
        if pattern.nodes.is_empty() {
            return Ok(results);
        }

        let start_node = &pattern.nodes[0];

        // Find all nodes matching the first pattern node
        for node_id in self.find_matching_nodes(graph, start_node)? {
            let mut bindings = HashMap::new();
            bindings.insert(start_node.variable.clone(), node_id.clone());

            // Try to extend the match following the pattern
            if self.extend_match(graph, pattern, &mut bindings, 0)? {
                results.push(bindings);
            }
        }

        Ok(results)
    }

    /// Find all nodes matching a pattern node
    fn find_matching_nodes(
        &self,
        graph: &Graph,
        pattern_node: &PatternNode
    ) -> Result<Vec<String>> {
        let mut matching_nodes = vec![];

        for node_id in graph.keys() {
            if let Some(node_type) = &pattern_node.node_type {
                // Check if node has the required type
                if graph.get_node_type(&node_id) == Some(node_type.clone()) {
                    matching_nodes.push(node_id);
                }
            } else {
                // No type constraint, all nodes match
                matching_nodes.push(node_id);
            }
        }

        Ok(matching_nodes)
    }

    /// Recursively extend a partial match
    fn extend_match(
        &self,
        graph: &Graph,
        pattern: &GraphPattern,
        bindings: &mut HashMap<String, String>,
        edge_index: usize
    ) -> Result<bool> {
        if edge_index >= pattern.edges.len() {
            // All edges matched, check where clause
            return self.check_where_clause(graph, pattern, bindings);
        }

        let edge_pattern = &pattern.edges[edge_index];
        let next_node_pattern = &pattern.nodes[edge_index + 1];

        let from_id = bindings.get(&pattern.nodes[edge_index].variable)
            .ok_or_else(|| GraphoidError::runtime_error(
                "Pattern matching error: missing binding".to_string(),
                None
            ))?;

        // Find edges matching the pattern
        for (to_id, edge_type) in self.find_matching_edges(
            graph,
            from_id,
            edge_pattern
        )? {
            // Check if target node matches pattern
            if self.node_matches(graph, &to_id, next_node_pattern)? {
                bindings.insert(next_node_pattern.variable.clone(), to_id.clone());

                // Recursively try to match remaining edges
                if self.extend_match(graph, pattern, bindings, edge_index + 1)? {
                    return Ok(true);
                }

                // Backtrack
                bindings.remove(&next_node_pattern.variable);
            }
        }

        Ok(false)
    }

    /// Find edges matching a pattern edge
    fn find_matching_edges(
        &self,
        graph: &Graph,
        from_id: &str,
        edge_pattern: &PatternEdge
    ) -> Result<Vec<(String, String)>> {
        let mut matching_edges = vec![];

        match &edge_pattern.length {
            EdgeLength::Fixed => {
                // Single-hop edge
                for to_id in graph.neighbors(from_id) {
                    if let Some(edge_type) = &edge_pattern.edge_type {
                        if graph.get_edge_type(from_id, &to_id) == Some(edge_type.clone()) {
                            matching_edges.push((to_id, edge_type.clone()));
                        }
                    } else {
                        // Any edge type
                        matching_edges.push((to_id, "".to_string()));
                    }
                }
            }
            EdgeLength::Variable { min, max } => {
                // Variable-length path
                matching_edges = self.find_variable_length_paths(
                    graph,
                    from_id,
                    edge_pattern,
                    *min,
                    *max
                )?;
            }
        }

        Ok(matching_edges)
    }

    /// Find variable-length paths
    fn find_variable_length_paths(
        &self,
        graph: &Graph,
        start: &str,
        edge_pattern: &PatternEdge,
        min: usize,
        max: usize
    ) -> Result<Vec<(String, String)>> {
        // BFS to find all paths within min..max hops
        let mut results = vec![];
        let mut visited = HashMap::new();
        let mut queue = vec![(start.to_string(), 0)];

        while let Some((node, depth)) = queue.pop() {
            if depth >= min && depth <= max {
                results.push((node.clone(), "".to_string()));
            }

            if depth < max {
                for neighbor in graph.neighbors(&node) {
                    if !visited.contains_key(&neighbor) {
                        visited.insert(neighbor.clone(), true);
                        queue.push((neighbor, depth + 1));
                    }
                }
            }
        }

        Ok(results)
    }

    /// Check if where clause is satisfied
    fn check_where_clause(
        &self,
        graph: &Graph,
        pattern: &GraphPattern,
        bindings: &HashMap<String, String>
    ) -> Result<bool> {
        if let Some(conditions) = &pattern.where_clause {
            // Evaluate each condition with bindings
            for condition in conditions {
                // TODO: Evaluate condition with variable bindings
                // This requires access to the executor
            }
        }
        Ok(true)
    }

    /// Check if a node matches a pattern node
    fn node_matches(
        &self,
        graph: &Graph,
        node_id: &str,
        pattern: &PatternNode
    ) -> Result<bool> {
        if let Some(node_type) = &pattern.node_type {
            Ok(graph.get_node_type(node_id) == Some(node_type.clone()))
        } else {
            Ok(true)
        }
    }
}
```

### Day 5: Integration with Executor

**Goal**: Wire pattern matching into execution engine

**File**: `src/execution/executor.rs`

```rust
use crate::execution::graph_pattern_matcher::GraphPatternMatcher;

impl Executor {
    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::GraphMatch { pattern, .. } => {
                // Evaluate the graph expression to get the graph value
                // (assuming graph is in scope as a variable)

                let matcher = GraphPatternMatcher::new();

                // Get graph from context
                let graph = self.env.get("graph")
                    .ok_or_else(|| GraphoidError::runtime_error(
                        "Graph pattern matching requires 'graph' variable".to_string(),
                        None
                    ))?;

                if let Value::Graph(g) = graph {
                    let matches = matcher.match_pattern(&g, pattern)?;

                    // Convert matches to list of hashes
                    let results: Vec<Value> = matches.iter().map(|bindings| {
                        let mut hash = HashMap::new();
                        for (var, node_id) in bindings {
                            hash.insert(var.clone(), Value::String(node_id.clone()));
                        }
                        Value::Hash(hash)
                    }).collect();

                    Ok(Value::List(List::from_vec(results)))
                } else {
                    Err(GraphoidError::type_error(
                        "Expected graph value".to_string(),
                        None
                    ))
                }
            }
            // ... other expressions ...
        }
    }
}
```

**Tests**: `tests/graph_pattern_matching_tests.rs` (NEW)

```rust
#[test]
fn test_simple_pattern_match() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 1)
        g.add_node("Bob", 2)
        g.add_edge("Alice", "Bob", "FRIEND", {})

        results = g.match((person) -[:FRIEND]-> (friend))
        results.size()
    "#;
    assert_eq!(eval(code), Value::Number(1.0));
}

#[test]
fn test_pattern_with_where_clause() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", {age: 25})
        g.add_node("Bob", {age: 17})
        g.add_node("Carol", {age: 30})
        g.add_edge("Alice", "Bob", "FRIEND", {})
        g.add_edge("Alice", "Carol", "FRIEND", {})

        results = g.match((person) -[:FRIEND]-> (friend))
                   .where(friend.age >= 18)
        results.size()
    "#;
    assert_eq!(eval(code), Value::Number(1.0));  // Only Carol
}

#[test]
fn test_variable_length_path() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B", "FOLLOWS", {})
        g.add_edge("B", "C", "FOLLOWS", {})

        results = g.match((user) -[:FOLLOWS*{min: 1, max: 2}]-> (other))
        results.size()
    "#;
    assert_eq!(eval(code), Value::Number(3.0));  // A->B, B->C, A->B->C
}
```

---

## Part B: Level 5 Subgraph Operations (Days 6-8)

### Day 6: Subgraph Extract Operations

**Goal**: Implement `graph.extract { nodes: ..., edges: ... }`

**From Language Specification §589-638**

**File**: `src/values/graph.rs`

```rust
impl Graph {
    /// Extract subgraph based on filters
    pub fn extract(
        &self,
        node_filter: Option<Box<dyn Fn(&GraphNode) -> bool>>,
        edge_filter: Option<Box<dyn Fn(&GraphEdge) -> bool>>,
        include_orphans: bool
    ) -> Result<Graph, GraphoidError> {
        let mut new_graph = Graph::new(self.graph_type.clone());

        // Filter nodes
        let selected_nodes: HashSet<String> = self.nodes.iter()
            .filter(|(id, node)| {
                if let Some(filter) = &node_filter {
                    filter(node)
                } else {
                    true
                }
            })
            .map(|(id, _)| id.clone())
            .collect();

        // Add filtered nodes
        for node_id in &selected_nodes {
            if let Some(node) = self.nodes.get(node_id) {
                new_graph.add_node(node_id.clone(), node.value.clone())?;
            }
        }

        // Filter edges
        for edge in &self.edges {
            let include_edge = if let Some(filter) = &edge_filter {
                filter(edge)
            } else {
                true
            };

            if include_edge &&
               selected_nodes.contains(&edge.from) &&
               selected_nodes.contains(&edge.to) {
                new_graph.add_edge(
                    &edge.from,
                    &edge.to,
                    edge.edge_type.clone(),
                    edge.properties.clone()
                )?;
            }
        }

        // Handle orphan nodes
        if !include_orphans {
            let nodes_with_edges: HashSet<String> = new_graph.edges.iter()
                .flat_map(|e| vec![e.from.clone(), e.to.clone()])
                .collect();

            new_graph.nodes.retain(|id, _| nodes_with_edges.contains(id));
        }

        Ok(new_graph)
    }

    /// Delete subgraph (opposite of extract)
    pub fn delete(
        &self,
        node_filter: Option<Box<dyn Fn(&GraphNode) -> bool>>,
        edge_filter: Option<Box<dyn Fn(&GraphEdge) -> bool>>
    ) -> Result<Graph, GraphoidError> {
        // Invert filters and call extract
        let inverted_node_filter = node_filter.map(|f| {
            Box::new(move |node: &GraphNode| !f(node)) as Box<dyn Fn(&GraphNode) -> bool>
        });

        let inverted_edge_filter = edge_filter.map(|f| {
            Box::new(move |edge: &GraphEdge| !f(edge)) as Box<dyn Fn(&GraphEdge) -> bool>
        });

        self.extract(inverted_node_filter, inverted_edge_filter, true)
    }

    /// Add subgraph (merge)
    pub fn add_subgraph(
        &mut self,
        other: &Graph,
        on_conflict: ConflictResolution
    ) -> Result<(), GraphoidError> {
        for (node_id, node) in &other.nodes {
            if self.nodes.contains_key(node_id) {
                match on_conflict {
                    ConflictResolution::KeepOriginal => {
                        // Skip
                    }
                    ConflictResolution::Overwrite => {
                        self.nodes.insert(node_id.clone(), node.clone());
                    }
                    ConflictResolution::Merge => {
                        // Merge attributes
                        if let Some(existing) = self.nodes.get_mut(node_id) {
                            // TODO: Implement attribute merging
                        }
                    }
                }
            } else {
                self.add_node(node_id.clone(), node.value.clone())?;
            }
        }

        for edge in &other.edges {
            if !self.has_edge(&edge.from, &edge.to) {
                self.add_edge(
                    &edge.from,
                    &edge.to,
                    edge.edge_type.clone(),
                    edge.properties.clone()
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ConflictResolution {
    KeepOriginal,
    Overwrite,
    Merge,
}
```

### Day 7-8: Testing & Integration

**Tests**: `tests/subgraph_operations_tests.rs` (NEW)

```rust
#[test]
fn test_extract_by_node_filter() {
    let code = r#"
        g = graph{}
        g.add_node("A", {active: true})
        g.add_node("B", {active: false})
        g.add_node("C", {active: true})
        g.add_edge("A", "B")
        g.add_edge("B", "C")

        active = g.extract {
            nodes: n => n.get_attribute("active") == true
        }
        active.node_count()
    "#;
    assert_eq!(eval(code), Value::Number(2.0));
}

#[test]
fn test_delete_nodes() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)

        cleaned = g.delete {
            nodes: n => n.value() == 2
        }
        cleaned.node_count()
    "#;
    assert_eq!(eval(code), Value::Number(2.0));
}

#[test]
fn test_add_subgraph() {
    let code = r#"
        g1 = graph{}
        g1.add_node("A", 1)

        g2 = graph{}
        g2.add_node("B", 2)

        combined = g1.add_subgraph(g2)
        combined.node_count()
    "#;
    assert_eq!(eval(code), Value::Number(2.0));
}
```

---

## Success Criteria

### Level 3: Pattern-Based Querying
- [ ] ✅ Parser handles Cypher-style pattern syntax
- [ ] ✅ Simple patterns work: `(a) -[:TYPE]-> (b)`
- [ ] ✅ Node type constraints work: `(person:User)`
- [ ] ✅ Edge type constraints work: `-[:FRIEND]->`
- [ ] ✅ Bidirectional patterns work: `-[:TYPE]-`
- [ ] ✅ Variable-length paths work: `-[:TYPE*{min: 1, max: 3}]->`
- [ ] ✅ Where clauses filter correctly
- [ ] ✅ Return clauses project fields
- [ ] ✅ Multiple patterns in single query work
- [ ] ✅ 30+ pattern matching tests passing

### Level 5: Subgraph Operations
- [ ] ✅ `graph.extract` with node filters works
- [ ] ✅ `graph.extract` with edge filters works
- [ ] ✅ `include_orphans` option works
- [ ] ✅ `graph.delete` removes matching subgraphs
- [ ] ✅ `graph.add_subgraph` merges graphs
- [ ] ✅ Conflict resolution strategies work
- [ ] ✅ 20+ subgraph operation tests passing

### General
- [ ] ✅ Zero compiler warnings
- [ ] ✅ REPL support working
- [ ] ✅ Documentation complete
- [ ] ✅ 50+ total tests passing for Phase 9

---

## Dependencies

- **Phase 7 (Function Pattern Matching)**: Shares parser concepts and pattern matching engine
- **Phase 8 (Behavior System)**: Behaviors can be applied to query results

---

## Notes for Implementation

1. **Performance**: Pattern matching uses backtracking. For large graphs, consider indexing by node/edge types.

2. **Query Optimization**: Future work could compile patterns to optimized query plans.

3. **Cypher Compatibility**: Our syntax is inspired by Cypher but not identical. Document differences.

4. **Return Clause**: Returns list of hashes with variable bindings.

---

## References

- **Language Specification**: §452 "Graph Querying", §509-553 "Level 3", §589-638 "Level 5"
- **Cypher Query Language**: Neo4j's pattern matching syntax (inspiration)
- **Phase 7**: Function pattern matching (foundational work)
