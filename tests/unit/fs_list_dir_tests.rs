// Tests for fs.list_dir() functionality

use graphoid::execution::Executor;
use graphoid::values::ValueKind;
use std::fs;

#[test]
fn test_fs_list_dir_basic() {
    let mut executor = Executor::new();

    // Create a test directory with some files
    let test_dir = "/tmp/graphoid_test_list_dir";
    fs::create_dir_all(test_dir).unwrap();
    fs::write(format!("{}/file1.txt", test_dir), "test1").unwrap();
    fs::write(format!("{}/file2.txt", test_dir), "test2").unwrap();
    fs::write(format!("{}/file3.gr", test_dir), "test3").unwrap();

    let code = r#"
        import "fs"
        result = fs.list_dir("/tmp/graphoid_test_list_dir")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    // Should return a list
    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        assert_eq!(items.len(), 3, "Should have 3 files");

        // Extract filenames
        let mut filenames: Vec<String> = items.iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        filenames.sort();
        assert_eq!(filenames, vec!["file1.txt", "file2.txt", "file3.gr"]);
    } else {
        panic!("Expected list, got {:?}", result.kind);
    }

    // Cleanup
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_fs_list_dir_empty_directory() {
    let mut executor = Executor::new();

    let test_dir = "/tmp/graphoid_test_empty_dir";
    fs::create_dir_all(test_dir).unwrap();

    let code = r#"
        import "fs"
        result = fs.list_dir("/tmp/graphoid_test_empty_dir")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    // Should return empty list
    if let ValueKind::List(list) = &result.kind {
        assert_eq!(list.to_vec().len(), 0, "Should be empty");
    } else {
        panic!("Expected list, got {:?}", result.kind);
    }

    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_fs_list_dir_nonexistent() {
    let mut executor = Executor::new();

    let code = r#"
        import "fs"
        result = fs.list_dir("/tmp/nonexistent_graphoid_dir_xyz")
    "#;

    let result = executor.execute_source(code);
    assert!(result.is_err(), "Should error on nonexistent directory");
}

#[test]
fn test_fs_list_dir_sorts_alphabetically() {
    let mut executor = Executor::new();

    let test_dir = "/tmp/graphoid_test_sorted_dir";
    fs::create_dir_all(test_dir).unwrap();
    fs::write(format!("{}/zebra.txt", test_dir), "z").unwrap();
    fs::write(format!("{}/apple.txt", test_dir), "a").unwrap();
    fs::write(format!("{}/middle.txt", test_dir), "m").unwrap();

    let code = r#"
        import "fs"
        result = fs.list_dir("/tmp/graphoid_test_sorted_dir")
    "#;

    executor.execute_source(code).unwrap();
    let result = executor.env().get("result").unwrap();

    if let ValueKind::List(list) = &result.kind {
        let items = list.to_vec();
        let filenames: Vec<String> = items.iter()
            .filter_map(|v| {
                if let ValueKind::String(s) = &v.kind {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .collect();

        assert_eq!(filenames, vec!["apple.txt", "middle.txt", "zebra.txt"]);
    } else {
        panic!("Expected list, got {:?}", result.kind);
    }

    fs::remove_dir_all(test_dir).unwrap();
}
