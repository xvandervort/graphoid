// Phase 20c: FFI Safety tests — taint tracking, bridge nodes, resource limits

use graphoid::values::{Value, propagate_taint};
use graphoid::ffi::limits::{FfiLimits, FfiUsage};

// =========================================================================
// Taint field basics
// =========================================================================

#[test]
fn test_value_untainted_by_default() {
    let v = Value::number(42.0);
    assert!(!v.tainted);
    assert!(v.taint_source.is_none());
}

#[test]
fn test_with_taint() {
    let v = Value::number(42.0).with_taint("bridge:func:sqrt".to_string());
    assert!(v.tainted);
    assert_eq!(v.taint_source, Some("bridge:func:sqrt".to_string()));
}

#[test]
fn test_is_tainted() {
    let clean = Value::number(1.0);
    let dirty = Value::string("hello".to_string()).with_taint("test".to_string());
    assert!(!clean.is_tainted());
    assert!(dirty.is_tainted());
}

#[test]
fn test_taint_excluded_from_equality() {
    let clean = Value::number(42.0);
    let dirty = Value::number(42.0).with_taint("test".to_string());
    assert_eq!(clean, dirty);
}

#[test]
fn test_taint_preserved_in_clone() {
    let v = Value::string("hello".to_string()).with_taint("src".to_string());
    let cloned = v.clone();
    assert!(cloned.tainted);
    assert_eq!(cloned.taint_source, Some("src".to_string()));
}

#[test]
fn test_taint_preserved_in_deep_copy_unfrozen() {
    let v = Value::string("hello".to_string()).with_taint("src".to_string());
    let copy = v.deep_copy_unfrozen();
    assert!(copy.tainted);
    assert_eq!(copy.taint_source, Some("src".to_string()));
}

#[test]
fn test_taint_preserved_in_deep_clone_for_send() {
    let v = Value::number(42.0).with_taint("ffi".to_string());
    let sendable = v.deep_clone_for_send();
    assert!(sendable.0.tainted);
    assert_eq!(sendable.0.taint_source, Some("ffi".to_string()));
}

// =========================================================================
// Taint propagation helper
// =========================================================================

#[test]
fn test_propagate_taint_clean_clean() {
    let left = Value::number(1.0);
    let right = Value::number(2.0);
    let result = propagate_taint(Value::number(3.0), &left, &right);
    assert!(!result.tainted);
    assert!(result.taint_source.is_none());
}

#[test]
fn test_propagate_taint_tainted_left() {
    let left = Value::number(1.0).with_taint("left_src".to_string());
    let right = Value::number(2.0);
    let result = propagate_taint(Value::number(3.0), &left, &right);
    assert!(result.tainted);
    assert_eq!(result.taint_source, Some("left_src".to_string()));
}

#[test]
fn test_propagate_taint_tainted_right() {
    let left = Value::number(1.0);
    let right = Value::number(2.0).with_taint("right_src".to_string());
    let result = propagate_taint(Value::number(3.0), &left, &right);
    assert!(result.tainted);
    assert_eq!(result.taint_source, Some("right_src".to_string()));
}

#[test]
fn test_propagate_taint_both_tainted() {
    let left = Value::number(1.0).with_taint("left_src".to_string());
    let right = Value::number(2.0).with_taint("right_src".to_string());
    let result = propagate_taint(Value::number(3.0), &left, &right);
    assert!(result.tainted);
    // Left source takes priority
    assert_eq!(result.taint_source, Some("left_src".to_string()));
}

// =========================================================================
// FfiLimits checks
// =========================================================================

#[test]
fn test_limits_default_no_limits() {
    let limits = FfiLimits::default();
    let usage = FfiUsage::default();
    assert!(limits.check_bridge_nodes(&usage).is_ok());
    assert!(limits.check_memory(&usage, 1000).is_ok());
    assert!(limits.check_libraries(&usage).is_ok());
    assert!(limits.check_pinned_callbacks(&usage).is_ok());
}

#[test]
fn test_limits_memory_exceeded() {
    let limits = FfiLimits {
        max_memory_bytes: Some(100),
        ..Default::default()
    };
    let usage = FfiUsage {
        allocated_bytes: 50,
        ..Default::default()
    };
    assert!(limits.check_memory(&usage, 30).is_ok());
    assert!(limits.check_memory(&usage, 60).is_err());
}

#[test]
fn test_limits_libraries_exceeded() {
    let limits = FfiLimits {
        max_libraries: Some(2),
        ..Default::default()
    };
    let mut usage = FfiUsage::default();
    assert!(limits.check_libraries(&usage).is_ok());
    usage.library_count = 2;
    assert!(limits.check_libraries(&usage).is_err());
}

#[test]
fn test_limits_bridge_nodes_exceeded() {
    let limits = FfiLimits {
        max_bridge_nodes: Some(5),
        ..Default::default()
    };
    let mut usage = FfiUsage::default();
    usage.bridge_nodes = 5;
    assert!(limits.check_bridge_nodes(&usage).is_err());
}

