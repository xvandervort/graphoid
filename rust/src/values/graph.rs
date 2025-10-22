//! Graph data structure implementation
//!
//! Graphoid's graph type uses index-free adjacency for O(1) neighbor lookups.
//! Each node stores direct pointers to its neighbors, avoiding index scans.

use std::collections::HashMap;
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
}

impl Graph {
    /// Create a new empty graph
    pub fn new(graph_type: GraphType) -> Self {
        Graph {
            graph_type,
            nodes: HashMap::new(),
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
}
