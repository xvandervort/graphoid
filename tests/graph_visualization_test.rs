//! Integration test for graph visualization methods
//!
//! Phase 9: visualize(), to_dot(), to_ascii()

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

// =============================================================================
// visualize() tests
// =============================================================================

#[test]
fn test_visualize_returns_string() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "connected")

        viz = g.visualize()
    "#;

    let exec = execute_with_result(code).expect("Should execute");
    let viz = exec.env().get("viz").expect("Should have viz");
    assert!(matches!(viz.kind, ValueKind::String(_)), "visualize() should return a string");
}

#[test]
fn test_visualize_contains_nodes() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 100)
        g.add_node("Bob", 200)

        viz = g.visualize()
        has_alice = viz.contains("Alice")
        has_bob = viz.contains("Bob")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_alice = exec.env().get("has_alice").expect("Should have has_alice");
    assert!(matches!(has_alice.kind, ValueKind::Boolean(true)), "Should contain Alice");

    let has_bob = exec.env().get("has_bob").expect("Should have has_bob");
    assert!(matches!(has_bob.kind, ValueKind::Boolean(true)), "Should contain Bob");
}

#[test]
fn test_visualize_contains_edges() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "likes")

        viz = g.visualize()
        has_edge_info = viz.contains("likes") or viz.contains("->")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_edge_info = exec.env().get("has_edge_info").expect("Should have has_edge_info");
    assert!(matches!(has_edge_info.kind, ValueKind::Boolean(true)), "Should contain edge info");
}

#[test]
fn test_visualize_data_only_by_default() {
    let code = r#"
        graph G {
            fn my_method() {
                return 42
            }
        }

        g = G.clone()
        g.add_node("data_node", 1)

        viz = g.visualize()
        has_data = viz.contains("data_node")
        has_methods = viz.contains("__methods__")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_data = exec.env().get("has_data").expect("Should have has_data");
    assert!(matches!(has_data.kind, ValueKind::Boolean(true)), "Should contain data node");

    let has_methods = exec.env().get("has_methods").expect("Should have has_methods");
    assert!(matches!(has_methods.kind, ValueKind::Boolean(false)), "Should NOT contain __methods__ by default");
}

#[test]
fn test_visualize_all_includes_methods() {
    let code = r#"
        graph G {
            fn my_method() {
                return 42
            }
        }

        g = G.clone()
        g.add_node("data_node", 1)

        viz = g.visualize(:all)
        has_methods = viz.contains("__methods__")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_methods = exec.env().get("has_methods").expect("Should have has_methods");
    assert!(matches!(has_methods.kind, ValueKind::Boolean(true)), "visualize(:all) should include __methods__");
}

// =============================================================================
// to_dot() tests
// =============================================================================

#[test]
fn test_to_dot_returns_string() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B")

        dot = g.to_dot()
    "#;

    let exec = execute_with_result(code).expect("Should execute");
    let dot = exec.env().get("dot").expect("Should have dot");
    assert!(matches!(dot.kind, ValueKind::String(_)), "to_dot() should return a string");
}

#[test]
fn test_to_dot_has_digraph_header() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)

        dot = g.to_dot()
        has_digraph = dot.contains("digraph")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_digraph = exec.env().get("has_digraph").expect("Should have has_digraph");
    assert!(matches!(has_digraph.kind, ValueKind::Boolean(true)), "DOT output should contain 'digraph'");
}

#[test]
fn test_to_dot_has_edge_syntax() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "connected")

        dot = g.to_dot()
        has_arrow = dot.contains("->")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_arrow = exec.env().get("has_arrow").expect("Should have has_arrow");
    assert!(matches!(has_arrow.kind, ValueKind::Boolean(true)), "DOT output should contain '->'");
}

#[test]
fn test_to_dot_has_edge_label() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "friendship")

        dot = g.to_dot()
        has_label = dot.contains("friendship")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_label = exec.env().get("has_label").expect("Should have has_label");
    assert!(matches!(has_label.kind, ValueKind::Boolean(true)), "DOT output should contain edge label");
}

