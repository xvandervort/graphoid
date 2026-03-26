//! Match expression body tests — verifying that match arm bodies
//! support full expressions (function calls, binary ops, method calls),
//! not just primary expressions.

use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::Value;

fn exec(source: &str) -> GraphExecutor {
    let mut executor = GraphExecutor::new();
    executor.execute_source(source).unwrap();
    executor
}

// =========================================================================
// Binary operations in match arm bodies
// =========================================================================

#[test]
fn test_match_arm_binary_add() {
    let e = exec("x = 2\nresult = match x { 2 => x + 10, _ => 0 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::number(12.0));
}

#[test]
fn test_match_arm_binary_multiply() {
    let e = exec("x = 3\nresult = match x { 3 => x * 5, _ => 0 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::number(15.0));
}

#[test]
fn test_match_arm_string_concat() {
    let e = exec(r#"x = "hello"
result = match x { "hello" => x + " world", _ => "nope" }"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::string("hello world".to_string()));
}

// =========================================================================
// Function calls in match arm bodies
// =========================================================================

#[test]
fn test_match_arm_function_call() {
    let e = exec(r#"
fn double(n) { return n * 2 }
x = 5
result = match x { 5 => double(x), _ => 0 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(10.0));
}

#[test]
fn test_match_arm_function_call_with_literal_arg() {
    let e = exec(r#"
fn greet(name) { return "hi " + name }
x = 1
result = match x { 1 => greet("alice"), _ => "nope" }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::string("hi alice".to_string()));
}

// =========================================================================
// Method calls in match arm bodies
// =========================================================================

#[test]
fn test_match_arm_method_call() {
    let e = exec(r#"
items = [1, 2, 3]
x = "list"
result = match x { "list" => items.length(), _ => 0 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(3.0));
}

#[test]
fn test_match_arm_string_method() {
    let e = exec(r#"
x = 1
result = match x { 1 => "hello".length(), _ => 0 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(5.0));
}

// =========================================================================
// Comparison and logical operators in match arm bodies
// =========================================================================

#[test]
fn test_match_arm_comparison() {
    let e = exec("x = 1\nresult = match x { 1 => 10 > 5, _ => false }");
    assert_eq!(e.get_variable("result").unwrap(), Value::boolean(true));
}

#[test]
fn test_match_arm_logical_and() {
    let e = exec("x = 1\nresult = match x { 1 => true and true, _ => false }");
    assert_eq!(e.get_variable("result").unwrap(), Value::boolean(true));
}

// =========================================================================
// Parenthesized and complex expressions
// =========================================================================

#[test]
fn test_match_arm_parenthesized_expr() {
    let e = exec("x = 2\nresult = match x { 2 => (x + 1) * 3, _ => 0 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::number(9.0));
}

// =========================================================================
// Multi-arm with mixed complexity
// =========================================================================

#[test]
fn test_match_multi_arm_mixed() {
    let e = exec(r#"
fn double(n) { return n * 2 }
x = 2
result = match x {
    1 => "one",
    2 => double(x),
    3 => x + 100,
    _ => "other"
}
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(4.0));
}

#[test]
fn test_match_default_arm_with_expression() {
    let e = exec("x = 99\nresult = match x { 1 => 10, _ => x * 2 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::number(198.0));
}

// =========================================================================
// Chained method calls
// =========================================================================

#[test]
fn test_match_arm_chained_methods() {
    let e = exec(r#"
x = 1
result = match x { 1 => "HELLO".lower(), _ => "nope" }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::string("hello".to_string()));
}

// =========================================================================
// List construction in match arms
// =========================================================================

#[test]
fn test_match_arm_list_literal() {
    let e = exec(r#"
x = 1
result = match x { 1 => [10, 20, 30], _ => [] }
"#);
    let val = e.get_variable("result").unwrap();
    assert_eq!(val.type_name(), "list");
}

#[test]
fn test_match_arm_list_method() {
    let e = exec(r#"
x = 1
result = match x { 1 => [3, 1, 2].length(), _ => 0 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(3.0));
}

// =========================================================================
// Map construction in match arms
// =========================================================================

#[test]
fn test_match_arm_map_literal() {
    let e = exec(r#"
x = 1
result = match x { 1 => {"a": 1}, _ => {} }
"#);
    let val = e.get_variable("result").unwrap();
    assert_eq!(val.type_name(), "map");
}

// =========================================================================
// Nested match expressions
// =========================================================================

#[test]
fn test_match_arm_nested_match() {
    let e = exec(r#"
x = 1
y = "a"
result = match x { 1 => match y { "a" => 100, _ => 0 }, _ => -1 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(100.0));
}

// =========================================================================
// Match result used in further expressions
// =========================================================================

#[test]
fn test_match_result_in_expression() {
    let e = exec(r#"
x = 2
result = (match x { 1 => 10, 2 => 20, _ => 0 }) + 5
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(25.0));
}

#[test]
fn test_match_as_function_argument() {
    let e = exec(r#"
fn add_one(n) { return n + 1 }
x = 3
result = add_one(match x { 3 => 99, _ => 0 })
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(100.0));
}

// =========================================================================
// Match arm with index access
// =========================================================================

#[test]
fn test_match_arm_index_access() {
    let e = exec(r#"
items = [10, 20, 30]
x = 1
result = match x { 1 => items[0], 2 => items[1], _ => 0 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(10.0));
}

// =========================================================================
// Match arm with negation
// =========================================================================

#[test]
fn test_match_arm_unary_negation() {
    let e = exec("x = 1\nresult = match x { 1 => -42, _ => 0 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::number(-42.0));
}

#[test]
fn test_match_arm_not_operator() {
    let e = exec("x = 1\nresult = match x { 1 => not false, _ => false }");
    assert_eq!(e.get_variable("result").unwrap(), Value::boolean(true));
}

// =========================================================================
// Match with variable binding patterns + expression bodies
// =========================================================================

#[test]
fn test_match_variable_binding_with_expression_body() {
    let e = exec(r#"
x = 42
result = match x { n => n * 2 }
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(84.0));
}

// =========================================================================
// Multi-line match arms with expressions
// =========================================================================

#[test]
fn test_match_multiline_with_expressions() {
    let e = exec(r#"
fn square(n) { return n * n }
fn cube(n) { return n * n * n }
x = 3
result = match x {
    1 => square(x),
    2 => cube(x),
    3 => square(x) + cube(x),
    _ => 0
}
"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::number(36.0));
}

// =========================================================================
// Existing simple expressions still work
// =========================================================================

#[test]
fn test_match_arm_simple_literal() {
    let e = exec(r#"x = 1
result = match x { 1 => "one", 2 => "two", _ => "other" }"#);
    assert_eq!(e.get_variable("result").unwrap(), Value::string("one".to_string()));
}

#[test]
fn test_match_arm_simple_variable() {
    let e = exec("y = 42\nx = 1\nresult = match x { 1 => y, _ => 0 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::number(42.0));
}

#[test]
fn test_match_arm_none() {
    let e = exec("x = 1\nresult = match x { 1 => none, _ => 0 }");
    assert_eq!(e.get_variable("result").unwrap(), Value::none());
}
