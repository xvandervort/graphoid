//! Tests for automatic property indexing
//!
//! These tests verify that the graph automatically creates indices after
//! repeated property lookups, and that these indices improve performance.

use graphoid::values::{Value, Graph};
use graphoid::values::graph::GraphType;
use std::collections::HashMap;

#[test]
fn test_no_index_before_threshold() {
    let mut graph = Graph::new(GraphType::Directed);

    // Add some nodes with properties
    graph.add_node("user1".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("user2".to_string(), Value::Number(2.0)).unwrap();

    // Do a few lookups (below threshold of 10)
    for _ in 0..5 {
        graph.find_nodes_by_property("user_id", &Value::Number(42.0));
    }

    // Should not have created an index yet
    assert!(!graph.has_auto_index("user_id"));
}

#[test]
fn test_auto_index_created_after_threshold() {
    let mut graph = Graph::new(GraphType::Directed);

    // Add some nodes with properties
    graph.add_node("user1".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("user2".to_string(), Value::Number(2.0)).unwrap();

    // Do 10 lookups (meets threshold)
    for _ in 0..10 {
        graph.find_nodes_by_property("user_id", &Value::Number(42.0));
    }

    // Should have created an index
    assert!(graph.has_auto_index("user_id"));
}

#[test]
fn test_find_nodes_by_property_without_index() {
    let mut graph = Graph::new(GraphType::Directed);

    // Add nodes and set a property manually (since add_node doesn't support properties yet)
    // For now, we'll just test that find_nodes works with empty results
    let results = graph.find_nodes_by_property("age", &Value::Number(25.0));

    assert_eq!(results.len(), 0);
}

#[test]
fn test_stats_shows_auto_indices() {
    let mut graph = Graph::new(GraphType::Directed);

    // Trigger auto-indexing
    for _ in 0..10 {
        graph.find_nodes_by_property("email", &Value::String("test@example.com".to_string()));
    }

    let stats = graph.stats();

    // Should show the auto-created index
    assert!(stats.contains_key("auto_indices"));

    if let Some(indices) = stats.get("auto_indices") {
        let indices_array = indices.as_array().unwrap();
        assert_eq!(indices_array.len(), 1);
        assert_eq!(indices_array[0].as_str().unwrap(), "email");
    }
}

#[test]
fn test_multiple_properties_indexed() {
    let mut graph = Graph::new(GraphType::Directed);

    // Trigger indexing on multiple properties
    for _ in 0..10 {
        graph.find_nodes_by_property("email", &Value::String("test@example.com".to_string()));
    }

    for _ in 0..10 {
        graph.find_nodes_by_property("age", &Value::Number(25.0));
    }

    // Both should be indexed
    assert!(graph.has_auto_index("email"));
    assert!(graph.has_auto_index("age"));

    let stats = graph.stats();
    if let Some(indices) = stats.get("auto_indices") {
        let indices_array = indices.as_array().unwrap();
        assert_eq!(indices_array.len(), 2);
    }
}

#[test]
fn test_index_created_only_once() {
    let mut graph = Graph::new(GraphType::Directed);

    // Do many lookups (well beyond threshold)
    for _ in 0..50 {
        graph.find_nodes_by_property("status", &Value::String("active".to_string()));
    }

    // Should still have only one index
    assert!(graph.has_auto_index("status"));

    let stats = graph.stats();
    if let Some(indices) = stats.get("auto_indices") {
        let indices_array = indices.as_array().unwrap();
        assert_eq!(indices_array.len(), 1);
    }
}

#[test]
fn test_stats_includes_node_count() {
    let mut graph = Graph::new(GraphType::Directed);

    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    let stats = graph.stats();

    assert!(stats.contains_key("node_count"));
    assert_eq!(stats.get("node_count").unwrap().as_u64().unwrap(), 3);
}

#[test]
fn test_stats_includes_edge_count() {
    let mut graph = Graph::new(GraphType::Directed);

    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_edge("A", "B", "link".to_string(), HashMap::new()).unwrap();

    let stats = graph.stats();

    assert!(stats.contains_key("edge_count"));
    assert_eq!(stats.get("edge_count").unwrap().as_u64().unwrap(), 1);
}
