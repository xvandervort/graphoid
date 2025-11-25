use graphoid::values::{Value, List, Hash, ValueKind};
use std::collections::HashMap;

#[test]
fn test_value_creation() {
    let num = Value::number(42.0);
    let str_val = Value::string("hello".to_string());
    let bool_val = Value::boolean(true);
    let none_val = Value::none();
    let sym = Value::symbol("test".to_string());

    assert_eq!(num, Value::number(42.0));
    assert_eq!(str_val, Value::string("hello".to_string()));
    assert_eq!(bool_val, Value::boolean(true));
    assert_eq!(none_val, Value::none());
    assert_eq!(sym, Value::symbol("test".to_string()));
}

#[test]
fn test_is_truthy() {
    assert!(Value::boolean(true).is_truthy());
    assert!(!Value::boolean(false).is_truthy());
    assert!(!Value::none().is_truthy());
    assert!(!Value::number(0.0).is_truthy());
    assert!(Value::number(1.0).is_truthy());
    assert!(Value::number(-5.0).is_truthy());
    assert!(Value::string("hello".to_string()).is_truthy());
    assert!(!Value::string("".to_string()).is_truthy());
    assert!(Value::symbol("test".to_string()).is_truthy());
    assert!(Value::list(List::from_vec(vec![Value::number(1.0)])).is_truthy());
    assert!(!Value::list(List::new()).is_truthy());
}

#[test]
fn test_to_number() {
    assert_eq!(Value::number(42.5).to_number(), Some(42.5));
    assert_eq!(Value::boolean(true).to_number(), Some(1.0));
    assert_eq!(Value::boolean(false).to_number(), Some(0.0));
    assert_eq!(Value::string("123.45".to_string()).to_number(), Some(123.45));
    assert_eq!(Value::string("not a number".to_string()).to_number(), None);
    assert_eq!(Value::none().to_number(), None);
}

#[test]
fn test_to_string_value() {
    assert_eq!(Value::number(42.0).to_string_value(), "42");
    assert_eq!(Value::number(42.5).to_string_value(), "42.5");
    assert_eq!(Value::string("hello".to_string()).to_string_value(), "hello");
    assert_eq!(Value::boolean(true).to_string_value(), "true");
    assert_eq!(Value::boolean(false).to_string_value(), "false");
    assert_eq!(Value::none().to_string_value(), "none");
    assert_eq!(Value::symbol("test".to_string()).to_string_value(), ":test");
}

#[test]
fn test_type_name() {
    assert_eq!(Value::number(42.0).type_name(), "num");
    assert_eq!(Value::string("hello".to_string()).type_name(), "string");
    assert_eq!(Value::boolean(true).type_name(), "bool");
    assert_eq!(Value::none().type_name(), "none");
    assert_eq!(Value::symbol("test".to_string()).type_name(), "symbol");
    assert_eq!(Value::list(List::new()).type_name(), "list");
    assert_eq!(Value::map(Hash::new()).type_name(), "map");
}

#[test]
fn test_list_creation() {
    let list = Value::list(List::from_vec(vec![
        Value::number(1.0),
        Value::number(2.0),
        Value::number(3.0),
    ]));

    if let ValueKind::List(l) = &list.kind {
        assert_eq!(l.len(), 3);
        assert_eq!(l.get(0), Some(&Value::number(1.0)));
    } else {
        panic!("Expected List variant");
    }
}

#[test]
fn test_map_creation() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), Value::string("Alice".to_string()));
    map.insert("age".to_string(), Value::number(30.0));

    let map_val = Value::map(Hash::from_hashmap(map));

    if let ValueKind::Map(h) = &map_val.kind {
        assert_eq!(h.len(), 2);
        assert_eq!(h.get("name"), Some(&Value::string("Alice".to_string())));
        assert_eq!(h.get("age"), Some(&Value::number(30.0)));
    } else {
        panic!("Expected Map variant");
    }
}
