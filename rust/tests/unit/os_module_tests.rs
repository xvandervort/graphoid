//! Tests for OS module (native implementation for system primitives)

use graphoid::values::{Value, ValueKind};
use graphoid::stdlib::NativeModule;

#[test]
fn test_os_module_has_correct_name() {
    let os = graphoid::stdlib::os::OSModule;
    assert_eq!(os.name(), "os");
}

#[test]
fn test_os_module_has_no_alias() {
    let os = graphoid::stdlib::os::OSModule;
    assert_eq!(os.alias(), None);
}

#[test]
fn test_system_timestamp_returns_number() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let system_timestamp = functions.get("system_timestamp").expect("system_timestamp should exist");

    let result = system_timestamp(&[]).expect("system_timestamp should succeed");

    match result.kind {
        ValueKind::Number(n) => {
            assert!(n > 0.0, "Timestamp should be positive");
            assert!(n > 1_600_000_000.0, "Timestamp should be reasonable (after 2020)");
        }
        _ => panic!("system_timestamp should return a number"),
    }
}

#[test]
fn test_system_timestamp_rejects_arguments() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let system_timestamp = functions.get("system_timestamp").unwrap();

    let result = system_timestamp(&[Value::number(42.0)]);
    assert!(result.is_err(), "system_timestamp should reject arguments");
}

#[test]
fn test_env_returns_string_for_existing_var() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let env = functions.get("env").expect("env should exist");

    // Set a test environment variable
    std::env::set_var("GRAPHOID_TEST_VAR", "test_value");

    let result = env(&[Value::string("GRAPHOID_TEST_VAR".to_string())]).expect("env should succeed");

    match result.kind {
        ValueKind::String(s) => {
            assert_eq!(s, "test_value");
        }
        _ => panic!("env should return a string"),
    }

    // Clean up
    std::env::remove_var("GRAPHOID_TEST_VAR");
}

#[test]
fn test_env_returns_none_for_missing_var() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let env = functions.get("env").unwrap();

    let result = env(&[Value::string("NONEXISTENT_VAR_12345".to_string())]).expect("env should succeed");

    match result.kind {
        ValueKind::None => {}
        _ => panic!("env should return none for missing variables"),
    }
}

#[test]
fn test_env_requires_string_argument() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let env = functions.get("env").unwrap();

    let result = env(&[Value::number(42.0)]);
    assert!(result.is_err(), "env should require string argument");
}

#[test]
fn test_env_requires_exactly_one_argument() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let env = functions.get("env").unwrap();

    let result = env(&[]);
    assert!(result.is_err(), "env should require exactly one argument");

    let result = env(&[Value::string("A".to_string()), Value::string("B".to_string())]);
    assert!(result.is_err(), "env should require exactly one argument");
}

#[test]
fn test_getcwd_returns_string() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let getcwd = functions.get("getcwd").expect("getcwd should exist");

    let result = getcwd(&[]).expect("getcwd should succeed");

    match result.kind {
        ValueKind::String(s) => {
            assert!(!s.is_empty(), "getcwd should return non-empty path");
        }
        _ => panic!("getcwd should return a string"),
    }
}

#[test]
fn test_getcwd_rejects_arguments() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let getcwd = functions.get("getcwd").unwrap();

    let result = getcwd(&[Value::number(42.0)]);
    assert!(result.is_err(), "getcwd should reject arguments");
}

#[test]
fn test_platform_returns_string() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let platform = functions.get("platform").expect("platform should exist");

    let result = platform(&[]).expect("platform should succeed");

    match result.kind {
        ValueKind::String(s) => {
            // Should be one of: "linux", "macos", "windows", "unix"
            assert!(["linux", "macos", "windows", "unix"].contains(&s.as_str()),
                   "platform should return recognized OS name, got: {}", s);
        }
        _ => panic!("platform should return a string"),
    }
}

#[test]
fn test_arch_returns_string() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let arch = functions.get("arch").expect("arch should exist");

    let result = arch(&[]).expect("arch should succeed");

    match result.kind {
        ValueKind::String(s) => {
            // Should be one of common architectures
            assert!(!s.is_empty(), "arch should return non-empty string");
        }
        _ => panic!("arch should return a string"),
    }
}

#[test]
fn test_env_all_returns_hash() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let env_all = functions.get("env_all").expect("env_all should exist");

    let result = env_all(&[]).expect("env_all should succeed");

    match result.kind {
        ValueKind::Map(h) => {
            assert!(!h.is_empty(), "env_all should return non-empty hash");

            // Verify all values are strings
            for key in h.keys() {
                assert!(!key.is_empty(), "env keys should not be empty");
                if let Some(value) = h.get(&key) {
                    match &value.kind {
                        ValueKind::String(_) => {}
                        _ => panic!("env_all values should all be strings"),
                    }
                }
            }
        }
        _ => panic!("env_all should return a hash"),
    }
}

#[test]
fn test_args_returns_list() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();
    let args = functions.get("args").expect("args should exist");

    let result = args(&[]).expect("args should succeed");

    match result.kind {
        ValueKind::List(items) => {
            // All items should be strings
            for item in items.to_vec() {
                match &item.kind {
                    ValueKind::String(_) => {}
                    _ => panic!("args should return list of strings"),
                }
            }
        }
        _ => panic!("args should return a list"),
    }
}

#[test]
fn test_os_module_functions_count() {
    let os = graphoid::stdlib::os::OSModule;
    let functions = os.functions();

    // Should have at least these functions:
    // system_timestamp, env, env_all, getcwd, platform, arch, args
    assert!(functions.len() >= 7, "OS module should have at least 7 functions");
}

#[test]
fn test_os_module_no_constants() {
    let os = graphoid::stdlib::os::OSModule;
    let constants = os.constants();

    // OS module should not have constants (just functions)
    assert_eq!(constants.len(), 0, "OS module should not have constants");
}
