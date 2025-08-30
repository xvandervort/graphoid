"""
Graph type definitions and enumerations for Glang.
"""

from enum import Enum, auto


class GraphType(Enum):
    """
    Enumeration of different graph types supported by Glang.
    
    This allows for type-specific behavior and validation while
    maintaining a unified graph interface.
    """
    
    LINEAR = auto()      # Simple linked list: A -> B -> C -> D
    TREE = auto()        # Hierarchical structure with parent-child relationships
    CYCLIC = auto()      # Contains cycles/loops
    WEIGHTED = auto()    # Edges have meaningful weights
    DIRECTED = auto()    # General directed graph
    UNDIRECTED = auto()  # Bidirectional edges (implemented as paired directed edges)
    
    def __str__(self) -> str:
        """Human-readable string representation."""
        return self.name.lower()
    
    @classmethod
    def from_string(cls, type_str: str) -> 'GraphType':
        """Create GraphType from string representation."""
        type_str = type_str.upper()
        for graph_type in cls:
            if graph_type.name == type_str:
                return graph_type
        raise ValueError(f"Unknown graph type: {type_str}")
    
    def is_linear(self) -> bool:
        """Check if this is a linear graph type."""
        return self == GraphType.LINEAR
    
    def is_hierarchical(self) -> bool:
        """Check if this is a hierarchical graph type."""
        return self == GraphType.TREE
    
    def allows_cycles(self) -> bool:
        """Check if this graph type allows cycles."""
        return self in {GraphType.CYCLIC, GraphType.DIRECTED, GraphType.UNDIRECTED}
    
    def is_weighted(self) -> bool:
        """Check if this graph type uses meaningful weights."""
        return self == GraphType.WEIGHTED