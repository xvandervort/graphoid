"""
Graph Foundation for Glang

This module provides the core graph structures that replace lists and hashes.
All collections in Glang are true graph structures with nodes and edges.

Key principles:
- Lists are sequential graphs (each node connects to the next)
- Hashes are keyed graphs (root node connects to value nodes via key edges)
- All values are graph nodes that can be connected to other nodes
- Edges carry metadata (like keys, indices, or relationships)
"""

from typing import Dict, List, Set, Optional, Any, Union, Iterator, Tuple
from dataclasses import dataclass, field
from abc import ABC, abstractmethod
from enum import Enum
import uuid


class EdgeType(Enum):
    """Types of edges in the graph."""
    SEQUENTIAL = "sequential"  # For list-like connections (0, 1, 2, ...)
    KEYED = "keyed"           # For hash-like connections (string keys)
    NAMED = "named"           # For general named relationships
    TYPED = "typed"           # For type-based relationships


@dataclass
class EdgeMetadata:
    """Metadata attached to edges."""
    edge_type: EdgeType
    key: Optional[Union[str, int]] = None
    weight: float = 1.0
    bidirectional: bool = False
    properties: Dict[str, Any] = field(default_factory=dict)

    def __str__(self) -> str:
        if self.edge_type == EdgeType.SEQUENTIAL:
            return f"[{self.key}]"
        elif self.edge_type == EdgeType.KEYED:
            return f'"{self.key}"'
        elif self.edge_type == EdgeType.NAMED:
            return str(self.key) if self.key else "unnamed"
        else:
            return f"{self.edge_type.value}"


class GraphNode:
    """A node in the graph that can hold a value and connect to other nodes."""

    def __init__(self, value: 'GlangValue', node_id: Optional[str] = None):
        self.value = value
        self.node_id = node_id or str(uuid.uuid4())

        # Outgoing edges: node_id -> (target_node, metadata)
        self._outgoing: Dict[str, Tuple['GraphNode', EdgeMetadata]] = {}

        # Incoming edges: node_id -> (source_node, metadata)
        self._incoming: Dict[str, Tuple['GraphNode', EdgeMetadata]] = {}

        # Parent graph reference
        self._graph: Optional['GraphStructure'] = None

    def add_edge_to(self, target: 'GraphNode', metadata: EdgeMetadata) -> None:
        """Add an outgoing edge to another node."""
        edge_id = f"{self.node_id}->{target.node_id}"
        self._outgoing[edge_id] = (target, metadata)
        target._incoming[edge_id] = (self, metadata)

        # If bidirectional, add reverse edge
        if metadata.bidirectional:
            reverse_edge_id = f"{target.node_id}->{self.node_id}"
            target._outgoing[reverse_edge_id] = (self, metadata)
            self._incoming[reverse_edge_id] = (target, metadata)

    def remove_edge_to(self, target: 'GraphNode') -> bool:
        """Remove an edge to another node. Returns True if edge was found and removed."""
        edge_id = f"{self.node_id}->{target.node_id}"
        if edge_id in self._outgoing:
            metadata = self._outgoing[edge_id][1]
            del self._outgoing[edge_id]
            del target._incoming[edge_id]

            # Remove reverse edge if bidirectional
            if metadata.bidirectional:
                reverse_edge_id = f"{target.node_id}->{self.node_id}"
                if reverse_edge_id in target._outgoing:
                    del target._outgoing[reverse_edge_id]
                    del self._incoming[reverse_edge_id]
            return True
        return False

    def get_neighbors(self, edge_type: Optional[EdgeType] = None) -> List['GraphNode']:
        """Get all nodes this node connects to, optionally filtered by edge type."""
        neighbors = []
        for target, metadata in self._outgoing.values():
            if edge_type is None or metadata.edge_type == edge_type:
                neighbors.append(target)
        return neighbors

    def get_incoming_neighbors(self, edge_type: Optional[EdgeType] = None) -> List['GraphNode']:
        """Get all nodes that connect to this node, optionally filtered by edge type."""
        neighbors = []
        for source, metadata in self._incoming.values():
            if edge_type is None or metadata.edge_type == edge_type:
                neighbors.append(source)
        return neighbors

    def get_edge_to(self, target: 'GraphNode') -> Optional[EdgeMetadata]:
        """Get edge metadata to a specific target node."""
        edge_id = f"{self.node_id}->{target.node_id}"
        if edge_id in self._outgoing:
            return self._outgoing[edge_id][1]
        return None

    def has_edge_to(self, target: 'GraphNode') -> bool:
        """Check if this node has an edge to the target node."""
        edge_id = f"{self.node_id}->{target.node_id}"
        return edge_id in self._outgoing

    def get_edges_by_key(self, key: Union[str, int]) -> List[Tuple['GraphNode', EdgeMetadata]]:
        """Get all edges with a specific key."""
        edges = []
        for target, metadata in self._outgoing.values():
            if metadata.key == key:
                edges.append((target, metadata))
        return edges

    def get_edge_by_key(self, key: Union[str, int]) -> Optional[Tuple['GraphNode', EdgeMetadata]]:
        """Get the first edge with a specific key."""
        edges = self.get_edges_by_key(key)
        return edges[0] if edges else None

    def __str__(self) -> str:
        return f"Node({self.value.to_display_string()})"

    def __repr__(self) -> str:
        edge_count = len(self._outgoing)
        return f"GraphNode(value={self.value.to_display_string()}, edges={edge_count})"


