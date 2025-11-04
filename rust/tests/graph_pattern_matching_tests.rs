//! Integration Tests for Graph Pattern Matching - Phase 9
//!
//! These tests verify explicit syntax graph pattern matching using pattern objects:
//! - g.match(node(...), edge(...), node(...))
//! - Pattern matching tests - matching patterns against graphs
//! - Subgraph operation tests - extract, delete, add (future)

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::execution::Executor;
use graphoid::values::Value;
use graphoid::ast::Stmt;

/// Helper to execute code and return the value of the last expression
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

    // Execute all statements except the last
    let statements = &program.statements;
    for stmt in statements.iter().take(statements.len().saturating_sub(1)) {
        executor
            .eval_stmt(stmt)
            .map_err(|e| format!("Runtime error: {}", e))?;
    }

    // Execute the last statement and return its value
    if let Some(last_stmt) = statements.last() {
        match last_stmt {
            Stmt::Expression { expr, .. } => {
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
// Explicit Pattern Syntax Tests (Pattern Objects in .match())
// ============================================================================

#[test]
fn test_explicit_syntax_simple_pattern() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 1)
        g.add_node("Bob", 2)
        g.add_edge("Alice", "Bob", "FRIEND")

        results = g.match(node("person"), edge(type: "FRIEND"), node("friend"))
        results
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    // Should return a list of matches
    let value = result.unwrap();
    assert_eq!(value.type_name(), "list");
}

#[test]
fn test_explicit_syntax_with_node_types() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 1)
        g.add_node("Bob", 2)
        g.add_edge("Alice", "Bob", "FRIEND")

        # Pattern with node types (matching will be implemented later)
        results = g.match(node("person", type: "User"), edge(type: "FRIEND"), node("friend", type: "User"))
        results
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "list");
}

#[test]
fn test_explicit_syntax_reusable_patterns() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 1)
        g.add_node("Bob", 2)
        g.add_edge("Alice", "Bob", "FRIEND")

        # Create reusable pattern
        user_node = node("person", type: "User")

        # Reuse pattern with .bind() method
        results = g.match(user_node.bind("alice"), edge(type: "FRIEND"), user_node.bind("bob"))
        results
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value.type_name(), "list");
}

// ============================================================================
// Pattern Matching Execution Tests - Day 3-5 (TDD RED -> GREEN -> REFACTOR)
// ============================================================================

#[test]
fn test_simple_pattern_match() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 1)
        g.add_node("Bob", 2)
        g.add_edge("Alice", "Bob", "FRIEND")

        results = g.match(node("person"), edge(type: "FRIEND"), node("friend"))
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    match value.kind {
        graphoid::values::ValueKind::Number(n) => assert_eq!(n, 1.0),
        _ => panic!("Expected number, got: {:?}", value.type_name()),
    }
}

#[test]
fn test_pattern_with_node_type() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", 1)
        g.set_node_type("Alice", "User")
        g.add_node("Bob", 2)
        g.set_node_type("Bob", "User")
        g.add_edge("Alice", "Bob", "FRIEND")

        results = g.match(node("person", type: "User"), edge(type: "FRIEND"), node("friend", type: "User"))
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    match value.kind {
        graphoid::values::ValueKind::Number(n) => assert_eq!(n, 1.0),
        _ => panic!("Expected number, got: {:?}", value.type_name()),
    }
}

#[test]
fn test_pattern_with_where_clause() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", {age: 25})
        g.add_node("Bob", {age: 17})
        g.add_node("Carol", {age: 30})
        g.add_edge("Alice", "Bob", "FRIEND")
        g.add_edge("Alice", "Carol", "FRIEND")

        results = g.match(node("person"), edge(type: "FRIEND"), node("friend"))
                   .where(friend.age >= 18)
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(1.0));  // Only Carol
}

