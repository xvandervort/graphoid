use graphoid::execution::Executor;
use graphoid::values::{Value, ValueKind};
use std::path::PathBuf;

#[test]
fn test_load_merges_into_current_namespace() {
    let mut executor = Executor::new();

    // Set current file path for relative resolution
    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"load "config.gr""#;
    executor.execute_source(source).unwrap();

    // Variables from config.gr should be directly accessible
    let debug = executor.get_variable("debug");
    assert!(debug.is_some(), "Variable 'debug' should be loaded");
    match &debug.unwrap().kind {
        ValueKind::Boolean(b) => assert_eq!(*b, true),
        _ => panic!("debug should be boolean"),
    }

    let max_connections = executor.get_variable("max_connections");
    assert!(max_connections.is_some(), "Variable 'max_connections' should be loaded");
    match &max_connections.unwrap().kind {
        ValueKind::Number(n) => assert_eq!(*n, 100.0),
        _ => panic!("max_connections should be number"),
    }

    let server_name = executor.get_variable("server_name");
    assert!(server_name.is_some(), "Variable 'server_name' should be loaded");
    match &server_name.unwrap().kind {
        ValueKind::String(s) => assert_eq!(s, "test_server"),
        _ => panic!("server_name should be string"),
    }
}

#[test]
fn test_load_functions_directly_accessible() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        load "helpers.gr"
        result = double(5)
    "#;
    executor.execute_source(source).unwrap();

    // Function from helpers.gr should be directly callable
    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        _ => panic!("result should be number"),
    }
}

#[test]
fn test_load_variable_directly_accessible() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        load "helpers.gr"
        value = helper_value
    "#;
    executor.execute_source(source).unwrap();

    // Variable from helpers.gr should be directly accessible
    let value = executor.get_variable("value").unwrap();
    match &value.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("value should be number"),
    }
}

#[test]
fn test_load_vs_import_difference() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    // Load merges into namespace
    let source1 = r#"
        load "config.gr"
        debug_from_load = debug
    "#;
    executor.execute_source(source1).unwrap();

    let debug_from_load = executor.get_variable("debug_from_load");
    assert!(debug_from_load.is_some(), "load should merge variables");

    // Create new executor for import test
    let mut executor2 = Executor::new();
    executor2.set_current_file(Some(PathBuf::from("tests/fixtures/load_tests/main.gr")));

    // Import creates separate namespace
    // Note: import uses the module path as-is for binding, so "config" not "config.gr"
    let source2 = r#"
        import "config"
        debug_from_import = config.debug
    "#;
    executor2.execute_source(source2).unwrap();

    // Direct access to 'debug' should fail with import
    let debug = executor2.get_variable("debug");
    assert!(debug.is_none(), "import should NOT merge variables into current namespace");

    // But module.variable should work
    let debug_from_import = executor2.get_variable("debug_from_import");
    assert!(debug_from_import.is_some(), "module.variable should work with import");
}

#[test]
fn test_load_relative_path() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"load "./config.gr""#;
    executor.execute_source(source).unwrap();

    let debug = executor.get_variable("debug");
    assert!(debug.is_some(), "Relative path should work with load");
}

#[test]
fn test_load_nonexistent_file() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"load "nonexistent.gr""#;
    let result = executor.execute_source(source);

    assert!(result.is_err(), "Loading nonexistent file should error");
}

#[test]
fn test_load_with_extension() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    // Both with and without extension should work
    let source1 = r#"load "config.gr""#;
    executor.execute_source(source1).unwrap();

    let debug1 = executor.get_variable("debug");
    assert!(debug1.is_some());

    // Create new executor for second test
    let mut executor2 = Executor::new();
    executor2.set_current_file(Some(PathBuf::from("tests/fixtures/load_tests/main.gr")));

    let source2 = r#"load "config""#;
    executor2.execute_source(source2).unwrap();

    let debug2 = executor2.get_variable("debug");
    assert!(debug2.is_some());
}

#[test]
fn test_multiple_loads() {
    let mut executor = Executor::new();

    let test_file = PathBuf::from("tests/fixtures/load_tests/main.gr");
    executor.set_current_file(Some(test_file));

    let source = r#"
        load "config.gr"
        load "helpers.gr"

        result = double(max_connections)
    "#;
    executor.execute_source(source).unwrap();

    // Should have variables from both files
    let debug = executor.get_variable("debug");
    assert!(debug.is_some());

    let helper_value = executor.get_variable("helper_value");
    assert!(helper_value.is_some());

    // Should be able to use both
    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 200.0), // double(100)
        _ => panic!("result should be number"),
    }
}
