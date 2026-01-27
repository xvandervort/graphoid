//! Tests for graph equality - data layer only comparison
//!
//! Graph equality should compare DATA only by default, not metadata.
//! This means:
//! - Rules attached to a graph do NOT affect equality
//! - Rulesets do NOT affect equality (but graph_type DOES)
//! - Methods attached do NOT affect equality
//! - Only data nodes and their values/edges matter

use graphoid::values::{Value, List, Graph, GraphType};
use graphoid::graph::{RuleSpec, RuleInstance};

// ============================================================================
// LIST EQUALITY TESTS
// ============================================================================

#[test]
fn test_list_with_rule_equals_list_without() {
    // Two lists with same values should be equal regardless of rules
    let list1 = List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(3.0),
    ]);

    let mut list2 = List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(3.0),
    ]);
    list2.add_rule(RuleInstance::new(RuleSpec::NoneToZero)).unwrap();

    // Should be equal - rules don't affect data-layer comparison
    assert_eq!(list1, list2, "Lists with same values should be equal despite different rules");
}

#[test]
fn test_list_with_multiple_rules_equals_list_without() {
    let list1 = List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(3.0),
    ]);

    let mut list2 = List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(3.0),
    ]);
    list2.add_rule(RuleInstance::new(RuleSpec::NoneToZero)).unwrap();
    list2.add_rule(RuleInstance::new(RuleSpec::Positive)).unwrap();

    assert_eq!(list1, list2, "Lists with same values should be equal despite multiple rules");
}

#[test]
fn test_list_after_transformation_equals_expected() {
    // A list with none_to_zero rule should equal a list with the transformed values
    let mut list_with_rule = List::from_vec(vec![
        Value::number(1.0),
        Value::none(),
        Value::number(3.0),
    ]);
    list_with_rule.add_rule(RuleInstance::new(RuleSpec::NoneToZero)).unwrap();

    let expected = List::from_vec(vec![
        Value::number(1.0),
        Value::number(0.0),
        Value::number(3.0),
    ]);

    // The rule should have transformed none to 0, so values should match
    assert_eq!(list_with_rule, expected, "List after transformation should equal expected values");
}

#[test]
fn test_lists_with_different_values_not_equal() {
    let list1 = List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(3.0),
    ]);

    let list2 = List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(4.0),
    ]);

    assert_ne!(list1, list2, "Lists with different values should not be equal");
}

#[test]
fn test_empty_list_with_rule_equals_empty_list_without() {
    let list1 = List::new();

    let mut list2 = List::new();
    list2.add_rule(RuleInstance::new(RuleSpec::NoneToZero)).unwrap();

    assert_eq!(list1, list2, "Empty lists should be equal regardless of rules");
}

// ============================================================================
// GRAPH EQUALITY TESTS
// ============================================================================

#[test]
fn test_graph_with_ruleset_equals_graph_without() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_node("b".to_string(), Value::number(2.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_node("b".to_string(), Value::number(2.0)).unwrap();

    assert_eq!(g1, g2, "Graphs with same nodes should be equal despite different rulesets");
}

#[test]
fn test_graph_with_methods_equals_graph_without() {
    use graphoid::values::Function;
    use graphoid::execution::Environment;
    use std::rc::Rc;
    use std::cell::RefCell;

    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("value".to_string(), Value::number(42.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("value".to_string(), Value::number(42.0)).unwrap();

    // Add a method to g2
    let func = Function {
        name: Some("get_value".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g2.attach_method("get_value".to_string(), func);

    assert_eq!(g1, g2, "Graphs with same data nodes should be equal despite attached methods");
}

#[test]
fn test_graphs_with_different_nodes_not_equal() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(2.0)).unwrap();

    assert_ne!(g1, g2, "Graphs with different node values should not be equal");
}

#[test]
fn test_graphs_with_different_types_not_equal() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Undirected);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();

    assert_ne!(g1, g2, "Graphs with different types should not be equal");
}

#[test]
fn test_empty_graphs_with_different_rulesets_equal() {
    let g1 = Graph::new(GraphType::Directed);
    let g2 = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());

    assert_eq!(g1, g2, "Empty graphs should be equal regardless of rulesets");
}

#[test]
fn test_graph_with_properties_only_compares_data() {
    // This tests that __properties__/ nodes don't affect equality
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("data".to_string(), Value::number(1.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("data".to_string(), Value::number(1.0)).unwrap();
    // Manually add a property node (simulating CLG property)
    g2.nodes.insert(
        "__properties__/name".to_string(),
        graphoid::values::GraphNode {
            id: "__properties__/name".to_string(),
            value: Value::string("test".to_string()),
            node_type: Some("property".to_string()),
            properties: std::collections::HashMap::new(),
            neighbors: std::collections::HashMap::new(),
            predecessors: std::collections::HashMap::new(),
        }
    );

    assert_eq!(g1, g2, "Graphs should be equal when data nodes match, ignoring __properties__/");
}

// ============================================================================
// EQUALS METHOD WITH LAYER OPTIONS TESTS
// ============================================================================

use graphoid::values::ComparisonLayer;
use std::collections::HashSet;

// --- include: tests (data + additional layers) ---

#[test]
fn test_equals_include_rules_same_rules() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rules].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs with same data and same rules should be equal with include: :rules");
}

