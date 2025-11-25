//! Value system unit tests

use graphoid::values::{Graph, GraphType, List, Hash, PatternMatchResults, Value, ValueKind};
use std::collections::HashMap;

// ============================================================================
// Value Creation Tests
// ============================================================================

#[test]
fn test_create_number() {
    let val = Value::number(42.5);
    assert!(matches!(val.kind, ValueKind::Number(_)));
    assert_eq!(val.type_name(), "num");
    assert!(!val.is_frozen());
}

#[test]
fn test_create_string() {
    let val = Value::string("hello".to_string());
    assert!(matches!(val.kind, ValueKind::String(_)));
    assert_eq!(val.type_name(), "string");
    assert!(!val.is_frozen());
}

#[test]
fn test_create_boolean() {
    let val_true = Value::boolean(true);
    let val_false = Value::boolean(false);
    assert!(matches!(val_true.kind, ValueKind::Boolean(true)));
    assert!(matches!(val_false.kind, ValueKind::Boolean(false)));
    assert_eq!(val_true.type_name(), "bool");
    assert!(!val_true.is_frozen());
}

#[test]
fn test_create_none() {
    let val = Value::none();
    assert!(matches!(val.kind, ValueKind::None));
    assert_eq!(val.type_name(), "none");
    assert!(!val.is_frozen());
}

#[test]
fn test_create_list() {
    let items = vec![Value::number(1.0), Value::number(2.0), Value::number(3.0)];
    let list = List::from_vec(items);
    let val = Value::list(list);
    assert!(matches!(val.kind, ValueKind::List(_)));
    assert_eq!(val.type_name(), "list");
}

#[test]
fn test_create_map() {
    let mut hash = Hash::new();
    let _ = hash.insert("name".to_string(), Value::string("Alice".to_string()));
    let _ = hash.insert("age".to_string(), Value::number(30.0));
    let val = Value::map(hash);
    assert!(matches!(val.kind, ValueKind::Map(_)));
    assert_eq!(val.type_name(), "map");
}

#[test]
fn test_create_graph() {
    let graph = Graph::new(GraphType::Directed);
    let val = Value::graph(graph);
    assert!(matches!(val.kind, ValueKind::Graph(_)));
    assert_eq!(val.type_name(), "graph");
}

#[test]
fn test_create_pattern_node() {
    let val = Value::pattern_node(Some("x".to_string()), Some("User".to_string()));
    assert!(matches!(val.kind, ValueKind::PatternNode(_)));
    assert_eq!(val.type_name(), "pattern_node");
}

#[test]
fn test_create_pattern_edge() {
    let val = Value::pattern_edge(Some("FRIEND".to_string()), "outgoing".to_string());
    assert!(matches!(val.kind, ValueKind::PatternEdge(_)));
    assert_eq!(val.type_name(), "pattern_edge");
}

#[test]
fn test_create_pattern_path() {
    let val = Value::pattern_path("FOLLOWS".to_string(), 1, 3, "outgoing".to_string());
    assert!(matches!(val.kind, ValueKind::PatternPath(_)));
    assert_eq!(val.type_name(), "pattern_path");
}

#[test]
fn test_create_pattern_match_results() {
    let graph = Graph::new(GraphType::Directed);
    let bindings = vec![];
    let results = PatternMatchResults::new(bindings, graph);
    let val = Value::pattern_match_results(results);
    assert!(matches!(val.kind, ValueKind::PatternMatchResults(_)));
    assert_eq!(val.type_name(), "pattern_match_results");
}

// ============================================================================
// Truthiness Tests
// ============================================================================

#[test]
fn test_truthiness_numbers() {
    assert!(Value::number(1.0).is_truthy());
    assert!(Value::number(-1.0).is_truthy());
    assert!(!Value::number(0.0).is_truthy());
}

#[test]
fn test_truthiness_strings() {
    assert!(Value::string("hello".to_string()).is_truthy());
    assert!(!Value::string("".to_string()).is_truthy());
}

#[test]
fn test_truthiness_booleans() {
    assert!(Value::boolean(true).is_truthy());
    assert!(!Value::boolean(false).is_truthy());
}

#[test]
fn test_truthiness_none() {
    assert!(!Value::none().is_truthy());
}

#[test]
fn test_truthiness_list() {
    let empty_list = Value::list(List::from_vec(vec![]));
    assert!(!empty_list.is_truthy());

    let non_empty_list = Value::list(List::from_vec(vec![Value::number(1.0)]));
    assert!(non_empty_list.is_truthy());
}

#[test]
fn test_truthiness_map() {
    let empty_map = Value::map(Hash::new());
    assert!(!empty_map.is_truthy());

    let mut hash = Hash::new();
    let _ = hash.insert("key".to_string(), Value::number(1.0));
    let non_empty_map = Value::map(hash);
    assert!(non_empty_map.is_truthy());
}

#[test]
fn test_truthiness_graph() {
    let empty_graph = Value::graph(Graph::new(GraphType::Directed));
    assert!(!empty_graph.is_truthy());

    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    let non_empty_graph = Value::graph(graph);
    assert!(non_empty_graph.is_truthy());
}

#[test]
fn test_truthiness_pattern_match_results() {
    let graph = Graph::new(GraphType::Directed);

    let empty_results = Value::pattern_match_results(PatternMatchResults::new(vec![], graph.clone()));
    assert!(!empty_results.is_truthy());

    let mut binding = HashMap::new();
    binding.insert("x".to_string(), "node1".to_string());
    let non_empty_results = Value::pattern_match_results(PatternMatchResults::new(vec![binding], graph));
    assert!(non_empty_results.is_truthy());
}

