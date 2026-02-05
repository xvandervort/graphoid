use graphoid::execution::module_manager::{ModuleManager, Module};
use graphoid::execution::Environment;
use std::path::PathBuf;

// =============================================================================
// Ticket 1: Module dependency graph in ModuleManager
// =============================================================================

fn make_test_module(name: &str) -> Module {
    Module {
        name: name.to_string(),
        alias: None,
        namespace: Environment::new(),
        file_path: PathBuf::from(format!("{}.gr", name)),
        config: None,
        private_symbols: std::collections::HashSet::new(),
        exports: vec![],
    }
}

fn make_test_module_with_exports(name: &str, exports: Vec<&str>) -> Module {
    Module {
        name: name.to_string(),
        alias: None,
        namespace: Environment::new(),
        file_path: PathBuf::from(format!("{}.gr", name)),
        config: None,
        private_symbols: std::collections::HashSet::new(),
        exports: exports.into_iter().map(|s| s.to_string()).collect(),
    }
}

#[test]
fn test_record_dependency_creates_forward_edge() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");

    let deps = manager.get_dependencies("app");
    assert_eq!(deps, vec!["math"]);
}

#[test]
fn test_record_dependency_creates_reverse_edge() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");

    let dependents = manager.get_dependents("math");
    assert_eq!(dependents, vec!["app"]);
}

#[test]
fn test_record_multiple_dependencies() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");
    manager.record_dependency("app", "json");
    manager.record_dependency("app", "http");

    let deps = manager.get_dependencies("app");
    assert_eq!(deps, vec!["http", "json", "math"]); // sorted
}

#[test]
fn test_multiple_dependents() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");
    manager.record_dependency("server", "math");
    manager.record_dependency("tests", "math");

    let dependents = manager.get_dependents("math");
    assert_eq!(dependents, vec!["app", "server", "tests"]); // sorted
}

#[test]
fn test_get_dependencies_unknown_module() {
    let manager = ModuleManager::new();
    let deps = manager.get_dependencies("nonexistent");
    assert!(deps.is_empty());
}

#[test]
fn test_get_dependents_unknown_module() {
    let manager = ModuleManager::new();
    let deps = manager.get_dependents("nonexistent");
    assert!(deps.is_empty());
}

#[test]
fn test_dependency_edges() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");
    manager.record_dependency("app", "json");
    manager.record_dependency("json", "io");

    let edges = manager.get_dependency_edges();
    assert_eq!(edges, vec![
        ("app".to_string(), "json".to_string()),
        ("app".to_string(), "math".to_string()),
        ("json".to_string(), "io".to_string()),
    ]);
}

#[test]
fn test_duplicate_dependency_is_idempotent() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");
    manager.record_dependency("app", "math"); // duplicate

    let deps = manager.get_dependencies("app");
    assert_eq!(deps, vec!["math"]); // only one entry
}

#[test]
fn test_topological_order_linear_chain() {
    let mut manager = ModuleManager::new();
    // app -> json -> io
    manager.register_module("app".to_string(), make_test_module("app"));
    manager.register_module("json".to_string(), make_test_module("json"));
    manager.register_module("io".to_string(), make_test_module("io"));
    manager.record_dependency("app", "json");
    manager.record_dependency("json", "io");

    let order = manager.topological_order().unwrap();
    // app must come before json, json before io
    let app_pos = order.iter().position(|x| x == "app").unwrap();
    let json_pos = order.iter().position(|x| x == "json").unwrap();
    let io_pos = order.iter().position(|x| x == "io").unwrap();
    assert!(app_pos < json_pos);
    assert!(json_pos < io_pos);
}

#[test]
fn test_topological_order_diamond() {
    let mut manager = ModuleManager::new();
    // app -> math, app -> json, math -> core, json -> core
    manager.register_module("app".to_string(), make_test_module("app"));
    manager.register_module("math".to_string(), make_test_module("math"));
    manager.register_module("json".to_string(), make_test_module("json"));
    manager.register_module("core".to_string(), make_test_module("core"));
    manager.record_dependency("app", "math");
    manager.record_dependency("app", "json");
    manager.record_dependency("math", "core");
    manager.record_dependency("json", "core");

    let order = manager.topological_order().unwrap();
    let app_pos = order.iter().position(|x| x == "app").unwrap();
    let math_pos = order.iter().position(|x| x == "math").unwrap();
    let json_pos = order.iter().position(|x| x == "json").unwrap();
    let core_pos = order.iter().position(|x| x == "core").unwrap();
    assert!(app_pos < math_pos);
    assert!(app_pos < json_pos);
    assert!(math_pos < core_pos);
    assert!(json_pos < core_pos);
}

#[test]
fn test_topological_order_cycle_returns_error() {
    let mut manager = ModuleManager::new();
    manager.register_module("a".to_string(), make_test_module("a"));
    manager.register_module("b".to_string(), make_test_module("b"));
    manager.record_dependency("a", "b");
    manager.record_dependency("b", "a");

    let result = manager.topological_order();
    assert!(result.is_err());
}

#[test]
fn test_find_cycles_no_cycles() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("app", "math");
    manager.record_dependency("math", "core");

    let cycles = manager.find_cycles();
    assert!(cycles.is_empty());
}

#[test]
fn test_find_cycles_simple_cycle() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("a", "b");
    manager.record_dependency("b", "a");

    let cycles = manager.find_cycles();
    assert!(!cycles.is_empty());
    // The cycle should contain both a and b
    let cycle = &cycles[0];
    assert!(cycle.contains(&"a".to_string()));
    assert!(cycle.contains(&"b".to_string()));
}

#[test]
fn test_find_cycles_three_node_cycle() {
    let mut manager = ModuleManager::new();
    manager.record_dependency("a", "b");
    manager.record_dependency("b", "c");
    manager.record_dependency("c", "a");

    let cycles = manager.find_cycles();
    assert!(!cycles.is_empty());
}

#[test]
fn test_module_exports_field() {
    let module = make_test_module_with_exports("math", vec!["sin", "cos", "sqrt"]);
    assert_eq!(module.exports, vec!["sin", "cos", "sqrt"]);
}

#[test]
fn test_get_all_module_keys() {
    let mut manager = ModuleManager::new();
    manager.register_module("math".to_string(), make_test_module("math"));
    manager.register_module("json".to_string(), make_test_module("json"));

    let keys = manager.get_all_module_keys();
    assert!(keys.contains(&"math".to_string()));
    assert!(keys.contains(&"json".to_string()));
}
