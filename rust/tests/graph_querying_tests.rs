//! Tests for Enhanced Graph Querying
//! Phase 6.5 Area 5

use graphoid::ast;
use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::{Value, List};

fn eval(code: &str) -> Value {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for (i, stmt) in program.statements.iter().enumerate() {
        if i == program.statements.len() - 1 {
            if let ast::Stmt::Expression { expr, .. } = stmt {
                return executor.eval_expr(expr).unwrap();
            }
        }
        executor.eval_stmt(stmt).unwrap();
    }

    Value::None
}

fn list_strings(strs: Vec<&str>) -> Value {
    Value::List(List::from_vec(
        strs.into_iter().map(|s| Value::String(s.to_string())).collect()
    ))
}

// ============================================================================
// HAS_PATH - Boolean path existence
// ============================================================================

#[test]
fn test_has_path_direct_edge() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B")
        g.has_path("A", "B")
    "#;
    assert_eq!(eval(code), Value::Boolean(true));
}

#[test]
fn test_has_path_indirect() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")
        g.has_path("A", "C")
    "#;
    assert_eq!(eval(code), Value::Boolean(true));
}

#[test]
fn test_has_path_no_path() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.has_path("B", "A")
    "#;
    // Directed graph - no path from B to A
    assert_eq!(eval(code), Value::Boolean(false));
}

#[test]
fn test_has_path_self() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.has_path("A", "A")
    "#;
    // Path from node to itself (zero edges)
    assert_eq!(eval(code), Value::Boolean(true));
}

// ============================================================================
// DISTANCE - Shortest path length
// ============================================================================

#[test]
fn test_distance_direct_edge() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B")
        g.distance("A", "B")
    "#;
    assert_eq!(eval(code), Value::Number(1.0));
}

#[test]
fn test_distance_two_hops() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")
        g.distance("A", "C")
    "#;
    assert_eq!(eval(code), Value::Number(2.0));
}

#[test]
fn test_distance_shortest_when_multiple_paths() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_node("D", 4)
        g.add_edge("A", "B")
        g.add_edge("B", "C")
        g.add_edge("C", "D")
        g.add_edge("A", "D")
        g.distance("A", "D")
    "#;
    // Direct path A->D is 1, indirect A->B->C->D is 3
    assert_eq!(eval(code), Value::Number(1.0));
}

#[test]
fn test_distance_no_path() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.distance("A", "B")
    "#;
    // No path exists - should return -1 or none
    assert_eq!(eval(code), Value::Number(-1.0));
}

// ============================================================================
// ALL_PATHS - Find all paths up to max length
// ============================================================================

#[test]
fn test_all_paths_single_path() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B")
        g.all_paths("A", "B", 5)
    "#;
    // Should return list of paths: [["A", "B"]]
    let result = eval(code);

    // Check it's a list with one element
    if let Value::List(paths) = result {
        assert_eq!(paths.len(), 1);

        // First path should be ["A", "B"]
        let path = paths.get(0).unwrap();
        assert_eq!(path, &list_strings(vec!["A", "B"]));
    } else {
        panic!("Expected list of paths, got {:?}", result);
    }
}

#[test]
fn test_all_paths_multiple_paths() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("A", "C")
        g.add_edge("B", "C")
        g.all_paths("A", "C", 5)
    "#;
    // Should return: [["A", "C"], ["A", "B", "C"]]
    let result = eval(code);

    if let Value::List(paths) = result {
        assert_eq!(paths.len(), 2);
    } else {
        panic!("Expected list of paths");
    }
}

#[test]
fn test_all_paths_respects_max_length() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B")
        g.add_edge("B", "C")
        g.all_paths("A", "C", 1)
    "#;
    // Max length 1 means only paths with 1 edge
    // A->B->C has 2 edges, so shouldn't be included
    let result = eval(code);

    if let Value::List(paths) = result {
        assert_eq!(paths.len(), 0); // No paths with length <= 1
    } else {
        panic!("Expected list of paths");
    }
}

#[test]
fn test_all_paths_no_paths() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.all_paths("A", "B", 5)
    "#;
    // No edges, so no paths
    let result = eval(code);

    if let Value::List(paths) = result {
        assert_eq!(paths.len(), 0);
    } else {
        panic!("Expected list of paths");
    }
}
