"""
Edge implementation for Glang graphs.
"""

from typing import Any, Optional, TYPE_CHECKING
import uuid

if TYPE_CHECKING:
    from .node import Node


class Edge:
    """
    A directed edge in a Glang graph.
    
    Represents a connection from one node to another, with optional
    weight and metadata for future extensibility.
    """
    
    def __init__(
        self,
        from_node: 'Node',
        to_node: 'Node',
        weight: float = 1.0,
        metadata: Optional[dict] = None,
        edge_id: Optional[str] = None
    ) -> None:
        """
        Initialize a new directed edge.
        
        Args:
            from_node: The source node
            to_node: The target node
            weight: Numeric weight for the edge (default: 1.0)
            metadata: Optional dictionary for storing additional data
            edge_id: Optional custom ID for the edge (generates UUID if None)
        """
        self.id = edge_id if edge_id is not None else str(uuid.uuid4())
        self.from_node = from_node
        self.to_node = to_node
        self.weight = weight
        self.metadata = metadata or {}
        
        # Register this edge with both nodes
        from_node.add_outgoing_edge(self)
        to_node.add_incoming_edge(self)
    
    def remove_from_nodes(self) -> None:
        """Remove this edge from its connected nodes."""
        self.from_node.remove_outgoing_edge(self)
        self.to_node.remove_incoming_edge(self)
    
    def get_metadata(self, key: str, default: Any = None) -> Any:
        """Get metadata value by key."""
        return self.metadata.get(key, default)
    
    def set_metadata(self, key: str, value: Any) -> None:
        """Set metadata value by key."""
        self.metadata[key] = value
    
    def has_metadata(self, key: str) -> bool:
        """Check if metadata key exists."""
        return key in self.metadata
    
    def reverse(self) -> 'Edge':
        """Create a new edge in the opposite direction."""
        return Edge(
            from_node=self.to_node,
            to_node=self.from_node,
            weight=self.weight,
            metadata=self.metadata.copy()
        )
    
    def is_self_loop(self) -> bool:
        """Check if this edge connects a node to itself."""
        return self.from_node == self.to_node
    
    def __str__(self) -> str:
        """String representation of the edge."""
        weight_str = f", weight={self.weight}" if self.weight != 1.0 else ""
        return f"Edge({self.from_node.id[:8]}... -> {self.to_node.id[:8]}...{weight_str})"
    
    def __repr__(self) -> str:
        """Detailed string representation of the edge."""
        return (
            f"Edge(id='{self.id}', from='{self.from_node.id}', "
            f"to='{self.to_node.id}', weight={self.weight}, metadata={self.metadata})"
        )
    
    def __hash__(self) -> int:
        """Hash based on edge ID."""
        return hash(self.id)
    
    def __eq__(self, other: object) -> bool:
        """Equality based on edge ID."""
        if not isinstance(other, Edge):
            return NotImplemented
        return self.id == other.id