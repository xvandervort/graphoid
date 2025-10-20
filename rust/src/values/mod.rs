use std::collections::HashMap;
use std::fmt;

/// Runtime value types in Graphoid.
/// Phase 3 includes basic scalar and collection types.
/// Functions will be added in Phase 4.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Numeric value (64-bit floating point)
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
    /// None/null value
    None,
    /// Symbol literal (e.g., :symbol_name)
    Symbol(String),
    /// List/array of values
    List(Vec<Value>),
    /// Map/dictionary with string keys
    Map(HashMap<String, Value>),
    // Function variants will be added in Phase 4
}

impl Value {
    /// Returns true if the value is "truthy" in Graphoid.
    /// Falsy values: `false`, `none`, `0`, empty strings, and empty collections.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::None => false,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(l) => !l.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Symbol(_) => true,
        }
    }

    /// Converts value to a number if possible.
    /// Returns None if conversion is not possible.
    pub fn to_number(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Boolean(true) => Some(1.0),
            Value::Boolean(false) => Some(0.0),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None,
        }
    }

    /// Converts value to a string.
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Number(n) => {
                // Format numbers nicely (no .0 for integers)
                if n.fract() == 0.0 {
                    format!("{:.0}", n)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::None => "none".to_string(),
            Value::Symbol(s) => format!(":{}", s),
            Value::List(items) => {
                let strs: Vec<String> = items.iter().map(|v| v.to_string_value()).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Map(map) => {
                let pairs: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_string_value()))
                    .collect();
                format!("{{{}}}", pairs.join(", "))
            }
        }
    }

    /// Returns the type name of the value as a string.
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "num",
            Value::String(_) => "string",
            Value::Boolean(_) => "bool",
            Value::None => "none",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::Map(_) => "map",
        }
    }
}

/// Display implementation for user-friendly output.
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(Value::List(vec![Value::Number(1.0)]).is_truthy());
        assert!(!Value::List(vec![]).is_truthy());
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
        assert_eq!(Value::List(vec![]).type_name(), "list");
        assert_eq!(Value::Map(HashMap::new()).type_name(), "map");
    }

    #[test]
    fn test_list_creation() {
        let list = Value::List(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
        ]);

        if let Value::List(items) = list {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Value::Number(1.0));
        } else {
            panic!("Expected List variant");
        }
    }

    #[test]
    fn test_map_creation() {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::String("Alice".to_string()));
        map.insert("age".to_string(), Value::Number(30.0));

        let map_val = Value::Map(map);

        if let Value::Map(m) = map_val {
            assert_eq!(m.len(), 2);
            assert_eq!(m.get("name"), Some(&Value::String("Alice".to_string())));
            assert_eq!(m.get("age"), Some(&Value::Number(30.0)));
        } else {
            panic!("Expected Map variant");
        }
    }
}
