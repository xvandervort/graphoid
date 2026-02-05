use graphoid::execution::Executor;
use graphoid::values::{Value, ValueKind};
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

// =============================================================================
// Phase 17: Selective import tests
// =============================================================================

#[test]
fn test_selective_import_binds_function() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { public_fn }
        result = public_fn(5)
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(50.0));
}

#[test]
fn test_selective_import_binds_variable() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { public_var }
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let value = executor.get_variable("public_var").unwrap();
    assert_eq!(value, Value::number(99.0));
}

#[test]
fn test_selective_import_with_alias() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { public_fn as pf }
        result = pf(3)
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(30.0));
}

#[test]
fn test_selective_import_multiple() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { public_fn, another_fn }
        r1 = public_fn(2)
        r2 = another_fn(10)
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    assert_eq!(executor.get_variable("r1").unwrap(), Value::number(20.0));
    assert_eq!(executor.get_variable("r2").unwrap(), Value::number(11.0));
}

#[test]
fn test_selective_import_does_not_bind_module() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { public_fn }
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    // The module itself should NOT be bound
    assert!(executor.get_variable("seltest").is_none(),
        "Module should not be bound after selective import");
    assert!(executor.get_variable("selective_test_module").is_none(),
        "Module should not be bound after selective import");
}

#[test]
fn test_selective_import_private_symbol_error() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { secret_fn }
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    let result = executor.execute_source(source);
    assert!(result.is_err(), "Should error when importing private symbol");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("private") || err.contains("secret_fn"),
        "Error should mention private: {}", err);
}

#[test]
fn test_priv_block_public_api_accessible() {
    let mut executor = Executor::new();
    let source = r#"
        import "priv_block_module"
        result = privmod.public_api(5)
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    // public_api(5) => internal_helper(5) * 2 => (5+10) * 2 = 30
    let result = executor.get_variable("result").unwrap();
    assert_eq!(result, Value::number(30.0));
}

#[test]
fn test_priv_block_public_variable_accessible() {
    let mut executor = Executor::new();
    let source = r#"
        import "priv_block_module"
        val = privmod.public_value
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();
    assert_eq!(executor.get_variable("val").unwrap(), Value::number(100.0));
}

#[test]
fn test_priv_block_private_function_inaccessible() {
    let mut executor = Executor::new();
    let source = r#"
        import "priv_block_module"
        result = privmod.internal_helper(5)
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    let result = executor.execute_source(source);
    assert!(result.is_err(), "Should error accessing private function from priv block");
}

#[test]
fn test_priv_block_private_variable_inaccessible() {
    let mut executor = Executor::new();
    let source = r#"
        import "priv_block_module"
        val = privmod.secret_value
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    let result = executor.execute_source(source);
    assert!(result.is_err(), "Should error accessing private variable from priv block");
}

#[test]
fn test_selective_import_nonexistent_symbol_error() {
    let mut executor = Executor::new();
    let source = r#"
        import "selective_test_module" { nonexistent }
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    let result = executor.execute_source(source);
    assert!(result.is_err(), "Should error when importing nonexistent symbol");
}

// =============================================================================
// Phase 17: Module introspection tests
// =============================================================================

#[test]
fn test_module_introspection_name() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        result = sim.name()
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => assert_eq!(s, "simple"),
        _ => panic!("Expected string, got {:?}", result),
    }
}

#[test]
fn test_module_introspection_exports() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        result = sim.exports()
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::List(list) => {
            let names: Vec<String> = list.to_vec().iter().map(|v| {
                match &v.kind {
                    ValueKind::String(s) => s.clone(),
                    _ => panic!("Expected string in exports list"),
                }
            }).collect();
            assert!(names.contains(&"double".to_string()), "Should export 'double', got {:?}", names);
            assert!(names.contains(&"helper_value".to_string()), "Should export 'helper_value', got {:?}", names);
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_module_introspection_path() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        result = sim.path()
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::String(s) => {
            assert!(s.contains("simple_module"), "Path should contain module name, got: {}", s);
        }
        _ => panic!("Expected string, got {:?}", result),
    }
}

#[test]
fn test_module_introspection_imports() {
    let mut executor = Executor::new();
    let source = r#"
        import "simple_module"
        result = sim.imports()
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::List(_) => {
            // simple_module has no imports, so should be empty list
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}

#[test]
fn test_priv_block_exports_exclude_private() {
    let mut executor = Executor::new();
    let source = r#"
        import "priv_block_module"
        result = privmod.exports()
    "#;

    let test_file = PathBuf::from("tests/fixtures/modules/test.gr");
    executor.set_current_file(Some(test_file));

    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    match &result.kind {
        ValueKind::List(list) => {
            let names: Vec<String> = list.to_vec().iter().map(|v| {
                match &v.kind {
                    ValueKind::String(s) => s.clone(),
                    _ => panic!("Expected string in exports list"),
                }
            }).collect();
            assert!(names.contains(&"public_api".to_string()), "Should export 'public_api', got {:?}", names);
            assert!(names.contains(&"public_value".to_string()), "Should export 'public_value', got {:?}", names);
            assert!(!names.contains(&"internal_helper".to_string()), "Should NOT export 'internal_helper', got {:?}", names);
            assert!(!names.contains(&"secret_value".to_string()), "Should NOT export 'secret_value', got {:?}", names);
        }
        _ => panic!("Expected list, got {:?}", result),
    }
}