#[test]
fn test_limits_pinned_callbacks_exceeded() {
    let limits = FfiLimits {
        max_pinned_callbacks: Some(3),
        ..Default::default()
    };
    let mut usage = FfiUsage::default();
    usage.pinned_callbacks = 3;
    assert!(limits.check_pinned_callbacks(&usage).is_err());
}

// =========================================================================
// Integration tests (via executor)
// =========================================================================

use graphoid::execution_graph::graph_executor::GraphExecutor;

fn run_graphoid(code: &str) -> Result<String, String> {
    let mut executor = GraphExecutor::new();
    executor.enable_output_capture();
    executor.execute_source(code).map_err(|e| format!("{}", e))?;
    Ok(executor.get_captured_output())
}

#[test]
fn test_reflect_tainted_untainted_value() {
    let output = run_graphoid(r#"
x = 42
print(reflect.tainted(x))
"#).unwrap();
    assert_eq!(output.trim(), "false");
}

#[test]
fn test_ffi_trust_clears_taint() {
    // We test via reflect.tainted on a value we manually trust
    let output = run_graphoid(r#"
# Create a tainted value by calling a C function
lib = ffi.c("m")
lib.decl("sqrt", ["f64"], "f64")
result = lib.sqrt(4.0)
print(reflect.tainted(result))
trusted = ffi.trust(result)
print(reflect.tainted(trusted))
"#).unwrap();
    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(lines[0], "true");
    assert_eq!(lines[1], "false");
}

#[test]
fn test_reflect_taint_source() {
    let output = run_graphoid(r#"
lib = ffi.c("m")
lib.decl("sqrt", ["f64"], "f64")
result = lib.sqrt(4.0)
print(reflect.taint_source(result))
"#).unwrap();
    assert_eq!(output.trim(), "bridge:func:sqrt");
}

#[test]
fn test_taint_propagation_arithmetic() {
    let output = run_graphoid(r#"
lib = ffi.c("m")
lib.decl("sqrt", ["f64"], "f64")
tainted_val = lib.sqrt(4.0)
clean_val = 10
result = tainted_val + clean_val
print(reflect.tainted(result))
"#).unwrap();
    assert_eq!(output.trim(), "true");
}

#[test]
fn test_comparison_does_not_propagate_taint() {
    let output = run_graphoid(r#"
lib = ffi.c("m")
lib.decl("sqrt", ["f64"], "f64")
tainted_val = lib.sqrt(4.0)
result = tainted_val == 2.0
print(reflect.tainted(result))
"#).unwrap();
    assert_eq!(output.trim(), "false");
}

#[test]
fn test_exec_blocks_tainted_data() {
    // String + tainted_number triggers taint propagation via apply_scalar_op
    let result = run_graphoid(r#"
lib = ffi.c("m")
lib.decl("sqrt", ["f64"], "f64")
tainted_val = lib.sqrt(4.0)
path = "/tmp/test_" + tainted_val
exec(path)
"#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("tainted"), "Error should mention tainted: {}", err);
}

#[test]
fn test_ffi_limits_library_count() {
    let result = run_graphoid(r#"
ffi.limits({max_libraries: 1})
lib1 = ffi.c("m")
lib2 = ffi.c("c")
"#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("library limit"), "Error should mention library limit: {}", err);
}

#[test]
fn test_ffi_limits_memory() {
    let result = run_graphoid(r#"
ffi.limits({max_memory_bytes: 100})
ptr = ffi.alloc(200)
"#);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("memory limit"), "Error should mention memory limit: {}", err);
}

#[test]
fn test_bridge_nodes_in_universe_graph() {
    let output = run_graphoid(r#"
lib = ffi.c("m")
realm = reflect.foreign_realm()
print(realm.node_count())
"#).unwrap();
    // Should have at least 2 nodes: bridge:lib:m and bridge:lib:m:path
    let count: f64 = output.trim().parse().unwrap();
    assert!(count >= 2.0, "Expected at least 2 bridge nodes, got {}", count);
}

#[test]
fn test_reflect_bridge_returns_metadata() {
    let output = run_graphoid(r#"
lib = ffi.c("m")
bridge = reflect.bridge(lib)
print(bridge["node_id"])
"#).unwrap();
    assert_eq!(output.trim(), "bridge:lib:m");
}

#[test]
fn test_foreign_type_hierarchy_in_universe() {
    let output = run_graphoid(r#"
ug = reflect.universe()
print(ug.has_node("type:foreign"))
print(ug.has_node("type:ForeignLib"))
print(ug.has_node("type:ForeignPtr"))
"#).unwrap();
    let lines: Vec<&str> = output.trim().lines().collect();
    assert_eq!(lines[0], "true");
    assert_eq!(lines[1], "true");
    assert_eq!(lines[2], "true");
}
