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
// Value type basics
// =====================================================================

#[test]
fn test_ffi_platform() {
    let executor = exec("p = ffi.platform()");
    let val = executor.get_variable("p").unwrap();
    let s = match &val.kind { ValueKind::String(s) => s.as_str(), _ => panic!("not string") };
    assert!(["linux", "macos", "windows"].contains(&s), "Unexpected platform: {}", s);
}

#[test]
fn test_ffi_arch() {
    let executor = exec("a = ffi.arch()");
    let val = executor.get_variable("a").unwrap();
    let s = match &val.kind { ValueKind::String(s) => s.as_str(), _ => panic!("not string") };
    assert!(["x86_64", "aarch64", "x86", "arm"].contains(&s), "Unexpected arch: {}", s);
}

#[test]
fn test_ffi_pointer_size() {
    let executor = exec("s = ffi.pointer_size()");
    let val = executor.get_variable("s").unwrap();
    assert_eq!(val, Value::number(std::mem::size_of::<*mut u8>() as f64));
}

#[test]
fn test_ffi_endian() {
    let executor = exec("e = ffi.endian()");
    let val = executor.get_variable("e").unwrap();
    let s = match &val.kind { ValueKind::String(s) => s.as_str(), _ => panic!("not string") };
    assert!(["little", "big"].contains(&s));
}

// =====================================================================
// Library loading
// =====================================================================

#[test]
fn test_ffi_c_load_libc() {
    let executor = exec(r#"lib = ffi.c("c")"#);
    let val = executor.get_variable("lib").unwrap();
    assert_eq!(val.type_name(), "foreign_lib");
}

#[test]
fn test_ffi_c_load_libm() {
    let executor = exec(r#"lib = ffi.c("m")"#);
    let val = executor.get_variable("lib").unwrap();
    assert_eq!(val.type_name(), "foreign_lib");
}

#[test]
fn test_ffi_c_load_nonexistent() {
    let err = exec_err(r#"lib = ffi.c("nonexistent_lib_xyz_12345")"#);
    assert!(err.contains("Failed to load library"), "Error: {}", err);
}

#[test]
fn test_ffi_lib_name() {
    let executor = exec(r#"
        lib = ffi.c("c")
        n = lib.name()
    "#);
    assert_eq!(executor.get_variable("n").unwrap(), Value::string("c".to_string()));
}

#[test]
fn test_ffi_lib_type() {
    let executor = exec(r#"
        lib = ffi.c("c")
        t = lib.type()
    "#);
    assert_eq!(executor.get_variable("t").unwrap(), Value::string("foreign_lib".to_string()));
}

// =====================================================================
// Function declaration
// =====================================================================

#[test]
fn test_ffi_decl_basic() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.decl("abs", ["int"], "int")
        d = lib.declarations()
    "#);
    let val = executor.get_variable("d").unwrap();
    match &val.kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 1);
            assert_eq!(list.to_vec()[0], Value::string("abs".to_string()));
        }
        _ => panic!("Expected list, got {}", val.type_name()),
    }
}

#[test]
fn test_ffi_decl_invalid_type() {
    let err = exec_err(r#"
        lib = ffi.c("c")
        lib.decl("abs", ["badtype"], "int")
    "#);
    assert!(err.contains("Unknown FFI type"), "Error: {}", err);
}

// =====================================================================
// Function calling
// =====================================================================

#[test]
fn test_ffi_call_abs() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.decl("abs", ["int"], "int")
        result = lib.abs(-42)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(42.0));
}

#[test]
fn test_ffi_call_abs_positive() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.decl("abs", ["int"], "int")
        result = lib.abs(7)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(7.0));
}

#[test]
fn test_ffi_call_strlen() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.decl("strlen", ["str"], "usize")
        result = lib.strlen("hello")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(5.0));
}

#[test]
fn test_ffi_call_strlen_empty() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.decl("strlen", ["str"], "usize")
        result = lib.strlen("")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(0.0));
}

