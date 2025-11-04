//! Integration Tests for Pattern Objects (Phase 9)
//!
//! Tests for pattern objects as first-class values:
//! 1. Built-in functions: node(), edge(), path()
//! 2. Pattern object methods: .bind()
//! 3. Pattern object properties: .variable, .type, .edge_type, etc.
//! 4. Programmatic pattern construction

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::execution::Executor;
use graphoid::values::Value;

/// Helper to execute code and return the last expression value
fn execute_and_return(code: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    let mut executor = Executor::new();

    // Execute all statements
    let statements = &program.statements;
    for stmt in statements.iter().take(statements.len().saturating_sub(1)) {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    // Execute the last statement and return its value
    if let Some(last_stmt) = statements.last() {
        match last_stmt {
            graphoid::ast::Stmt::Expression { expr, .. } => {
                executor
                    .eval_expr(&expr)
                    .map_err(|e| format!("Runtime error: {}", e))
            }
            _ => {
                executor
                    .eval_stmt(last_stmt)
                    .map_err(|e| format!("Runtime error: {}", e))?;
                Ok(Value::none())
            }
        }
    } else {
        Ok(Value::none())
    }
}

// ============================================================================
// Built-in Function Tests: node()
// ============================================================================

#[test]
fn test_node_with_variable_only() {
    let code = r#"
        pn = node("person")
        pn
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_node");
}

#[test]
fn test_node_with_variable_and_type() {
    let code = r#"
        pn = node("person", type: "User")
        pn
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_node");
}

#[test]
fn test_node_type_only() {
    let code = r#"
        pn = node(type: "User")
        pn
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_node");
}

#[test]
fn test_node_no_args() {
    let code = r#"
        pn = node()
        pn
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_node");
}

#[test]
fn test_node_invalid_param() {
    let code = r#"
        pn = node("person", invalid: "value")
        pn
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for invalid parameter");
    assert!(result.unwrap_err().contains("does not accept parameter 'invalid'"));
}

// ============================================================================
// Built-in Function Tests: edge()
// ============================================================================

#[test]
fn test_edge_no_args() {
    let code = r#"
        pe = edge()
        pe
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_edge");
}

#[test]
fn test_edge_with_type() {
    let code = r#"
        pe = edge(type: "FRIEND")
        pe
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_edge");
}

#[test]
fn test_edge_with_direction() {
    let code = r#"
        pe = edge(direction: :incoming)
        pe
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_edge");
}

#[test]
fn test_edge_with_type_and_direction() {
    let code = r#"
        pe = edge(type: "FRIEND", direction: :both)
        pe
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_edge");
}

#[test]
fn test_edge_positional_arg_fails() {
    let code = r#"
        pe = edge("FRIEND")
        pe
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for positional argument");
    assert!(result.unwrap_err().contains("does not accept positional arguments"));
}

#[test]
fn test_edge_invalid_direction_type() {
    let code = r#"
        pe = edge(direction: "outgoing")
        pe
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for non-symbol direction");
    assert!(result.unwrap_err().contains("direction must be a symbol"));
}

// ============================================================================
// Built-in Function Tests: path()
// ============================================================================

#[test]
fn test_path_required_params() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3)
        pp
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_path");
}

#[test]
fn test_path_with_direction() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3, direction: :incoming)
        pp
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap().type_name(), "pattern_path");
}

#[test]
fn test_path_missing_required_param() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1)
        pp
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for missing max parameter");
    assert!(result.unwrap_err().contains("requires 'max' parameter"));
}

#[test]
fn test_path_min_greater_than_max() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 5, max: 2)
        pp
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for min > max");
    let err = result.unwrap_err();
    assert!(err.contains("min") && err.contains("max"));
}

#[test]
fn test_path_positional_arg_fails() {
    let code = r#"
        pp = path("FOLLOWS", 1, 3)
        pp
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for positional arguments");
    assert!(result.unwrap_err().contains("does not accept positional arguments"));
}

// ============================================================================
// Pattern Object Property Tests
// ============================================================================

