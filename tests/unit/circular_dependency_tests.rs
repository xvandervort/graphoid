use graphoid::execution::module_manager::ModuleManager;
use graphoid::error::GraphoidError;
use std::path::PathBuf;

#[test]
fn test_begin_and_end_loading() {
    let mut manager = ModuleManager::new();
    let path = PathBuf::from("test.gr");

    assert!(!manager.is_loading(&path));

    manager.begin_loading(path.clone()).unwrap();
    assert!(manager.is_loading(&path));

    manager.end_loading(&path);
    assert!(!manager.is_loading(&path));
}

#[test]
fn test_detect_direct_circular_dependency() {
    let mut manager = ModuleManager::new();
    let path_a = PathBuf::from("a.gr");

    // Begin loading a.gr
    manager.begin_loading(path_a.clone()).unwrap();

    // Try to load a.gr again (circular!)
    let result = manager.check_circular(&path_a);
    assert!(result.is_err());

    // Verify error is CircularDependency
    match result {
        Err(GraphoidError::CircularDependency { chain, .. }) => {
            assert_eq!(chain.len(), 2);
            assert!(chain[0].contains("a.gr"));
            assert!(chain[1].contains("a.gr"));
        }
        _ => panic!("Expected CircularDependency error"),
    }
}

#[test]
fn test_detect_indirect_circular_dependency() {
    let mut manager = ModuleManager::new();
    let path_a = PathBuf::from("a.gr");
    let path_b = PathBuf::from("b.gr");
    let path_c = PathBuf::from("c.gr");

    // a.gr imports b.gr imports c.gr
    manager.begin_loading(path_a.clone()).unwrap();
    manager.begin_loading(path_b.clone()).unwrap();
    manager.begin_loading(path_c.clone()).unwrap();

    // c.gr tries to import a.gr → circular!
    let result = manager.check_circular(&path_a);
    assert!(result.is_err());

    match result {
        Err(GraphoidError::CircularDependency { chain, .. }) => {
            assert_eq!(chain.len(), 4); // a → b → c → a
            assert!(chain[0].contains("a.gr"));
            assert!(chain[1].contains("b.gr"));
            assert!(chain[2].contains("c.gr"));
            assert!(chain[3].contains("a.gr"));
        }
        _ => panic!("Expected CircularDependency error"),
    }
}

#[test]
fn test_no_circular_dependency_different_modules() {
    let mut manager = ModuleManager::new();
    let path_a = PathBuf::from("a.gr");
    let path_b = PathBuf::from("b.gr");

    manager.begin_loading(path_a.clone()).unwrap();

    // Loading b.gr is fine (not circular)
    let result = manager.check_circular(&path_b);
    assert!(result.is_ok());
}

#[test]
fn test_import_stack_tracking() {
    let mut manager = ModuleManager::new();

    assert_eq!(manager.import_stack_depth(), 0);

    manager.begin_loading(PathBuf::from("a.gr")).unwrap();
    assert_eq!(manager.import_stack_depth(), 1);

    manager.begin_loading(PathBuf::from("b.gr")).unwrap();
    assert_eq!(manager.import_stack_depth(), 2);

    manager.end_loading(&PathBuf::from("b.gr"));
    assert_eq!(manager.import_stack_depth(), 1);

    manager.end_loading(&PathBuf::from("a.gr"));
    assert_eq!(manager.import_stack_depth(), 0);
}
