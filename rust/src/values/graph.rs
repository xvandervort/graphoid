//! Graph data structure implementation
//!
//! Graphoid's graph type uses index-free adjacency for O(1) neighbor lookups.
//! Each node stores direct pointers to its neighbors, avoiding index scans.

use std::collections::{HashMap, HashSet, VecDeque};
use super::Value;
use crate::graph::rules::{Rule, RuleContext, GraphOperation, RuleSpec, RuleInstance, RuleSeverity};
use crate::graph::rulesets::get_ruleset_rules;
use crate::error::GraphoidError;

/// Type of graph: directed or undirected
#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Directed,
    Undirected,
}

/// A node in the graph
#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    /// Node identifier
    pub id: String,
    /// Node value
    pub value: Value,
    /// Node properties (for property-based indexing)
    pub properties: HashMap<String, Value>,
    /// Outgoing edges (neighbor_id -> edge_info)
    pub neighbors: HashMap<String, EdgeInfo>,
}

/// Information about an edge
#[derive(Debug, Clone, PartialEq)]
pub struct EdgeInfo {
    /// Edge type/label
    pub edge_type: String,
    /// Edge properties
    pub properties: HashMap<String, Value>,
}

/// Execution plan for graph operations
///
/// Shows what algorithm will be used, why, and estimated cost
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    /// Name of the operation
    pub operation: String,
    /// Steps in the execution plan
    pub steps: Vec<String>,
    /// Estimated cost (number of operations)
    pub estimated_cost: usize,
    /// Rule optimizations applied
    pub optimizations: Vec<String>,
}

impl ExecutionPlan {
    /// Create a new execution plan
    pub fn new(operation: String) -> Self {
        ExecutionPlan {
            operation,
            steps: Vec::new(),
            estimated_cost: 0,
            optimizations: Vec::new(),
        }
    }

    /// Add a step to the execution plan
    pub fn add_step(&mut self, step: String) {
        self.steps.push(step);
    }

    /// Add an optimization note
    pub fn add_optimization(&mut self, optimization: String) {
        self.optimizations.push(optimization);
    }

    /// Set the estimated cost
    pub fn set_cost(&mut self, cost: usize) {
        self.estimated_cost = cost;
    }

    /// Check if the plan shows an estimated cost
    pub fn shows_estimated_cost(&self) -> bool {
        self.estimated_cost > 0
    }
}

impl std::fmt::Display for ExecutionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Execution Plan: {}", self.operation)?;
        for (i, step) in self.steps.iter().enumerate() {
            writeln!(f, "  {}. {}", i + 1, step)?;
        }
        writeln!(f, "Estimated cost: {} operations", self.estimated_cost)?;
        if !self.optimizations.is_empty() {
            writeln!(f, "Optimizations applied:")?;
            for opt in &self.optimizations {
                writeln!(f, "  - {}", opt)?;
            }
        }
        Ok(())
    }
}

/// Result of validation - either allowed or rejected with severity
enum ValidationResult {
    Allowed,
    Rejected {
        rule: String,
        severity: RuleSeverity,
        message: String,
    },
}

/// Graph data structure with index-free adjacency and auto-optimization
#[derive(Debug, Clone)]
pub struct Graph {
    /// Graph type (directed or undirected)
    pub graph_type: GraphType,
    /// Nodes by ID for O(1) lookup
    pub nodes: HashMap<String, GraphNode>,
    /// Active rulesets (e.g., "tree", "dag", "bst")
    /// Predefined bundles of rules applied via with_ruleset()
    pub rulesets: Vec<String>,
    /// Ad hoc rules added via add_rule()
    /// These are in addition to any ruleset rules
    /// Each rule includes its configured severity
    pub rules: Vec<RuleInstance>,

