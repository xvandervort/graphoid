//! Graph and Tree unit tests

use graphoid::values::{Graph, GraphType, Value};
// Tree import removed - Tree is no longer a separate type (Option A refactor)
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
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();

    assert_eq!(g.node_count(), 2);
    assert!(g.has_node("alice"));
    assert!(g.has_node("bob"));
    assert!(!g.has_node("charlie"));
}

#[test]
fn test_graph_add_edge() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    g.add_edge("alice", "bob", "follows".to_string(), None, HashMap::new()).unwrap();

    assert_eq!(g.edge_count(), 1);
    assert!(g.has_edge("alice", "bob"));
    assert!(!g.has_edge("bob", "alice")); // Directed
}

#[test]
fn test_graph_undirected_edge() {
    let mut g = Graph::new(GraphType::Undirected);
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    g.add_edge("alice", "bob", "friend".to_string(), None, HashMap::new()).unwrap();

    // Undirected graphs have edges in both directions
    assert!(g.has_edge("alice", "bob"));
    assert!(g.has_edge("bob", "alice"));
    // But edge count is still reported correctly (counts both directions)
    assert_eq!(g.edge_count(), 2);
}

#[test]
fn test_graph_neighbors() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    g.add_node("charlie".to_string(), Value::number(3.0)).unwrap();

    g.add_edge("alice", "bob", "follows".to_string(), None, HashMap::new()).unwrap();
    g.add_edge("alice", "charlie", "follows".to_string(), None, HashMap::new()).unwrap();

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
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    g.add_edge("alice", "bob", "follows".to_string(), None, HashMap::new()).unwrap();

    assert_eq!(g.node_count(), 2);
    assert_eq!(g.edge_count(), 1);

    g.remove_node("bob", None).unwrap();
    assert_eq!(g.node_count(), 1);
    assert_eq!(g.edge_count(), 0); // Edge to bob should be removed
    assert!(!g.has_node("bob"));
}

#[test]
fn test_graph_remove_edge() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();
    g.add_edge("alice", "bob", "follows".to_string(), None, HashMap::new()).unwrap();

    assert!(g.has_edge("alice", "bob"));

    let removed = g.remove_edge("alice", "bob").unwrap();
    assert!(removed);
    assert!(!g.has_edge("alice", "bob"));
    assert_eq!(g.edge_count(), 0);
}

#[test]
fn test_graph_get_node() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::number(42.0)).unwrap();

    assert_eq!(g.get_node("alice"), Some(&Value::number(42.0)));
    assert_eq!(g.get_node("bob"), None);
}

#[test]
fn test_graph_keys_values() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();
    g.add_node("bob".to_string(), Value::number(2.0)).unwrap();

    let keys = g.keys();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"alice".to_string()));
    assert!(keys.contains(&"bob".to_string()));

    let values = g.values();
    assert_eq!(values.len(), 2);
}

// ============================================================================
// BASIC TREE TESTS (using Graph with :tree ruleset)
// ============================================================================
// NOTE: These tests were rewritten in Step 6 to test basic trees (not BST)
// Basic trees have manual structure, not automatic BST ordering

#[test]
fn test_basic_tree_creation() {
    // Basic tree is just a graph with :tree ruleset
    let t = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    assert_eq!(t.node_count(), 0);
    assert!(t.has_ruleset("tree"));
}

#[test]
fn test_basic_tree_manual_structure() {
    // Basic tree requires manual parent specification
    let mut t = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    let root = t.insert(Value::number(5.0), None).unwrap();
    let left = t.insert(Value::number(3.0), Some(&root)).unwrap();
    let right = t.insert(Value::number(7.0), Some(&root)).unwrap();

    assert_eq!(t.node_count(), 3);
    assert!(t.has_node(&root));
    assert!(t.has_node(&left));
    assert!(t.has_node(&right));
}

