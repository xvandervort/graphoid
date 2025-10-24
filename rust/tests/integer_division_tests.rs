//! Tests for integer division operator (//)
//! Phase 6.5 Area 2 - Parser Completeness

use graphoid::ast;
use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::Value;

fn eval_expr(code: &str) -> Value {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut executor = Executor::new();

    // Evaluate all statements
    for (i, stmt) in program.statements.iter().enumerate() {
        // Last statement might be an expression we want to return
        if i == program.statements.len() - 1 {
            if let ast::Stmt::Expression { expr, .. } = stmt {
                return executor.eval_expr(expr).unwrap();
            }
        }
        executor.eval_stmt(stmt).unwrap();
    }

    Value::None
}

// ============================================================================
// BASIC INTEGER DIVISION
// ============================================================================

#[test]
fn test_int_div_positive_exact() {
    let result = eval_expr("10 // 2");
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_int_div_positive_truncate() {
    let result = eval_expr("10 // 3");
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_int_div_negative_dividend() {
    let result = eval_expr("-10 // 3");
    assert_eq!(result, Value::Number(-3.0));
}

#[test]
fn test_int_div_negative_divisor() {
    let result = eval_expr("10 // -3");
    assert_eq!(result, Value::Number(-3.0));
}

#[test]
fn test_int_div_both_negative() {
    let result = eval_expr("-10 // -3");
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_int_div_with_floats() {
    let result = eval_expr("10.5 // 2");
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_int_div_result_is_floor() {
    // 7 / 2 = 3.5, floor = 3
    let result = eval_expr("7 // 2");
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_int_div_zero_dividend() {
    let result = eval_expr("0 // 5");
    assert_eq!(result, Value::Number(0.0));
}

// ============================================================================
// CHAINING AND EXPRESSIONS
// ============================================================================

#[test]
fn test_int_div_in_expression() {
    let result = eval_expr("(20 // 3) + 1");
    // 20 // 3 = 6, then 6 + 1 = 7
    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_int_div_with_multiplication() {
    let result = eval_expr("20 // 3 * 2");
    // 20 // 3 = 6, then 6 * 2 = 12
    assert_eq!(result, Value::Number(12.0));
}

#[test]
fn test_int_div_precedence() {
    // // and / should have same precedence
    let result = eval_expr("20 / 2 // 3");
    // Left-to-right: 20 / 2 = 10, then 10 // 3 = 3
    assert_eq!(result, Value::Number(3.0));
}

// ============================================================================
// VARIABLE USAGE
// ============================================================================

#[test]
fn test_int_div_with_variables() {
    let code = r#"
        x = 17
        y = 5
        x // y
    "#;
    let result = eval_expr(code);
    assert_eq!(result, Value::Number(3.0));
}
