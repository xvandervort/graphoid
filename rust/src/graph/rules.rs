//! Graph rule system for enforcing constraints
//!
//! This module implements the rule validation system that allows graphs to
//! enforce constraints like acyclicity, tree structure, BST ordering, etc.
//!
//! # Architecture
//!
//! Rules can be applied to graphs in two ways:
//! 1. **Rulesets**: Predefined bundles like `:tree`, `:dag`, `:binary_tree`
//! 2. **Ad hoc rules**: Individual rules added/removed via `add_rule()`/`remove_rule()`
//!
//! Rules are stored as `RuleSpec` (clonable specifications) and instantiated
//! on-demand during validation.

use crate::error::GraphoidError;
use crate::values::{Value, Graph};
use std::collections::{HashSet, HashMap};

/// Rule severity - controls notification, NOT enforcement
///
/// In ALL cases, the operation is REJECTED (prevented).
/// Severity only controls how the user is notified.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSeverity {
    /// REJECT operation, no notification
    Silent,
    /// REJECT operation, log warning (DEFAULT - user friendly!)
    Warning,
    /// REJECT operation, throw error (opt-in strict mode)
    Error,
}

impl Default for RuleSeverity {
    fn default() -> Self {
        RuleSeverity::Warning
    }
}

/// Policy for handling existing violations when a rule is added
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetroactivePolicy {
    /// Fix existing violations (DEFAULT - rules implement behaviors, behaviors are retroactive)
    Clean,
    /// Keep existing data, warn about violations
    Warn,
    /// Error if violations exist when rule is added
    Enforce,
    /// Don't check existing data
    Ignore,
}

impl Default for RetroactivePolicy {
    fn default() -> Self {
        RetroactivePolicy::Clean
    }
}

/// Instance of a rule with configured severity
/// This pairs a rule specification with its severity level
#[derive(Debug, Clone, PartialEq)]
pub struct RuleInstance {
    /// The rule specification
    pub spec: RuleSpec,
    /// The configured severity (how violations are handled)
    pub severity: RuleSeverity,
}

impl RuleInstance {
    /// Create a new rule instance with the rule's default severity
    pub fn new(spec: RuleSpec) -> Self {
        let severity = spec.instantiate().default_severity();
        RuleInstance { spec, severity }
    }

    /// Create a new rule instance with a specific severity
    pub fn with_severity(spec: RuleSpec, severity: RuleSeverity) -> Self {
        RuleInstance { spec, severity }
    }
}

/// Specification for a rule that can be stored and cloned
#[derive(Debug, Clone, PartialEq)]
pub enum RuleSpec {
    /// No cycles allowed (for DAGs and trees)
    NoCycles,
    /// Must have exactly one root node
    SingleRoot,
    /// All nodes must be reachable from any starting node
    Connected,
    /// Limit maximum outgoing edges per node
    MaxDegree(usize),
    /// Binary tree (max 2 children per node)
    BinaryTree,
    /// No duplicate values allowed (for lists/sets)
    NoDuplicates,
}

impl RuleSpec {
    /// Convert this specification into an actual Rule instance
    pub fn instantiate(&self) -> Box<dyn Rule> {
        match self {
            RuleSpec::NoCycles => Box::new(NoCyclesRule::new()),
            RuleSpec::SingleRoot => Box::new(SingleRootRule::new()),
            RuleSpec::Connected => Box::new(ConnectedRule::new()),
            RuleSpec::MaxDegree(n) => Box::new(MaxDegreeRule::new(*n)),
            RuleSpec::BinaryTree => Box::new(BinaryTreeRule::new()),
            RuleSpec::NoDuplicates => Box::new(NoDuplicatesRule::new()),
        }
    }

    /// Get the name of this rule
    pub fn name(&self) -> &str {
        match self {
            RuleSpec::NoCycles => "no_cycles",
            RuleSpec::SingleRoot => "single_root",
            RuleSpec::Connected => "connected",
            RuleSpec::MaxDegree(_) => "max_degree",
            RuleSpec::BinaryTree => "binary_tree",
            RuleSpec::NoDuplicates => "no_duplicates",
        }
    }
}

/// Operations that can be performed on a graph
#[derive(Debug, Clone, PartialEq)]
pub enum GraphOperation {
    /// Adding a node to the graph
    AddNode {
        id: String,
        value: Value,
    },
    /// Adding an edge between two nodes
    AddEdge {
        from: String,
        to: String,
        edge_type: String,
        properties: HashMap<String, Value>,
    },
    /// Removing a node from the graph
    RemoveNode {
        id: String,
    },
    /// Removing an edge between two nodes
    RemoveEdge {
        from: String,
        to: String,
    },
}

