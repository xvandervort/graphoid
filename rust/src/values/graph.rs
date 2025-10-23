//! Graph data structure implementation
//!
//! Graphoid's graph type uses index-free adjacency for O(1) neighbor lookups.
//! Each node stores direct pointers to its neighbors, avoiding index scans.

use std::collections::{HashMap, HashSet, VecDeque};
use super::Value;

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

/// Graph data structure with index-free adjacency
#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    /// Graph type (directed or undirected)
    pub graph_type: GraphType,
    /// Nodes by ID for O(1) lookup
    pub nodes: HashMap<String, GraphNode>,
    /// Active rulesets (e.g., "tree", "dag", "bst")
    /// Rules are not enforced yet - this is just storage for future implementation
    pub rulesets: Vec<String>,
}

impl Graph {
    /// Create a new empty graph
    pub fn new(graph_type: GraphType) -> Self {
        Graph {
            graph_type,
            nodes: HashMap::new(),
            rulesets: Vec::new(),
        }
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, id: String, value: Value) {
        self.nodes.insert(
            id.clone(),
            GraphNode {
                id,
                value,
                neighbors: HashMap::new(),
            },
        );
    }

    /// Add an edge between two nodes
    pub fn add_edge(&mut self, from: &str, to: &str, edge_type: String, properties: HashMap<String, Value>) {
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
    pub fn remove_node(&mut self, id: &str) -> Option<GraphNode> {
        // Remove the node
        let removed = self.nodes.remove(id);

        // Remove all edges pointing to this node
        for node in self.nodes.values_mut() {
            node.neighbors.remove(id);
        }

        removed
    }

    /// Remove an edge
    pub fn remove_edge(&mut self, from: &str, to: &str) -> bool {
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

        removed
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
    pub fn insert(&mut self, value: Value, parent: Option<&str>) -> String {
        // Generate unique node ID
        let node_id = format!("node_{}", self.nodes.len());

        // Add the node
        self.add_node(node_id.clone(), value);

        // If parent specified, add edge from parent to child
        if let Some(parent_id) = parent {
            self.add_edge(parent_id, &node_id, "child".to_string(), HashMap::new());
        }

        node_id
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
    // Ruleset methods (for tree{}, DAG{}, etc. support)
    // ========================================================================
    // ⚠️ PARTIAL IMPLEMENTATION (January 2025)
    // - Ruleset STORAGE works (stores ruleset names)
    // - Ruleset ENFORCEMENT does NOT work yet (no validation)
    // - Scheduled for completion: Phase 6 Week 2
    // - See: rust/RULESET_TODO.md for full status

    /// Apply a ruleset to this graph
    /// Returns self for method chaining
    ///
    /// ⚠️ NOTE: Rules are NOT enforced yet - this just stores the ruleset name
    /// Enforcement will be added in Phase 6 Week 2
    /// Future: Will validate graph structure against ruleset constraints
    pub fn with_ruleset(mut self, ruleset: String) -> Self {
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
}
