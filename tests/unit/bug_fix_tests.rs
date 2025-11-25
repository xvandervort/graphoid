//! Regression tests for bug fixes
//!
//! These tests ensure critical bugs stay fixed and don't regress.

use graphoid::execution::Executor;
use graphoid::values::ValueKind;

/// Helper function to execute source code and return the executor
fn execute_and_get_executor(source: &str) -> Executor {
    let mut executor = Executor::new();
    executor.execute_source(source).expect("Execution should succeed");
    executor
}

/// Helper function to get a string variable value
fn get_string_var(executor: &Executor, name: &str) -> String {
    let value = executor.env().get(name).expect(&format!("{} should exist", name));
    match &value.kind {
        ValueKind::String(s) => s.clone(),
        _ => panic!("{} should be a string, got {:?}", name, value.kind),
    }
}

#[test]
fn test_data_variable_name_not_reserved() {
    // Issue #1: "data" was incorrectly reserved as a keyword
    // This prevented using "data" as a variable name
    let mut executor = Executor::new();

    let source = r#"
        data = [1, 2, 3]
        x = data
    "#;

    executor.execute_source(source).expect("Parser should accept 'data' as variable name");

    // Verify data variable exists and has correct value
    let data_value = executor.env().get("data").expect("'data' variable should exist");
    match &data_value.kind {
        ValueKind::List(_) => {}, // Success
        _ => panic!("'data' should contain a list, got {:?}", data_value.kind),
    }
}

#[test]
fn test_data_variable_after_import() {
    // Issue #1 specifically occurred after import statements
    let mut executor = Executor::new();

    let source = r#"
        # This used to fail with "Expected identifier, got Equal"
        data = [1, 2, 3, 4, 5]
        result = data[0]
    "#;

    executor.execute_source(source).expect("Parser should accept 'data' as variable name after import");
}

#[test]
fn test_numeric_list_to_string() {
    // Issue #2: Numeric lists displayed as empty string
    // Root cause: Overly aggressive byte array optimization
    let source = r#"
        nums = [1, 2, 3]
        result = nums.to_string()
    "#;

    let executor = execute_and_get_executor(source);
    let result_str = get_string_var(&executor, "result");

    // Should display as "[1, 2, 3]", NOT as empty string
    assert!(!result_str.is_empty(), "Numeric list to_string should not be empty");
    assert!(result_str.contains("1"), "Should contain '1'");
    assert!(result_str.contains("2"), "Should contain '2'");
    assert!(result_str.contains("3"), "Should contain '3'");
    assert_eq!(result_str, "[1, 2, 3]", "Should format as [1, 2, 3]");
}

#[test]
fn test_string_list_to_string() {
    // Ensure string lists still work correctly
    let source = r#"
        strs = ["a", "b", "c"]
        result = strs.to_string()
    "#;

    let executor = execute_and_get_executor(source);
    let result_str = get_string_var(&executor, "result");
    assert_eq!(result_str, r#"["a", "b", "c"]"#, "String list should display correctly");
}

#[test]
fn test_mixed_list_to_string() {
    // Ensure mixed-type lists work correctly
    let source = r#"
        mixed = [1, "hello", true]
        result = mixed.to_string()
    "#;

    let executor = execute_and_get_executor(source);
    let result_str = get_string_var(&executor, "result");

    assert!(result_str.contains("1"), "Should contain number");
    assert!(result_str.contains("hello"), "Should contain string");
    assert!(result_str.contains("true"), "Should contain boolean");
    assert_eq!(result_str, r#"[1, "hello", true]"#, "Mixed list should display correctly");
}

#[test]
fn test_empty_list_to_string() {
    // Edge case: empty list
    let source = r#"
        empty = []
        result = empty.to_string()
    "#;

    let executor = execute_and_get_executor(source);
    let result_str = get_string_var(&executor, "result");
    assert_eq!(result_str, "[]", "Empty list should display as []");
}

#[test]
fn test_large_numeric_list_to_string() {
    // Issue #2 affected ANY numeric list with values 0-255
    // Test with values outside byte range to ensure no edge cases
    let source = r#"
        large = [256, 1000, 999999]
        result = large.to_string()
    "#;

    let executor = execute_and_get_executor(source);
    let result_str = get_string_var(&executor, "result");

    assert!(result_str.contains("256"), "Should contain '256'");
    assert!(result_str.contains("1000"), "Should contain '1000'");
    assert!(result_str.contains("999999"), "Should contain '999999'");
    assert_eq!(result_str, "[256, 1000, 999999]", "Large numbers should display correctly");
}

#[test]
fn test_float_list_to_string() {
    // Ensure floats display correctly (not as "1.0" but as "1")
    let source = r#"
        floats = [1.0, 2.5, 3.0]
        result = floats.to_string()
    "#;

    let executor = execute_and_get_executor(source);
    let result_str = get_string_var(&executor, "result");

    // Integers should display without .0, but actual floats should show decimals
    assert!(result_str.contains("1") && !result_str.contains("1.0"), "Should contain '1' (not '1.0')");
    assert!(result_str.contains("2.5"), "Should contain '2.5'");
    assert!(result_str.contains("3") && result_str.matches("3").count() == 1, "Should contain '3' (not '3.0')");
}
