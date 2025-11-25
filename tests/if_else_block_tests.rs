use graphoid::execution::Executor;
use graphoid::values::ValueKind;

/// TDD: Tests for traditional if-else block syntax
/// These tests should FAIL initially, then pass after parser implementation

#[test]
fn test_if_block_without_else() {
    let mut executor = Executor::new();
    let source = r#"
        x = 10
        if x > 5 {
            result = "big"
        }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => assert_eq!(s, "big"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_if_block_with_else() {
    let mut executor = Executor::new();
    let source = r#"
        x = 3
        if x > 5 {
            result = "big"
        } else {
            result = "small"
        }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => assert_eq!(s, "small"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_if_block_multiline_body() {
    let mut executor = Executor::new();
    let source = r#"
        x = 10
        if x > 5 {
            y = x * 2
            z = y + 10
            result = z
        }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 30.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_if_else_both_multiline() {
    let mut executor = Executor::new();
    let source = r#"
        age = 25
        if age >= 18 {
            status = "adult"
            rights = "full"
        } else {
            status = "minor"
            rights = "limited"
        }
    "#;

    executor.execute_source(source).unwrap();

    let status = executor.get_variable("status").unwrap();
    let rights = executor.get_variable("rights").unwrap();

    match &status.kind {
        ValueKind::String(s) => assert_eq!(s, "adult"),
        _ => panic!("Expected string"),
    }
    match &rights.kind {
        ValueKind::String(s) => assert_eq!(s, "full"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_if_as_expression_with_blocks() {
    let mut executor = Executor::new();
    let source = r#"
        x = 10
        result = if x > 5 { "big" } else { "small" }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => assert_eq!(s, "big"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_nested_if_blocks() {
    let mut executor = Executor::new();
    let source = r#"
        x = 15
        if x > 10 {
            if x > 20 {
                result = "very big"
            } else {
                result = "medium"
            }
        } else {
            result = "small"
        }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => assert_eq!(s, "medium"),
        _ => panic!("Expected string"),
    }
}

#[test]
fn test_if_block_with_function_calls() {
    let mut executor = Executor::new();
    let source = r#"
        fn double(x) {
            return x * 2
        }

        n = 5
        if n > 0 {
            result = double(n)
        } else {
            result = 0
        }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        _ => panic!("Expected number"),
    }
}

#[test]
fn test_if_block_empty_else() {
    let mut executor = Executor::new();
    let source = r#"
        x = 3
        result = "default"
        if x > 5 {
            result = "big"
        } else {
            # else block can be empty or just have comments
        }
    "#;

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => assert_eq!(s, "default"),
        _ => panic!("Expected string"),
    }
}
