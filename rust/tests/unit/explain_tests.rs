//! Tests for execution plan explanation (explain functionality)
//!
//! These tests verify that graphs can generate human-readable execution plans
//! showing what algorithms will be used and why.

use graphoid::values::{Value, Graph, ExecutionPlan};
use graphoid::values::graph::GraphType;
use graphoid::graph::RuleInstance;
use graphoid::graph::RuleSpec;

#[test]
fn test_explain_find_property_without_index() {
    let graph = Graph::new(GraphType::Directed);

    let plan = graph.explain_find_property("email");

    // Should show linear scan
    assert!(plan.to_string().contains("Linear scan"));
    assert!(plan.to_string().contains("O(n)"));
    assert!(plan.shows_estimated_cost());
}

#[test]
fn test_explain_find_property_with_index() {
    let mut graph = Graph::new(GraphType::Directed);

    // Trigger auto-indexing
    for _ in 0..10 {
        graph.find_nodes_by_property("email", &Value::String("test@example.com".to_string()));
    }

    let plan = graph.explain_find_property("email");

    // Should show index usage
    assert!(plan.to_string().contains("index"));
    assert!(plan.to_string().contains("O(1)"));
    assert!(plan.to_string().contains("indexed"));
}

#[test]
fn test_explain_shortest_path_with_no_cycles() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_rule(RuleInstance::new(RuleSpec::NoCycles)).unwrap();

    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let plan = graph.explain_shortest_path("A", "B");

    // Should show topological optimization
    assert!(plan.to_string().contains("topological"));
    assert!(plan.to_string().contains("no_cycles"));
    assert!(plan.shows_estimated_cost());
}

#[test]
fn test_explain_shortest_path_without_rules() {
    let mut graph = Graph::new(GraphType::Directed);

    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

    let plan = graph.explain_shortest_path("A", "B");

    // Should show standard BFS
    assert!(plan.to_string().contains("BFS"));
    assert!(plan.shows_estimated_cost());
}

#[test]
fn test_explain_bfs_with_connected_rule() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_rule(RuleInstance::new(RuleSpec::Connected)).unwrap();

    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();

    let plan = graph.explain_bfs("A");

    // Should mention connected optimization
    assert!(plan.to_string().contains("connected"));
    assert!(plan.to_string().contains("skip component check"));
}

#[test]
fn test_explain_bfs_basic() {
    let mut graph = Graph::new(GraphType::Directed);
    graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();

    let plan = graph.explain_bfs("A");

    // Should show BFS steps
    assert!(plan.to_string().contains("queue"));
    assert!(plan.to_string().contains("visited"));
    assert!(plan.shows_estimated_cost());
}

#[test]
fn test_execution_plan_structure() {
    let mut plan = ExecutionPlan::new("test_operation".to_string());

    plan.add_step("Step 1".to_string());
    plan.add_step("Step 2".to_string());
    plan.add_optimization("Optimization 1".to_string());
    plan.set_cost(100);

    assert_eq!(plan.operation, "test_operation");
    assert_eq!(plan.steps.len(), 2);
    assert_eq!(plan.optimizations.len(), 1);
    assert_eq!(plan.estimated_cost, 100);
    assert!(plan.shows_estimated_cost());
}

#[test]
fn test_execution_plan_display() {
    let mut plan = ExecutionPlan::new("sample_op".to_string());
    plan.add_step("First step".to_string());
    plan.add_step("Second step".to_string());
    plan.add_optimization("Some optimization".to_string());
    plan.set_cost(50);

    let output = plan.to_string();

    assert!(output.contains("Execution Plan: sample_op"));
    assert!(output.contains("1. First step"));
    assert!(output.contains("2. Second step"));
    assert!(output.contains("Estimated cost: 50 operations"));
    assert!(output.contains("Some optimization"));
}

#[test]
fn test_explain_shows_access_count() {
    let mut graph = Graph::new(GraphType::Directed);

    // Do a few lookups (below threshold)
    for _ in 0..5 {
        graph.find_nodes_by_property("age", &Value::Number(25.0));
    }

    let plan = graph.explain_find_property("age");

    // Should show access count progress toward threshold
    assert!(plan.to_string().contains("Access count: 5/10"));
}