    // Auto-optimization state (not included in PartialEq)
    /// Track property lookup frequencies for auto-indexing
    /// Maps property name -> access count
    property_access_counts: HashMap<String, usize>,
    /// Auto-created property indices
    /// Maps property name -> (value_string -> node IDs with that property value)
    /// We use String for the value key because Value contains f64 which doesn't impl Hash
    property_indices: HashMap<String, HashMap<String, Vec<String>>>,
    /// Threshold for auto-index creation (default: 10 accesses)
    auto_index_threshold: usize,
}

// Manual PartialEq implementation that ignores optimization state
impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.graph_type == other.graph_type
            && self.nodes == other.nodes
            && self.rulesets == other.rulesets
            && self.rules == other.rules
        // Deliberately ignore: property_access_counts, property_indices, auto_index_threshold
    }
}

impl Graph {
    /// Create a new empty graph
    pub fn new(graph_type: GraphType) -> Self {
        Graph {
            graph_type,
            nodes: HashMap::new(),
            rulesets: Vec::new(),
            rules: Vec::new(),
            // Auto-optimization state
            property_access_counts: HashMap::new(),
            property_indices: HashMap::new(),
            auto_index_threshold: 10, // Create index after 10 lookups
        }
    }

    /// Get all active rules for this graph from both rulesets AND ad hoc rules
    fn get_active_rules(&self) -> Vec<(Box<dyn Rule>, RuleSeverity)> {
        let mut rule_instances: Vec<RuleInstance> = Vec::new();

        // Add rules from predefined rulesets using the rulesets module
        for ruleset in &self.rulesets {
            let ruleset_rules = get_ruleset_rules(ruleset);
            rule_instances.extend(ruleset_rules);
        }

        // Add ad hoc rules (with their configured severities)
        rule_instances.extend(self.rules.clone());

        // Deduplicate rules by name (keep first occurrence)
        let mut seen = HashSet::new();
        let mut unique_instances = Vec::new();
        for instance in rule_instances {
            if seen.insert(instance.spec.name().to_string()) {
                unique_instances.push(instance);
            }
        }

        // Instantiate all rule instances into (Rule, Severity) pairs
        unique_instances
            .into_iter()
            .map(|instance| (instance.spec.instantiate(), instance.severity))
            .collect()
    }

