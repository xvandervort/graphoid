//! Unit Tests for Filter-Based Subgraph Operations
//!
//! Tests verify the specification-compliant filter-based API:
//! - extract(filter_map) with node/edge filters
//! - delete(filter_map) with node/edge filters
//! - add_subgraph(other, on_conflict) with conflict resolution

use graphoid::values::graph::{Graph, GraphType};
use graphoid::values::{Value, ValueKind};
use std::collections::HashMap;

// =========================================================================
// Extract with Node Filter Tests
// =========================================================================

#[test]
fn test_extract_with_node_filter_only() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "link".to_string(), None, HashMap::new()).unwrap();

    // Extract nodes where value > 1.5
    let node_filter = Box::new(|_id: &str, val: &Value| -> bool {
        if let ValueKind::Number(n) = &val.kind {
            *n > 1.5
        } else {
            false
        }
    });

    let result = graph.extract_filtered(Some(node_filter), None, true).unwrap();

    assert_eq!(result.node_count(), 2, "Should extract B and C");
    assert!(!result.has_node("A"));
    assert!(result.has_node("B"));
    assert!(result.has_node("C"));
    assert!(result.has_edge("B", "C"), "Should preserve edge between extracted nodes");
}

#[test]
fn test_extract_with_node_filter_no_matches() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();

    // Filter that matches nothing
    let node_filter = Box::new(|_id: &str, val: &Value| -> bool {
        if let ValueKind::Number(n) = &val.kind {
            *n > 100.0
        } else {
            false
        }
    });

    let result = graph.extract_filtered(Some(node_filter), None, true).unwrap();

    assert_eq!(result.node_count(), 0, "Should extract no nodes");
    assert_eq!(result.edge_count(), 0, "Should have no edges");
}

#[test]
fn test_extract_with_node_filter_all_match() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, HashMap::new()).unwrap();

    // Filter that matches everything
    let node_filter = Box::new(|_id: &str, _val: &Value| -> bool { true });

    let result = graph.extract_filtered(Some(node_filter), None, true).unwrap();

    assert_eq!(result.node_count(), 2, "Should extract all nodes");
    assert_eq!(result.edge_count(), 1, "Should preserve all edges");
}

// =========================================================================
// Extract with Edge Filter Tests
// =========================================================================

#[test]
fn test_extract_with_edge_filter_only() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "friend".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "colleague".to_string(), None, HashMap::new()).unwrap();

    // Extract edges of type "friend" only
    let edge_filter = Box::new(|_from: &str, _to: &str, edge_type: &str, _weight: Option<f64>, _attrs: &HashMap<String, Value>| -> bool {
        edge_type == "friend"
    });

    let result = graph.extract_filtered(None, Some(edge_filter), true).unwrap();

    // With include_orphans: true, should have all nodes but filtered edges
    assert_eq!(result.node_count(), 3, "Should include all nodes (orphans included)");
    assert_eq!(result.edge_count(), 1, "Should only have friend edge");
    assert!(result.has_edge("A", "B"));
    assert!(!result.has_edge("B", "C"));
}

#[test]
fn test_extract_with_edge_filter_exclude_orphans() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("A", "B", "friend".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "colleague".to_string(), None, HashMap::new()).unwrap();

    // Extract edges of type "friend" only, exclude orphans
    let edge_filter = Box::new(|_from: &str, _to: &str, edge_type: &str, _weight: Option<f64>, _attrs: &HashMap<String, Value>| -> bool {
        edge_type == "friend"
    });

    let result = graph.extract_filtered(None, Some(edge_filter), false).unwrap();

    // Should only have A and B (nodes connected by friend edge)
    assert_eq!(result.node_count(), 2, "Should exclude orphan nodes C and D");
    assert!(result.has_node("A"));
    assert!(result.has_node("B"));
    assert!(!result.has_node("C"));
    assert!(!result.has_node("D"));
    assert_eq!(result.edge_count(), 1);
}

// =========================================================================
// Extract with Both Node and Edge Filters Tests
// =========================================================================

