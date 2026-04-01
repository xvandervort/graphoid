//! Tests for graph traversal and query methods: bfs, dfs, neighbors, nodes_within, has_key

use graphoid::values::{Graph, GraphType, Value};
use std::collections::HashMap;

fn make_test_graph() -> Graph {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("A".to_string(), Value::number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::number(3.0)).unwrap();
    g.add_node("D".to_string(), Value::number(4.0)).unwrap();
    g.add_edge("A", "B", "link".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("B", "C", "link".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("A", "D", "other".to_string(), None, HashMap::new()).unwrap();
    g
}

// ============================================================================
// BFS TESTS
// ============================================================================

#[test]
fn test_bfs_basic_traversal() {
    let g = make_test_graph();
    let order = g.bfs("A");
    // A should be first, then its neighbors B and D in some order, then C
    assert_eq!(order[0], "A");
    assert!(order.contains(&"B".to_string()));
    assert!(order.contains(&"C".to_string()));
    assert!(order.contains(&"D".to_string()));
    assert_eq!(order.len(), 4);
}

#[test]
fn test_bfs_visits_all_reachable() {
    let g = make_test_graph();
    let order = g.bfs("A");
    assert_eq!(order.len(), 4);
}

#[test]
fn test_bfs_from_middle_node() {
    let g = make_test_graph();
    let order = g.bfs("B");
    // B can reach C but not A or D (directed)
    assert!(order.contains(&"B".to_string()));
    assert!(order.contains(&"C".to_string()));
    assert!(!order.contains(&"A".to_string()));
}

#[test]
fn test_bfs_nonexistent_start() {
    let g = make_test_graph();
    let order = g.bfs("Z");
    assert!(order.is_empty());
}

#[test]
fn test_bfs_single_node() {
    let g = make_test_graph();
    let order = g.bfs("C");
    // C has no outgoing edges
    assert_eq!(order, vec!["C"]);
}

// ============================================================================
// DFS TESTS
// ============================================================================

#[test]
fn test_dfs_basic_traversal() {
    let g = make_test_graph();
    let order = g.dfs("A");
    assert_eq!(order[0], "A");
    assert!(order.contains(&"B".to_string()));
    assert!(order.contains(&"C".to_string()));
    assert!(order.contains(&"D".to_string()));
    assert_eq!(order.len(), 4);
}

#[test]
fn test_dfs_from_middle_node() {
    let g = make_test_graph();
    let order = g.dfs("B");
    assert!(order.contains(&"B".to_string()));
    assert!(order.contains(&"C".to_string()));
    assert!(!order.contains(&"A".to_string()));
}

#[test]
fn test_dfs_nonexistent_start() {
    let g = make_test_graph();
    let order = g.dfs("Z");
    assert!(order.is_empty());
}

#[test]
fn test_dfs_single_node() {
    let g = make_test_graph();
    let order = g.dfs("C");
    assert_eq!(order, vec!["C"]);
}

// ============================================================================
// NEIGHBORS TESTS
// ============================================================================

#[test]
fn test_neighbors_returns_adjacent_nodes() {
    let g = make_test_graph();
    let n = g.neighbors("A");
    assert!(n.contains(&"B".to_string()));
    assert!(n.contains(&"D".to_string()));
    assert_eq!(n.len(), 2);
}

#[test]
fn test_neighbors_single_neighbor() {
    let g = make_test_graph();
    let n = g.neighbors("B");
    assert_eq!(n.len(), 1);
    assert!(n.contains(&"C".to_string()));
}

#[test]
fn test_neighbors_no_neighbors() {
    let g = make_test_graph();
    let n = g.neighbors("C");
    assert!(n.is_empty());
}

#[test]
fn test_neighbors_nonexistent_node() {
    let g = make_test_graph();
    let n = g.neighbors("Z");
    assert!(n.is_empty());
}

#[test]
fn test_neighbors_filtered_by_edge_type() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("A".to_string(), Value::number(1.0)).unwrap();
    g.add_node("B".to_string(), Value::number(2.0)).unwrap();
    g.add_node("C".to_string(), Value::number(3.0)).unwrap();
    g.add_node("D".to_string(), Value::number(4.0)).unwrap();
    g.add_edge("A", "B", "friend".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("A", "C", "friend".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("A", "D", "colleague".to_string(), None, HashMap::new()).unwrap();

    let friends = g.neighbors_by_type("A", "friend");
    assert!(friends.contains(&"B".to_string()));
    assert!(friends.contains(&"C".to_string()));
    assert!(!friends.contains(&"D".to_string()));
    assert_eq!(friends.len(), 2);

    let colleagues = g.neighbors_by_type("A", "colleague");
    assert_eq!(colleagues, vec!["D"]);

    let none = g.neighbors_by_type("A", "enemy");
    assert!(none.is_empty());
}

// ============================================================================
// NODES_WITHIN TESTS
// ============================================================================

#[test]
fn test_nodes_within_zero_hops() {
    let g = make_test_graph();
    let nodes = g.nodes_within("A", 0, None);
    assert_eq!(nodes, vec!["A"]);
}

#[test]
fn test_nodes_within_one_hop() {
    let g = make_test_graph();
    let nodes = g.nodes_within("A", 1, None);
    assert!(nodes.contains(&"A".to_string()));
    assert!(nodes.contains(&"B".to_string()));
    assert!(nodes.contains(&"D".to_string()));
    assert!(!nodes.contains(&"C".to_string())); // 2 hops away
}

#[test]
fn test_nodes_within_two_hops() {
    let g = make_test_graph();
    let nodes = g.nodes_within("A", 2, None);
    assert_eq!(nodes.len(), 4); // A, B, C, D all reachable
}

#[test]
fn test_nodes_within_with_edge_type_filter() {
    let g = make_test_graph();
    // Only follow "link" edges — should reach B and C but not D ("other" edge)
    let nodes = g.nodes_within("A", 2, Some("link"));
    assert!(nodes.contains(&"A".to_string()));
    assert!(nodes.contains(&"B".to_string()));
    assert!(nodes.contains(&"C".to_string()));
    assert!(!nodes.contains(&"D".to_string()));
}

#[test]
fn test_nodes_within_nonexistent_start() {
    let g = make_test_graph();
    let nodes = g.nodes_within("Z", 5, None);
    assert!(nodes.is_empty());
}
