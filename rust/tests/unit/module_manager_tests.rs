use graphoid::execution::module_manager::{ModuleManager, Module};
use graphoid::execution::environment::Environment;
use std::path::PathBuf;

#[test]
fn test_module_manager_creation() {
    let manager = ModuleManager::new();
    assert_eq!(manager.search_paths().len(), 3); // src/, lib/, stdlib/
}

#[test]
fn test_resolve_relative_path() {
    let manager = ModuleManager::new();
    let _current_file = PathBuf::from("/home/user/project/src/main.gr");

    // Create test file for resolution
    std::fs::create_dir_all("/tmp/graphoid_test/src").unwrap();
    std::fs::write("/tmp/graphoid_test/src/utils.gr", "# test file").unwrap();

    let current = PathBuf::from("/tmp/graphoid_test/src/main.gr");
    let result = manager.resolve_module_path("./utils.gr", Some(&current));
    assert!(result.is_ok());
    assert!(result.unwrap().to_string_lossy().contains("utils.gr"));
}

#[test]
fn test_resolve_parent_relative_path() {
    let manager = ModuleManager::new();

    // Create test files
    std::fs::create_dir_all("/tmp/graphoid_test/src/app").unwrap();
    std::fs::write("/tmp/graphoid_test/src/config.gr", "# config").unwrap();

    let current_file = PathBuf::from("/tmp/graphoid_test/src/app/server.gr");
    let result = manager.resolve_module_path("../config.gr", Some(&current_file));
    assert!(result.is_ok());
    assert!(result.unwrap().to_string_lossy().contains("config.gr"));
}

#[test]
fn test_resolve_with_gr_extension() {
    let manager = ModuleManager::new();

    // Create test file
    std::fs::create_dir_all("/tmp/graphoid_test/src").unwrap();
    std::fs::write("/tmp/graphoid_test/src/helpers.gr", "# helpers").unwrap();

    let current = PathBuf::from("/tmp/graphoid_test/src/main.gr");

    // Should work with or without .gr extension
    let result1 = manager.resolve_module_path("./helpers.gr", Some(&current));
    let result2 = manager.resolve_module_path("./helpers", Some(&current));

    assert!(result1.is_ok());
    assert!(result2.is_ok());

    // Paths should resolve to the same file
    let path1 = result1.unwrap().canonicalize().unwrap();
    let path2 = result2.unwrap().canonicalize().unwrap();
    assert_eq!(path1, path2);
}

#[test]
fn test_module_not_found_error() {
    let manager = ModuleManager::new();
    let result = manager.resolve_module_path("nonexistent/module", None);
    assert!(result.is_err());
}

#[test]
fn test_register_and_get_module() {
    let mut manager = ModuleManager::new();
    let module = Module {
        name: "test_module".to_string(),
        alias: None,
        namespace: Environment::new(),
        file_path: PathBuf::from("test.gr"),
        config: None,
    };

    manager.register_module("test_module".to_string(), module);
    assert!(manager.is_loaded("test_module"));
    assert!(manager.get_module("test_module").is_some());
}

#[test]
fn test_module_not_loaded() {
    let manager = ModuleManager::new();
    assert!(!manager.is_loaded("nonexistent"));
    assert!(manager.get_module("nonexistent").is_none());
}
