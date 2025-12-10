//! Integration test for Graphoid language syntax: graph.add_rule()
//!
//! This tests the END-TO-END use case with Graphoid code, not just Rust API.
//! Phase 7 of CLASS_LIKE_GRAPHS implementation.

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::ValueKind;

/// Helper function to execute source code and return the executor
fn execute_with_result(source: &str) -> Result<Executor, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();
    for stmt in &program.statements {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    Ok(executor)
}

/// Helper function to execute source code
fn execute(source: &str) -> Result<(), String> {
    execute_with_result(source)?;
    Ok(())
}

// =============================================================================
// Basic add_rule tests
// =============================================================================

#[test]
fn test_graph_add_rule_no_cycles() {
    // Create a graph and add no_cycles rule using Graphoid syntax
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_rule(:no_cycles)
    "#;

    execute(code).expect("Should execute successfully");
}

#[test]
fn test_graph_add_rule_single_root() {
    let code = r#"
        g = graph{}
        g.add_node("root", 1)
        g.add_rule(:single_root)
    "#;

    execute(code).expect("Should execute successfully");
}

#[test]
fn test_graph_add_rule_connected() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_rule(:connected)
    "#;

    execute(code).expect("Should execute successfully");
}

#[test]
fn test_graph_add_rule_with_parameter() {
    // Test max_degree with parameter
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_rule(:max_degree, 2)
    "#;

    execute(code).expect("Should execute successfully");
}

// =============================================================================
// Rule enforcement tests
// =============================================================================

#[test]
fn test_graph_no_cycles_prevents_cycle() {
    // Add rule, then try to create a cycle
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")
        g.add_rule(:no_cycles)
        g.add_edge("C", "A")
    "#;

    let result = execute(code);
    assert!(result.is_err(), "Should reject cycle creation");
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("cycle") || err_msg.contains("Cycle"),
        "Error should mention cycle: {}",
        err_msg
    );
}

#[test]
fn test_graph_max_degree_enforced() {
    // Add max_degree rule, then try to exceed it
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_node("D", 4)
        g.add_rule(:max_degree, 2)
        g.add_edge("A", "B")
        g.add_edge("A", "C")
        g.add_edge("A", "D")
    "#;

    let result = execute(code);
    assert!(result.is_err(), "Should reject exceeding max degree");
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("degree") || err_msg.contains("Degree") || err_msg.contains("children") || err_msg.contains("outgoing"),
        "Error should mention degree limit: {}",
        err_msg
    );
}

#[test]
fn test_graph_binary_tree_rule() {
    // Binary tree allows max 2 children
    let code = r#"
        g = graph{}
        g.add_node("root", 1)
        g.add_node("left", 2)
        g.add_node("right", 3)
        g.add_node("extra", 4)
        g.add_rule(:binary_tree)
        g.add_edge("root", "left")
        g.add_edge("root", "right")
        g.add_edge("root", "extra")
    "#;

    let result = execute(code);
    assert!(result.is_err(), "Should reject third child in binary tree");
}

// =============================================================================
// Rules scope to data layer (skip __methods__ branch)
// =============================================================================

#[test]
fn test_rules_ignore_methods_branch() {
    // Define a method, add rules - rules should only apply to data nodes
    let code = r#"
        g = graph{}

        fn g.get_count() {
            return self.node_count()
        }

        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B")

        g.add_rule(:no_cycles)

        count = g.get_count()
    "#;

    let exec = execute_with_result(code).expect("Should execute with methods and rules");

    // Verify method still works
    let count = exec.env().get("count").expect("Should have count variable");
    if let ValueKind::Number(n) = count.kind {
        assert_eq!(n, 2.0, "node_count should be 2 (data nodes only)");
    } else {
        panic!("count should be a number");
    }
}

#[test]
fn test_methods_branch_not_checked_for_cycles() {
    // The __methods__ branch has cycles (method nodes reference function values)
    // But that shouldn't affect the no_cycles rule on the data layer
    let code = r#"
        g = graph{}

        fn g.method_a() {
            return 1
        }

        fn g.method_b() {
            return 2
        }

        g.add_node("X", 10)
        g.add_node("Y", 20)
        g.add_edge("X", "Y")

        g.add_rule(:no_cycles)

        result = g.method_a() + g.method_b()
    "#;

    let exec = execute_with_result(code).expect("Methods should work with no_cycles rule");

    let result = exec.env().get("result").expect("Should have result");
    if let ValueKind::Number(n) = result.kind {
        assert_eq!(n, 3.0);
    } else {
        panic!("result should be 3");
    }
}