#[test]
fn test_truthiness_pattern_objects() {
    // Pattern objects are always truthy
    assert!(Value::pattern_node(Some("x".to_string()), None).is_truthy());
    assert!(Value::pattern_edge(None, "outgoing".to_string()).is_truthy());
    assert!(Value::pattern_path("".to_string(), 1, 3, "outgoing".to_string()).is_truthy());
}

// ============================================================================
// String Representation Tests
// ============================================================================

#[test]
fn test_to_string_value_number() {
    let val = Value::number(42.5);
    assert_eq!(val.to_string_value(), "42.5");
}

#[test]
fn test_to_string_value_string() {
    let val = Value::string("hello".to_string());
    assert_eq!(val.to_string_value(), "hello");
}

#[test]
fn test_to_string_value_boolean() {
    assert_eq!(Value::boolean(true).to_string_value(), "true");
    assert_eq!(Value::boolean(false).to_string_value(), "false");
}

#[test]
fn test_to_string_value_none() {
    assert_eq!(Value::none().to_string_value(), "none");
}

#[test]
fn test_to_string_value_list() {
    let items = vec![Value::number(1.0), Value::number(2.0), Value::number(3.0)];
    let val = Value::list(List::from_vec(items));
    assert_eq!(val.to_string_value(), "[1, 2, 3]");
}

#[test]
fn test_to_string_value_map() {
    let mut hash = Hash::new();
    let _ = hash.insert("a".to_string(), Value::number(1.0));
    let val = Value::map(hash);
    let s = val.to_string_value();
    assert!(s.starts_with("{") && s.ends_with("}"));
    assert!(s.contains("\"a\": 1")); // Keys are quoted in map string representation
}

// ============================================================================
// Freezing Tests
// ============================================================================

#[test]
fn test_freeze_value() {
    let mut val = Value::number(42.0);
    assert!(!val.is_frozen());

    val.freeze();
    assert!(val.is_frozen());
}

#[test]
fn test_frozen_list_propagates() {
    let items = vec![Value::number(1.0), Value::number(2.0)];
    let list = List::from_vec(items);
    let mut val = Value::list(list);

    val.freeze();
    assert!(val.is_frozen());

    // The frozen state propagates to the underlying graph
    // (List doesn't expose is_frozen directly, but the Value wrapper tracks it)
}

#[test]
fn test_frozen_map_propagates() {
    let mut hash = Hash::new();
    let _ = hash.insert("key".to_string(), Value::number(1.0));
    let mut val = Value::map(hash);

    val.freeze();
    assert!(val.is_frozen());

    // The frozen state propagates to the underlying graph
    // (Hash doesn't expose is_frozen directly, but the Value wrapper tracks it)
}

#[test]
fn test_frozen_graph_propagates() {
    let graph = Graph::new(GraphType::Directed);
    let mut val = Value::graph(graph);

    val.freeze();
    assert!(val.is_frozen());

    if let ValueKind::Graph(g) = &val.kind {
        assert!(g.is_frozen());
    }
}

// ============================================================================
// Type Conversion Tests
// ============================================================================

#[test]
fn test_to_number() {
    // Number to number
    assert_eq!(Value::number(42.5).to_number(), Some(42.5));

    // Boolean to number (true=1, false=0)
    assert_eq!(Value::boolean(true).to_number(), Some(1.0));
    assert_eq!(Value::boolean(false).to_number(), Some(0.0));

    // String to number (if parseable)
    assert_eq!(Value::string("123.5".to_string()).to_number(), Some(123.5));
    assert_eq!(Value::string("not a number".to_string()).to_number(), None);

    // None and others return None
    assert_eq!(Value::none().to_number(), None);
}

#[test]
fn test_value_equality() {
    // Same type, same value
    assert_eq!(Value::number(42.0), Value::number(42.0));
    assert_eq!(Value::string("hello".to_string()), Value::string("hello".to_string()));
    assert_eq!(Value::boolean(true), Value::boolean(true));
    assert_eq!(Value::none(), Value::none());

    // Different values
    assert_ne!(Value::number(42.0), Value::number(43.0));
    assert_ne!(Value::string("hello".to_string()), Value::string("world".to_string()));
    assert_ne!(Value::boolean(true), Value::boolean(false));

    // Different types
    assert_ne!(Value::number(42.0), Value::string("42".to_string()));
    assert_ne!(Value::boolean(false), Value::none());
}

// ============================================================================
// Edge Cases and Special Values
// ============================================================================

#[test]
fn test_empty_collections() {
    let empty_list = Value::list(List::from_vec(vec![]));
    let empty_map = Value::map(Hash::new());
    let empty_graph = Value::graph(Graph::new(GraphType::Directed));

    assert!(!empty_list.is_truthy());
    assert!(!empty_map.is_truthy());
    assert!(!empty_graph.is_truthy());
}

#[test]
fn test_nested_collections() {
    let inner_list = Value::list(List::from_vec(vec![Value::number(1.0)]));
    let outer_list = Value::list(List::from_vec(vec![inner_list]));

    assert!(outer_list.is_truthy());
    assert_eq!(outer_list.to_string_value(), "[[1]]");
}

#[test]
fn test_pattern_node_with_no_variable() {
    let val = Value::pattern_node(None, Some("User".to_string()));
    assert!(matches!(val.kind, ValueKind::PatternNode(_)));
}

#[test]
fn test_pattern_edge_with_no_type() {
    let val = Value::pattern_edge(None, "both".to_string());
    assert!(matches!(val.kind, ValueKind::PatternEdge(_)));
}