#[test]
fn test_pattern_with_return_clause() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", {name: "Alice", age: 25, city: "NYC"})
        g.add_node("Bob", {name: "Bob", age: 30, city: "LA"})
        g.add_edge("Alice", "Bob", "FRIEND")

        # Match pattern and return only specific fields
        results = g.match(node("person"), edge(type: "FRIEND"), node("friend"))
                   .return(person.name, friend.age)

        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(1.0));  // One match: Alice->Bob
}

#[test]
fn test_return_clause_validates_fields() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", {name: "Alice", age: 25, city: "NYC"})
        g.add_node("Bob", {name: "Bob", age: 30, city: "LA"})
        g.add_edge("Alice", "Bob", "FRIEND")

        # Match pattern and return only specific fields
        results = g.match(node("person"), edge(type: "FRIEND"), node("friend"))
                   .return(person.name, friend.age)

        # Get the first result and validate it contains only the projected fields
        first = results[0]

        # Should have person.name field
        has_person_name = first.has_key("person.name")

        # Should have friend.age field
        has_friend_age = first.has_key("friend.age")

        # Should NOT have other fields like person.age or friend.city
        has_person_age = first.has_key("person.age")

        # Return validation: both required fields present, extra field absent
        has_person_name and has_friend_age and not has_person_age
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    let value = result.unwrap();
    assert_eq!(value, Value::boolean(true), "Expected return clause to project only specified fields");
}

#[test]
fn test_return_clause_with_multiple_matches() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", {name: "Alice", age: 25})
        g.add_node("Bob", {name: "Bob", age: 30})
        g.add_node("Carol", {name: "Carol", age: 35})
        g.add_edge("Alice", "Bob", "FRIEND")
        g.add_edge("Alice", "Carol", "FRIEND")

        # Match pattern and return only names
        results = g.match(node("person"), edge(type: "FRIEND"), node("friend"))
                   .return(friend.name)

        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(2.0));  // Two matches: Alice->Bob and Alice->Carol
}

#[test]
fn test_variable_length_path() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_edge("A", "B", "FOLLOWS")
        g.add_edge("B", "C", "FOLLOWS")

        # Variable-length path: find paths of 1-2 hops
        results = g.match(node("user"), path(edge_type: "FOLLOWS", min: 1, max: 2), node("other"))
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    // Should find: A->B, B->C, A->B->C = 3 paths
    assert_eq!(result.unwrap(), Value::number(3.0));
}

#[test]
fn test_variable_length_path_exact_distance() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)
        g.add_node("D", 4)
        g.add_edge("A", "B", "FOLLOWS")
        g.add_edge("B", "C", "FOLLOWS")
        g.add_edge("C", "D", "FOLLOWS")

        # Find paths of exactly 2 hops
        results = g.match(node("start"), path(edge_type: "FOLLOWS", min: 2, max: 2), node("end"))
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    // Should find: A->B->C, B->C->D = 2 paths
    assert_eq!(result.unwrap(), Value::number(2.0));
}

// ============================================================================
// Subgraph Operations Tests - Day 6-8 (TDD RED)
// ============================================================================

/*
#[test]
fn test_extract_by_node_filter() {
    let code = r#"
        g = graph{}
        g.add_node("A", {active: true})
        g.add_node("B", {active: false})
        g.add_node("C", {active: true})
        g.add_edge("A", "B")
        g.add_edge("B", "C")

        active = g.extract {
            nodes: n => n.get_attribute("active") == true
        }
        active.node_count()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(2.0));
}

#[test]
fn test_delete_nodes() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_node("C", 3)

        cleaned = g.delete {
            nodes: n => n.value() == 2
        }
        cleaned.node_count()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(2.0));
}

#[test]
fn test_add_subgraph() {
    let code = r#"
        g1 = graph{}
        g1.add_node("A", 1)

        g2 = graph{}
        g2.add_node("B", 2)

        combined = g1.add_subgraph(g2)
        combined.node_count()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(2.0));
}
*/
