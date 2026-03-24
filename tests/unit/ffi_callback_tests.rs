use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::{Value, ValueKind};

fn exec(source: &str) -> GraphExecutor {
    let mut executor = GraphExecutor::new();
    executor.execute_source(source).unwrap();
    executor
}

fn exec_err(source: &str) -> String {
    let mut executor = GraphExecutor::new();
    match executor.execute_source(source) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("Expected error but execution succeeded"),
    }
}

// =====================================================================
// Pin/unpin (persistent callbacks)
// =====================================================================

#[test]
fn test_ffi_pin_creates_callback() {
    let executor = exec(r#"
        fn adder(a, b) { return a + b }
        cb = ffi.pin(adder, ["int", "int"], "int")
        t = cb.type()
    "#);
    assert_eq!(executor.get_variable("t").unwrap(), Value::string("foreign_callback".to_string()));
}

#[test]
fn test_ffi_pin_unpin() {
    exec(r#"
        fn doubler(x) { return x * 2 }
        cb = ffi.pin(doubler, ["int"], "int")
        ffi.unpin(cb)
    "#);
    // Should not panic
}

#[test]
fn test_ffi_double_unpin_error() {
    let err = exec_err(r#"
        fn identity(x) { return x }
        cb = ffi.pin(identity, ["int"], "int")
        ffi.unpin(cb)
        ffi.unpin(cb)
    "#);
    assert!(err.contains("already unpinned"), "Error: {}", err);
}

// =====================================================================
// Call-scoped callbacks (via cdef with function pointer params)
// =====================================================================

#[test]
fn test_callback_qsort_noop() {
    // qsort with a no-op comparator — just verifies the callback mechanism works
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.cdef("void qsort(void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*));")

        buf = ffi.alloc(16)
        buf.set(0, 4)  buf.set(1, 0)  buf.set(2, 0)  buf.set(3, 0)
        buf.set(4, 2)  buf.set(5, 0)  buf.set(6, 0)  buf.set(7, 0)
        buf.set(8, 3)  buf.set(9, 0)  buf.set(10, 0) buf.set(11, 0)
        buf.set(12, 1) buf.set(13, 0) buf.set(14, 0) buf.set(15, 0)

        fn noop_compare(a, b) { return 0 }
        lib.qsort(buf, 4, 4, noop_compare)

        result = "ok"
        ffi.free(buf)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("ok".to_string()));
}

#[test]
fn test_callback_display() {
    let executor = exec(r#"
        fn identity(x) { return x }
        cb = ffi.pin(identity, ["int"], "int")
        s = cb.to_string()
    "#);
    let val = executor.get_variable("s").unwrap();
    let s = match &val.kind { ValueKind::String(s) => s.as_str(), _ => panic!("not string") };
    assert!(s.starts_with("<foreign_callback:"), "Display: {}", s);
}

#[test]
fn test_callback_none_as_null() {
    // Passing none for a callback parameter should send NULL
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.cdef("void qsort(void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*));")
        buf = ffi.alloc(4)
        lib.qsort(buf, 0, 4, none)
        result = "ok"
        ffi.free(buf)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("ok".to_string()));
}
