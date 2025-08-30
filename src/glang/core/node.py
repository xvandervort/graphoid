"""
Node implementation for Glang graphs.
"""

from typing import Any, Optional, Set
import uuid


class Node:
    """
    A node in a Glang graph.
    
    Each node has a unique identifier, can store arbitrary data,
    and maintains references to connected edges.
    """
    
    def __init__(self, data: Any = None, node_id: Optional[str] = None) -> None:
        """
        Initialize a new node.
        
        Args:
            data: The data stored in this node
            node_id: Optional custom ID for the node (generates UUID if None)
        """
        self.id = node_id if node_id is not None else str(uuid.uuid4())
        self.data = data
        self._outgoing_edges: Set['Edge'] = set()
        self._incoming_edges: Set['Edge'] = set()
    
    def add_outgoing_edge(self, edge: 'Edge') -> None:
        """Add an outgoing edge from this node."""
        self._outgoing_edges.add(edge)
    
    def add_incoming_edge(self, edge: 'Edge') -> None:
        """Add an incoming edge to this node."""
        self._incoming_edges.add(edge)
    
    def remove_outgoing_edge(self, edge: 'Edge') -> None:
        """Remove an outgoing edge from this node."""
        self._outgoing_edges.discard(edge)
    
    def remove_incoming_edge(self, edge: 'Edge') -> None:
        """Remove an incoming edge from this node."""
        self._incoming_edges.discard(edge)
    
    @property
    def outgoing_edges(self) -> Set['Edge']:
        """Get all outgoing edges from this node."""
        return self._outgoing_edges.copy()
    
    @property
    def incoming_edges(self) -> Set['Edge']:
        """Get all incoming edges to this node."""
        return self._incoming_edges.copy()
    
    @property
    def out_degree(self) -> int:
        """Number of outgoing edges."""
        return len(self._outgoing_edges)
    
    @property
    def in_degree(self) -> int:
        """Number of incoming edges."""
        return len(self._incoming_edges)
    
    @property
    def degree(self) -> int:
        """Total number of edges (in + out)."""
        return self.in_degree + self.out_degree
    
    def get_neighbors(self) -> Set['Node']:
        """Get all directly connected neighbor nodes."""
        neighbors = set()
        for edge in self._outgoing_edges:
            neighbors.add(edge.to_node)
        for edge in self._incoming_edges:
            neighbors.add(edge.from_node)
        return neighbors
    
    def get_successors(self) -> Set['Node']:
        """Get all nodes this node points to (via outgoing edges)."""
        return {edge.to_node for edge in self._outgoing_edges}
    
    def get_predecessors(self) -> Set['Node']:
        """Get all nodes that point to this node (via incoming edges)."""
        return {edge.from_node for edge in self._incoming_edges}
    
    def __str__(self) -> str:
        """String representation of the node."""
        return f"Node({self.id[:8]}..., data={self.data})"
    
    def __repr__(self) -> str:
        """Detailed string representation of the node."""
        return f"Node(id='{self.id}', data={self.data!r}, out_degree={self.out_degree}, in_degree={self.in_degree})"
    
    def __hash__(self) -> int:
        """Hash based on node ID."""
        return hash(self.id)
    
    def __eq__(self, other: object) -> bool:
        """Equality based on node ID."""
        if not isinstance(other, Node):
            return NotImplemented
        return self.id == other.id