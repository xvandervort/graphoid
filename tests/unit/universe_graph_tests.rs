use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// Type hierarchy present at init
// ============================================================================

#[test]
fn test_universe_has_type_nodes_at_init() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        result = u.has_node("type:any")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_universe_has_all_type_nodes() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        types = ["type:any", "type:num", "type:int", "type:float", "type:bignum",
                 "type:string", "type:bool", "type:none", "type:symbol",
                 "type:collection", "type:list", "type:map", "type:graph",
                 "type:function", "type:module", "type:error", "type:time"]
        count = 0
        for t in types {
            if u.has_node(t) {
                count = count + 1
            }
        }
        result = count
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Number(n) if *n == 17.0));
}

#[test]
fn test_universe_has_scope_main_node() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        result = u.has_node("scope:main")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

// ============================================================================
// Type hierarchy edges (subtype_of paths)
// ============================================================================

#[test]
fn test_universe_int_is_subtype_of_num() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        result = u.has_path("type:int", "type:num")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_universe_int_has_path_to_any() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        result = u.has_path("type:int", "type:any")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_universe_list_is_subtype_of_collection() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        result = u.has_path("type:list", "type:collection")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_universe_string_not_subtype_of_num() {
    let mut executor = Executor::new();
    let code = r#"
        u = reflect.universe()
        result = u.has_path("type:string", "type:num")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(false)));
}

// ============================================================================
// Module nodes appear after import
// ============================================================================

#[test]
fn test_universe_has_module_node_after_import() {
    let mut executor = Executor::new();
    let code = r#"
        import "math"
        u = reflect.universe()
        result = u.has_node("module:math")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_universe_has_import_edge_after_import() {
    let mut executor = Executor::new();
    let code = r#"
        import "math"
        u = reflect.universe()
        result = u.has_path("scope:main", "module:math")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

// ============================================================================
// reflect.type_hierarchy()
// ============================================================================

#[test]
fn test_type_hierarchy_returns_graph() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.type_hierarchy()
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Graph(_)));
}

#[test]
fn test_type_hierarchy_has_only_type_nodes() {
    let mut executor = Executor::new();
    let code = r#"
        import "math"
        th = reflect.type_hierarchy()
        has_type = th.has_node("type:int")
        has_module = th.has_node("module:math")
        has_scope = th.has_node("scope:main")
    "#;
    executor.execute_source(code).unwrap();
    let has_type = executor.env().get("has_type").unwrap();
    let has_module = executor.env().get("has_module").unwrap();
    let has_scope = executor.env().get("has_scope").unwrap();
    assert!(matches!(&has_type.kind, ValueKind::Boolean(true)));
    assert!(matches!(&has_module.kind, ValueKind::Boolean(false)));
    assert!(matches!(&has_scope.kind, ValueKind::Boolean(false)));
}

#[test]
fn test_type_hierarchy_has_subtype_edges() {
    let mut executor = Executor::new();
    let code = r#"
        th = reflect.type_hierarchy()
        result = th.has_path("type:float", "type:any")
    "#;
    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();
    assert!(matches!(&result.kind, ValueKind::Boolean(true)));
}

#[test]
fn test_type_hierarchy_no_args() {
    let mut executor = Executor::new();
    let code = r#"
        result = reflect.type_hierarchy("bad")
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

// ============================================================================
// Snapshot semantics (clone, not live reference)
// ============================================================================

#[test]
fn test_universe_is_snapshot() {
    let mut executor = Executor::new();
    let code = r#"
        u1 = reflect.universe()
        before = u1.has_node("module:math")
        import "math"
        after = u1.has_node("module:math")
    "#;
    executor.execute_source(code).unwrap();
    let before = executor.env().get("before").unwrap();
    let after = executor.env().get("after").unwrap();
    // u1 was captured before import, so it should NOT have the module node
    assert!(matches!(&before.kind, ValueKind::Boolean(false)));
    assert!(matches!(&after.kind, ValueKind::Boolean(false)));
}