    /// Validate an operation against all active rules
    /// Returns Allowed if all rules pass, or Rejected with severity if any rule fails
    fn validate_rules(&self, operation: GraphOperation) -> ValidationResult {
        let rules = self.get_active_rules();
        let context = RuleContext::new(operation.clone());

        for (rule, severity) in rules {
            if rule.should_run_on(&operation) {
                if let Err(err) = rule.validate(self, &context) {
                    // Rule violation detected
                    return ValidationResult::Rejected {
                        rule: rule.name().to_string(),
                        severity,
                        message: err.to_string(),
                    };
                }
            }
        }

        ValidationResult::Allowed
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, id: String, value: Value) -> Result<(), GraphoidError> {
        // Validate the operation against active rules
        let operation = GraphOperation::AddNode {
            id: id.clone(),
            value: value.clone(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                self.nodes.insert(
                    id.clone(),
                    GraphNode {
                        id,
                        value,
                        properties: HashMap::new(),
                        neighbors: HashMap::new(),
                    },
                );
                Ok(())
            }
            ValidationResult::Rejected {
                rule,
                severity,
                message,
            } => {
                // Operation is ALWAYS rejected (returns Err)
                // Severity only controls logging
                match severity {
                    RuleSeverity::Silent => {
                        // REJECT: Return RuleViolation error without logging
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                    RuleSeverity::Warning => {
                        // REJECT: Log warning and return RuleViolation error
                        eprintln!("WARNING: {}", message);
                        Err(GraphoidError::RuleViolation {
                            rule: rule.clone(),
                            message,
                        })
                    }
                    RuleSeverity::Error => {
                        // REJECT: Return RuleViolation error
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                }
            }
        }
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: &str, to: &str, edge_type: String, properties: HashMap<String, Value>) -> Result<(), GraphoidError> {
        // Validate the operation against active rules
        let operation = GraphOperation::AddEdge {
            from: from.to_string(),
            to: to.to_string(),
            edge_type: edge_type.clone(),
            properties: properties.clone(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                // Add forward edge
                if let Some(from_node) = self.nodes.get_mut(from) {
                    from_node.neighbors.insert(
                        to.to_string(),
                        EdgeInfo {
                            edge_type: edge_type.clone(),
                            properties: properties.clone(),
                        },
                    );
                }

                // For undirected graphs, add reverse edge
                if self.graph_type == GraphType::Undirected {
                    if let Some(to_node) = self.nodes.get_mut(to) {
                        to_node.neighbors.insert(
                            from.to_string(),
                            EdgeInfo {
                                edge_type,
                                properties,
                            },
                        );
                    }
                }

                Ok(())
            }
            ValidationResult::Rejected {
                rule,
                severity,
                message,
            } => {
                // Operation is ALWAYS rejected (returns Err)
                // Severity only controls logging
                match severity {
                    RuleSeverity::Silent => {
                        // REJECT: Return RuleViolation error without logging
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                    RuleSeverity::Warning => {
                        // REJECT: Log warning and return RuleViolation error
                        eprintln!("WARNING: {}", message);
                        Err(GraphoidError::RuleViolation {
                            rule: rule.clone(),
                            message,
                        })
                    }
                    RuleSeverity::Error => {
                        // REJECT: Return RuleViolation error
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                }
            }
        }
    }

    /// Check if a node exists
    pub fn has_node(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    /// Check if an edge exists
    pub fn has_edge(&self, from: &str, to: &str) -> bool {
        if let Some(node) = self.nodes.get(from) {
            node.neighbors.contains_key(to)
        } else {
            false
        }
    }

    /// Get neighbors of a node (O(1) lookup, O(degree) iteration)
    pub fn neighbors(&self, id: &str) -> Vec<String> {
        if let Some(node) = self.nodes.get(id) {
            node.neighbors.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.nodes.values().map(|n| n.neighbors.len()).sum()
    }

    /// Remove a node from the graph
    pub fn remove_node(&mut self, id: &str) -> Result<Option<GraphNode>, GraphoidError> {
        // Validate the operation against active rules
        let operation = GraphOperation::RemoveNode {
            id: id.to_string(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                // Remove the node
                let removed = self.nodes.remove(id);

                // Remove all edges pointing to this node
                for node in self.nodes.values_mut() {
                    node.neighbors.remove(id);
                }

                Ok(removed)
            }
            ValidationResult::Rejected {
                rule,
                severity,
                message,
            } => {
                // Operation is ALWAYS rejected (returns Err)
                // Severity only controls logging
                match severity {
                    RuleSeverity::Silent => {
                        // REJECT: Return RuleViolation error without logging
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                    RuleSeverity::Warning => {
                        // REJECT: Log warning and return RuleViolation error
                        eprintln!("WARNING: {}", message);
                        Err(GraphoidError::RuleViolation {
                            rule: rule.clone(),
                            message,
                        })
                    }
                    RuleSeverity::Error => {
                        // REJECT: Return RuleViolation error
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                }
            }
        }
    }

    /// Remove an edge
    pub fn remove_edge(&mut self, from: &str, to: &str) -> Result<bool, GraphoidError> {
        // Validate the operation against active rules
        let operation = GraphOperation::RemoveEdge {
            from: from.to_string(),
            to: to.to_string(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                let mut removed = false;

                if let Some(from_node) = self.nodes.get_mut(from) {
                    removed = from_node.neighbors.remove(to).is_some();
                }

                // For undirected graphs, remove reverse edge
                if self.graph_type == GraphType::Undirected {
                    if let Some(to_node) = self.nodes.get_mut(to) {
                        to_node.neighbors.remove(from);
                    }
                }

                Ok(removed)
            }
            ValidationResult::Rejected {
                rule,
                severity,
                message,
            } => {
                // Operation is ALWAYS rejected (returns Err)
                // Severity only controls logging
                match severity {
                    RuleSeverity::Silent => {
                        // REJECT: Return RuleViolation error without logging
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                    RuleSeverity::Warning => {
                        // REJECT: Log warning and return RuleViolation error
                        eprintln!("WARNING: {}", message);
                        Err(GraphoidError::RuleViolation {
                            rule: rule.clone(),
                            message,
                        })
                    }
                    RuleSeverity::Error => {
                        // REJECT: Return RuleViolation error
                        Err(GraphoidError::RuleViolation { rule, message })
                    }
                }
            }
        }
    }

    /// Get node value
    pub fn get_node(&self, id: &str) -> Option<&Value> {
        self.nodes.get(id).map(|n| &n.value)
    }

    /// Get all node IDs (like map.keys())
    pub fn keys(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }

    /// Get all node values (like map.values())
    pub fn values(&self) -> Vec<Value> {
        self.nodes.values().map(|n| n.value.clone()).collect()
    }

    // ========================================================================
    // Tree-like convenience methods (for Option A refactor)
    // ========================================================================

    /// Insert a value into the graph with optional parent
    /// Returns the ID of the newly created node
    ///
    /// This is a tree-like convenience method that:
    /// - Generates a unique node ID
    /// - Adds the node with the given value
    /// - If parent is specified, adds an edge from parent to new node
    pub fn insert(&mut self, value: Value, parent: Option<&str>) -> Result<String, GraphoidError> {
        // Generate unique node ID
        let node_id = format!("node_{}", self.nodes.len());

        // Add the node
        self.add_node(node_id.clone(), value)?;

        // If parent specified, add edge from parent to child
        if let Some(parent_id) = parent {
            self.add_edge(parent_id, &node_id, "child".to_string(), HashMap::new())?;
        }

        Ok(node_id)
    }

    /// Check if the graph contains a node with the given value
    pub fn contains(&self, value: &Value) -> bool {
        self.nodes.values().any(|node| &node.value == value)
    }

    /// Breadth-first search traversal starting from a given node
    /// Returns node IDs in BFS order
    pub fn bfs(&self, start: &str) -> Vec<String> {
        // Check if start node exists
        if !self.has_node(start) {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Start from the given node
        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());

            // Add unvisited neighbors to queue
            if let Some(node) = self.nodes.get(&node_id) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }
        }

        result
    }

    /// Depth-first search traversal starting from a given node
    /// Returns node IDs in DFS order
    pub fn dfs(&self, start: &str) -> Vec<String> {
        // Check if start node exists
        if !self.has_node(start) {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        // Start from the given node
        stack.push(start.to_string());

        while let Some(node_id) = stack.pop() {
            if visited.contains(&node_id) {
                continue;
            }

            visited.insert(node_id.clone());
            result.push(node_id.clone());

            // Add unvisited neighbors to stack
            if let Some(node) = self.nodes.get(&node_id) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        stack.push(neighbor_id.clone());
                    }
                }
            }
        }

        result
    }

    /// In-order traversal (left, root, right) starting from a given node
    /// Assumes binary tree structure where first child is left, second is right
    /// Returns values in in-order
    pub fn in_order(&self, start: &str) -> Vec<Value> {
        let mut result = Vec::new();
        self.in_order_rec(start, &mut result);
        result
    }

    /// Recursive helper for in-order traversal
    fn in_order_rec(&self, node_id: &str, result: &mut Vec<Value>) {
        if let Some(node) = self.nodes.get(node_id) {
            let children: Vec<String> = node.neighbors.keys().cloned().collect();

            // Process left child (first child)
            if !children.is_empty() {
                self.in_order_rec(&children[0], result);
            }

            // Process current node
            result.push(node.value.clone());

            // Process right child (second child)
            if children.len() > 1 {
                self.in_order_rec(&children[1], result);
            }
        }
    }

    /// Pre-order traversal (root, left, right) starting from a given node
    /// Assumes binary tree structure where first child is left, second is right
    /// Returns values in pre-order
    pub fn pre_order(&self, start: &str) -> Vec<Value> {
        let mut result = Vec::new();
        self.pre_order_rec(start, &mut result);
        result
    }

    /// Recursive helper for pre-order traversal
    fn pre_order_rec(&self, node_id: &str, result: &mut Vec<Value>) {
        if let Some(node) = self.nodes.get(node_id) {
            let children: Vec<String> = node.neighbors.keys().cloned().collect();

            // Process current node first
            result.push(node.value.clone());

            // Process left child (first child)
            if !children.is_empty() {
                self.pre_order_rec(&children[0], result);
            }

            // Process right child (second child)
            if children.len() > 1 {
                self.pre_order_rec(&children[1], result);
            }
        }
    }

    /// Post-order traversal (left, right, root) starting from a given node
    /// Assumes binary tree structure where first child is left, second is right
    /// Returns values in post-order
    pub fn post_order(&self, start: &str) -> Vec<Value> {
        let mut result = Vec::new();
        self.post_order_rec(start, &mut result);
        result
    }

    /// Recursive helper for post-order traversal
    fn post_order_rec(&self, node_id: &str, result: &mut Vec<Value>) {
        if let Some(node) = self.nodes.get(node_id) {
            let children: Vec<String> = node.neighbors.keys().cloned().collect();

            // Process left child (first child)
            if !children.is_empty() {
                self.post_order_rec(&children[0], result);
            }

            // Process right child (second child)
            if children.len() > 1 {
                self.post_order_rec(&children[1], result);
            }

            // Process current node last
            result.push(node.value.clone());
        }
    }

    // ========================================================================
    // Rule and Ruleset methods
    // ========================================================================
    // Rules can be applied in two ways:
    // 1. Rulesets: Predefined bundles (e.g., :tree, :dag, :binary_tree)
    // 2. Ad hoc rules: Individual rules added/removed dynamically

    /// Apply a ruleset to this graph
    /// Returns self for method chaining
    ///
    /// Rulesets are predefined bundles of rules:
    /// - :tree → no_cycles + single_root + connected
    /// - :binary_tree → tree rules + max 2 children
    /// - :dag → no_cycles only
    ///
    /// When a ruleset is applied:
    /// 1. The ruleset name is stored in self.rulesets
    /// 2. Rules from the ruleset are automatically enforced during validation
    /// 3. Ruleset rules are kept separate from ad hoc rules added via add_rule()
    ///
    /// # Example
    ///
    /// ```
    /// use graphoid::values::{Graph, GraphType, Value};
    ///
    /// let mut g = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    /// g.add_node("root".to_string(), Value::Number(1.0)).unwrap();
    /// // Tree rules are now enforced: no_cycles, single_root, connected
    /// ```
    pub fn with_ruleset(mut self, ruleset: String) -> Self {
        // Store the ruleset name
        // Rules from the ruleset will be retrieved dynamically during validation
        if !self.rulesets.contains(&ruleset) {
            self.rulesets.push(ruleset);
        }
        self
    }

    /// Check if graph has a specific ruleset applied
    pub fn has_ruleset(&self, ruleset: &str) -> bool {
        self.rulesets.contains(&ruleset.to_string())
    }

    /// Get all active rulesets
    pub fn get_rulesets(&self) -> &[String] {
        &self.rulesets
    }

    /// Add an ad hoc rule to this graph
    ///
    /// Rules are enforced on all mutation operations (add_node, add_edge, etc.)
    /// Rules are in addition to any ruleset rules.
    pub fn add_rule(&mut self, rule_instance: RuleInstance) -> Result<(), GraphoidError> {
        // Don't add duplicate rules (check by spec)
        if self.rules.iter().any(|r| r.spec == rule_instance.spec) {
            return Ok(());
        }

        // Handle retroactive policy
        let retroactive_policy = rule_instance.spec.instantiate().default_retroactive_policy();
        match retroactive_policy {
            crate::graph::RetroactivePolicy::Clean => {
                // Try to clean existing violations
                let rule_obj = rule_instance.spec.instantiate();
                match rule_obj.clean(self) {
                    Ok(()) => {
                        // Cleaning succeeded - proceed to add the rule
                    }
                    Err(_) => {
                        // clean() failed - either rule doesn't support cleaning OR can't clean violations
                        // Check if there are ACTUAL violations
                        let dummy_op = GraphOperation::AddNode {
                            id: "__validation_check__".to_string(),
                            value: Value::Number(0.0),
                        };
                        let context = RuleContext::new(dummy_op);

                        if let Err(_) = rule_obj.validate(self, &context) {
                            // There ARE violations - reject add_rule()
                            eprintln!(
                                "WARNING: Cannot add rule '{}' - existing data violates rule and cannot be automatically cleaned",
                                rule_instance.spec.name()
                            );
                            return Ok(());
                        }
                        // No violations - safe to add the rule even though clean() failed
                        // (probably just means rule doesn't support cleaning)
                    }
                }
            }
            crate::graph::RetroactivePolicy::Warn => {
                // Check for existing violations and warn
                // We'll implement this later - for now just add the rule
            }
            crate::graph::RetroactivePolicy::Enforce => {
                // Error if violations exist
                // We'll implement this later - for now just add the rule
            }
            crate::graph::RetroactivePolicy::Ignore => {
                // Don't check existing data - just add the rule
            }
        }

        self.rules.push(rule_instance);
        Ok(())
    }

    /// Remove an ad hoc rule from this graph
    ///
    /// This removes a rule that was added via add_rule().
    /// It does NOT remove rules that come from rulesets.
    pub fn remove_rule(&mut self, rule_spec: &RuleSpec) {
        self.rules.retain(|r| &r.spec != rule_spec);
    }

    /// Get all ad hoc rules (not including ruleset rules)
    pub fn get_rules(&self) -> &[RuleInstance] {
        &self.rules
    }

    /// Get all active rule specs (including both ruleset rules and ad hoc rules)
    ///
    /// This returns a vector of RuleSpec objects representing all rules currently
    /// enforced on this graph, from both applied rulesets and ad hoc rules.
    ///
    /// # Example
    ///
    /// ```
    /// use graphoid::values::{Graph, GraphType};
    /// use graphoid::graph::RuleSpec;
    ///
    /// let g = Graph::new(GraphType::Directed).with_ruleset("tree".to_string());
    /// let specs = g.get_active_rule_specs();
    /// assert!(specs.contains(&RuleSpec::NoCycles));
    /// assert!(specs.contains(&RuleSpec::SingleRoot));
    /// assert!(specs.contains(&RuleSpec::Connected));
    /// ```
    pub fn get_active_rule_specs(&self) -> Vec<RuleSpec> {
        let mut specs = Vec::new();

        // Add rules from rulesets
        for ruleset in &self.rulesets {
            let ruleset_rules = get_ruleset_rules(ruleset);
            specs.extend(ruleset_rules.iter().map(|r| r.spec.clone()));
        }

        // Add ad hoc rules
        specs.extend(self.rules.iter().map(|r| r.spec.clone()));

        // Deduplicate by spec
        specs.sort_by(|a, b| format!("{:?}", a).cmp(&format!("{:?}", b)));
        specs.dedup();

        specs
    }

    /// Check if a specific rule is active (from either rulesets or ad hoc)
    pub fn has_rule(&self, rule_name: &str) -> bool {
        // Check ad hoc rules
        if self.rules.iter().any(|r| r.spec.name() == rule_name) {
            return true;
        }

        // Check ruleset rules
        for ruleset in &self.rulesets {
            match ruleset.as_str() {
                "tree" => {
                    if matches!(rule_name, "no_cycles" | "single_root" | "connected") {
                        return true;
                    }
                }
                "binary_tree" => {
                    if matches!(rule_name, "no_cycles" | "single_root" | "connected" | "binary_tree") {
                        return true;
                    }
                }
                "dag" => {
                    if rule_name == "no_cycles" {
                        return true;
                    }
                }
                _ => {}
            }
        }

        false
    }

    // ========================================================================
    // Auto-Optimization: Property-based Indexing
    // ========================================================================

    /// Find nodes by property value with automatic indexing
    ///
    /// Tracks access patterns and automatically creates indices after threshold (default: 10 lookups).
    /// First lookups are O(n) but become O(1) after index is created.
    ///
    /// # Example
    /// ```no_run
    /// use graphoid::values::{Graph, Value};
    /// use graphoid::values::graph::GraphType;
    ///
    /// let mut g = Graph::new(GraphType::Directed);
    /// // After 10+ lookups on "user_id", an index is auto-created
    /// let nodes = g.find_nodes_by_property("user_id", &Value::Number(42.0));
    /// ```
    pub fn find_nodes_by_property(&mut self, property: &str, value: &Value) -> Vec<String> {
        // Track access pattern
        *self.property_access_counts.entry(property.to_string()).or_insert(0) += 1;
        let access_count = self.property_access_counts[property];

        // Create index if threshold reached and index doesn't exist
        if access_count >= self.auto_index_threshold && !self.property_indices.contains_key(property) {
            self.create_property_index(property);
        }

        // Use index if available (O(1) lookup)
        if let Some(index) = self.property_indices.get(property) {
            let value_key = value.to_string();
            if let Some(node_ids) = index.get(&value_key) {
                return node_ids.clone();
            } else {
                return Vec::new();
            }
        }

        // Otherwise, linear scan (O(n))
        let mut result = Vec::new();
        for (node_id, node) in &self.nodes {
            if let Some(prop_value) = node.properties.get(property) {
                if prop_value == value {
                    result.push(node_id.clone());
                }
            }
        }
        result
    }

    /// Create an index for a property
    ///
    /// Scans all nodes and builds a HashMap: property_value_string -> Vec<node_id>
    fn create_property_index(&mut self, property: &str) {
        let mut index: HashMap<String, Vec<String>> = HashMap::new();

        for (node_id, node) in &self.nodes {
            if let Some(value) = node.properties.get(property) {
                let value_key = value.to_string();
                index
                    .entry(value_key)
                    .or_insert_with(Vec::new)
                    .push(node_id.clone());
            }
        }

        self.property_indices.insert(property.to_string(), index);
    }

    /// Get comprehensive statistics about the graph
    ///
    /// Returns detailed information including:
    /// - Node and edge counts
    /// - Degree distribution (min, max, average)
    /// - Auto-created indices
    /// - Active rules and rulesets
    pub fn stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();

        // Basic counts
        stats.insert("node_count".to_string(), serde_json::json!(self.nodes.len()));
        stats.insert("edge_count".to_string(), serde_json::json!(self.edge_count()));

        // Degree distribution
        let degrees = self.degree_distribution();
        stats.insert("degree_distribution".to_string(), serde_json::json!(degrees));

        // Auto-optimization info
        let auto_indices: Vec<String> = self.property_indices.keys().cloned().collect();
        stats.insert("auto_indices".to_string(), serde_json::json!(auto_indices));

        // Auto-optimizations summary
        let mut auto_opts = Vec::new();
        for property in &auto_indices {
            auto_opts.push(format!("{} indexed", property));
        }
        stats.insert("auto_optimizations".to_string(), serde_json::json!(auto_opts));

        // Rules information
        stats.insert("rulesets".to_string(), serde_json::json!(self.rulesets));
        stats.insert("ad_hoc_rules".to_string(), serde_json::json!(self.rules.len()));

        stats
    }

    /// Calculate degree distribution statistics
    fn degree_distribution(&self) -> HashMap<String, usize> {
        let mut dist = HashMap::new();

        if self.nodes.is_empty() {
            dist.insert("min".to_string(), 0);
            dist.insert("max".to_string(), 0);
            dist.insert("average".to_string(), 0);
            return dist;
        }

        let mut degrees: Vec<usize> = self.nodes.values()
            .map(|node| node.neighbors.len())
            .collect();

        degrees.sort_unstable();

        let min = *degrees.first().unwrap_or(&0);
        let max = *degrees.last().unwrap_or(&0);
        let sum: usize = degrees.iter().sum();
        let avg = sum / degrees.len();

        dist.insert("min".to_string(), min);
        dist.insert("max".to_string(), max);
        dist.insert("average".to_string(), avg);

        dist
    }

    /// Check if a property has an auto-created index
    pub fn has_auto_index(&self, property: &str) -> bool {
        self.property_indices.contains_key(property)
    }

    // ========================================================================
    // Explain: Show Execution Plans
    // ========================================================================

    /// Explain how a property lookup would be executed
    ///
    /// Shows whether an index exists, what algorithm will be used, and estimated cost
    pub fn explain_find_property(&self, property: &str) -> ExecutionPlan {
        let mut plan = ExecutionPlan::new(format!("find_nodes_by_property('{}')", property));

        // Check if index exists
        if self.has_auto_index(property) {
            plan.add_step("Use property index (O(1) lookup)".to_string());
            plan.add_optimization(format!("Property '{}' is indexed", property));
            plan.set_cost(1); // O(1) hash lookup
        } else {
            plan.add_step("Linear scan through all nodes (O(n))".to_string());
            let access_count = self.property_access_counts.get(property).unwrap_or(&0);
            plan.add_step(format!(
                "Access count: {}/{} (index created after {} accesses)",
                access_count, self.auto_index_threshold, self.auto_index_threshold
            ));
            // O(n) linear scan - minimum cost of 1 even for empty graphs
            plan.set_cost(self.nodes.len().max(1));
        }

        plan
    }

    /// Explain how a shortest path operation would be executed
    ///
    /// Shows which algorithm will be used based on active rules
    pub fn explain_shortest_path(&self, from: &str, to: &str) -> ExecutionPlan {
        let mut plan = ExecutionPlan::new(format!("shortest_path('{}', '{}')", from, to));

        // Check for no_cycles rule (enables topological algorithms)
        if self.has_rule("no_cycles") {
            plan.add_step("Topological sort (DAG-optimized)".to_string());
            plan.add_step(format!("BFS from '{}'", from));
            plan.add_step("Path reconstruction".to_string());
            plan.add_optimization("no_cycles → enabled topological algorithms".to_string());
            plan.set_cost(self.nodes.len() + self.edge_count());
        } else {
            plan.add_step(format!("BFS from '{}'", from));
            plan.add_step("Path reconstruction".to_string());
            plan.set_cost(self.nodes.len() + self.edge_count());
        }

        plan
    }

    /// Explain how a BFS traversal would be executed
    pub fn explain_bfs(&self, start: &str) -> ExecutionPlan {
        let mut plan = ExecutionPlan::new(format!("bfs('{}')", start));

        plan.add_step("Initialize queue with start node".to_string());
        plan.add_step("Mark start node as visited".to_string());
        plan.add_step("While queue not empty: dequeue, visit neighbors".to_string());
        plan.add_step("Add unvisited neighbors to queue".to_string());

        // Check for connected rule
        if self.has_rule("connected") {
            plan.add_optimization("connected → skip component check".to_string());
        }

        plan.set_cost(self.nodes.len() + self.edge_count());

        plan
    }
}
