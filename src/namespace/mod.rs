//! Graph-based namespace implementation (Phase 15)
//!
//! This module implements NamespaceGraph, where:
//! - Variables are nodes with binding edges to values
//! - Scopes are subgraphs with parent edges
//! - Variable lookup is graph traversal
//!
//! The implementation uses a specialized structure optimized for namespace
//! operations while maintaining graph semantics.

use std::collections::HashMap;
use crate::error::{GraphoidError, Result};
use crate::values::Value;

/// Unique scope identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(u64);

/// Type of scope in the namespace graph
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    /// Global/top-level scope
    Global,
    /// Function scope with function name
    Function(String),
    /// Block scope (if, while, for, etc.)
    Block,
    /// Module scope with module name
    Module(String),
    /// Class scope with class name
    Class(String),
}

/// A scope in the namespace graph
///
/// Conceptually, this is a subgraph containing variable nodes
/// with a parent edge to the enclosing scope.
#[derive(Debug, Clone)]
struct Scope {
    /// Type of this scope
    scope_type: ScopeType,
    /// Variables defined in this scope (name -> value)
    /// Each entry represents a variable node with a "binds_to" edge to its value
    variables: HashMap<String, Value>,
    /// Parent scope ID (represents the "parent" edge to enclosing scope)
    parent: Option<ScopeId>,
}

impl Scope {
    /// Create a new scope with the given type and optional parent
    fn new(scope_type: ScopeType, parent: Option<ScopeId>) -> Self {
        Scope {
            scope_type,
            variables: HashMap::new(),
            parent,
        }
    }
}

/// Graph-based namespace for variable storage
///
/// Implements the same API as Environment but with graph semantics:
/// - Scopes are nodes connected by parent edges
/// - Variables are nodes within scopes
/// - Lookup traverses the parent edge chain
#[derive(Debug, Clone)]
pub struct NamespaceGraph {
    /// All scopes in the namespace (scope_id -> scope)
    scopes: HashMap<ScopeId, Scope>,
    /// Current active scope ID
    current_scope_id: ScopeId,
    /// Counter for generating unique scope IDs
    next_scope_id: u64,
}

impl NamespaceGraph {
    /// Creates a new namespace with a global scope
    pub fn new() -> Self {
        let global_id = ScopeId(0);
        let global_scope = Scope::new(ScopeType::Global, None);

        let mut scopes = HashMap::new();
        scopes.insert(global_id, global_scope);

        NamespaceGraph {
            scopes,
            current_scope_id: global_id,
            next_scope_id: 1,
        }
    }

    /// Creates a new namespace with the given namespace as parent
    ///
    /// This is the primary API for creating child scopes, matching
    /// Environment::with_parent().
    pub fn with_parent(parent: NamespaceGraph) -> Self {
        // Create a new namespace that contains all parent scopes
        // plus a new child scope
        let mut ns = parent;
        let child_id = ScopeId(ns.next_scope_id);
        ns.next_scope_id += 1;

        let child_scope = Scope::new(ScopeType::Block, Some(ns.current_scope_id));
        ns.scopes.insert(child_id, child_scope);
        ns.current_scope_id = child_id;

        ns
    }

    /// Returns the type of the current scope
    pub fn current_scope_type(&self) -> ScopeType {
        self.scopes
            .get(&self.current_scope_id)
            .map(|s| s.scope_type.clone())
            .unwrap_or(ScopeType::Global)
    }

