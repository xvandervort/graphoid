mod execution_graph_test_helpers;
use execution_graph_test_helpers::{eval_var, as_number, as_string, as_bool};
use graphoid::values::ValueKind;

// --- String methods ---

#[test]
fn test_string_length_method() {
    let val = eval_var("result = \"hello\".length()", "result");
    assert_eq!(as_number(&val), 5.0);
}

#[test]
fn test_string_upper_method() {
    let val = eval_var("result = \"hello\".upper()", "result");
    assert_eq!(as_string(&val), "HELLO");
}

#[test]
fn test_string_lower_method() {
    let val = eval_var("result = \"HELLO\".lower()", "result");
    assert_eq!(as_string(&val), "hello");
}

#[test]
fn test_string_contains_method() {
    let val = eval_var("result = \"hello world\".contains(\"world\")", "result");
    assert_eq!(as_bool(&val), true);
}

#[test]
fn test_string_split_method() {
    let val = eval_var("result = \"a,b,c\".split(\",\")", "result");
    assert!(matches!(val.kind, ValueKind::List(_)));
}

// --- List methods ---

#[test]
fn test_list_length_method() {
    let val = eval_var("result = [1, 2, 3].length()", "result");
    assert_eq!(as_number(&val), 3.0);
}

#[test]
fn test_list_append_method() {
    // In Graphoid, append returns a new list (no in-place mutation)
    let source = r#"
items = [1, 2]
items2 = items.append(3)
result = items2.length()
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 3.0);
}

#[test]
fn test_list_map_method() {
    let source = r#"
items = [1, 2, 3]
result = items.map(x => x * 2)
"#;
    let val = eval_var(source, "result");
    // Verify it's a list
    assert!(matches!(val.kind, ValueKind::List(_)));
}

#[test]
fn test_list_filter_method() {
    let source = r#"
items = [1, 2, 3, 4, 5]
result = items.filter(x => x > 3)
"#;
    let val = eval_var(source, "result");
    assert!(matches!(val.kind, ValueKind::List(_)));
}

// --- Map methods ---

#[test]
fn test_map_keys_method() {
    let val = eval_var("result = {\"a\": 1, \"b\": 2}.keys()", "result");
    assert!(matches!(val.kind, ValueKind::List(_)));
}

#[test]
fn test_map_values_method() {
    let val = eval_var("result = {\"a\": 1, \"b\": 2}.values()", "result");
    assert!(matches!(val.kind, ValueKind::List(_)));
}

// --- Index access ---

#[test]
fn test_list_index_access() {
    let val = eval_var("result = [10, 20, 30][1]", "result");
    assert_eq!(as_number(&val), 20.0);
}

#[test]
fn test_map_index_access() {
    let val = eval_var("result = {\"a\": 42}[\"a\"]", "result");
    assert_eq!(as_number(&val), 42.0);
}

#[test]
fn test_string_index_access() {
    let val = eval_var("result = \"hello\"[0]", "result");
    assert_eq!(as_string(&val), "h");
}

// --- Index assignment ---

#[test]
fn test_list_index_assignment() {
    let source = r#"
items = [1, 2, 3]
items[1] = 99
result = items[1]
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 99.0);
}

#[test]
fn test_map_index_assignment() {
    let source = r#"
m = {"a": 1}
m["b"] = 2
result = m["b"]
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_number(&val), 2.0);
}

// --- Property access ---

#[test]
fn test_property_access_on_map() {
    // In Graphoid, map.key is property access on maps
    let source = r#"
m = {"name": "Alice"}
result = m["name"]
"#;
    let val = eval_var(source, "result");
    assert_eq!(as_string(&val), "Alice");
}
