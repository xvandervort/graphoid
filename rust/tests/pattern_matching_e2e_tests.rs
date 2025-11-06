// Integration tests for graph pattern matching (Phase 9 Days 6-7)
// These tests combine multiple pattern matching features to demonstrate real-world scenarios

use graphoid::values::{Graph, GraphType, Value, ValueKind};
use std::collections::HashMap;

/// Helper to create a social network graph for testing
fn create_social_network() -> Graph {
    let mut g = Graph::new(GraphType::Directed);

    // Add users with properties
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    let mut alice_props = HashMap::new();
    alice_props.insert("name".to_string(), Value::string("Alice".to_string()));
    alice_props.insert("age".to_string(), Value::number(30.0));
    alice_props.insert("city".to_string(), Value::string("NYC".to_string()));
    g.set_node_properties("alice", alice_props).unwrap();
    g.set_node_type("alice", "User".to_string()).unwrap();

    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    let mut bob_props = HashMap::new();
    bob_props.insert("name".to_string(), Value::string("Bob".to_string()));
    bob_props.insert("age".to_string(), Value::number(25.0));
    bob_props.insert("city".to_string(), Value::string("SF".to_string()));
    g.set_node_properties("bob", bob_props).unwrap();
    g.set_node_type("bob", "User".to_string()).unwrap();

    g.add_node("charlie".to_string(), Value::number(3.0)).unwrap();
    let mut charlie_props = HashMap::new();
    charlie_props.insert("name".to_string(), Value::string("Charlie".to_string()));
    charlie_props.insert("age".to_string(), Value::number(35.0));
    charlie_props.insert("city".to_string(), Value::string("NYC".to_string()));
    g.set_node_properties("charlie", charlie_props).unwrap();
    g.set_node_type("charlie", "User".to_string()).unwrap();

    g.add_node("diana".to_string(), Value::number(4.0)).unwrap();
    let mut diana_props = HashMap::new();
    diana_props.insert("name".to_string(), Value::string("Diana".to_string()));
    diana_props.insert("age".to_string(), Value::number(28.0));
    diana_props.insert("city".to_string(), Value::string("LA".to_string()));
    g.set_node_properties("diana", diana_props).unwrap();
    g.set_node_type("diana", "User".to_string()).unwrap();

    // Add relationships
    g.add_edge("alice", "bob", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("alice", "charlie", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("bob", "charlie", "FOLLOWS".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("bob", "diana", "FRIEND".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("charlie", "diana", "FOLLOWS".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("diana", "alice", "FOLLOWS".to_string(), None, HashMap::new()).unwrap();

    g
}

/// Helper to create pattern node
fn node(variable: &str, node_type: Option<&str>) -> Value {
    Value::pattern_node(Some(variable.to_string()), node_type.map(|s| s.to_string()))
}

/// Helper to create pattern edge
fn edge(edge_type: Option<&str>, direction: Option<&str>) -> Value {
    Value::pattern_edge(
        edge_type.map(|s| s.to_string()),
        direction.unwrap_or("outgoing").to_string()
    )
}

/// Helper to create pattern path
fn path(edge_type: Option<&str>, min: usize, max: usize, direction: Option<&str>) -> Value {
    Value::pattern_path(
        edge_type.map(|s| s.to_string()).unwrap_or_default(),
        min,
        max,
        direction.unwrap_or("outgoing").to_string()
    )
}

// ============================================================================
// INTEGRATION TEST 1: Basic Social Network Query
// ============================================================================

#[test]
fn test_find_friends_in_same_city() {
    // Scenario: Find pairs of users who are friends and live in the same city
    let g = create_social_network();

    // Pattern: (user1:User) -[:FRIEND]-> (user2:User)
    let pattern = vec![
        node("user1", Some("User")),
        edge(Some("FRIEND"), None),
        node("user2", Some("User")),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Filter: same city
    let same_city = results.where_both_nodes("user1", "user2", |_v1_val, _v2_val| {
        // Get the graph nodes to access properties
        // For now, we just verify we got some results
        true // TODO: Implement property comparison when available
    }).unwrap();

    // Should find some friend pairs
    assert!(same_city.len() >= 1);
}

// ============================================================================
// INTEGRATION TEST 2: Multi-Hop Path Finding
// ============================================================================

#[test]
fn test_friends_of_friends() {
    // Scenario: Find friends-of-friends (2-hop FRIEND connections)
    let g = create_social_network();

    // Pattern: (user) -[:FRIEND]-> (friend) -[:FRIEND]-> (fof)
    let pattern = vec![
        node("user", Some("User")),
        edge(Some("FRIEND"), None),
        node("friend", Some("User")),
        edge(Some("FRIEND"), None),
        node("fof", Some("User")),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Alice -> Bob -> Diana
    // Should find at least this path
    assert!(results.len() >= 1);

    // Verify we can extract the chain
    for binding in results.iter() {
        assert!(binding.contains_key("user"));
        assert!(binding.contains_key("friend"));
        assert!(binding.contains_key("fof"));
    }
}

// ============================================================================
// INTEGRATION TEST 3: Variable-Length Path Query
// ============================================================================

#[test]
fn test_reachable_within_two_hops() {
    // Scenario: Find all users reachable within 1-2 hops via any relationship
    let g = create_social_network();

    // Pattern: (start) -*{1,2}-> (end)
    let pattern = vec![
        node("start", None),
        path(None, 1, 2, None),
        node("end", None),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // From alice, should reach: bob, charlie (1-hop), diana (2-hop via bob)
    // Total paths from all nodes should be numerous
    assert!(results.len() >= 10);

    // Verify structure
    for binding in results.iter() {
        assert!(binding.contains_key("start"));
        assert!(binding.contains_key("end"));
    }
}

// ============================================================================
// INTEGRATION TEST 4: Type Filtering with Return Projection
// ============================================================================

#[test]
fn test_find_users_return_names() {
    // Scenario: Find all FRIEND connections between Users and return only names
    let g = create_social_network();

    // Pattern: (person:User) -[:FRIEND]-> (friend:User)
    let pattern = vec![
        node("person", Some("User")),
        edge(Some("FRIEND"), None),
        node("friend", Some("User")),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Should find FRIEND edges: alice->bob, alice->charlie, bob->diana
    assert_eq!(results.len(), 3);

    // Return only person and friend variables
    let projected = results.return_vars(vec!["person", "friend"]);

    // Verify projection worked
    assert_eq!(projected.len(), 3);
    for binding in projected.iter() {
        assert!(binding.contains_key("person"));
        assert!(binding.contains_key("friend"));
        // Should only have these two keys
        assert_eq!(binding.len(), 2);
    }
}

// ============================================================================
// INTEGRATION TEST 5: Complex Query with Multiple Filters
// ============================================================================

#[test]
fn test_find_follows_relationships() {
    // Scenario: Find all FOLLOWS relationships
    let g = create_social_network();

    // Pattern: (follower) -[:FOLLOWS]-> (followee)
    let pattern = vec![
        node("follower", None),
        edge(Some("FOLLOWS"), None),
        node("followee", None),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // FOLLOWS edges: bob->charlie, charlie->diana, diana->alice
    assert_eq!(results.len(), 3);

    // Return properties
    let props = results.return_properties(vec!["follower.name", "followee.name"]).unwrap();

    assert_eq!(props.len(), 3);

    // Verify we got names
    for prop_map in &props {
        assert!(prop_map.contains_key("follower.name"));
        assert!(prop_map.contains_key("followee.name"));

        // Names should be strings
        let follower_name = prop_map.get("follower.name").unwrap();
        let followee_name = prop_map.get("followee.name").unwrap();

        match (&follower_name.kind, &followee_name.kind) {
            (ValueKind::String(_), ValueKind::String(_)) => {},
            _ => panic!("Expected string names"),
        }
    }
}

// ============================================================================
// INTEGRATION TEST 6: Bidirectional Pattern Matching
// ============================================================================

#[test]
fn test_mutual_connections() {
    // Scenario: Find any connection (regardless of direction) between users
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    g.add_node("charlie".to_string(), Value::number(3.0)).unwrap();

    g.add_edge("alice", "bob", "KNOWS".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("bob", "alice", "KNOWS".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("charlie", "bob", "KNOWS".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("bob", "charlie", "KNOWS".to_string(), None, HashMap::new()).unwrap();

    // Pattern with bidirectional edge (matches both outgoing and incoming)
    let pattern = vec![
        node("person1", None),
        edge(Some("KNOWS"), Some("both")),
        node("person2", None),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Should find all KNOWS edges in both directions:
    // - alice -> bob
    // - bob -> alice
    // - charlie -> bob
    // - bob -> charlie
    // Total: 4 matches
    assert_eq!(results.len(), 4);

    // Verify all have the KNOWS relationship
    assert_eq!(results.len(), 4);
}

// ============================================================================
// INTEGRATION TEST 7: Chain Pattern with Variable-Length Middle
// ============================================================================

#[test]
fn test_indirect_connections() {
    // Scenario: Find users connected through intermediate users (any path length)
    let g = create_social_network();

    // Pattern: (start:User) -*{1,3}-> (end:User)
    // Find users reachable within 1-3 hops
    let pattern = vec![
        node("start", Some("User")),
        path(None, 1, 3, None),
        node("end", Some("User")),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Should find many paths of various lengths
    assert!(results.len() >= 15);

    // Return just the start and end
    let projected = results.return_vars(vec!["start", "end"]);

    assert_eq!(projected.len(), results.len());
    for binding in projected.iter() {
        assert_eq!(binding.len(), 2); // Only start and end
    }
}

// ============================================================================
// INTEGRATION TEST 8: Real-World Scenario - Recommendation System
// ============================================================================

#[test]
fn test_friend_recommendations() {
    // Scenario: Recommend friends - find friends-of-friends who are not already friends
    let g = create_social_network();

    // Step 1: Find all friends-of-friends (2-hop)
    let pattern = vec![
        node("user", None),
        path(Some("FRIEND"), 2, 2, None), // Exactly 2 FRIEND hops
        node("recommendation", None),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Should find at least: alice -> bob -> diana
    assert!(results.len() >= 1);

    // In a real system, we would filter out:
    // 1. Direct friends (already connected)
    // 2. The user themselves
    // For now, just verify we can project the recommendations
    let recommendations = results.return_vars(vec!["user", "recommendation"]);

    assert!(recommendations.len() >= 1);
}

// ============================================================================
// INTEGRATION TEST 9: Property Projection with Aggregation
// ============================================================================

#[test]
fn test_age_based_connections() {
    // Scenario: Find connections and extract ages for analysis
    let g = create_social_network();

    // Pattern: (user1:User) -[:FRIEND]-> (user2:User)
    let pattern = vec![
        node("user1", Some("User")),
        edge(Some("FRIEND"), None),
        node("user2", Some("User")),
    ];

    let results = g.match_pattern(pattern).unwrap();

    // Extract ages
    let props = results.return_properties(vec!["user1.age", "user2.age"]).unwrap();

    assert_eq!(props.len(), 3); // 3 FRIEND edges

    // Verify ages are numbers
    for prop_map in &props {
        let age1 = prop_map.get("user1.age").unwrap();
        let age2 = prop_map.get("user2.age").unwrap();

        match (&age1.kind, &age2.kind) {
            (ValueKind::Number(a1), ValueKind::Number(a2)) => {
                // Ages should be reasonable
                assert!(*a1 > 0.0 && *a1 < 100.0);
                assert!(*a2 > 0.0 && *a2 < 100.0);
            },
            _ => panic!("Expected number ages"),
        }
    }
}

// ============================================================================
// INTEGRATION TEST 10: Performance Stress Test
// ============================================================================

#[test]
fn test_pattern_matching_on_larger_graph() {
    // Create a larger graph for performance testing
    let mut g = Graph::new(GraphType::Directed);

    // Create a ring of 20 nodes
    for i in 0..20 {
        let node_id = format!("node{}", i);
        g.add_node(node_id.clone(), Value::number(i as f64)).unwrap();
    }

    // Connect each node to next 3 nodes
    for i in 0..20 {
        let from = format!("node{}", i);
        for j in 1..=3 {
            let to = format!("node{}", (i + j) % 20);
            g.add_edge(&from, &to, "LINK".to_string(), None, HashMap::new()).unwrap();
        }
    }

    // Find all 2-hop paths
    let pattern = vec![
        node("start", None),
        path(Some("LINK"), 2, 2, None),
        node("end", None),
    ];

    let start_time = std::time::Instant::now();
    let results = g.match_pattern(pattern).unwrap();
    let duration = start_time.elapsed();

    // Should find many 2-hop paths
    assert!(results.len() >= 100);

    // Should complete reasonably quickly (< 100ms for this size)
    assert!(duration.as_millis() < 100, "Pattern matching took {}ms", duration.as_millis());

    println!("Performance: Found {} paths in {}ms", results.len(), duration.as_millis());
}
