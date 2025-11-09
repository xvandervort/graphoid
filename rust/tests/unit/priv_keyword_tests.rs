// Unit Tests for priv Keyword - Phase 10
//
// Tests privacy enforcement in module system:
// - Private functions/variables not accessible from imports
// - Private symbols accessible within same module
// - Public symbols (default) accessible everywhere

use graphoid::execution::Executor;

#[test]
fn test_priv_function_not_accessible_from_import() {
    // Create a module with a private function
    let module_code = r#"
module helpers

priv fn internal_helper() {
    return 42
}

fn public_api() {
    return internal_helper()
}
"#;

    // Write module to temp file
    let temp_dir = std::env::temp_dir().join("graphoid_priv_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("helpers.gr");
    std::fs::write(&module_path, module_code).unwrap();

    // Import and try to access private function
    let main_code = format!(r#"
import "{}"

result = helpers.internal_helper()
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    let result = executor.execute_source(&main_code);

    // Should fail - cannot access private function
    assert!(result.is_err(), "Should not be able to access private function");
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("private") || err_msg.contains("not found"),
            "Error should mention privacy: {}", err_msg);

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_priv_function_accessible_within_module() {
    // Module with private helper called by public function
    let module_code = r#"
module math_utils

priv fn internal_square(x) {
    return x * x
}

fn square_plus_one(x) {
    return internal_square(x) + 1
}
"#;

    let temp_dir = std::env::temp_dir().join("graphoid_priv_test2");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("math_utils.gr");
    std::fs::write(&module_path, module_code).unwrap();

    let main_code = format!(r#"
import "{}"

result = math_utils.square_plus_one(5)
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    executor.execute_source(&main_code).unwrap();

    let result = executor.get_variable("result").unwrap();
    if let graphoid::values::ValueKind::Number(n) = result.kind {
        assert_eq!(n, 26.0, "Should be 5*5 + 1 = 26");
    } else {
        panic!("Expected number result");
    }

    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_public_function_accessible_from_import() {
    let module_code = r#"
module utils

fn public_helper() {
    return "Hello"
}
"#;

    let temp_dir = std::env::temp_dir().join("graphoid_priv_test3");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("utils.gr");
    std::fs::write(&module_path, module_code).unwrap();

    let main_code = format!(r#"
import "{}"

result = utils.public_helper()
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    executor.execute_source(&main_code).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result.to_string_value(), "Hello");

    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_priv_variable_not_accessible_from_import() {
    let module_code = r#"
module config

priv SECRET_KEY = "super_secret"

PUBLIC_VALUE = "public"
"#;

    let temp_dir = std::env::temp_dir().join("graphoid_priv_test4");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("config.gr");
    std::fs::write(&module_path, module_code).unwrap();

    let main_code = format!(r#"
import "{}"

secret = config.SECRET_KEY
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    let result = executor.execute_source(&main_code);

    assert!(result.is_err(), "Should not be able to access private variable");

    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_priv_variable_accessible_within_module() {
    let module_code = r#"
module config

priv API_KEY = "secret123"

fn get_config() {
    return API_KEY
}
"#;

    let temp_dir = std::env::temp_dir().join("graphoid_priv_test5");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("config.gr");
    std::fs::write(&module_path, module_code).unwrap();

    let main_code = format!(r#"
import "{}"

result = config.get_config()
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    executor.execute_source(&main_code).unwrap();

    let result = executor.get_variable("result").unwrap();
    assert_eq!(result.to_string_value(), "secret123");

    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_public_variable_accessible_from_import() {
    let module_code = r#"
module constants

PI = 3.14159
E = 2.71828
"#;

    let temp_dir = std::env::temp_dir().join("graphoid_priv_test6");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("constants.gr");
    std::fs::write(&module_path, module_code).unwrap();

    let main_code = format!(r#"
import "{}"

pi = constants.PI
e = constants.E
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    executor.execute_source(&main_code).unwrap();

    let pi = executor.get_variable("pi").unwrap();
    let e = executor.get_variable("e").unwrap();

    if let graphoid::values::ValueKind::Number(n) = pi.kind {
        assert!((n - 3.14159).abs() < 0.0001);
    }
    if let graphoid::values::ValueKind::Number(n) = e.kind {
        assert!((n - 2.71828).abs() < 0.0001);
    }

    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_multiple_priv_symbols_in_module() {
    let module_code = r#"
module helpers

priv SECRET = "hidden"

priv fn helper1() {
    return 1
}

priv fn helper2() {
    return 2
}

fn public_sum() {
    return helper1() + helper2()
}
"#;

    let temp_dir = std::env::temp_dir().join("graphoid_priv_test7");
    std::fs::create_dir_all(&temp_dir).unwrap();
    let module_path = temp_dir.join("helpers.gr");
    std::fs::write(&module_path, module_code).unwrap();

    let main_code = format!(r#"
import "{}"

result = helpers.public_sum()
"#, module_path.to_str().unwrap());

    let mut executor = Executor::new();
    executor.execute_source(&main_code).unwrap();

    let result = executor.get_variable("result").unwrap();
    if let graphoid::values::ValueKind::Number(n) = result.kind {
        assert_eq!(n, 3.0);
    }

    std::fs::remove_dir_all(&temp_dir).ok();
}
