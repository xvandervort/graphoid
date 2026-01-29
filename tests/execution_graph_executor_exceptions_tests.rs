mod execution_graph_test_helpers;
use execution_graph_test_helpers::{eval_var, as_string};

// --- Try/catch ---

#[test]
fn test_try_catch_basic() {
    let source = r#"
result = "ok"
try {
  x = 1 / 0
} catch {
  result = "caught"
}
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "caught");
}

#[test]
fn test_try_no_error() {
    let source = r#"
result = "before"
try {
  result = "in try"
} catch {
  result = "caught"
}
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "in try");
}

#[test]
fn test_try_finally() {
    let source = r#"
result = "before"
cleanup = "no"
try {
  result = "in try"
} catch {
  result = "caught"
} finally {
  cleanup = "yes"
}
"#;
    let val = eval_var(source, "cleanup");
    assert_eq!(as_string(&val), "yes");
}

#[test]
fn test_try_catch_finally_with_error() {
    let source = r#"
result = "before"
cleanup = "no"
try {
  x = 1 / 0
} catch {
  result = "caught"
} finally {
  cleanup = "yes"
}
"#;
    let cleanup = eval_var(source, "cleanup");
    assert_eq!(as_string(&cleanup), "yes");
    let result = eval_var(source, "result");
    assert_eq!(as_string(&result), "caught");
}

// --- Raise ---

#[test]
fn test_raise_and_catch() {
    let source = r#"
result = "ok"
try {
  raise "something went wrong"
} catch {
  result = "caught"
}
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "caught");
}

// --- Nested try/catch ---

#[test]
fn test_nested_try_catch() {
    let source = r#"
outer = "ok"
inner = "ok"
try {
  try {
    x = 1 / 0
  } catch {
    inner = "inner caught"
  }
  outer = "outer ok"
} catch {
  outer = "outer caught"
}
"#;
    let inner = eval_var(source, "inner");
    assert_eq!(as_string(&inner), "inner caught");
    let outer = eval_var(source, "outer");
    assert_eq!(as_string(&outer), "outer ok");
}
