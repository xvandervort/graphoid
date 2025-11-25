//! Pattern Matching Integration Tests - Phase 7 Day 3 (TDD RED phase)
//!
//! These tests verify end-to-end pattern matching execution.

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
// Basic Pattern Matching Tests
// ============================================================================

#[test]
fn test_literal_number_pattern_matching() {
    let code = r#"
        fn is_zero(n) {
            |0| => true
            |x| => false
        }
        is_zero(0)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(true));
}

#[test]
fn test_literal_number_pattern_no_match() {
    let code = r#"
        fn is_zero(n) {
            |0| => true
            |x| => false
        }
        is_zero(42)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::boolean(false));
}

#[test]
fn test_variable_pattern_binding() {
    let code = r#"
        fn double(x) {
            |n| => n * 2
        }
        double(21)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::number(42.0));
}

// ============================================================================
// Recursive Pattern Matching Tests
// ============================================================================

#[test]
fn test_factorial_pattern_matching() {
    let code = r#"
        fn factorial(n) {
            |0| => 1
            |1| => 1
            |x| => x * factorial(x - 1)
        }
        factorial(5)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::number(120.0));
}

#[test]
fn test_fibonacci_pattern_matching() {
    let code = r#"
        fn fib(n) {
            |0| => 0
            |1| => 1
            |x| => fib(x - 1) + fib(x - 2)
        }
        fib(10)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::number(55.0));
}

// ============================================================================
// String Pattern Matching Tests
// ============================================================================

#[test]
fn test_string_pattern_matching() {
    let code = r#"
        fn get_sound(animal) {
            |"dog"| => "woof"
            |"cat"| => "meow"
            |"cow"| => "moo"
        }
        get_sound("cat")
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("meow".to_string()));
}

#[test]
fn test_string_pattern_no_match_returns_none() {
    let code = r#"
        fn get_sound(animal) {
            |"dog"| => "woof"
            |"cat"| => "meow"
        }
        get_sound("bird")
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::none());
}

// ============================================================================
// PATTERN GUARD TESTS (Phase 7 - TDD RED)
// ============================================================================

#[test]
fn test_pattern_guard_with_less_than() {
    let code = r#"
fn classify(n) {
    |x| if x < 0 => "negative"
    |0| => "zero"
    |x| if x > 0 => "positive"
}
classify(-5)
"#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("negative".to_string()));
}

#[test]
fn test_pattern_guard_with_zero() {
    let code = r#"
fn classify(n) {
    |x| if x < 0 => "negative"
    |0| => "zero"
    |x| if x > 0 => "positive"
}
classify(0)
"#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("zero".to_string()));
}

#[test]
fn test_pattern_guard_with_greater_than() {
    let code = r#"
fn classify(n) {
    |x| if x < 0 => "negative"
    |0| => "zero"
    |x| if x > 0 => "positive"
}
classify(10)
"#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("positive".to_string()));
}

#[test]
fn test_pattern_guard_fallthrough_when_false() {
    let code = r#"
fn check(n) {
    |x| if x > 100 => "big"
    |x| if x > 10 => "medium"
    |x| => "small"
}
check(5)
"#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("small".to_string()));
}

#[test]
fn test_pattern_guard_with_equality() {
    let code = r#"
fn check(s) {
    |x| if x == "hello" => "greeting"
    |x| if x == "bye" => "farewell"
    |x| => "unknown"
}
check("hello")
"#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("greeting".to_string()));
}

#[test]
fn test_pattern_guard_multiple_conditions() {
    let code = r#"
fn check_range(n) {
    |x| if x >= 0 and x <= 10 => "low"
    |x| if x > 10 and x <= 100 => "medium"
    |x| if x > 100 => "high"
    |x| => "out of range"
}
check_range(50)
"#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("medium".to_string()));
}

#[test]
fn test_pattern_guard_with_none_fallthrough() {
    let code = r#"
fn check(n) {
    |x| if x > 0 => "positive"
    |x| if x < 0 => "negative"
}
check(0)
"#;

    // 0 doesn't match any guard, should fallthrough to none
    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::none());
}

// ============================================================================
// Boolean Pattern Matching Tests
// ============================================================================

#[test]
fn test_boolean_pattern_matching() {
    let code = r#"
        fn bool_to_string(b) {
            |true| => "yes"
            |false| => "no"
        }
        bool_to_string(true)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("yes".to_string()));
}

// ============================================================================
// None Pattern Matching Tests
// ============================================================================

#[test]
fn test_none_pattern_matching() {
    let code = r#"
        fn handle_optional(val) {
            |none| => "nothing"
            |x| => "something"
        }
        handle_optional(none)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("nothing".to_string()));
}

// ============================================================================
// Wildcard Pattern Tests
// ============================================================================

#[test]
fn test_wildcard_pattern() {
    let code = r#"
        fn always_match(x) {
            |_| => "matched"
        }
        always_match(42)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::string("matched".to_string()));
}

// ============================================================================
// Pattern Order Tests
// ============================================================================

#[test]
fn test_pattern_order_matters() {
    let code = r#"
        fn classify(n) {
            |x| => "any"
            |0| => "zero"
        }
        classify(0)
    "#;

    let result = execute_and_return(code).unwrap();
    // First pattern matches, so returns "any"
    assert_eq!(result, Value::string("any".to_string()));
}

// ============================================================================
// Complex Expression in Pattern Body Tests
// ============================================================================

#[test]
fn test_complex_expression_in_pattern_body() {
    let code = r#"
        fn calc(x) {
            |0| => 100
            |n| => n * 2 + 10
        }
        calc(5)
    "#;

    let result = execute_and_return(code).unwrap();
    assert_eq!(result, Value::number(20.0));
}

// ============================================================================
// Variable Shadowing Tests
// ============================================================================

#[test]
fn test_variable_shadowing_in_patterns() {
    let code = r#"
        x = 100
        fn use_x(n) {
            |0| => x
            |x| => x * 2
        }
        use_x(5)
    "#;

    let result = execute_and_return(code).unwrap();
    // Pattern variable x shadows outer x, so 5 * 2 = 10
    assert_eq!(result, Value::number(10.0));
}

#[test]
fn test_outer_variable_accessible_in_pattern_body() {
    let code = r#"
        x = 100
        fn use_x(n) {
            |0| => x
            |y| => y * 2
        }
        use_x(0)
    "#;

    let result = execute_and_return(code).unwrap();
    // Outer x is accessible
    assert_eq!(result, Value::number(100.0));
}
