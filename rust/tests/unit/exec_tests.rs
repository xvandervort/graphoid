// Tests for exec() functionality - executes .gr files and returns output

use graphoid::execution::Executor;
use graphoid::values::ValueKind;
use std::fs;

#[test]
fn test_exec_simple_file() {
    let mut executor = Executor::new();

    // Create a simple test file
    let test_file = "/tmp/graphoid_exec_test1.gr";
    fs::write(test_file, r#"
print("Hello from executed file")
print("Line 2")
    "#).unwrap();

    let code = r#"
        output = exec("/tmp/graphoid_exec_test1.gr")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("output").unwrap();

    // Should return stdout as string
    if let ValueKind::String(s) = &result.kind {
        assert!(s.contains("Hello from executed file"), "Should contain output");
        assert!(s.contains("Line 2"), "Should contain all output");
    } else {
        panic!("Expected string output, got {:?}", result.kind);
    }

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_exec_returns_output_not_result() {
    let mut executor = Executor::new();

    // File that has both print statements and a final value
    let test_file = "/tmp/graphoid_exec_test2.gr";
    fs::write(test_file, r#"
print("Output line 1")
result = 42
print("Output line 2")
result
    "#).unwrap();

    let code = r#"
        output = exec("/tmp/graphoid_exec_test2.gr")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("output").unwrap();

    // Should return stdout (print statements), not the final value
    if let ValueKind::String(s) = &result.kind {
        assert!(s.contains("Output line 1"), "Should contain first print");
        assert!(s.contains("Output line 2"), "Should contain second print");
        assert!(!s.contains("42"), "Should not contain final value (only prints)");
    } else {
        panic!("Expected string output, got {:?}", result.kind);
    }

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_exec_error_in_file() {
    let mut executor = Executor::new();

    // File with an error
    let test_file = "/tmp/graphoid_exec_test_error.gr";
    fs::write(test_file, r#"
print("Before error")
x = undefined_variable
print("After error")
    "#).unwrap();

    let code = r#"
        output = exec("/tmp/graphoid_exec_test_error.gr")
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "Should propagate error from executed file");

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_exec_propagates_errors_immediately() {
    let mut executor = Executor::new();

    // File with an error
    let test_file = "/tmp/graphoid_exec_test_propagate.gr";
    fs::write(test_file, r#"
print("Before error")
x = undefined_variable
    "#).unwrap();

    let code = r#"
        output = exec("/tmp/graphoid_exec_test_propagate.gr")
    "#;

    // exec() should propagate errors immediately (not collect them)
    let result = executor.execute_source(code);
    assert!(result.is_err(), "Should propagate error from executed file");
    assert!(result.unwrap_err().to_string().contains("undefined_variable"),
            "Error message should mention the undefined variable");

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_exec_nonexistent_file() {
    let mut executor = Executor::new();

    let code = r#"
        output = exec("/tmp/nonexistent_file_xyz.gr")
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "Should error on nonexistent file");
}

#[test]
fn test_exec_empty_file() {
    let mut executor = Executor::new();

    let test_file = "/tmp/graphoid_exec_empty.gr";
    fs::write(test_file, "").unwrap();

    let code = r#"
        output = exec("/tmp/graphoid_exec_empty.gr")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("output").unwrap();

    // Should return empty string
    if let ValueKind::String(s) = &result.kind {
        assert_eq!(s, "", "Empty file should produce empty output");
    } else {
        panic!("Expected string, got {:?}", result.kind);
    }

    fs::remove_file(test_file).unwrap();
}

#[test]
fn test_exec_file_with_imports() {
    let mut executor = Executor::new();

    let test_file = "/tmp/graphoid_exec_imports.gr";
    fs::write(test_file, r#"
import "io"
print("Imported io module successfully")
    "#).unwrap();

    // Set stdlib path for imports to work
    std::env::set_var("GRAPHOID_STDLIB_PATH", "/home/irv/work/grang/stdlib");

    let code = r#"
        output = exec("/tmp/graphoid_exec_imports.gr")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("output").unwrap();

    if let ValueKind::String(s) = &result.kind {
        assert!(s.contains("Imported io module successfully"), "Should handle imports");
    } else {
        panic!("Expected string, got {:?}", result.kind);
    }

    fs::remove_file(test_file).unwrap();
}
