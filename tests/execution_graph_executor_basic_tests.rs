mod execution_graph_test_helpers;
use execution_graph_test_helpers::{eval_var, as_number, as_string, as_bool};
use graphoid::execution::Executor as GraphExecutor;
use graphoid::values::ValueKind;

fn as_symbol(val: &graphoid::values::Value) -> String {
    match &val.kind {
        ValueKind::Symbol(s) => s.clone(),
        _ => panic!("Expected symbol, got {:?}", val.kind),
    }
}

// --- Number arithmetic ---

#[test]
fn test_number_addition() {
    let val = eval_var("x = 3 + 4", "x");
    assert_eq!(as_number(&val), 7.0);
}

#[test]
fn test_number_subtraction() {
    let val = eval_var("x = 10 - 3", "x");
    assert_eq!(as_number(&val), 7.0);
}

#[test]
fn test_number_multiplication() {
    let val = eval_var("x = 6 * 7", "x");
    assert_eq!(as_number(&val), 42.0);
}

#[test]
fn test_number_division() {
    let val = eval_var("x = 15 / 3", "x");
    assert_eq!(as_number(&val), 5.0);
}

#[test]
fn test_number_modulo() {
    let val = eval_var("x = 10 % 3", "x");
    assert_eq!(as_number(&val), 1.0);
}

#[test]
fn test_number_power() {
    let val = eval_var("x = 2 ** 10", "x");
    assert_eq!(as_number(&val), 1024.0);
}

#[test]
fn test_compound_expression() {
    let val = eval_var("x = 3 + 4 * 2", "x");
    assert_eq!(as_number(&val), 11.0);
}

// --- String operations ---

#[test]
fn test_string_assignment() {
    let val = eval_var("y = \"hello\"", "y");
    assert_eq!(as_string(&val), "hello");
}

#[test]
fn test_string_concatenation() {
    let val = eval_var("y = \"hello\" + \" world\"", "y");
    assert_eq!(as_string(&val), "hello world");
}

// --- Boolean operations ---

#[test]
fn test_boolean_true() {
    let val = eval_var("b = true", "b");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_boolean_false() {
    let val = eval_var("b = false", "b");
    assert_eq!(as_bool(&val), false);
}

#[test]
fn test_boolean_not() {
    let val = eval_var("a = true\nb = not a", "a");
    assert_eq!(as_bool(&val), true);
    let val_b = eval_var("a = true\nb = not a", "b");
    assert_eq!(as_bool(&val_b), false);
}

#[test]
fn test_boolean_and() {
    let val = eval_var("b = true and false", "b");
    assert_eq!(as_bool(&val), false);
}

#[test]
fn test_boolean_or() {
    let val = eval_var("b = false or true", "b");
    assert_eq!(as_bool(&val), true);
}

// --- Comparison ---

#[test]
fn test_equal() {
    let val = eval_var("b = 5 == 5", "b");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_not_equal() {
    let val = eval_var("b = 5 != 3", "b");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_less_than() {
    let val = eval_var("b = 3 < 5", "b");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_greater_than() {
    let val = eval_var("b = 5 > 3", "b");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_less_equal() {
    let val = eval_var("b = 5 <= 5", "b");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_greater_equal() {
    let val = eval_var("b = 5 >= 6", "b");
    assert_eq!(as_bool(&val), false);
}

// --- Negate ---

#[test]
fn test_negate() {
    let val = eval_var("x = -5", "x");
    assert_eq!(as_number(&val), -5.0);
}

// --- None ---

#[test]
fn test_none_literal() {
    let val = eval_var("x = none", "x");
    assert!(matches!(val.kind, ValueKind::None));
}

// --- Symbol ---

#[test]
fn test_symbol_literal() {
    let val = eval_var("x = :my_sym", "x");
    assert_eq!(as_symbol(&val), "my_sym");
}

// --- Variable reassignment ---

#[test]
fn test_variable_reassignment() {
    let val = eval_var("x = 5\nx = x + 1", "x");
    assert_eq!(as_number(&val), 6.0);
}

#[test]
fn test_multiple_variables() {
    let val = eval_var("x = 10\ny = 20\nz = x + y", "z");
    assert_eq!(as_number(&val), 30.0);
}

// --- List literal ---

#[test]
fn test_list_literal() {
    let val = eval_var("x = [1, 2, 3]", "x");
    assert!(matches!(val.kind, ValueKind::List(_)));
}

// --- Map literal ---

#[test]
fn test_map_literal() {
    let val = eval_var("x = {\"a\": 1, \"b\": 2}", "x");
    assert!(matches!(val.kind, ValueKind::Map(_)));
}

// --- Expression statement ---

#[test]
fn test_expression_statement() {
    let mut executor = GraphExecutor::new();
    executor.execute_source("42").unwrap();
}

// --- Integer division ---

#[test]
fn test_integer_division() {
    let val = eval_var("x = 7 // 2", "x");
    assert_eq!(as_number(&val), 3.0);
}
