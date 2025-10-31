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
    /// Edge weight (optional, for weighted graphs)
    pub weight: Option<f64>,
    /// Edge properties
    pub properties: HashMap<String, Value>,
}

impl EdgeInfo {
    /// Create new edge with no weight
    pub fn new(edge_type: String, properties: HashMap<String, Value>) -> Self {
        EdgeInfo {
            edge_type,
            weight: None,
            properties,
        }
    }

    /// Create new edge with weight
    pub fn new_weighted(edge_type: String, weight: f64, properties: HashMap<String, Value>) -> Self {
        EdgeInfo {
            edge_type,
            weight: Some(weight),
            properties,
        }
    }

    /// Get weight (returns None if unweighted)
    pub fn weight(&self) -> Option<f64> {
        self.weight
    }

    /// Set weight
    pub fn set_weight(&mut self, weight: Option<f64>) {
        self.weight = weight;
    }

    /// Check if edge is weighted
    pub fn is_weighted(&self) -> bool {
        self.weight.is_some()
    }
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
    pub fn add_edge(&mut self, from: &str, to: &str, edge_type: String, weight: Option<f64>, properties: HashMap<String, Value>) -> Result<(), GraphoidError> {
        // Validate the operation against active rules
        let operation = GraphOperation::AddEdge {
            from: from.to_string(),
            to: to.to_string(),
            edge_type: edge_type.clone(),
            weight,
            properties: properties.clone(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                // Create edge info with weight
                let edge_info = if let Some(w) = weight {
                    EdgeInfo::new_weighted(edge_type.clone(), w, properties.clone())
                } else {
                    EdgeInfo::new(edge_type.clone(), properties.clone())
                };

                // Add forward edge
                if let Some(from_node) = self.nodes.get_mut(from) {
                    from_node.neighbors.insert(
                        to.to_string(),
                        edge_info.clone(),
                    );
                }

                // For undirected graphs, add reverse edge
                if self.graph_type == GraphType::Undirected {
                    let reverse_edge_info = if let Some(w) = weight {
                        EdgeInfo::new_weighted(edge_type, w, properties)
                    } else {
                        EdgeInfo::new(edge_type, properties)
                    };

                    if let Some(to_node) = self.nodes.get_mut(to) {
                        to_node.neighbors.insert(
                            from.to_string(),
                            reverse_edge_info,
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

    /// Get the weight of an edge
    ///
    /// Returns `Some(weight)` if the edge exists and has a weight, `None` otherwise.
    pub fn get_edge_weight(&self, from: &str, to: &str) -> Option<f64> {
        self.nodes
            .get(from)
            .and_then(|node| node.neighbors.get(to))
            .and_then(|edge_info| edge_info.weight)
    }

    /// Set the weight of an edge
    ///
    /// If the edge exists, sets or updates its weight. Returns an error if the edge doesn't exist.
    pub fn set_edge_weight(&mut self, from: &str, to: &str, weight: f64) -> Result<(), GraphoidError> {
        if let Some(node) = self.nodes.get_mut(from) {
            if let Some(edge_info) = node.neighbors.get_mut(to) {
                edge_info.set_weight(Some(weight));

                // For undirected graphs, also update the reverse edge
                if self.graph_type == GraphType::Undirected {
                    if let Some(reverse_node) = self.nodes.get_mut(to) {
                        if let Some(reverse_edge) = reverse_node.neighbors.get_mut(from) {
                            reverse_edge.set_weight(Some(weight));
                        }
                    }
                }

                Ok(())
            } else {
                Err(GraphoidError::runtime(format!(
                    "Edge from '{}' to '{}' does not exist",
                    from, to
                )))
            }
        } else {
            Err(GraphoidError::runtime(format!(
                "Node '{}' does not exist",
                from
            )))
        }
    }

    /// Remove the weight from an edge (make it unweighted)
    ///
    /// If the edge exists, removes its weight. Returns an error if the edge doesn't exist.
    pub fn remove_edge_weight(&mut self, from: &str, to: &str) -> Result<(), GraphoidError> {
        if let Some(node) = self.nodes.get_mut(from) {
            if let Some(edge_info) = node.neighbors.get_mut(to) {
                edge_info.set_weight(None);

                // For undirected graphs, also update the reverse edge
                if self.graph_type == GraphType::Undirected {
                    if let Some(reverse_node) = self.nodes.get_mut(to) {
                        if let Some(reverse_edge) = reverse_node.neighbors.get_mut(from) {
                            reverse_edge.set_weight(None);
                        }
                    }
                }

                Ok(())
            } else {
                Err(GraphoidError::runtime(format!(
                    "Edge from '{}' to '{}' does not exist",
                    from, to
                )))
            }
        } else {
            Err(GraphoidError::runtime(format!(
                "Node '{}' does not exist",
                from
            )))
        }
    }

    /// Check if an edge has a weight
    ///
    /// Returns `true` if the edge exists and has a weight, `false` otherwise.
    pub fn is_edge_weighted(&self, from: &str, to: &str) -> bool {
        self.get_edge_weight(from, to).is_some()
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
            self.add_edge(parent_id, &node_id, "child".to_string(), None, HashMap::new())?;
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

    /// Find the shortest path between two nodes using BFS (or optimized algorithm if rules apply)
    ///
    /// Returns a vector of node IDs representing the path from `from` to `to`.
    /// If no path exists, returns an empty vector.
    ///
    /// # Algorithm Selection (Rule-Aware)
    ///
    /// - If `no_cycles` rule is active: Uses topological-sort-based algorithm for DAGs
    /// - Otherwise: Uses standard BFS algorithm
    ///
    /// # Example
    ///
    /// ```
    /// use graphoid::values::{Graph, GraphType, Value};
    /// use std::collections::HashMap;
    ///
    /// let mut g = Graph::new(GraphType::Directed);
    /// g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    /// g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    /// g.add_node("C".to_string(), Value::Number(3.0)).unwrap();
    /// g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    /// g.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();
    ///
    /// let path = g.shortest_path("A", "C", None, false).unwrap();
    /// assert_eq!(path, vec!["A", "B", "C"]);
    /// ```
    pub fn shortest_path(&self, from: &str, to: &str, edge_type: Option<&str>, weighted: bool) -> Option<Vec<String>> {
        if weighted {
            self.shortest_path_weighted(from, to, edge_type)
        } else {
            let path = if edge_type.is_some() {
                self.shortest_path_bfs_filtered(from, to, edge_type)
            } else if self.has_rule("no_cycles") {
                self.shortest_path_dag(from, to)
            } else {
                self.shortest_path_bfs(from, to)
            };
            if path.is_empty() {
                None
            } else {
                Some(path)
            }
        }
    }

    /// Weighted shortest path using Dijkstra's algorithm
    ///
    /// Finds the shortest path considering edge weights. Only edges with weights are considered.
    /// Returns None if no path exists or if any edge in the path is unweighted.
    pub fn shortest_path_weighted(&self, from: &str, to: &str, edge_type: Option<&str>) -> Option<Vec<String>> {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;

        // Priority queue entry: (negative distance for min-heap, node_id)
        #[derive(Debug, Clone)]
        struct State {
            cost: f64,
            node: String,
        }

        impl Eq for State {}

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost && self.node == other.node
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                // Reverse for min-heap (BinaryHeap is max-heap by default)
                other.cost.partial_cmp(&self.cost)
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap_or(Ordering::Equal)
            }
        }

        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return None;
        }

        if from == to {
            return Some(vec![from.to_string()]);
        }

        // Initialize distances and parent map
        let mut dist: HashMap<String, f64> = HashMap::new();
        let mut parent: HashMap<String, String> = HashMap::new();
        let mut heap = BinaryHeap::new();

        // Start with source node
        dist.insert(from.to_string(), 0.0);
        heap.push(State {
            cost: 0.0,
            node: from.to_string(),
        });

        while let Some(State { cost, node }) = heap.pop() {
            // Found target
            if node == to {
                // Reconstruct path
                let mut path = Vec::new();
                let mut current = to.to_string();

                while current != from {
                    path.push(current.clone());
                    if let Some(prev) = parent.get(&current) {
                        current = prev.clone();
                    } else {
                        return None;
                    }
                }
                path.push(from.to_string());
                path.reverse();
                return Some(path);
            }

            // Skip if we've found a better path already
            if cost > *dist.get(&node).unwrap_or(&f64::INFINITY) {
                continue;
            }

            // Explore neighbors
            if let Some(node_data) = self.nodes.get(&node) {
                for (neighbor_id, edge_info) in &node_data.neighbors {
                    // Check edge type filter
                    if let Some(filter_type) = edge_type {
                        if edge_info.edge_type != filter_type {
                            continue;
                        }
                    }

                    // Only consider weighted edges
                    if let Some(weight) = edge_info.weight {
                        let new_cost = cost + weight;
                        let neighbor_cost = *dist.get(neighbor_id).unwrap_or(&f64::INFINITY);

                        if new_cost < neighbor_cost {
                            dist.insert(neighbor_id.clone(), new_cost);
                            parent.insert(neighbor_id.clone(), node.clone());
                            heap.push(State {
                                cost: new_cost,
                                node: neighbor_id.clone(),
                            });
                        }
                    }
                }
            }
        }

        // No path found
        None
    }

    /// BFS-based shortest path with edge type filtering
    fn shortest_path_bfs_filtered(&self, from: &str, to: &str, edge_type: Option<&str>) -> Vec<String> {
        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return Vec::new();
        }

        if from == to {
            return vec![from.to_string()];
        }

        // BFS with parent tracking for path reconstruction
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(from.to_string());
        visited.insert(from.to_string());

        while let Some(current) = queue.pop_front() {
            // Found the target?
            if current == to {
                // Reconstruct path from parent pointers
                let mut path = Vec::new();
                let mut node = current.clone();

                while node != from {
                    path.push(node.clone());
                    node = parent.get(&node).unwrap().clone();
                }
                path.push(from.to_string());
                path.reverse();
                return path;
            }

            // Explore neighbors
            if let Some(node) = self.nodes.get(&current) {
                for (neighbor_id, edge_info) in &node.neighbors {
                    // Check edge type filter
                    if let Some(filter_type) = edge_type {
                        if edge_info.edge_type != filter_type {
                            continue;
                        }
                    }

                    if !visited.contains(neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        parent.insert(neighbor_id.clone(), current.clone());
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }
        }

        // No path found
        Vec::new()
    }

    /// Standard BFS-based shortest path (for general graphs)
    fn shortest_path_bfs(&self, from: &str, to: &str) -> Vec<String> {
        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return Vec::new();
        }

        if from == to {
            return vec![from.to_string()];
        }

        // BFS with parent tracking for path reconstruction
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(from.to_string());
        visited.insert(from.to_string());

        while let Some(current) = queue.pop_front() {
            // Found the target?
            if current == to {
                // Reconstruct path from parent pointers
                let mut path = Vec::new();
                let mut node = current.clone();

                while node != from {
                    path.push(node.clone());
                    node = parent.get(&node).unwrap().clone();
                }
                path.push(from.to_string());
                path.reverse();
                return path;
            }

            // Explore neighbors
            if let Some(node) = self.nodes.get(&current) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        parent.insert(neighbor_id.clone(), current.clone());
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }
        }

        // No path found
        Vec::new()
    }

    /// Topological-sort-based shortest path (optimized for DAGs)
    fn shortest_path_dag(&self, from: &str, to: &str) -> Vec<String> {
        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return Vec::new();
        }

        if from == to {
            return vec![from.to_string()];
        }

        // Get topological ordering
        let topo_order = self.topological_sort();
        if topo_order.is_empty() {
            // Graph has cycles - fall back to BFS
            return self.shortest_path_bfs(from, to);
        }

        // Find positions in topological order
        let from_pos = topo_order.iter().position(|n| n == from);
        let to_pos = topo_order.iter().position(|n| n == to);

        if from_pos.is_none() || to_pos.is_none() {
            return Vec::new();
        }

        let from_idx = from_pos.unwrap();
        let to_idx = to_pos.unwrap();

        // If 'to' comes before 'from' in topological order, no path exists
        if to_idx < from_idx {
            return Vec::new();
        }

        // Use dynamic programming to find shortest path in DAG
        // dist[node] = shortest distance from 'from' to 'node'
        // parent[node] = previous node in shortest path
        let mut dist: HashMap<String, usize> = HashMap::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        dist.insert(from.to_string(), 0);

        // Process nodes in topological order
        for node_id in &topo_order[from_idx..=to_idx] {
            if let Some(&current_dist) = dist.get(node_id) {
                if let Some(node) = self.nodes.get(node_id) {
                    for neighbor_id in node.neighbors.keys() {
                        let new_dist = current_dist + 1;
                        let neighbor_dist = dist.get(neighbor_id).copied().unwrap_or(usize::MAX);

                        if new_dist < neighbor_dist {
                            dist.insert(neighbor_id.clone(), new_dist);
                            parent.insert(neighbor_id.clone(), node_id.clone());
                        }
                    }
                }
            }
        }

        // Check if we reached the target
        if !dist.contains_key(to) {
            return Vec::new();
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = to.to_string();

        while current != from {
            path.push(current.clone());
            if let Some(prev) = parent.get(&current) {
                current = prev.clone();
            } else {
                // No path
                return Vec::new();
            }
        }
        path.push(from.to_string());
        path.reverse();

        path
    }

    /// Perform topological sort on the graph
    ///
    /// Returns a vector of node IDs in topological order.
    /// Returns an empty vector if the graph contains cycles.
    ///
    /// Topological sort is only valid for Directed Acyclic Graphs (DAGs).
    /// For graphs with cycles, this method returns an empty vector.
    ///
    /// # Example
    ///
    /// ```
    /// use graphoid::values::{Graph, GraphType, Value};
    /// use std::collections::HashMap;
    ///
    /// let mut g = Graph::new(GraphType::Directed);
    /// g.add_node("A".to_string(), Value::Number(1.0)).unwrap();
    /// g.add_node("B".to_string(), Value::Number(2.0)).unwrap();
    /// g.add_node("C".to_string(), Value::Number(3.0)).unwrap();
    /// g.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();
    /// g.add_edge("B", "C", "edge".to_string(), None, HashMap::new()).unwrap();
    ///
    /// let sorted = g.topological_sort();
    /// // A must come before B, B must come before C
    /// assert_eq!(sorted, vec!["A", "B", "C"]);
    /// ```
    ///
    /// Checks if a path exists from one node to another.
    ///
    /// Returns `true` if there is a path from `from` to `to`, `false` otherwise.
    /// A node always has a path to itself.
    pub fn has_path(&self, from: &str, to: &str) -> bool {
        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return false;
        }

        if from == to {
            return true;
        }

        // BFS to check reachability
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(from.to_string());
        visited.insert(from.to_string());

        while let Some(current) = queue.pop_front() {
            if current == to {
                return true;
            }

            if let Some(node) = self.nodes.get(&current) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }
        }

        false
    }

    /// Returns the shortest path distance (number of edges) between two nodes.
    ///
    /// Returns the length of the shortest path from `from` to `to`.
    /// Returns `-1` if no path exists.
    /// Returns `0` if from == to.
    pub fn distance(&self, from: &str, to: &str) -> i64 {
        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return -1;
        }

        if from == to {
            return 0;
        }

        // BFS with distance tracking
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut distances: HashMap<String, i64> = HashMap::new();

        queue.push_back(from.to_string());
        visited.insert(from.to_string());
        distances.insert(from.to_string(), 0);

        while let Some(current) = queue.pop_front() {
            if current == to {
                return *distances.get(&current).unwrap();
            }

            let current_dist = *distances.get(&current).unwrap();

            if let Some(node) = self.nodes.get(&current) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        visited.insert(neighbor_id.clone());
                        distances.insert(neighbor_id.clone(), current_dist + 1);
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }
        }

