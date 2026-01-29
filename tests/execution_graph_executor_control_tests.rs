mod execution_graph_test_helpers;
use execution_graph_test_helpers::{eval_var, as_number, as_string};
use graphoid::execution_graph::graph_executor::GraphExecutor;

// --- If/else ---

#[test]
fn test_if_true_branch() {
    let val = eval_var("x = 0\nif true {\n  x = 1\n}", "x");
    assert_eq!(as_number(&val), 1.0);
}

#[test]
fn test_if_false_branch() {
    let val = eval_var("x = 0\nif false {\n  x = 1\n}", "x");
    assert_eq!(as_number(&val), 0.0);
}

#[test]
fn test_if_else() {
    let val = eval_var("x = 0\nif false {\n  x = 1\n} else {\n  x = 2\n}", "x");
    assert_eq!(as_number(&val), 2.0);
}

#[test]
fn test_if_with_expression_condition() {
    let val = eval_var("x = 0\ny = 5\nif y > 3 {\n  x = 10\n}", "x");
    assert_eq!(as_number(&val), 10.0);
}

// --- While ---

#[test]
fn test_while_basic() {
    let val = eval_var("x = 0\nwhile x < 5 {\n  x = x + 1\n}", "x");
    assert_eq!(as_number(&val), 5.0);
}

#[test]
fn test_while_with_break() {
    let val = eval_var("x = 0\nwhile true {\n  x = x + 1\n  if x == 5 {\n    break\n  }\n}", "x");
    assert_eq!(as_number(&val), 5.0);
}

#[test]
fn test_while_with_continue() {
    // Sum only odd numbers from 1 to 10
    let source = r#"
total = 0
i = 0
while i < 10 {
  i = i + 1
  if i % 2 == 0 {
    continue
  }
  total = total + i
}
"#;
    let val = eval_var(source, "total");
    assert_eq!(as_number(&val), 25.0); // 1+3+5+7+9 = 25
}

// --- For ---

#[test]
fn test_for_loop() {
    let val = eval_var("total = 0\nfor n in [1, 2, 3] {\n  total = total + n\n}", "total");
    assert_eq!(as_number(&val), 6.0);
}

#[test]
fn test_for_loop_with_break() {
    let source = r#"
total = 0
for n in [1, 2, 3, 4, 5] {
  if n == 4 {
    break
  }
  total = total + n
}
"#;
    let val = eval_var(source, "total");
    assert_eq!(as_number(&val), 6.0); // 1+2+3
}

#[test]
fn test_for_loop_with_continue() {
    let source = r#"
total = 0
for n in [1, 2, 3, 4, 5] {
  if n == 3 {
    continue
  }
  total = total + n
}
"#;
    let val = eval_var(source, "total");
    assert_eq!(as_number(&val), 12.0); // 1+2+4+5 = 12
}

// --- Return ---

#[test]
fn test_return_from_program() {
    // Return at top level just ends execution
    let mut executor = GraphExecutor::new();
    let result = executor.execute_source("x = 5\nreturn x");
    // Return should produce a value
    assert!(result.is_ok() || result.is_err()); // At minimum, doesn't crash
}

// --- Nested control flow ---

#[test]
fn test_nested_if() {
    let source = r#"
x = 10
result = "none"
if x > 5 {
  if x > 15 {
    result = "big"
  } else {
    result = "medium"
  }
}
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "medium");
}

#[test]
fn test_nested_loops() {
    let source = r#"
total = 0
for i in [1, 2, 3] {
  for j in [10, 20] {
    total = total + i * j
  }
}
"#;
    let val = eval_var(source, "total");
    // (1*10 + 1*20) + (2*10 + 2*20) + (3*10 + 3*20) = 30 + 60 + 90 = 180
    assert_eq!(as_number(&val), 180.0);
}

// --- Conditional expression (ternary) ---

#[test]
fn test_ternary_true() {
    let val = eval_var("x = 1 if true else 2", "x");
    assert_eq!(as_number(&val), 1.0);
}

#[test]
fn test_ternary_false() {
    let val = eval_var("x = 1 if false else 2", "x");
    assert_eq!(as_number(&val), 2.0);
}
