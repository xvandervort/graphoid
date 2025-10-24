//! Tests for Missing Collection Methods
//! Phase 6.5 Area 4

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

fn list_nums(nums: Vec<f64>) -> Value {
    Value::List(List::from_vec(nums.into_iter().map(Value::Number).collect()))
}

// ============================================================================
// SLICE WITH STEP PARAMETER
// ============================================================================

#[test]
fn test_slice_with_step_basic() {
    // Every other element
    let code = "[1, 2, 3, 4, 5, 6].slice(0, 6, 2)";
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0]));
}

#[test]
fn test_slice_with_step_offset() {
    // Every other element starting from index 1
    let code = "[1, 2, 3, 4, 5, 6].slice(1, 6, 2)";
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_slice_with_step_three() {
    // Every third element
    let code = "[1, 2, 3, 4, 5, 6, 7, 8, 9].slice(0, 9, 3)";
    assert_eq!(eval(code), list_nums(vec![1.0, 4.0, 7.0]));
}

#[test]
fn test_slice_step_one_same_as_no_step() {
    // Step of 1 should be same as normal slice
    let code = "[1, 2, 3, 4, 5].slice(1, 4, 1)";
    assert_eq!(eval(code), list_nums(vec![2.0, 3.0, 4.0]));
}

#[test]
fn test_slice_step_larger_than_range() {
    // Step larger than range returns just start element
    let code = "[1, 2, 3, 4, 5].slice(0, 5, 10)";
    assert_eq!(eval(code), list_nums(vec![1.0]));
}

// ============================================================================
// GENERATE METHOD - Range Mode
// ============================================================================

#[test]
fn test_generate_range_basic() {
    // Generate sequence with step
    let code = "list.generate(1, 10, 2)";
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0, 7.0, 9.0]));
}

#[test]
fn test_generate_range_step_one() {
    // Generate consecutive numbers
    let code = "list.generate(1, 5, 1)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_generate_range_negative_step() {
    // Generate descending sequence
    let code = "list.generate(10, 5, -1)";
    assert_eq!(eval(code), list_nums(vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0]));
}

// ============================================================================
// GENERATE METHOD - Function Mode
// ============================================================================

#[test]
fn test_generate_function_squares() {
    // Generate using function
    let code = "list.generate(1, 5, x => x * x)";
    assert_eq!(eval(code), list_nums(vec![1.0, 4.0, 9.0, 16.0, 25.0]));
}

#[test]
fn test_generate_function_custom() {
    // Generate using custom function
    let code = "list.generate(1, 4, x => x * 2 + 1)";
    assert_eq!(eval(code), list_nums(vec![3.0, 5.0, 7.0, 9.0]));
}

// ============================================================================
// UPTO HELPER
// ============================================================================

#[test]
fn test_upto_basic() {
    // Generate 0 to n inclusive
    let code = "list.upto(5)";
    assert_eq!(eval(code), list_nums(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_upto_zero() {
    // upto(0) should return [0]
    let code = "list.upto(0)";
    assert_eq!(eval(code), list_nums(vec![0.0]));
}

#[test]
fn test_upto_in_expression() {
    // Use upto in expression
    let code = "list.upto(3).map(x => x * 2)";
    assert_eq!(eval(code), list_nums(vec![0.0, 2.0, 4.0, 6.0]));
}

// ============================================================================
// ADDITIONAL PREDICATES - Verify existing ones work
// ============================================================================

#[test]
fn test_predicate_positive() {
    let code = "[-1, 0, 1, 2].filter(:positive)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0]));
}

#[test]
fn test_predicate_negative() {
    let code = "[-2, -1, 0, 1].filter(:negative)";
    assert_eq!(eval(code), list_nums(vec![-2.0, -1.0]));
}

#[test]
fn test_predicate_even() {
    let code = "[1, 2, 3, 4, 5].filter(:even)";
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0]));
}

#[test]
fn test_predicate_odd() {
    let code = "[1, 2, 3, 4, 5].filter(:odd)";
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 5.0]));
}

// ============================================================================
// ADDITIONAL TRANSFORMATIONS - Verify existing ones work
// ============================================================================

#[test]
fn test_transformation_double() {
    let code = "[1, 2, 3].map(:double)";
    assert_eq!(eval(code), list_nums(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_transformation_square() {
    let code = "[2, 3, 4].map(:square)";
    assert_eq!(eval(code), list_nums(vec![4.0, 9.0, 16.0]));
}

#[test]
fn test_transformation_negate() {
    let code = "[1, -2, 3].map(:negate)";
    assert_eq!(eval(code), list_nums(vec![-1.0, 2.0, -3.0]));
}

#[test]
fn test_transformation_increment() {
    let code = "[1, 2, 3].map(:increment)";
    assert_eq!(eval(code), list_nums(vec![2.0, 3.0, 4.0]));
}

#[test]
fn test_transformation_decrement() {
    let code = "[1, 2, 3].map(:decrement)";
    assert_eq!(eval(code), list_nums(vec![0.0, 1.0, 2.0]));
}
