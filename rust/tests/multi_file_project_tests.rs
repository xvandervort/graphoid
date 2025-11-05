use graphoid::execution::Executor;
use graphoid::values::{Value, ValueKind};
use std::path::PathBuf;

#[test]
fn test_import_from_subdirectory() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        import "models/user"
        alice = user.create_user("Alice", 30)
    "#;
    executor.execute_source(source).unwrap();

    let alice = executor.get_variable("alice").unwrap();
    match &alice.kind {
        ValueKind::Map(hash) => {
            let name = hash.get("name").unwrap();
            match &name.kind {
                ValueKind::String(s) => assert_eq!(s, "Alice"),
                _ => panic!("name should be string"),
            }
        }
        _ => panic!("alice should be hash"),
    }
}

#[test]
fn test_module_imports_another_module() {
    let mut executor = Executor::new();

    // Import calculator service which internally imports math utils
    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        import "services/calculator"
        sum = calculator.sum_two_numbers(10, 20)
    "#;
    executor.execute_source(source).unwrap();

    let sum = executor.get_variable("sum").unwrap();
    match &sum.kind {
        ValueKind::Number(n) => assert_eq!(*n, 30.0),
        _ => panic!("sum should be number"),
    }
}

#[test]
fn test_nested_module_imports() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    // Calculator uses math utils internally
    let source = r#"
        import "services/calculator"
        area = calculator.calculate_circle_area(5.0)
    "#;
    executor.execute_source(source).unwrap();

    let area = executor.get_variable("area").unwrap();
    match &area.kind {
        ValueKind::Number(n) => {
            // Area of circle with radius 5 = pi * r^2 = 3.14159 * 25 = 78.53975
            assert!((n - 78.53975).abs() < 0.001);
        }
        _ => panic!("area should be number"),
    }
}

#[test]
fn test_multiple_independent_imports() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        import "models/user"
        import "services/calculator"

        alice = user.create_user("Alice", 30)
        sum = calculator.sum_two_numbers(5, 10)
    "#;
    executor.execute_source(source).unwrap();

    let alice = executor.get_variable("alice").unwrap();
    assert!(matches!(&alice.kind, ValueKind::Map(_)));

    let sum = executor.get_variable("sum").unwrap();
    match &sum.kind {
        ValueKind::Number(n) => assert_eq!(*n, 15.0),
        _ => panic!("sum should be number"),
    }
}

#[test]
fn test_module_declaration_with_alias() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/models/test.gr");
    executor.set_current_file(Some(test_file));

    // user.gr declares "module user_model alias user"
    // When imported, both the module name and alias should be recognized
    let source = r#"
        import "user"
        alice = user.create_user("Alice", 30)
    "#;
    executor.execute_source(source).unwrap();

    let alice = executor.get_variable("alice").unwrap();
    assert!(matches!(&alice.kind, ValueKind::Map(_)));
}

#[test]
fn test_module_constants_accessible() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        import "utils/math"
        pi_value = math.pi
    "#;
    executor.execute_source(source).unwrap();

    let pi_value = executor.get_variable("pi_value").unwrap();
    match &pi_value.kind {
        ValueKind::Number(n) => assert_eq!(*n, 3.14159),
        _ => panic!("pi_value should be number"),
    }
}

#[test]
fn test_full_main_file_execution() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file.clone()));

    // Execute the entire main.gr file
    let source = std::fs::read_to_string(&test_file).unwrap();
    executor.execute_source(&source).unwrap();

    // Verify all expected variables exist
    let alice = executor.get_variable("alice");
    assert!(alice.is_some(), "alice should be defined");

    let area = executor.get_variable("area");
    assert!(area.is_some(), "area should be defined");

    let sum_result = executor.get_variable("sum_result");
    assert!(sum_result.is_some(), "sum_result should be defined");

    // Verify values
    match &sum_result.unwrap().kind {
        ValueKind::Number(n) => assert_eq!(*n, 30.0),
        _ => panic!("sum_result should be number"),
    }
}

#[test]
fn test_relative_import_from_service() {
    let mut executor = Executor::new();

    // Import calculator which uses relative import "../utils/math"
    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        import "services/calculator"
        result = calculator.sum_two_numbers(7, 8)
    "#;
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 15.0),
        _ => panic!("result should be number"),
    }
}

#[test]
fn test_module_caching_across_imports() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/multi_file_project/main.gr");
    executor.set_current_file(Some(test_file));

    // Import calculator which also imports math
    // Then import math directly - should use cached version
    let source = r#"
        import "services/calculator"
        import "utils/math"

        # Both should work
        sum1 = calculator.sum_two_numbers(5, 5)
        sum2 = math.add(3, 3)
    "#;
    executor.execute_source(source).unwrap();

    let sum1 = executor.get_variable("sum1").unwrap();
    match &sum1.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        _ => panic!("sum1 should be number"),
    }

    let sum2 = executor.get_variable("sum2").unwrap();
    match &sum2.kind {
        ValueKind::Number(n) => assert_eq!(*n, 6.0),
        _ => panic!("sum2 should be number"),
    }
}
