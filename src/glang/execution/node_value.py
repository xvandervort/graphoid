"""
Node Value Wrapper for Glang

This module provides a wrapper for GraphNode objects so they can be used
as Glang values with method dispatch.
"""

from typing import Optional, Any, List
from .values import GlangValue
from .graph_foundation import GraphNode
from ..ast.nodes import SourcePosition


class NodeValue(GlangValue):
    """Wrapper for GraphNode to make it a Glang value."""

    def __init__(self, graph_node: GraphNode, position: Optional[SourcePosition] = None):
        super().__init__(position)
        self.graph_node = graph_node

    def get_type(self) -> str:
        return "node"

    def to_python(self) -> Any:
        # Return a dictionary representation of the node
        return {
            "id": self.graph_node.id,
            "value": self.graph_node.value.to_python() if self.graph_node.value else None,
            "neighbor_count": len(self.graph_node.neighbors)
        }

    def to_display_string(self) -> str:
        value_str = self.graph_node.value.to_display_string() if self.graph_node.value else "None"
        neighbor_count = len(self.graph_node.neighbors)
        return f"Node({value_str}, neighbors={neighbor_count})"

    # Expose GraphNode properties as GlangValue methods
    @property
    def neighbors(self) -> 'ListValue':
        """Get all neighboring nodes as a list."""
        from .graph_values import ListValue
        neighbor_nodes = [NodeValue(n) for n in self.graph_node.neighbors]
        return ListValue(neighbor_nodes)

    @property
    def value(self) -> GlangValue:
        """Get the value wrapped by this node."""
        return self.graph_node.value

    @property
    def container(self) -> GlangValue:
        """Get the container graph."""
        # For now, return the raw graph structure
        # TODO: Wrap in appropriate graph value type
        return self.graph_node.container

    @property
    def id(self) -> 'StringValue':
        """Get the unique node ID."""
        from .values import StringValue
        return StringValue(self.graph_node.id)

    def has_neighbor(self, other: 'NodeValue') -> 'BooleanValue':
        """Check if another node is a neighbor."""
        from .values import BooleanValue
        if not isinstance(other, NodeValue):
            return BooleanValue(False)
        return BooleanValue(self.graph_node.has_neighbor(other.graph_node))

    def path_to(self, target: 'NodeValue') -> Optional['ListValue']:
        """Find a path to another node."""
        if not isinstance(target, NodeValue):
            return None
        path = self.graph_node.path_to(target.graph_node)
        if path:
            from .graph_values import ListValue
            path_nodes = [NodeValue(n) for n in path]
            return ListValue(path_nodes)
        return None

    def distance_to(self, target: 'NodeValue') -> Optional['NumberValue']:
        """Get the distance to another node."""
        from .values import NumberValue
        if not isinstance(target, NodeValue):
            return None
        distance = self.graph_node.distance_to(target.graph_node)
        if distance is not None:
            return NumberValue(distance)
        return None

    def edges(self, direction: str = "out") -> 'ListValue':
        """Get edges from this node.

        Args:
            direction: "out" (default), "in", or "all"
        """
        from .graph_values import ListValue
        from .values import StringValue

        edge_tuples = []

        if direction in ("out", "all"):
            # Get outgoing edges
            for neighbor in self.graph_node.neighbors:
                edge_metadata = self.graph_node.get_edge_to(neighbor)
                if edge_metadata:
                    # Create readable edge description
                    neighbor_value = neighbor.value.to_display_string() if neighbor.value else "None"
                    edge_info = f"→ {neighbor_value}"
                    if edge_metadata.key is not None:
                        edge_info += f" [{edge_metadata.key}]"
                    edge_tuples.append(StringValue(edge_info))

        if direction in ("in", "all"):
            # Get incoming edges
            for neighbor in self.graph_node.neighbors:
                edge_metadata = neighbor.get_edge_to(self.graph_node)
                if edge_metadata:
                    # Create readable edge description
                    neighbor_value = neighbor.value.to_display_string() if neighbor.value else "None"
                    edge_info = f"← {neighbor_value}"
                    if edge_metadata.key is not None:
                        edge_info += f" [{edge_metadata.key}]"
                    edge_tuples.append(StringValue(edge_info))

        return ListValue(edge_tuples)