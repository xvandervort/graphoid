//! Unit Tests for Subgraph Operations
//!
//! Tests verify:
//! - extract_subgraph (with and without depth limits)
//! - insert_subgraph (validation, attachment, merging)

use graphoid::values::graph::{Graph, GraphType};
use graphoid::values::Value;
use std::collections::HashMap;

// =========================================================================
// Extract Subgraph Tests
// =========================================================================

#[test]
fn test_extract_subgraph_single_node() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();

    let subgraph = graph.extract_subgraph("A", None).unwrap();
    assert_eq!(subgraph.node_count(), 1);
    assert_eq!(subgraph.edge_count(), 0);
    assert!(subgraph.has_node("A"));
}

#[test]
fn test_extract_subgraph_linear_chain() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "next".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "next".to_string(), None, HashMap::new()).unwrap();

    let subgraph = graph.extract_subgraph("A", None).unwrap();
    assert_eq!(subgraph.node_count(), 3);
    assert_eq!(subgraph.edge_count(), 2);
    assert!(subgraph.has_node("A"));
    assert!(subgraph.has_node("B"));
    assert!(subgraph.has_node("C"));
    assert!(subgraph.has_edge("A", "B"));
    assert!(subgraph.has_edge("B", "C"));
}

#[test]
fn test_extract_subgraph_with_depth_limit() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("A", "B", "next".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "next".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("C", "D", "next".to_string(), None, HashMap::new()).unwrap();

    // Extract with depth 1 (only A and B)
    let subgraph = graph.extract_subgraph("A", Some(1)).unwrap();
    assert_eq!(subgraph.node_count(), 2, "Should extract 2 nodes at depth 1");
    assert!(subgraph.has_node("A"));
    assert!(subgraph.has_node("B"));
    assert!(!subgraph.has_node("C"));
    assert!(!subgraph.has_node("D"));
}

#[test]
fn test_extract_subgraph_tree_structure() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("left".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("right".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("left_child".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("root", "left", "child".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("root", "right", "child".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("left", "left_child", "child".to_string(), None, HashMap::new()).unwrap();

    let subgraph = graph.extract_subgraph("root", None).unwrap();
    assert_eq!(subgraph.node_count(), 4);
    assert_eq!(subgraph.edge_count(), 3);
    assert!(subgraph.has_edge("root", "left"));
    assert!(subgraph.has_edge("root", "right"));
    assert!(subgraph.has_edge("left", "left_child"));
}

#[test]
fn test_extract_subgraph_preserves_config() {
    use graphoid::values::graph::OrphanPolicy;

    let mut graph = Graph::new(GraphType::Directed);
    graph.config.orphan_policy = OrphanPolicy::Delete;
    graph.config.allow_overrides = true;

    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "next".to_string(), None, HashMap::new()).unwrap();

    let subgraph = graph.extract_subgraph("A", None).unwrap();

    // Verify config is preserved
    assert!(matches!(subgraph.config.orphan_policy, OrphanPolicy::Delete));
    assert_eq!(subgraph.config.allow_overrides, true);
}

#[test]
fn test_extract_subgraph_nonexistent_root() {
    let graph = Graph::new(GraphType::Directed);
    let result = graph.extract_subgraph("nonexistent", None);
    assert!(result.is_err(), "Should error on nonexistent root");
}

#[test]
fn test_extract_subgraph_depth_zero() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "next".to_string(), None, HashMap::new()).unwrap();

    let subgraph = graph.extract_subgraph("A", Some(0)).unwrap();
    assert_eq!(subgraph.node_count(), 1, "Depth 0 should only include root");
    assert!(subgraph.has_node("A"));
    assert!(!subgraph.has_node("B"));
}

// =========================================================================
// Insert Subgraph Tests
// =========================================================================

#[test]
fn test_insert_subgraph_simple() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("A".to_string(), Value::number(1.0)).unwrap();

    let mut subgraph = Graph::new(GraphType::Directed);
    subgraph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    subgraph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    subgraph.add_edge("B", "C", "link".to_string(), None, HashMap::new()).unwrap();

    main_graph.insert_subgraph(&subgraph, "A", "connects".to_string()).unwrap();

    assert_eq!(main_graph.node_count(), 3);
    assert!(main_graph.has_node("A"));
    assert!(main_graph.has_node("B"));
    assert!(main_graph.has_node("C"));
    assert!(main_graph.has_edge("B", "C"));
    assert!(main_graph.has_edge("A", "B")); // Should connect A to subgraph root
}

#[test]
fn test_insert_subgraph_with_root() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("main".to_string(), Value::number(1.0)).unwrap();

    let mut subgraph = Graph::new(GraphType::Directed);
    subgraph.add_node("sub_root".to_string(), Value::number(2.0)).unwrap();
    subgraph.add_node("sub_child".to_string(), Value::number(3.0)).unwrap();
    subgraph.add_edge("sub_root", "sub_child", "child".to_string(), None, HashMap::new()).unwrap();

    main_graph.insert_subgraph(&subgraph, "main", "has".to_string()).unwrap();

    // Should connect main to sub_root (the root of subgraph)
    assert!(main_graph.has_edge("main", "sub_root"));
    assert!(main_graph.has_edge("sub_root", "sub_child"));
}

