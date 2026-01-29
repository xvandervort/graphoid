//! Execution Graph module.
//!
//! Phase 16: Represents the AST as a graph with arena-allocated nodes.
//! Execution is graph traversal rather than pattern matching on enums.

pub mod arena;
pub mod node;
pub mod converter;
pub mod graph_executor;

use std::collections::HashMap;
use arena::{ArenaId, ArenaSet, NodeRef};
use node::{AstGraphNode, ExecEdgeType};

/// The execution graph: an AST represented as arena-allocated nodes with typed edges.
pub struct ExecutionGraph {
    nodes: ArenaSet<AstGraphNode>,
    edges: HashMap<NodeRef, Vec<(ExecEdgeType, NodeRef)>>,
    root: Option<NodeRef>,
}

impl ExecutionGraph {
    pub fn new() -> Self {
        ExecutionGraph {
            nodes: ArenaSet::new(),
            edges: HashMap::new(),
            root: None,
        }
    }

    /// Create a new arena (for a module, function, etc.).
    pub fn new_arena(&mut self) -> ArenaId {
        self.nodes.new_arena()
    }

    /// Add a node to the specified arena. Returns a NodeRef.
    pub fn add_node(&mut self, arena_id: ArenaId, node: AstGraphNode) -> NodeRef {
        let index = self.nodes.alloc(arena_id, node);
        NodeRef::new(arena_id, index)
    }

    /// Get a reference to a node.
    pub fn get_node(&self, node_ref: NodeRef) -> Option<&AstGraphNode> {
        self.nodes.get(node_ref)
    }

    /// Get a mutable reference to a node.
    pub fn get_node_mut(&mut self, node_ref: NodeRef) -> Option<&mut AstGraphNode> {
        self.nodes.get_mut(node_ref)
    }

    /// Add a directed edge from one node to another.
    pub fn add_edge(&mut self, from: NodeRef, edge_type: ExecEdgeType, to: NodeRef) {
        self.edges.entry(from).or_default().push((edge_type, to));
    }

    /// Get the target of a specific edge type from a node.
    /// Returns the first matching edge target.
    pub fn get_edge_target(&self, from: NodeRef, edge_type: &ExecEdgeType) -> Option<NodeRef> {
        self.edges.get(&from)
            .and_then(|edges| {
                edges.iter()
                    .find(|(et, _)| et == edge_type)
                    .map(|(_, target)| *target)
            })
    }

    /// Get all edges from a node.
    pub fn get_edges(&self, from: NodeRef) -> &[(ExecEdgeType, NodeRef)] {
        self.edges.get(&from)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get ordered edge targets for indexed edge types (Argument, Element, etc.).
    /// Filters edges by prefix name and sorts by index.
    pub fn get_ordered_edges(&self, from: NodeRef, prefix: &str) -> Vec<NodeRef> {
        let edges = match self.edges.get(&from) {
            Some(e) => e,
            None => return Vec::new(),
        };

        let mut indexed: Vec<(u32, NodeRef)> = edges.iter()
            .filter(|(et, _)| et.prefix() == prefix)
            .filter_map(|(et, target)| et.index().map(|i| (i, *target)))
            .collect();

        indexed.sort_by_key(|(i, _)| *i);
        indexed.into_iter().map(|(_, target)| target).collect()
    }

    /// Set the root node of the graph.
    pub fn set_root(&mut self, node_ref: NodeRef) {
        self.root = Some(node_ref);
    }

    /// Get the root node.
    pub fn root(&self) -> Option<NodeRef> {
        self.root
    }

    /// Total number of nodes across all arenas.
    pub fn node_count(&self) -> usize {
        self.nodes.total_node_count()
    }

    /// Drop an arena (for incremental re-parsing).
    pub fn drop_arena(&mut self, arena_id: ArenaId) {
        self.nodes.drop_arena(arena_id);
        // Remove edges from/to nodes in the dropped arena
        self.edges.retain(|from, _| from.arena_id != arena_id);
        for edges in self.edges.values_mut() {
            edges.retain(|(_, to)| to.arena_id != arena_id);
        }
    }
}
