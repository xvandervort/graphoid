use graphoid::values::Value;

/// Helper function to create a pattern node value
#[allow(dead_code)]
fn create_pattern_node(variable: &str, node_type: Option<&str>) -> Value {
    Value::pattern_node(
        Some(variable.to_string()),
        node_type.map(|s| s.to_string())
    )
}

/// Helper function to create a pattern edge value
#[allow(dead_code)]
fn create_pattern_edge(edge_type: Option<&str>, direction: Option<&str>) -> Value {
    Value::pattern_edge(
        edge_type.map(|s| s.to_string()),
        direction.unwrap_or("outgoing").to_string()
    )
}

// DEPRECATED: Old API tests - functionality now in unified select/return method
// // DEPRECATED: Old API tests - functionality now in unified select/return method
// // DEPRECATED: Old API tests - functionality now in unified select/return method
// // DEPRECATED: Old API tests - functionality now in unified select/return method
// // DEPRECATED: Old API tests - functionality now in unified select/return method
// // DEPRECATED: Old API tests - functionality now in unified select/return method
// #[test]
// // // // // // fn test_simple_two_node_pattern() {
// // // // // //     // Build a simple graph: Alice -> Bob
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("person") -edge()-> node("friend")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("person", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("friend", None),
// // // // // //     ];
// // // // // //
// // // // // //     // Execute pattern matching
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find one match: person=Alice, friend=Bob
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("person").unwrap(), "Alice");
// // // // // //     assert_eq!(results[0].get("friend").unwrap(), "Bob");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_with_edge_type_filter() {
// // // // // //     // Build graph: Alice -FRIEND-> Bob, Alice -FOLLOWS-> Charlie
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Alice", "Charlie", "FOLLOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("person") -edge(type: "FRIEND")-> node("friend")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("person", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("friend", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should only match the FRIEND edge, not FOLLOWS
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("person").unwrap(), "Alice");
// // // // // //     assert_eq!(results[0].get("friend").unwrap(), "Bob");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_with_node_type_filter() {
// // // // // //     // Build graph with typed nodes
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //
// // // // // //     // Add nodes and set their types
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.set_node_type("Alice", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.set_node_type("Bob", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("System".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.set_node_type("System", "System".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Alice", "System", "USES".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("person", type: "User") -edge()-> node("friend", type: "User")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("person", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("friend", Some("User")),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should only match Alice -> Bob (both User type), not Alice -> System
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("person").unwrap(), "Alice");
// // // // // //     assert_eq!(results[0].get("friend").unwrap(), "Bob");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_multiple_matches() {
// // // // // //     // Build graph: Alice -> Bob, Charlie -> Dave
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("Dave".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Charlie", "Dave", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("a") -edge(type: "FRIEND")-> node("b")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("b", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find two matches
// // // // // //     assert_eq!(results.len(), 2);
// // // // // //
// // // // // //     // Check both matches exist (order may vary)
// // // // // //     let has_alice_bob = results.iter().any(|r|
// // // // // //         r.get("a").unwrap() == "Alice" && r.get("b").unwrap() == "Bob"
// // // // // //     );
// // // // // //     let has_charlie_dave = results.iter().any(|r|
// // // // // //         r.get("a").unwrap() == "Charlie" && r.get("b").unwrap() == "Dave"
// // // // // //     );
// // // // // //
// // // // // //     assert!(has_alice_bob, "Should find Alice -> Bob match");
// // // // // //     assert!(has_charlie_dave, "Should find Charlie -> Dave match");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_no_matches() {
// // // // // //     // Build graph with no edges
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //
// // // // // //     // Pattern: node("a") -edge()-> node("b")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("b", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_empty_graph() {
// // // // // //     // Empty graph
// // // // // //     let graph = Graph::new(GraphType::Directed);
// // // // // //
// // // // // //     // Pattern: node("a") -edge()-> node("b")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("b", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_single_node() {
// // // // // //     // Build graph with nodes
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //
// // // // // //     // Pattern: just node("person") - single node pattern
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("person", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should match all nodes
// // // // // //     assert_eq!(results.len(), 2);
// // // // // //
// // // // // //     let has_alice = results.iter().any(|r| r.get("person").unwrap() == "Alice");
// // // // // //     let has_bob = results.iter().any(|r| r.get("person").unwrap() == "Bob");
// // // // // //
// // // // // //     assert!(has_alice, "Should match Alice");
// // // // // //     assert!(has_bob, "Should match Bob");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_bidirectional_edge() {
// // // // // //     // Build graph: Alice <-> Bob (bidirectional)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Bob", "Alice", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("a") -edge(direction: :both)-> node("b")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(None, Some("both")),
// // // // // //         create_pattern_node("b", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should match edges in both directions
// // // // // //     assert!(results.len() >= 1, "Should find at least one bidirectional match");
// // // // // // }
// // // // // //
// // // // // // // ============================================================================
// // // // // // // Day 2: Complex Multi-Node Patterns
// // // // // // // ============================================================================
// // // // // //
// // // // // // #[test]
// // // // // // fn test_three_node_chain_pattern() {
// // // // // //     // Build graph: Alice -> Bob -> Charlie
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Bob", "Charlie", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("a") -edge()-> node("b") -edge()-> node("c")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("b", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("c", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find one complete chain: a=Alice, b=Bob, c=Charlie
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("a").unwrap(), "Alice");
// // // // // //     assert_eq!(results[0].get("b").unwrap(), "Bob");
// // // // // //     assert_eq!(results[0].get("c").unwrap(), "Charlie");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_three_node_chain_with_type_filter() {
// // // // // //     // Build graph with typed nodes
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.set_node_type("Alice", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.set_node_type("Bob", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("System".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.set_node_type("System", "System".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Bob", "System", "USES".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: User -> User -> User (should NOT match because System is not User)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("b", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("c", Some("User")),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find zero matches
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_four_node_chain_pattern() {
// // // // // //     // Build graph: A -> B -> C -> D
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "D", "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node -> node -> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("n1", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n2", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n3", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n4", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find one complete chain
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("n1").unwrap(), "A");
// // // // // //     assert_eq!(results[0].get("n2").unwrap(), "B");
// // // // // //     assert_eq!(results[0].get("n3").unwrap(), "C");
// // // // // //     assert_eq!(results[0].get("n4").unwrap(), "D");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_multiple_chain_matches() {
// // // // // //     // Build graph with multiple parallel chains:
// // // // // //     // A -> B -> C
// // // // // //     // D -> E -> F
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.add_node("E".to_string(), Value::number(5.0)).unwrap();
// // // // // //     graph.add_node("F".to_string(), Value::number(6.0)).unwrap();
// // // // // //
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("D", "E", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("E", "F", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node -> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find two chains: A->B->C and D->E->F
// // // // // //     assert_eq!(results.len(), 2);
// // // // // //
// // // // // //     let has_abc = results.iter().any(|r|
// // // // // //         r.get("x").unwrap() == "A" && r.get("y").unwrap() == "B" && r.get("z").unwrap() == "C"
// // // // // //     );
// // // // // //     let has_def = results.iter().any(|r|
// // // // // //         r.get("x").unwrap() == "D" && r.get("y").unwrap() == "E" && r.get("z").unwrap() == "F"
// // // // // //     );
// // // // // //
// // // // // //     assert!(has_abc, "Should find A->B->C chain");
// // // // // //     assert!(has_def, "Should find D->E->F chain");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_branching_graph_pattern() {
// // // // // //     // Build graph with branching:
// // // // // //     //     B
// // // // // //     //   /   \
// // // // // //     //  A     D
// // // // // //     //   \   /
// // // // // //     //     C
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("A", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "D", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "D", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: A -> X -> D (should find both A->B->D and A->C->D)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("mid", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find multiple paths from A to D
// // // // // //     assert!(results.len() >= 2, "Should find at least 2 paths through the diamond");
// // // // // //
// // // // // //     // All paths should start from A
// // // // // //     for result in &results {
// // // // // //         if result.get("start").unwrap() == "A" && result.get("end").unwrap() == "D" {
// // // // // //             // Valid path found
// // // // // //             assert!(result.get("mid").unwrap() == "B" || result.get("mid").unwrap() == "C");
// // // // // //         }
// // // // // //     }
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_mixed_edge_types_in_chain() {
// // // // // //     // Build graph: Alice -FRIEND-> Bob -FOLLOWS-> Charlie
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Bob", "Charlie", "FOLLOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -FRIEND-> node -FOLLOWS-> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("b", None),
// // // // // //         create_pattern_edge(Some("FOLLOWS"), None),
// // // // // //         create_pattern_node("c", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find exactly one match
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("a").unwrap(), "Alice");
// // // // // //     assert_eq!(results[0].get("b").unwrap(), "Bob");
// // // // // //     assert_eq!(results[0].get("c").unwrap(), "Charlie");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_incomplete_chain_no_match() {
// // // // // //     // Build graph: A -> B (incomplete chain, no C)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: A -> B -> C (requires 3 nodes)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches (chain incomplete)
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_cycle_detection_in_pattern() {
// // // // // //     // Build graph with a cycle: A -> B -> C -> A
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "A", "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node -> node (linear chain)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find 3 matches: A->B->C, B->C->A, C->A->B
// // // // // //     assert_eq!(results.len(), 3);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_long_chain_five_nodes() {
// // // // // //     // Build graph: 1 -> 2 -> 3 -> 4 -> 5
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     for i in 1..=5 {
// // // // // //         graph.add_node(i.to_string(), Value::number(i as f64)).unwrap();
// // // // // //     }
// // // // // //     for i in 1..5 {
// // // // // //         graph.add_edge(&i.to_string(), &(i+1).to_string(), "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     }
// // // // // //
// // // // // //     // Pattern: 5-node chain
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("n1", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n2", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n3", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n4", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n5", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find exactly one 5-node chain
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("n1").unwrap(), "1");
// // // // // //     assert_eq!(results[0].get("n2").unwrap(), "2");
// // // // // //     assert_eq!(results[0].get("n3").unwrap(), "3");
// // // // // //     assert_eq!(results[0].get("n4").unwrap(), "4");
// // // // // //     assert_eq!(results[0].get("n5").unwrap(), "5");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_overlapping_chains() {
// // // // // //     // Build graph where chains share nodes:
// // // // // //     // A -> B -> C
// // // // // //     // B -> D -> E
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.add_node("E".to_string(), Value::number(5.0)).unwrap();
// // // // // //
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "D", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("D", "E", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node -> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find: A->B->C, A->B->D, B->D->E
// // // // // //     assert_eq!(results.len(), 3);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_complex_type_filtering_across_chain() {
// // // // // //     // Build typed chain: User -> System -> User -> System
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //
// // // // // //     graph.add_node("U1".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.set_node_type("U1", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("S1".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.set_node_type("S1", "System".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("U2".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.set_node_type("U2", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("S2".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.set_node_type("S2", "System".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_edge("U1", "S1", "USES".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("S1", "U2", "SERVES".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("U2", "S2", "USES".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: User -> System -> User -> System
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("user1", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("sys1", Some("System")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("user2", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("sys2", Some("System")),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find exactly one match with correct type sequence
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("user1").unwrap(), "U1");
// // // // // //     assert_eq!(results[0].get("sys1").unwrap(), "S1");
// // // // // //     assert_eq!(results[0].get("user2").unwrap(), "U2");
// // // // // //     assert_eq!(results[0].get("sys2").unwrap(), "S2");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_edge_type_mismatch_in_chain() {
// // // // // //     // Build graph: A -FRIEND-> B -ENEMY-> C
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "ENEMY".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -FRIEND-> node -FRIEND-> node (requires both FRIEND)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches (second edge is ENEMY, not FRIEND)
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_self_loop_pattern() {
// // // // // //     // Build graph with self-loop: A -> A
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_edge("A", "A", "SELF".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node (should match A -> A)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find one match where start=end=A
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("start").unwrap(), "A");
// // // // // //     assert_eq!(results[0].get("end").unwrap(), "A");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_star_pattern_one_center_many_spokes() {
// // // // // //     // Build star graph: B -> A, C -> A, D -> A (A is the center)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //
// // // // // //     graph.add_edge("B", "A", "POINTS_TO".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "A", "POINTS_TO".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("D", "A", "POINTS_TO".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node (should match all spokes)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("spoke", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("center", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find 3 matches: B->A, C->A, D->A
// // // // // //     assert_eq!(results.len(), 3);
// // // // // //
// // // // // //     // All should point to A as center
// // // // // //     for result in &results {
// // // // // //         assert_eq!(result.get("center").unwrap(), "A");
// // // // // //     }
// // // // // // }
// // // // // //
// // // // // // // ============================================================================
// // // // // // // Day 3: Edge Cases and Advanced Type Matching
// // // // // // // ============================================================================
// // // // // //
// // // // // // #[test]
// // // // // // fn test_direction_outgoing_explicit() {
// // // // // //     // Build directed graph: A -> B
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern with explicit outgoing direction (default)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("from", None),
// // // // // //         create_pattern_edge(None, Some("outgoing")),
// // // // // //         create_pattern_node("to", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find A -> B
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("from").unwrap(), "A");
// // // // // //     assert_eq!(results[0].get("to").unwrap(), "B");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_direction_incoming_simple() {
// // // // // //     // Build directed graph: A -> B
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern with incoming direction: start at B, follow incoming edge to A
// // // // // //     // This pattern says: find node x with incoming edge from node y
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("target", None),
// // // // // //         create_pattern_edge(None, Some("incoming")),
// // // // // //         create_pattern_node("source", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find B <- A (which is the reverse view of A -> B)
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("target").unwrap(), "B");
// // // // // //     assert_eq!(results[0].get("source").unwrap(), "A");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_direction_incoming_chain() {
// // // // // //     // Build directed graph: A -> B -> C
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: C <- B <- A (following edges backward)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("end", None),
// // // // // //         create_pattern_edge(None, Some("incoming")),
// // // // // //         create_pattern_node("mid", None),
// // // // // //         create_pattern_edge(None, Some("incoming")),
// // // // // //         create_pattern_node("start", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find the chain traversed backward
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("end").unwrap(), "C");
// // // // // //     assert_eq!(results[0].get("mid").unwrap(), "B");
// // // // // //     assert_eq!(results[0].get("start").unwrap(), "A");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_direction_incoming_with_type_filter() {
// // // // // //     // Build directed graph with types: Person(Alice) -> Person(Bob)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.set_node_type("Alice", "Person".to_string()).unwrap();
// // // // // //     graph.set_node_type("Bob", "Person".to_string()).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "KNOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: find Person <- Person via KNOWS edge (backward)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("target", Some("Person")),
// // // // // //         create_pattern_edge(Some("KNOWS"), Some("incoming")),
// // // // // //         create_pattern_node("source", Some("Person")),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find Bob <- Alice
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("target").unwrap(), "Bob");
// // // // // //     assert_eq!(results[0].get("source").unwrap(), "Alice");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_direction_incoming_no_match() {
// // // // // //     // Build graph with only isolated nodes (no edges at all)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     // No edges added - isolated nodes only
// // // // // //
// // // // // //     // Pattern: look for nodes with incoming edges
// // // // // //     // Since no edges exist, no node has predecessors
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("node", None),
// // // // // //         create_pattern_edge(None, Some("incoming")),
// // // // // //         create_pattern_node("predecessor", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // No nodes have incoming edges, so should find nothing
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_disconnected_graph_components() {
// // // // // //     // Build graph with disconnected components:
// // // // // //     // Component 1: A -> B
// // // // // //     // Component 2: C -> D (isolated)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "D", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find both A->B and C->D
// // // // // //     assert_eq!(results.len(), 2);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_isolated_nodes_no_match() {
// // // // // //     // Build graph with isolated nodes: A, B, C (no edges)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //
// // // // // //     // Pattern: node -> node (requires edge)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches (no edges)
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_single_node_in_graph_with_edges() {
// // // // // //     // Build graph: A -> B, C (isolated)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: single node (should match all)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("n", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should match all three nodes
// // // // // //     assert_eq!(results.len(), 3);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_type_constraint_with_missing_types() {
// // // // // //     // Build graph where some nodes have types, some don't
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.set_node_type("A", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     // B has no type
// // // // // //
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: User -> User
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("u1", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("u2", Some("User")),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches (B has no type)
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_type_constraint_no_constraint() {
// // // // // //     // Build typed graph
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.set_node_type("A", "User".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.set_node_type("B", "System".to_string()).unwrap();
// // // // // //
// // // // // //     graph.add_edge("A", "B", "USES".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: any node -> any node (no type constraints)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should match regardless of types
// // // // // //     assert_eq!(results.len(), 1);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_large_graph_many_matches() {
// // // // // //     // Build larger graph: 10 nodes in a chain
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     for i in 0..10 {
// // // // // //         graph.add_node(i.to_string(), Value::number(i as f64)).unwrap();
// // // // // //     }
// // // // // //     for i in 0..9 {
// // // // // //         graph.add_edge(&i.to_string(), &(i+1).to_string(), "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     }
// // // // // //
// // // // // //     // Pattern: node -> node -> node (3-node chain)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("b", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("c", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find 8 overlapping 3-node chains: 0-1-2, 1-2-3, ..., 7-8-9
// // // // // //     assert_eq!(results.len(), 8);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_dense_graph_many_connections() {
// // // // // //     // Build dense graph: every node connects to every other
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     for i in 1..=4 {
// // // // // //         graph.add_node(i.to_string(), Value::number(i as f64)).unwrap();
// // // // // //     }
// // // // // //     // Connect every node to every other node
// // // // // //     for i in 1..=4 {
// // // // // //         for j in 1..=4 {
// // // // // //             if i != j {
// // // // // //                 graph.add_edge(&i.to_string(), &j.to_string(), "CONN".to_string(), None, HashMap::new()).unwrap();
// // // // // //             }
// // // // // //         }
// // // // // //     }
// // // // // //
// // // // // //     // Pattern: node -> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find 4*3 = 12 connections (every pair)
// // // // // //     assert_eq!(results.len(), 12);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_with_duplicate_variable_names() {
// // // // // //     // Build graph: A -> B
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("x") -> node("x") - same variable twice
// // // // // //     // This should match A -> A (self-loop) but we don't have one
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("x", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches (no self-loops)
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_multiple_edges_same_nodes_different_types() {
// // // // // //     // Build graph: A -FRIEND-> B, A -COLLEAGUE-> B (multiple edges)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     // Note: Current graph implementation may not support multiple edges with different types
// // // // // //     // This test documents expected behavior
// // // // // //
// // // // // //     // Pattern: A -FRIEND-> B
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find one match
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("x").unwrap(), "A");
// // // // // //     assert_eq!(results[0].get("y").unwrap(), "B");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_very_long_chain_pattern() {
// // // // // //     // Build 7-node chain
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     for i in 1..=7 {
// // // // // //         graph.add_node(i.to_string(), Value::number(i as f64)).unwrap();
// // // // // //     }
// // // // // //     for i in 1..7 {
// // // // // //         graph.add_edge(&i.to_string(), &(i+1).to_string(), "NEXT".to_string(), None, HashMap::new()).unwrap();
// // // // // //     }
// // // // // //
// // // // // //     // Pattern: 7-node chain
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("n1", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n2", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n3", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n4", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n5", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n6", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("n7", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find exactly one 7-node chain
// // // // // //     assert_eq!(results.len(), 1);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_alternating_types_long_chain() {
// // // // // //     // Build chain with alternating types: U-S-U-S-U-S
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     for i in 1..=6 {
// // // // // //         graph.add_node(i.to_string(), Value::number(i as f64)).unwrap();
// // // // // //         let node_type = if i % 2 == 1 { "User" } else { "System" };
// // // // // //         graph.set_node_type(&i.to_string(), node_type.to_string()).unwrap();
// // // // // //     }
// // // // // //     for i in 1..6 {
// // // // // //         graph.add_edge(&i.to_string(), &(i+1).to_string(), "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     }
// // // // // //
// // // // // //     // Pattern: User -> System -> User -> System
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("u1", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("s1", Some("System")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("u2", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("s2", Some("System")),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find 3 matches: 1-2-3-4, 3-4-5-6 (but 3 can't start both, so just 1)
// // // // // //     // Actually should find 2: starting at 1 (1-2-3-4) and starting at 3 (3-4-5-6)
// // // // // //     assert!(results.len() >= 1, "Should find at least one alternating type chain");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_pattern_all_same_edge_type() {
// // // // // //     // Build graph with mixed edge types
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //
// // // // // //     graph.add_edge("A", "B", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "D", "ENEMY".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: all FRIEND edges (3 nodes)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(Some("FRIEND"), None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find A-B-C only (C-D has ENEMY edge)
// // // // // //     assert_eq!(results.len(), 1);
// // // // // //     assert_eq!(results[0].get("x").unwrap(), "A");
// // // // // //     assert_eq!(results[0].get("y").unwrap(), "B");
// // // // // //     assert_eq!(results[0].get("z").unwrap(), "C");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_empty_variable_name_error() {
// // // // // //     // This test documents that pattern nodes need variable names
// // // // // //     // The current implementation requires variables
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //
// // // // // //     // Pattern with valid variable
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //     assert_eq!(results.len(), 1);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_complex_diamond_all_paths() {
// // // // // //     // Build double diamond:
// // // // // //     //      B   D
// // // // // //     //     / \ / \
// // // // // //     //    A   C   E
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     for node in ["A", "B", "C", "D", "E"] {
// // // // // //         graph.add_node(node.to_string(), Value::number(1.0)).unwrap();
// // // // // //     }
// // // // // //     graph.add_edge("A", "B", "L".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("A", "C", "L".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "L".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "D", "L".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "E", "L".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("D", "E", "L".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: 3-node chain
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("z", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find all valid 3-node paths
// // // // // //     // A-B-C, A-C-D, A-C-E, C-D-E
// // // // // //     assert!(results.len() >= 4, "Should find at least 4 paths in double diamond");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_type_filter_with_no_matching_nodes() {
// // // // // //     // Build graph with no User-typed nodes
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.set_node_type("A", "System".to_string()).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.set_node_type("B", "System".to_string()).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: User -> any (but no User nodes exist)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("u", Some("User")),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("x", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_edge_type_filter_with_no_matching_edges() {
// // // // // //     // Build graph with only FRIEND edges
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: any -ENEMY-> any (but no ENEMY edges exist)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(Some("ENEMY"), None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no matches
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // // ============================================================================
// // // // // // // Day 4: Where Clause Filtering
// // // // // // // ============================================================================
// // // // // //
// // // // // // #[test]
// // // // // // fn test_where_filter_numeric_value_greater_than() {
// // // // // //     // Build graph: Person(Alice, age=25) -> Person(Bob, age=30)
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(25.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(30.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(17.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "KNOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Alice", "Charlie", "KNOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: find all KNOWS relationships
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("person", None),
// // // // // //         create_pattern_edge(Some("KNOWS"), None),
// // // // // //         create_pattern_node("friend", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Filter: friend.value > 18 (only Bob passes, Charlie doesn't)
// // // // // //     let filtered = results.where_node_value("friend", |value| {
// // // // // //         if let Some(n) = value.to_number() {
// // // // // //             n > 18.0
// // // // // //         } else {
// // // // // //             false
// // // // // //         }
// // // // // //     }).unwrap();
// // // // // //
// // // // // //     assert_eq!(filtered.len(), 1);
// // // // // //     assert_eq!(filtered.get(0).unwrap().get("person").unwrap(), "Alice");
// // // // // //     assert_eq!(filtered.get(0).unwrap().get("friend").unwrap(), "Bob");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_where_filter_multiple_nodes() {
// // // // // //     // Build graph with ages
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(25.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(30.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(15.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "KNOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Charlie", "Bob", "KNOWS".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("a", None),
// // // // // //         create_pattern_edge(Some("KNOWS"), None),
// // // // // //         create_pattern_node("b", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Filter: both a and b must be > 18
// // // // // //     let filtered = results.where_both_nodes("a", "b", |a_val, b_val| {
// // // // // //         let a_num = a_val.to_number().unwrap_or(0.0);
// // // // // //         let b_num = b_val.to_number().unwrap_or(0.0);
// // // // // //         a_num > 18.0 && b_num > 18.0
// // // // // //     }).unwrap();
// // // // // //
// // // // // //     // Only Alice-Bob should pass (Charlie is 15)
// // // // // //     assert_eq!(filtered.len(), 1);
// // // // // //     assert_eq!(filtered.get(0).unwrap().get("a").unwrap(), "Alice");
// // // // // //     assert_eq!(filtered.get(0).unwrap().get("b").unwrap(), "Bob");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_where_filter_all_pass() {
// // // // // //     // All matches pass the filter
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(100.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(200.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(300.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Filter: all values > 50 (all should pass)
// // // // // //     let filtered = results.where_node_value("x", |value| {
// // // // // //         value.to_number().unwrap_or(0.0) > 50.0
// // // // // //     }).unwrap();
// // // // // //
// // // // // //     assert_eq!(filtered.len(), 2); // A->B and B->C
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_where_filter_none_pass() {
// // // // // //     // No matches pass the filter
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(10.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(20.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Filter: value > 100 (none pass)
// // // // // //     let filtered = results.where_node_value("x", |value| {
// // // // // //         value.to_number().unwrap_or(0.0) > 100.0
// // // // // //     }).unwrap();
// // // // // //
// // // // // //     assert_eq!(filtered.len(), 0);
// // // // // // }
// // // // // //
// // // // // // // TODO: Implement test_where_filter_with_properties once set_node_properties is available
// // // // // // // #[test]
// // // // // // // fn test_where_filter_with_properties() { ... }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_where_filter_chaining() {
// // // // // //     // Test chaining multiple filters
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(100.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(200.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(50.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("A", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Chain filters: x > 50 AND y > 100
// // // // // //     let filtered = results
// // // // // //         .where_node_value("x", |v| v.to_number().unwrap_or(0.0) > 50.0).unwrap()
// // // // // //         .where_node_value("y", |v| v.to_number().unwrap_or(0.0) > 100.0).unwrap();
// // // // // //
// // // // // //     // Only A->B should pass (A->C fails because C is 50)
// // // // // //     assert_eq!(filtered.len(), 1);
// // // // // //     assert_eq!(filtered.get(0).unwrap().get("x").unwrap(), "A");
// // // // // //     assert_eq!(filtered.get(0).unwrap().get("y").unwrap(), "B");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_where_filter_empty_results() {
// // // // // //     // Filter on empty match results
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     // No edges
// // // // // //
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("x", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("y", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //     assert_eq!(results.len(), 0);
// // // // // //
// // // // // //     // Filter on empty results should return empty
// // // // // //     let filtered = results.where_node_value("x", |v| {
// // // // // //         v.to_number().unwrap_or(0.0) > 0.0
// // // // // //     }).unwrap();
// // // // // //
// // // // // //     assert_eq!(filtered.len(), 0);
// // // // // // }
// // // // // //
// // // // // // // ============================================================================
// // // // // // // Day 5 Part B: Variable-Length Paths
// // // // // // // ============================================================================
// // // // // //
// // // // // // /// Helper function to create a pattern path value
// // // // // // fn create_pattern_path(edge_type: Option<&str>, min: usize, max: usize, direction: Option<&str>) -> Value {
// // // // // //     Value::pattern_path(
// // // // // //         edge_type.unwrap_or("").to_string(),
// // // // // //         min,
// // // // // //         max,
// // // // // //         direction.unwrap_or("outgoing").to_string()
// // // // // //     )
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_single_hop() {
// // // // // //     // Build graph: A -> B -> C
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -[*1..1]-> node (exactly 1 hop, equivalent to regular edge)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(None, 1, 1, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find: A->B and B->C (2 single-hop paths)
// // // // // //     assert_eq!(results.len(), 2);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_one_to_two_hops() {
// // // // // //     // Build graph: A -> B -> C
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -[*1..2]-> node (1 or 2 hops)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(None, 1, 2, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find: A->B (1 hop), B->C (1 hop), A->C (2 hops) = 3 paths
// // // // // //     assert_eq!(results.len(), 3);
// // // // // //
// // // // // //     // Verify we have both direct and indirect paths
// // // // // //     let has_direct_ab = results.iter().any(|r|
// // // // // //         r.get("start").unwrap() == "A" && r.get("end").unwrap() == "B"
// // // // // //     );
// // // // // //     let has_direct_bc = results.iter().any(|r|
// // // // // //         r.get("start").unwrap() == "B" && r.get("end").unwrap() == "C"
// // // // // //     );
// // // // // //     let has_indirect_ac = results.iter().any(|r|
// // // // // //         r.get("start").unwrap() == "A" && r.get("end").unwrap() == "C"
// // // // // //     );
// // // // // //
// // // // // //     assert!(has_direct_ab, "Should find A->B");
// // // // // //     assert!(has_direct_bc, "Should find B->C");
// // // // // //     assert!(has_indirect_ac, "Should find A->C via B");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_exactly_two_hops() {
// // // // // //     // Build graph: A -> B -> C -> D
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_node("D".to_string(), Value::number(4.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "D", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -[*2..2]-> node (exactly 2 hops)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(None, 2, 2, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find: A->C (via B) and B->D (via C) = 2 paths
// // // // // //     assert_eq!(results.len(), 2);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_with_edge_type() {
// // // // // //     // Build graph with typed edges
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("A", "C", "ENEMY".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -[FRIEND*1..2]-> node (1-2 FRIEND edges)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(Some("FRIEND"), 1, 2, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find: A->B (FRIEND), B->C (FRIEND), A->C (via B with FRIENDs)
// // // // // //     // Should NOT find: A->C direct (it's ENEMY)
// // // // // //     assert_eq!(results.len(), 3);
// // // // // //
// // // // // //     // Verify A->C via ENEMY is not included
// // // // // //     let direct_enemy = results.iter().any(|r| {
// // // // // //         r.get("start").unwrap() == "A" &&
// // // // // //         r.get("end").unwrap() == "C" &&
// // // // // //         results.len() == 1  // If this was the only path, it would be the direct one
// // // // // //     });
// // // // // //     assert!(!direct_enemy, "Should not find direct A->C ENEMY path when filtering for FRIEND");
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_no_paths_found() {
// // // // // //     // Build disconnected graph
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     // No edges
// // // // // //
// // // // // //     // Pattern: node -[*1..3]-> node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(None, 1, 3, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find no paths
// // // // // //     assert_eq!(results.len(), 0);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_with_cycle() {
// // // // // //     // Build graph with cycle: A -> B -> C -> A
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("C", "A", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -[*1..2]-> node (max 2 hops to avoid infinite paths)
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(None, 1, 2, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find:
// // // // // //     // 1 hop: A->B, B->C, C->A (3 paths)
// // // // // //     // 2 hops: A->C (via B), B->A (via C), C->B (via A) (3 paths)
// // // // // //     // Total: 6 paths
// // // // // //     assert_eq!(results.len(), 6);
// // // // // // }
// // // // // //
// // // // // // #[test]
// // // // // // fn test_variable_path_star_syntax() {
// // // // // //     // Build graph: A -> B -> C
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("C".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("A", "B", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("B", "C", "LINK".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node -[*0..10]-> node (0-10 hops, like Cypher's *)
// // // // // //     // Note: 0 hops means match same node
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("start", None),
// // // // // //         create_pattern_path(None, 0, 10, None),
// // // // // //         create_pattern_node("end", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find:
// // // // // //     // 0 hops: A->A, B->B, C->C (3 paths)
// // // // // //     // 1 hop: A->B, B->C (2 paths)
// // // // // //     // 2 hops: A->C (1 path)
// // // // // //     // Total: 6 paths
// // // // // //     assert_eq!(results.len(), 6);
// // // // // // }
// // // // // //
// // // // // // // ============================================================================
// // // // // // // RETURN CLAUSE TESTS (Day 5 Part A)
// // // // // // // ============================================================================
// // // // // //
// // // // // // #[test]
// // // // // // #[ignore] // Old API - now using unified select/return method
// // // // // // fn test_return_vars_single_variable() {
// // // // // //     // Build graph: Alice -> Bob -> Charlie
// // // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // // //     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
// // // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //     graph.add_edge("Bob", "Charlie", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // // //
// // // // // //     // Pattern: node("person") -edge()-> node("friend")
// // // // // //     let pattern_args = vec![
// // // // // //         create_pattern_node("person", None),
// // // // // //         create_pattern_edge(None, None),
// // // // // //         create_pattern_node("friend", None),
// // // // // //     ];
// // // // // //
// // // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // // //
// // // // // //     // Should find 2 matches:
// // // // // //     // Match 1: person=Alice, friend=Bob
// // // // // //     // Match 2: person=Bob, friend=Charlie
// // // // // //     assert_eq!(results.len(), 2);
// // // // // //
// // // // // //     // Return only "person" variable
// // // // // //     let projected = results.select(vec!["person"]);
// // // // // //
// // // // // //     // Should still have 2 matches, but only "person" variable
// // // // // //     assert_eq!(projected.len(), 2);
// // // // // //
// // // // // //     // Collect person values (order may vary due to HashMap iteration)
// // // // // //     let persons: Vec<&str> = projected.iter()
// // // // // //         .map(|m| m.get("person").unwrap().as_str())
// // // // // //         .collect();
// // // // // //
// // // // // //     // Both matches should have "person" but not "friend"
// // // // // //     for binding in &projected {
// // // // // //         assert!(binding.contains_key("person"));
// // // // // //         assert!(!binding.contains_key("friend"));
// // // // // //     }
// // // // // //
// // // // // //     // Should contain both Alice and Bob
// // // // // //     assert!(persons.contains(&"Alice"));
// // // // // //     assert!(persons.contains(&"Bob"));
// // // // // // }
// // // // //
// // // // // #[test]
// // // // // #[ignore] // Old API - now using unified select/return method
// // // // // fn test_return_vars_multiple_variables() {
// // // // //     // Build graph: Alice -> Bob -> Charlie -> David
// // // // //     let mut graph = Graph::new(GraphType::Directed);
// // // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // // //     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
// // // // //     graph.add_node("David".to_string(), Value::number(4.0)).unwrap();
// // // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // //     graph.add_edge("Bob", "Charlie", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // //     graph.add_edge("Charlie", "David", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // // //
// // // // //     // Pattern: node("a") -edge()-> node("b") -edge()-> node("c")
// // // // //     let pattern_args = vec![
// // // // //         create_pattern_node("a", None),
// // // // //         create_pattern_edge(None, None),
// // // // //         create_pattern_node("b", None),
// // // // //         create_pattern_edge(None, None),
// // // // //         create_pattern_node("c", None),
// // // // //     ];
// // // // //
// // // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // // //     assert_eq!(results.len(), 2); // Alice->Bob->Charlie, Bob->Charlie->David
// // // // //
// // // // //     // Return only "a" and "c" variables (skip "b")
// // // // //     let projected = results.select(vec!["a", "c"]);
// // // // //
// // // // //     assert_eq!(projected.len(), 2);
// // // // //
// // // // //     // All matches should have "a" and "c" but not "b"
// // // // //     for binding in &projected {
// // // // //         assert!(binding.contains_key("a"));
// // // // //         assert!(!binding.contains_key("b"));
// // // // //         assert!(binding.contains_key("c"));
// // // // //     }
// // // // //
// // // // //     // Collect the (a, c) pairs (order may vary)
// // // // //     let mut pairs: Vec<(&str, &str)> = projected.iter()
// // // // //         .map(|m| (m.get("a").unwrap().as_str(), m.get("c").unwrap().as_str()))
// // // // //         .collect();
// // // // //     pairs.sort();
// // // // //
// // // // //     // Should contain: (Alice, Charlie) and (Bob, David)
// // // // //     assert_eq!(pairs, vec![("Alice", "Charlie"), ("Bob", "David")]);
// // // // // }
// // // //
// // // // #[test]
// // // // #[ignore] // Old API - now using unified select/return method
// // // // fn test_return_vars_all_variables() {
// // // //     // Build graph: Alice -> Bob
// // // //     let mut graph = Graph::new(GraphType::Directed);
// // // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // // //
// // // //     // Pattern: node("person") -edge()-> node("friend")
// // // //     let pattern_args = vec![
// // // //         create_pattern_node("person", None),
// // // //         create_pattern_edge(None, None),
// // // //         create_pattern_node("friend", None),
// // // //     ];
// // // //
// // // //     let results = graph.match_pattern(pattern_args).unwrap();
// // // //
// // // //     // Return all variables (should be same as original)
// // // //     let projected = results.select(vec!["person", "friend"]);
// // // //
// // // //     assert_eq!(projected.len(), 1);
// // // //     assert!(projected[0].contains_key("person"));
// // // //     assert!(projected[0].contains_key("friend"));
// // // //     assert_eq!(projected[0].get("person").unwrap(), "Alice");
// // // //     assert_eq!(projected[0].get("friend").unwrap(), "Bob");
// // // // }
// // //
// // // #[test]
// // // #[ignore] // Old API - now using unified select/return method
// // // fn test_return_vars_nonexistent_variable() {
// // //     // Build graph: Alice -> Bob
// // //     let mut graph = Graph::new(GraphType::Directed);
// // //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// // //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// // //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// // //
// // //     // Pattern: node("person") -edge()-> node("friend")
// // //     let pattern_args = vec![
// // //         create_pattern_node("person", None),
// // //         create_pattern_edge(None, None),
// // //         create_pattern_node("friend", None),
// // //     ];
// // //
// // //     let results = graph.match_pattern(pattern_args).unwrap();
// // //
// // //     // Return a variable that doesn't exist (should be silently ignored)
// // //     let projected = results.select(vec!["person", "nonexistent"]);
// // //
// // //     assert_eq!(projected.len(), 1);
// // //     assert!(projected[0].contains_key("person"));
// // //     assert!(!projected[0].contains_key("nonexistent"));
// // //     assert!(!projected[0].contains_key("friend"));
// // // }
// //
// // #[test]
// // #[ignore] // Old API - now using unified select/return method
// // fn test_return_properties_simple() {
// //     // Build graph with properties: Alice(age:30) -> Bob(age:25)
// //     let mut graph = Graph::new(GraphType::Directed);
// //
// //     let mut alice_props = HashMap::new();
// //     alice_props.insert("age".to_string(), Value::number(30.0));
// //     alice_props.insert("name".to_string(), Value::string("Alice".to_string()));
// //     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
// //     graph.set_node_properties("Alice", alice_props).unwrap();
// //
// //     let mut bob_props = HashMap::new();
// //     bob_props.insert("age".to_string(), Value::number(25.0));
// //     bob_props.insert("name".to_string(), Value::string("Bob".to_string()));
// //     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
// //     graph.set_node_properties("Bob", bob_props).unwrap();
// //
// //     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
// //
// //     // Pattern: node("person") -edge()-> node("friend")
// //     let pattern_args = vec![
// //         create_pattern_node("person", None),
// //         create_pattern_edge(None, None),
// //         create_pattern_node("friend", None),
// //     ];
// //
// //     let results = graph.match_pattern(pattern_args).unwrap();
// //
// //     // Return specific properties: person.name and friend.age
// //     let projected = results.select(vec!["person.name", "friend.age"]);
// //
// //     // Should get a vector of HashMaps with requested properties
// //     assert_eq!(projected.len(), 1);
// //
// //     // First match should have person.name = "Alice" and friend.age = 25.0
// //     assert_eq!(projected[0].get("person.name").unwrap(), &Value::string("Alice".to_string()));
// //     assert_eq!(projected[0].get("friend.age").unwrap(), &Value::number(25.0));
// //     assert!(!projected[0].contains_key("person.age"));
// //     assert!(!projected[0].contains_key("friend.name"));
// // }
//
// #[test]
// #[ignore] // Old API - now using unified select/return method
// fn test_return_properties_multiple_matches() {
//     // Build graph: Alice(age:30) -> Bob(age:25) -> Charlie(age:35)
//     let mut graph = Graph::new(GraphType::Directed);
//
//     let mut alice_props = HashMap::new();
//     alice_props.insert("age".to_string(), Value::number(30.0));
//     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
//     graph.set_node_properties("Alice", alice_props).unwrap();
//
//     let mut bob_props = HashMap::new();
//     bob_props.insert("age".to_string(), Value::number(25.0));
//     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
//     graph.set_node_properties("Bob", bob_props).unwrap();
//
//     let mut charlie_props = HashMap::new();
//     charlie_props.insert("age".to_string(), Value::number(35.0));
//     graph.add_node("Charlie".to_string(), Value::number(3.0)).unwrap();
//     graph.set_node_properties("Charlie", charlie_props).unwrap();
//
//     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
//     graph.add_edge("Bob", "Charlie", "FRIEND".to_string(), None, HashMap::new()).unwrap();
//
//     // Pattern: node("person") -edge()-> node("friend")
//     let pattern_args = vec![
//         create_pattern_node("person", None),
//         create_pattern_edge(None, None),
//         create_pattern_node("friend", None),
//     ];
//
//     let results = graph.match_pattern(pattern_args).unwrap();
//     assert_eq!(results.len(), 2);
//
//     // Return person.age for all matches
//     let projected = results.select(vec!["person.age"]);
//
//     assert_eq!(projected.len(), 2);
//
//     // Collect ages (order may vary due to HashMap iteration)
//     let ages: Vec<f64> = projected.iter()
//         .map(|m| {
//             if let ValueKind::Number(n) = m.get("person.age").unwrap().kind {
//                 n
//             } else {
//                 panic!("Expected number");
//             }
//         })
//         .collect();
//
//     // Should contain both Alice's age (30) and Bob's age (25)
//     assert!(ages.contains(&30.0));
//     assert!(ages.contains(&25.0));
// }
//
// #[test]
// #[ignore] // Old API - now using unified select/return method
// fn test_return_properties_nonexistent_property() {
//     // Build graph: Alice(age:30) -> Bob(age:25)
//     let mut graph = Graph::new(GraphType::Directed);
//
//     let mut alice_props = HashMap::new();
//     alice_props.insert("age".to_string(), Value::number(30.0));
//     graph.add_node("Alice".to_string(), Value::number(1.0)).unwrap();
//     graph.set_node_properties("Alice", alice_props).unwrap();
//
//     let mut bob_props = HashMap::new();
//     bob_props.insert("age".to_string(), Value::number(25.0));
//     graph.add_node("Bob".to_string(), Value::number(2.0)).unwrap();
//     graph.set_node_properties("Bob", bob_props).unwrap();
//
//     graph.add_edge("Alice", "Bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
//
//     // Pattern: node("person") -edge()-> node("friend")
//     let pattern_args = vec![
//         create_pattern_node("person", None),
//         create_pattern_edge(None, None),
//         create_pattern_node("friend", None),
//     ];
//
//     let results = graph.match_pattern(pattern_args).unwrap();
//
//     // Return a property that doesn't exist (should use None/null)
//     let projected = results.select(vec!["person.nonexistent"]);
//
//     assert_eq!(projected.len(), 1);
//     assert_eq!(projected[0].get("person.nonexistent").unwrap(), &Value::none());
// }