#[test]
fn test_pattern_node_variable_property() {
    let code = r#"
        pn = node("person", type: "User")
        pn.variable
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "person");
}

#[test]
fn test_pattern_node_type_property() {
    let code = r#"
        pn = node("person", type: "User")
        pn.type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "User");
}

#[test]
fn test_pattern_node_pattern_type_property() {
    let code = r#"
        pn = node("person")
        pn.pattern_type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    // Should be a symbol :node
    assert_eq!(value.type_name(), "symbol");
    assert_eq!(value.to_string_value(), ":node");
}

#[test]
fn test_pattern_edge_edge_type_property() {
    let code = r#"
        pe = edge(type: "FRIEND")
        pe.edge_type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "FRIEND");
}

#[test]
fn test_pattern_edge_direction_property() {
    let code = r#"
        pe = edge(direction: :incoming)
        pe.direction
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "symbol");
    assert_eq!(value.to_string_value(), ":incoming");
}

#[test]
fn test_pattern_edge_pattern_type_property() {
    let code = r#"
        pe = edge()
        pe.pattern_type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "symbol");
    assert_eq!(value.to_string_value(), ":edge");
}

#[test]
fn test_pattern_path_edge_type_property() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3)
        pp.edge_type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "FOLLOWS");
}

#[test]
fn test_pattern_path_min_property() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3)
        pp.min
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "num");
    match value.kind {
        graphoid::values::ValueKind::Number(n) => assert_eq!(n, 1.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_pattern_path_max_property() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3)
        pp.max
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "num");
    match value.kind {
        graphoid::values::ValueKind::Number(n) => assert_eq!(n, 3.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_pattern_path_direction_property() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3, direction: :both)
        pp.direction
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "symbol");
    assert_eq!(value.to_string_value(), ":both");
}

#[test]
fn test_pattern_path_pattern_type_property() {
    let code = r#"
        pp = path(edge_type: "FOLLOWS", min: 1, max: 3)
        pp.pattern_type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "symbol");
    assert_eq!(value.to_string_value(), ":path");
}

// ============================================================================
// Pattern Object Method Tests: .bind()
// ============================================================================

#[test]
fn test_pattern_node_bind() {
    let code = r#"
        user_node = node("person", type: "User")
        alice_node = user_node.bind("alice")
        alice_node.variable
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "alice");
}

#[test]
fn test_pattern_node_bind_preserves_type() {
    let code = r#"
        user_node = node("person", type: "User")
        alice_node = user_node.bind("alice")
        alice_node.type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "User");
}

#[test]
fn test_pattern_node_bind_chain() {
    let code = r#"
        user_node = node("person", type: "User")
        alice = user_node.bind("alice")
        bob = user_node.bind("bob")
        carol = user_node.bind("carol")
        carol.variable
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "carol");
}

#[test]
fn test_pattern_node_bind_no_arg_fails() {
    let code = r#"
        pn = node("person")
        pn.bind()
    "#;
    let result = execute_and_return(code);
    assert!(result.is_err(), "Expected error for missing argument");
    assert!(result.unwrap_err().contains("expects 1 argument"));
}

// ============================================================================
// Programmatic Pattern Construction Tests
// ============================================================================

#[test]
fn test_store_pattern_in_variable() {
    let code = r#"
        user_pattern = node("person", type: "User")
        friend_edge = edge(type: "FRIEND")

        # Should be able to access later
        user_pattern.type
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "User");
}

#[test]
fn test_pattern_objects_in_list() {
    let code = r#"
        patterns = [node("a", type: "User"), edge(type: "FRIEND"), node("b", type: "User")]
        patterns.size()
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    match value.kind {
        graphoid::values::ValueKind::Number(n) => assert_eq!(n, 3.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_pattern_object_reuse() {
    let code = r#"
        user_node = node("person", type: "User")

        # Reuse the same pattern with different variable names
        alice = user_node.bind("alice")
        bob = user_node.bind("bob")

        # Original should be unchanged
        user_node.variable
    "#;
    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.to_string_value(), "person");
}
