//! Unit Tests for Orphan Detection and Management
//!
//! Tests verify:
//! - Orphan detection (find, count, has)
//! - Orphan deletion (delete all orphans)
//! - Orphan reconnection (to root, manual)

use graphoid::values::graph::{Graph, GraphType, ReconnectStrategy};
use graphoid::values::Value;

// =========================================================================
// Orphan Detection Tests
// =========================================================================

#[test]
fn test_find_orphans_empty_graph() {
    let graph = Graph::new(GraphType::Directed);
    let orphans = graph.find_orphans();
    assert_eq!(orphans.len(), 0, "Empty graph should have no orphans");
}

#[test]
fn test_find_orphans_no_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let orphans = graph.find_orphans();
    assert_eq!(orphans.len(), 0, "Graph with connected nodes should have no orphans");
}

#[test]
fn test_find_orphans_single_orphan() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("orphan".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let orphans = graph.find_orphans();
    assert_eq!(orphans.len(), 1, "Should find exactly one orphan");
    assert!(orphans.contains(&"orphan".to_string()), "Should find the orphan node");
}

#[test]
fn test_find_orphans_multiple_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("orphan1".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("orphan2".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let orphans = graph.find_orphans();
    assert_eq!(orphans.len(), 2, "Should find exactly two orphans");
    assert!(orphans.contains(&"orphan1".to_string()), "Should find orphan1");
    assert!(orphans.contains(&"orphan2".to_string()), "Should find orphan2");
}

#[test]
fn test_count_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("orphan1".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("orphan2".to_string(), Value::number(3.0)).unwrap();

    assert_eq!(graph.count_orphans(), 3, "All nodes are orphans");

    graph.add_edge("A", "orphan1", "link".to_string(), None, std::collections::HashMap::new()).unwrap();
    assert_eq!(graph.count_orphans(), 1, "Only orphan2 should remain");
}

#[test]
fn test_has_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    assert!(!graph.has_orphans(), "Empty graph has no orphans");

    graph.add_node("orphan".to_string(), Value::number(1.0)).unwrap();
    assert!(graph.has_orphans(), "Graph with isolated node has orphans");

    graph.add_node("A".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "orphan", "link".to_string(), None, std::collections::HashMap::new()).unwrap();
    assert!(!graph.has_orphans(), "All nodes connected, no orphans");
}

// =========================================================================
// Orphan Deletion Tests
// =========================================================================

#[test]
fn test_delete_orphans_empty_graph() {
    let mut graph = Graph::new(GraphType::Directed);
    let deleted = graph.delete_orphans().unwrap();
    assert_eq!(deleted.len(), 0, "No orphans to delete in empty graph");
}

#[test]
fn test_delete_orphans_no_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let deleted = graph.delete_orphans().unwrap();
    assert_eq!(deleted.len(), 0, "No orphans to delete");
    assert_eq!(graph.node_count(), 2, "Both nodes should remain");
}

#[test]
fn test_delete_orphans_single() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("orphan".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let deleted = graph.delete_orphans().unwrap();
    assert_eq!(deleted.len(), 1, "Should delete one orphan");
    assert!(deleted.contains(&"orphan".to_string()), "Should delete the orphan");
    assert_eq!(graph.node_count(), 2, "Only A and B should remain");
    assert!(!graph.has_orphans(), "No orphans should remain");
}

#[test]
fn test_delete_orphans_multiple() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("orphan1".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("orphan2".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("orphan3".to_string(), Value::number(4.0)).unwrap();

    assert_eq!(graph.count_orphans(), 4, "All nodes are orphans initially");

    let deleted = graph.delete_orphans().unwrap();
    assert_eq!(deleted.len(), 4, "Should delete all orphans");
    assert_eq!(graph.node_count(), 0, "Graph should be empty");
    assert!(!graph.has_orphans(), "No orphans should remain");
}

#[test]
fn test_delete_orphans_preserves_connected() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("child1".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("child2".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("orphan".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("root", "child1", "link".to_string(), None, std::collections::HashMap::new()).unwrap();
    graph.add_edge("root", "child2", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let deleted = graph.delete_orphans().unwrap();
    assert_eq!(deleted.len(), 1, "Should delete only the orphan");
    assert_eq!(graph.node_count(), 3, "Root and children should remain");
    assert!(graph.has_node("root"), "Root should remain");
    assert!(graph.has_node("child1"), "Child1 should remain");
    assert!(graph.has_node("child2"), "Child2 should remain");
    assert!(!graph.has_node("orphan"), "Orphan should be deleted");
}

// =========================================================================
// Orphan Reconnection Tests
// =========================================================================

#[test]
fn test_reconnect_orphan_success() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("orphan".to_string(), Value::number(2.0)).unwrap();

    assert!(graph.has_orphans(), "Should have orphans initially");

    graph.reconnect_orphan("orphan", "root", "reconnected".to_string()).unwrap();

    assert!(!graph.has_orphans(), "Should have no orphans after reconnection");
    assert!(graph.has_edge("root", "orphan"), "Edge should exist from root to orphan");
}

#[test]
fn test_reconnect_orphan_nonexistent_orphan() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();

    let result = graph.reconnect_orphan("nonexistent", "root", "link".to_string());
    assert!(result.is_err(), "Should error when orphan doesn't exist");
}

#[test]
fn test_reconnect_orphan_not_an_orphan() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let result = graph.reconnect_orphan("B", "A", "link".to_string());
    assert!(result.is_err(), "Should error when node is not an orphan");
}

#[test]
fn test_reconnect_orphan_nonexistent_parent() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("orphan".to_string(), Value::number(1.0)).unwrap();

    let result = graph.reconnect_orphan("orphan", "nonexistent", "link".to_string());
    assert!(result.is_err(), "Should error when parent doesn't exist");
}

#[test]
fn test_reconnect_orphans_to_root() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("child".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("orphan1".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("orphan2".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("root", "child", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    assert_eq!(graph.count_orphans(), 2, "Should have 2 orphans initially");

    let count = graph.reconnect_orphans(ReconnectStrategy::ToRoot).unwrap();
    assert_eq!(count, 2, "Should reconnect 2 orphans");
    assert!(!graph.has_orphans(), "Should have no orphans after reconnection");
    assert!(graph.has_edge("root", "orphan1"), "Orphan1 should be connected to root");
    assert!(graph.has_edge("root", "orphan2"), "Orphan2 should be connected to root");
}

#[test]
fn test_reconnect_orphans_no_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, std::collections::HashMap::new()).unwrap();

    let count = graph.reconnect_orphans(ReconnectStrategy::ToRoot).unwrap();
    assert_eq!(count, 0, "No orphans to reconnect");
}

#[test]
fn test_reconnect_orphans_no_root() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("orphan1".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("orphan2".to_string(), Value::number(2.0)).unwrap();

    let result = graph.reconnect_orphans(ReconnectStrategy::ToRoot);
    assert!(result.is_err(), "Should error when no root node exists");
}

#[test]
fn test_reconnect_orphans_to_parent_siblings_not_implemented() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("orphan".to_string(), Value::number(1.0)).unwrap();

    let result = graph.reconnect_orphans(ReconnectStrategy::ToParentSiblings);
    assert!(result.is_err(), "ToParentSiblings strategy not yet implemented");
}
