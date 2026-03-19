use graphoid::ffi::cdef_parser::{parse_cdef, CdefItem};
use graphoid::ffi::types::{FfiType, FfiCallbackSig};
use graphoid::execution_graph::graph_executor::GraphExecutor;
use graphoid::values::{Value, ValueKind};

// =====================================================================
// cdef parser unit tests
// =====================================================================

#[test]
fn test_cdef_parse_simple_function() {
    let items = parse_cdef("int abs(int x);").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Function(decl) => {
            assert_eq!(decl.name, "abs");
            assert_eq!(decl.params.len(), 1);
            assert_eq!(decl.params[0], FfiType::Int);
            assert_eq!(decl.return_type, FfiType::Int);
        }
        _ => panic!("Expected function"),
    }
}

#[test]
fn test_cdef_parse_string_param() {
    let items = parse_cdef("size_t strlen(const char* s);").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Function(decl) => {
            assert_eq!(decl.name, "strlen");
            assert_eq!(decl.params, vec![FfiType::Str]);
            assert_eq!(decl.return_type, FfiType::USize);
        }
        _ => panic!("Expected function"),
    }
}

#[test]
fn test_cdef_parse_void_return() {
    let items = parse_cdef("void free(void* ptr);").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Function(decl) => {
            assert_eq!(decl.name, "free");
            assert_eq!(decl.params, vec![FfiType::Ptr]);
            assert_eq!(decl.return_type, FfiType::Void);
        }
        _ => panic!("Expected function"),
    }
}

#[test]
fn test_cdef_parse_double_params() {
    let items = parse_cdef("double pow(double x, double y);").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Function(decl) => {
            assert_eq!(decl.name, "pow");
            assert_eq!(decl.params, vec![FfiType::F64, FfiType::F64]);
            assert_eq!(decl.return_type, FfiType::F64);
        }
        _ => panic!("Expected function"),
    }
}

#[test]
fn test_cdef_parse_struct() {
    let items = parse_cdef("struct Point { double x; double y; };").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Struct(def) => {
            assert_eq!(def.name, "Point");
            assert_eq!(def.fields.len(), 2);
            assert_eq!(def.fields[0].name, "x");
            assert_eq!(def.fields[0].ffi_type, FfiType::F64);
            assert_eq!(def.fields[0].offset, 0);
            assert_eq!(def.fields[1].name, "y");
            assert_eq!(def.fields[1].ffi_type, FfiType::F64);
            assert_eq!(def.fields[1].offset, 8);
            assert_eq!(def.size, 16);
        }
        _ => panic!("Expected struct"),
    }
}

#[test]
fn test_cdef_parse_struct_with_padding() {
    let items = parse_cdef("struct Mixed { int8_t a; double b; int32_t c; };").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Struct(def) => {
            assert_eq!(def.name, "Mixed");
            assert_eq!(def.fields.len(), 3);
            // a at offset 0, size 1
            assert_eq!(def.fields[0].offset, 0);
            assert_eq!(def.fields[0].size, 1);
            // b at offset 8 (aligned to 8), size 8
            assert_eq!(def.fields[1].offset, 8);
            assert_eq!(def.fields[1].size, 8);
            // c at offset 16 (aligned to 4), size 4
            assert_eq!(def.fields[2].offset, 16);
            assert_eq!(def.fields[2].size, 4);
            // total: 20, padded to 24 (alignment 8)
            assert_eq!(def.size, 24);
            assert_eq!(def.alignment, 8);
        }
        _ => panic!("Expected struct"),
    }
}

#[test]
fn test_cdef_parse_opaque_typedef() {
    let items = parse_cdef("typedef struct sqlite3 sqlite3;").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::OpaqueType(name) => {
            assert_eq!(name, "sqlite3");
        }
        _ => panic!("Expected opaque type"),
    }
}

