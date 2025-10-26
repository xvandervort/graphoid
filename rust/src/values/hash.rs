//! Hash (map/dictionary) implementation as a graph
//!
//! In Graphoid, hashes are graphs where node IDs are keys and node values are the map values.
//! This allows hashes to use the full rule system and graph operations.

use super::{Value, Graph};
use crate::values::graph::GraphType;
use crate::graph::{RuleSpec, RuleInstance};
use crate::error::GraphoidError;

/// Hash is a key-value graph
///
/// Representation: Each key is a node ID, value is the node's value
/// No edges needed - just a collection of named nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Hash {
    /// Underlying graph storing key-value pairs
    pub graph: Graph,
}

impl Hash {
    /// Create a new empty hash
    pub fn new() -> Self {
        Hash {
            graph: Graph::new(GraphType::Directed),
        }
    }

    /// Create a hash from a HashMap
    pub fn from_hashmap(map: std::collections::HashMap<String, Value>) -> Self {
        let mut hash = Hash::new();
        for (key, value) in map {
            hash.insert(key, value).unwrap(); // Should never fail for new hash
        }
        hash
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: String, value: Value) -> Result<Option<Value>, GraphoidError> {
        // Apply transformation rules to incoming value (proactive application)
        let transformed = self.apply_transformation_rules(value)?;

        // Check if key already exists
        let old_value = self.graph.get_node(&key).cloned();

        // Remove old node if it exists
        if old_value.is_some() {
            self.graph.remove_node(&key)?;
        }

        // Add new node with key as ID and transformed value
        self.graph.add_node(key, transformed)?;

        Ok(old_value)
    }

    /// Insert a key-value pair without applying behaviors
    ///
    /// This is an internal method used by the executor when behaviors have
    /// already been applied with full executor context (for function-based behaviors).
    ///
    /// # Arguments
    /// * `key` - The key to insert
    /// * `value` - The value to insert (already transformed)
    ///
    /// # Returns
    /// `Ok(Some(old_value))` if key existed, `Ok(None)` if new key, or an error if the operation fails
    pub fn insert_raw(&mut self, key: String, value: Value) -> Result<Option<Value>, GraphoidError> {
        // Check if key already exists
        let old_value = self.graph.get_node(&key).cloned();

        // Remove old node if it exists
        if old_value.is_some() {
            self.graph.remove_node(&key)?;
        }

        // Add new node with key as ID (no behavior application)
        self.graph.add_node(key, value)?;

        Ok(old_value)
    }

    /// Get value for a key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.graph.get_node(key)
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.graph.has_node(key)
    }

    /// Remove a key-value pair
    pub fn remove(&mut self, key: &str) -> Result<Option<Value>, GraphoidError> {
        let value = self.graph.get_node(key).cloned();
        self.graph.remove_node(key)?;
        Ok(value)
    }

    /// Get all keys
    pub fn keys(&self) -> Vec<String> {
        self.graph.keys()
    }

    /// Get all values
    pub fn values(&self) -> Vec<Value> {
        self.graph.values()
    }

    /// Get the number of key-value pairs
    pub fn len(&self) -> usize {
        self.graph.node_count()
    }

    /// Check if the hash is empty
    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }

    /// Convert hash to a HashMap
    pub fn to_hashmap(&self) -> std::collections::HashMap<String, Value> {
        let mut map = std::collections::HashMap::new();
        for key in self.keys() {
            if let Some(value) = self.get(&key) {
                map.insert(key, value.clone());
            }
        }
        map
    }

    /// Add a rule to this hash
    ///
    /// If the rule is a transformation rule, it will be applied retroactively
    /// to all existing values in the hash (with RetroactivePolicy::Clean).
    pub fn add_rule(&mut self, rule: RuleInstance) -> Result<(), GraphoidError> {
        // If it's a transformation rule, apply it retroactively to existing values
        if rule.spec.is_transformation_rule() {
            let rule_impl = rule.spec.instantiate();

            // Transform all existing values
            let keys: Vec<String> = self.graph.nodes.keys().cloned().collect();
            for key in keys {
                if let Some(node) = self.graph.nodes.get_mut(&key) {
                    node.value = rule_impl.transform(&node.value)?;
                }
            }
        }

        // Add the rule to the graph
        self.graph.add_rule(rule)
    }

    /// Remove a rule from this hash
    pub fn remove_rule(&mut self, rule: &RuleSpec) {
        self.graph.remove_rule(rule);
    }

    /// Check if a rule is active
    pub fn has_rule(&self, rule_name: &str) -> bool {
        self.graph.has_rule(rule_name)
    }

    /// Get all ad hoc rules (not including ruleset rules)
    pub fn get_rules(&self) -> &[RuleInstance] {
        self.graph.get_rules()
    }

    /// Apply a ruleset to this hash
    pub fn with_ruleset(mut self, ruleset: String) -> Self {
        self.graph = self.graph.with_ruleset(ruleset);
        self
    }

    /// Check if a ruleset is applied
    pub fn has_ruleset(&self, ruleset: &str) -> bool {
        self.graph.has_ruleset(ruleset)
    }

    /// Apply transformation rules to a value
    ///
    /// Filters the graph's rules for transformation rules and applies them in sequence.
    /// Rules are applied in order: first added = first applied.
    ///
    /// # Arguments
    /// * `value` - The value to transform
    ///
    /// # Returns
    /// The transformed value, or an error if transformation fails
    fn apply_transformation_rules(&self, value: Value) -> Result<Value, GraphoidError> {
        let mut current = value;

        // Apply transformation rules from graph.rules in order
        for rule_instance in &self.graph.rules {
            if rule_instance.spec.is_transformation_rule() {
                let rule = rule_instance.spec.instantiate();
                current = rule.transform(&current)?;
            }
        }

        Ok(current)
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::new()
    }
}
