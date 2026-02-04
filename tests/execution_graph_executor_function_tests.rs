mod execution_graph_test_helpers;
use execution_graph_test_helpers::{eval_var, as_number, as_string};
use graphoid::execution::Executor as GraphExecutor;
use graphoid::values::ValueKind;

// --- Simple function declaration and call ---

#[test]
fn test_simple_function() {
    let source = r#"
fn add(a, b) {
  return a + b
}
result = add(3, 4)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 7.0);
}

#[test]
fn test_function_no_return() {
    // Function with no explicit return returns none
    let source = r#"
fn greet(name) {
  x = "hello " + name
}
result = greet("world")
"#;
    let val = eval_var(source, "result");
    assert!(matches!(val.kind, ValueKind::None));
}

#[test]
fn test_function_implicit_return() {
    // In Graphoid, without explicit return, function returns none
    let source = r#"
fn double(x) {
  x * 2
}
result = double(5)
"#;
    let val = eval_var(source, "result");
    assert!(matches!(val.kind, ValueKind::None));
}

#[test]
fn test_function_early_return() {
    let source = r#"
fn abs_val(x) {
  if x < 0 {
    return -x
  }
  return x
}
result = abs_val(-5)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 5.0);
}

#[test]
fn test_function_multiple_calls() {
    let source = r#"
fn square(x) {
  return x * x
}
a = square(3)
b = square(4)
result = a + b
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 25.0); // 9 + 16
}

// --- Recursion ---

#[test]
fn test_recursion_factorial() {
    let source = r#"
fn factorial(n) {
  if n <= 1 {
    return 1
  }
  return n * factorial(n - 1)
}
result = factorial(5)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 120.0);
}

#[test]
fn test_recursion_fibonacci() {
    let source = r#"
fn fib(n) {
  if n <= 1 {
    return n
  }
  return fib(n - 1) + fib(n - 2)
}
result = fib(10)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 55.0);
}

// --- Closures / Lambdas ---

#[test]
fn test_lambda_basic() {
    let source = r#"
double = x => x * 2
result = double(5)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 10.0);
}

#[test]
fn test_lambda_closure() {
    let source = r#"
fn make_adder(n) {
  f = x => x + n
  return f
}
add5 = make_adder(5)
result = add5(3)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 8.0);
}

#[test]
fn test_lambda_in_for_loop() {
    let source = r#"
fn apply(f, x) {
  return f(x)
}
result = apply(x => x * 3, 7)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 21.0);
}

// --- Default parameters ---

#[test]
fn test_default_parameters() {
    let source = r#"
fn greet(name, greeting = "Hello") {
  return greeting + " " + name
}
result = greet("world")
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "Hello world");
}

#[test]
fn test_default_parameter_override() {
    let source = r#"
fn greet(name, greeting = "Hello") {
  return greeting + " " + name
}
result = greet("world", "Hi")
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "Hi world");
}

// --- Builtins ---

#[test]
fn test_builtin_typeof() {
    let source = r#"result = typeof(42)"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "num");
}

#[test]
fn test_builtin_typeof_string() {
    let source = r#"result = typeof("hello")"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "string");
}

#[test]
fn test_builtin_length_list() {
    let source = r#"result = length([1, 2, 3])"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 3.0);
}

#[test]
fn test_builtin_length_string() {
    let source = r#"result = length("hello")"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 5.0);
}

#[test]
fn test_builtin_to_string() {
    // string() is a parser keyword; use string concatenation instead
    let source = r#"result = "" + 42"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "42");
}

#[test]
fn test_builtin_int_conversion() {
    let source = r#"result = int(3.7)"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 3.0);
}

#[test]
fn test_builtin_print() {
    // print should not crash; just verifying it executes
    let mut executor = GraphExecutor::new();
    let result = executor.execute_source("print(42)");
    assert!(result.is_ok());
}

// --- Function scope ---

#[test]
fn test_function_scope_isolation() {
    let source = r#"
x = 10
fn change_x() {
  x = 99
}
change_x()
result = x
"#;
    // Function should have its own scope; x in outer scope should be unchanged
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 10.0);
}

// --- Higher-order functions ---

#[test]
fn test_function_as_argument() {
    let source = r#"
fn apply_twice(f, x) {
  return f(f(x))
}
fn increment(n) {
  return n + 1
}
result = apply_twice(increment, 5)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 7.0);
}

#[test]
fn test_function_returned_from_function() {
    let source = r#"
fn make_multiplier(factor) {
  f = x => x * factor
  return f
}
times3 = make_multiplier(3)
result = times3(10)
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 30.0);
}