#[test]
fn test_insert_subgraph_node_conflict() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    main_graph.add_node("B".to_string(), Value::number(2.0)).unwrap();

    let mut subgraph = Graph::new(GraphType::Directed);
    subgraph.add_node("B".to_string(), Value::number(3.0)).unwrap(); // Conflict!

    let result = main_graph.insert_subgraph(&subgraph, "A", "link".to_string());
    assert!(result.is_err(), "Should error on node ID conflict");
}

#[test]
fn test_insert_subgraph_nonexistent_attachment() {
    let main_graph = Graph::new(GraphType::Directed);
    let subgraph = Graph::new(GraphType::Directed);

    let result = main_graph.clone().insert_subgraph(&subgraph, "nonexistent", "link".to_string());
    assert!(result.is_err(), "Should error when attachment node doesn't exist");
}

#[test]
fn test_insert_subgraph_frozen_graph() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    main_graph.freeze();

    let subgraph = Graph::new(GraphType::Directed);
    let result = main_graph.insert_subgraph(&subgraph, "A", "link".to_string());

    assert!(result.is_err(), "Should error when trying to insert into frozen graph");
}

#[test]
fn test_insert_subgraph_empty_subgraph() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("A".to_string(), Value::number(1.0)).unwrap();

    let subgraph = Graph::new(GraphType::Directed);

    main_graph.insert_subgraph(&subgraph, "A", "link".to_string()).unwrap();

    // Should succeed but not add anything
    assert_eq!(main_graph.node_count(), 1);
}

#[test]
fn test_insert_subgraph_multiple_roots() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("main".to_string(), Value::number(1.0)).unwrap();

    let mut subgraph = Graph::new(GraphType::Directed);
    subgraph.add_node("root1".to_string(), Value::number(2.0)).unwrap();
    subgraph.add_node("root2".to_string(), Value::number(3.0)).unwrap();
    subgraph.add_node("child".to_string(), Value::number(4.0)).unwrap();
    subgraph.add_edge("root1", "child", "link".to_string(), None, HashMap::new()).unwrap();
    subgraph.add_edge("root2", "child", "link".to_string(), None, HashMap::new()).unwrap();

    main_graph.insert_subgraph(&subgraph, "main", "connects".to_string()).unwrap();

    // Should connect main to both roots
    assert!(main_graph.has_edge("main", "root1"));
    assert!(main_graph.has_edge("main", "root2"));
}

#[test]
fn test_insert_subgraph_preserves_edges() {
    let mut main_graph = Graph::new(GraphType::Directed);
    main_graph.add_node("A".to_string(), Value::number(1.0)).unwrap();

    let mut subgraph = Graph::new(GraphType::Directed);
    subgraph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    subgraph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    subgraph.add_node("D".to_string(), Value::number(4.0)).unwrap();
    subgraph.add_edge("B", "C", "link1".to_string(), None, HashMap::new()).unwrap();
    subgraph.add_edge("B", "D", "link2".to_string(), None, HashMap::new()).unwrap();

    main_graph.insert_subgraph(&subgraph, "A", "connects".to_string()).unwrap();

    // Verify all internal edges are preserved
    assert!(main_graph.has_edge("B", "C"));
    assert!(main_graph.has_edge("B", "D"));
}

// =========================================================================
// Integration Tests
// =========================================================================

#[test]
fn test_extract_then_insert() {
    // Create original graph
    let mut original = Graph::new(GraphType::Directed);
    original.add_node("A".to_string(), Value::number(1.0)).unwrap();
    original.add_node("B".to_string(), Value::number(2.0)).unwrap();
    original.add_node("C".to_string(), Value::number(3.0)).unwrap();
    original.add_edge("A", "B", "link".to_string(), None, HashMap::new()).unwrap();
    original.add_edge("B", "C", "link".to_string(), None, HashMap::new()).unwrap();

    // Extract subgraph
    let extracted = original.extract_subgraph("B", None).unwrap();
    assert_eq!(extracted.node_count(), 2); // B and C

    // Create new graph and insert extracted subgraph
    let mut new_graph = Graph::new(GraphType::Directed);
    new_graph.add_node("X".to_string(), Value::number(10.0)).unwrap();
    new_graph.insert_subgraph(&extracted, "X", "has".to_string()).unwrap();

    assert_eq!(new_graph.node_count(), 3); // X, B, C
    assert!(new_graph.has_node("X"));
    assert!(new_graph.has_node("B"));
    assert!(new_graph.has_node("C"));
    assert!(new_graph.has_edge("X", "B")); // X connected to B (root of extracted)
    assert!(new_graph.has_edge("B", "C")); // Original edge preserved
}
