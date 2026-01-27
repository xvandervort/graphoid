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

    /// Gets all variable bindings in the current scope only (not parent scopes).
    /// Returns a Vec of (name, value) pairs.
    /// Used for load statement to merge variables into another environment.
    pub fn get_all_bindings(&self) -> Vec<(String, Value)> {
        self.variables.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// Gets all variable bindings from current scope AND all parent scopes.
    /// Child scope bindings shadow parent bindings with the same name.
    /// Returns a Vec of (name, value) pairs.
    /// Used for load statement to pass full context to loaded files.
    pub fn get_all_bindings_recursive(&self) -> Vec<(String, Value)> {
        let mut bindings: HashMap<String, Value> = HashMap::new();

        // First, collect parent bindings (if any)
        if let Some(parent) = &self.parent {
            for (name, value) in parent.get_all_bindings_recursive() {
                bindings.insert(name, value);
            }
        }

        // Then, add/overwrite with current scope bindings
        for (name, value) in &self.variables {
            bindings.insert(name.clone(), value.clone());
        }

        bindings.into_iter().collect()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