#[test]
fn test_basic_tree_contains() {
    let mut t = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    let root = t.insert(Value::number(5.0), None).unwrap();
    t.insert(Value::number(3.0), Some(&root)).unwrap();
    t.insert(Value::number(7.0), Some(&root)).unwrap();

    assert!(t.contains(&Value::number(5.0)));
    assert!(t.contains(&Value::number(3.0)));
    assert!(t.contains(&Value::number(7.0)));
    assert!(!t.contains(&Value::number(10.0)));
}

#[test]
fn test_basic_tree_traversals() {
    // Build a simple tree structure manually
    //     5
    //    / \
    //   3   7
    let mut t = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    let root = t.insert(Value::number(5.0), None).unwrap();
    t.insert(Value::number(3.0), Some(&root)).unwrap();
    t.insert(Value::number(7.0), Some(&root)).unwrap();

    // BFS and DFS should work
    let bfs = t.bfs(&root);
    assert_eq!(bfs.len(), 3);
    assert_eq!(bfs[0], root);

    let dfs = t.dfs(&root);
    assert_eq!(dfs.len(), 3);
    assert_eq!(dfs[0], root);
}

// Note: Ordered traversals (in_order, pre_order, post_order) work on any graph
// They just assume binary structure, no ordering requirement

// ============================================================================
// GRAPH TREE-LIKE METHODS TESTS (for Option A refactor)
// ============================================================================

#[test]
fn test_graph_insert_without_parent() {
    let mut g = Graph::new(GraphType::Directed);
    let node_id = g.insert(Value::number(5.0), None).unwrap();

    assert_eq!(g.node_count(), 1);
    assert!(g.has_node(&node_id));
    assert_eq!(g.get_node(&node_id), Some(&Value::number(5.0)));
}

#[test]
fn test_graph_insert_with_parent() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(5.0), None).unwrap();
    let child = g.insert(Value::number(3.0), Some(&root)).unwrap();

    assert_eq!(g.node_count(), 2);
    assert!(g.has_node(&root));
    assert!(g.has_node(&child));
    assert!(g.has_edge(&root, &child));
}

#[test]
fn test_graph_insert_multiple_children() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(5.0), None).unwrap();
    let left = g.insert(Value::number(3.0), Some(&root)).unwrap();
    let right = g.insert(Value::number(7.0), Some(&root)).unwrap();

    assert_eq!(g.node_count(), 3);
    let neighbors = g.neighbors(&root);
    assert_eq!(neighbors.len(), 2);
    assert!(neighbors.contains(&left));
    assert!(neighbors.contains(&right));
}

#[test]
fn test_graph_contains_found() {
    let mut g = Graph::new(GraphType::Directed);
    g.insert(Value::number(5.0), None).unwrap();
    g.insert(Value::number(3.0), None).unwrap();
    g.insert(Value::number(7.0), None).unwrap();

    assert!(g.contains(&Value::number(5.0)));
    assert!(g.contains(&Value::number(3.0)));
    assert!(g.contains(&Value::number(7.0)));
}

#[test]
fn test_graph_contains_not_found() {
    let mut g = Graph::new(GraphType::Directed);
    g.insert(Value::number(5.0), None).unwrap();

    assert!(!g.contains(&Value::number(10.0)));
}

#[test]
fn test_graph_contains_empty() {
    let g = Graph::new(GraphType::Directed);
    assert!(!g.contains(&Value::number(5.0)));
}

#[test]
fn test_graph_bfs_simple() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(1.0), None).unwrap();
    let left = g.insert(Value::number(2.0), Some(&root)).unwrap();
    let right = g.insert(Value::number(3.0), Some(&root)).unwrap();

    let traversal = g.bfs(&root);
    // BFS should visit root first, then both children (order doesn't matter)
    assert_eq!(traversal.len(), 3);
    assert_eq!(traversal[0], root);
    assert!(traversal.contains(&left));
    assert!(traversal.contains(&right));
}

