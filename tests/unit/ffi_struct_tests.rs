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
// Struct creation
// =====================================================================

#[test]
fn test_struct_create_basic() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 1.0, y: 2.0})
        t = p.type()
    "#);
    assert_eq!(executor.get_variable("t").unwrap(), Value::string("foreign_struct".to_string()));
}

#[test]
fn test_struct_get_field() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 3.0, y: 4.0})
        x = p.get("x")
        y = p.get("y")
    "#);
    assert_eq!(executor.get_variable("x").unwrap(), Value::number(3.0));
    assert_eq!(executor.get_variable("y").unwrap(), Value::number(4.0));
}

#[test]
fn test_struct_set_field() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 0.0, y: 0.0})
        p.set("x", 5.0)
        p.set("y", 10.0)
        x = p.get("x")
        y = p.get("y")
    "#);
    assert_eq!(executor.get_variable("x").unwrap(), Value::number(5.0));
    assert_eq!(executor.get_variable("y").unwrap(), Value::number(10.0));
}

#[test]
fn test_struct_fields_map() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 1.5, y: 2.5})
        f = p.fields()
    "#);
    let val = executor.get_variable("f").unwrap();
    match &val.kind {
        ValueKind::Map(hash) => {
            let m = hash.to_hashmap();
            assert_eq!(m.get("x").unwrap(), &Value::number(1.5));
            assert_eq!(m.get("y").unwrap(), &Value::number(2.5));
        }
        _ => panic!("Expected map, got {}", val.type_name()),
    }
}

#[test]
fn test_struct_name() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 0.0, y: 0.0})
        n = p.struct_name()
    "#);
    assert_eq!(executor.get_variable("n").unwrap(), Value::string("Point".to_string()));
}

#[test]
fn test_struct_size() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 0.0, y: 0.0})
        s = p.size()
    "#);
    assert_eq!(executor.get_variable("s").unwrap(), Value::number(16.0));
}

#[test]
fn test_struct_address() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 0.0, y: 0.0})
        a = p.address()
    "#);
    let val = executor.get_variable("a").unwrap();
    let addr = match &val.kind { ValueKind::Number(n) => *n, _ => panic!("not number") };
    assert!(addr > 0.0);
}

#[test]
fn test_struct_integer_fields() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Pair { int32_t a; int32_t b; };")
        p = lib.new("Pair", {a: 42, b: 99})
        a = p.get("a")
        b = p.get("b")
    "#);
    assert_eq!(executor.get_variable("a").unwrap(), Value::number(42.0));
    assert_eq!(executor.get_variable("b").unwrap(), Value::number(99.0));
}

// =====================================================================
// Struct pass to C functions
// =====================================================================

#[test]
fn test_struct_pass_to_c() {
    // Use memset to zero out a struct, then read fields
    let executor = exec(r#"
        libc = ffi.c("c")
        libm = ffi.c("m")
        libm.cdef("struct Point { double x; double y; };")
        libc.cdef("void* memset(void* s, int c, size_t n);")
        p = libm.new("Point", {x: 5.0, y: 10.0})
        libc.memset(p, 0, 16)
        x = p.get("x")
        y = p.get("y")
    "#);
    assert_eq!(executor.get_variable("x").unwrap(), Value::number(0.0));
    assert_eq!(executor.get_variable("y").unwrap(), Value::number(0.0));
}

// =====================================================================
// Error cases
// =====================================================================

#[test]
fn test_struct_unknown_field_error() {
    let err = exec_err(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 0.0, y: 0.0})
        p.get("z")
    "#);
    assert!(err.contains("no field 'z'"), "Error: {}", err);
}

#[test]
fn test_struct_unknown_struct_error() {
    let err = exec_err(r#"
        lib = ffi.c("m")
        lib.new("NonExistent", {})
    "#);
    assert!(err.contains("no struct definition"), "Error: {}", err);
}

#[test]
fn test_struct_missing_field_error() {
    let err = exec_err(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 1.0})
    "#);
    assert!(err.contains("Missing field 'y'"), "Error: {}", err);
}

#[test]
fn test_struct_display() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        p = lib.new("Point", {x: 0.0, y: 0.0})
        s = p.to_string()
    "#);
    let val = executor.get_variable("s").unwrap();
    let s = match &val.kind { ValueKind::String(s) => s.as_str(), _ => panic!("not string") };
    assert!(s.starts_with("<foreign_struct:Point@"), "Display: {}", s);
}
