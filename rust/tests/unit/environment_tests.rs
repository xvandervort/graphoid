use graphoid::execution::Environment;
use graphoid::values::Value;

#[test]
fn test_define_and_get() {
    let mut env = Environment::new();
    env.define("x".to_string(), Value::number(42.0));

    assert_eq!(env.get("x").unwrap(), Value::number(42.0));
}

#[test]
fn test_get_undefined_variable() {
    let env = Environment::new();
    assert!(env.get("undefined").is_err());
}

#[test]
fn test_set_existing_variable() {
    let mut env = Environment::new();
    env.define("x".to_string(), Value::number(10.0));
    env.set("x", Value::number(20.0)).unwrap();

    assert_eq!(env.get("x").unwrap(), Value::number(20.0));
}

#[test]
fn test_set_undefined_variable() {
    let mut env = Environment::new();
    assert!(env.set("undefined", Value::number(42.0)).is_err());
}

#[test]
fn test_nested_scope_get() {
    let mut parent = Environment::new();
    parent.define("x".to_string(), Value::number(10.0));

    let child = Environment::with_parent(parent);

    assert_eq!(child.get("x").unwrap(), Value::number(10.0));
}

#[test]
fn test_nested_scope_shadow() {
    let mut parent = Environment::new();
    parent.define("x".to_string(), Value::number(10.0));

    let mut child = Environment::with_parent(parent);
    child.define("x".to_string(), Value::number(20.0));

    // Child scope shadows parent
    assert_eq!(child.get("x").unwrap(), Value::number(20.0));
}

#[test]
fn test_nested_scope_set() {
    let mut parent = Environment::new();
    parent.define("x".to_string(), Value::number(10.0));

    let mut child = Environment::with_parent(parent);
    child.set("x", Value::number(30.0)).unwrap();

    // Setting in child scope modifies parent variable
    assert_eq!(child.get("x").unwrap(), Value::number(30.0));
}

#[test]
fn test_exists() {
    let mut env = Environment::new();
    env.define("x".to_string(), Value::number(42.0));

    assert!(env.exists("x"));
    assert!(!env.exists("y"));
}

#[test]
fn test_exists_in_parent() {
    let mut parent = Environment::new();
    parent.define("x".to_string(), Value::number(10.0));

    let child = Environment::with_parent(parent);

    assert!(child.exists("x"));
    assert!(!child.exists("y"));
}