#[test]
fn test_graph_bfs_empty() {
    let g = Graph::new(GraphType::Directed);
    let traversal = g.bfs("nonexistent");
    assert_eq!(traversal, Vec::<String>::new());
}

#[test]
fn test_graph_bfs_deeper_tree() {
    let mut g = Graph::new(GraphType::Directed);
    //       1
    //      / \
    //     2   3
    //    /
    //   4
    let n1 = g.insert(Value::number(1.0), None).unwrap();
    let n2 = g.insert(Value::number(2.0), Some(&n1)).unwrap();
    let n3 = g.insert(Value::number(3.0), Some(&n1)).unwrap();
    let n4 = g.insert(Value::number(4.0), Some(&n2)).unwrap();

    let traversal = g.bfs(&n1);
    // BFS order: n1 first, then n2 and n3 (in any order), then n4
    assert_eq!(traversal.len(), 4);
    assert_eq!(traversal[0], n1); // Root first
    // n2 and n3 should be at indices 1 and 2 (any order)
    assert!(traversal[1] == n2 || traversal[1] == n3);
    assert!(traversal[2] == n2 || traversal[2] == n3);
    assert_eq!(traversal[3], n4); // n4 should be last (level 3)
}

#[test]
fn test_graph_dfs_simple() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(1.0), None).unwrap();
    let left = g.insert(Value::number(2.0), Some(&root)).unwrap();
    let right = g.insert(Value::number(3.0), Some(&root)).unwrap();

    let traversal = g.dfs(&root);
    // DFS should visit root first, then explore depth-first
    assert_eq!(traversal.len(), 3);
    assert_eq!(traversal[0], root);
    assert!(traversal.contains(&left));
    assert!(traversal.contains(&right));
}

#[test]
fn test_graph_dfs_empty() {
    let g = Graph::new(GraphType::Directed);
    let traversal = g.dfs("nonexistent");
    assert_eq!(traversal, Vec::<String>::new());
}

#[test]
fn test_graph_dfs_deeper_tree() {
    let mut g = Graph::new(GraphType::Directed);
    //       1
    //      / \
    //     2   3
    //    /
    //   4
    let n1 = g.insert(Value::number(1.0), None).unwrap();
    let n2 = g.insert(Value::number(2.0), Some(&n1)).unwrap();
    let n3 = g.insert(Value::number(3.0), Some(&n1)).unwrap();
    let n4 = g.insert(Value::number(4.0), Some(&n2)).unwrap();

    let traversal = g.dfs(&n1);
    // DFS should go deep before wide
    assert_eq!(traversal.len(), 4);
    assert_eq!(traversal[0], n1); // Root first
    assert!(traversal.contains(&n2));
    assert!(traversal.contains(&n3));
    assert!(traversal.contains(&n4));
}

#[test]
fn test_graph_in_order_simple() {
    let mut g = Graph::new(GraphType::Directed);
    //     5
    //    / \
    //   3   7
    let root = g.insert(Value::number(5.0), None).unwrap();
    g.insert(Value::number(3.0), Some(&root)).unwrap();
    g.insert(Value::number(7.0), Some(&root)).unwrap();

    let values = g.in_order(&root);
    // In-order: left, root, right = 3, 5, 7
    assert_eq!(values.len(), 3);
    assert!(values.contains(&Value::number(3.0)));
    assert!(values.contains(&Value::number(5.0)));
    assert!(values.contains(&Value::number(7.0)));
}

#[test]
fn test_graph_in_order_empty() {
    let g = Graph::new(GraphType::Directed);
    let values = g.in_order("nonexistent");
    assert_eq!(values, Vec::<Value>::new());
}

#[test]
fn test_graph_in_order_single_node() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(5.0), None).unwrap();

    let values = g.in_order(&root);
    assert_eq!(values, vec![Value::number(5.0)]);
}