/// Context information for rule validation
#[derive(Debug, Clone)]
pub struct RuleContext {
    /// The operation being performed
    pub operation: GraphOperation,
    /// Nodes affected by this operation
    pub affected_nodes: Vec<String>,
}

impl RuleContext {
    /// Create a new rule context for the given operation
    pub fn new(operation: GraphOperation) -> Self {
        let affected_nodes = match &operation {
            GraphOperation::AddNode { id, .. } => vec![id.clone()],
            GraphOperation::AddEdge { from, to, .. } => vec![from.clone(), to.clone()],
            GraphOperation::RemoveNode { id } => vec![id.clone()],
            GraphOperation::RemoveEdge { from, to } => vec![from.clone(), to.clone()],
        };

        RuleContext {
            operation,
            affected_nodes,
        }
    }
}

/// Trait for graph validation rules
pub trait Rule {
    /// Get the name of this rule
    fn name(&self) -> &str;

    /// Get the default severity for this rule
    /// All rules default to Warning (reject + log, never crash)
    fn default_severity(&self) -> RuleSeverity {
        RuleSeverity::Warning
    }

    /// Get the default retroactive policy for this rule
    /// Most rules default to Clean (fix existing violations)
    fn default_retroactive_policy(&self) -> RetroactivePolicy {
        RetroactivePolicy::Clean
    }

    /// Validate the graph against this rule
    /// Returns Ok(()) if valid, Err(GraphoidError::RuleViolation) if invalid
    fn validate(&self, graph: &Graph, context: &RuleContext) -> Result<(), GraphoidError>;

    /// Check if this rule should run for the given operation
    /// Returns true if the rule needs to validate this operation
    fn should_run_on(&self, operation: &GraphOperation) -> bool;

    /// Attempt to clean existing violations (retroactive application)
    /// Returns Err if the rule doesn't support cleaning
    fn clean(&self, _graph: &mut Graph) -> Result<(), GraphoidError> {
        Err(GraphoidError::runtime(format!(
            "Rule '{}' does not support automatic cleaning",
            self.name()
        )))
    }
}

/// Rule that prevents cycles in the graph
pub struct NoCyclesRule;

impl NoCyclesRule {
    /// Create a new no-cycles rule
    pub fn new() -> Self {
        NoCyclesRule
    }

    /// Detect if adding an edge would create a cycle using DFS
    fn would_create_cycle(graph: &Graph, from: &str, to: &str) -> bool {
        // If we can reach 'from' starting from 'to', adding edge from->to creates a cycle
        let mut visited = HashSet::new();
        let mut stack = vec![to.to_string()];

        while let Some(current) = stack.pop() {
            if current == from {
                return true; // Found a path from 'to' to 'from', adding edge would create cycle
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            // Add all neighbors to the stack
            if let Some(node) = graph.nodes.get(&current) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        stack.push(neighbor_id.clone());
                    }
                }
            }
        }

        false
    }

    /// Check if the graph currently has any cycles
    fn has_cycle(graph: &Graph) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();

        for node_id in graph.nodes.keys() {
            if !visited.contains(node_id) {
                if Self::has_cycle_util(graph, node_id, &mut visited, &mut rec_stack) {
                    return true;
                }
            }
        }

        false
    }

    /// DFS utility for cycle detection
    fn has_cycle_util(
        graph: &Graph,
        node_id: &str,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());

        if let Some(node) = graph.nodes.get(node_id) {
            for neighbor_id in node.neighbors.keys() {
                if !visited.contains(neighbor_id) {
                    if Self::has_cycle_util(graph, neighbor_id, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor_id) {
                    return true;
                }
            }
        }

        rec_stack.remove(node_id);
        false
    }
}

impl Rule for NoCyclesRule {
    fn name(&self) -> &str {
        "no_cycles"
    }

