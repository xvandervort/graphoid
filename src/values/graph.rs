//! Graph data structure implementation
//!
//! Graphoid's graph type uses index-free adjacency for O(1) neighbor lookups.
//! Each node stores direct pointers to its neighbors, avoiding index scans.

use std::collections::{HashMap, HashSet, VecDeque};
use super::{Value, ValueKind, PatternNode, PatternEdge, PatternPath, Function, List};
use crate::graph::rules::{Rule, RuleContext, GraphOperation, RuleSpec, RuleInstance, RuleSeverity};
use crate::graph::rulesets::get_ruleset_rules;
use crate::error::GraphoidError;

/// Enum to represent either a fixed edge or variable-length path in pattern matching
#[derive(Debug, Clone)]
enum EdgeOrPath {
    Edge(PatternEdge),
    Path(PatternPath),
}

/// Type of graph: directed or undirected
#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    Directed,
    Undirected,
}

/// Policy for handling orphaned nodes (nodes with no edges)
#[derive(Debug, Clone, PartialEq)]
pub enum OrphanPolicy {
    /// Orphans can exist - no special handling
    Allow,
    /// Reject operations that would create orphans
    Reject,
    /// Automatically delete all orphans after operation
    Delete,
    /// Automatically reconnect orphans using strategy
    Reconnect,
}

/// Strategy for reconnecting orphaned nodes
#[derive(Debug, Clone, PartialEq)]
pub enum ReconnectStrategy {
    /// Connect orphans to the root node
    ToRoot,
    /// Connect orphans to siblings of their deleted parent
    ToParentSiblings,
}

/// Configuration for graph behavior
#[derive(Debug, Clone, PartialEq)]
pub struct GraphConfig {
    /// How to handle orphaned nodes
    pub orphan_policy: OrphanPolicy,
    /// Strategy for reconnecting orphans (if policy is Reconnect)
    pub reconnect_strategy: Option<ReconnectStrategy>,
    /// Whether operations can override graph configuration
    pub allow_overrides: bool,
}

impl Default for GraphConfig {
    fn default() -> Self {
        GraphConfig {
            orphan_policy: OrphanPolicy::Allow,
            reconnect_strategy: None,
            allow_overrides: false,
        }
    }
}

/// A node in the graph
#[derive(Debug, Clone, PartialEq)]
pub struct GraphNode {
    /// Node identifier
    pub id: String,
    /// Node value
    pub value: Value,
    /// Node type (optional label like "User", "Product", etc.)
    pub node_type: Option<String>,
    /// Node properties (for property-based indexing)
    pub properties: HashMap<String, Value>,
    /// Outgoing edges (neighbor_id -> edge_info)
    pub neighbors: HashMap<String, EdgeInfo>,
    /// Incoming edges (predecessor_id -> edge_info)
    /// Maintained automatically when edges are added/removed
    pub predecessors: HashMap<String, EdgeInfo>,
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
    /// Graph configuration (orphan policies, etc.)
    pub config: GraphConfig,
    /// Nodes by ID for O(1) lookup
    pub nodes: HashMap<String, GraphNode>,
    /// Active rulesets (e.g., "tree", "dag", "bst")
    /// Predefined bundles of rules applied via with_ruleset()
    pub rulesets: Vec<String>,
    /// Ad hoc rules added via add_rule()
    /// These are in addition to any ruleset rules
    /// Each rule includes its configured severity
    pub rules: Vec<RuleInstance>,

    /// Parent graph reference for inheritance (Phase 14)
    /// Used for super calls and is_a() checks
    pub parent: Option<Box<Graph>>,

    /// Type name for is_a() and type_of() checks (Phase 18)
    /// Set when graph is assigned to a variable: `Dog = graph{}`
    pub type_name: Option<String>,

