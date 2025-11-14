use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// Universal Casting Tests
// Spec lines 3185: All types should support to_num(), to_string(), to_bool()
// ============================================================================

// ============================================================================
// Boolean Casting Tests (Spec line 179)
// ============================================================================

#[test]
fn test_bool_to_num_true() {
    let mut executor = Executor::new();
    let code = r#"
        result = true.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 1.0));
}

#[test]
fn test_bool_to_num_false() {
    let mut executor = Executor::new();
    let code = r#"
        result = false.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 0.0));
}

#[test]
fn test_bool_to_string_true() {
    let mut executor = Executor::new();
    let code = r#"
        result = true.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "true"));
}

#[test]
fn test_bool_to_string_false() {
    let mut executor = Executor::new();
    let code = r#"
        result = false.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "false"));
}

#[test]
fn test_bool_to_bool() {
    let mut executor = Executor::new();
    let code = r#"
        result1 = true.to_bool()
        result2 = false.to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result1 = executor.env().get("result1").unwrap();
    let result2 = executor.env().get("result2").unwrap();
    assert!(matches!(&result1.kind, ValueKind::Boolean(b) if *b));
    assert!(matches!(&result2.kind, ValueKind::Boolean(b) if !*b));
}

// ============================================================================
// None Casting Tests (Spec lines 206-207)
// ============================================================================

#[test]
fn test_none_to_num() {
    let mut executor = Executor::new();
    let code = r#"
        result = none.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 0.0));
}

#[test]
fn test_none_to_string() {
    let mut executor = Executor::new();
    let code = r#"
        result = none.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == ""));
}

#[test]
fn test_none_to_bool() {
    let mut executor = Executor::new();
    let code = r#"
        result = none.to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if !*b));
}

// ============================================================================
// List Casting Tests (Spec line 260)
// ============================================================================

#[test]
fn test_list_to_bool_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = [].to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if !*b));
}

#[test]
fn test_list_to_bool_non_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = [1, 2, 3].to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if *b));
}

#[test]
fn test_list_to_string_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = [].to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "[]"));
}

#[test]
fn test_list_to_string_numbers() {
    let mut executor = Executor::new();
    let code = r#"
        result = [1, 2, 3].to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "[1, 2, 3]"));
}

#[test]
fn test_list_to_string_mixed() {
    let mut executor = Executor::new();
    let code = r#"
        result = [1, "hello", true].to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    // Should produce a reasonable string representation
    assert!(matches!(&result.kind, ValueKind::String(s) if s.contains("1") && s.contains("hello") && s.contains("true")));
}

#[test]
fn test_list_to_num_size() {
    let mut executor = Executor::new();
    let code = r#"
        result = [1, 2, 3, 4, 5].to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    // to_num() on a list should return its size
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 5.0));
}

// ============================================================================
// Hash Casting Tests (Spec line 274)
// ============================================================================

#[test]
fn test_hash_to_bool_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = {}.to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if !*b));
}

#[test]
fn test_hash_to_bool_non_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = {"a": 1, "b": 2}.to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if *b));
}

#[test]
fn test_hash_to_string_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = {}.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "{}"));
}

#[test]
fn test_hash_to_string_simple() {
    let mut executor = Executor::new();
    let code = r#"
        result = {"name": "Alice", "age": 30}.to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    // Should produce a reasonable string representation
    assert!(matches!(&result.kind, ValueKind::String(s) if s.contains("name") && s.contains("Alice")));
}

#[test]
fn test_hash_to_num_size() {
    let mut executor = Executor::new();
    let code = r#"
        result = {"a": 1, "b": 2, "c": 3}.to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    // to_num() on a hash should return its size (number of keys)
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 3.0));
}

// ============================================================================
// String Casting Tests (for completeness)
// ============================================================================

#[test]
fn test_string_to_num_valid() {
    let mut executor = Executor::new();
    let code = r#"
        result = "123.45".to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if (*n - 123.45).abs() < 0.001));
}

#[test]
fn test_string_to_num_invalid() {
    let mut executor = Executor::new();
    let code = r#"
        result = "hello".to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    // Invalid string should convert to 0 or NaN (check implementation)
    assert!(matches!(&result.kind, ValueKind::Number(n) if n.is_nan() || *n == 0.0));
}

#[test]
fn test_string_to_bool_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = "".to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if !*b));
}

#[test]
fn test_string_to_bool_non_empty() {
    let mut executor = Executor::new();
    let code = r#"
        result = "hello".to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if *b));
}

// ============================================================================
// Number Casting Tests (for completeness)
// ============================================================================

#[test]
fn test_number_to_string() {
    let mut executor = Executor::new();
    let code = r#"
        result = (42.5).to_string()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "42.5"));
}

#[test]
fn test_number_to_bool_zero() {
    let mut executor = Executor::new();
    let code = r#"
        result = (0.0).to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if !*b));
}

#[test]
fn test_number_to_bool_non_zero() {
    let mut executor = Executor::new();
    let code = r#"
        result = (42.0).to_bool()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(b) if *b));
}

#[test]
fn test_number_to_num() {
    let mut executor = Executor::new();
    let code = r#"
        result = (42.5).to_num()
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 42.5));
}
