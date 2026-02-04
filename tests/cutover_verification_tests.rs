/// Cutover verification tests for Phase 15 (NamespaceGraph) and Phase 16 (GraphExecutor).
/// These tests verify that the graph-based implementations are the active types.

use graphoid::execution::{Executor, Environment};

fn execute_and_get_var(source: &str, var: &str) -> graphoid::values::Value {
    let mut exec = Executor::new();
    exec.execute_source(source).expect("execute_source failed");
    exec.get_variable(var).expect(&format!("variable '{}' not found", var))
}

#[test]
fn test_executor_is_graph_executor() {
    let type_name = std::any::type_name::<Executor>();
    assert!(
        type_name.contains("GraphExecutor"),
        "Executor should be GraphExecutor, but type_name is: {}",
        type_name
    );
}

#[test]
fn test_list_static_method_available() {
    // list.generate(start, end, step) generates a numeric range
    let val = execute_and_get_var(r#"result = list.generate(1, 5, 1)"#, "result");
    if let graphoid::values::ValueKind::List(l) = &val.kind {
        assert_eq!(l.len(), 5);
    } else {
        panic!("Expected list, got {:?}", val.kind);
    }
}

#[test]
fn test_string_static_method_available() {
    let val = execute_and_get_var(r#"result = string.generate("a", "e")"#, "result");
    if let graphoid::values::ValueKind::String(s) = &val.kind {
        assert_eq!(s.as_str(), "abcde");
    } else {
        panic!("Expected string, got {:?}", val.kind);
    }
}

#[test]
fn test_bitwise_delegation_works() {
    let val = execute_and_get_var("result = 5 & 3", "result");
    if let graphoid::values::ValueKind::Number(n) = &val.kind {
        assert_eq!(*n, 1.0); // 5 & 3 = 1
    } else {
        panic!("Expected number, got {:?}", val.kind);
    }
}

// --- Phase 15: NamespaceGraph cutover verification ---

#[test]
fn test_environment_is_namespace_graph() {
    let type_name = std::any::type_name::<Environment>();
    assert!(
        type_name.contains("NamespaceGraph"),
        "Environment should be NamespaceGraph, but type_name is: {}",
        type_name
    );
}

#[test]
fn test_namespace_graph_scope_type() {
    // current_scope_type() is only available on NamespaceGraph, not old Environment
    let env = Environment::new();
    let scope_type = env.current_scope_type();
    assert_eq!(scope_type, graphoid::namespace::ScopeType::Global);
}
