// Unit Tests for BST Ordering Rule
//
// Tests Binary Search Tree ordering constraint:
// - Left child < parent < right child
// - Rejection of invalid insertions
// - Validation of existing trees

use graphoid::values::{Value, Graph, GraphType};
use graphoid::graph::{RuleSpec, RuleInstance};

#[test]
fn test_bst_ordering_rule_validates_left_child_less_than_parent() {
    let mut g = Graph::new(GraphType::Directed);

    // Add root with value 10
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Try to add left child with value 5 (valid: 5 < 10)
    g.add_node("left".to_string(), Value::number(5.0)).unwrap();
    g.add_edge("root", "left", "left".to_string(), None, std::collections::HashMap::new()).unwrap();

    // Verify nodes exist
    assert_eq!(g.node_count(), 2);
}

#[test]
fn test_bst_ordering_rule_rejects_left_child_greater_than_parent() {
    let mut g = Graph::new(GraphType::Directed);

    // Add root with value 10
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Add left child with value 15
    g.add_node("left".to_string(), Value::number(15.0)).unwrap();

    // Try to add edge (should fail: 15 > 10 violates BST ordering for left child)
    let result = g.add_edge("root", "left", "left".to_string(), None, std::collections::HashMap::new());

    assert!(result.is_err(), "Should reject left child greater than parent");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("BST") || err.contains("ordering") || err.contains("left"),
            "Error should mention BST ordering violation: {}", err);
}

#[test]
fn test_bst_ordering_rule_validates_right_child_greater_than_parent() {
    let mut g = Graph::new(GraphType::Directed);

    // Add root with value 10
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Try to add right child with value 15 (valid: 15 > 10)
    g.add_node("right".to_string(), Value::number(15.0)).unwrap();
    g.add_edge("root", "right", "right".to_string(), None, std::collections::HashMap::new()).unwrap();

    // Verify nodes exist
    assert_eq!(g.node_count(), 2);
}

#[test]
fn test_bst_ordering_rule_rejects_right_child_less_than_parent() {
    let mut g = Graph::new(GraphType::Directed);

    // Add root with value 10
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Add right child with value 5
    g.add_node("right".to_string(), Value::number(5.0)).unwrap();

    // Try to add edge (should fail: 5 < 10 violates BST ordering for right child)
    let result = g.add_edge("root", "right", "right".to_string(), None, std::collections::HashMap::new());

    assert!(result.is_err(), "Should reject right child less than parent");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("BST") || err.contains("ordering") || err.contains("right"),
            "Error should mention BST ordering violation: {}", err);
}

#[test]
fn test_bst_ordering_rule_rejects_equal_values() {
    let mut g = Graph::new(GraphType::Directed);

    // Add root with value 10
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Add left child with value 10 (equal to parent)
    g.add_node("left".to_string(), Value::number(10.0)).unwrap();

    // Try to add edge (should fail: BST requires strict inequality)
    let result = g.add_edge("root", "left", "left".to_string(), None, std::collections::HashMap::new());

    assert!(result.is_err(), "Should reject equal values in BST");
}

#[test]
fn test_bst_ordering_rule_validates_complex_tree() {
    let mut g = Graph::new(GraphType::Directed);

    // Add BST ordering rule first
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Build a valid BST:
    //        10
    //       /  \
    //      5    15
    //     / \
    //    3   7

    g.add_node("10".to_string(), Value::number(10.0)).unwrap();
    g.add_node("5".to_string(), Value::number(5.0)).unwrap();
    g.add_node("15".to_string(), Value::number(15.0)).unwrap();
    g.add_node("3".to_string(), Value::number(3.0)).unwrap();
    g.add_node("7".to_string(), Value::number(7.0)).unwrap();

    // Add edges
    g.add_edge("10", "5", "left".to_string(), None, std::collections::HashMap::new()).unwrap();
    g.add_edge("10", "15", "right".to_string(), None, std::collections::HashMap::new()).unwrap();
    g.add_edge("5", "3", "left".to_string(), None, std::collections::HashMap::new()).unwrap();
    g.add_edge("5", "7", "right".to_string(), None, std::collections::HashMap::new()).unwrap();

    // Verify all nodes exist
    assert_eq!(g.node_count(), 5);
}

#[test]
fn test_bst_ordering_rule_only_validates_direct_children() {
    let mut g = Graph::new(GraphType::Directed);

    // Add BST ordering rule first
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Build tree:
    //        10
    //       /  \
    //      5    15
    //     / \
    //    3   12  <- 12 > 5 (parent), which is valid for direct parent-child

    g.add_node("10".to_string(), Value::number(10.0)).unwrap();
    g.add_node("5".to_string(), Value::number(5.0)).unwrap();
    g.add_node("15".to_string(), Value::number(15.0)).unwrap();
    g.add_node("3".to_string(), Value::number(3.0)).unwrap();
    g.add_node("12".to_string(), Value::number(12.0)).unwrap();

    // Add valid edges first
    g.add_edge("10", "5", "left".to_string(), None, std::collections::HashMap::new()).unwrap();
    g.add_edge("10", "15", "right".to_string(), None, std::collections::HashMap::new()).unwrap();
    g.add_edge("5", "3", "left".to_string(), None, std::collections::HashMap::new()).unwrap();

    // Try to add edge: 12 as right child of 5
    // BST ordering rule only validates direct parent-child relationship
    // 12 > 5, so it's valid for right child (even though full BST invariant requires 12 > 10)
    g.add_edge("5", "12", "right".to_string(), None, std::collections::HashMap::new()).unwrap();

    // This should succeed because we only check direct parent-child
    assert_eq!(g.node_count(), 5);
}

#[test]
fn test_bst_ordering_rule_requires_numeric_values() {
    let mut g = Graph::new(GraphType::Directed);

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Add root with string value (should fail - BST requires numbers)
    let result = g.add_node("root".to_string(), Value::string("hello".to_string()));

    assert!(result.is_err(), "BST ordering rule should require numeric values");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("numeric") || err.contains("number") || err.contains("BST"),
            "Error should mention numeric requirement: {}", err);
}

#[test]
fn test_bst_ordering_with_edge_types() {
    let mut g = Graph::new(GraphType::Directed);

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Add nodes
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();
    g.add_node("child1".to_string(), Value::number(5.0)).unwrap();
    g.add_node("child2".to_string(), Value::number(15.0)).unwrap();

    // Only edges labeled "left" or "right" should trigger BST validation
    g.add_edge("root", "child1", "left".to_string(), None, std::collections::HashMap::new()).unwrap();
    g.add_edge("root", "child2", "right".to_string(), None, std::collections::HashMap::new()).unwrap();

    assert_eq!(g.node_count(), 3);
}

#[test]
fn test_bst_ordering_ignores_non_left_right_edges() {
    let mut g = Graph::new(GraphType::Directed);

    // Add BST ordering rule
    g.add_rule(RuleInstance::new(RuleSpec::BSTOrdering)).unwrap();

    // Add nodes
    g.add_node("root".to_string(), Value::number(10.0)).unwrap();
    g.add_node("other".to_string(), Value::number(15.0)).unwrap();

    // Edge with different label should not trigger BST validation
    g.add_edge("root", "other", "custom".to_string(), None, std::collections::HashMap::new()).unwrap();

    assert_eq!(g.node_count(), 2);
}