#[test]
fn test_extract_with_both_node_and_edge_filters() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("A", "B", "friend".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "friend".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("C", "D", "colleague".to_string(), None, HashMap::new()).unwrap();

    // Filter nodes: value >= 2
    let node_filter = Box::new(|_id: &str, val: &Value| -> bool {
        if let ValueKind::Number(n) = &val.kind {
            *n >= 2.0
        } else {
            false
        }
    });

    // Filter edges: type == "friend"
    let edge_filter = Box::new(|_from: &str, _to: &str, edge_type: &str, _weight: Option<f64>, _attrs: &HashMap<String, Value>| -> bool {
        edge_type == "friend"
    });

    let result = graph.extract_filtered(Some(node_filter), Some(edge_filter), true).unwrap();

    // Nodes B, C, D match node filter (value >= 2)
    // Edges: B->C matches both filters (friend edge between filtered nodes)
    // Edge A->B doesn't match (A is filtered out)
    // Edge C->D doesn't match (colleague edge)
    assert_eq!(result.node_count(), 3, "Should have B, C, D");
    assert!(result.has_node("B"));
    assert!(result.has_node("C"));
    assert!(result.has_node("D"));
    assert_eq!(result.edge_count(), 1, "Should only have B->C friend edge");
    assert!(result.has_edge("B", "C"));
}

// =========================================================================
// Extract Edge Cases
// =========================================================================

#[test]
fn test_extract_empty_graph() {
    let graph = Graph::new(GraphType::Directed);

    let node_filter = Box::new(|_id: &str, _val: &Value| -> bool { true });
    let result = graph.extract_filtered(Some(node_filter), None, true).unwrap();

    assert_eq!(result.node_count(), 0);
    assert_eq!(result.edge_count(), 0);
}

#[test]
fn test_extract_preserves_graph_config() {
    use graphoid::values::graph::OrphanPolicy;

    let mut graph = Graph::new(GraphType::Directed);
    graph.config.orphan_policy = OrphanPolicy::Delete;
    graph.config.allow_overrides = true;

    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();

    let node_filter = Box::new(|_id: &str, _val: &Value| -> bool { true });
    let result = graph.extract_filtered(Some(node_filter), None, true).unwrap();

    // Should preserve configuration
    assert!(matches!(result.config.orphan_policy, OrphanPolicy::Delete));
    assert_eq!(result.config.allow_overrides, true);
}

// =========================================================================
// Delete with Filters Tests
// =========================================================================

#[test]
fn test_delete_with_node_filter() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), None, HashMap::new()).unwrap();

    // Delete nodes where value < 2
    let node_filter = Box::new(|_id: &str, val: &Value| -> bool {
        if let ValueKind::Number(n) = &val.kind {
            *n < 2.0
        } else {
            false
        }
    });

    let result = graph.delete_filtered(Some(node_filter), None).unwrap();

    // Should keep B and C, delete A
    assert_eq!(result.node_count(), 2);
    assert!(!result.has_node("A"));
    assert!(result.has_node("B"));
    assert!(result.has_node("C"));
}

#[test]
fn test_delete_with_edge_filter() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("A", "B", "friend".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("B", "C", "colleague".to_string(), None, HashMap::new()).unwrap();

    // Delete colleague edges
    let edge_filter = Box::new(|_from: &str, _to: &str, edge_type: &str, _weight: Option<f64>, _attrs: &HashMap<String, Value>| -> bool {
        edge_type == "colleague"
    });

    let result = graph.delete_filtered(None, Some(edge_filter)).unwrap();

    // Should keep all nodes but delete colleague edge
    assert_eq!(result.node_count(), 3);
    assert_eq!(result.edge_count(), 1);
    assert!(result.has_edge("A", "B"));
    assert!(!result.has_edge("B", "C"));
}

#[test]
fn test_delete_nothing() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();

    // Filter that matches nothing
    let node_filter = Box::new(|_id: &str, _val: &Value| -> bool { false });

    let result = graph.delete_filtered(Some(node_filter), None).unwrap();

    // Should keep everything
    assert_eq!(result.node_count(), 2);
}