// =============================================================================
// has_rule and remove_rule
// =============================================================================

#[test]
fn test_graph_has_rule() {
    let code = r#"
        g = graph{}
        g.add_rule(:no_cycles)
        has_it = g.has_rule(:no_cycles)
        doesnt_have = g.has_rule(:connected)
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_it = exec.env().get("has_it").expect("Should have has_it");
    assert!(matches!(has_it.kind, ValueKind::Boolean(true)));

    let doesnt_have = exec.env().get("doesnt_have").expect("Should have doesnt_have");
    assert!(matches!(doesnt_have.kind, ValueKind::Boolean(false)));
}

#[test]
fn test_graph_remove_rule() {
    // Add rule, remove it, then the previously forbidden operation should work
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")
        g.add_rule(:no_cycles)
        g.remove_rule(:no_cycles)
        g.add_edge("C", "A")
    "#;

    execute(code).expect("Should allow cycle after rule removal");
}

// =============================================================================
// Error handling
// =============================================================================

#[test]
fn test_graph_unknown_rule_error() {
    let code = r#"
        g = graph{}
        g.add_rule(:unknown_rule_xyz)
    "#;

    let result = execute(code);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("Unknown rule") || err_msg.contains("unknown"),
        "Should report unknown rule: {}",
        err_msg
    );
}

#[test]
fn test_graph_add_rule_wrong_arg_type() {
    let code = r#"
        g = graph{}
        g.add_rule("no_cycles")
    "#;

    let result = execute(code);
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(
        err_msg.contains("symbol") || err_msg.contains("Symbol"),
        "Should require symbol argument: {}",
        err_msg
    );
}

// =============================================================================
// Combination with rulesets
// =============================================================================

#[test]
fn test_ad_hoc_rule_plus_ruleset() {
    // Apply ruleset, then add additional ad hoc rule
    let code = r#"
        g = graph{}.with_ruleset(:dag)
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_rule(:max_degree, 3)
        has_dag = g.has_ruleset(:dag)
        has_max = g.has_rule(:max_degree)
    "#;

    let exec = execute_with_result(code).expect("Should combine ruleset with ad hoc rule");

    let has_dag = exec.env().get("has_dag").expect("Should have has_dag");
    assert!(matches!(has_dag.kind, ValueKind::Boolean(true)));

    let has_max = exec.env().get("has_max").expect("Should have has_max");
    assert!(matches!(has_max.kind, ValueKind::Boolean(true)));
}

// =============================================================================
// rule() tests
// =============================================================================

#[test]
fn test_rule_returns_param_value() {
    // rule() should return the parameter value for parameterized rules
    let code = r#"
        g = graph{}
        g.add_rule(:max_degree, 5)
        val = g.rule(:max_degree)
    "#;

    let exec = execute_with_result(code).expect("Should execute");
    let val = exec.env().get("val").expect("Should have val");
    if let ValueKind::Number(n) = val.kind {
        assert_eq!(n, 5.0, "rule(:max_degree) should return 5");
    } else {
        panic!("Expected number, got {:?}", val.kind);
    }
}

#[test]
fn test_rule_returns_true_for_simple_rule() {
    // rule() should return true for non-parameterized rules
    let code = r#"
        g = graph{}
        g.add_rule(:no_cycles)
        val = g.rule(:no_cycles)
    "#;

    let exec = execute_with_result(code).expect("Should execute");
    let val = exec.env().get("val").expect("Should have val");
    assert!(matches!(val.kind, ValueKind::Boolean(true)), "rule(:no_cycles) should return true");
}

#[test]
fn test_rule_returns_none_for_missing() {
    // rule() should return none for rules that don't exist
    let code = r#"
        g = graph{}
        val = g.rule(:nonexistent)
        is_none = val == none
    "#;

    let exec = execute_with_result(code).expect("Should execute");
    let is_none = exec.env().get("is_none").expect("Should have is_none");
    assert!(matches!(is_none.kind, ValueKind::Boolean(true)), "rule() for missing rule should return none");
}
