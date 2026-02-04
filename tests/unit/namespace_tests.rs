//! Unit tests for the NamespaceGraph (graph-based environment)
//!
//! These tests verify the Phase 15 implementation where variables are nodes
//! and scopes are subgraphs with parent edges.

use graphoid::namespace::{NamespaceGraph, ScopeType};
use graphoid::values::Value;

// =============================================================================
// Phase 1: Basic Creation and Variable Operations
// =============================================================================

#[test]
fn test_create_namespace() {
    let ns = NamespaceGraph::new();
    // New namespace should have a global scope
    assert_eq!(ns.current_scope_type(), ScopeType::Global);
}

#[test]
fn test_define_and_get() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(42.0));

    assert_eq!(ns.get("x").unwrap(), Value::number(42.0));
}

#[test]
fn test_get_undefined_returns_error() {
    let ns = NamespaceGraph::new();
    assert!(ns.get("undefined").is_err());
}

#[test]
fn test_define_multiple_variables() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(1.0));
    ns.define("y".to_string(), Value::number(2.0));
    ns.define("z".to_string(), Value::string("hello".to_string()));

    assert_eq!(ns.get("x").unwrap(), Value::number(1.0));
    assert_eq!(ns.get("y").unwrap(), Value::number(2.0));
    assert_eq!(ns.get("z").unwrap(), Value::string("hello".to_string()));
}

#[test]
fn test_define_overwrites_in_same_scope() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(1.0));
    ns.define("x".to_string(), Value::number(2.0));

    assert_eq!(ns.get("x").unwrap(), Value::number(2.0));
}

// =============================================================================
// Phase 2: Variable Mutation (set, exists)
// =============================================================================

#[test]
fn test_set_existing_variable() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(10.0));
    ns.set("x", Value::number(20.0)).unwrap();

    assert_eq!(ns.get("x").unwrap(), Value::number(20.0));
}

#[test]
fn test_set_undefined_returns_error() {
    let mut ns = NamespaceGraph::new();
    assert!(ns.set("undefined", Value::number(42.0)).is_err());
}

#[test]
fn test_exists_in_current_scope() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(42.0));

    assert!(ns.exists("x"));
    assert!(!ns.exists("y"));
}

// =============================================================================
// Phase 3: Nested Scopes (with_parent, parent traversal)
// =============================================================================

#[test]
fn test_nested_scope_get() {
    let mut parent = NamespaceGraph::new();
    parent.define("x".to_string(), Value::number(10.0));

    let child = NamespaceGraph::with_parent(parent);

    // Child can access parent's variables
    assert_eq!(child.get("x").unwrap(), Value::number(10.0));
}