// =========================================================================
// Add Subgraph with Conflict Resolution Tests
// =========================================================================

#[test]
fn test_add_subgraph_no_conflicts() {
    let mut graph_a = Graph::new(GraphType::Directed);
    graph_a.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph_a.add_node("B".to_string(), Value::number(2.0)).unwrap();

    let mut graph_b = Graph::new(GraphType::Directed);
    graph_b.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph_b.add_node("D".to_string(), Value::number(4.0)).unwrap();
    graph_b.add_edge("C", "D", "link".to_string(), None, HashMap::new()).unwrap();

    let result = graph_a.add_subgraph(&graph_b, None).unwrap();

    assert_eq!(result.node_count(), 4);
    assert!(result.has_node("A"));
    assert!(result.has_node("B"));
    assert!(result.has_node("C"));
    assert!(result.has_node("D"));
    assert!(result.has_edge("C", "D"));
}

#[test]
fn test_add_subgraph_keep_original() {
    let mut graph_a = Graph::new(GraphType::Directed);
    graph_a.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph_a.add_node("B".to_string(), Value::number(2.0)).unwrap();

    let mut graph_b = Graph::new(GraphType::Directed);
    graph_b.add_node("B".to_string(), Value::number(99.0)).unwrap(); // Conflict!
    graph_b.add_node("C".to_string(), Value::number(3.0)).unwrap();

    let result = graph_a.add_subgraph(&graph_b, Some("keep_original".to_string())).unwrap();

    assert_eq!(result.node_count(), 3);

    // B should have original value (2.0)
    if let Some(node_value) = result.get_node("B") {
        if let ValueKind::Number(n) = &node_value.kind {
            assert_eq!(*n, 2.0, "Should keep original value");
        } else {
            panic!("Expected B to have number value");
        }
    } else {
        panic!("Node B not found");
    }
}

#[test]
fn test_add_subgraph_overwrite() {
    let mut graph_a = Graph::new(GraphType::Directed);
    graph_a.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph_a.add_node("B".to_string(), Value::number(2.0)).unwrap();

    let mut graph_b = Graph::new(GraphType::Directed);
    graph_b.add_node("B".to_string(), Value::number(99.0)).unwrap(); // Conflict!
    graph_b.add_node("C".to_string(), Value::number(3.0)).unwrap();

    let result = graph_a.add_subgraph(&graph_b, Some("overwrite".to_string())).unwrap();

    assert_eq!(result.node_count(), 3);

    // B should have new value (99.0)
    if let Some(node_value) = result.get_node("B") {
        if let ValueKind::Number(n) = &node_value.kind {
            assert_eq!(*n, 99.0, "Should overwrite with new value");
        } else {
            panic!("Expected B to have number value");
        }
    } else {
        panic!("Node B not found");
    }
}

#[test]
fn test_add_subgraph_merge_edges() {
    let mut graph_a = Graph::new(GraphType::Directed);
    graph_a.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph_a.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph_a.add_edge("A", "B", "link1".to_string(), None, HashMap::new()).unwrap();

    let mut graph_b = Graph::new(GraphType::Directed);
    graph_b.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph_b.add_node("C".to_string(), Value::number(3.0)).unwrap();
    graph_b.add_edge("B", "C", "link2".to_string(), None, HashMap::new()).unwrap();

    let result = graph_a.add_subgraph(&graph_b, None).unwrap();

    assert_eq!(result.node_count(), 3);
    assert_eq!(result.edge_count(), 2, "Should merge all edges");
    assert!(result.has_edge("A", "B"));
    assert!(result.has_edge("B", "C"));
}

#[test]
fn test_add_subgraph_empty() {
    let mut graph_a = Graph::new(GraphType::Directed);
    graph_a.add_node("A".to_string(), Value::number(1.0)).unwrap();

    let graph_b = Graph::new(GraphType::Directed);

    let result = graph_a.add_subgraph(&graph_b, None).unwrap();

    assert_eq!(result.node_count(), 1, "Should keep original graph");
}