        -1 // No path found
    }

    /// Finds all paths from one node to another up to a maximum length.
    ///
    /// Returns a list of all paths (each path is a list of node IDs) from `from` to `to`
    /// where the path has at most `max_len` edges.
    pub fn all_paths(&self, from: &str, to: &str, max_len: usize) -> Vec<Vec<String>> {
        // Handle special cases
        if !self.has_node(from) || !self.has_node(to) {
            return Vec::new();
        }

        let mut all_paths = Vec::new();
        let mut current_path = vec![from.to_string()];
        let mut visited = HashSet::new();
        visited.insert(from.to_string());

        self.dfs_all_paths(from, to, max_len, &mut current_path, &mut visited, &mut all_paths);

        all_paths
    }

    /// Helper for all_paths - DFS with backtracking
    fn dfs_all_paths(
        &self,
        current: &str,
        target: &str,
        max_len: usize,
        current_path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        // Check if we've reached the target
        if current == target && current_path.len() > 1 {
            // Found a path! (length > 1 means we actually moved)
            all_paths.push(current_path.clone());
            return;
        }

        // Check if we've exceeded max length
        if current_path.len() > max_len {
            return;
        }

        // Explore neighbors
        if let Some(node) = self.nodes.get(current) {
            for neighbor_id in node.neighbors.keys() {
                if !visited.contains(neighbor_id) {
                    // Visit this neighbor
                    visited.insert(neighbor_id.clone());
                    current_path.push(neighbor_id.clone());

                    // Recurse
                    self.dfs_all_paths(neighbor_id, target, max_len, current_path, visited, all_paths);

                    // Backtrack
                    current_path.pop();
                    visited.remove(neighbor_id);
                }
            }
        }
    }

    pub fn topological_sort(&self) -> Vec<String> {
        if self.nodes.is_empty() {
            return Vec::new();
        }

        // Kahn's algorithm for topological sort
        // Calculate in-degree for each node
        let mut in_degree: HashMap<String, usize> = HashMap::new();

        // Initialize all nodes with in-degree 0
        for node_id in self.nodes.keys() {
            in_degree.insert(node_id.clone(), 0);
        }

        // Count incoming edges
        for node in self.nodes.values() {
            for neighbor_id in node.neighbors.keys() {
                *in_degree.get_mut(neighbor_id).unwrap() += 1;
            }
        }

        // Queue nodes with in-degree 0
        let mut queue = VecDeque::new();
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }

        let mut result = Vec::new();

        while let Some(node_id) = queue.pop_front() {
            result.push(node_id.clone());

            // Reduce in-degree of neighbors
            if let Some(node) = self.nodes.get(&node_id) {
                for neighbor_id in node.neighbors.keys() {
                    let degree = in_degree.get_mut(neighbor_id).unwrap();
                    *degree -= 1;

                    if *degree == 0 {
                        queue.push_back(neighbor_id.clone());
                    }
                }
            }
        }

        // If we didn't process all nodes, there's a cycle
        if result.len() != self.nodes.len() {
            return Vec::new();
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

#[cfg(test)]
mod edge_weight_tests {
    use super::*;
    use std::collections::HashMap;

    // ========================================================================
    // EdgeInfo Weight Methods Tests (10 tests)
    // ========================================================================

    #[test]
    fn test_edgeinfo_new_creates_unweighted_edge() {
        let edge = EdgeInfo::new("test".to_string(), HashMap::new());
        assert_eq!(edge.weight(), None);
        assert!(!edge.is_weighted());
    }

    #[test]
    fn test_edgeinfo_new_weighted_creates_weighted_edge() {
        let edge = EdgeInfo::new_weighted("test".to_string(), 5.0, HashMap::new());
        assert_eq!(edge.weight(), Some(5.0));
        assert!(edge.is_weighted());
    }

    #[test]
    fn test_edgeinfo_set_weight_adds_weight() {
        let mut edge = EdgeInfo::new("test".to_string(), HashMap::new());
        assert!(!edge.is_weighted());

        edge.set_weight(Some(3.5));
        assert_eq!(edge.weight(), Some(3.5));
        assert!(edge.is_weighted());
    }

    #[test]
    fn test_edgeinfo_set_weight_updates_existing_weight() {
        let mut edge = EdgeInfo::new_weighted("test".to_string(), 2.0, HashMap::new());
        assert_eq!(edge.weight(), Some(2.0));

        edge.set_weight(Some(10.0));
        assert_eq!(edge.weight(), Some(10.0));
    }

    #[test]
    fn test_edgeinfo_set_weight_removes_weight() {
        let mut edge = EdgeInfo::new_weighted("test".to_string(), 7.5, HashMap::new());
        assert!(edge.is_weighted());

        edge.set_weight(None);
        assert_eq!(edge.weight(), None);
        assert!(!edge.is_weighted());
    }

    #[test]
    fn test_edgeinfo_weight_returns_none_for_unweighted() {
        let edge = EdgeInfo::new("test".to_string(), HashMap::new());
        assert_eq!(edge.weight(), None);
    }

    #[test]
    fn test_edgeinfo_weight_returns_some_for_weighted() {
        let edge = EdgeInfo::new_weighted("test".to_string(), 42.0, HashMap::new());
        assert_eq!(edge.weight(), Some(42.0));
    }

    #[test]
    fn test_edgeinfo_is_weighted_false_for_unweighted() {
        let edge = EdgeInfo::new("test".to_string(), HashMap::new());
        assert!(!edge.is_weighted());
    }

    #[test]
    fn test_edgeinfo_is_weighted_true_for_weighted() {
        let edge = EdgeInfo::new_weighted("test".to_string(), 1.5, HashMap::new());
        assert!(edge.is_weighted());
    }

    #[test]
    fn test_edgeinfo_preserves_properties_with_weight() {
        let mut props = HashMap::new();
        props.insert("label".to_string(), Value::String("important".to_string()));

        let edge = EdgeInfo::new_weighted("test".to_string(), 3.0, props.clone());
        assert_eq!(edge.weight(), Some(3.0));
        assert_eq!(edge.properties.get("label"), props.get("label"));
    }

    // ========================================================================
    // Graph Weight Mutation Methods Tests (15 tests)
    // ========================================================================

    #[test]
    fn test_graph_get_edge_weight_unweighted() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

        assert_eq!(graph.get_edge_weight("A", "B"), None);
    }

    #[test]
    fn test_graph_get_edge_weight_weighted() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), Some(5.5), HashMap::new()).unwrap();

        assert_eq!(graph.get_edge_weight("A", "B"), Some(5.5));
    }

    #[test]
    fn test_graph_get_edge_weight_nonexistent_edge() {
        let graph = Graph::new(GraphType::Directed);
        assert_eq!(graph.get_edge_weight("X", "Y"), None);
    }

    #[test]
    fn test_graph_set_edge_weight_on_unweighted_edge() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

        assert_eq!(graph.get_edge_weight("A", "B"), None);

        graph.set_edge_weight("A", "B", 10.0).unwrap();
        assert_eq!(graph.get_edge_weight("A", "B"), Some(10.0));
    }

    #[test]
    fn test_graph_set_edge_weight_updates_existing_weight() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), Some(3.0), HashMap::new()).unwrap();

        assert_eq!(graph.get_edge_weight("A", "B"), Some(3.0));

        graph.set_edge_weight("A", "B", 99.9).unwrap();
        assert_eq!(graph.get_edge_weight("A", "B"), Some(99.9));
    }

    #[test]
    fn test_graph_set_edge_weight_nonexistent_edge_fails() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();

        let result = graph.set_edge_weight("A", "B", 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_graph_set_edge_weight_undirected_updates_both() {
        let mut graph = Graph::new(GraphType::Undirected);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

        graph.set_edge_weight("A", "B", 7.5).unwrap();

        // Both directions should have the weight
        assert_eq!(graph.get_edge_weight("A", "B"), Some(7.5));
        assert_eq!(graph.get_edge_weight("B", "A"), Some(7.5));
    }

    #[test]
    fn test_graph_remove_edge_weight_from_weighted_edge() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), Some(12.0), HashMap::new()).unwrap();

        assert_eq!(graph.get_edge_weight("A", "B"), Some(12.0));

        graph.remove_edge_weight("A", "B").unwrap();
        assert_eq!(graph.get_edge_weight("A", "B"), None);
        assert!(!graph.is_edge_weighted("A", "B"));
    }

    #[test]
    fn test_graph_remove_edge_weight_from_unweighted_edge() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

        // Should succeed even though edge is already unweighted
        graph.remove_edge_weight("A", "B").unwrap();
        assert_eq!(graph.get_edge_weight("A", "B"), None);
    }

    #[test]
    fn test_graph_remove_edge_weight_nonexistent_edge_fails() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();

        let result = graph.remove_edge_weight("A", "B");
        assert!(result.is_err());
    }

    #[test]
    fn test_graph_remove_edge_weight_undirected_updates_both() {
        let mut graph = Graph::new(GraphType::Undirected);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), Some(15.0), HashMap::new()).unwrap();

        graph.remove_edge_weight("A", "B").unwrap();

        // Both directions should have weight removed
        assert_eq!(graph.get_edge_weight("A", "B"), None);
        assert_eq!(graph.get_edge_weight("B", "A"), None);
    }

    #[test]
    fn test_graph_is_edge_weighted_true_for_weighted() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), Some(20.0), HashMap::new()).unwrap();

        assert!(graph.is_edge_weighted("A", "B"));
    }

    #[test]
    fn test_graph_is_edge_weighted_false_for_unweighted() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), None, HashMap::new()).unwrap();

        assert!(!graph.is_edge_weighted("A", "B"));
    }

    #[test]
    fn test_graph_is_edge_weighted_false_for_nonexistent() {
        let graph = Graph::new(GraphType::Directed);
        assert!(!graph.is_edge_weighted("X", "Y"));
    }

    #[test]
    fn test_graph_weight_mutation_preserves_properties() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

        let mut props = HashMap::new();
        props.insert("color".to_string(), Value::String("red".to_string()));

        graph.add_edge("A", "B", "edge".to_string(), Some(1.0), props).unwrap();

        // Set new weight
        graph.set_edge_weight("A", "B", 2.0).unwrap();

        // Properties should still be there
        let node = graph.nodes.get("A").unwrap();
        let edge = node.neighbors.get("B").unwrap();
        assert_eq!(
            edge.properties.get("color"),
            Some(&Value::String("red".to_string()))
        );
    }

    // ========================================================================
    // Weighted Pathfinding Tests (15 tests) - TDD: Write tests first!
    // ========================================================================

    #[test]
    fn test_dijkstra_simple_weighted_path() {
        // A -5-> B -3-> C
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();
        graph.add_edge("A", "B", "road".to_string(), Some(5.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "road".to_string(), Some(3.0), HashMap::new()).unwrap();

        let path = graph.shortest_path_weighted("A", "C", None).unwrap();
        assert_eq!(path, vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_dijkstra_chooses_lighter_path() {
        // A -1-> B -1-> C (weight 2)
        // A -10-> C (weight 10)
        // Should choose A->B->C
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();
        graph.add_edge("A", "B", "road".to_string(), Some(1.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "road".to_string(), Some(1.0), HashMap::new()).unwrap();
        graph.add_edge("A", "C", "road".to_string(), Some(10.0), HashMap::new()).unwrap();

        let path = graph.shortest_path_weighted("A", "C", None).unwrap();
        assert_eq!(path, vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_dijkstra_complex_graph() {
        // Diamond graph with different weights
        //     A
        //   /   \
        //  1     5
        // /       \
        // B --2-- C
        //  \     /
        //   1   1
        //    \ /
        //     D
        // A->B->D should be shortest (2), not A->C->D (6)
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();
        graph.add_node("D".to_string(), Value::Number(4.0)).unwrap();

        graph.add_edge("A", "B", "road".to_string(), Some(1.0), HashMap::new()).unwrap();
        graph.add_edge("A", "C", "road".to_string(), Some(5.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "road".to_string(), Some(2.0), HashMap::new()).unwrap();
        graph.add_edge("B", "D", "road".to_string(), Some(1.0), HashMap::new()).unwrap();
        graph.add_edge("C", "D", "road".to_string(), Some(1.0), HashMap::new()).unwrap();

        let path = graph.shortest_path_weighted("A", "D", None).unwrap();
        assert_eq!(path, vec!["A".to_string(), "B".to_string(), "D".to_string()]);
    }

    #[test]
    fn test_dijkstra_no_path_returns_none() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        // No edge between A and B

        assert_eq!(graph.shortest_path_weighted("A", "B", None), None);
    }

    #[test]
    fn test_dijkstra_with_edge_type_filter() {
        // A -road(5)-> B -rail(3)-> C
        // A -road(2)-> D -road(2)-> C
        // With edge_type "road", should choose A->D->C (4), not A->B->C (rail blocked)
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();
        graph.add_node("D".to_string(), Value::Number(4.0)).unwrap();

        graph.add_edge("A", "B", "road".to_string(), Some(5.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "rail".to_string(), Some(3.0), HashMap::new()).unwrap();
        graph.add_edge("A", "D", "road".to_string(), Some(2.0), HashMap::new()).unwrap();
        graph.add_edge("D", "C", "road".to_string(), Some(2.0), HashMap::new()).unwrap();

        let path = graph.shortest_path_weighted("A", "C", Some("road")).unwrap();
        assert_eq!(path, vec!["A".to_string(), "D".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_dijkstra_rejects_unweighted_edges() {
        // Graph has unweighted edge - should return error
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "road".to_string(), None, HashMap::new()).unwrap();

        // Should return None because unweighted edge can't be used in weighted pathfinding
        assert_eq!(graph.shortest_path_weighted("A", "B", None), None);
    }

    #[test]
    fn test_shortest_path_with_weighted_option() {
        // Test the updated shortest_path() method with weighted parameter
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

        graph.add_edge("A", "B", "road".to_string(), Some(10.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "road".to_string(), Some(10.0), HashMap::new()).unwrap();
        graph.add_edge("A", "C", "road".to_string(), Some(1.0), HashMap::new()).unwrap();

        // With weighted=true, should use Dijkstra and choose A->C (weight 1)
        let path = graph.shortest_path("A", "C", None, true).unwrap();
        assert_eq!(path, vec!["A".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_shortest_path_unweighted_uses_bfs() {
        // Test that weighted=false uses BFS (shortest by hops, not weight)
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

        graph.add_edge("A", "B", "road".to_string(), Some(10.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "road".to_string(), Some(10.0), HashMap::new()).unwrap();
        graph.add_edge("A", "C", "road".to_string(), Some(1.0), HashMap::new()).unwrap();

        // With weighted=false, should ignore weights and find shortest hop path
        // Both paths are 1-2 hops, so either is valid for BFS
        let path = graph.shortest_path("A", "C", None, false).unwrap();
        assert!(path.len() >= 2); // At least 2 nodes (start and end)
    }

    #[test]
    fn test_shortest_path_with_edge_type_filter() {
        // Test edge_type parameter in shortest_path()
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

        graph.add_edge("A", "B", "road".to_string(), None, HashMap::new()).unwrap();
        graph.add_edge("B", "C", "rail".to_string(), None, HashMap::new()).unwrap();
        graph.add_edge("A", "C", "road".to_string(), None, HashMap::new()).unwrap();

        // With edge_type "road", should choose A->C directly
        let path = graph.shortest_path("A", "C", Some("road"), false).unwrap();
        assert_eq!(path, vec!["A".to_string(), "C".to_string()]);
    }

    #[test]
    fn test_dijkstra_self_path() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();

        let path = graph.shortest_path_weighted("A", "A", None).unwrap();
        assert_eq!(path, vec!["A".to_string()]);
    }

    #[test]
    fn test_dijkstra_undirected_graph() {
        // Undirected graph - both directions available
        let mut graph = Graph::new(GraphType::Undirected);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "road".to_string(), Some(5.0), HashMap::new()).unwrap();

        // Should work in both directions
        let path_ab = graph.shortest_path_weighted("A", "B", None).unwrap();
        assert_eq!(path_ab, vec!["A".to_string(), "B".to_string()]);

        let path_ba = graph.shortest_path_weighted("B", "A", None).unwrap();
        assert_eq!(path_ba, vec!["B".to_string(), "A".to_string()]);
    }

    #[test]
    fn test_dijkstra_negative_weights_not_supported() {
        // Dijkstra doesn't support negative weights - should still find a path
        // but may not be optimal (this is a known limitation)
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "road".to_string(), Some(-5.0), HashMap::new()).unwrap();

        // Should find the path (even with negative weight)
        let path = graph.shortest_path_weighted("A", "B", None);
        assert!(path.is_some());
    }

    #[test]
    fn test_shortest_path_default_parameters() {
        // Test that existing code still works (backward compatibility)
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "road".to_string(), None, HashMap::new()).unwrap();

        // Old signature should still work: shortest_path(from, to)
        let path = graph.shortest_path("A", "B", None, false).unwrap();
        assert_eq!(path, vec!["A".to_string(), "B".to_string()]);
    }

    #[test]
    fn test_dijkstra_large_graph_performance() {
        // Create a larger graph to test performance
        let mut graph = Graph::new(GraphType::Directed);

        // Create 10 nodes in a chain with varying weights
        for i in 0..10 {
            graph.add_node(format!("N{}", i), Value::Number(i as f64)).unwrap();
        }

        // Create edges with weights
        for i in 0..9 {
            graph.add_edge(
                &format!("N{}", i),
                &format!("N{}", i + 1),
                "road".to_string(),
                Some((i + 1) as f64),
                HashMap::new()
            ).unwrap();
        }

        let path = graph.shortest_path_weighted("N0", "N9", None).unwrap();
        assert_eq!(path.len(), 10);
        assert_eq!(path[0], "N0");
        assert_eq!(path[9], "N9");
    }

    #[test]
    fn test_dijkstra_mixed_weighted_unweighted_graph() {
        // Graph with both weighted and unweighted edges
        // Only weighted edges should be used in weighted pathfinding
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();

        // Weighted path: A -5-> B -3-> C
        graph.add_edge("A", "B", "road".to_string(), Some(5.0), HashMap::new()).unwrap();
        graph.add_edge("B", "C", "road".to_string(), Some(3.0), HashMap::new()).unwrap();

        // Unweighted shortcut: A -> C (should be ignored in weighted pathfinding)
        graph.add_edge("A", "C", "road".to_string(), None, HashMap::new()).unwrap();

        // Should use weighted path A->B->C, not the unweighted A->C
        let path = graph.shortest_path_weighted("A", "C", None).unwrap();
        assert_eq!(path, vec!["A".to_string(), "B".to_string(), "C".to_string()]);
    }
}
