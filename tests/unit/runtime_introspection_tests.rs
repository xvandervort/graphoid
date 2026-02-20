// Phase 18.7: Runtime Introspection Tests
// Tests for runtime.*, modules.*, error.stack(), __MODULE__

use graphoid::execution_graph::graph_executor::GraphExecutor as Executor;
use graphoid::values::{Value, ValueKind};

// ============================================================================
// runtime.version()
// ============================================================================

#[test]
fn test_runtime_version_returns_string() {
    let mut executor = Executor::new();
    executor.execute_source(r#"v = runtime.version()"#).unwrap();
    let result = executor.get_variable("v").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(_)));
}

#[test]
fn test_runtime_version_value() {
    let mut executor = Executor::new();
    executor.execute_source(r#"v = runtime.version()"#).unwrap();
    let result = executor.get_variable("v").unwrap();
    assert_eq!(result, Value::string("0.1.0".to_string()));
}

#[test]
fn test_runtime_version_no_args_error() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"v = runtime.version("bad")"#);
    assert!(result.is_err());
}

// ============================================================================
// runtime.uptime()
// ============================================================================

#[test]
fn test_runtime_uptime_returns_number() {
    let mut executor = Executor::new();
    executor.execute_source(r#"t = runtime.uptime()"#).unwrap();
    let result = executor.get_variable("t").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(_)));
}

#[test]
fn test_runtime_uptime_is_non_negative() {
    let mut executor = Executor::new();
    executor.execute_source(r#"t = runtime.uptime()"#).unwrap();
    let result = executor.get_variable("t").unwrap();
    if let ValueKind::Number(n) = &result.kind {
        assert!(*n >= 0.0);
    } else {
        panic!("Expected number");
    }
}

#[test]
fn test_runtime_uptime_no_args_error() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"t = runtime.uptime(42)"#);
    assert!(result.is_err());
}

// ============================================================================
// runtime.memory()
// ============================================================================

#[test]
fn test_runtime_memory_returns_map() {
    let mut executor = Executor::new();
    executor.execute_source(r#"m = runtime.memory()"#).unwrap();
    let result = executor.get_variable("m").unwrap();
    assert!(matches!(&result.kind, ValueKind::Map(_)));
}

#[test]
fn test_runtime_memory_has_used_key() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        m = runtime.memory()
        u = m["used"]
    "#).unwrap();
    let result = executor.get_variable("u").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(_)));
}

// ============================================================================
// runtime.module_count()
// ============================================================================

#[test]
fn test_runtime_module_count_returns_number() {
    let mut executor = Executor::new();
    executor.execute_source(r#"c = runtime.module_count()"#).unwrap();
    let result = executor.get_variable("c").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(_)));
}

#[test]
fn test_runtime_module_count_starts_at_zero() {
    let mut executor = Executor::new();
    executor.execute_source(r#"c = runtime.module_count()"#).unwrap();
    let result = executor.get_variable("c").unwrap();
    if let ValueKind::Number(n) = &result.kind {
        assert_eq!(*n, 0.0);
    } else {
        panic!("Expected number");
    }
}

// ============================================================================
// runtime — error cases and shadowing
// ============================================================================

#[test]
fn test_runtime_unknown_method_errors() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"v = runtime.foo()"#);
    assert!(result.is_err());
}

#[test]
fn test_runtime_shadowed_by_variable() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        runtime = "my value"
        result = runtime
    "#).unwrap();
    let result = executor.get_variable("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "my value"));
}

// ============================================================================
// modules.list()
// ============================================================================

