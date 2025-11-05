use graphoid::values::{Graph, GraphType, Value};
use std::collections::HashMap;

/// Helper function to create a pattern node value
fn create_pattern_node(variable: &str, node_type: Option<&str>) -> Value {
    Value::pattern_node(
        Some(variable.to_string()),
        node_type.map(|s| s.to_string())
    )
}

/// Helper function to create a pattern edge value
fn create_pattern_edge(edge_type: Option<&str>, direction: Option<&str>) -> Value {
    Value::pattern_edge(
        edge_type.map(|s| s.to_string()),
        direction.unwrap_or("outgoing").to_string()
    )
}

#[test]
fn test_simple_two_node_pattern() {
    // Build a simple graph: Alice -> Bob
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();

    // Pattern: node("person") -edge()-> node("friend")
    let pattern_args = vec![
        create_pattern_node("person", None),
        create_pattern_edge(None, None),
        create_pattern_node("friend", None),
    ];

    // Execute pattern matching
    let results = graph.match_pattern(pattern_args).unwrap();

    // Should find one match: person=Alice, friend=Bob
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].get("person").unwrap(), "Alice");
    assert_eq!(results[0].get("friend").unwrap(), "Bob");
}

#[test]
fn test_pattern_with_edge_type_filter() {
    // Build graph: Alice -FRIEND-> Bob, Alice -FOLLOWS-> Charlie
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
    graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("Alice", "Charlie", "FOLLOWS".to_string(), None, HashMap::new()).unwrap();

    // Pattern: node("person") -edge(type: "FRIEND")-> node("friend")
    let pattern_args = vec![
        create_pattern_node("person", None),
        create_pattern_edge(Some("FRIEND"), None),
        create_pattern_node("friend", None),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should only match the FRIEND edge, not FOLLOWS
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].get("person").unwrap(), "Alice");
    assert_eq!(results[0].get("friend").unwrap(), "Bob");
}

#[test]
fn test_pattern_with_node_type_filter() {
    // Build graph with typed nodes
    let mut graph = Graph::new(GraphType::Directed);

    // Add nodes and set their types
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.set_node_type("Alice", "User".to_string()).unwrap();

    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
    graph.set_node_type("Bob", "User".to_string()).unwrap();

    graph.add_node("System".to_string(), Value::number(3.0)).unwrap();
    graph.set_node_type("System", "System".to_string()).unwrap();

    graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("Alice", "System", "USES".to_string(), None, HashMap::new()).unwrap();

    // Pattern: node("person", type: "User") -edge()-> node("friend", type: "User")
    let pattern_args = vec![
        create_pattern_node("person", Some("User")),
        create_pattern_edge(None, None),
        create_pattern_node("friend", Some("User")),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should only match Alice -> Bob (both User type), not Alice -> System
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].get("person").unwrap(), "Alice");
    assert_eq!(results[0].get("friend").unwrap(), "Bob");
}

#[test]
fn test_pattern_multiple_matches() {
    // Build graph: Alice -> Bob, Charlie -> Dave
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("Dave".to_string(), Value::number(4.0)).unwrap();
    graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("Charlie", "Dave", "FRIEND".to_string(), None, HashMap::new()).unwrap();

    // Pattern: node("a") -edge(type: "FRIEND")-> node("b")
    let pattern_args = vec![
        create_pattern_node("a", None),
        create_pattern_edge(Some("FRIEND"), None),
        create_pattern_node("b", None),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should find two matches
    assert_eq!(results.len(), 2);

    // Check both matches exist (order may vary)
    let has_alice_bob = results.iter().any(|r|
        r.get("a").unwrap() == "Alice" && r.get("b").unwrap() == "Bob"
    );
    let has_charlie_dave = results.iter().any(|r|
        r.get("a").unwrap() == "Charlie" && r.get("b").unwrap() == "Dave"
    );

    assert!(has_alice_bob, "Should find Alice -> Bob match");
    assert!(has_charlie_dave, "Should find Charlie -> Dave match");
}

#[test]
fn test_pattern_no_matches() {
    // Build graph with no edges
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();

    // Pattern: node("a") -edge()-> node("b")
    let pattern_args = vec![
        create_pattern_node("a", None),
        create_pattern_edge(None, None),
        create_pattern_node("b", None),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should find no matches
    assert_eq!(results.len(), 0);
}

#[test]
fn test_pattern_empty_graph() {
    // Empty graph
    let graph = Graph::new(GraphType::Directed);

    // Pattern: node("a") -edge()-> node("b")
    let pattern_args = vec![
        create_pattern_node("a", None),
        create_pattern_edge(None, None),
        create_pattern_node("b", None),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should find no matches
    assert_eq!(results.len(), 0);
}

#[test]
fn test_pattern_single_node() {
    // Build graph with nodes
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();

    // Pattern: just node("person") - single node pattern
    let pattern_args = vec![
        create_pattern_node("person", None),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should match all nodes
    assert_eq!(results.len(), 2);

    let has_alice = results.iter().any(|r| r.get("person").unwrap() == "Alice");
    let has_bob = results.iter().any(|r| r.get("person").unwrap() == "Bob");

    assert!(has_alice, "Should match Alice");
    assert!(has_bob, "Should match Bob");
}

#[test]
fn test_pattern_bidirectional_edge() {
    // Build graph: Alice <-> Bob (bidirectional)
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("Bob", "Alice", "FRIEND".to_string(), None, HashMap::new()).unwrap();

    // Pattern: node("a") -edge(direction: :both)-> node("b")
    let pattern_args = vec![
        create_pattern_node("a", None),
        create_pattern_edge(None, Some("both")),
        create_pattern_node("b", None),
    ];

    let results = graph.match_pattern(pattern_args).unwrap();

    // Should match edges in both directions
    assert!(results.len() >= 1, "Should find at least one bidirectional match");
}