#[test]
fn test_nested_scope_shadow() {
    let mut parent = NamespaceGraph::new();
    parent.define("x".to_string(), Value::number(10.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.define("x".to_string(), Value::number(20.0));

    // Child scope shadows parent
    assert_eq!(child.get("x").unwrap(), Value::number(20.0));
}

#[test]
fn test_nested_scope_set() {
    let mut parent = NamespaceGraph::new();
    parent.define("x".to_string(), Value::number(10.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.set("x", Value::number(30.0)).unwrap();

    // Setting in child modifies parent variable
    assert_eq!(child.get("x").unwrap(), Value::number(30.0));
}

#[test]
fn test_exists_in_parent_scope() {
    let mut parent = NamespaceGraph::new();
    parent.define("x".to_string(), Value::number(10.0));

    let child = NamespaceGraph::with_parent(parent);

    assert!(child.exists("x"));
    assert!(!child.exists("y"));
}

#[test]
fn test_deeply_nested_scopes() {
    let mut global = NamespaceGraph::new();
    global.define("a".to_string(), Value::number(1.0));

    let mut level1 = NamespaceGraph::with_parent(global);
    level1.define("b".to_string(), Value::number(2.0));

    let mut level2 = NamespaceGraph::with_parent(level1);
    level2.define("c".to_string(), Value::number(3.0));

    // Can access all levels
    assert_eq!(level2.get("a").unwrap(), Value::number(1.0));
    assert_eq!(level2.get("b").unwrap(), Value::number(2.0));
    assert_eq!(level2.get("c").unwrap(), Value::number(3.0));
}

// =============================================================================
// Phase 3: Scope Management (take_parent, variable_names, bindings)
// =============================================================================

#[test]
fn test_take_parent_extracts_parent() {
    let mut parent = NamespaceGraph::new();
    parent.define("x".to_string(), Value::number(10.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.define("y".to_string(), Value::number(20.0));

    let extracted = child.take_parent();
    assert!(extracted.is_some());

    let parent_box = extracted.unwrap();
    assert_eq!(parent_box.get("x").unwrap(), Value::number(10.0));
}

#[test]
fn test_take_parent_on_global_returns_none() {
    let mut ns = NamespaceGraph::new();
    assert!(ns.take_parent().is_none());
}

#[test]
fn test_get_variable_names() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(1.0));
    ns.define("y".to_string(), Value::number(2.0));
    ns.define("z".to_string(), Value::number(3.0));

    let names = ns.get_variable_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"x".to_string()));
    assert!(names.contains(&"y".to_string()));
    assert!(names.contains(&"z".to_string()));
}

#[test]
fn test_get_variable_names_current_scope_only() {
    let mut parent = NamespaceGraph::new();
    parent.define("parent_var".to_string(), Value::number(1.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.define("child_var".to_string(), Value::number(2.0));

    let names = child.get_variable_names();
    assert_eq!(names.len(), 1);
    assert!(names.contains(&"child_var".to_string()));
    assert!(!names.contains(&"parent_var".to_string()));
}

#[test]
fn test_remove_variable() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(42.0));

    assert!(ns.exists("x"));
    assert!(ns.remove_variable("x"));
    assert!(!ns.exists("x"));
}

#[test]
fn test_remove_nonexistent_returns_false() {
    let mut ns = NamespaceGraph::new();
    assert!(!ns.remove_variable("nonexistent"));
}

#[test]
fn test_get_all_bindings_current_scope() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(1.0));
    ns.define("y".to_string(), Value::number(2.0));

    let bindings = ns.get_all_bindings();
    assert_eq!(bindings.len(), 2);

    let binding_map: std::collections::HashMap<String, Value> = bindings.into_iter().collect();
    assert_eq!(binding_map.get("x").unwrap(), &Value::number(1.0));
    assert_eq!(binding_map.get("y").unwrap(), &Value::number(2.0));
}

#[test]
fn test_get_all_bindings_excludes_parent() {
    let mut parent = NamespaceGraph::new();
    parent.define("parent_var".to_string(), Value::number(1.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.define("child_var".to_string(), Value::number(2.0));

    let bindings = child.get_all_bindings();
    assert_eq!(bindings.len(), 1);
    assert_eq!(bindings[0].0, "child_var");
}

#[test]
fn test_get_all_bindings_recursive() {
    let mut parent = NamespaceGraph::new();
    parent.define("parent_var".to_string(), Value::number(1.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.define("child_var".to_string(), Value::number(2.0));

    let bindings = child.get_all_bindings_recursive();
    assert_eq!(bindings.len(), 2);

    let binding_map: std::collections::HashMap<String, Value> = bindings.into_iter().collect();
    assert_eq!(binding_map.get("parent_var").unwrap(), &Value::number(1.0));
    assert_eq!(binding_map.get("child_var").unwrap(), &Value::number(2.0));
}

#[test]
fn test_get_all_bindings_recursive_child_shadows_parent() {
    let mut parent = NamespaceGraph::new();
    parent.define("x".to_string(), Value::number(1.0));

    let mut child = NamespaceGraph::with_parent(parent);
    child.define("x".to_string(), Value::number(2.0));

    let bindings = child.get_all_bindings_recursive();
    assert_eq!(bindings.len(), 1); // Only one "x"

    let binding_map: std::collections::HashMap<String, Value> = bindings.into_iter().collect();
    assert_eq!(binding_map.get("x").unwrap(), &Value::number(2.0)); // Child's value
}

// =============================================================================
// Clone and Closure Semantics
// =============================================================================

#[test]
fn test_clone_creates_independent_copy() {
    let mut ns = NamespaceGraph::new();
    ns.define("x".to_string(), Value::number(10.0));

    let mut cloned = ns.clone();
    cloned.set("x", Value::number(20.0)).unwrap();

    // Original should be unchanged
    assert_eq!(ns.get("x").unwrap(), Value::number(10.0));
    assert_eq!(cloned.get("x").unwrap(), Value::number(20.0));
}

#[test]
fn test_default_creates_empty_global() {
    let ns = NamespaceGraph::default();
    assert_eq!(ns.current_scope_type(), ScopeType::Global);
}
