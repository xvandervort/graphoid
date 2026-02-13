use graphoid::execution::Executor;
use graphoid::values::ValueKind;

// ============================================================================
// Map patterns
// ============================================================================

#[test]
fn test_reflect_pattern_map_with_bindings() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ name: n, age: a })
        has_root = p.has_node("pattern:root")
        has_name = p.has_node("field:name")
        has_age = p.has_node("field:age")
        node_count = p.nodes().length()
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("has_root").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("has_name").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("has_age").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("node_count").unwrap().kind, ValueKind::Number(n) if *n == 3.0));
}

#[test]
fn test_reflect_pattern_map_has_field_edges() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ name: n, age: a })
        path1 = p.has_path("pattern:root", "field:name")
        path2 = p.has_path("pattern:root", "field:age")
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("path1").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("path2").unwrap().kind, ValueKind::Boolean(true)));
}

#[test]
fn test_reflect_pattern_map_with_literal_value() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ name: "Alice" })
        has_name = p.has_node("field:name")
        node_count = p.nodes().length()
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("has_name").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("node_count").unwrap().kind, ValueKind::Number(n) if *n == 2.0));
}

#[test]
fn test_reflect_pattern_map_binding_node_value_has_properties() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ name: n })
        root_val = p.get_node("pattern:root")
        field_val = p.get_node("field:name")
        pt = root_val["pattern_type"]
        ft = field_val["type"]
        fk = field_val["key"]
        fn_ = field_val["name"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "map"));
    assert!(matches!(&executor.env().get("ft").unwrap().kind, ValueKind::String(s) if s == "binding"));
    assert!(matches!(&executor.env().get("fk").unwrap().kind, ValueKind::String(s) if s == "name"));
    assert!(matches!(&executor.env().get("fn_").unwrap().kind, ValueKind::String(s) if s == "n"));
}

#[test]
fn test_reflect_pattern_map_literal_node_value_has_properties() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ age: 30 })
        field_val = p.get_node("field:age")
        ft = field_val["type"]
        fv = field_val["value"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("ft").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("fv").unwrap().kind, ValueKind::Number(n) if *n == 30.0));
}

#[test]
fn test_reflect_pattern_map_wildcard_field() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ name: _ })
        field_val = p.get_node("field:name")
        ft = field_val["type"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("ft").unwrap().kind, ValueKind::String(s) if s == "wildcard"));
}

// ============================================================================
// List patterns
// ============================================================================

#[test]
fn test_reflect_pattern_list_with_bindings() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern([x, y, z])
        has_root = p.has_node("pattern:root")
        has_e0 = p.has_node("element:0")
        has_e1 = p.has_node("element:1")
        has_e2 = p.has_node("element:2")
        node_count = p.nodes().length()
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("has_root").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("has_e0").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("has_e1").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("has_e2").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("node_count").unwrap().kind, ValueKind::Number(n) if *n == 4.0));
}

#[test]
fn test_reflect_pattern_list_has_element_edges() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern([x, y])
        path0 = p.has_path("pattern:root", "element:0")
        path1 = p.has_path("pattern:root", "element:1")
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("path0").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("path1").unwrap().kind, ValueKind::Boolean(true)));
}

#[test]
fn test_reflect_pattern_list_mixed_elements() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern([1, x, "hi", _])
        e0 = p.get_node("element:0")
        e1 = p.get_node("element:1")
        e2 = p.get_node("element:2")
        e3 = p.get_node("element:3")
        t0 = e0["type"]
        t1 = e1["type"]
        t2 = e2["type"]
        t3 = e3["type"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("t0").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("t1").unwrap().kind, ValueKind::String(s) if s == "binding"));
    assert!(matches!(&executor.env().get("t2").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("t3").unwrap().kind, ValueKind::String(s) if s == "wildcard"));
}

#[test]
fn test_reflect_pattern_list_element_node_properties() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern([x, 42])
        e0 = p.get_node("element:0")
        e1 = p.get_node("element:1")
        name0 = e0["name"]
        idx0 = e0["index"]
        val1 = e1["value"]
        idx1 = e1["index"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("name0").unwrap().kind, ValueKind::String(s) if s == "x"));
    assert!(matches!(&executor.env().get("idx0").unwrap().kind, ValueKind::Number(n) if *n == 0.0));
    assert!(matches!(&executor.env().get("val1").unwrap().kind, ValueKind::Number(n) if *n == 42.0));
    assert!(matches!(&executor.env().get("idx1").unwrap().kind, ValueKind::Number(n) if *n == 1.0));
}

// ============================================================================
// Literal patterns
// ============================================================================

#[test]
fn test_reflect_pattern_number_literal() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern(42)
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
        v = rv["value"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("v").unwrap().kind, ValueKind::Number(n) if *n == 42.0));
}

#[test]
fn test_reflect_pattern_string_literal() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern("hello")
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
        v = rv["value"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("v").unwrap().kind, ValueKind::String(s) if s == "hello"));
}

#[test]
fn test_reflect_pattern_bool_literal() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern(true)
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
        v = rv["value"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("v").unwrap().kind, ValueKind::Boolean(true)));
}

#[test]
fn test_reflect_pattern_none_literal() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern(none)
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "literal"));
}

#[test]
fn test_reflect_pattern_symbol_literal() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern(:foo)
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
        v = rv["value"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "literal"));
    assert!(matches!(&executor.env().get("v").unwrap().kind, ValueKind::Symbol(s) if s == "foo"));
}

// ============================================================================
// Binding and wildcard patterns
// ============================================================================

#[test]
fn test_reflect_pattern_single_binding() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern(x)
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
        name = rv["name"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "binding"));
    assert!(matches!(&executor.env().get("name").unwrap().kind, ValueKind::String(s) if s == "x"));
}

#[test]
fn test_reflect_pattern_wildcard() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern(_)
        rv = p.get_node("pattern:root")
        pt = rv["pattern_type"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("pt").unwrap().kind, ValueKind::String(s) if s == "wildcard"));
}

// ============================================================================
// Guard support
// ============================================================================

#[test]
fn test_reflect_pattern_with_guard() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ age: a }, a => a > 18)
        has_guard = p.has_node("guard:0")
        guard_path = p.has_path("pattern:root", "guard:0")
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("has_guard").unwrap().kind, ValueKind::Boolean(true)));
    assert!(matches!(&executor.env().get("guard_path").unwrap().kind, ValueKind::Boolean(true)));
}

#[test]
fn test_reflect_pattern_guard_node_value() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ age: a }, a => a > 18)
        gv = p.get_node("guard:0")
        gt = gv["type"]
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("gt").unwrap().kind, ValueKind::String(s) if s == "guard"));
}

// ============================================================================
// Error cases
// ============================================================================

#[test]
fn test_reflect_pattern_no_args_errors() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern()
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

#[test]
fn test_reflect_pattern_too_many_args_errors() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ a: b }, x => x, "extra")
    "#;
    let result = executor.execute_source(code);
    assert!(result.is_err());
}

// ============================================================================
// Return type is graph
// ============================================================================

#[test]
fn test_reflect_pattern_returns_graph() {
    let mut executor = Executor::new();
    let code = r#"
        p = reflect.pattern({ name: n })
        result = typeof(p)
    "#;
    executor.execute_source(code).unwrap();
    assert!(matches!(&executor.env().get("result").unwrap().kind, ValueKind::String(s) if s == "graph"));
}
