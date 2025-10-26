//! Hash (map/dictionary) implementation as a graph
//!
//! In Graphoid, hashes are graphs where node IDs are keys and node values are the map values.
//! This allows hashes to use the full rule system and graph operations.

use super::{Value, Graph};
use crate::values::graph::GraphType;
use crate::graph::{RuleSpec, RuleInstance, BehaviorInstance};
use crate::error::GraphoidError;

/// Hash is a key-value graph
///
/// Representation: Each key is a node ID, value is the node's value
/// No edges needed - just a collection of named nodes
#[derive(Debug, Clone, PartialEq)]
pub struct Hash {
    /// Underlying graph storing key-value pairs
    pub graph: Graph,
    /// Behaviors attached to this hash
    /// Applied in order: first added = first applied
    pub behaviors: Vec<BehaviorInstance>,
}

impl Hash {
    /// Create a new empty hash
    pub fn new() -> Self {
        Hash {
            graph: Graph::new(GraphType::Directed),
            behaviors: Vec::new(),
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
        use crate::graph::behaviors::apply_behaviors;

        // Apply behaviors to incoming value (proactive application)
        let transformed = apply_behaviors(value, &self.behaviors)?;

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
    pub fn add_rule(&mut self, rule: RuleInstance) -> Result<(), GraphoidError> {
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

    /// Add a behavior to this hash
    ///
    /// The behavior will be applied retroactively to existing values based on
    /// the RetroactivePolicy, then added to the behaviors list for proactive
    /// application to future values.
    ///
    /// # Arguments
    /// * `behavior` - The behavior instance to add
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if retroactive application fails
    pub fn add_behavior(&mut self, behavior: BehaviorInstance) -> Result<(), GraphoidError> {
        use crate::graph::behaviors::apply_retroactive_to_hash;

        // Apply retroactively based on policy
        apply_retroactive_to_hash(self, &behavior)?;

        // Add to behaviors list for future proactive application
        self.behaviors.push(behavior);

        Ok(())
    }

    /// Get all behaviors attached to this hash
    ///
    /// Returns a slice of behavior instances in the order they were added.
    /// Behaviors are applied in this order: first added = first applied.
    pub fn get_behaviors(&self) -> &[BehaviorInstance] {
        &self.behaviors
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::new()
    }
}