#[test]
fn test_cdef_parse_multiple() {
    let items = parse_cdef(r#"
        typedef struct sqlite3 sqlite3;
        int sqlite3_open(const char* filename, void* ppDb);
        int sqlite3_close(void* db);
    "#).unwrap();
    assert_eq!(items.len(), 3);
    assert!(matches!(&items[0], CdefItem::OpaqueType(_)));
    assert!(matches!(&items[1], CdefItem::Function(_)));
    assert!(matches!(&items[2], CdefItem::Function(_)));
}

#[test]
fn test_cdef_parse_callback_param() {
    let items = parse_cdef(
        "void qsort(void* base, size_t nmemb, size_t size, int (*compar)(const void*, const void*));"
    ).unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Function(decl) => {
            assert_eq!(decl.name, "qsort");
            assert_eq!(decl.params.len(), 4);
            assert_eq!(decl.params[0], FfiType::Ptr);
            assert_eq!(decl.params[1], FfiType::USize);
            assert_eq!(decl.params[2], FfiType::USize);
            match &decl.params[3] {
                FfiType::Callback(sig) => {
                    assert_eq!(sig.params, vec![FfiType::Ptr, FfiType::Ptr]);
                    assert_eq!(*sig.return_type, FfiType::Int);
                }
                other => panic!("Expected callback, got {:?}", other),
            }
        }
        _ => panic!("Expected function"),
    }
}

#[test]
fn test_cdef_parse_typedef_struct_body() {
    let items = parse_cdef("typedef struct Vec2 { float x; float y; } Vec2;").unwrap();
    assert_eq!(items.len(), 1);
    match &items[0] {
        CdefItem::Struct(def) => {
            assert_eq!(def.name, "Vec2");
            assert_eq!(def.fields.len(), 2);
            assert_eq!(def.size, 8);
        }
        _ => panic!("Expected struct"),
    }
}

// =====================================================================
// Type size and alignment tests
// =====================================================================

#[test]
fn test_ffi_type_sizes() {
    assert_eq!(FfiType::I8.size(), 1);
    assert_eq!(FfiType::U8.size(), 1);
    assert_eq!(FfiType::I16.size(), 2);
    assert_eq!(FfiType::U16.size(), 2);
    assert_eq!(FfiType::I32.size(), 4);
    assert_eq!(FfiType::U32.size(), 4);
    assert_eq!(FfiType::Int.size(), 4);
    assert_eq!(FfiType::F32.size(), 4);
    assert_eq!(FfiType::I64.size(), 8);
    assert_eq!(FfiType::U64.size(), 8);
    assert_eq!(FfiType::F64.size(), 8);
    assert_eq!(FfiType::Ptr.size(), std::mem::size_of::<*mut u8>());
    assert_eq!(FfiType::Str.size(), std::mem::size_of::<*mut u8>());
}

#[test]
fn test_ffi_type_alignment() {
    assert_eq!(FfiType::I8.alignment(), 1);
    assert_eq!(FfiType::I32.alignment(), 4);
    assert_eq!(FfiType::F64.alignment(), 8);
    assert_eq!(FfiType::Ptr.alignment(), std::mem::align_of::<*mut u8>());
}

#[test]
fn test_struct_layout_no_padding() {
    use graphoid::ffi::types::FfiStructDef;
    let def = FfiStructDef::compute_layout("Pair".to_string(), vec![
        ("x".to_string(), FfiType::F64),
        ("y".to_string(), FfiType::F64),
    ]);
    assert_eq!(def.size, 16);
    assert_eq!(def.alignment, 8);
    assert_eq!(def.fields[0].offset, 0);
    assert_eq!(def.fields[1].offset, 8);
}

// =====================================================================
// Integration tests: cdef through executor
// =====================================================================

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

#[test]
fn test_lib_cdef_function_then_call() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.cdef("int abs(int x);")
        result = lib.abs(-42)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(42.0));
}

#[test]
fn test_lib_cdef_multiple_functions() {
    let executor = exec(r#"
        lib = ffi.c("c")
        lib.cdef("
            int abs(int x);
            size_t strlen(const char* s);
        ")
        a = lib.abs(-7)
        b = lib.strlen("hello")
    "#);
    assert_eq!(executor.get_variable("a").unwrap(), Value::number(7.0));
    assert_eq!(executor.get_variable("b").unwrap(), Value::number(5.0));
}

#[test]
fn test_lib_cdef_struct_registered() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("struct Point { double x; double y; };")
        s = lib.structs()
    "#);
    let val = executor.get_variable("s").unwrap();
    match &val.kind {
        ValueKind::List(list) => {
            assert_eq!(list.len(), 1);
            assert_eq!(list.to_vec()[0], Value::string("Point".to_string()));
        }
        _ => panic!("Expected list, got {}", val.type_name()),
    }
}

#[test]
fn test_lib_cdef_libm_sqrt() {
    let executor = exec(r#"
        lib = ffi.c("m")
        lib.cdef("double sqrt(double x);")
        result = lib.sqrt(25.0)
    "#);
    assert_eq!(executor.get_variable("result").unwrap(), Value::number(5.0));
}