#[test]
fn test_equals_include_rules_different_rules() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_rule(RuleInstance::new(RuleSpec::Positive)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rules].into_iter().collect();
    assert!(!g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs with same data but different rules should NOT be equal with include: :rules");
}

#[test]
fn test_equals_include_rules_one_has_rules() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rules].into_iter().collect();
    assert!(!g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs where only one has rules should NOT be equal with include: :rules");
}

#[test]
fn test_equals_include_rulesets_same() {
    let mut g1 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rulesets].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs with same rulesets should be equal with include: :rulesets");
}

#[test]
fn test_equals_include_rulesets_different() {
    let mut g1 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rulesets].into_iter().collect();
    assert!(!g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs with different rulesets should NOT be equal with include: :rulesets");
}

#[test]
fn test_equals_include_methods_same() {
    use graphoid::values::Function;
    use graphoid::execution::Environment;
    use std::rc::Rc;
    use std::cell::RefCell;

    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    let func1 = Function {
        name: Some("test".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g1.attach_method("test".to_string(), func1);

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    let func2 = Function {
        name: Some("test".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g2.attach_method("test".to_string(), func2);

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Methods].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs with same method names should be equal with include: :methods");
}

#[test]
fn test_equals_include_methods_different() {
    use graphoid::values::Function;
    use graphoid::execution::Environment;
    use std::rc::Rc;
    use std::cell::RefCell;

    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    let func1 = Function {
        name: Some("foo".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g1.attach_method("foo".to_string(), func1);

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    let func2 = Function {
        name: Some("bar".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g2.attach_method("bar".to_string(), func2);

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Methods].into_iter().collect();
    assert!(!g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs with different method names should NOT be equal with include: :methods");
}

#[test]
fn test_equals_include_multiple_layers() {
    let mut g1 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rules, ComparisonLayer::Rulesets].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs should be equal when all specified layers match");
}

#[test]
fn test_equals_include_all() {
    let mut g1 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::All].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), false),
        "Graphs should be equal with include: :all when everything matches");
}

// --- only: tests (specific layer only, excluding data) ---

#[test]
fn test_equals_only_rules_same() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("b".to_string(), Value::number(999.0)).unwrap(); // Different data!
    g2.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rules].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), true),
        "Graphs with same rules should be equal with only: :rules (ignoring data)");
}

#[test]
fn test_equals_only_rules_different() {
    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap(); // Same data
    g2.add_rule(RuleInstance::new(RuleSpec::Positive)).unwrap(); // Different rule

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rules].into_iter().collect();
    assert!(!g1.equals_with_layers(&g2, Some(&layers), true),
        "Graphs with different rules should NOT be equal with only: :rules");
}

#[test]
fn test_equals_only_rulesets() {
    let mut g1 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g1.add_node("x".to_string(), Value::number(100.0)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g2.add_node("y".to_string(), Value::number(200.0)).unwrap(); // Different data

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Rulesets].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), true),
        "Graphs with same rulesets should be equal with only: :rulesets (ignoring data)");
}

#[test]
fn test_equals_only_methods() {
    use graphoid::values::Function;
    use graphoid::execution::Environment;
    use std::rc::Rc;
    use std::cell::RefCell;

    let mut g1 = Graph::new(GraphType::Directed);
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    let func1 = Function {
        name: Some("calculate".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g1.attach_method("calculate".to_string(), func1);

    let mut g2 = Graph::new(GraphType::Directed);
    g2.add_node("z".to_string(), Value::string("different".to_string())).unwrap(); // Different data
    let func2 = Function {
        name: Some("calculate".to_string()),
        params: vec![],
        parameters: vec![],
        body: vec![],
        pattern_clauses: None,
        env: Rc::new(RefCell::new(Environment::new())),
        node_id: None,
        is_setter: false,
        is_static: false,
        guard: None,
    };
    g2.attach_method("calculate".to_string(), func2);

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Methods].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), true),
        "Graphs with same methods should be equal with only: :methods (ignoring data)");
}

#[test]
fn test_equals_only_data() {
    // only: :data should compare just data, ignoring everything else
    let mut g1 = Graph::new(GraphType::Directed).with_ruleset("dag".to_string());
    g1.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g1.add_rule(RuleInstance::new(RuleSpec::NoDuplicates)).unwrap();

    let mut g2 = Graph::new(GraphType::Directed).with_ruleset("tree".to_string()); // Different ruleset
    g2.add_node("a".to_string(), Value::number(1.0)).unwrap();
    g2.add_rule(RuleInstance::new(RuleSpec::Positive)).unwrap(); // Different rule

    let layers: HashSet<ComparisonLayer> = [ComparisonLayer::Data].into_iter().collect();
    assert!(g1.equals_with_layers(&g2, Some(&layers), true),
        "Graphs with same data should be equal with only: :data (ignoring rules/rulesets)");
}
