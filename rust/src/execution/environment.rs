use crate::error::{GraphoidError, Result};
use crate::values::Value;
use std::collections::HashMap;

/// Execution environment for variable storage.
/// Supports nested scopes via parent pointer.
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    /// Variables defined in this scope
    variables: HashMap<String, Value>,
    /// Parent scope (for nested scopes)
    parent: Option<Box<Environment>>,
}

impl Environment {
    /// Creates a new top-level environment with no parent.
    pub fn new() -> Self {
        Environment {
            variables: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a new environment with a parent scope.
    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Defines a new variable in the current scope.
    /// If the variable already exists in this scope, it will be redefined.
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// Gets a variable value by name.
    /// Searches current scope first, then parent scopes.
    /// Returns an error if the variable is not found.
    pub fn get(&self, name: &str) -> Result<Value> {
        if let Some(value) = self.variables.get(name) {
            Ok(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            Err(GraphoidError::undefined_variable(name))
        }
    }

    /// Sets a variable value by name.
    /// Searches current scope first, then parent scopes.
    /// Returns an error if the variable is not found.
    /// Use `define` to create new variables.
    pub fn set(&mut self, name: &str, value: Value) -> Result<()> {
        if self.variables.contains_key(name) {
            self.variables.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &mut self.parent {
            parent.set(name, value)
        } else {
            Err(GraphoidError::undefined_variable(name))
        }
    }

    /// Checks if a variable exists in the current scope or any parent scope.
    pub fn exists(&self, name: &str) -> bool {
        self.variables.contains_key(name)
            || self.parent.as_ref().map_or(false, |p| p.exists(name))
    }

    /// Gets all variable names in the current scope only (not parent scopes).
    /// Used for tracking which variables are defined locally.
    pub fn get_variable_names(&self) -> Vec<String> {
        self.variables.keys().cloned().collect()
    }

    /// Removes a variable from the current scope only (not parent scopes).
    /// Returns true if the variable was found and removed, false otherwise.
    pub fn remove_variable(&mut self, name: &str) -> bool {
        self.variables.remove(name).is_some()
    }

    /// Takes the parent environment, leaving None in its place.
    /// Used when extracting a modified parent from a child scope.
    pub fn take_parent(&mut self) -> Option<Box<Environment>> {
        self.parent.take()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(42.0));

        assert_eq!(env.get("x").unwrap(), Value::Number(42.0));
    }

    #[test]
    fn test_get_undefined_variable() {
        let env = Environment::new();
        assert!(env.get("undefined").is_err());
    }

    #[test]
    fn test_set_existing_variable() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(10.0));
        env.set("x", Value::Number(20.0)).unwrap();

        assert_eq!(env.get("x").unwrap(), Value::Number(20.0));
    }

    #[test]
    fn test_set_undefined_variable() {
        let mut env = Environment::new();
        assert!(env.set("undefined", Value::Number(42.0)).is_err());
    }

    #[test]
    fn test_nested_scope_get() {
        let mut parent = Environment::new();
        parent.define("x".to_string(), Value::Number(10.0));

        let child = Environment::with_parent(parent);

        assert_eq!(child.get("x").unwrap(), Value::Number(10.0));
    }

    #[test]
    fn test_nested_scope_shadow() {
        let mut parent = Environment::new();
        parent.define("x".to_string(), Value::Number(10.0));

        let mut child = Environment::with_parent(parent);
        child.define("x".to_string(), Value::Number(20.0));

        // Child scope shadows parent
        assert_eq!(child.get("x").unwrap(), Value::Number(20.0));
    }

    #[test]
    fn test_nested_scope_set() {
        let mut parent = Environment::new();
        parent.define("x".to_string(), Value::Number(10.0));

        let mut child = Environment::with_parent(parent);
        child.set("x", Value::Number(30.0)).unwrap();

        // Setting in child scope modifies parent variable
        assert_eq!(child.get("x").unwrap(), Value::Number(30.0));
    }

    #[test]
    fn test_exists() {
        let mut env = Environment::new();
        env.define("x".to_string(), Value::Number(42.0));

        assert!(env.exists("x"));
        assert!(!env.exists("y"));
    }

    #[test]
    fn test_exists_in_parent() {
        let mut parent = Environment::new();
        parent.define("x".to_string(), Value::Number(10.0));

        let child = Environment::with_parent(parent);

        assert!(child.exists("x"));
        assert!(!child.exists("y"));
    }
}