class GraphStructure:
    """A graph structure that manages nodes and their relationships."""

    def __init__(self, root_node: Optional[GraphNode] = None):
        self.nodes: Dict[str, GraphNode] = {}
        self.root_node = root_node

        if root_node:
            self.add_node(root_node)

    def add_node(self, node: GraphNode) -> None:
        """Add a node to the graph."""
        self.nodes[node.node_id] = node
        node._graph = self

    def remove_node(self, node: GraphNode) -> None:
        """Remove a node and all its edges from the graph."""
        # Remove all incoming edges
        for edge_id in list(node._incoming.keys()):
            source_node = node._incoming[edge_id][0]
            source_node.remove_edge_to(node)

        # Remove all outgoing edges
        for edge_id in list(node._outgoing.keys()):
            target_node = node._outgoing[edge_id][0]
            node.remove_edge_to(target_node)

        # Remove from graph
        if node.node_id in self.nodes:
            del self.nodes[node.node_id]
        node._graph = None

    def get_node(self, node_id: str) -> Optional[GraphNode]:
        """Get a node by its ID."""
        return self.nodes.get(node_id)

    def find_nodes_by_value(self, value: 'GlangValue') -> List[GraphNode]:
        """Find all nodes containing a specific value."""
        return [node for node in self.nodes.values()
                if node.value == value]

    def get_all_nodes(self) -> List[GraphNode]:
        """Get all nodes in the graph."""
        return list(self.nodes.values())

    def get_connected_components(self) -> List[List[GraphNode]]:
        """Get all connected components in the graph."""
        visited = set()
        components = []

        for node in self.nodes.values():
            if node.node_id not in visited:
                component = []
                self._dfs_component(node, visited, component)
                components.append(component)

        return components

    def _dfs_component(self, node: GraphNode, visited: Set[str], component: List[GraphNode]) -> None:
        """Depth-first search for finding connected components."""
        visited.add(node.node_id)
        component.append(node)

        # Visit all neighbors (both outgoing and incoming)
        for neighbor in node.get_neighbors():
            if neighbor.node_id not in visited:
                self._dfs_component(neighbor, visited, component)

        for neighbor in node.get_incoming_neighbors():
            if neighbor.node_id not in visited:
                self._dfs_component(neighbor, visited, component)

    def shortest_path(self, start: GraphNode, end: GraphNode) -> Optional[List[GraphNode]]:
        """Find shortest path between two nodes using BFS."""
        if start == end:
            return [start]

        queue = [(start, [start])]
        visited = {start.node_id}

        while queue:
            current, path = queue.pop(0)

            for neighbor in current.get_neighbors():
                if neighbor.node_id not in visited:
                    new_path = path + [neighbor]
                    if neighbor == end:
                        return new_path

                    queue.append((neighbor, new_path))
                    visited.add(neighbor.node_id)

        return None

    def is_connected(self, start: GraphNode, end: GraphNode) -> bool:
        """Check if there's a path between two nodes."""
        return self.shortest_path(start, end) is not None

    def __len__(self) -> int:
        return len(self.nodes)

    def __str__(self) -> str:
        return f"Graph({len(self.nodes)} nodes)"


