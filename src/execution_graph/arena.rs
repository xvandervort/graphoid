//! Arena allocator for execution graph nodes.
//!
//! Provides per-scope arena allocation with `(ArenaId, NodeIndex)` references.
//! Each arena corresponds to a scope (module, function body) enabling
//! incremental re-parsing by dropping and rebuilding individual arenas.

use std::collections::HashMap;

/// Identifies a specific arena within an ArenaSet.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ArenaId(pub u32);

/// Index of a node within a specific arena.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeIndex(pub u32);

/// A reference to a node: (ArenaId, NodeIndex) pair.
/// This is the universal pointer type for the execution graph.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct NodeRef {
    pub arena_id: ArenaId,
    pub index: NodeIndex,
}

impl NodeRef {
    pub fn new(arena_id: ArenaId, index: NodeIndex) -> Self {
        NodeRef { arena_id, index }
    }
}

/// Manages multiple named arenas of type T.
///
/// Each arena is a contiguous Vec<T> with bump-pointer allocation.
/// Arenas can be individually dropped for incremental re-parsing.
pub struct ArenaSet<T> {
    arenas: HashMap<ArenaId, Vec<T>>,
    next_id: u32,
}

impl<T> ArenaSet<T> {
    pub fn new() -> Self {
        ArenaSet {
            arenas: HashMap::new(),
            next_id: 0,
        }
    }

    /// Create a new empty arena, returning its id.
    pub fn new_arena(&mut self) -> ArenaId {
        let id = ArenaId(self.next_id);
        self.next_id += 1;
        self.arenas.insert(id, Vec::new());
        id
    }

    /// Allocate a node in the specified arena. Returns the index within that arena.
    pub fn alloc(&mut self, arena_id: ArenaId, item: T) -> NodeIndex {
        let arena = self.arenas.get_mut(&arena_id)
            .expect("alloc: arena does not exist");
        let index = NodeIndex(arena.len() as u32);
        arena.push(item);
        index
    }

    /// Get a reference to the node at the given NodeRef.
    /// Returns None if the arena was dropped or the index is out of bounds.
    pub fn get(&self, node_ref: NodeRef) -> Option<&T> {
        self.arenas.get(&node_ref.arena_id)
            .and_then(|arena| arena.get(node_ref.index.0 as usize))
    }

    /// Get a mutable reference to the node at the given NodeRef.
    /// Returns None if the arena was dropped or the index is out of bounds.
    pub fn get_mut(&mut self, node_ref: NodeRef) -> Option<&mut T> {
        self.arenas.get_mut(&node_ref.arena_id)
            .and_then(|arena| arena.get_mut(node_ref.index.0 as usize))
    }

    /// Drop an entire arena, freeing all its nodes.
    /// After this call, any NodeRef pointing into this arena will return None.
    pub fn drop_arena(&mut self, arena_id: ArenaId) {
        self.arenas.remove(&arena_id);
    }

    /// Number of active arenas.
    pub fn arena_count(&self) -> usize {
        self.arenas.len()
    }

    /// Number of nodes in a specific arena. Returns None if arena doesn't exist.
    pub fn node_count(&self, arena_id: ArenaId) -> Option<usize> {
        self.arenas.get(&arena_id).map(|a| a.len())
    }

    /// Total number of nodes across all arenas.
    pub fn total_node_count(&self) -> usize {
        self.arenas.values().map(|a| a.len()).sum()
    }
}
