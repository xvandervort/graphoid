use std::process::Command;

/// Integration tests for sample .gr files
/// These tests verify that all sample files execute successfully

#[test]
fn test_example_hello_world() {
    let output = Command::new(env!("CARGO_BIN_EXE_gr"))
        .arg("samples/01-basics/hello_world.gr")
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
    let output = Command::new(env!("CARGO_BIN_EXE_gr"))
        .arg("samples/02-intermediate/behaviors.gr")
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
    let output = Command::new(env!("CARGO_BIN_EXE_gr"))
        .arg("samples/01-basics/collections.gr")
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

// modules_basic.gr was removed during cleanup (was just explanatory text, not executable)
// #[test]
// fn test_example_modules_basic() { ... }

// modules_main.gr depends on modules_math.gr which was removed during cleanup
// TODO: Either restore modules_math.gr or remove/rewrite modules_main.gr
// #[test]
// fn test_example_modules_main() { ... }

// Note: functions.gr and graphs.gr have known parser issues with multiline constructs
// They are documented as not working in the session summary

#[test]
fn test_example_functions() {
    let output = Command::new(env!("CARGO_BIN_EXE_gr"))
        .arg("samples/01-basics/functions.gr")
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
    let output = Command::new(env!("CARGO_BIN_EXE_gr"))
        .arg("samples/01-basics/graphs.gr")
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

/// Test that verifies stdlib math module import works correctly
#[test]
fn test_modules_main_stdlib_math() {
    let output = Command::new(env!("CARGO_BIN_EXE_gr"))
        .arg("samples/04-modules/modules_main.gr")
        .output()
        .expect("Failed to execute modules_main.gr");

    if !output.status.success() {
        eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
        panic!("modules_main.gr failed to execute");
    }

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Verify stdlib math module is working
    assert!(stdout.contains("Using Stdlib Math Module"), "Should demonstrate stdlib math module");
    assert!(stdout.contains("pi ="), "Should show pi constant");
    assert!(stdout.contains("e ="), "Should show e constant");
    assert!(stdout.contains("abs(-42) = 42"), "Should demonstrate abs function");
    assert!(stdout.contains("pow(2, 8) = 256"), "Should demonstrate pow function");
}
