use graphoid::values::{Value, List, Hash};
use std::collections::HashMap;

#[test]
fn test_value_creation() {
    let num = Value::Number(42.0);
    let str_val = Value::String("hello".to_string());
    let bool_val = Value::Boolean(true);
    let none_val = Value::None;
    let sym = Value::Symbol("test".to_string());

    assert_eq!(num, Value::Number(42.0));
    assert_eq!(str_val, Value::String("hello".to_string()));
    assert_eq!(bool_val, Value::Boolean(true));
    assert_eq!(none_val, Value::None);
    assert_eq!(sym, Value::Symbol("test".to_string()));
}

#[test]
fn test_is_truthy() {
    assert!(Value::Boolean(true).is_truthy());
    assert!(!Value::Boolean(false).is_truthy());
    assert!(!Value::None.is_truthy());
    assert!(!Value::Number(0.0).is_truthy());
    assert!(Value::Number(1.0).is_truthy());
    assert!(Value::Number(-5.0).is_truthy());
    assert!(Value::String("hello".to_string()).is_truthy());
    assert!(!Value::String("".to_string()).is_truthy());
    assert!(Value::Symbol("test".to_string()).is_truthy());
    assert!(Value::List(List::from_vec(vec![Value::Number(1.0)])).is_truthy());
    assert!(!Value::List(List::new()).is_truthy());
}

#[test]
fn test_to_number() {
    assert_eq!(Value::Number(42.5).to_number(), Some(42.5));
    assert_eq!(Value::Boolean(true).to_number(), Some(1.0));
    assert_eq!(Value::Boolean(false).to_number(), Some(0.0));
    assert_eq!(Value::String("123.45".to_string()).to_number(), Some(123.45));
    assert_eq!(Value::String("not a number".to_string()).to_number(), None);
    assert_eq!(Value::None.to_number(), None);
}

#[test]
fn test_to_string_value() {
    assert_eq!(Value::Number(42.0).to_string_value(), "42");
    assert_eq!(Value::Number(42.5).to_string_value(), "42.5");
    assert_eq!(Value::String("hello".to_string()).to_string_value(), "hello");
    assert_eq!(Value::Boolean(true).to_string_value(), "true");
    assert_eq!(Value::Boolean(false).to_string_value(), "false");
    assert_eq!(Value::None.to_string_value(), "none");
    assert_eq!(Value::Symbol("test".to_string()).to_string_value(), ":test");
}

#[test]
fn test_type_name() {
    assert_eq!(Value::Number(42.0).type_name(), "num");
    assert_eq!(Value::String("hello".to_string()).type_name(), "string");
    assert_eq!(Value::Boolean(true).type_name(), "bool");
    assert_eq!(Value::None.type_name(), "none");
    assert_eq!(Value::Symbol("test".to_string()).type_name(), "symbol");
    assert_eq!(Value::List(List::new()).type_name(), "list");
    assert_eq!(Value::Map(Hash::new()).type_name(), "map");
}

#[test]
fn test_list_creation() {
    let list = Value::List(List::from_vec(vec![
        Value::Number(1.0),
        Value::Number(2.0),
        Value::Number(3.0),
    ]));

    if let Value::List(l) = list {
        assert_eq!(l.len(), 3);
        assert_eq!(l.get(0), Some(&Value::Number(1.0)));
    } else {
        panic!("Expected List variant");
    }
}

#[test]
fn test_map_creation() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), Value::String("Alice".to_string()));
    map.insert("age".to_string(), Value::Number(30.0));

    let map_val = Value::Map(Hash::from_hashmap(map));

    if let Value::Map(h) = map_val {
        assert_eq!(h.len(), 2);
        assert_eq!(h.get("name"), Some(&Value::String("Alice".to_string())));
        assert_eq!(h.get("age"), Some(&Value::Number(30.0)));
    } else {
        panic!("Expected Map variant");
    }
}
