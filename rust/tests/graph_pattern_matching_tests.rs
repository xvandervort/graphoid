//! Integration Tests for Graph Pattern Matching - Phase 9 (TDD RED phase)
//!
//! These tests verify Cypher-style graph pattern matching:
//! 1. Parser tests - parsing pattern syntax
//! 2. Pattern matching tests - matching patterns against graphs
//! 3. Subgraph operation tests - extract, delete, add

use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::execution::Executor;
use graphoid::values::Value;
use graphoid::ast::{Stmt, Expr, GraphPattern, PatternNode, PatternEdge, EdgeDirection, EdgeLength};

/// Helper to parse code and return AST
fn parse(code: &str) -> Result<Expr, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser
        .parse()
        .map_err(|e| format!("Parser error: {}", e))?;

    // Return the first expression statement
    if let Some(Stmt::Expression { expr, .. }) = program.statements.first() {
        Ok(expr.clone())
    } else {
        Err("No expression found".to_string())
    }
}

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
// Parser Tests - Day 1-2 (TDD RED)
// ============================================================================

#[test]
fn test_parse_simple_graph_pattern() {
    let code = "g.match((person:User) -[:FRIEND]-> (friend:User))";
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());

    let expr = result.unwrap();
    match expr {
        Expr::GraphMatch { pattern, .. } => {
            // Should be a GraphMatch expression
            assert_eq!(pattern.nodes.len(), 2);
            assert_eq!(pattern.edges.len(), 1);
            assert_eq!(pattern.nodes[0].variable, "person");
            assert_eq!(pattern.nodes[0].node_type, Some("User".to_string()));
            assert_eq!(pattern.nodes[1].variable, "friend");
            assert_eq!(pattern.nodes[1].node_type, Some("User".to_string()));
        }
        other => panic!("Expected GraphMatch, got: {:?}", other),
    }
}

#[test]
fn test_parse_pattern_node_with_type() {
    let code = "g.match((person:User))";
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify node has variable="person" and type="User"
}

#[test]
fn test_parse_pattern_node_without_type() {
    let code = "g.match((person))";
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify node has variable="person" and type=None
}

#[test]
fn test_parse_directed_edge() {
    let code = "g.match((a) -[:FRIEND]-> (b))";
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify edge is directed with type="FRIEND"
}

#[test]
fn test_parse_bidirectional_edge() {
    let code = "g.match((a:User) -[:FRIEND]- (b:User))";
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify edge is bidirectional
}

#[test]
fn test_parse_variable_length_path() {
    let code = "g.match((user:User) -[:FOLLOWS*{min: 1, max: 3}]-> (influencer:User))";
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify edge length is Variable { min: 1, max: 3 }
}

#[test]
fn test_parse_pattern_with_where_clause() {
    let code = r#"
        g.match((person:User) -[:FRIEND]-> (friend:User))
            .where(person.age > 18, friend.age > 18)
    "#;
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify where clause contains 2 conditions
}

#[test]
fn test_parse_pattern_with_return_clause() {
    let code = r#"
        g.match((person) -[:FRIEND]-> (friend))
            .return(person, friend)
    "#;
    let result = parse(code);

    assert!(result.is_ok(), "Expected parsing to succeed, got: {:?}", result.err());
    // TODO: Verify return clause contains 2 fields
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

        results = g.match((person) -[:FRIEND]-> (friend))
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

        results = g.match((person:User) -[:FRIEND]-> (friend:User))
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

/*

#[test]
fn test_pattern_with_where_clause() {
    let code = r#"
        g = graph{}
        g.add_node("Alice", {age: 25})
        g.add_node("Bob", {age: 17})
        g.add_node("Carol", {age: 30})
        g.add_edge("Alice", "Bob", "FRIEND")
        g.add_edge("Alice", "Carol", "FRIEND")

        results = g.match((person) -[:FRIEND]-> (friend))
                   .where(friend.age >= 18)
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(1.0));  // Only Carol
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

        results = g.match((user) -[:FOLLOWS*1..2]-> (other))
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    // Should find: A->B, B->C, A->B->C = 3 paths
    assert_eq!(result.unwrap(), Value::number(3.0));
}

#[test]
fn test_bidirectional_pattern() {
    let code = r#"
        g = graph{}
        g.add_node("A", 1)
        g.add_node("B", 2)
        g.add_edge("A", "B", "FRIEND")
        # Note: for bidirectional, should match even though edge is only A->B

        results = g.match((a) -[:FRIEND]- (b))
        results.size()
    "#;

    let result = execute_and_return(code);
    assert!(result.is_ok(), "Expected execution to succeed, got: {:?}", result.err());
    assert_eq!(result.unwrap(), Value::number(1.0));
}

// ============================================================================
// Subgraph Operations Tests - Day 6-8 (TDD RED)
// ============================================================================

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
