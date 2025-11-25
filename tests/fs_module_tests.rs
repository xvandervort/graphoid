use graphoid::stdlib::{FSModule, NativeModule};
use graphoid::values::Value;
use std::fs;

#[test]
fn test_fs_module_name() {
    let module = FSModule;
    assert_eq!(module.name(), "fs");
    assert_eq!(module.alias(), None);
}

#[test]
fn test_fs_module_has_functions() {
    let module = FSModule;
    let functions = module.functions();

    assert!(functions.contains_key("open"));
    assert!(functions.contains_key("read"));
    assert!(functions.contains_key("write"));
    assert!(functions.contains_key("close"));
}

#[test]
fn test_fs_open_write_close() {
    let module = FSModule;
    let functions = module.functions();

    let test_path = "/tmp/graphoid_test_fs_open_write.txt";

    // Clean up any existing file
    let _ = fs::remove_file(test_path);

    // Open file for writing
    let open_fn = functions.get("open").unwrap();
    let result = open_fn(&[
        Value::string(test_path.to_string()),
        Value::string("w".to_string()),
    ]);
    assert!(result.is_ok());
    let file_id = result.unwrap();

    // Verify we got a numeric file ID
    match &file_id.kind {
        graphoid::values::ValueKind::Number(n) => {
            assert!(*n > 0.0);
        }
        _ => panic!("Expected number for file ID"),
    }

    // Write to file
    let write_fn = functions.get("write").unwrap();
    let write_result = write_fn(&[
        file_id.clone(),
        Value::string("Test content\n".to_string()),
    ]);
    assert!(write_result.is_ok());

    // Close file
    let close_fn = functions.get("close").unwrap();
    let close_result = close_fn(&[file_id]);
    assert!(close_result.is_ok());

    // Verify file exists
    assert!(std::path::Path::new(test_path).exists());

    // Clean up
    let _ = fs::remove_file(test_path);
}

#[test]
fn test_fs_open_read_close() {
    let module = FSModule;
    let functions = module.functions();

    let test_path = "/tmp/graphoid_test_fs_read.txt";

    // Create test file
    fs::write(test_path, "Test read content\n").unwrap();

    // Open file for reading
    let open_fn = functions.get("open").unwrap();
    let result = open_fn(&[
        Value::string(test_path.to_string()),
        Value::string("r".to_string()),
    ]);
    assert!(result.is_ok());
    let file_id = result.unwrap();

    // Read from file
    let read_fn = functions.get("read").unwrap();
    let read_result = read_fn(&[
        file_id.clone(),
        Value::number(1000.0),
    ]);
    assert!(read_result.is_ok());

    let content = read_result.unwrap();
    match &content.kind {
        graphoid::values::ValueKind::String(s) => {
            assert_eq!(s, "Test read content\n");
        }
        _ => panic!("Expected string for read content"),
    }

    // Close file
    let close_fn = functions.get("close").unwrap();
    let close_result = close_fn(&[file_id]);
    assert!(close_result.is_ok());

    // Clean up
    let _ = fs::remove_file(test_path);
}

#[test]
fn test_fs_open_append_close() {
    let module = FSModule;
    let functions = module.functions();

    let test_path = "/tmp/graphoid_test_fs_append.txt";

    // Create initial file
    fs::write(test_path, "Line 1\n").unwrap();

    // Open file for appending
    let open_fn = functions.get("open").unwrap();
    let result = open_fn(&[
        Value::string(test_path.to_string()),
        Value::string("a".to_string()),
    ]);
    assert!(result.is_ok());
    let file_id = result.unwrap();

    // Append to file
    let write_fn = functions.get("write").unwrap();
    let write_result = write_fn(&[
        file_id.clone(),
        Value::string("Line 2\n".to_string()),
    ]);
    assert!(write_result.is_ok());

    // Close file
    let close_fn = functions.get("close").unwrap();
    let _ = close_fn(&[file_id]);

    // Read back and verify
    let content = fs::read_to_string(test_path).unwrap();
    assert_eq!(content, "Line 1\nLine 2\n");

    // Clean up
    let _ = fs::remove_file(test_path);
}

#[test]
fn test_fs_invalid_mode() {
    let module = FSModule;
    let functions = module.functions();

    let open_fn = functions.get("open").unwrap();
    let result = open_fn(&[
        Value::string("/tmp/test.txt".to_string()),
        Value::string("invalid".to_string()),
    ]);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid mode"));
}

#[test]
fn test_fs_invalid_file_handle() {
    let module = FSModule;
    let functions = module.functions();

    // Try to read with invalid handle
    let read_fn = functions.get("read").unwrap();
    let result = read_fn(&[
        Value::number(99999.0),
        Value::number(100.0),
    ]);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Invalid file handle"));
}
