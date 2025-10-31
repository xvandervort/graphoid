//! Tests for graph rule implementations
//!
//! Tests for built-in graph rules like no_cycles, single_root, max_degree, etc.

use graphoid::graph::rules::{
    NoCyclesRule, SingleRootRule, MaxDegreeRule, BinaryTreeRule,
    WeightedEdgesRule, UnweightedEdgesRule, RuleContext, RuleSpec,
};
use graphoid::graph::GraphOperation;
use graphoid::values::{Value, Graph};
use graphoid::values::graph::GraphType;
use graphoid::error::GraphoidError;
use std::collections::HashMap;

#[test]
fn test_no_cycles_rule_allows_acyclic_edge() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let rule = NoCyclesRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "A".to_string(),
        to: "B".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    assert!(rule.validate(&graph, &context).is_ok());
}

#[test]
fn test_no_cycles_rule_detects_cycle() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

    let rule = NoCyclesRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "B".to_string(),
        to: "A".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    let result = rule.validate(&graph, &context);
    assert!(result.is_err());
    if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
        assert_eq!(rule_name, "no_cycles");
        assert!(message.contains("cycle"));
    } else {
        panic!("Expected RuleViolation error");
    }
}

#[test]
fn test_single_root_rule_allows_single_root() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("child".to_string(), Value::Number(2.0)).unwrap();
    graph.add_edge("root", "child", "edge".to_string(), None, HashMap::new()).unwrap();

    let rule = SingleRootRule::new();
    let context = RuleContext::new(GraphOperation::AddNode {
        id: "another_child".to_string(),
        value: Value::Number(3.0),
    });

    assert!(rule.validate(&graph, &context).is_ok());
}

#[test]
fn test_single_root_rule_detects_multiple_roots() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root1".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("root2".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("child".to_string(), Value::Number(3.0)).unwrap();

    let rule = SingleRootRule::new();
    let context = RuleContext::new(GraphOperation::AddNode {
        id: "test".to_string(),
        value: Value::Number(4.0),
    });

    let result = rule.validate(&graph, &context);
    assert!(result.is_err());
    if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
        assert_eq!(rule_name, "single_root");
        assert!(message.contains("must have exactly one root"));
    }
}

#[test]
fn test_max_degree_rule() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();
    graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

    let rule = MaxDegreeRule::new(1);
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "A".to_string(),
        to: "C".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    let result = rule.validate(&graph, &context);
    assert!(result.is_err());
    if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
        assert_eq!(rule_name, "max_degree");
        assert!(message.contains("maximum is 1"));
    }
}

#[test]
fn test_binary_tree_rule_allows_two_children() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("left".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("right".to_string(), Value::Number(3.0)).unwrap();
    graph.add_edge("root", "left", "edge".to_string(), None, HashMap::new()).unwrap();

    let rule = BinaryTreeRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "root".to_string(),
        to: "right".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    assert!(rule.validate(&graph, &context).is_ok());
}

#[test]
fn test_binary_tree_rule_rejects_three_children() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("root".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("child1".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("child2".to_string(), Value::Number(3.0)).unwrap();
    graph.add_node("child3".to_string(), Value::Number(4.0)).unwrap();
    graph.add_edge("root", "child1", "edge".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("root", "child2", "edge".to_string(), None, HashMap::new()).unwrap();

    let rule = BinaryTreeRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "root".to_string(),
        to: "child3".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    let result = rule.validate(&graph, &context);
    assert!(result.is_err());
}

// ============================================================================
// Weight Validation Rules Tests (10 tests)
// ============================================================================

#[test]
fn test_weighted_edges_rule_allows_weighted_edge() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let rule = WeightedEdgesRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "A".to_string(),
        to: "B".to_string(),
        edge_type: "edge".to_string(),
        weight: Some(5.0),
        properties: HashMap::new(),
    });

    assert!(rule.validate(&graph, &context).is_ok());
}

#[test]
fn test_weighted_edges_rule_rejects_unweighted_edge() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let rule = WeightedEdgesRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "A".to_string(),
        to: "B".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    let result = rule.validate(&graph, &context);
    assert!(result.is_err());
    if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
        assert_eq!(rule_name, "weighted_edges");
        assert!(message.contains("must have a weight"));
    }
}

#[test]
fn test_weighted_edges_rule_clean_removes_unweighted() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    // Add one weighted and one unweighted edge
    graph.add_edge("A", "B", "edge".to_string(), Some(5.0), HashMap::new()).unwrap();
    graph.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();

    assert_eq!(graph.edge_count(), 2);

    let rule = WeightedEdgesRule::new();
    rule.clean(&mut graph).unwrap();

    // Only the weighted edge should remain
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.has_edge("A", "B"));
    assert!(!graph.has_edge("B", "C"));
}

#[test]
fn test_unweighted_edges_rule_allows_unweighted_edge() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let rule = UnweightedEdgesRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "A".to_string(),
        to: "B".to_string(),
        edge_type: "edge".to_string(),
        weight: None,
        properties: HashMap::new(),
    });

    assert!(rule.validate(&graph, &context).is_ok());
}

#[test]
fn test_unweighted_edges_rule_rejects_weighted_edge() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let rule = UnweightedEdgesRule::new();
    let context = RuleContext::new(GraphOperation::AddEdge {
        from: "A".to_string(),
        to: "B".to_string(),
        edge_type: "edge".to_string(),
        weight: Some(3.0),
        properties: HashMap::new(),
    });

    let result = rule.validate(&graph, &context);
    assert!(result.is_err());
    if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
        assert_eq!(rule_name, "unweighted_edges");
        assert!(message.contains("must not have a weight"));
    }
}

#[test]
fn test_unweighted_edges_rule_clean_removes_weights() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

    // Add weighted edges
    graph.add_edge("A", "B", "edge".to_string(), Some(5.0), HashMap::new()).unwrap();
    graph.add_edge("B", "C", "edge".to_string(), Some(10.0), HashMap::new()).unwrap();

    assert!(graph.is_edge_weighted("A", "B"));
    assert!(graph.is_edge_weighted("B", "C"));

    let rule = UnweightedEdgesRule::new();
    rule.clean(&mut graph).unwrap();

    // Edges should still exist but without weights
    assert!(graph.has_edge("A", "B"));
    assert!(graph.has_edge("B", "C"));
    assert!(!graph.is_edge_weighted("A", "B"));
    assert!(!graph.is_edge_weighted("B", "C"));
}

#[test]
fn test_weighted_edges_rule_spec_name() {
    assert_eq!(RuleSpec::WeightedEdges.name(), "weighted_edges");
}

#[test]
fn test_unweighted_edges_rule_spec_name() {
    assert_eq!(RuleSpec::UnweightedEdges.name(), "unweighted_edges");
}

#[test]
fn test_weighted_edges_rule_instantiation() {
    let rule = RuleSpec::WeightedEdges.instantiate();
    assert_eq!(rule.name(), "weighted_edges");
}

#[test]
fn test_unweighted_edges_rule_instantiation() {
    let rule = RuleSpec::UnweightedEdges.instantiate();
    assert_eq!(rule.name(), "unweighted_edges");
}
