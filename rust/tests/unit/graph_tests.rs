//! Graph and Tree unit tests

use graphoid::values::{Graph, GraphType, Tree, Value};
use std::collections::HashMap;

// ============================================================================
// GRAPH TESTS
// ============================================================================

#[test]
fn test_graph_creation() {
    let g = Graph::new(GraphType::Directed);
    assert_eq!(g.node_count(), 0);
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn test_graph_add_node() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));

    assert_eq!(g.node_count(), 2);
    assert!(g.has_node("alice"));
    assert!(g.has_node("bob"));
    assert!(!g.has_node("charlie"));
}

#[test]
fn test_graph_add_edge() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));
    g.add_edge("alice", "bob", "follows".to_string(), HashMap::new());

    assert_eq!(g.edge_count(), 1);
    assert!(g.has_edge("alice", "bob"));
    assert!(!g.has_edge("bob", "alice")); // Directed
}

#[test]
fn test_graph_undirected_edge() {
    let mut g = Graph::new(GraphType::Undirected);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));
    g.add_edge("alice", "bob", "friend".to_string(), HashMap::new());

    // Undirected graphs have edges in both directions
    assert!(g.has_edge("alice", "bob"));
    assert!(g.has_edge("bob", "alice"));
    // But edge count is still reported correctly (counts both directions)
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_graph_neighbors() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));
    g.add_node("charlie".to_string(), Value::Number(3.0));

    g.add_edge("alice", "bob", "follows".to_string(), HashMap::new());
    g.add_edge("alice", "charlie", "follows".to_string(), HashMap::new());

    let neighbors = g.neighbors("alice");
    assert_eq!(neighbors.len(), 2);
    assert!(neighbors.contains(&"bob".to_string()));
    assert!(neighbors.contains(&"charlie".to_string()));

    let bob_neighbors = g.neighbors("bob");
    assert_eq!(bob_neighbors.len(), 0); // No outgoing edges
}

#[test]
fn test_graph_remove_node() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));
    g.add_edge("alice", "bob", "follows".to_string(), HashMap::new());

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);

    g.remove_node("bob");
    assert_eq!(g.node_count(), 1);
    assert_eq!(g.edge_count(), 0); // Edge to bob should be removed
    assert!(!g.has_node("bob"));
}

#[test]
fn test_graph_remove_edge() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));
    g.add_edge("alice", "bob", "follows".to_string(), HashMap::new());

    assert!(g.has_edge("alice", "bob"));

    let removed = g.remove_edge("alice", "bob");
    assert!(removed);
    assert!(!g.has_edge("alice", "bob"));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn test_graph_get_node() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(42.0));

    assert_eq!(g.get_node("alice"), Some(&Value::Number(42.0)));
    assert_eq!(g.get_node("bob"), None);
}

#[test]
fn test_graph_keys_values() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));
    g.add_node("bob".to_string(), Value::Number(2.0));

    let keys = g.keys();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"alice".to_string()));
    assert!(keys.contains(&"bob".to_string()));

    let values = g.values();
    assert_eq!(values.len(), 2);
}

// ============================================================================
// TREE TESTS
// ============================================================================

#[test]
fn test_tree_creation() {
    let t = Tree::new();
    assert_eq!(t.len(), 0);
    assert!(t.is_empty());
}

#[test]
fn test_tree_insert() {
    let mut t = Tree::new();
    t.insert(Value::Number(5.0));
    t.insert(Value::Number(3.0));
    t.insert(Value::Number(7.0));

    assert_eq!(t.len(), 3);
    assert!(!t.is_empty());
}

#[test]
fn test_tree_contains() {
    let mut t = Tree::new();
    t.insert(Value::Number(5.0));
    t.insert(Value::Number(3.0));
    t.insert(Value::Number(7.0));

    assert!(t.contains(&Value::Number(5.0)));
    assert!(t.contains(&Value::Number(3.0)));
    assert!(t.contains(&Value::Number(7.0)));
    assert!(!t.contains(&Value::Number(10.0)));
}

#[test]
fn test_tree_in_order_traversal() {
    let mut t = Tree::new();
    t.insert(Value::Number(5.0));
    t.insert(Value::Number(3.0));
    t.insert(Value::Number(7.0));
    t.insert(Value::Number(1.0));
    t.insert(Value::Number(9.0));

    let traversal = t.in_order();
    assert_eq!(traversal, vec![
        Value::Number(1.0),
        Value::Number(3.0),
        Value::Number(5.0),
        Value::Number(7.0),
        Value::Number(9.0),
    ]);
}

#[test]
fn test_tree_pre_order_traversal() {
    let mut t = Tree::new();
    t.insert(Value::Number(5.0));
    t.insert(Value::Number(3.0));
    t.insert(Value::Number(7.0));

    let traversal = t.pre_order();
    // Root, left, right
    assert_eq!(traversal, vec![
        Value::Number(5.0),
        Value::Number(3.0),
        Value::Number(7.0),
    ]);
}

#[test]
fn test_tree_post_order_traversal() {
    let mut t = Tree::new();
    t.insert(Value::Number(5.0));
    t.insert(Value::Number(3.0));
    t.insert(Value::Number(7.0));

    let traversal = t.post_order();
    // Left, right, root
    assert_eq!(traversal, vec![
        Value::Number(3.0),
        Value::Number(7.0),
        Value::Number(5.0),
    ]);
}

// ============================================================================
// VALUE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_graph_as_value() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::Number(1.0));

    let val = Value::Graph(g);
    assert_eq!(val.type_name(), "graph");
    assert!(val.is_truthy());
}

#[test]
fn test_empty_graph_is_falsy() {
    let g = Graph::new(GraphType::Directed);
    let val = Value::Graph(g);
    assert!(!val.is_truthy());
}

#[test]
fn test_tree_as_value() {
    let mut t = Tree::new();
    t.insert(Value::Number(5.0));

    let val = Value::Tree(t);
    assert_eq!(val.type_name(), "tree");
    assert!(val.is_truthy());
}

#[test]
fn test_empty_tree_is_falsy() {
    let t = Tree::new();
    let val = Value::Tree(t);
    assert!(!val.is_truthy());
}
