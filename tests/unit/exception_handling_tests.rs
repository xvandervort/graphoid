//! Tests for exception handling, specifically the fix for user-raised error type matching.
//!
//! These tests verify that `raise ValueError("message")` can be caught by `catch ValueError`.
//! The original bug was that the error type parser was finding the first colon in
//! "Runtime error: ValueError: message" instead of the user error type.

use graphoid::execution::Executor;
use graphoid::values::Value;

// ============================================================================
// USER-RAISED ERROR TYPE MATCHING TESTS
// These tests verify the fix for catching user-raised errors by type
// ============================================================================

#[test]
fn test_catch_user_raised_valueerror_by_type() {
    // This is the core bug fix test - ValueError should be caught by catch ValueError
    let source = r#"
caught = false
error_type = ""
try {
    raise ValueError("bad value")
}
catch ValueError as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true), "ValueError should be caught by catch ValueError");

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("ValueError".to_string()), "Error type should be ValueError");
}

#[test]
fn test_catch_user_raised_typeerror_by_type() {
    let source = r#"
caught = false
error_type = ""
try {
    raise TypeError("wrong type")
}
catch TypeError as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true), "TypeError should be caught by catch TypeError");

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("TypeError".to_string()));
}

#[test]
fn test_catch_user_raised_ioerror_by_type() {
    let source = r#"
caught = false
error_type = ""
try {
    raise IOError("file not found")
}
catch IOError as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true), "IOError should be caught by catch IOError");

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("IOError".to_string()));
}

#[test]
fn test_catch_user_raised_networkerror_by_type() {
    let source = r#"
caught = false
error_type = ""
try {
    raise NetworkError("connection refused")
}
catch NetworkError as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true), "NetworkError should be caught by catch NetworkError");

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("NetworkError".to_string()));
}

#[test]
fn test_catch_user_raised_parseerror_by_type() {
    let source = r#"
caught = false
error_type = ""
try {
    raise ParseError("invalid syntax")
}
catch ParseError as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true), "ParseError should be caught by catch ParseError");

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("ParseError".to_string()));
}

#[test]
fn test_user_error_not_caught_by_wrong_type() {
    // ValueError should NOT be caught by catch TypeError
    let source = r#"
caught_by_wrong = false
caught_by_right = false
try {
    raise ValueError("bad value")
}
catch TypeError {
    caught_by_wrong = true
}
catch ValueError {
    caught_by_right = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught_by_wrong = executor.get_variable("caught_by_wrong").unwrap();
    assert_eq!(caught_by_wrong, Value::boolean(false), "ValueError should NOT be caught by catch TypeError");

    let caught_by_right = executor.get_variable("caught_by_right").unwrap();
    assert_eq!(caught_by_right, Value::boolean(true), "ValueError should be caught by catch ValueError");
}

#[test]
fn test_user_error_message_preserved() {
    // The error message should be preserved correctly
    let source = r#"
message = ""
try {
    raise ValueError("custom message with special chars: !@#$%")
}
catch ValueError as e {
    message = e.message()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let message = executor.get_variable("message").unwrap();
    assert_eq!(message, Value::string("custom message with special chars: !@#$%".to_string()));
}

#[test]
fn test_multiple_catch_clauses_with_user_errors() {
    // Test that the correct catch clause is selected for different error types
    let source = r#"
which_caught = ""
try {
    raise TypeError("type problem")
}
catch ValueError {
    which_caught = "ValueError"
}
catch TypeError {
    which_caught = "TypeError"
}
catch IOError {
    which_caught = "IOError"
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let which_caught = executor.get_variable("which_caught").unwrap();
    assert_eq!(which_caught, Value::string("TypeError".to_string()));
}

#[test]
fn test_catch_all_still_works_for_user_errors() {
    // Catch-all should still catch user errors
    let source = r#"
caught = false
error_type = ""
try {
    raise ValueError("any error")
}
catch as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true));

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("ValueError".to_string()));
}

#[test]
fn test_nested_try_catch_with_user_errors() {
    // Test nested try/catch with different user error types
    let source = r#"
inner_caught = ""
outer_caught = ""
try {
    try {
        raise ValueError("inner error")
    }
    catch ValueError as e {
        inner_caught = e.type()
        raise TypeError("re-raise as different type")
    }
}
catch TypeError as e {
    outer_caught = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let inner_caught = executor.get_variable("inner_caught").unwrap();
    assert_eq!(inner_caught, Value::string("ValueError".to_string()));

    let outer_caught = executor.get_variable("outer_caught").unwrap();
    assert_eq!(outer_caught, Value::string("TypeError".to_string()));
}

#[test]
fn test_finally_runs_with_user_error() {
    // Finally should always run even with user-raised errors
    let source = r#"
finally_ran = false
error_caught = false
try {
    raise ValueError("error")
}
catch ValueError {
    error_caught = true
}
finally {
    finally_ran = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let finally_ran = executor.get_variable("finally_ran").unwrap();
    assert_eq!(finally_ran, Value::boolean(true));

    let error_caught = executor.get_variable("error_caught").unwrap();
    assert_eq!(error_caught, Value::boolean(true));
}

#[test]
fn test_user_error_with_empty_message() {
    // Test that errors with empty messages work correctly
    let source = r#"
caught = false
msg = "not empty"
try {
    raise ValueError("")
}
catch ValueError as e {
    caught = true
    msg = e.message()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true));

    let msg = executor.get_variable("msg").unwrap();
    assert_eq!(msg, Value::string("".to_string()));
}

#[test]
fn test_user_error_with_colon_in_message() {
    // Test that colons in the error message don't break parsing
    let source = r#"
caught = false
msg = ""
try {
    raise ValueError("Error: something: else: went: wrong")
}
catch ValueError as e {
    caught = true
    msg = e.message()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true));

    let msg = executor.get_variable("msg").unwrap();
    assert_eq!(msg, Value::string("Error: something: else: went: wrong".to_string()));
}

// ============================================================================
// RUNTIME ERROR VS USER ERROR TESTS
// These tests verify that RuntimeError (from division by zero, etc.) is
// distinct from user-raised RuntimeError
// ============================================================================

#[test]
fn test_runtime_error_from_division_still_works() {
    // Division by zero should still be caught as RuntimeError
    let source = r#"
caught = false
try {
    x = 1 / 0
}
catch RuntimeError {
    caught = true
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true));
}

#[test]
fn test_user_raised_runtimeerror() {
    // User can also raise RuntimeError explicitly
    let source = r#"
caught = false
error_type = ""
try {
    raise RuntimeError("user runtime error")
}
catch RuntimeError as e {
    caught = true
    error_type = e.type()
}
"#;
    let mut executor = Executor::new();
    executor.execute_source(source).unwrap();

    let caught = executor.get_variable("caught").unwrap();
    assert_eq!(caught, Value::boolean(true));

    let error_type = executor.get_variable("error_type").unwrap();
    assert_eq!(error_type, Value::string("RuntimeError".to_string()));
}

// ============================================================================
// Total: 16 exception handling tests specifically for the user error type fix
// ============================================================================