#[test]
fn test_graph_pre_order_simple() {
    let mut g = Graph::new(GraphType::Directed);
    //     5
    //    / \
    //   3   7
    let root = g.insert(Value::number(5.0), None).unwrap();
    g.insert(Value::number(3.0), Some(&root)).unwrap();
    g.insert(Value::number(7.0), Some(&root)).unwrap();

    let values = g.pre_order(&root);
    // Pre-order: root, left, right
    assert_eq!(values.len(), 3);
    assert!(values.contains(&Value::number(3.0)));
    assert!(values.contains(&Value::number(5.0)));
    assert!(values.contains(&Value::number(7.0)));
    // Root should be first
    assert_eq!(values[0], Value::number(5.0));
}

#[test]
fn test_graph_pre_order_empty() {
    let g = Graph::new(GraphType::Directed);
    let values = g.pre_order("nonexistent");
    assert_eq!(values, Vec::<Value>::new());
}

#[test]
fn test_graph_pre_order_single_node() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(5.0), None).unwrap();

    let values = g.pre_order(&root);
    assert_eq!(values, vec![Value::number(5.0)]);
}

#[test]
fn test_graph_post_order_simple() {
    let mut g = Graph::new(GraphType::Directed);
    //     5
    //    / \
    //   3   7
    let root = g.insert(Value::number(5.0), None).unwrap();
    g.insert(Value::number(3.0), Some(&root)).unwrap();
    g.insert(Value::number(7.0), Some(&root)).unwrap();

    let values = g.post_order(&root);
    // Post-order: left, right, root
    assert_eq!(values.len(), 3);
    assert!(values.contains(&Value::number(3.0)));
    assert!(values.contains(&Value::number(5.0)));
    assert!(values.contains(&Value::number(7.0)));
    // Root should be last
    assert_eq!(values[2], Value::number(5.0));
}

#[test]
fn test_graph_post_order_empty() {
    let g = Graph::new(GraphType::Directed);
    let values = g.post_order("nonexistent");
    assert_eq!(values, Vec::<Value>::new());
}

#[test]
fn test_graph_post_order_single_node() {
    let mut g = Graph::new(GraphType::Directed);
    let root = g.insert(Value::number(5.0), None).unwrap();

    let values = g.post_order(&root);
    assert_eq!(values, vec![Value::number(5.0)]);
}

// ============================================================================
// VALUE INTEGRATION TESTS
// ============================================================================

#[test]
fn test_graph_as_value() {
    let mut g = Graph::new(GraphType::Directed);
    g.add_node("alice".to_string(), Value::number(1.0)).unwrap();

    let val = Value::graph(g);
    assert_eq!(val.type_name(), "graph");
    assert!(val.is_truthy());
}

#[test]
fn test_empty_graph_is_falsy() {
    let g = Graph::new(GraphType::Directed);
    let val = Value::graph(g);
    assert!(!val.is_truthy());
}

// NOTE: These tests rewritten in Step 6 to test tree as graph with ruleset

#[test]
fn test_tree_as_graph_value() {
    // tree{} creates a graph with :tree ruleset
    let t = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    let mut graph_with_node = t.clone();
    graph_with_node.insert(Value::number(5.0), None).unwrap();

    let val = Value::graph(graph_with_node);
    assert_eq!(val.type_name(), "graph");  // It's a graph, not a separate type
    assert!(val.is_truthy());  // Non-empty graph is truthy
}

#[test]
fn test_empty_tree_graph_is_falsy() {
    let t = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    let val = Value::graph(t);
    assert!(!val.is_truthy());  // Empty graph is falsy
}

// ============================================================================
// METHOD STORAGE TESTS (Class-like Graphs)
// ============================================================================

use graphoid::values::Function;
use graphoid::execution::Environment;
use std::rc::Rc;
use std::cell::RefCell;