#[test]
fn test_modules_list_returns_list() {
    let mut executor = Executor::new();
    executor.execute_source(r#"m = modules.list()"#).unwrap();
    let result = executor.get_variable("m").unwrap();
    assert!(matches!(&result.kind, ValueKind::List(_)));
}

#[test]
fn test_modules_list_empty_when_no_imports() {
    let mut executor = Executor::new();
    executor.execute_source(r#"m = modules.list()"#).unwrap();
    let result = executor.get_variable("m").unwrap();
    if let ValueKind::List(list) = &result.kind {
        assert_eq!(list.len(), 0);
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_modules_list_no_args_error() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"m = modules.list("bad")"#);
    assert!(result.is_err());
}

// ============================================================================
// modules.info()
// ============================================================================

#[test]
fn test_modules_info_returns_none_for_unknown() {
    let mut executor = Executor::new();
    executor.execute_source(r#"info = modules.info("nonexistent")"#).unwrap();
    let result = executor.get_variable("info").unwrap();
    assert!(matches!(&result.kind, ValueKind::None));
}

#[test]
fn test_modules_info_requires_string_arg() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"info = modules.info(42)"#);
    assert!(result.is_err());
}

#[test]
fn test_modules_info_requires_one_arg() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"info = modules.info()"#);
    assert!(result.is_err());
}

// ============================================================================
// modules — error cases and shadowing
// ============================================================================

#[test]
fn test_modules_unknown_method_errors() {
    let mut executor = Executor::new();
    let result = executor.execute_source(r#"m = modules.foo()"#);
    assert!(result.is_err());
}

#[test]
fn test_modules_shadowed_by_variable() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        modules = "my value"
        result = modules
    "#).unwrap();
    let result = executor.get_variable("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::String(s) if s == "my value"));
}

// ============================================================================
// error.stack()
// ============================================================================

#[test]
fn test_error_stack_returns_list() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        trace = none
        try {
            raise("test error")
        } catch as e {
            trace = e.stack()
        }
    "#).unwrap();
    let result = executor.get_variable("trace").unwrap();
    assert!(matches!(&result.kind, ValueKind::List(_)));
}

#[test]
fn test_error_stack_from_nested_functions() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        fn inner() {
            raise("inner error")
        }
        fn outer() {
            inner()
        }
        trace = none
        try {
            outer()
        } catch as e {
            trace = e.stack()
        }
        trace_len = trace.length()
    "#).unwrap();
    let result = executor.get_variable("trace_len").unwrap();
    if let ValueKind::Number(n) = &result.kind {
        assert!(*n >= 2.0, "Stack trace should have at least 2 frames, got {}", n);
    } else {
        panic!("Expected number, got {:?}", result.kind);
    }
}

#[test]
fn test_error_stack_frame_is_map() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        fn failing() {
            raise("boom")
        }
        frame = none
        try {
            failing()
        } catch as e {
            trace = e.stack()
            frame = trace[0]
        }
        frame_type = typeof(frame)
    "#).unwrap();
    let result = executor.get_variable("frame_type").unwrap();
    assert_eq!(result, Value::string("map".to_string()));
}

#[test]
fn test_error_stack_frame_has_function_key() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        fn failing() {
            raise("boom")
        }
        func_name = ""
        try {
            failing()
        } catch as e {
            trace = e.stack()
            func_name = trace[0]["function"]
        }
    "#).unwrap();
    let result = executor.get_variable("func_name").unwrap();
    assert_eq!(result, Value::string("failing".to_string()));
}

#[test]
fn test_error_stack_preserves_order() {
    let mut executor = Executor::new();
    executor.execute_source(r#"
        fn a() { raise("boom") }
        fn b() { a() }
        fn c() { b() }
        names = []
        try {
            c()
        } catch as e {
            trace = e.stack()
            for frame in trace {
                names.append!(frame["function"])
            }
        }
    "#).unwrap();
    let result = executor.get_variable("names").unwrap();
    if let ValueKind::List(list) = &result.kind {
        let items: Vec<String> = list.to_vec().iter()
            .map(|v| v.to_string())
            .collect();
        // Innermost first: a, b, c
        assert_eq!(items[0], "a");
        assert_eq!(items[1], "b");
        assert_eq!(items[2], "c");
    } else {
        panic!("Expected list");
    }
}

// ============================================================================
// __MODULE__
// ============================================================================

#[test]
fn test_module_var_is_main_in_top_level() {
    let mut executor = Executor::new();
    executor.execute_source(r#"m = __MODULE__"#).unwrap();
    let result = executor.get_variable("m").unwrap();
    assert_eq!(result, Value::string("__main__".to_string()));
}