    fn validate(&self, graph: &Graph, context: &RuleContext) -> Result<(), GraphoidError> {
        match &context.operation {
            GraphOperation::AddEdge { from, to, .. } => {
                if Self::would_create_cycle(graph, from, to) {
                    return Err(GraphoidError::RuleViolation {
                        rule: self.name().to_string(),
                        message: format!("Adding edge from '{}' to '{}' would create a cycle", from, to),
                    });
                }
            }
            _ => {
                // For other operations, check if graph has any cycles
                if Self::has_cycle(graph) {
                    return Err(GraphoidError::RuleViolation {
                        rule: self.name().to_string(),
                        message: "Graph contains a cycle".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    fn should_run_on(&self, operation: &GraphOperation) -> bool {
        // No-cycles rule should run on edge additions and removals
        matches!(operation, GraphOperation::AddEdge { .. } | GraphOperation::RemoveEdge { .. })
    }
}

/// Rule that ensures the graph has a single root node
pub struct SingleRootRule;

impl SingleRootRule {
    /// Create a new single-root rule
    pub fn new() -> Self {
        SingleRootRule
    }

    /// Find all root nodes (nodes with no incoming edges)
    fn find_roots(graph: &Graph) -> Vec<String> {
        let mut has_incoming = HashSet::new();

        // Mark all nodes that have incoming edges
        for node in graph.nodes.values() {
            for neighbor_id in node.neighbors.keys() {
                has_incoming.insert(neighbor_id.clone());
            }
        }

        // Roots are nodes without incoming edges
        graph.nodes.keys()
            .filter(|id| !has_incoming.contains(*id))
            .cloned()
            .collect()
    }
}

impl Rule for SingleRootRule {
    fn name(&self) -> &str {
        "single_root"
    }

    fn validate(&self, graph: &Graph, context: &RuleContext) -> Result<(), GraphoidError> {
        // Allow empty graphs (trees being built from scratch)
        if graph.node_count() == 0 {
            return Ok(());
        }

        // For AddEdge operations, we're connecting nodes, which is fine during construction
        // The key insight: adding edges REDUCES the number of roots (connects things)
        // We only enforce single root on REMOVAL operations that might break the tree
        if matches!(context.operation, GraphOperation::AddEdge { .. }) {
            return Ok(());
        }

        // For removal operations, check that we maintain single root
        let roots = Self::find_roots(graph);

        if roots.is_empty() {
            return Err(GraphoidError::RuleViolation {
                rule: self.name().to_string(),
                message: "Tree must have at least one root node (no incoming edges)".to_string(),
            });
        }

        if roots.len() > 1 {
            return Err(GraphoidError::RuleViolation {
                rule: self.name().to_string(),
                message: format!("Tree must have exactly one root, found {} roots: {:?}", roots.len(), roots),
            });
        }

        Ok(())
    }

    fn should_run_on(&self, operation: &GraphOperation) -> bool {
        // Single-root rule should only run on edge operations and removals
        // We skip AddNode because insert() adds nodes then edges, and we don't want
        // to fail when adding a child node before the parent edge is created
        matches!(
            operation,
            GraphOperation::AddEdge { .. } |
            GraphOperation::RemoveNode { .. } |
            GraphOperation::RemoveEdge { .. }
        )
    }
}

/// Rule that ensures the graph is connected
pub struct ConnectedRule;

impl ConnectedRule {
    /// Create a new connected rule
    pub fn new() -> Self {
        ConnectedRule
    }

    /// Check if the graph is connected (all nodes reachable from any starting node)
    fn is_connected(graph: &Graph) -> bool {
        if graph.nodes.is_empty() {
            return true; // Empty graph is considered connected
        }

        // Start from any node
        let start_node = graph.nodes.keys().next().unwrap();
        let mut visited = HashSet::new();
        let mut stack = vec![start_node.clone()];

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            // Add all neighbors (both directions for undirected, or need to check incoming edges too)
            if let Some(node) = graph.nodes.get(&current) {
                for neighbor_id in node.neighbors.keys() {
                    if !visited.contains(neighbor_id) {
                        stack.push(neighbor_id.clone());
                    }
                }
            }

            // For directed graphs, also check incoming edges
            if graph.graph_type == crate::values::graph::GraphType::Directed {
                for node in graph.nodes.values() {
                    if node.neighbors.contains_key(&current) && !visited.contains(&node.id) {
                        stack.push(node.id.clone());
                    }
                }
            }
        }

        // Graph is connected if we visited all nodes
        visited.len() == graph.nodes.len()
    }
}

impl Rule for ConnectedRule {
    fn name(&self) -> &str {
        "connected"
    }

    fn validate(&self, graph: &Graph, _context: &RuleContext) -> Result<(), GraphoidError> {
        if !Self::is_connected(graph) {
            return Err(GraphoidError::RuleViolation {
                rule: self.name().to_string(),
                message: "Graph must be connected (all nodes reachable)".to_string(),
            });
        }
        Ok(())
    }

    fn should_run_on(&self, operation: &GraphOperation) -> bool {
        // Connected rule should only run on removal operations
        // We skip AddNode and AddEdge because trees are built incrementally
        // and may be temporarily disconnected during construction
        matches!(
            operation,
            GraphOperation::RemoveNode { .. } |
            GraphOperation::RemoveEdge { .. }
        )
    }
}

/// Rule that limits the maximum degree (number of outgoing edges) of nodes
pub struct MaxDegreeRule {
    max_degree: usize,
}

impl MaxDegreeRule {
    /// Create a new max-degree rule with the specified limit
    pub fn new(max_degree: usize) -> Self {
        MaxDegreeRule { max_degree }
    }
}

impl Rule for MaxDegreeRule {
    fn name(&self) -> &str {
        "max_degree"
    }

    fn validate(&self, graph: &Graph, context: &RuleContext) -> Result<(), GraphoidError> {
        match &context.operation {
            GraphOperation::AddEdge { from, .. } => {
                // Check if adding this edge would exceed max degree
                if let Some(node) = graph.nodes.get(from) {
                    if node.neighbors.len() >= self.max_degree {
                        return Err(GraphoidError::RuleViolation {
                            rule: self.name().to_string(),
                            message: format!(
                                "Node '{}' already has {} edges, maximum is {}",
                                from, node.neighbors.len(), self.max_degree
                            ),
                        });
                    }
                }
            }
            _ => {
                // For other operations, check all nodes
                for (node_id, node) in &graph.nodes {
                    if node.neighbors.len() > self.max_degree {
                        return Err(GraphoidError::RuleViolation {
                            rule: self.name().to_string(),
                            message: format!(
                                "Node '{}' has {} edges, maximum is {}",
                                node_id, node.neighbors.len(), self.max_degree
                            ),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn should_run_on(&self, operation: &GraphOperation) -> bool {
        // Max-degree rule should run on edge additions
        matches!(operation, GraphOperation::AddEdge { .. })
    }
}

/// Rule that enforces binary tree structure (max 2 children per node)
pub struct BinaryTreeRule;

impl BinaryTreeRule {
    /// Create a new binary tree rule
    pub fn new() -> Self {
        BinaryTreeRule
    }
}

impl Rule for BinaryTreeRule {
    fn name(&self) -> &str {
        "binary_tree"
    }

    fn validate(&self, graph: &Graph, context: &RuleContext) -> Result<(), GraphoidError> {
        // Binary tree is just max degree of 2
        let max_degree_rule = MaxDegreeRule::new(2);
        max_degree_rule.validate(graph, context)
    }

    fn should_run_on(&self, operation: &GraphOperation) -> bool {
        matches!(operation, GraphOperation::AddEdge { .. })
    }
}

/// Rule that prevents duplicate values in the graph
pub struct NoDuplicatesRule;

impl NoDuplicatesRule {
    /// Create a new no-duplicates rule
    pub fn new() -> Self {
        NoDuplicatesRule
    }

    /// Check if the graph has any duplicate values
    fn has_duplicates(graph: &Graph) -> bool {
        let mut seen_values: Vec<&Value> = Vec::new();
        for node in graph.nodes.values() {
            if seen_values.contains(&&node.value) {
                return true; // Found a duplicate
            }
            seen_values.push(&node.value);
        }
        false
    }

    /// Check if adding a node would create a duplicate
    fn would_create_duplicate(graph: &Graph, value: &Value) -> bool {
        for node in graph.nodes.values() {
            if &node.value == value {
                return true;
            }
        }
        false
    }
}

impl Rule for NoDuplicatesRule {
    fn name(&self) -> &str {
        "no_duplicates"
    }

    fn validate(&self, graph: &Graph, context: &RuleContext) -> Result<(), GraphoidError> {
        match &context.operation {
            GraphOperation::AddNode { value, .. } => {
                if Self::would_create_duplicate(graph, value) {
                    return Err(GraphoidError::RuleViolation {
                        rule: self.name().to_string(),
                        message: format!("Value {:?} already exists in collection", value),
                    });
                }
            }
            _ => {
                // For other operations, check if graph has duplicates
                if Self::has_duplicates(graph) {
                    return Err(GraphoidError::RuleViolation {
                        rule: self.name().to_string(),
                        message: "Collection contains duplicate values".to_string(),
                    });
                }
            }
        }
        Ok(())
    }

    fn should_run_on(&self, operation: &GraphOperation) -> bool {
        // No-duplicates rule should run on node additions
        matches!(operation, GraphOperation::AddNode { .. })
    }

    fn clean(&self, graph: &mut Graph) -> Result<(), GraphoidError> {
        // Remove duplicate nodes, keeping first occurrence
        let mut seen_values: Vec<Value> = Vec::new();
        let mut to_remove: Vec<String> = Vec::new();

        for (id, node) in &graph.nodes {
            if seen_values.contains(&node.value) {
                to_remove.push(id.clone());
            } else {
                seen_values.push(node.value.clone());
            }
        }

        // Remove duplicate nodes
        for id in to_remove {
            graph.remove_node(&id)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::values::{Value, Graph};
    use crate::values::graph::GraphType;

    #[test]
    fn test_no_cycles_rule_allows_acyclic_edge() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();

        let rule = NoCyclesRule::new();
        let context = RuleContext::new(GraphOperation::AddEdge {
            from: "A".to_string(),
            to: "B".to_string(),
            edge_type: "edge".to_string(),
            properties: HashMap::new(),
        });

        assert!(rule.validate(&graph, &context).is_ok());
    }

    #[test]
    fn test_no_cycles_rule_detects_cycle() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), HashMap::new()).unwrap();

        let rule = NoCyclesRule::new();
        let context = RuleContext::new(GraphOperation::AddEdge {
            from: "B".to_string(),
            to: "A".to_string(),
            edge_type: "edge".to_string(),
            properties: HashMap::new(),
        });

        let result = rule.validate(&graph, &context);
        assert!(result.is_err());
        if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
            assert_eq!(rule_name, "no_cycles");
            assert!(message.contains("cycle"));
        } else {
            panic!("Expected RuleViolation error");
        }
    }

    #[test]
    fn test_single_root_rule_allows_single_root() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("root".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("child".to_string(), Value::Number(2.0)).unwrap();
        graph.add_edge("root", "child", "edge".to_string(), HashMap::new()).unwrap();

        let rule = SingleRootRule::new();
        let context = RuleContext::new(GraphOperation::AddNode {
            id: "another_child".to_string(),
            value: Value::Number(3.0),
        });

        assert!(rule.validate(&graph, &context).is_ok());
    }

    #[test]
    fn test_single_root_rule_detects_multiple_roots() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("root1".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("root2".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("child".to_string(), Value::Number(3.0)).unwrap();

        let rule = SingleRootRule::new();
        let context = RuleContext::new(GraphOperation::AddNode {
            id: "test".to_string(),
            value: Value::Number(4.0),
        });

        let result = rule.validate(&graph, &context);
        assert!(result.is_err());
        if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
            assert_eq!(rule_name, "single_root");
            assert!(message.contains("must have exactly one root"));
        }
    }

    #[test]
    fn test_max_degree_rule() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("A".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("B".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("C".to_string(), Value::Number(3.0)).unwrap();
        graph.add_edge("A", "B", "edge".to_string(), HashMap::new()).unwrap();

        let rule = MaxDegreeRule::new(1);
        let context = RuleContext::new(GraphOperation::AddEdge {
            from: "A".to_string(),
            to: "C".to_string(),
            edge_type: "edge".to_string(),
            properties: HashMap::new(),
        });

        let result = rule.validate(&graph, &context);
        assert!(result.is_err());
        if let Err(GraphoidError::RuleViolation { rule: rule_name, message }) = result {
            assert_eq!(rule_name, "max_degree");
            assert!(message.contains("maximum is 1"));
        }
    }

    #[test]
    fn test_binary_tree_rule_allows_two_children() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("root".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("left".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("right".to_string(), Value::Number(3.0)).unwrap();
        graph.add_edge("root", "left", "edge".to_string(), HashMap::new()).unwrap();

        let rule = BinaryTreeRule::new();
        let context = RuleContext::new(GraphOperation::AddEdge {
            from: "root".to_string(),
            to: "right".to_string(),
            edge_type: "edge".to_string(),
            properties: HashMap::new(),
        });

        assert!(rule.validate(&graph, &context).is_ok());
    }

    #[test]
    fn test_binary_tree_rule_rejects_three_children() {
        let mut graph = Graph::new(GraphType::Directed);
        graph.add_node("root".to_string(), Value::Number(1.0)).unwrap();
        graph.add_node("child1".to_string(), Value::Number(2.0)).unwrap();
        graph.add_node("child2".to_string(), Value::Number(3.0)).unwrap();
        graph.add_node("child3".to_string(), Value::Number(4.0)).unwrap();
        graph.add_edge("root", "child1", "edge".to_string(), HashMap::new()).unwrap();
        graph.add_edge("root", "child2", "edge".to_string(), HashMap::new()).unwrap();

        let rule = BinaryTreeRule::new();
        let context = RuleContext::new(GraphOperation::AddEdge {
            from: "root".to_string(),
            to: "child3".to_string(),
            edge_type: "edge".to_string(),
            properties: HashMap::new(),
        });

        let result = rule.validate(&graph, &context);
        assert!(result.is_err());
    }
}
