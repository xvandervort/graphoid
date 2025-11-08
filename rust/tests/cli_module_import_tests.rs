use std::process::Command;
use std::fs;

/// Test that CLI file execution can import modules using relative paths
/// This verifies the fix in src/main.rs that sets current_file for module resolution
#[test]
fn test_cli_executes_file_with_relative_module_import() {
    // Create temporary test files
    let temp_dir = std::env::temp_dir().join("graphoid_cli_test");
    fs::create_dir_all(&temp_dir).unwrap();

    // Create module file
    let module_path = temp_dir.join("test_module.gr");
    fs::write(&module_path, r#"
module test_mod alias tm

greeting = "Hello from module"

fn double(x) {
    return x * 2
}
"#).unwrap();

    // Create main file that imports the module
    let main_path = temp_dir.join("main.gr");
    fs::write(&main_path, r#"
import "./test_module"

result = tm.double(5)
msg = tm.greeting
"#).unwrap();

    // Execute main.gr using the CLI
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg(main_path.to_str().unwrap())
        .output()
        .expect("Failed to execute CLI");

    // Verify it executed successfully (no errors)
    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("CLI execution failed");
    }

    assert!(output.status.success(), "CLI should execute file with module import successfully");

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_cli_executes_file_with_module_function_call() {
    let temp_dir = std::env::temp_dir().join("graphoid_cli_test2");
    fs::create_dir_all(&temp_dir).unwrap();

    // Create math module
    let module_path = temp_dir.join("math.gr");
    fs::write(&module_path, r#"
module math alias m

PI = 3.14159

fn square(x) {
    return x * x
}
"#).unwrap();

    // Create main file that uses module functions
    let main_path = temp_dir.join("main.gr");
    fs::write(&main_path, r#"
import "./math"

result = m.square(7)
pi_val = m.PI

print("Square of 7:", result)
print("PI:", pi_val)
"#).unwrap();

    // Execute
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg(main_path.to_str().unwrap())
        .output()
        .expect("Failed to execute CLI");

    // Check success
    assert!(output.status.success(), "CLI should execute file with module function calls");

    // Verify output contains expected values
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Square of 7: 49"), "Output should contain 'Square of 7: 49'");
    assert!(stdout.contains("PI: 3.14159"), "Output should contain 'PI: 3.14159'");

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_cli_fails_gracefully_on_missing_module() {
    let temp_dir = std::env::temp_dir().join("graphoid_cli_test3");
    fs::create_dir_all(&temp_dir).unwrap();

    // Create main file that imports non-existent module
    let main_path = temp_dir.join("main.gr");
    fs::write(&main_path, r#"
import "./nonexistent_module"
"#).unwrap();

    // Execute - should fail
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg(main_path.to_str().unwrap())
        .output()
        .expect("Failed to execute CLI");

    // Should fail with error
    assert!(!output.status.success(), "CLI should fail when module not found");

    // Error message should mention module not found
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Module not found") || stderr.contains("nonexistent_module"),
            "Error should mention module not found");

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_cli_nested_module_imports() {
    let temp_dir = std::env::temp_dir().join("graphoid_cli_test4");
    fs::create_dir_all(&temp_dir).unwrap();
    fs::create_dir_all(temp_dir.join("lib")).unwrap();

    // Create nested module in lib/
    let module_path = temp_dir.join("lib/utils.gr");
    fs::write(&module_path, r#"
module utils alias u

fn helper(x) {
    return x + 10
}
"#).unwrap();

    // Create main file that imports from subdirectory
    let main_path = temp_dir.join("main.gr");
    fs::write(&main_path, r#"
import "./lib/utils"

result = u.helper(5)
print("Result:", result)
"#).unwrap();

    // Execute
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg(main_path.to_str().unwrap())
        .output()
        .expect("Failed to execute CLI");

    // Check success
    assert!(output.status.success(), "CLI should handle nested module imports");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Result: 15"), "Output should contain 'Result: 15'");

    // Cleanup
    fs::remove_dir_all(&temp_dir).ok();
}