/// Helper to create a simple test function
fn make_test_function(name: &str) -> Function {
    Function {
        name: Some(name.to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
    }
}

#[test]
fn test_attach_method_creates_methods_branch() {
    let mut g = Graph::new(GraphType::Directed);

    // Initially no __methods__ branch
    assert!(!g.has_node("__methods__"));

    // Attach a method
    let func = make_test_function("add");
    g.attach_method("add".to_string(), func);

    // Now __methods__ branch should exist
    assert!(g.has_node("__methods__"));
    // And the method node
    assert!(g.has_node("__methods__/add"));
}

#[test]
fn test_attach_method_creates_edge_to_method() {
    let mut g = Graph::new(GraphType::Directed);

    let func = make_test_function("increment");
    g.attach_method("increment".to_string(), func);

    // Should have edge from __methods__ to __methods__/increment
    assert!(g.has_edge("__methods__", "__methods__/increment"));
}

#[test]
fn test_get_method_returns_function() {
    let mut g = Graph::new(GraphType::Directed);

    let func = make_test_function("calculate");
    g.attach_method("calculate".to_string(), func);

    // Should retrieve the method
    let retrieved = g.get_method("calculate");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, Some("calculate".to_string()));
}

#[test]
fn test_get_method_returns_none_for_nonexistent() {
    let g = Graph::new(GraphType::Directed);
    assert!(g.get_method("nonexistent").is_none());
}

#[test]
fn test_has_method() {
    let mut g = Graph::new(GraphType::Directed);

    assert!(!g.has_method("foo"));

    let func = make_test_function("foo");
    g.attach_method("foo".to_string(), func);

    assert!(g.has_method("foo"));
    assert!(!g.has_method("bar"));
}

#[test]
fn test_method_names() {
    let mut g = Graph::new(GraphType::Directed);

    g.attach_method("add".to_string(), make_test_function("add"));
    g.attach_method("subtract".to_string(), make_test_function("subtract"));
    g.attach_method("multiply".to_string(), make_test_function("multiply"));

    let mut names = g.method_names();
    names.sort();

    assert_eq!(names, vec!["add", "multiply", "subtract"]);
}

#[test]
fn test_data_node_ids_excludes_methods() {
    let mut g = Graph::new(GraphType::Directed);

    // Add data nodes
    g.add_node("count".to_string(), Value::number(0.0)).unwrap();
    g.add_node("name".to_string(), Value::string("test".to_string())).unwrap();

    // Add methods
    g.attach_method("increment".to_string(), make_test_function("increment"));
    g.attach_method("get_name".to_string(), make_test_function("get_name"));

    // data_node_ids should only return data nodes
    let mut data_ids = g.data_node_ids();
    data_ids.sort();

    assert_eq!(data_ids, vec!["count", "name"]);

    // Total node count includes method nodes
    // 2 data + 1 __methods__ branch + 2 method nodes = 5
    assert_eq!(g.node_count(), 5);
}

#[test]
fn test_clone_preserves_methods() {
    let mut g = Graph::new(GraphType::Directed);

    g.add_node("value".to_string(), Value::number(42.0)).unwrap();
    g.attach_method("get_value".to_string(), make_test_function("get_value"));

    // Clone the graph
    let g2 = g.clone();

    // Clone should have the method
    assert!(g2.has_method("get_value"));
    assert!(g2.get_method("get_value").is_some());

    // And the data
    assert!(g2.has_node("value"));
}

#[test]
fn test_method_node_has_correct_type() {
    let mut g = Graph::new(GraphType::Directed);

    g.attach_method("test".to_string(), make_test_function("test"));

    // The method node should have node_type "__method__"
    // Access through public nodes field to check GraphNode properties
    if let Some(node) = g.nodes.get("__methods__/test") {
        assert_eq!(node.node_type, Some("__method__".to_string()));
    } else {
        panic!("Method node not found");
    }

    // The branch node should have node_type "__branch__"
    if let Some(node) = g.nodes.get("__methods__") {
        assert_eq!(node.node_type, Some("__branch__".to_string()));
    } else {
        panic!("Branch node not found");
    }
}
