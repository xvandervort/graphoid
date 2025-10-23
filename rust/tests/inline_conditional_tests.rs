//! Tests for inline conditionals (Phase 6.5 Area 2)
//!
//! Syntax forms to support:
//! 1. if-then-else: `value if condition else other`
//! 2. suffix if: `value if condition` (returns none if false)
//! 3. suffix unless: `value unless condition` (returns none if true)

use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::Value;

// ============================================================================
// If-Then-Else Inline Conditionals
// ============================================================================

#[test]
fn test_simple_if_then_else_true() {
    let source = r#"
        x = 10
        result = "big" if x > 5 else "small"
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::String("big".to_string()));
}

#[test]
fn test_simple_if_then_else_false() {
    let source = r#"
        x = 3
        result = "big" if x > 5 else "small"
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::String("small".to_string()));
}

#[test]
fn test_if_then_else_with_expressions() {
    let source = r#"
        threshold = 100
        value = 150
        result = value * 2 if value > threshold else value / 2
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::Number(300.0)); // 150 * 2
}

#[test]
fn test_nested_if_then_else() {
    let source = r#"
        x = 15
        result = "large" if x > 20 else ("medium" if x > 10 else "small")
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::String("medium".to_string()));
}

#[test]
fn test_if_then_else_with_boolean_literals() {
    let source = r#"
        result = "yes" if true else "no"
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::String("yes".to_string()));
}

// ============================================================================
// Suffix If (returns none when false)
// ============================================================================

#[test]
fn test_suffix_if_true() {
    let source = r#"
        ready = true
        result = "go" if ready
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::String("go".to_string()));
}

#[test]
fn test_suffix_if_false() {
    let source = r#"
        ready = false
        result = "go" if ready
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::None);
}

#[test]
fn test_suffix_if_with_comparison() {
    let source = r#"
        x = 10
        threshold = 5
        result = x * 2 if x > threshold
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    assert_eq!(result, Value::Number(20.0));
}

// ============================================================================
// Suffix Unless (returns none when true)
// ============================================================================

#[test]
fn test_suffix_unless_false() {
    let source = r#"
        error_occurred = false
        message = "OK" unless error_occurred
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let message = executor.env().get("message").unwrap();
    assert_eq!(message, Value::String("OK".to_string()));
}

#[test]
fn test_suffix_unless_true() {
    let source = r#"
        error_occurred = true
        message = "OK" unless error_occurred
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let message = executor.env().get("message").unwrap();
    assert_eq!(message, Value::None);
}

#[test]
fn test_suffix_unless_with_comparison() {
    let source = r#"
        paused = false
        status = "active" unless paused
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let status = executor.env().get("status").unwrap();
    assert_eq!(status, Value::String("active".to_string()));
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_if_then_else_with_none_values() {
    let source = r#"
        x = none
        result = "something" if x else "nothing"
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    // none is falsy, so should take else branch
    assert_eq!(result, Value::String("nothing".to_string()));
}

#[test]
fn test_if_then_else_with_numeric_condition() {
    let source = r#"
        x = 0
        result = "truthy" if x else "falsy"
    "#;

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    for stmt in &program.statements {
        executor.eval_stmt(stmt).unwrap();
    }

    let result = executor.env().get("result").unwrap();
    // 0 is falsy in most languages
    assert_eq!(result, Value::String("falsy".to_string()));
}
