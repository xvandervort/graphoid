use graphoid::execution::Executor;
use graphoid::values::ValueKind;
use std::path::PathBuf;

#[test]
fn test_import_creates_module_namespace() {
    let mut executor = Executor::new();
    let source = r#"import "simple_module""#;

    // Set current file path so module resolution works
    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    // Verify module exists in environment
    // simple_module.gr declares "module simple alias sim", so it's bound as "sim"
    let module_value = executor.get_variable("sim");
    assert!(module_value.is_some(), "Module 'sim' should be in environment");
}

#[test]
fn test_import_with_alias() {
    let mut executor = Executor::new();
    let source = r#"import "simple_module" as sm"#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    // Verify alias works
    let module_value = executor.get_variable("sm");
    assert!(module_value.is_some(), "Module alias 'sm' should be in environment");
}

#[test]
fn test_access_module_function() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        result = sim.double(5)
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        _ => panic!("Expected number result, got {:?}", result),
    }
}

#[test]
fn test_access_module_variable() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        value = sim.helper_value
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let value = executor.get_variable("value").unwrap();
    match &value.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number value, got {:?}", value),
    }
}

#[test]
fn test_module_caching() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        import "simple_module"
        value = sim.helper_value
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    // Should not error, should reuse cached module
    executor.execute_source(source).unwrap();

    // Verify the module is accessible
    let value = executor.get_variable("value").unwrap();
    match &value.kind {
        ValueKind::Number(n) => assert_eq!(*n, 42.0),
        _ => panic!("Expected number value"),
    }
}

#[test]
fn test_import_nonexistent_module() {
    let mut executor = Executor::new();
    let source = r#"import "does_not_exist""#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    let result = executor.execute_source(source);
    assert!(result.is_err(), "Should error when importing nonexistent module");
}