    // Freeze state (not included in PartialEq)
    /// Whether this graph (and by extension, List/Hash backed by it) is frozen
    /// Frozen graphs cannot be modified
    frozen: bool,

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
    // Note: Methods are stored as nodes with node_type "__method__"
    // This follows Graphoid's "everything is a graph" principle
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
            config: GraphConfig::default(),
            nodes: HashMap::new(),
            rulesets: Vec::new(),
            rules: Vec::new(),
            parent: None,
            type_name: None,  // Phase 18: set on assignment
            // Freeze state
            frozen: false,
            // Auto-optimization state
            property_access_counts: HashMap::new(),
            property_indices: HashMap::new(),
            auto_index_threshold: 10, // Create index after 10 lookups
            // Methods are stored as nodes with node_type "__method__"
        }
    }

    /// Create a new graph that inherits from a parent
    pub fn from_parent(parent: Graph) -> Self {
        let mut child = parent.clone();

        // Store parent's type name before we lose it
        let parent_type_name = parent.type_name.clone();

        child.parent = Some(Box::new(parent));
        // Phase 18: Reset type_name so the child gets its own type when assigned
        // The parent's type_name is preserved in the parent field for is_a() checks
        child.type_name = None;

        // Phase 3: Add __parent__ node and inherits_from edge for graph-based inheritance
        if let Some(parent_name) = parent_type_name {
            // Create __parent__ node storing reference to parent graph name
            child.nodes.insert("__parent__".to_string(), GraphNode {
                id: "__parent__".to_string(),
                value: Value::string(parent_name.clone()),
                node_type: Some("parent_reference".to_string()),
                properties: HashMap::new(),
                neighbors: HashMap::new(),
                predecessors: HashMap::new(),
            });

            // Create inherits_from edge from child's type node to __parent__
            // We'll create a virtual root node representing the child graph itself
            if !child.nodes.contains_key("__self__") {
                child.nodes.insert("__self__".to_string(), GraphNode {
                    id: "__self__".to_string(),
                    value: Value::string("self".to_string()),
                    node_type: Some("graph_self".to_string()),
                    properties: HashMap::new(),
                    neighbors: HashMap::new(),
                    predecessors: HashMap::new(),
                });
            }

            // Add inherits_from edge from __self__ to __parent__
            if let Some(self_node) = child.nodes.get_mut("__self__") {
                self_node.neighbors.insert("__parent__".to_string(), EdgeInfo {
                    edge_type: "inherits_from".to_string(),
                    weight: None,
                    properties: HashMap::new(),
                });
            }
            if let Some(parent_node) = child.nodes.get_mut("__parent__") {
                parent_node.predecessors.insert("__self__".to_string(), EdgeInfo {
                    edge_type: "inherits_from".to_string(),
                    weight: None,
                    properties: HashMap::new(),
                });
            }
        }

        child
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

    /// Add a node to the graph, or update an existing node's value (preserving edges)
    pub fn add_node(&mut self, id: String, value: Value) -> Result<(), GraphoidError> {
        // Check if graph is frozen
        if self.frozen {
            return Err(GraphoidError::runtime(
                "Cannot modify frozen graph".to_string()
            ));
        }

        // Validate the operation against active rules
        let operation = GraphOperation::AddNode {
            id: id.clone(),
            value: value.clone(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                // If node exists, update value while preserving edges and properties
                if let Some(existing) = self.nodes.get_mut(&id) {
                    existing.value = value;
                } else {
                    // New node - create fresh
                    self.nodes.insert(
                        id.clone(),
                        GraphNode {
                            id,
                            value,
                            node_type: None,
                            properties: HashMap::new(),
                            neighbors: HashMap::new(),
                            predecessors: HashMap::new(),
                        },
                    );
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

    /// Set the type of a node (e.g., "User", "Product")
    pub fn set_node_type(&mut self, id: &str, node_type: String) -> Result<(), GraphoidError> {
        let node = self.nodes.get_mut(id).ok_or_else(|| {
            GraphoidError::runtime(format!("Node '{}' not found", id))
        })?;

        node.node_type = Some(node_type);
        Ok(())
    }

    /// Get the type of a node
    pub fn get_node_type(&self, id: &str) -> Option<String> {
        self.nodes.get(id).and_then(|node| node.node_type.clone())
    }

    /// Set properties for a node (replaces existing properties)
    pub fn set_node_properties(&mut self, id: &str, properties: HashMap<String, Value>) -> Result<(), GraphoidError> {
        let node = self.nodes.get_mut(id).ok_or_else(|| {
            GraphoidError::runtime(format!("Node '{}' not found", id))
        })?;

        node.properties = properties;
        Ok(())
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: &str, to: &str, edge_type: String, weight: Option<f64>, properties: HashMap<String, Value>) -> Result<(), GraphoidError> {
        // Check if graph is frozen
        if self.frozen {
            return Err(GraphoidError::runtime(
                "Cannot modify frozen graph".to_string()
            ));
        }

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

                // Add forward edge (from -> to)
                if let Some(from_node) = self.nodes.get_mut(from) {
                    from_node.neighbors.insert(
                        to.to_string(),
                        edge_info.clone(),
                    );
                }

                // Add reverse index (to <- from)
                if let Some(to_node) = self.nodes.get_mut(to) {
                    to_node.predecessors.insert(
                        from.to_string(),
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

                    // Add reverse edge (to -> from) for undirected graphs
                    if let Some(to_node) = self.nodes.get_mut(to) {
                        to_node.neighbors.insert(
                            from.to_string(),
                            reverse_edge_info.clone(),
                        );
                    }

                    // Add reverse predecessor (from <- to) for undirected graphs
                    if let Some(from_node) = self.nodes.get_mut(from) {
                        from_node.predecessors.insert(
                            to.to_string(),
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
        // Check if graph is frozen
        if self.frozen {
            return Err(GraphoidError::runtime(
                "Cannot modify frozen graph".to_string()
            ));
        }

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

    /// Get data node count (excludes internal nodes like __methods__ branch)
    pub fn node_count(&self) -> usize {
        self.data_node_ids().len()
    }

    /// Get total node count including internal nodes
    pub fn total_node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get edge count (data edges only)
    pub fn edge_count(&self) -> usize {
        // Count only edges where both endpoints are data nodes
        let data_nodes: std::collections::HashSet<&String> =
            self.nodes.keys()
                .filter(|id| !id.starts_with("__methods__"))
                .collect();

        self.nodes.iter()
            .filter(|(id, _)| !id.starts_with("__methods__"))
            .map(|(_, node)| {
                node.neighbors.iter()
                    .filter(|(to_id, _)| data_nodes.contains(to_id))
                    .count()
            })
            .sum()
    }

    /// Get all node IDs as a list (data nodes only by default)
    pub fn node_ids(&self) -> Vec<String> {
        self.data_node_ids()
    }

    /// Get all node IDs including internal nodes
    pub fn all_node_ids(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }

    /// Get all edges as a list of tuples (from, to, edge_type) - includes internal edges
    pub fn edge_list(&self) -> Vec<(String, String, String)> {
        let mut edges = Vec::new();
        for (from_id, node) in &self.nodes {
            for (to_id, edge_info) in &node.neighbors {
                edges.push((from_id.clone(), to_id.clone(), edge_info.edge_type.clone()));
            }
        }
        edges
    }

    /// Get data edges only (excludes edges involving __methods__ branch)
    pub fn data_edge_list(&self) -> Vec<(String, String, String)> {
        let mut edges = Vec::new();
        for (from_id, node) in &self.nodes {
            // Skip edges from method branch nodes
            if from_id.starts_with("__methods__") {
                continue;
            }
            for (to_id, edge_info) in &node.neighbors {
                // Skip edges to method branch nodes
                if to_id.starts_with("__methods__") {
                    continue;
                }
                edges.push((from_id.clone(), to_id.clone(), edge_info.edge_type.clone()));
            }
        }
        edges
    }

    /// Remove a node from the graph
    /// Remove a node with optional orphan handling policy override
    pub fn remove_node(
        &mut self,
        id: &str,
        orphan_handling: Option<OrphanPolicy>,
    ) -> Result<Option<GraphNode>, GraphoidError> {
        // Determine effective orphan policy
        let effective_policy = if let Some(override_policy) = orphan_handling {
            // Check if overrides are allowed
            if self.config.allow_overrides {
                override_policy
            } else {
                return Err(GraphoidError::runtime(
                    "Orphan policy overrides are not allowed for this graph".to_string()
                ));
            }
        } else {
            self.config.orphan_policy.clone()
        };

        // For Reject policy, check if removal would create orphans BEFORE removing
        if matches!(effective_policy, OrphanPolicy::Reject) {
            let would_be_orphans = self.find_would_be_orphans(id);
            if !would_be_orphans.is_empty() {
                return Err(GraphoidError::runtime(format!(
                    "Cannot remove node '{}': would create {} orphan(s) (policy: reject)",
                    id,
                    would_be_orphans.len()
                )));
            }
        }

        // Perform the actual removal
        let removed = self.remove_node_internal(id)?;

        // Handle orphans based on policy AFTER removal
        match effective_policy {
            OrphanPolicy::Allow => {
                // Do nothing - orphans are allowed
            }
            OrphanPolicy::Reject => {
                // Already checked above
            }
            OrphanPolicy::Delete => {
                // Delete all orphans
                self.delete_orphans()?;
            }
            OrphanPolicy::Reconnect => {
                // Reconnect orphans using the configured strategy
                if let Some(strategy) = &self.config.reconnect_strategy {
                    self.reconnect_orphans(strategy.clone())?;
                } else {
                    return Err(GraphoidError::runtime(
                        "Orphan policy is :reconnect but no reconnect_strategy is configured".to_string()
                    ));
                }
            }
        }

        Ok(removed)
    }

    /// Internal method to remove a node without orphan handling
    /// Used by delete_orphans to avoid infinite recursion
    fn remove_node_internal(&mut self, id: &str) -> Result<Option<GraphNode>, GraphoidError> {
        // Check if graph is frozen
        if self.frozen {
            return Err(GraphoidError::runtime(
                "Cannot modify frozen graph".to_string()
            ));
        }

        // Validate the operation against active rules
        let operation = GraphOperation::RemoveNode {
            id: id.to_string(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                // Remove the node
                let removed = self.nodes.remove(id);

                // Remove all edges pointing to/from this node
                for node in self.nodes.values_mut() {
                    node.neighbors.remove(id);     // Remove outgoing edges to this node
                    node.predecessors.remove(id);  // Remove incoming edges from this node
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
        // Check if graph is frozen
        if self.frozen {
            return Err(GraphoidError::runtime(
                "Cannot modify frozen graph".to_string()
            ));
        }

        // Validate the operation against active rules
        let operation = GraphOperation::RemoveEdge {
            from: from.to_string(),
            to: to.to_string(),
        };

        match self.validate_rules(operation) {
            ValidationResult::Allowed => {
                // All rules passed - perform the operation
                let mut removed = false;

                // Remove forward edge (from -> to)
                if let Some(from_node) = self.nodes.get_mut(from) {
                    removed = from_node.neighbors.remove(to).is_some();
                }

                // Remove reverse index (to <- from)
                if let Some(to_node) = self.nodes.get_mut(to) {
                    to_node.predecessors.remove(from);
                }

                // For undirected graphs, remove reverse edge
                if self.graph_type == GraphType::Undirected {
                    // Remove reverse edge (to -> from)
                    if let Some(to_node) = self.nodes.get_mut(to) {
                        to_node.neighbors.remove(from);
                    }

                    // Remove reverse predecessor (from <- to)
                    if let Some(from_node) = self.nodes.get_mut(from) {
                        from_node.predecessors.remove(to);
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
        self.nodes.get(id).and_then(|n| {
            // Don't return computed property alias nodes - these are for dependency tracking only
            if n.node_type.as_deref() == Some("computed_property_alias") {
                None
            } else {
                Some(&n.value)
            }
        })
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
    /// g.add_node("A".to_string(), Value::number(1.0)).unwrap();
    /// g.add_node("B".to_string(), Value::number(2.0)).unwrap();
    /// g.add_node("C".to_string(), Value::number(3.0)).unwrap();
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
    /// g.add_node("A".to_string(), Value::number(1.0)).unwrap();
    /// g.add_node("B".to_string(), Value::number(2.0)).unwrap();
    /// g.add_node("C".to_string(), Value::number(3.0)).unwrap();
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

    /// Match a pattern in the graph and return all matches as bindings.
    ///
    /// Pattern arguments should be alternating PatternNode and PatternEdge/PatternPath values.
    /// For example: [node("a"), edge(), node("b")] matches a simple two-node pattern.
    ///
    /// Returns a list of binding maps where keys are variable names and values are node IDs.
    pub fn match_pattern(&self, pattern_args: Vec<Value>) -> Result<crate::values::PatternMatchResults, GraphoidError> {
        // Parse pattern arguments into nodes and edges/paths
        let (pattern_nodes, pattern_edges) = {
            let mut nodes = Vec::new();
            let mut edges = Vec::new();
            for (i, arg) in pattern_args.iter().enumerate() {
                match &arg.kind {
                    ValueKind::PatternNode(pn) => nodes.push(pn.clone()),
                    ValueKind::PatternEdge(pe) => edges.push(EdgeOrPath::Edge(pe.clone())),
                    ValueKind::PatternPath(pp) => edges.push(EdgeOrPath::Path(pp.clone())),
                    _ => return Err(GraphoidError::runtime(format!(
                        "Invalid pattern argument at position {}: expected PatternNode, PatternEdge, or PatternPath", i
                    ))),
                }
            }
            (nodes, edges)
        };

        // Handle empty pattern
        if pattern_nodes.is_empty() {
            return Ok(crate::values::PatternMatchResults::new(Vec::new(), self.clone()));
        }

        let mut results = Vec::new();
        let first_var = pattern_nodes[0].variable.as_ref()
            .ok_or_else(|| GraphoidError::runtime("Pattern node must have a variable name".to_string()))?;

        // Find all nodes matching the first pattern node
        for (node_id, _node) in &self.nodes {
            // Check if node matches type constraint
            let matches_type = match &pattern_nodes[0].node_type {
                None => true,
                Some(required_type) => self.get_node_type(node_id) == Some(required_type.clone()),
            };

            if !matches_type {
                continue;
            }

            // Start building a binding with this node
            let mut binding = HashMap::new();
            binding.insert(first_var.clone(), node_id.clone());

            // If no edges, this is a complete match (single node pattern)
            if pattern_edges.is_empty() {
                results.push(binding);
                continue;
            }

            // Try to extend the match following edges (recursive backtracking)
            Self::extend_pattern_match(
                &self.nodes,
                &mut binding,
                node_id,
                &pattern_nodes,
                &pattern_edges,
                0,
                &mut results
            );
        }

        Ok(crate::values::PatternMatchResults::new(results, self.clone()))
    }

    /// Find all paths from start node with length in range [min_len, max_len].
    /// Uses BFS to explore paths level by level.
    fn find_variable_length_paths(
        graph_nodes: &HashMap<String, GraphNode>,
        start_node: &str,
        min_len: usize,
        max_len: usize,
        edge_type: Option<&str>,
        direction: &str
    ) -> Vec<Vec<String>> {
        let mut results = Vec::new();

        // Handle zero-length paths (same node)
        if min_len == 0 {
            results.push(vec![start_node.to_string()]);
        }

        if max_len == 0 {
            return results;
        }

        // Use BFS with path tracking
        let mut queue: Vec<Vec<String>> = vec![vec![start_node.to_string()]];

        while let Some(current_path) = queue.pop() {
            let current_len = current_path.len() - 1; // Path length is number of edges

            if current_len >= max_len {
                continue; // Don't extend beyond max_len
            }

            let current_node = current_path.last().unwrap();
            let graph_node = match graph_nodes.get(current_node) {
                Some(n) => n,
                None => continue,
            };

            // Choose which edges to follow based on direction
            let edges_to_follow: Vec<(&String, &EdgeInfo)> = match direction {
                "incoming" => graph_node.predecessors.iter().collect(),
                "outgoing" => graph_node.neighbors.iter().collect(),
                "both" => {
                    let mut edges: Vec<(&String, &EdgeInfo)> = graph_node.neighbors.iter().collect();
                    edges.extend(graph_node.predecessors.iter());
                    edges
                },
                _ => graph_node.neighbors.iter().collect(),
            };

            for (neighbor_id, edge_info) in edges_to_follow {
                // Check edge type constraint
                if let Some(required_type) = edge_type {
                    if edge_info.edge_type != required_type {
                        continue;
                    }
                }

                // Create new path by extending current path
                let mut new_path = current_path.clone();
                new_path.push(neighbor_id.clone());

                let new_len = new_path.len() - 1;

                // Add to results if within range
                if new_len >= min_len && new_len <= max_len {
                    results.push(new_path.clone());
                }

                // Add to queue for further exploration if not at max
                if new_len < max_len {
                    queue.push(new_path);
                }
            }
        }

        results
    }

    /// Extend a partial match by following edges or variable-length paths (unified recursive algorithm).
    /// Uses backtracking to find all complete matches.
    fn extend_pattern_match(
        graph_nodes: &HashMap<String, GraphNode>,
        binding: &mut HashMap<String, String>,
        current_node: &str,
        pattern_nodes: &[PatternNode],
        pattern_edges: &[EdgeOrPath],
        edge_index: usize,
        results: &mut Vec<HashMap<String, String>>
    ) {
        // Base case: all edges/paths processed, we have a complete match
        if edge_index >= pattern_edges.len() {
            results.push(binding.clone());
            return;
        }

        let next_node_pattern = &pattern_nodes[edge_index + 1];
        let next_var = match &next_node_pattern.variable {
            Some(v) => v,
            None => return,
        };

        // Handle either fixed edge or variable-length path
        match &pattern_edges[edge_index] {
            EdgeOrPath::Edge(edge_pattern) => {
                // Original single-edge matching logic
                let current_graph_node = match graph_nodes.get(current_node) {
                    Some(n) => n,
                    None => return,
                };

                // Choose which edges to follow based on direction
                let edges_to_follow: Vec<(&String, &EdgeInfo)> = match edge_pattern.direction.as_str() {
                    "incoming" => current_graph_node.predecessors.iter().collect(),
                    "outgoing" => current_graph_node.neighbors.iter().collect(),
                    "both" => current_graph_node.neighbors.iter().collect(),
                    _ => current_graph_node.neighbors.iter().collect(),
                };

                // Try each neighbor that matches the pattern
                for (neighbor_id, edge_info) in edges_to_follow {
                    // Check edge type constraint
                    if let Some(ref required_type) = edge_pattern.edge_type {
                        if edge_info.edge_type != *required_type {
                            continue;
                        }
                    }

                    // Check neighbor node type constraint
                    let matches_type = match &next_node_pattern.node_type {
                        None => true,
                        Some(required_type) => {
                            match graph_nodes.get(neighbor_id) {
                                Some(node) => node.node_type.as_ref() == Some(required_type),
                                None => false,
                            }
                        }
                    };
                    if !matches_type {
                        continue;
                    }

                    // Check bidirectional constraint (only for "both" direction)
                    if edge_pattern.direction == "both" {
                        let has_reverse = graph_nodes.get(neighbor_id)
                            .map_or(false, |n| n.neighbors.contains_key(current_node));
                        if !has_reverse {
                            continue;
                        }
                    }

                    // Check if variable is already bound
                    let was_bound = binding.contains_key(next_var);
                    if let Some(existing_binding) = binding.get(next_var) {
                        if existing_binding != neighbor_id {
                            continue;
                        }
                    } else {
                        binding.insert(next_var.clone(), neighbor_id.clone());
                    }

                    // Recurse to extend the match
                    Self::extend_pattern_match(
                        graph_nodes,
                        binding,
                        neighbor_id,
                        pattern_nodes,
                        pattern_edges,
                        edge_index + 1,
                        results
                    );

                    // Backtrack: remove binding only if we added it
                    if !was_bound {
                        binding.remove(next_var);
                    }
                }
            },
            EdgeOrPath::Path(path_pattern) => {
                // Variable-length path matching
                let edge_type = if path_pattern.edge_type.is_empty() {
                    None
                } else {
                    Some(path_pattern.edge_type.as_str())
                };

                // Find all paths from current node with the specified length range
                let paths = Self::find_variable_length_paths(
                    graph_nodes,
                    current_node,
                    path_pattern.min,
                    path_pattern.max,
                    edge_type,
                    &path_pattern.direction
                );

                // Try each found path
                for path in paths {
                    if path.is_empty() {
                        continue;
                    }

                    let end_node = path.last().unwrap();

                    // Check end node type constraint
                    let matches_type = match &next_node_pattern.node_type {
                        None => true,
                        Some(required_type) => {
                            match graph_nodes.get(end_node) {
                                Some(node) => node.node_type.as_ref() == Some(required_type),
                                None => false,
                            }
                        }
                    };
                    if !matches_type {
                        continue;
                    }

                    // Check if variable is already bound
                    let was_bound = binding.contains_key(next_var);
                    if let Some(existing_binding) = binding.get(next_var) {
                        if existing_binding != end_node {
                            continue;
                        }
                    } else {
                        binding.insert(next_var.clone(), end_node.clone());
                    }

                    // Recurse to extend the match
                    Self::extend_pattern_match(
                        graph_nodes,
                        binding,
                        end_node,
                        pattern_nodes,
                        pattern_edges,
                        edge_index + 1,
                        results
                    );

                    // Backtrack: remove binding only if we added it
                    if !was_bound {
                        binding.remove(next_var);
                    }
                }
            }
        }
    }

    /// Returns all nodes reachable within N hops from a starting node.
    ///
    /// Uses BFS to find all nodes that can be reached from `start` within `hops` edges.
    /// Includes the starting node itself (at distance 0).
    ///
    /// # Arguments
    /// * `start` - The starting node ID
    /// * `hops` - Maximum number of edges to traverse
    /// * `edge_type` - Optional edge type filter (only traverse edges of this type)
    ///
    /// # Returns
    /// Vector of node IDs reachable within the specified hops
    ///
    /// # Example
    /// ```
    /// use graphoid::values::{Graph, GraphType, Value};
    /// use std::collections::HashMap;
    ///
    /// let mut g = Graph::new(GraphType::Directed);
    /// g.add_node("A".to_string(), Value::number(1.0)).unwrap();
    /// g.add_node("B".to_string(), Value::number(2.0)).unwrap();
    /// g.add_node("C".to_string(), Value::number(3.0)).unwrap();
    /// g.add_edge("A", "B", "road".to_string(), None, HashMap::new()).unwrap();
    /// g.add_edge("B", "C", "road".to_string(), None, HashMap::new()).unwrap();
    ///
    /// let nodes = g.nodes_within("A", 1, None);
    /// assert!(nodes.contains(&"A".to_string()));
    /// assert!(nodes.contains(&"B".to_string()));
    /// assert!(!nodes.contains(&"C".to_string())); // C is 2 hops away
    /// ```
    pub fn nodes_within(&self, start: &str, hops: usize, edge_type: Option<&str>) -> Vec<String> {
        // Handle special cases
        if !self.has_node(start) {
            return Vec::new();
        }

        // BFS with hop tracking
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Queue stores (node_id, current_hops)
        queue.push_back((start.to_string(), 0));
        visited.insert(start.to_string());
        result.push(start.to_string());

        while let Some((current, current_hops)) = queue.pop_front() {
            // Don't explore beyond max hops
            if current_hops >= hops {
                continue;
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
                        result.push(neighbor_id.clone());
                        queue.push_back((neighbor_id.clone(), current_hops + 1));
                    }
                }
            }
        }

        result
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
    /// g.add_node("root".to_string(), Value::number(1.0)).unwrap();
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
                            value: Value::number(0.0),
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

    /// Get a specific rule if active (from either rulesets or ad hoc)
    /// Returns the RuleSpec which contains any parameters
    pub fn get_rule(&self, rule_name: &str) -> Option<RuleSpec> {
        // Check ad hoc rules first (they may have specific parameters)
        for rule in &self.rules {
            if rule.spec.name() == rule_name {
                return Some(rule.spec.clone());
            }
        }

        // Check ruleset rules (return default spec for ruleset rules)
        for ruleset in &self.rulesets {
            match ruleset.as_str() {
                "tree" => {
                    match rule_name {
                        "no_cycles" => return Some(RuleSpec::NoCycles),
                        "single_root" => return Some(RuleSpec::SingleRoot),
                        "connected" => return Some(RuleSpec::Connected),
                        _ => {}
                    }
                }
                "binary_tree" => {
                    match rule_name {
                        "no_cycles" => return Some(RuleSpec::NoCycles),
                        "single_root" => return Some(RuleSpec::SingleRoot),
                        "connected" => return Some(RuleSpec::Connected),
                        "binary_tree" => return Some(RuleSpec::BinaryTree),
                        _ => {}
                    }
                }
                "dag" => {
                    if rule_name == "no_cycles" {
                        return Some(RuleSpec::NoCycles);
                    }
                }
                _ => {}
            }
        }

        None
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
    /// let nodes = g.find_nodes_by_property("user_id", &Value::number(42.0));
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

    // =========================================================================
    // Freeze Control (Phase 8)
    // =========================================================================

    /// Mark this graph as frozen (immutable)
    ///
    /// Frozen graphs cannot be modified (no add_node, add_edge, remove operations)
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Check if this graph is frozen
    pub fn is_frozen(&self) -> bool {
        self.frozen
    }

    /// Create an unfrozen deep copy of this graph
    ///
    /// The copy will have the same structure and data, but frozen=false
    pub fn deep_copy_unfrozen(&self) -> Self {
        let mut copy = self.clone();
        copy.frozen = false;
        copy
    }

    // =========================================================================
    // Method Storage (Class-like Graphs)
    // =========================================================================
    //
    // Methods are stored in a dedicated `__methods__` branch of the graph.
    // Structure:
    //   __methods__              (container node)
    //   __methods__/add          (method node with Function value)
    //   __methods__/increment    (method node with Function value)
    //
    //   Edges: __methods__ --has_method--> __methods__/add
    //
    // This follows Graphoid's "everything is a graph" principle with a clean
    // separation between data nodes and method nodes.

    /// The prefix used for method node IDs
    const METHOD_BRANCH: &'static str = "__methods__";

    /// Get the full node ID for a method
    fn method_node_id(name: &str) -> String {
        format!("__methods__/{}", name)
    }

    /// Get the full node ID for a CLG property
    /// Properties are stored at __properties__/name to keep them separate from user data nodes
    pub fn property_node_id(name: &str) -> String {
        format!("__properties__/{}", name)
    }

    /// Ensure the __methods__ branch node exists
    fn ensure_methods_branch(&mut self) {
        if !self.nodes.contains_key(Self::METHOD_BRANCH) {
            let branch_node = GraphNode {
                id: Self::METHOD_BRANCH.to_string(),
                value: Value::none(),
                node_type: Some("__branch__".to_string()),
                properties: HashMap::new(),
                neighbors: HashMap::new(),
                predecessors: HashMap::new(),
            };
            self.nodes.insert(Self::METHOD_BRANCH.to_string(), branch_node);
        }
    }

    /// Attach a method (function) to this graph
    ///
    /// Methods attached to a graph can be called with the graph as `self`.
    /// This enables class-like behavior where graphs act as objects.
    /// Methods are stored under the `__methods__` branch, keeping them
    /// cleanly separated from data nodes.
    ///
    /// # Example
    /// ```
    /// use graphoid::values::{Graph, GraphType, Function};
    /// use graphoid::execution::Environment;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let mut g = Graph::new(GraphType::Directed);
    /// let func = Function {
    ///     name: Some("add".to_string()),
    ///     params: vec![],
    ///     parameters: vec![],
    ///     body: vec![],
    ///     pattern_clauses: None,
    ///     env: Rc::new(RefCell::new(Environment::new())),
    ///     node_id: None,
    ///     is_setter: false,
    ///     is_static: false,
    ///     guard: None,
    /// };
    /// g.attach_method("add".to_string(), func);
    ///
    /// assert!(g.has_method("add"));
    /// assert!(g.has_node("__methods__"));
    /// assert!(g.has_node("__methods__/add"));
    /// ```
    pub fn attach_method(&mut self, name: String, func: Function) {
        // Ensure __methods__ branch exists
        self.ensure_methods_branch();

        let method_id = Self::method_node_id(&name);

        // Phase 21: If the function has a guard or if a method with this name already exists,
        // we need to store multiple variants as a list.
        if func.guard.is_some() || self.nodes.contains_key(&method_id) {
            // Check if method already exists
            if let Some(existing_node) = self.nodes.get_mut(&method_id) {
                // Method exists - convert to list if needed and append
                match &mut existing_node.value.kind {
                    ValueKind::List(list) => {
                        // Already a list, just append
                        let _ = list.append_raw(Value::function(func));
                    }
                    ValueKind::Function(_) => {
                        // Convert single function to list and append new one
                        let old_value = std::mem::replace(&mut existing_node.value, Value::none());
                        let new_list = List::from_vec(vec![old_value, Value::function(func)]);
                        existing_node.value = Value::list(new_list);
                    }
                    _ => {
                        // Unexpected type - replace with new function
                        existing_node.value = Value::function(func);
                    }
                }
            } else {
                // New guarded method - store as single function (will become list if more added)
                let method_node = GraphNode {
                    id: method_id.clone(),
                    value: Value::function(func),
                    node_type: Some("__method__".to_string()),
                    properties: HashMap::new(),
                    neighbors: HashMap::new(),
                    predecessors: HashMap::new(),
                };
                self.nodes.insert(method_id.clone(), method_node);

                // Add edge from __methods__ branch to this method node
                if let Some(branch) = self.nodes.get_mut(Self::METHOD_BRANCH) {
                    branch.neighbors.insert(method_id.clone(), EdgeInfo {
                        edge_type: "has_method".to_string(),
                        weight: None,
                        properties: HashMap::new(),
                    });
                }
                // Add predecessor edge back to branch
                if let Some(method) = self.nodes.get_mut(&method_id) {
                    method.predecessors.insert(Self::METHOD_BRANCH.to_string(), EdgeInfo {
                        edge_type: "has_method".to_string(),
                        weight: None,
                        properties: HashMap::new(),
                    });
                }
            }
        } else {
            // Simple case: no guard and no existing method - just store the function
            let method_node = GraphNode {
                id: method_id.clone(),
                value: Value::function(func),
                node_type: Some("__method__".to_string()),
                properties: HashMap::new(),
                neighbors: HashMap::new(),
                predecessors: HashMap::new(),
            };
            self.nodes.insert(method_id.clone(), method_node);

            // Add edge from __methods__ branch to this method node
            if let Some(branch) = self.nodes.get_mut(Self::METHOD_BRANCH) {
                branch.neighbors.insert(method_id.clone(), EdgeInfo {
                    edge_type: "has_method".to_string(),
                    weight: None,
                    properties: HashMap::new(),
                });
            }
            // Add predecessor edge back to branch
            if let Some(method) = self.nodes.get_mut(&method_id) {
                method.predecessors.insert(Self::METHOD_BRANCH.to_string(), EdgeInfo {
                    edge_type: "has_method".to_string(),
                    weight: None,
                    properties: HashMap::new(),
                });
            }
        }
    }

    /// Helper to add a semantic edge between two nodes (bidirectional: neighbor + predecessor)
    fn add_semantic_edge(&mut self, from_id: &str, to_id: &str, edge_type: &str) {
        if !self.nodes.contains_key(to_id) {
            return;
        }
        // Add edge from -> to
        if let Some(from_node) = self.nodes.get_mut(from_id) {
            from_node.neighbors.insert(to_id.to_string(), EdgeInfo {
                edge_type: edge_type.to_string(),
                weight: None,
                properties: HashMap::new(),
            });
        }
        // Add predecessor edge back
        if let Some(to_node) = self.nodes.get_mut(to_id) {
            to_node.predecessors.insert(from_id.to_string(), EdgeInfo {
                edge_type: edge_type.to_string(),
                weight: None,
                properties: HashMap::new(),
            });
        }
    }

    /// Add semantic edges from a method to the properties it reads/writes
    /// This enables graph traversal from methods to data properties
    pub fn add_method_property_edges(&mut self, method_name: &str, reads: &[String], writes: &[String]) {
        let method_id = Self::method_node_id(method_name);

        // Only add edges if the method node exists
        if !self.nodes.contains_key(&method_id) {
            return;
        }

        // Add "reads" edges from method to properties
        for prop in reads {
            self.add_semantic_edge(&method_id, prop, "reads");
        }

        // Add "writes" edges from method to properties
        for prop in writes {
            self.add_semantic_edge(&method_id, prop, "writes");
        }
    }

    /// Get all data property names (excluding internal nodes like __methods__)
    pub fn property_names(&self) -> Vec<String> {
        self.nodes.keys()
            .filter(|id| !id.starts_with("__"))
            .filter(|id| {
                // Also exclude computed_property_alias nodes
                if let Some(node) = self.nodes.get(*id) {
                    node.node_type.as_deref() != Some("computed_property_alias")
                } else {
                    true
                }
            })
            .cloned()
            .collect()
    }

    /// Suggest similar property names for error messages
    /// Returns properties that are similar to the given name (prefix match or edit distance)
    pub fn suggest_similar_properties(&self, name: &str) -> Vec<String> {
        let properties = self.property_names();
        let name_lower = name.to_lowercase();

        let mut suggestions: Vec<(String, usize)> = properties.iter()
            .filter_map(|prop| {
                let prop_lower = prop.to_lowercase();
                // Exact prefix match gets priority
                if prop_lower.starts_with(&name_lower) || name_lower.starts_with(&prop_lower) {
                    return Some((prop.clone(), 0));
                }
                // Simple edit distance (Levenshtein-like)
                let dist = Self::edit_distance(&name_lower, &prop_lower);
                if dist <= 2 {
                    Some((prop.clone(), dist))
                } else {
                    None
                }
            })
            .collect();

        // Sort by edit distance
        suggestions.sort_by_key(|(_, dist)| *dist);
        suggestions.into_iter().map(|(s, _)| s).take(3).collect()
    }

    /// Simple edit distance calculation
    fn edit_distance(a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let m = a_chars.len();
        let n = b_chars.len();

        if m == 0 { return n; }
        if n == 0 { return m; }

        let mut dp = vec![vec![0usize; n + 1]; m + 1];

        for i in 0..=m { dp[i][0] = i; }
        for j in 0..=n { dp[0][j] = j; }

        for i in 1..=m {
            for j in 1..=n {
                let cost = if a_chars[i-1] == b_chars[j-1] { 0 } else { 1 };
                dp[i][j] = (dp[i-1][j] + 1)
                    .min(dp[i][j-1] + 1)
                    .min(dp[i-1][j-1] + cost);
            }
        }

        dp[m][n]
    }

    /// Get list of properties that a method reads
    pub fn method_reads(&self, method_name: &str) -> Vec<String> {
        let method_id = Self::method_node_id(method_name);
        if let Some(method_node) = self.nodes.get(&method_id) {
            method_node.neighbors.iter()
                .filter(|(_, edge)| edge.edge_type == "reads")
                .map(|(id, _)| id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get list of properties that a method writes
    pub fn method_writes(&self, method_name: &str) -> Vec<String> {
        let method_id = Self::method_node_id(method_name);
        if let Some(method_node) = self.nodes.get(&method_id) {
            method_node.neighbors.iter()
                .filter(|(_, edge)| edge.edge_type == "writes")
                .map(|(id, _)| id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get list of methods that read a property
    pub fn property_readers(&self, property_name: &str) -> Vec<String> {
        if let Some(prop_node) = self.nodes.get(property_name) {
            prop_node.predecessors.iter()
                .filter(|(_, edge)| edge.edge_type == "reads")
                .filter_map(|(id, _)| {
                    // Extract method name from __methods__/name
                    if id.starts_with("__methods__/") {
                        Some(id.strip_prefix("__methods__/").unwrap().to_string())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get list of methods that write to a property
    pub fn property_writers(&self, property_name: &str) -> Vec<String> {
        if let Some(prop_node) = self.nodes.get(property_name) {
            prop_node.predecessors.iter()
                .filter(|(_, edge)| edge.edge_type == "writes")
                .filter_map(|(id, _)| {
                    // Extract method name from __methods__/name
                    if id.starts_with("__methods__/") {
                        Some(id.strip_prefix("__methods__/").unwrap().to_string())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    // ==========================================================================
    // Property Dependency Edges
    // ==========================================================================

    /// Get list of properties that a computed property depends on
    pub fn dependencies(&self, property_name: &str) -> Vec<String> {
        if let Some(prop_node) = self.nodes.get(property_name) {
            prop_node.neighbors.iter()
                .filter(|(_, edge)| edge.edge_type == "depends_on")
                .map(|(id, _)| id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get list of computed properties that depend on this property
    pub fn dependents(&self, property_name: &str) -> Vec<String> {
        if let Some(prop_node) = self.nodes.get(property_name) {
            prop_node.predecessors.iter()
                .filter(|(_, edge)| edge.edge_type == "depends_on")
                // Filter out internal __computed__/ prefixed nodes
                .filter(|(id, _)| !id.starts_with("__computed__/"))
                .map(|(id, _)| id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get properties in topological order based on dependencies.
    /// Properties with no dependencies come first, then computed properties.
    pub fn dependency_order(&self) -> Vec<String> {
        use std::collections::{VecDeque, HashSet};

        // Collect all property nodes (excluding method nodes and internal nodes)
        let properties: Vec<String> = self.nodes.keys()
            .filter(|id| !id.starts_with("__methods__") && !id.starts_with("__") && !id.starts_with("__computed__/"))
            .cloned()
            .collect();

        // Build dependency counts
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut deps_map: HashMap<String, Vec<String>> = HashMap::new();

        for prop in &properties {
            in_degree.insert(prop.clone(), 0);
            deps_map.insert(prop.clone(), Vec::new());
        }

        // Count dependencies
        for prop in &properties {
            if let Some(node) = self.nodes.get(prop) {
                for (neighbor, edge) in &node.neighbors {
                    if edge.edge_type == "depends_on" && properties.contains(neighbor) {
                        // prop depends on neighbor
                        if let Some(count) = in_degree.get_mut(prop) {
                            *count += 1;
                        }
                        if let Some(deps) = deps_map.get_mut(neighbor) {
                            deps.push(prop.clone());
                        }
                    }
                }
            }
        }

        // Kahn's algorithm for topological sort
        let mut result = Vec::new();
        let mut queue: VecDeque<String> = in_degree.iter()
            .filter(|(_, &count)| count == 0)
            .map(|(id, _)| id.clone())
            .collect();

        let mut visited: HashSet<String> = HashSet::new();

        while let Some(prop) = queue.pop_front() {
            if visited.contains(&prop) {
                continue;
            }
            visited.insert(prop.clone());
            result.push(prop.clone());

            if let Some(dependents) = deps_map.get(&prop) {
                for dep in dependents {
                    if let Some(count) = in_degree.get_mut(dep) {
                        *count = count.saturating_sub(1);
                        if *count == 0 && !visited.contains(dep) {
                            queue.push_back(dep.clone());
                        }
                    }
                }
            }
        }

        result
    }

    // ==========================================================================
    // Phase 3: Inheritance as Graph Structure
    // ==========================================================================

    /// Finalize inheritance node by replacing __self__ with actual type name.
    /// Called after type_name is set to update the inheritance graph structure.
    pub fn finalize_inheritance_node(&mut self, type_name: &str) {
        // If there's a __self__ node, replace it with the type name node
        if let Some(mut self_node) = self.nodes.remove("__self__") {
            // Update node ID and value
            self_node.id = type_name.to_string();
            self_node.value = Value::string(type_name.to_string());

            // Re-insert with new ID
            self.nodes.insert(type_name.to_string(), self_node);

            // Update predecessor references in __parent__ if it exists
            if let Some(parent_node) = self.nodes.get_mut("__parent__") {
                if let Some(edge) = parent_node.predecessors.remove("__self__") {
                    parent_node.predecessors.insert(type_name.to_string(), edge);
                }
            }
        }
    }

    /// Get list of ancestor type names by following the inheritance chain.
    /// Returns type names of all parent graphs in order from immediate parent to root.
    pub fn ancestors(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = self.parent.as_ref();

        while let Some(parent) = current {
            if let Some(name) = &parent.type_name {
                result.push(name.clone());
            }
            current = parent.parent.as_ref();
        }

        result
    }

    /// Get a method attached to this graph by name
    /// Returns the first Function if the method exists (for backward compatibility)
    /// For structure-based dispatch with guards, use get_method_variants instead.
    pub fn get_method(&self, name: &str) -> Option<&Function> {
        let method_id = Self::method_node_id(name);
        if let Some(node) = self.nodes.get(&method_id) {
            // Check if it's a single function - for lists we need to use get_method_variants
            if let ValueKind::Function(func) = &node.value.kind {
                return Some(func);
            }
            // Note: For lists, we fall through. Caller should use get_method_variants
            // for structure-based dispatch. This preserves backward compatibility
            // for methods defined without guards.
        }
        None
    }

    /// Get the first method variant (owned) - for when we need to iterate methods with guards
    pub fn get_first_method(&self, name: &str) -> Option<Function> {
        let method_id = Self::method_node_id(name);
        if let Some(node) = self.nodes.get(&method_id) {
            match &node.value.kind {
                ValueKind::Function(func) => return Some(func.clone()),
                ValueKind::List(list) => {
                    // Return the first variant from the list
                    if let Some(first) = list.get(0) {
                        if let ValueKind::Function(func) = &first.kind {
                            return Some(func.clone());
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Get all method variants for a method name (for structure-based dispatch)
    /// Returns all Function implementations with their guards.
    /// If the method has no variants (was defined without `when`), returns a single-element vector.
    pub fn get_method_variants(&self, name: &str) -> Vec<Function> {
        let method_id = Self::method_node_id(name);
        if let Some(node) = self.nodes.get(&method_id) {
            match &node.value.kind {
                ValueKind::Function(func) => return vec![func.clone()],
                ValueKind::List(list) => {
                    // List is graph-backed, use to_vec() to get elements
                    return list.to_vec().into_iter()
                        .filter_map(|v| {
                            if let ValueKind::Function(func) = v.kind {
                                Some(func)
                            } else {
                                None
                            }
                        })
                        .collect();
                }
                _ => {}
            }
        }
        Vec::new()
    }

    /// Check if this graph has a method with the given name
    pub fn has_method(&self, name: &str) -> bool {
        let method_id = Self::method_node_id(name);
        self.nodes.contains_key(&method_id)
    }

    /// Get all method names attached to this graph
    pub fn method_names(&self) -> Vec<String> {
        const PREFIX: &str = "__methods__/";
        self.nodes.keys()
            .filter(|id| id.starts_with(PREFIX))
            .map(|id| id[PREFIX.len()..].to_string())
            .collect()
    }

    /// Remove a method from this graph by name
    /// Returns true if the method was found and removed, false if it didn't exist
    pub fn remove_method(&mut self, name: &str) -> bool {
        let method_id = Self::method_node_id(name);

        // Check if method exists
        if !self.nodes.contains_key(&method_id) {
            return false;
        }

        // Remove edge from __methods__ branch to this method
        if let Some(branch) = self.nodes.get_mut(Self::METHOD_BRANCH) {
            branch.neighbors.remove(&method_id);
        }

        // Remove the method node itself
        self.nodes.remove(&method_id);

        true
    }

    /// Include all methods from another graph (mixin pattern)
    /// Returns a list of method names that were copied
    pub fn include_methods_from(&mut self, other: &Graph) -> Vec<String> {
        let mut included = Vec::new();

        for method_name in other.method_names() {
            // Skip private methods (starting with underscore)
            if method_name.starts_with('_') {
                continue;
            }

            // Get all variants of this method (for structure-based dispatch)
            let variants = other.get_method_variants(&method_name);
            for func in variants {
                self.attach_method(method_name.clone(), func);
            }
            included.push(method_name);
        }

        included
    }

    // =========================================================================
    // Phase 19: Setters (Computed Property Assignment)
    // =========================================================================

    /// Get the setter method node ID for a property name
    /// Setters are stored as __methods__/__set__<name>
    fn setter_node_id(name: &str) -> String {
        format!("__methods__/__set__{}", name)
    }

    /// Attach a setter to this graph for a property
    /// Setters are stored with the naming convention __set__<property_name>
    pub fn attach_setter(&mut self, property_name: String, func: Function) {
        // Ensure __methods__ branch exists
        self.ensure_methods_branch();

        // Create setter node with namespaced ID
        let setter_id = Self::setter_node_id(&property_name);

        // Create the method node with node_type "__setter__"
        let setter_node = GraphNode {
            id: setter_id.clone(),
            value: Value::function(func),
            node_type: Some("__setter__".to_string()),
            properties: HashMap::new(),
            neighbors: HashMap::new(),
            predecessors: HashMap::new(),
        };

        // Add the setter node to the graph
        self.nodes.insert(setter_id.clone(), setter_node);

        // Add edge from __methods__ branch to this setter
        if let Some(branch) = self.nodes.get_mut(Self::METHOD_BRANCH) {
            branch.neighbors.insert(
                setter_id,
                EdgeInfo::new("setter".to_string(), HashMap::new())
            );
        }
    }

    /// Get a setter attached to this graph by property name
    /// Returns the Function if the setter exists
    pub fn get_setter(&self, property_name: &str) -> Option<&Function> {
        let setter_id = Self::setter_node_id(property_name);
        if let Some(node) = self.nodes.get(&setter_id) {
            if let ValueKind::Function(func) = &node.value.kind {
                return Some(func);
            }
        }
        None
    }

    /// Check if this graph has a setter for the given property
    pub fn has_setter(&self, property_name: &str) -> bool {
        let setter_id = Self::setter_node_id(property_name);
        self.nodes.contains_key(&setter_id)
    }

    // =========================================================================
    // Phase 20: Static Methods (Class Methods)
    // =========================================================================

    /// Get the static method node ID for a method name
    /// Static methods are stored as __methods__/__static__<name>
    fn static_method_node_id(name: &str) -> String {
        format!("__methods__/__static__{}", name)
    }

    /// Attach a static method to this graph
    /// Static methods can be called on the class without creating an instance
    pub fn attach_static_method(&mut self, name: String, func: Function) {
        // Ensure __methods__ branch exists
        self.ensure_methods_branch();

        // Create static method node with namespaced ID
        let static_id = Self::static_method_node_id(&name);

        // Create the method node with node_type "__static__"
        let static_node = GraphNode {
            id: static_id.clone(),
            value: Value::function(func),
            node_type: Some("__static__".to_string()),
            properties: HashMap::new(),
            neighbors: HashMap::new(),
            predecessors: HashMap::new(),
        };

        // Add the static method node to the graph
        self.nodes.insert(static_id.clone(), static_node);

        // Add edge from __methods__ branch to this static method
        if let Some(branch) = self.nodes.get_mut(Self::METHOD_BRANCH) {
            branch.neighbors.insert(
                static_id,
                EdgeInfo::new("static".to_string(), HashMap::new())
            );
        }
    }

    /// Get a static method attached to this graph by name
    /// Returns the Function if the static method exists
    pub fn get_static_method(&self, name: &str) -> Option<&Function> {
        let static_id = Self::static_method_node_id(name);
        if let Some(node) = self.nodes.get(&static_id) {
            if let ValueKind::Function(func) = &node.value.kind {
                return Some(func);
            }
        }
        None
    }

    /// Check if this graph has a static method with the given name
    pub fn has_static_method(&self, name: &str) -> bool {
        let static_id = Self::static_method_node_id(name);
        self.nodes.contains_key(&static_id)
    }

    /// Get all data node IDs (excluding internal branches)
    /// Filters out:
    /// - Method branch nodes (__methods__/*)
    /// - Property branch nodes (__properties__/*)
    /// - Internal nodes (__parent__, __self__)
    /// Use this when you want to iterate over user-added data nodes only
    pub fn data_node_ids(&self) -> Vec<String> {
        self.nodes.keys()
            .filter(|id| {
                // Exclude method branch nodes
                if id.starts_with("__methods__") {
                    return false;
                }
                // Exclude property branch nodes
                if id.starts_with("__properties__") {
                    return false;
                }
                // Exclude internal nodes
                if *id == "__parent__" || *id == "__self__" {
                    return false;
                }
                true
            })
            .cloned()
            .collect()
    }

    /// Get all CLG property names (from __properties__/ branch)
    /// Returns just the property names without the __properties__/ prefix
    /// Use this to access the class-defined properties of a CLG
    pub fn property_node_ids(&self) -> Vec<String> {
        self.nodes.keys()
            .filter(|id| id.starts_with("__properties__/"))
            .map(|id| id.strip_prefix("__properties__/").unwrap().to_string())
            .collect()
    }

    /// Get all constrainable node IDs (for rule enforcement)
    /// Excludes only method branch nodes, includes property nodes
    /// Used by constraint checking to detect node additions/removals
    pub fn constrainable_node_ids(&self) -> Vec<String> {
        self.nodes.keys()
            .filter(|id| {
                // Only exclude method branch nodes and internal system nodes
                !id.starts_with("__methods__") && *id != "__parent__" && *id != "__self__"
            })
            .cloned()
            .collect()
    }

    // =========================================================================
    // Orphan Management (Subgraph Operations)
    // =========================================================================

    /// Find all orphaned nodes (nodes with no edges)
    /// An orphan has no predecessors AND no successors
    pub fn find_orphans(&self) -> Vec<String> {
        self.nodes.iter()
            .filter(|(_, node)| {
                node.neighbors.is_empty() && node.predecessors.is_empty()
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Count orphaned nodes
    pub fn count_orphans(&self) -> usize {
        self.find_orphans().len()
    }

    /// Check if graph has orphans
    pub fn has_orphans(&self) -> bool {
        self.count_orphans() > 0
    }

    /// Find nodes that would become orphans if the specified node was removed
    fn find_would_be_orphans(&self, id: &str) -> Vec<String> {
        let mut would_be_orphans = Vec::new();

        // Get the node being removed
        let node_to_remove = match self.nodes.get(id) {
            Some(n) => n,
            None => return would_be_orphans, // Node doesn't exist, no orphans
        };

        // Check each neighbor of the node being removed
        for neighbor_id in node_to_remove.neighbors.keys() {
            if let Some(neighbor) = self.nodes.get(neighbor_id) {
                // After removal, would this neighbor have no edges?
                let would_have_predecessors = neighbor.predecessors.len() > 1 ||
                    (neighbor.predecessors.len() == 1 && !neighbor.predecessors.contains_key(id));
                let would_have_successors = !neighbor.neighbors.is_empty();

                if !would_have_predecessors && !would_have_successors {
                    would_be_orphans.push(neighbor_id.clone());
                }
            }
        }

        // Also check nodes that have the removed node as their only predecessor
        for (potential_orphan_id, potential_orphan) in &self.nodes {
            if potential_orphan_id == id {
                continue; // Skip the node being removed
            }

            // Would this node become an orphan?
            let would_lose_only_predecessor = potential_orphan.predecessors.len() == 1
                && potential_orphan.predecessors.contains_key(id);
            let has_no_successors = potential_orphan.neighbors.is_empty();

            if would_lose_only_predecessor && has_no_successors {
                would_be_orphans.push(potential_orphan_id.clone());
            }
        }

        would_be_orphans
    }

    /// Delete ALL orphaned nodes (never selective!)
    /// Returns list of deleted node IDs
    pub fn delete_orphans(&mut self) -> Result<Vec<String>, GraphoidError> {
        let orphan_ids = self.find_orphans();

        for id in &orphan_ids {
            self.remove_node_internal(id)?;
        }

        Ok(orphan_ids)
    }

    /// Reconnect a single orphan to a parent node
    /// Creates a new edge from parent to orphan
    pub fn reconnect_orphan(
        &mut self,
        orphan_id: &str,
        parent_id: &str,
        edge_type: String,
    ) -> Result<(), GraphoidError> {
        // Verify orphan exists and is actually an orphan
        if !self.has_node(orphan_id) {
            return Err(GraphoidError::runtime(format!(
                "Node '{}' does not exist",
                orphan_id
            )));
        }

        let orphans = self.find_orphans();
        if !orphans.contains(&orphan_id.to_string()) {
            return Err(GraphoidError::runtime(format!(
                "Node '{}' is not an orphan",
                orphan_id
            )));
        }

        // Verify parent exists
        if !self.has_node(parent_id) {
            return Err(GraphoidError::runtime(format!(
                "Parent node '{}' does not exist",
                parent_id
            )));
        }

        // Create edge from parent to orphan
        self.add_edge(parent_id, orphan_id, edge_type, None, std::collections::HashMap::new())?;

        Ok(())
    }

    /// Reconnect all orphans using the specified strategy
    /// Returns the number of orphans reconnected
    pub fn reconnect_orphans(
        &mut self,
        strategy: ReconnectStrategy,
    ) -> Result<usize, GraphoidError> {
        let orphan_ids = self.find_orphans();
        let orphan_count = orphan_ids.len();

        if orphan_count == 0 {
            return Ok(0);
        }

        match strategy {
            ReconnectStrategy::ToRoot => {
                // Find root node (node with no predecessors but has successors)
                let root_id = self.nodes.iter()
                    .find(|(_, node)| node.predecessors.is_empty() && !node.neighbors.is_empty())
                    .map(|(id, _)| id.clone());

                let root_id = root_id.ok_or_else(|| {
                    GraphoidError::runtime("No root node found for reconnection".to_string())
                })?;

                // Reconnect each orphan to root
                for orphan_id in &orphan_ids {
                    self.add_edge(
                        &root_id,
                        orphan_id,
                        "reconnected".to_string(),
                        None,
                        std::collections::HashMap::new(),
                    )?;
                }
            }

            ReconnectStrategy::ToParentSiblings => {
                // This strategy would need to track which nodes were parents of orphans
                // For now, we'll return an error indicating it needs more implementation
                return Err(GraphoidError::runtime(
                    "ToParentSiblings strategy not yet fully implemented".to_string()
                ));
            }
        }

        Ok(orphan_count)
    }

    // =========================================================================
    // Subgraph Operations
    // =========================================================================

    /// Extract a subgraph starting from a root node
    ///
    /// Creates a new graph containing the root node and all descendants
    /// up to the specified depth.
    ///
    /// # Arguments
    /// * `root` - The ID of the root node to start extraction
    /// * `depth` - Maximum depth to traverse (None = infinite)
    ///
    /// # Returns
    /// A new Graph containing the extracted subgraph with the same configuration
    pub fn extract_subgraph(
        &self,
        root: &str,
        depth: Option<usize>,
    ) -> Result<Graph, GraphoidError> {
        // Verify root exists
        if !self.has_node(root) {
            return Err(GraphoidError::runtime(format!(
                "Root node '{}' does not exist",
                root
            )));
        }

        // Create new graph with same type and config
        let mut subgraph = Graph::new(self.graph_type.clone());
        subgraph.config = self.config.clone();

        // BFS to collect nodes up to depth
        use std::collections::{VecDeque, HashSet};
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((root.to_string(), 0));
        visited.insert(root.to_string());

        while let Some((node_id, current_depth)) = queue.pop_front() {
            // Add this node to subgraph
            if let Some(node) = self.nodes.get(&node_id) {
                subgraph.add_node(node_id.clone(), node.value.clone())?;

                // If we haven't reached max depth, add neighbors to queue
                if depth.is_none() || current_depth < depth.unwrap() {
                    for (neighbor_id, edge_info) in &node.neighbors {
                        if !visited.contains(neighbor_id) {
                            visited.insert(neighbor_id.clone());
                            queue.push_back((neighbor_id.clone(), current_depth + 1));
                        }

                        // Add edge to subgraph (if both nodes are in visited set)
                        if visited.contains(neighbor_id) {
                            subgraph.add_edge(
                                &node_id,
                                neighbor_id,
                                edge_info.edge_type.clone(),
                                edge_info.weight,
                                edge_info.properties.clone(),
                            )?;
                        }
                    }
                }
            }
        }

        Ok(subgraph)
    }

    /// Insert a subgraph into this graph at a specified node
    ///
    /// Merges another graph into this one, connecting it via an edge
    /// from the attachment point to the subgraph's nodes.
    ///
    /// # Arguments
    /// * `subgraph` - The graph to insert
    /// * `at` - The node ID in this graph to attach to
    /// * `edge_type` - The type of edge to create from attachment point
    ///
    /// # Returns
    /// Ok(()) if successful, error if operation would violate graph rules
    pub fn insert_subgraph(
        &mut self,
        subgraph: &Graph,
        at: &str,
        edge_type: String,
    ) -> Result<(), GraphoidError> {
        // Check if graph is frozen
        if self.frozen {
            return Err(GraphoidError::runtime(
                "Cannot modify frozen graph".to_string()
            ));
        }

        // Verify attachment point exists
        if !self.has_node(at) {
            return Err(GraphoidError::runtime(format!(
                "Attachment node '{}' does not exist",
                at
            )));
        }

        // Check for node ID conflicts
        for node_id in subgraph.nodes.keys() {
            if self.has_node(node_id) {
                return Err(GraphoidError::runtime(format!(
                    "Cannot insert subgraph: node '{}' already exists in target graph",
                    node_id
                )));
            }
        }

        // Copy all nodes from subgraph
        for (node_id, node) in &subgraph.nodes {
            self.add_node(node_id.clone(), node.value.clone())?;
        }

        // Copy all edges from subgraph
        for (from_id, from_node) in &subgraph.nodes {
            for (to_id, edge_info) in &from_node.neighbors {
                self.add_edge(
                    from_id,
                    to_id,
                    edge_info.edge_type.clone(),
                    edge_info.weight,
                    edge_info.properties.clone(),
                )?;
            }
        }

        // Find root nodes in subgraph (nodes with no predecessors)
        let subgraph_roots: Vec<String> = subgraph.nodes.iter()
            .filter(|(_, node)| node.predecessors.is_empty())
            .map(|(id, _)| id.clone())
            .collect();

        // If no clear root, connect to all nodes with no predecessors in the original subgraph
        if subgraph_roots.is_empty() {
            // If subgraph has no clear entry points, connect to first node
            if let Some(first_id) = subgraph.nodes.keys().next() {
                self.add_edge(
                    at,
                    first_id,
                    edge_type.clone(),
                    None,
                    std::collections::HashMap::new(),
                )?;
            }
        } else {
            // Connect attachment point to all root nodes
            for root_id in &subgraph_roots {
                self.add_edge(
                    at,
                    root_id,
                    edge_type.clone(),
                    None,
                    std::collections::HashMap::new(),
                )?;
            }
        }

        Ok(())
    }

    /// Extract a subgraph using filter predicates (Level 5 - Specification §877-920)
    ///
    /// Filters nodes and edges based on lambda predicates, returning a new graph
    /// containing only matching elements.
    ///
    /// # Arguments
    /// * `node_filter` - Optional predicate for filtering nodes: (id, value) -> bool
    /// * `edge_filter` - Optional predicate for filtering edges: (from, to, type, weight, attrs) -> bool
    /// * `include_orphans` - Whether to include nodes with no edges (default: true)
    ///
    /// # Returns
    /// A new Graph containing only elements matching the filters
    pub fn extract_filtered(
        &self,
        node_filter: Option<Box<dyn Fn(&str, &Value) -> bool>>,
        edge_filter: Option<Box<dyn Fn(&str, &str, &str, Option<f64>, &std::collections::HashMap<String, Value>) -> bool>>,
        include_orphans: bool,
    ) -> Result<Graph, GraphoidError> {
        // Create new graph with same configuration
        let mut result = Graph::new(self.graph_type.clone());
        result.config = self.config.clone();

        // Step 1: Filter nodes
        let mut matching_nodes: std::collections::HashSet<String> = std::collections::HashSet::new();

        if let Some(filter) = node_filter {
            for (node_id, node) in &self.nodes {
                if filter(node_id, &node.value) {
                    matching_nodes.insert(node_id.clone());
                }
            }
        } else {
            // No node filter - all nodes match initially
            for node_id in self.nodes.keys() {
                matching_nodes.insert(node_id.clone());
            }
        }

        // Step 2: Filter edges and track nodes involved in matching edges
        let mut nodes_with_edges: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut matching_edges: Vec<(String, String, EdgeInfo)> = vec![];

        for (from_id, from_node) in &self.nodes {
            // Skip if source node doesn't match node filter
            if !matching_nodes.contains(from_id) {
                continue;
            }

            for (to_id, edge_info) in &from_node.neighbors {
                // Skip if target node doesn't match node filter
                if !matching_nodes.contains(to_id) {
                    continue;
                }

                // Apply edge filter if provided
                let edge_matches = if let Some(ref filter) = edge_filter {
                    filter(from_id, to_id, &edge_info.edge_type, edge_info.weight, &edge_info.properties)
                } else {
                    true // No edge filter - all edges between matching nodes match
                };

                if edge_matches {
                    matching_edges.push((from_id.clone(), to_id.clone(), edge_info.clone()));
                    nodes_with_edges.insert(from_id.clone());
                    nodes_with_edges.insert(to_id.clone());
                }
            }
        }

        // Step 3: Determine final node set based on include_orphans
        let final_nodes: std::collections::HashSet<String> = if include_orphans {
            // Include all matching nodes, even orphans
            matching_nodes
        } else {
            // Only include nodes that have at least one edge
            matching_nodes.intersection(&nodes_with_edges).cloned().collect()
        };

        // Step 4: Add nodes to result graph
        for node_id in &final_nodes {
            if let Some(node) = self.nodes.get(node_id) {
                result.add_node(node_id.clone(), node.value.clone())?;

                // Preserve node type if it exists
                if let Some(node_type) = &node.node_type {
                    result.set_node_type(node_id, node_type.clone())?;
                }
            }
        }

        // Step 5: Add matching edges to result graph
        for (from_id, to_id, edge_info) in matching_edges {
            result.add_edge(
                &from_id,
                &to_id,
                edge_info.edge_type,
                edge_info.weight,
                edge_info.properties,
            )?;
        }

        Ok(result)
    }

    /// Delete a subgraph using filter predicates (Level 5 - Specification §877-920)
    ///
    /// Returns a new graph WITHOUT elements matching the filters.
    /// Essentially the inverse of extract_filtered.
    ///
    /// # Arguments
    /// * `node_filter` - Optional predicate for nodes to DELETE: (id, value) -> bool
    /// * `edge_filter` - Optional predicate for edges to DELETE: (from, to, type, weight, attrs) -> bool
    ///
    /// # Returns
    /// A new Graph without elements matching the filters
    pub fn delete_filtered(
        &self,
        node_filter: Option<Box<dyn Fn(&str, &Value) -> bool>>,
        edge_filter: Option<Box<dyn Fn(&str, &str, &str, Option<f64>, &std::collections::HashMap<String, Value>) -> bool>>,
    ) -> Result<Graph, GraphoidError> {
        // Create inverted filters
        let inverted_node_filter = node_filter.map(|f| {
            Box::new(move |id: &str, val: &Value| -> bool {
                !f(id, val)
            }) as Box<dyn Fn(&str, &Value) -> bool>
        });

        let inverted_edge_filter = edge_filter.map(|f| {
            Box::new(move |from: &str, to: &str, edge_type: &str, weight: Option<f64>, attrs: &std::collections::HashMap<String, Value>| -> bool {
                !f(from, to, edge_type, weight, attrs)
            }) as Box<dyn Fn(&str, &str, &str, Option<f64>, &std::collections::HashMap<String, Value>) -> bool>
        });

        // Use extract_filtered with inverted filters
        // Always include orphans when deleting (keep nodes that don't match delete filter)
        self.extract_filtered(inverted_node_filter, inverted_edge_filter, true)
    }

    /// Add/merge another graph into this one (Level 5 - Specification §877-920)
    ///
    /// Merges all nodes and edges from another graph into a new graph.
    /// Handles node ID conflicts with configurable strategies.
    ///
    /// # Arguments
    /// * `other` - The graph to merge
    /// * `on_conflict` - Conflict resolution strategy: "keep_original", "overwrite", or None (default: keep_original)
    ///
    /// # Returns
    /// A new Graph containing merged elements from both graphs
    pub fn add_subgraph(
        &self,
        other: &Graph,
        on_conflict: Option<String>,
    ) -> Result<Graph, GraphoidError> {
        // Create result graph starting with a copy of self
        let mut result = Graph::new(self.graph_type.clone());
        result.config = self.config.clone();

        // Determine conflict strategy
        let strategy = on_conflict.as_deref().unwrap_or("keep_original");

        // Step 1: Add all nodes from self
        for (node_id, node) in &self.nodes {
            result.add_node(node_id.clone(), node.value.clone())?;
            if let Some(node_type) = &node.node_type {
                result.set_node_type(node_id, node_type.clone())?;
            }
        }

        // Step 2: Add nodes from other, handling conflicts
        for (node_id, node) in &other.nodes {
            if result.has_node(node_id) {
                // Node exists - handle conflict
                match strategy {
                    "keep_original" => {
                        // Skip - keep existing node
                        continue;
                    }
                    "overwrite" => {
                        // Replace with new value
                        // We need to update the existing node's value
                        if let Some(existing_node) = result.nodes.get_mut(node_id) {
                            existing_node.value = node.value.clone();
                            existing_node.node_type = node.node_type.clone();
                        }
                    }
                    "merge" => {
                        // For now, merge means overwrite value but keep all edges
                        if let Some(existing_node) = result.nodes.get_mut(node_id) {
                            existing_node.value = node.value.clone();
                            // Node type: prefer other's type if set
                            if node.node_type.is_some() {
                                existing_node.node_type = node.node_type.clone();
                            }
                        }
                    }
                    _ => {
                        return Err(GraphoidError::runtime(format!(
                            "Unknown conflict resolution strategy: '{}'. Use 'keep_original', 'overwrite', or 'merge'",
                            strategy
                        )));
                    }
                }
            } else {
                // Node doesn't exist - add it
                result.add_node(node_id.clone(), node.value.clone())?;
                if let Some(node_type) = &node.node_type {
                    result.set_node_type(node_id, node_type.clone())?;
                }
            }
        }

        // Step 3: Add all edges from self
        for (from_id, from_node) in &self.nodes {
            for (to_id, edge_info) in &from_node.neighbors {
                result.add_edge(
                    from_id,
                    to_id,
                    edge_info.edge_type.clone(),
                    edge_info.weight,
                    edge_info.properties.clone(),
                )?;
            }
        }

        // Step 4: Add all edges from other
        // Note: Edges are uniquely identified by (from, to, type) tuple
        // If same edge exists, it will be skipped by add_edge
        for (from_id, from_node) in &other.nodes {
            for (to_id, edge_info) in &from_node.neighbors {
                // Only add edge if both nodes exist in result
                if result.has_node(from_id) && result.has_node(to_id) {
                    // Try to add edge - if it already exists, this will fail gracefully
                    // depending on graph configuration
                    let _ = result.add_edge(
                        from_id,
                        to_id,
                        edge_info.edge_type.clone(),
                        edge_info.weight,
                        edge_info.properties.clone(),
                    );
                    // Ignore errors for duplicate edges - keep first one
                }
            }
        }

        Ok(result)
    }
}

