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

// ============================================================================
// BASIC LIST METHODS - Phase 5
// ============================================================================

#[test]
fn test_list_size() {
    let code = "[1, 2, 3, 4, 5].size()";
    assert_eq!(eval(code), Value::Number(5.0));
}

#[test]
fn test_list_size_empty() {
    let code = "[].size()";
    assert_eq!(eval(code), Value::Number(0.0));
}

#[test]
fn test_list_first() {
    let code = "[10, 20, 30].first()";
    assert_eq!(eval(code), Value::Number(10.0));
}

#[test]
fn test_list_last() {
    let code = "[10, 20, 30].last()";
    assert_eq!(eval(code), Value::Number(30.0));
}

#[test]
fn test_list_is_empty_false() {
    let code = "[1, 2, 3].is_empty()";
    assert_eq!(eval(code), Value::Boolean(false));
}

#[test]
fn test_list_is_empty_true() {
    let code = "[].is_empty()";
    assert_eq!(eval(code), Value::Boolean(true));
}

#[test]
fn test_list_contains_true() {
    let code = "[1, 2, 3, 4, 5].contains(3)";
    assert_eq!(eval(code), Value::Boolean(true));
}

#[test]
fn test_list_contains_false() {
    let code = "[1, 2, 3, 4, 5].contains(10)";
    assert_eq!(eval(code), Value::Boolean(false));
}

#[test]
fn test_list_index_of_found() {
    let code = "[10, 20, 30, 40].index_of(30)";
    assert_eq!(eval(code), Value::Number(2.0));
}

#[test]
fn test_list_index_of_not_found() {
    let code = "[10, 20, 30, 40].index_of(99)";
    assert_eq!(eval(code), Value::Number(-1.0));
}

// ============================================================================
// LIST FUNCTIONAL METHODS - each, reduce
// ============================================================================

#[test]
fn test_list_each_returns_original() {
    let code = r#"
sum = 0
result = [1, 2, 3].each(x => { sum = sum + x })
result
"#;
    // each should return the original list
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_list_reduce_sum() {
    let code = "[1, 2, 3, 4, 5].reduce(0, (acc, x) => acc + x)";
    assert_eq!(eval(code), Value::Number(15.0));
}

#[test]
fn test_list_reduce_product() {
    let code = "[1, 2, 3, 4].reduce(1, (acc, x) => acc * x)";
    assert_eq!(eval(code), Value::Number(24.0));
}

#[test]
fn test_list_select_alias() {
    let code = "[1, 2, 3, 4, 5].select(x => x > 3)";
    assert_eq!(eval(code), list_nums(vec![4.0, 5.0]));
}

#[test]
fn test_list_reject() {
    let code = "[1, 2, 3, 4, 5].reject(x => x > 3)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

// ============================================================================
// LIST MUTATION METHODS - append!, prepend!, insert!, remove!
// ============================================================================

#[test]
fn test_list_append_immutable() {
    let code = "[1, 2, 3].append(4)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_append_mutable() {
    let code = r#"
items = [1, 2, 3]
items.append!(4)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_prepend_immutable() {
    let code = "[2, 3, 4].prepend(1)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_prepend_mutable() {
    let code = r#"
items = [2, 3, 4]
items.prepend!(1)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_insert_immutable() {
    let code = "[1, 2, 4].insert(2, 3)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_insert_mutable() {
    let code = r#"
items = [1, 2, 4]
items.insert!(2, 3)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_remove_immutable() {
    let code = "[1, 2, 3, 2, 4].remove(2)";
    // Should remove first occurrence
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 2.0, 4.0]));
}

#[test]
fn test_list_remove_mutable() {
    let code = r#"
items = [1, 2, 3, 2, 4]
items.remove!(2)
items
"#;
    // Should remove first occurrence
    assert_eq!(eval(code), list_nums(vec![1.0, 3.0, 2.0, 4.0]));
}

#[test]
fn test_list_remove_at_index_immutable() {
    let code = "[1, 2, 3, 4].remove_at_index(2)";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 4.0]));
}

#[test]
fn test_list_remove_at_index_mutable() {
    let code = r#"
items = [1, 2, 3, 4]
items.remove_at_index!(2)
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 4.0]));
}

#[test]
fn test_list_pop_immutable() {
    let code = "[1, 2, 3].pop()";
    // pop() without ! returns the popped value
    assert_eq!(eval(code), Value::Number(3.0));
}

#[test]
fn test_list_pop_mutable() {
    let code = r#"
items = [1, 2, 3]
popped = items.pop!()
popped
"#;
    // pop!() returns the popped value
    assert_eq!(eval(code), Value::Number(3.0));
}

#[test]
fn test_list_clear_immutable() {
    let code = "[1, 2, 3].clear()";
    assert_eq!(eval(code), list_nums(vec![]));
}

#[test]
fn test_list_clear_mutable() {
    let code = r#"
items = [1, 2, 3]
items.clear!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![]));
}

// ============================================================================
// LIST TRANSFORMATION METHODS - sort, reverse, uniq, compact
// ============================================================================

#[test]
fn test_list_sort_immutable() {
    let code = "[3, 1, 4, 1, 5].sort()";
    assert_eq!(eval(code), list_nums(vec![1.0, 1.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_list_sort_mutable() {
    let code = r#"
items = [3, 1, 4, 1, 5]
items.sort!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 1.0, 3.0, 4.0, 5.0]));
}

#[test]
fn test_list_reverse_immutable() {
    let code = "[1, 2, 3, 4].reverse()";
    assert_eq!(eval(code), list_nums(vec![4.0, 3.0, 2.0, 1.0]));
}

#[test]
fn test_list_reverse_mutable() {
    let code = r#"
items = [1, 2, 3, 4]
items.reverse!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![4.0, 3.0, 2.0, 1.0]));
}

#[test]
fn test_list_uniq_immutable() {
    let code = "[1, 2, 2, 3, 3, 3, 4].uniq()";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_uniq_mutable() {
    let code = r#"
items = [1, 2, 2, 3, 3, 3, 4]
items.uniq!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn test_list_compact_immutable() {
    let code = "[1, none, 2, none, 3].compact()";
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}

#[test]
fn test_list_compact_mutable() {
    let code = r#"
items = [1, none, 2, none, 3]
items.compact!()
items
"#;
    assert_eq!(eval(code), list_nums(vec![1.0, 2.0, 3.0]));
}
