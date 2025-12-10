//! Integration test for graph layer visibility
//!
//! Phase 8: nodes() and edges() should hide internal layers by default,
//! with :all option to show everything.

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
// nodes() visibility tests
// =============================================================================

#[test]
fn test_nodes_hides_methods_by_default() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)

        fn g.my_method() {
            return 42
        }

        node_list = g.nodes()
        count = node_list.len()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let count = exec.env().get("count").expect("Should have count");
    if let ValueKind::Number(n) = count.kind {
        assert_eq!(n, 2.0, "nodes() should return only data nodes (A, B), not method nodes");
    } else {
        panic!("count should be a number");
    }
}

#[test]
fn test_nodes_all_includes_methods() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)

        fn g.method1() {
            return 1
        }

        fn g.method2() {
            return 2
        }

        # :all should include __methods__ branch and method nodes
        all_nodes = g.nodes(:all)
        all_count = all_nodes.len()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let all_count = exec.env().get("all_count").expect("Should have all_count");
    if let ValueKind::Number(n) = all_count.kind {
        // 2 data nodes + 1 __methods__ container + 2 method nodes = 5
        assert!(n >= 5.0, "nodes(:all) should include method nodes, got {}", n);
    } else {
        panic!("all_count should be a number");
    }
}

#[test]
fn test_node_count_data_only() {
    let code = r#"
        g = graph{}
        g.add_node("X", 10)
        g.add_node("Y", 20)
        g.add_node("Z", 30)

        fn g.helper() {
            return self.get_node("X")
        }

        count = g.node_count()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let count = exec.env().get("count").expect("Should have count");
    if let ValueKind::Number(n) = count.kind {
        assert_eq!(n, 3.0, "node_count() should return 3 (data nodes only)");
    } else {
        panic!("count should be a number");
    }
}

// =============================================================================
// edges() visibility tests
// =============================================================================

#[test]
fn test_edges_hides_method_edges_by_default() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "connected")

        fn g.get_a() {
            return self.get_node("A")
        }

        edge_list = g.edges()
        edge_count = edge_list.len()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let edge_count = exec.env().get("edge_count").expect("Should have edge_count");
    if let ValueKind::Number(n) = edge_count.kind {
        assert_eq!(n, 1.0, "edges() should return only data edges");
    } else {
        panic!("edge_count should be a number");
    }
}

#[test]
fn test_edges_all_includes_method_edges() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "connected")

        fn g.method1() {
            return 1
        }

        # :all should include edges to method nodes
        all_edges = g.edges(:all)
        all_edge_count = all_edges.len()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let all_edge_count = exec.env().get("all_edge_count").expect("Should have all_edge_count");
    if let ValueKind::Number(n) = all_edge_count.kind {
        // 1 data edge + at least 1 method edge (__methods__ -> method1)
        assert!(n >= 2.0, "edges(:all) should include method edges, got {}", n);
    } else {
        panic!("all_edge_count should be a number");
    }
}

#[test]
fn test_edge_count_data_only() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")

        fn g.method() {
            return 0
        }

        count = g.edge_count()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let count = exec.env().get("count").expect("Should have count");
    if let ValueKind::Number(n) = count.kind {
        assert_eq!(n, 2.0, "edge_count() should return 2 (data edges only)");
    } else {
        panic!("count should be a number");
    }
}

// =============================================================================
// Edge format tests
// =============================================================================

#[test]
fn test_edges_returns_readable_format() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "connected")

        edges = g.edges()
        first_edge = edges[0]
        from_node = first_edge[0]
        to_node = first_edge[1]
        edge_type = first_edge[2]
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let from_node = exec.env().get("from_node").expect("Should have from_node");
    let to_node = exec.env().get("to_node").expect("Should have to_node");
    let edge_type = exec.env().get("edge_type").expect("Should have edge_type");

    assert!(matches!(from_node.kind, ValueKind::String(ref s) if s == "A"));
    assert!(matches!(to_node.kind, ValueKind::String(ref s) if s == "B"));
    assert!(matches!(edge_type.kind, ValueKind::String(ref s) if s == "connected"));
}

// =============================================================================
// Consistency tests
// =============================================================================

#[test]
fn test_nodes_and_node_count_consistent() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)

        fn g.test() {
            return 0
        }

        nodes_len = g.nodes().len()
        node_count_val = g.node_count()
        consistent = nodes_len == node_count_val
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let consistent = exec.env().get("consistent").expect("Should have consistent");
    assert!(matches!(consistent.kind, ValueKind::Boolean(true)),
            "nodes().len() should equal node_count()");
}

#[test]
fn test_edges_and_edge_count_consistent() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")

        fn g.test() {
            return 0
        }

        edges_len = g.edges().len()
        edge_count_val = g.edge_count()
        consistent = edges_len == edge_count_val
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let consistent = exec.env().get("consistent").expect("Should have consistent");
    assert!(matches!(consistent.kind, ValueKind::Boolean(true)),
            "edges().len() should equal edge_count()");
}

// =============================================================================
// Graph without methods (baseline)
// =============================================================================

#[test]
fn test_graph_without_methods_unchanged() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B")

        # No methods - nodes() and nodes(:all) should be same
        regular_count = g.nodes().len()
        all_count = g.nodes(:all).len()
    "#;

    let exec = execute_with_result(code).expect("Should execute");

    let regular_count = exec.env().get("regular_count").expect("Should have regular_count");
    let all_count = exec.env().get("all_count").expect("Should have all_count");

    if let (ValueKind::Number(r), ValueKind::Number(a)) = (&regular_count.kind, &all_count.kind) {
        assert_eq!(*r, *a, "Without methods, nodes() and nodes(:all) should be equal");
    } else {
        panic!("Counts should be numbers");
    }
}
