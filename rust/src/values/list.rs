//! List implementation as a graph
//!
//! In Graphoid, lists are linear directed graphs with sequential nodes.
//! This allows lists to use the full rule system and graph operations.

use super::{Value, Graph};
use crate::values::graph::GraphType;
use crate::graph::{RuleSpec, RuleInstance, BehaviorInstance};
use crate::error::GraphoidError;
use std::collections::HashMap;

/// List is a linear graph
///
/// Representation: node_0 → node_1 → node_2 → ...
/// Each node stores a value, edges represent sequential order
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    /// Underlying graph with linear structure
    pub graph: Graph,
    /// Number of items (cached for O(1) access)
    length: usize,
    /// Behaviors attached to this list
    /// Applied in order: first added = first applied
    pub behaviors: Vec<BehaviorInstance>,
}

impl List {
    /// Create a new empty list
    pub fn new() -> Self {
        let graph = Graph::new(GraphType::Directed);
        // Lists have linear structure rules
        // Note: We don't enforce these strictly to allow list construction
        // But they're available for validation if needed
        List {
            graph,
            length: 0,
            behaviors: Vec::new(),
        }
    }

    /// Create a list from a vector of values
    pub fn from_vec(items: Vec<Value>) -> Self {
        let mut list = List::new();
        for item in items {
            list.append(item).unwrap(); // Should never fail for empty list
        }
        list
    }

    /// Append a value to the end of the list
    pub fn append(&mut self, value: Value) -> Result<(), GraphoidError> {
        use crate::graph::behaviors::apply_behaviors;

        // Apply behaviors to incoming value (proactive application)
        let transformed = apply_behaviors(value, &self.behaviors)?;

        let new_id = format!("node_{}", self.length);

        // Add the new node with transformed value
        self.graph.add_node(new_id.clone(), transformed)?;

        // If not the first node, link from previous node
        if self.length > 0 {
            let prev_id = format!("node_{}", self.length - 1);
            self.graph.add_edge(&prev_id, &new_id, "next".to_string(), HashMap::new())?;
        }

        self.length += 1;
        Ok(())
    }

    /// Get value at index
    pub fn get(&self, index: usize) -> Option<&Value> {
        if index >= self.length {
            return None;
        }
        let node_id = format!("node_{}", index);
        self.graph.get_node(&node_id)
    }

    /// Get mutable value at index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        if index >= self.length {
            return None;
        }
        let _node_id = format!("node_{}", index);
        // For now, we don't support direct mutable access to graph nodes
        // This would require adding a method to Graph
        None
    }

    /// Set value at index
    pub fn set(&mut self, index: usize, value: Value) -> Result<(), GraphoidError> {
        use crate::graph::behaviors::apply_behaviors;

        if index >= self.length {
            return Err(GraphoidError::runtime(format!(
                "Index {} out of bounds for list of length {}",
                index, self.length
            )));
        }

        // Apply behaviors to incoming value (proactive application)
        let transformed = apply_behaviors(value, &self.behaviors)?;

        let node_id = format!("node_{}", index);

        // Remove old node and add new one with same ID
        self.graph.remove_node(&node_id)?;
        self.graph.add_node(node_id.clone(), transformed)?;

        // Restore edges
        if index > 0 {
            let prev_id = format!("node_{}", index - 1);
            self.graph.add_edge(&prev_id, &node_id, "next".to_string(), HashMap::new())?;
        }
        if index < self.length - 1 {
            let next_id = format!("node_{}", index + 1);
            self.graph.add_edge(&node_id, &next_id, "next".to_string(), HashMap::new())?;
        }

        Ok(())
    }

    /// Get the length of the list
    pub fn len(&self) -> usize {
        self.length
    }

    /// Check if the list is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Convert list to a vector
    pub fn to_vec(&self) -> Vec<Value> {
        let mut result = Vec::with_capacity(self.length);
        for i in 0..self.length {
            if let Some(value) = self.get(i) {
                result.push(value.clone());
            }
        }
        result
    }

    /// Add a rule to this list
    pub fn add_rule(&mut self, rule: RuleInstance) -> Result<(), GraphoidError> {
        self.graph.add_rule(rule)?;
        // Recompute length in case cleaning removed nodes
        self.length = self.graph.node_count();
        Ok(())
    }

    /// Remove a rule from this list
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

    /// Apply a ruleset to this list
    pub fn with_ruleset(mut self, ruleset: String) -> Self {
        self.graph = self.graph.with_ruleset(ruleset);
        self
    }

    /// Check if a ruleset is applied
    pub fn has_ruleset(&self, ruleset: &str) -> bool {
        self.graph.has_ruleset(ruleset)
    }

    /// Add a behavior to this list
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
        use crate::graph::behaviors::apply_retroactive_to_list;

        // Apply retroactively based on policy
        apply_retroactive_to_list(self, &behavior)?;

        // Add to behaviors list for future proactive application
        self.behaviors.push(behavior);

        Ok(())
    }

    /// Get all behaviors attached to this list
    ///
    /// Returns a slice of behavior instances in the order they were added.
    /// Behaviors are applied in this order: first added = first applied.
    pub fn get_behaviors(&self) -> &[BehaviorInstance] {
        &self.behaviors
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}