class SequentialGraph(GraphStructure):
    """A graph structure that maintains sequential ordering (like a list)."""

    def __init__(self, values: Optional[List['GlangValue']] = None):
        super().__init__()
        self.sequence_order: List[GraphNode] = []

        if values:
            self._build_from_values(values)

    def _build_from_values(self, values: List['GlangValue']) -> None:
        """Build a sequential graph from a list of values."""
        prev_node = None

        for i, value in enumerate(values):
            node = GraphNode(value)
            self.add_node(node)
            self.sequence_order.append(node)

            if prev_node:
                # Connect previous node to current with sequential edge
                metadata = EdgeMetadata(
                    edge_type=EdgeType.SEQUENTIAL,
                    key=i-1
                )
                prev_node.add_edge_to(node, metadata)

            prev_node = node

        # Set root to first node if any
        if self.sequence_order:
            self.root_node = self.sequence_order[0]

    def append(self, value: 'GlangValue') -> None:
        """Append a value to the end of the sequence."""
        node = GraphNode(value)
        self.add_node(node)

        # Connect last node to new node
        if self.sequence_order:
            last_node = self.sequence_order[-1]
            metadata = EdgeMetadata(
                edge_type=EdgeType.SEQUENTIAL,
                key=len(self.sequence_order) - 1
            )
            last_node.add_edge_to(node, metadata)
        else:
            # First node becomes root
            self.root_node = node

        self.sequence_order.append(node)

    def get_at_index(self, index: int) -> Optional['GlangValue']:
        """Get value at a specific index."""
        # Handle negative indexing like Python lists
        length = len(self.sequence_order)
        if index < 0:
            index = length + index

        if 0 <= index < length:
            return self.sequence_order[index].value
        return None

    def set_at_index(self, index: int, value: 'GlangValue') -> bool:
        """Set value at a specific index."""
        # Handle negative indexing like Python lists
        length = len(self.sequence_order)
        if index < 0:
            index = length + index

        if 0 <= index < length:
            self.sequence_order[index].value = value
            return True
        return False

    def get_values(self) -> List['GlangValue']:
        """Get all values in sequential order."""
        return [node.value for node in self.sequence_order]

    def __len__(self) -> int:
        return len(self.sequence_order)


class KeyedGraph(GraphStructure):
    """A graph structure that uses string keys to access values (like a hash)."""

    def __init__(self, pairs: Optional[List[Tuple[str, 'GlangValue']]] = None):
        # Create a root node for the hash itself
        from .values import NoneValue  # Import here to avoid circular imports
        root_value = NoneValue()  # Placeholder value for root
        root_node = GraphNode(root_value)

        super().__init__(root_node)
        self.key_to_node: Dict[str, GraphNode] = {}

        if pairs:
            self._build_from_pairs(pairs)

    def _build_from_pairs(self, pairs: List[Tuple[str, 'GlangValue']]) -> None:
        """Build a keyed graph from key-value pairs."""
        for key, value in pairs:
            self.set(key, value)

    def set(self, key: str, value: 'GlangValue') -> None:
        """Set a key-value pair."""
        # Remove existing key if present
        if key in self.key_to_node:
            old_node = self.key_to_node[key]
            self.root_node.remove_edge_to(old_node)
            self.remove_node(old_node)

        # Create new node for value
        node = GraphNode(value)
        self.add_node(node)

        # Connect root to value node with keyed edge
        metadata = EdgeMetadata(
            edge_type=EdgeType.KEYED,
            key=key
        )
        self.root_node.add_edge_to(node, metadata)

        # Track the key-to-node mapping
        self.key_to_node[key] = node

    def get(self, key: str) -> Optional['GlangValue']:
        """Get value by key."""
        if key in self.key_to_node:
            return self.key_to_node[key].value
        return None

    def has_key(self, key: str) -> bool:
        """Check if key exists."""
        return key in self.key_to_node

    def remove(self, key: str) -> bool:
        """Remove a key-value pair."""
        if key in self.key_to_node:
            node = self.key_to_node[key]
            self.root_node.remove_edge_to(node)
            self.remove_node(node)
            del self.key_to_node[key]
            return True
        return False

    def keys(self) -> List[str]:
        """Get all keys."""
        return list(self.key_to_node.keys())

    def values(self) -> List['GlangValue']:
        """Get all values."""
        return [node.value for node in self.key_to_node.values()]

    def items(self) -> List[Tuple[str, 'GlangValue']]:
        """Get all key-value pairs."""
        return [(key, node.value) for key, node in self.key_to_node.items()]

    def __len__(self) -> int:
        return len(self.key_to_node)