//! Tests for ad hoc rule addition and removal
//!
//! These tests verify that rules can be added and removed dynamically,
//! independent of rulesets.

use graphoid::values::{Value, Graph};
use graphoid::values::graph::GraphType;
use graphoid::graph::{RuleSpec, RuleInstance};
use graphoid::error::GraphoidError;
use std::collections::HashMap;

#[test]
fn test_add_rule_enforces_constraint() {
    // Create a plain graph with no rules
    let mut graph = Graph::new(GraphType::Directed);

    // Should be able to create a cycle without rules
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    assert!(graph.add_edge("B", "A", "edge".to_string(), None, HashMap::new()).is_ok());

    // Now add no_cycles rule
    let mut graph2 = Graph::new(GraphType::Directed);
    graph2.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();

    // Build same structure
    graph2.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph2.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph2.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

    // Now adding the reverse edge should fail
    let result = graph2.add_edge("B", "A", "edge".to_string(), None, HashMap::new());
    assert!(result.is_err());
    match result {
        Err(GraphoidError::RuleViolation { rule, .. }) => {
            assert_eq!(rule, "no_cycles");
        }
        _ => panic!("Expected RuleViolation"),
    }
}

#[test]
fn test_add_rule_with_parameter() {
    // Test max_degree rule with specific parameter
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_rule(RuleInstance::new(RuleSpec::MaxDegree(2))).unwrap();

    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("child1".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("child2".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("child3".to_string(), Value::number(4.0)).unwrap();

    // First two edges should succeed
    graph.add_edge("root", "child1", "edge".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("root", "child2", "edge".to_string(), None, HashMap::new()).unwrap();

    // Third edge should fail
    let result = graph.add_edge("root", "child3", "edge".to_string(), None, HashMap::new());
    assert!(result.is_err());
    match result {
        Err(GraphoidError::RuleViolation { rule, message }) => {
            assert_eq!(rule, "max_degree");
            assert!(message.contains("maximum is 2"));
        }
        _ => panic!("Expected RuleViolation"),
    }
}

#[test]
fn test_remove_rule_disables_constraint() {
    // Create graph with no_cycles rule
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();

    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

    // Cycle should fail
    assert!(graph.add_edge("B", "A", "edge".to_string(), None, HashMap::new()).is_err());

    // Remove the rule
    graph.remove_rule(&RuleSpec::NoCycles);

    // Now cycle should succeed
    assert!(graph.add_edge("B", "A", "edge".to_string(), None, HashMap::new()).is_ok());
}

#[test]
fn test_has_rule_checks_ad_hoc_rules() {
    let mut graph = Graph::new(GraphType::Directed);

    assert!(!graph.has_rule("no_cycles"));

    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();

    assert!(graph.has_rule("no_cycles"));
}

#[test]
fn test_has_rule_checks_ruleset_rules() {
    let graph = Graph::new(GraphType::Directed)
        .with_ruleset("tree".to_string());

    // Tree ruleset includes these rules
    assert!(graph.has_rule("no_cycles"));
    assert!(graph.has_rule("single_root"));
    assert!(graph.has_rule("connected"));

    // But not this one
    assert!(!graph.has_rule("binary_tree"));
}

#[test]
fn test_get_rules_returns_ad_hoc_only() {
    let mut graph = Graph::new(GraphType::Directed)
        .with_ruleset("tree".to_string());

    // get_rules() should be empty (only returns ad hoc rules)
    assert_eq!(graph.get_rules().len(), 0);

    // Add ad hoc rule
    graph.add_rule(RuleInstance::new(RuleSpec::MaxDegree(3))).unwrap();

    // Now should have one ad hoc rule
    assert_eq!(graph.get_rules().len(), 1);
    assert_eq!(graph.get_rules()[0].spec, RuleSpec::MaxDegree(3));
}

#[test]
fn test_ad_hoc_rules_combine_with_rulesets() {
    // Start with tree ruleset
    let mut graph = Graph::new(GraphType::Directed)
        .with_ruleset("tree".to_string());

    // Add max_degree as ad hoc rule
    graph.add_rule(RuleInstance::new(RuleSpec::MaxDegree(2))).unwrap();

    // Now graph should enforce BOTH tree rules AND max_degree
    assert!(graph.has_rule("no_cycles"));
    assert!(graph.has_rule("single_root"));
    assert!(graph.has_rule("connected"));
    assert!(graph.has_rule("max_degree"));

    // Build a tree
    graph.add_node("root".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("left".to_string(), Value::number(2.0)).unwrap();
    graph.add_node("right".to_string(), Value::number(3.0)).unwrap();
    graph.add_node("extra".to_string(), Value::number(4.0)).unwrap();

    graph.add_edge("root", "left", "child".to_string(), None, HashMap::new()).unwrap();
    graph.add_edge("root", "right", "child".to_string(), None, HashMap::new()).unwrap();

    // Third child should fail due to max_degree
    let result = graph.add_edge("root", "extra", "child".to_string(), None, HashMap::new());
    assert!(result.is_err());
}

#[test]
fn test_duplicate_rules_not_added() {
    let mut graph = Graph::new(GraphType::Directed);

    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();
    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();
    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();

    // Should only have one instance
    assert_eq!(graph.get_rules().len(), 1);
}

#[test]
fn test_remove_rule_that_doesnt_exist() {
    let mut graph = Graph::new(GraphType::Directed);

    // Should not panic
    graph.remove_rule(&RuleSpec::NoCycles);

    assert_eq!(graph.get_rules().len(), 0);
}

#[test]
fn test_remove_rule_does_not_affect_ruleset() {
    let mut graph = Graph::new(GraphType::Directed)
        .with_ruleset("tree".to_string());

    // Tree has no_cycles from ruleset
    assert!(graph.has_rule("no_cycles"));

    // Try to remove it as ad hoc rule (should have no effect)
    graph.remove_rule(&RuleSpec::NoCycles);

    // Should still be active from ruleset
    assert!(graph.has_rule("no_cycles"));
}

#[test]
fn test_ruleset_and_ad_hoc_deduplication() {
    // Tree ruleset includes NoCycles
    let mut graph = Graph::new(GraphType::Directed)
        .with_ruleset("tree".to_string());

    // Add NoCycles as ad hoc rule too
    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();

    // Build a structure
    graph.add_node("A".to_string(), Value::number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::number(2.0)).unwrap();
    graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

    // Cycle should still fail (rule should only be checked once, not twice)
    let result = graph.add_edge("B", "A", "edge".to_string(), None, HashMap::new());
    assert!(result.is_err());
}

#[test]
fn test_multiple_max_degree_parameters() {
    let mut graph1 = Graph::new(GraphType::Directed);
    graph1.add_rule(RuleInstance::new(RuleSpec::MaxDegree(1))).unwrap();

    let mut graph2 = Graph::new(GraphType::Directed);
    graph2.add_rule(RuleInstance::new(RuleSpec::MaxDegree(3))).unwrap();

    // Different graphs, different constraints
    assert_eq!(graph1.get_rules()[0].spec, RuleSpec::MaxDegree(1));
    assert_eq!(graph2.get_rules()[0].spec, RuleSpec::MaxDegree(3));
}

#[test]
fn test_empty_graph_with_no_rules() {
    let graph = Graph::new(GraphType::Directed);

    assert!(!graph.has_rule("no_cycles"));
    assert_eq!(graph.get_rules().len(), 0);
    assert_eq!(graph.get_rulesets().len(), 0);
}