#[test]
fn test_ffi_call_wrong_arg_count() {
    let err = exec_err(r#"
        lib = ffi.c("c")
        lib.decl("abs", ["int"], "int")
        result = lib.abs(1, 2)
    "#);
    assert!(err.contains("expects 1 arguments, got 2"), "Error: {}", err);
}

#[test]
fn test_ffi_call_undeclared() {
    let err = exec_err(r#"
        lib = ffi.c("c")
        result = lib.undeclared_function()
    "#);
    assert!(err.contains("no declared function"), "Error: {}", err);
}

#[test]
fn test_ffi_call_atoi() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.decl("atoi", ["str"], "int")
        result = lib.atoi("123")
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(123.0));
}

#[test]
fn test_ffi_call_libm_sqrt() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.decl("sqrt", ["double"], "double")
        result = lib.sqrt(16.0)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(4.0));
}

#[test]
fn test_ffi_call_libm_pow() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.decl("pow", ["double", "double"], "double")
        result = lib.pow(2.0, 10.0)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(1024.0));
}

// =====================================================================
// Pointer operations
// =====================================================================

#[test]
fn test_ffi_ptr_create() {
    let executor = exec("p = ffi.ptr()");
    let val = executor.get_variable("p").unwrap();
    assert_eq!(val.type_name(), "foreign_ptr");
}

#[test]
fn test_ffi_alloc_free() {
    let executor = exec(r#"
        buf = ffi.alloc(1024)
        t = buf.type()
        s1 = buf.state()
        ffi.free(buf)
        s2 = buf.state()
    "#);
    assert_eq!(executor.get_variable("t").unwrap(), Value::string("foreign_ptr".to_string()));
    assert_eq!(executor.get_variable("s1").unwrap(), Value::symbol("allocated".to_string()));
    assert_eq!(executor.get_variable("s2").unwrap(), Value::symbol("freed".to_string()));
}

#[test]
fn test_ffi_alloc_zero() {
    let err = exec_err("buf = ffi.alloc(0)");
    assert!(err.contains("size must be > 0"), "Error: {}", err);
}

#[test]
fn test_ffi_double_free() {
    let err = exec_err(r#"
        buf = ffi.alloc(64)
        ffi.free(buf)
        ffi.free(buf)
    "#);
    assert!(err.contains("Double free"), "Error: {}", err);
}

#[test]
fn test_ffi_ptr_address() {
    let executor = exec(r#"
        buf = ffi.alloc(64)
        addr = buf.address()
    "#);
    let val = executor.get_variable("addr").unwrap();
    let addr = match &val.kind { ValueKind::Number(n) => *n, _ => panic!("not number") };
    assert!(addr > 0.0);
}

#[test]
fn test_ffi_ptr_size() {
    let executor = exec(r#"
        buf = ffi.alloc(128)
        s = buf.size()
    "#);
    assert_eq!(executor.get_variable("s").unwrap(), Value::number(128.0));
}

#[test]
fn test_ffi_ptr_write_read() {
    let executor = exec(r#"
        buf = ffi.alloc(64)
        buf.write("hello world")
        result = buf.read_str()
        ffi.free(buf)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("hello world".to_string()));
}

#[test]
fn test_ffi_ptr_set_get_byte() {
    let executor = exec(r#"
        buf = ffi.alloc(8)
        buf.set(0, 65)
        buf.set(1, 66)
        buf.set(2, 0)
        result = buf.read_str()
        ffi.free(buf)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::string("AB".to_string()));
}

#[test]
fn test_ffi_use_after_free() {
    let err = exec_err(r#"
        buf = ffi.alloc(64)
        ffi.free(buf)
        buf.read_str()
    "#);
    assert!(err.contains("Use after free"), "Error: {}", err);
}

// =====================================================================
// Import syntax
// =====================================================================

#[test]
fn test_ffi_import_unsafe_no_error() {
    // `import "ffi" unsafe` should succeed (no-op, suppresses warning)
    let executor = exec(r#"
        import "ffi" unsafe
        p = ffi.platform()
    "#);
    let val = executor.get_variable("p").unwrap();
    assert_eq!(val.type_name(), "string");
}

#[test]
fn test_ffi_import_without_unsafe_no_error() {
    // `import "ffi"` should succeed (no-op, emits warning to stderr)
    let executor = exec(r#"
        import "ffi"
        p = ffi.platform()
    "#);
    let val = executor.get_variable("p").unwrap();
    assert_eq!(val.type_name(), "string");
}
