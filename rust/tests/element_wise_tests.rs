//! Tests for element-wise operators (.+, .*, ./, .//, .^, and comparison variants)
//! Phase 6.5 Area 2 - Parser Completeness

use graphoid::ast;
use graphoid::execution::Executor;
use graphoid::lexer::Lexer;
use graphoid::parser::Parser;
use graphoid::values::{Value, List};

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

fn list_from_numbers(nums: Vec<f64>) -> Value {
    Value::List(List::from_vec(nums.into_iter().map(Value::Number).collect()))
}

fn list_from_bools(bools: Vec<bool>) -> Value {
    Value::List(List::from_vec(bools.into_iter().map(Value::Boolean).collect()))
}

// ============================================================================
// SCALAR OPERATIONS - List .op Scalar
// ============================================================================

#[test]
fn test_element_add_scalar() {
    let result = eval_expr("[1, 2, 3] .+ 10");
    assert_eq!(result, list_from_numbers(vec![11.0, 12.0, 13.0]));
}

#[test]
fn test_element_sub_scalar() {
    let result = eval_expr("[10, 20, 30] .- 5");
    assert_eq!(result, list_from_numbers(vec![5.0, 15.0, 25.0]));
}

#[test]
fn test_element_mul_scalar() {
    let result = eval_expr("[1, 2, 3] .* 2");
    assert_eq!(result, list_from_numbers(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_element_div_scalar() {
    let result = eval_expr("[10, 20, 30] ./ 2");
    assert_eq!(result, list_from_numbers(vec![5.0, 10.0, 15.0]));
}

#[test]
fn test_element_int_div_scalar() {
    let result = eval_expr("[10, 21, 32] .// 3");
    assert_eq!(result, list_from_numbers(vec![3.0, 7.0, 10.0]));
}

#[test]
fn test_element_pow_scalar() {
    let result = eval_expr("[2, 3, 4] .^ 2");
    assert_eq!(result, list_from_numbers(vec![4.0, 9.0, 16.0]));
}

// ============================================================================
// VECTOR OPERATIONS - List .op List (element-wise zip)
// ============================================================================

#[test]
fn test_element_add_vector() {
    let result = eval_expr("[1, 2, 3] .+ [4, 5, 6]");
    assert_eq!(result, list_from_numbers(vec![5.0, 7.0, 9.0]));
}

#[test]
fn test_element_sub_vector() {
    let result = eval_expr("[10, 20, 30] .- [1, 2, 3]");
    assert_eq!(result, list_from_numbers(vec![9.0, 18.0, 27.0]));
}

#[test]
fn test_element_mul_vector() {
    let result = eval_expr("[2, 3, 4] .* [5, 6, 7]");
    assert_eq!(result, list_from_numbers(vec![10.0, 18.0, 28.0]));
}

#[test]
fn test_element_div_vector() {
    let result = eval_expr("[10, 20, 30] ./ [2, 4, 5]");
    assert_eq!(result, list_from_numbers(vec![5.0, 5.0, 6.0]));
}

#[test]
fn test_element_pow_vector() {
    let result = eval_expr("[2, 3, 4] .^ [2, 2, 2]");
    assert_eq!(result, list_from_numbers(vec![4.0, 9.0, 16.0]));
}

// ============================================================================
// COMPARISON OPERATIONS - Scalar
// ============================================================================

#[test]
fn test_element_gt_scalar() {
    let result = eval_expr("[10, 20, 30] .> 15");
    assert_eq!(result, list_from_bools(vec![false, true, true]));
}

#[test]
fn test_element_lt_scalar() {
    let result = eval_expr("[10, 20, 30] .< 25");
    assert_eq!(result, list_from_bools(vec![true, true, false]));
}

#[test]
fn test_element_gte_scalar() {
    let result = eval_expr("[10, 20, 30] .>= 20");
    assert_eq!(result, list_from_bools(vec![false, true, true]));
}

#[test]
fn test_element_lte_scalar() {
    let result = eval_expr("[10, 20, 30] .<= 20");
    assert_eq!(result, list_from_bools(vec![true, true, false]));
}

#[test]
fn test_element_eq_scalar() {
    let result = eval_expr("[10, 20, 30] .== 20");
    assert_eq!(result, list_from_bools(vec![false, true, false]));
}

#[test]
fn test_element_ne_scalar() {
    let result = eval_expr("[10, 20, 30] .!= 20");
    assert_eq!(result, list_from_bools(vec![true, false, true]));
}

// ============================================================================
// COMPARISON OPERATIONS - Vector
// ============================================================================

#[test]
fn test_element_gt_vector() {
    let result = eval_expr("[10, 20, 30] .> [15, 15, 15]");
    assert_eq!(result, list_from_bools(vec![false, true, true]));
}

#[test]
fn test_element_eq_vector() {
    let result = eval_expr("[1, 2, 3] .== [1, 5, 3]");
    assert_eq!(result, list_from_bools(vec![true, false, true]));
}

// ============================================================================
// EDGE CASES
// ============================================================================

#[test]
fn test_element_empty_list() {
    let result = eval_expr("[] .+ 10");
    assert_eq!(result, list_from_numbers(vec![]));
}

#[test]
fn test_element_single_element() {
    let result = eval_expr("[5] .* 3");
    assert_eq!(result, list_from_numbers(vec![15.0]));
}

#[test]
fn test_element_mismatched_length_vector() {
    // When vector lengths don't match, zip to shorter length
    let result = eval_expr("[1, 2, 3, 4] .+ [10, 20]");
    assert_eq!(result, list_from_numbers(vec![11.0, 22.0]));
}

#[test]
fn test_element_with_floats() {
    let result = eval_expr("[1.5, 2.5, 3.5] .* 2");
    assert_eq!(result, list_from_numbers(vec![3.0, 5.0, 7.0]));
}

#[test]
fn test_element_negative_numbers() {
    let result = eval_expr("[-1, -2, -3] .* -1");
    assert_eq!(result, list_from_numbers(vec![1.0, 2.0, 3.0]));
}

// ============================================================================
// CHAINING AND EXPRESSIONS
// ============================================================================

#[test]
fn test_element_chained_operations() {
    // Use parentheses to enforce left-to-right evaluation
    let result = eval_expr("([1, 2, 3] .+ 10) .* 2");
    // First: [1, 2, 3] .+ 10 = [11, 12, 13]
    // Then: [11, 12, 13] .* 2 = [22, 24, 26]
    assert_eq!(result, list_from_numbers(vec![22.0, 24.0, 26.0]));
}

#[test]
fn test_element_in_variable() {
    let code = r#"
        numbers = [1, 2, 3]
        doubled = numbers .* 2
        doubled
    "#;
    let result = eval_expr(code);
    assert_eq!(result, list_from_numbers(vec![2.0, 4.0, 6.0]));
}

#[test]
fn test_element_with_parentheses() {
    let result = eval_expr("([1, 2, 3] .+ 5) .* 2");
    // First: [1, 2, 3] .+ 5 = [6, 7, 8]
    // Then: [6, 7, 8] .* 2 = [12, 14, 16]
    assert_eq!(result, list_from_numbers(vec![12.0, 14.0, 16.0]));
}
