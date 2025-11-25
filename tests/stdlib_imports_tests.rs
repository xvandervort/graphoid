use graphoid::execution::Executor;
use graphoid::values::ValueKind;

#[test]
fn test_import_json_module() {
    let mut executor = Executor::new();

    let source = r#"
        import "json"
        result = json.stringify({})
    "#;
    executor.execute_source(source).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(_)));
}

#[test]
fn test_import_string_module() {
    let mut executor = Executor::new();

    let source = r#"
        import "string"
        result = str.uppercase("hello")
    "#;
    executor.execute_source(source).unwrap();

    // Module should be accessible via its alias "str"
    let result = executor.get_variable("result");
    assert!(result.is_some());
}

#[test]
fn test_import_math_module() {
    let mut executor = Executor::new();

    let source = r#"
        import "math"
        pi_value = math.pi
    "#;
    executor.execute_source(source).unwrap();

    let pi_value = executor.get_variable("pi_value").unwrap();
    match &pi_value.kind {
        ValueKind::Number(n) => {
            assert!((n - 3.141592653589793).abs() < 0.0001);
        }
        _ => panic!("pi should be a number"),
    }
}

#[test]
fn test_math_module_functions() {
    let mut executor = Executor::new();

    let source = r#"
        import "math"
        result_max = math.max(10, 20)
        result_min = math.min(10, 20)
        result_abs = math.abs(-5)
    "#;
    executor.execute_source(source).unwrap();

    let result_max = executor.get_variable("result_max").unwrap();
    match &result_max.kind {
        ValueKind::Number(n) => assert_eq!(*n, 20.0),
        _ => panic!("result_max should be number"),
    }

    let result_min = executor.get_variable("result_min").unwrap();
    match &result_min.kind {
        ValueKind::Number(n) => assert_eq!(*n, 10.0),
        _ => panic!("result_min should be number"),
    }

    let result_abs = executor.get_variable("result_abs").unwrap();
    match &result_abs.kind {
        ValueKind::Number(n) => assert_eq!(*n, 5.0),
        _ => panic!("result_abs should be number"),
    }
}

#[test]
fn test_stdlib_module_caching() {
    let mut executor = Executor::new();

    // Import same module twice - should use cached version
    let source = r#"
        import "math"
        import "math"
        pi1 = math.pi
    "#;
    executor.execute_source(source).unwrap();

    let pi1 = executor.get_variable("pi1").unwrap();
    assert!(matches!(&pi1.kind, ValueKind::Number(_)));
}

#[test]
fn test_multiple_stdlib_imports() {
    let mut executor = Executor::new();

    let source = r#"
        import "math"
        import "json"

        pi_value = math.pi
        json_result = json.stringify({})
    "#;
    executor.execute_source(source).unwrap();

    let pi_value = executor.get_variable("pi_value");
    assert!(pi_value.is_some());

    let json_result = executor.get_variable("json_result");
    assert!(json_result.is_some());
}
