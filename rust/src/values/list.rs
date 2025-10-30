//! List implementation as a graph
//!
//! In Graphoid, lists are linear directed graphs with sequential nodes.
//! This allows lists to use the full rule system and graph operations.

use super::{Value, Graph};
use crate::values::graph::GraphType;
use crate::graph::{RuleSpec, RuleInstance};
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
        // Apply transformation rules to incoming value (proactive application)
        let transformed = self.apply_transformation_rules(value)?;

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

    /// Append a value to the end of the list without applying behaviors
    ///
    /// This is an internal method used by the executor when behaviors have
    /// already been applied with full executor context (for function-based behaviors).
    ///
    /// # Arguments
    /// * `value` - The value to append (already transformed)
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if the operation fails
    pub fn append_raw(&mut self, value: Value) -> Result<(), GraphoidError> {
        let new_id = format!("node_{}", self.length);

        // Add the new node (no behavior application)
        self.graph.add_node(new_id.clone(), value)?;

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
        if index >= self.length {
            return Err(GraphoidError::runtime(format!(
                "Index {} out of bounds for list of length {}",
                index, self.length
            )));
        }

        // Apply transformation rules to incoming value (proactive application)
        let transformed = self.apply_transformation_rules(value)?;

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

    /// Set value at index without applying behaviors
    ///
    /// This is an internal method used by the executor when behaviors have
    /// already been applied with full executor context (for function-based behaviors).
    ///
    /// # Arguments
    /// * `index` - The index to set
    /// * `value` - The value to set (already transformed)
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if the operation fails
    pub fn set_raw(&mut self, index: usize, value: Value) -> Result<(), GraphoidError> {
        if index >= self.length {
            return Err(GraphoidError::runtime(format!(
                "Index {} out of bounds for list of length {}",
                index, self.length
            )));
        }

        let node_id = format!("node_{}", index);

        // Remove old node and add new one with same ID (no behavior application)
        self.graph.remove_node(&node_id)?;
        self.graph.add_node(node_id.clone(), value)?;

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

    /// Insert a value at a specific index
    ///
    /// This method inserts a value at the given index, shifting all subsequent
    /// values to the right. Behaviors are automatically applied to the inserted value.
    ///
    /// # Arguments
    /// * `index` - The position to insert at (0 = beginning, length = end)
    /// * `value` - The value to insert
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if the index is out of bounds
    pub fn insert_at(&mut self, index: usize, value: Value) -> Result<(), GraphoidError> {
        if index > self.length {
            return Err(GraphoidError::runtime(format!(
                "Index {} out of bounds for list of length {} (insert)",
                index, self.length
            )));
        }

        // Apply transformation rules to incoming value (proactive application)
        let transformed = self.apply_transformation_rules(value)?;

        // Collect all values
        let mut values = self.to_vec();

        // Insert transformed value at position
        values.insert(index, transformed);

        // Save the rules before rebuilding
        let saved_rules = self.graph.rules.clone();
        let saved_rulesets = self.graph.rulesets.clone();

        // Rebuild list from scratch
        // Clear the graph
        self.graph = Graph::new(GraphType::Directed);
        self.length = 0;

        // Restore the rules and rulesets
        self.graph.rules = saved_rules;
        self.graph.rulesets = saved_rulesets;

        // Re-add all values
        for val in values {
            self.append_raw(val)?;
        }

        Ok(())
    }

    /// Insert a value at a specific index without applying behaviors
    ///
    /// This is an internal method used by the executor when behaviors have
    /// already been applied with full executor context (for function-based behaviors).
    ///
    /// # Arguments
    /// * `index` - The position to insert at (0 = beginning, length = end)
    /// * `value` - The value to insert (already transformed)
    ///
    /// # Returns
    /// `Ok(())` if successful, or an error if the index is out of bounds
    pub fn insert_at_raw(&mut self, index: usize, value: Value) -> Result<(), GraphoidError> {
        if index > self.length {
            return Err(GraphoidError::runtime(format!(
                "Index {} out of bounds for list of length {} (insert)",
                index, self.length
            )));
        }

        // Collect all values
        let mut values = self.to_vec();

        // Insert value at position (no behavior application)
        values.insert(index, value);

        // Save the rules before rebuilding
        let saved_rules = self.graph.rules.clone();
        let saved_rulesets = self.graph.rulesets.clone();

        // Rebuild list from scratch
        // Clear the graph
        self.graph = Graph::new(GraphType::Directed);
        self.length = 0;

        // Restore the rules and rulesets
        self.graph.rules = saved_rules;
        self.graph.rulesets = saved_rulesets;

        // Re-add all values
        for val in values {
            self.append_raw(val)?;
        }

        Ok(())
    }

    /// Add a rule to this list
    ///
    /// If the rule is a transformation rule, it will be applied retroactively
    /// to all existing values in the list (with RetroactivePolicy::Clean).
    pub fn add_rule(&mut self, rule: RuleInstance) -> Result<(), GraphoidError> {
        // If it's a transformation rule, apply it retroactively to existing values
        if rule.spec.is_transformation_rule() {
            let rule_impl = rule.spec.instantiate();

            // Transform all existing values
            for i in 0..self.length {
                let node_id = format!("node_{}", i);
                if let Some(node) = self.graph.nodes.get_mut(&node_id) {
                    node.value = rule_impl.transform(&node.value)?;
                }
            }
        }

        // Add the rule to the graph
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

    /// Prepend a value to the beginning of the list (raw, no rules applied)
    pub fn prepend_raw(&mut self, value: Value) -> Result<(), GraphoidError> {
        self.insert_at_raw(0, value)
    }

    /// Remove the first occurrence of a value from the list
    pub fn remove_value(&mut self, value: &Value) -> Result<(), GraphoidError> {
        let elements = self.to_vec();
        for (idx, element) in elements.iter().enumerate() {
            if element == value {
                return self.remove_at_index(idx);
            }
        }
        // Value not found - not an error, just no-op
        Ok(())
    }

    /// Remove element at a specific index
    pub fn remove_at_index(&mut self, index: usize) -> Result<(), GraphoidError> {
        if index >= self.len() {
            return Err(GraphoidError::runtime(format!(
                "Index {} out of bounds for list of length {}",
                index,
                self.len()
            )));
        }

        // Rebuild the list without the element at the given index
        let mut new_elements = Vec::new();
        let elements = self.to_vec();
        for (idx, element) in elements.into_iter().enumerate() {
            if idx != index {
                new_elements.push(element);
            }
        }

        // Preserve rules before rebuilding
        let old_rules = self.graph.rules.clone();

        // Rebuild the graph
        self.graph = Graph::new(GraphType::Directed);
        self.graph.rules = old_rules;
        self.length = 0; // Reset length

        for value in new_elements {
            self.append_raw(value)?;
        }

        Ok(())
    }

    /// Remove and return the last element
    pub fn pop(&mut self) -> Result<Value, GraphoidError> {
        if self.is_empty() {
            return Err(GraphoidError::runtime("Cannot pop from empty list".to_string()));
        }

        let last_index = self.len() - 1;
        let last_value = self.get(last_index)
            .cloned()
            .ok_or_else(|| GraphoidError::runtime("Failed to get last element".to_string()))?;

        self.remove_at_index(last_index)?;
        Ok(last_value)
    }

    /// Clear all elements from the list
    pub fn clear(&mut self) {
        let old_rules = self.graph.rules.clone();
        self.graph = Graph::new(GraphType::Directed);
        self.graph.rules = old_rules;
        self.length = 0; // Reset length
    }

}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}
