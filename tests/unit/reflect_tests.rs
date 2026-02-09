use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// reflect.loaded_modules()
// ============================================================================

#[test]
fn test_reflect_loaded_modules_returns_list() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.loaded_modules()
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::List(_)));
}

#[test]
fn test_reflect_loaded_modules_no_args() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.loaded_modules("bad")
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

// ============================================================================
// reflect.module()
// ============================================================================

#[test]
fn test_reflect_module_not_found_returns_none() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.module("nonexistent")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::None));
}

#[test]
fn test_reflect_module_requires_string_arg() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.module(42)
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

#[test]
fn test_reflect_module_requires_one_arg() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.module()
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

// ============================================================================
// reflect.current_scope()
// ============================================================================

#[test]
fn test_reflect_current_scope_returns_map() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.current_scope()
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Map(_)));
}

#[test]
fn test_reflect_current_scope_has_type_key() {
    let mut executor = Executor::new();
    let code = r#"
        scope = reflect.current_scope()
        scope_type = scope["type"]
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("scope_type").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "global"));
}

#[test]
fn test_reflect_current_scope_has_variables_key() {
    let mut executor = Executor::new();
    let code = r#"
        x = 42
        scope = reflect.current_scope()
        vars = scope["variables"]
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("vars").unwrap();
    assert!(matches!(&result.kind, ValueKind::List(_)));
}

#[test]
fn test_reflect_current_scope_has_depth_key() {
    let mut executor = Executor::new();
    let code = r#"
        scope = reflect.current_scope()
        d = scope["depth"]
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("d").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 0.0));
}

// ============================================================================
// reflect.universe()
// ============================================================================

#[test]
fn test_reflect_universe_returns_graph() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.universe()
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Graph(_)));
}

#[test]
fn test_reflect_universe_no_args() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.universe("bad")
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

// ============================================================================
// Unknown method
// ============================================================================

#[test]
fn test_reflect_unknown_method_errors() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.foo()
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

// ============================================================================
// reflect is shadowed by user variable
// ============================================================================

#[test]
fn test_reflect_shadowed_by_variable() {
    let mut executor = Executor::new();
    let code = r#"
        reflect = "my value"
        result = reflect
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "my value"));
}