    /// Defines a new variable in the current scope
    ///
    /// If the variable already exists in this scope, it will be redefined.
    /// This creates a variable node with a "binds_to" edge to the value.
    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.get_mut(&self.current_scope_id) {
            scope.variables.insert(name, value);
        }
    }

    /// Gets a variable value by name
    ///
    /// Traverses the scope chain (following parent edges) until the variable
    /// is found or we reach the global scope.
    pub fn get(&self, name: &str) -> Result<Value> {
        let mut scope_id = self.current_scope_id;

        loop {
            let scope = self.scopes.get(&scope_id)
                .ok_or_else(|| GraphoidError::runtime("Scope not found in namespace graph".to_string()))?;

            // Check if variable exists in this scope
            if let Some(value) = scope.variables.get(name) {
                return Ok(value.clone());
            }

            // Traverse to parent scope (follow parent edge)
            match scope.parent {
                Some(parent_id) => scope_id = parent_id,
                None => return Err(GraphoidError::undefined_variable(name)),
            }
        }
    }

    /// Sets a variable value by name
    ///
    /// Searches the scope chain to find where the variable is defined,
    /// then updates it there. Returns an error if the variable doesn't exist.
    pub fn set(&mut self, name: &str, value: Value) -> Result<()> {
        let mut scope_id = self.current_scope_id;

        loop {
            // Check if we can find the scope
            let has_var = self.scopes.get(&scope_id)
                .map(|s| s.variables.contains_key(name))
                .unwrap_or(false);

            if has_var {
                // Found it - update the variable
                if let Some(scope) = self.scopes.get_mut(&scope_id) {
                    scope.variables.insert(name.to_string(), value);
                }
                return Ok(());
            }

            // Get parent ID for next iteration
            let parent_id = self.scopes.get(&scope_id)
                .and_then(|s| s.parent);

            match parent_id {
                Some(pid) => scope_id = pid,
                None => return Err(GraphoidError::undefined_variable(name)),
            }
        }
    }

    /// Checks if a variable exists in the current scope or any parent scope
    pub fn exists(&self, name: &str) -> bool {
        let mut scope_id = self.current_scope_id;

        loop {
            if let Some(scope) = self.scopes.get(&scope_id) {
                if scope.variables.contains_key(name) {
                    return true;
                }

                match scope.parent {
                    Some(parent_id) => scope_id = parent_id,
                    None => return false,
                }
            } else {
                return false;
            }
        }
    }

    /// Checks if a variable exists in the current scope only (not parent scopes)
    pub fn exists_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .get(&self.current_scope_id)
            .map(|s| s.variables.contains_key(name))
            .unwrap_or(false)
    }

    /// Gets a variable value from the current scope only (not parent scopes)
    pub fn get_in_current_scope(&self, name: &str) -> Option<Value> {
        self.scopes
            .get(&self.current_scope_id)
            .and_then(|s| s.variables.get(name).cloned())
    }

    /// Gets all variable names in the current scope only (not parent scopes)
    pub fn get_variable_names(&self) -> Vec<String> {
        self.scopes
            .get(&self.current_scope_id)
            .map(|s| s.variables.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Removes a variable from the current scope only
    ///
    /// Returns true if the variable was found and removed, false otherwise.
    pub fn remove_variable(&mut self, name: &str) -> bool {
        if let Some(scope) = self.scopes.get_mut(&self.current_scope_id) {
            scope.variables.remove(name).is_some()
        } else {
            false
        }
    }

    /// Takes the parent environment, leaving the current scope without a parent
    ///
    /// This is used for extracting a modified parent from a child scope,
    /// particularly important for closure writeback semantics.
    pub fn take_parent(&mut self) -> Option<Box<NamespaceGraph>> {
        // Get parent scope ID
        let parent_id = self.scopes.get(&self.current_scope_id)
            .and_then(|s| s.parent)?;

        // Build a new NamespaceGraph containing only parent scopes
        let mut parent_scopes = HashMap::new();
        let mut scope_id = parent_id;

        // Collect all parent scopes
        loop {
            if let Some(scope) = self.scopes.get(&scope_id) {
                parent_scopes.insert(scope_id, scope.clone());
                match scope.parent {
                    Some(pid) => scope_id = pid,
                    None => break,
                }
            } else {
                break;
            }
        }

        // Find the highest scope ID to set next_scope_id correctly
        let max_id = parent_scopes.keys().map(|id| id.0).max().unwrap_or(0);

        // Disconnect current scope from parent
        if let Some(scope) = self.scopes.get_mut(&self.current_scope_id) {
            scope.parent = None;
        }

        Some(Box::new(NamespaceGraph {
            scopes: parent_scopes,
            current_scope_id: parent_id,
            next_scope_id: max_id + 1,
        }))
    }

    /// Gets all variable bindings in the current scope only
    ///
    /// Returns a Vec of (name, value) pairs.
    pub fn get_all_bindings(&self) -> Vec<(String, Value)> {
        self.scopes
            .get(&self.current_scope_id)
            .map(|s| s.variables.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    /// Gets all variable bindings from current scope AND all parent scopes
    ///
    /// Child scope bindings shadow parent bindings with the same name.
    pub fn get_all_bindings_recursive(&self) -> Vec<(String, Value)> {
        let mut bindings: HashMap<String, Value> = HashMap::new();
        let mut scope_ids = Vec::new();

        // Collect scope chain (from current to global)
        let mut scope_id = self.current_scope_id;
        loop {
            scope_ids.push(scope_id);
            match self.scopes.get(&scope_id).and_then(|s| s.parent) {
                Some(parent_id) => scope_id = parent_id,
                None => break,
            }
        }

        // Process from parent to child (so child shadows parent)
        for sid in scope_ids.into_iter().rev() {
            if let Some(scope) = self.scopes.get(&sid) {
                for (name, value) in &scope.variables {
                    bindings.insert(name.clone(), value.clone());
                }
            }
        }

        bindings.into_iter().collect()
    }
}

impl Default for NamespaceGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for NamespaceGraph {
    fn eq(&self, other: &Self) -> bool {
        // Compare current scope variables (simplified equality)
        self.get_all_bindings_recursive() == other.get_all_bindings_recursive()
    }
}