#[test]
fn test_to_dot_data_only_by_default() {
    let code = r#"
        graph G {
            fn method() {
                return 0
            }
        }

        g = G.clone()
        g.add_node("data", 1)

        dot = g.to_dot()
        has_methods = dot.contains("__methods__")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_methods = exec.env().get("has_methods").expect("Should have has_methods");
    assert!(matches!(has_methods.kind, ValueKind::Boolean(false)), "to_dot() should not include __methods__ by default");
}

// =============================================================================
// to_ascii() tests
// =============================================================================

#[test]
fn test_to_ascii_returns_string() {
    let code = r#"
        g = graph{}
        g.add_node("root", 1)
        g.add_node("child", 2)
        g.add_edge("root", "child")

        ascii = g.to_ascii()
    "#;

    let exec = execute_with_result(code).expect("Should execute");
    let ascii = exec.env().get("ascii").expect("Should have ascii");
    assert!(matches!(ascii.kind, ValueKind::String(_)), "to_ascii() should return a string");
}

#[test]
fn test_to_ascii_contains_nodes() {
    let code = r#"
        g = graph{}
        g.add_node("parent", 1)
        g.add_node("child1", 2)
        g.add_node("child2", 3)
        g.add_edge("parent", "child1")
        g.add_edge("parent", "child2")

        ascii = g.to_ascii()
        has_parent = ascii.contains("parent")
        has_child1 = ascii.contains("child1")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_parent = exec.env().get("has_parent").expect("Should have has_parent");
    assert!(matches!(has_parent.kind, ValueKind::Boolean(true)));

    let has_child1 = exec.env().get("has_child1").expect("Should have has_child1");
    assert!(matches!(has_child1.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_to_ascii_shows_hierarchy() {
    let code = r#"
        g = graph{}
        g.add_node("root", 1)
        g.add_node("child", 2)
        g.add_edge("root", "child")

        ascii = g.to_ascii()
        # Should have tree structure characters (Unicode box-drawing or newlines)
        has_root = ascii.contains("root")
        has_child = ascii.contains("child")
        has_newline = ascii.contains("\n")
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let has_root = exec.env().get("has_root").expect("Should have has_root");
    assert!(matches!(has_root.kind, ValueKind::Boolean(true)), "Should contain root");

    let has_child = exec.env().get("has_child").expect("Should have has_child");
    assert!(matches!(has_child.kind, ValueKind::Boolean(true)), "Should contain child");

    let has_newline = exec.env().get("has_newline").expect("Should have has_newline");
    assert!(matches!(has_newline.kind, ValueKind::Boolean(true)), "Should have structure (newlines)");
}

// =============================================================================
// Empty graph tests
// =============================================================================

#[test]
fn test_visualize_empty_graph() {
    let code = r#"
        g = graph{}
        viz = g.visualize()
    "#;

    let exec = execute_with_result(code).expect("Should handle empty graph");
    let viz = exec.env().get("viz").expect("Should have viz");
    assert!(matches!(viz.kind, ValueKind::String(_)));
}

#[test]
fn test_to_dot_empty_graph() {
    let code = r#"
        g = graph{}
        dot = g.to_dot()
        has_digraph = dot.contains("digraph")
    "#;

    let exec = execute_with_result(code).expect("Should handle empty graph");
    let has_digraph = exec.env().get("has_digraph").expect("Should have has_digraph");
    assert!(matches!(has_digraph.kind, ValueKind::Boolean(true)), "Empty graph should still have digraph header");
}

#[test]
fn test_to_ascii_empty_graph() {
    let code = r#"
        g = graph{}
        ascii = g.to_ascii()
    "#;

    let exec = execute_with_result(code).expect("Should handle empty graph");
    let ascii = exec.env().get("ascii").expect("Should have ascii");
    assert!(matches!(ascii.kind, ValueKind::String(_)));
}
