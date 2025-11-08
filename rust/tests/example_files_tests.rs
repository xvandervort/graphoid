use std::process::Command;

/// Integration tests for example .gr files
/// These tests verify that all example files execute successfully

#[test]
fn test_example_hello_world() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/hello_world.gr")
        .output()
        .expect("Failed to execute hello_world.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("hello_world.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Hello, World!"), "Should print Hello, World!");
}

#[test]
fn test_example_behaviors() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/behaviors.gr")
        .output()
        .expect("Failed to execute behaviors.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("behaviors.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check that behavior transformations are working
    assert!(stdout.contains("Behavior") || stdout.contains("Transformation") || stdout.contains("rule"),
            "Should demonstrate behavior/transformation system");
}

#[test]
fn test_example_collections() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/collections.gr")
        .output()
        .expect("Failed to execute collections.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("collections.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verify collections output
    assert!(stdout.contains("List") || stdout.contains("Map"),
            "Should demonstrate list or map operations");
}

#[test]
fn test_example_modules_basic() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/modules_basic.gr")
        .output()
        .expect("Failed to execute modules_basic.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("modules_basic.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Module System"), "Should explain module system");
}

#[test]
fn test_example_modules_main() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/modules_main.gr")
        .output()
        .expect("Failed to execute modules_main.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("modules_main.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verify module import worked and functions executed
    assert!(stdout.contains("Using Math Module"), "Should show module usage");
    assert!(stdout.contains("PI = 3.14159"), "Should display PI constant");
    assert!(stdout.contains("square(7) = 49"), "Should calculate square");
    assert!(stdout.contains("power(2, 10) = 1024"), "Should calculate power");
    assert!(stdout.contains("Circle with radius 5"), "Should show circle calculations");
    assert!(stdout.contains("abs(-42) = 42"), "Should calculate absolute value");
}

// Note: functions.gr and graphs.gr have known parser issues with multiline constructs
// They are documented as not working in the session summary

#[test]
fn test_example_functions() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/functions.gr")
        .output()
        .expect("Failed to execute functions.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("functions.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Functions make Graphoid expressive"),
            "Should demonstrate functions");
}

#[test]
fn test_example_graphs() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/graphs.gr")
        .output()
        .expect("Failed to execute graphs.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("graphs.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Graphs are the foundation"),
            "Should demonstrate graphs");
}

/// Test that verifies the module example demonstrates all key features
#[test]
fn test_modules_example_demonstrates_all_features() {
    let output = Command::new(env!("CARGO_BIN_EXE_graphoid"))
        .arg("examples/modules_main.gr")
        .output()
        .expect("Failed to execute modules_main.gr");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify all module features are demonstrated
    assert!(stdout.contains("Mathematical constants"), "Should show constants section");
    assert!(stdout.contains("Basic operations"), "Should show basic operations");
    assert!(stdout.contains("Circle with radius"), "Should show geometric functions");
    assert!(stdout.contains("Utility functions"), "Should show utility functions");

    // Verify specific calculations are correct
    assert!(stdout.contains("3.14159"), "Should display PI");
    assert!(stdout.contains("2.71828"), "Should display E");
    assert!(stdout.contains("1.61803"), "Should display Golden Ratio");
    assert!(stdout.contains("49"), "Should calculate 7²");
    assert!(stdout.contains("27"), "Should calculate 3³");
    assert!(stdout.contains("1024"), "Should calculate 2¹⁰");
}
