//! Tests for graph algorithms and rule-aware optimizations
//!
//! Following TDD: These tests are written BEFORE implementation.
//! They should fail initially, then pass once we implement the algorithms.

use graphoid::values::{Value, Graph};
use graphoid::values::graph::GraphType;
use std::collections::HashMap;

// ============================================================================
// Basic shortest_path() Tests - Using BFS
// ============================================================================

#[test]
fn test_shortest_path_simple_linear() {
    let mut g = Graph::new(GraphType::Directed);

    // Create linear path: A -> B -> C
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();

    // Test shortest path
    let path = g.shortest_path("A", "C", None, false).unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);
}

#[test]
fn test_shortest_path_with_multiple_routes() {
    let mut g = Graph::new(GraphType::Directed);

    // Create diamond:
    //     A
    //    / \
    //   B   C
    //    \ /
    //     D

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();
    g.add_node("D".to_string(), Value::Number(4.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("A", "C", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "D", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("C", "D", "edge".to_string(), None, HashMap::new()).unwrap();

    // Should find A -> B -> D or A -> C -> D (both are length 3)
    let path = g.shortest_path("A", "D", None, false).unwrap();
    assert_eq!(path.len(), 3);
    assert_eq!(path[0], "A");
    assert_eq!(path[2], "D");
    // Middle node should be either B or C
    assert!(path[1] == "B" || path[1] == "C");
}

#[test]
fn test_shortest_path_direct_edge() {
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

    let path = g.shortest_path("A", "B", None, false).unwrap();
    assert_eq!(path, vec!["A", "B"]);
}

#[test]
fn test_shortest_path_same_node() {
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();

    // Path from A to A should be just [A]
    let path = g.shortest_path("A", "A", None, false).unwrap();
    assert_eq!(path, vec!["A"]);
}

#[test]
fn test_shortest_path_no_path_exists() {
    let mut g = Graph::new(GraphType::Directed);

    // Disconnected nodes
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    // No edge between them
    let path = g.shortest_path("A", "B", None, false).unwrap_or(vec![]);
    assert_eq!(path.len(), 0); // Empty path = no path exists
}

#[test]
fn test_shortest_path_nonexistent_start() {
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();

    let path = g.shortest_path("Z", "A", None, false).unwrap_or(vec![]);
    assert_eq!(path.len(), 0);
}

#[test]
fn test_shortest_path_nonexistent_end() {
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();

    let path = g.shortest_path("A", "Z", None, false).unwrap_or(vec![]);
    assert_eq!(path.len(), 0);
}

// ============================================================================
// Topological Sort Tests
// ============================================================================

#[test]
fn test_topological_sort_simple_dag() {
    let mut g = Graph::new(GraphType::Directed);

    // Create simple DAG: A -> B -> C
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();

    let sorted = g.topological_sort();

    // A must come before B, B must come before C
    let pos_a = sorted.iter().position(|x| x == "A").unwrap();
    let pos_b = sorted.iter().position(|x| x == "B").unwrap();
    let pos_c = sorted.iter().position(|x| x == "C").unwrap();

    assert!(pos_a < pos_b);
    assert!(pos_b < pos_c);
}

#[test]
fn test_topological_sort_diamond_dag() {
    let mut g = Graph::new(GraphType::Directed);

    // Diamond DAG:
    //     A
    //    / \
    //   B   C
    //    \ /
    //     D

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();
    g.add_node("D".to_string(), Value::Number(4.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("A", "C", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "D", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("C", "D", "edge".to_string(), None, HashMap::new()).unwrap();

    let sorted = g.topological_sort();

    // Verify ordering constraints
    let pos_a = sorted.iter().position(|x| x == "A").unwrap();
    let pos_b = sorted.iter().position(|x| x == "B").unwrap();
    let pos_c = sorted.iter().position(|x| x == "C").unwrap();
    let pos_d = sorted.iter().position(|x| x == "D").unwrap();

    assert!(pos_a < pos_b);
    assert!(pos_a < pos_c);
    assert!(pos_b < pos_d);
    assert!(pos_c < pos_d);
}

#[test]
fn test_topological_sort_empty_graph() {
    let g = Graph::new(GraphType::Directed);

    let sorted = g.topological_sort();
    assert_eq!(sorted.len(), 0);
}

#[test]
fn test_topological_sort_single_node() {
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();

    let sorted = g.topological_sort();
    assert_eq!(sorted, vec!["A"]);
}

#[test]
fn test_topological_sort_with_cycle_returns_empty() {
    let mut g = Graph::new(GraphType::Directed);

    // Create cycle: A -> B -> C -> A
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    // Add edges that form a cycle
    // Note: This will only work if no_cycles rule is NOT active
    let _ = g.add_edge("A", "B", "edge".to_string(), None, HashMap::new());
    let _ = g.add_edge("B", "C", "edge".to_string(), None, HashMap::new());
    let _ = g.add_edge("C", "A", "edge".to_string(), None, HashMap::new());

    // Topological sort should detect cycle and return empty
    let sorted = g.topological_sort();
    assert_eq!(sorted.len(), 0);
}

// ============================================================================
// Rule-Aware Algorithm Selection Tests
// ============================================================================

#[test]
fn test_shortest_path_uses_topological_sort_with_no_cycles_rule() {
    let mut g = Graph::new(GraphType::Directed)
        .with_ruleset("dag".to_string());

    // Create DAG
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();
    g.add_node("D".to_string(), Value::Number(4.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("A", "C", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "D", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("C", "D", "edge".to_string(), None, HashMap::new()).unwrap();

    // Should use optimized topological-based algorithm
    let path = g.shortest_path("A", "D", None, false).unwrap();

    // Verify we get a valid shortest path
    assert_eq!(path.len(), 3);
    assert_eq!(path[0], "A");
    assert_eq!(path[2], "D");
}

#[test]
fn test_shortest_path_without_rules_uses_bfs() {
    let mut g = Graph::new(GraphType::Directed);

    // No rules applied - should use standard BFS
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();

    let path = g.shortest_path("A", "C", None, false).unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);
}

#[test]
fn test_explain_reflects_actual_algorithm_used() {
    // Test that explain() output matches what shortest_path() actually does

    // With no_cycles rule
    let g_dag = Graph::new(GraphType::Directed)
        .with_ruleset("dag".to_string());

    let plan = g_dag.explain_shortest_path("A", "B");
    assert!(plan.to_string().contains("topological") ||
            plan.to_string().contains("Topological"));

    // Without rules
    let g_normal = Graph::new(GraphType::Directed);
    let plan_normal = g_normal.explain_shortest_path("A", "B");
    assert!(plan_normal.to_string().contains("BFS"));
}

// ============================================================================
// Undirected Graph Tests
// ============================================================================

#[test]
fn test_shortest_path_undirected() {
    let mut g = Graph::new(GraphType::Undirected);

    // Create undirected graph
    g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    // In undirected graph, edge goes both ways
    g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();

    // Should be able to go A -> B -> C
    let path = g.shortest_path("A", "C", None, false).unwrap();
    assert_eq!(path, vec!["A", "B", "C"]);

    // Should also work in reverse (since undirected)
    let path_reverse = g.shortest_path("C", "A", None, false).unwrap();
    assert_eq!(path_reverse, vec!["C", "B", "A"]);
}
