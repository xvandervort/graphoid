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


class MetadataLayer:
    """Universal metadata layer for all graph structures.

    Always present on every graph, provides extensible key-value metadata storage.
    Handles element names, units, source info, timestamps, etc.
    """

    def __init__(self):
        self.properties: Dict[str, Any] = {}

    def set(self, key: str, value: Any) -> None:
        """Set a metadata property."""
        self.properties[key] = value

    def get(self, key: str, default: Any = None) -> Any:
        """Get a metadata property with graceful fallback."""
        return self.properties.get(key, default)

    def has(self, key: str) -> bool:
        """Check if a metadata property exists."""
        return key in self.properties

    def remove(self, key: str) -> bool:
        """Remove a metadata property. Returns True if it existed."""
        if key in self.properties:
            del self.properties[key]
            return True
        return False

    def clear(self) -> None:
        """Clear all metadata."""
        self.properties.clear()

    def keys(self) -> List[str]:
        """Get all metadata keys."""
        return list(self.properties.keys())

    def copy(self) -> 'MetadataLayer':
        """Create a copy of this metadata layer."""
        new_layer = MetadataLayer()
        new_layer.properties = self.properties.copy()
        return new_layer


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

        # Set bidirectional link: value knows its node, node knows its value
        value._graph_node = self

    def add_edge_to(self, target: 'GraphNode', metadata: EdgeMetadata) -> None:
        """Add an outgoing edge to another node."""
        # Validate edge operation through control layer (if graph has one)
        if self._graph and hasattr(self._graph, 'control_layer'):
            is_valid, reason = self._graph.control_layer.validate_edge_operation(self, target, metadata)
            if not is_valid:
                from .control_layer import RuleViolationError
                raise RuleViolationError("edge_validation", reason)

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

    @property
    def neighbors(self) -> List['GraphNode']:
        """Get all neighboring nodes (both outgoing and incoming connections)."""
        # Combine outgoing and incoming neighbors, removing duplicates
        all_neighbors = set()
        for target, _ in self._outgoing.values():
            all_neighbors.add(target)
        for source, _ in self._incoming.values():
            all_neighbors.add(source)
        return list(all_neighbors)

    @property
    def container(self) -> Optional['GraphStructure']:
        """Get the graph structure that contains this node."""
        return self._graph

    @property
    def id(self) -> str:
        """Get the unique identifier for this node."""
        return self.node_id

    def has_neighbor(self, other: 'GraphNode') -> bool:
        """Check if another node is a neighbor."""
        return self.has_edge_to(other) or other.has_edge_to(self)

    def path_to(self, target: 'GraphNode') -> Optional[List['GraphNode']]:
        """Find a path from this node to the target node."""
        if self._graph:
            return self._graph.shortest_path(self, target)
        return None

    def distance_to(self, target: 'GraphNode') -> Optional[int]:
        """Get the shortest distance to another node."""
        path = self.path_to(target)
        return len(path) - 1 if path else None

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
        # Universal metadata layer - always present
        self.metadata = MetadataLayer()

        # Control layer - Layer 3 governance (import here to avoid circular imports)
        from .control_layer import ControlLayer
        self.control_layer = ControlLayer(self)

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

    # Element naming methods (using metadata layer)
    def set_names(self, names: List[Optional[str]]) -> None:
        """Set names for all elements (None for unnamed elements)."""
        if len(names) != len(self.sequence_order):
            raise ValueError(f"Names list length ({len(names)}) doesn't match sequence length ({len(self.sequence_order)})")

        self.metadata.set("element_names", names)

    def get_names(self) -> List[Optional[str]]:
        """Get names for all elements (None for unnamed elements)."""
        default_names = [None] * len(self.sequence_order)
        return self.metadata.get("element_names", default_names)

    def get_name(self, index: int) -> Optional[str]:
        """Get the name of an element at given index."""
        names = self.get_names()
        if 0 <= index < len(names):
            return names[index]
        return None

    def set_name(self, index: int, name: Optional[str]) -> bool:
        """Set the name for a single element."""
        if not (0 <= index < len(self.sequence_order)):
            return False

        names = self.get_names()
        names[index] = name
        self.metadata.set("element_names", names)
        return True

    def get_index_by_name(self, name: str) -> Optional[int]:
        """Get the index of the first element with the given name."""
        names = self.get_names()
        try:
            return names.index(name)
        except ValueError:
            return None

    def get_value_by_name(self, name: str) -> Optional['GlangValue']:
        """Get the value of an element by its name."""
        index = self.get_index_by_name(name)
        if index is not None:
            return self.get_at_index(index)
        return None

    def has_names(self) -> bool:
        """Check if any elements have names."""
        names = self.get_names()
        return any(name is not None for name in names)


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

    # Element naming methods (using metadata layer) - for hash elements by insertion order
    def set_names(self, names: List[Optional[str]]) -> None:
        """Set names for all hash elements by insertion order (None for unnamed elements)."""
        if len(names) != len(self.key_to_node):
            raise ValueError(f"Names list length ({len(names)}) doesn't match hash size ({len(self.key_to_node)})")

        self.metadata.set("element_names", names)

    def get_names(self) -> List[Optional[str]]:
        """Get names for all hash elements by insertion order (None for unnamed elements)."""
        default_names = [None] * len(self.key_to_node)
        return self.metadata.get("element_names", default_names)

    def get_name(self, index: int) -> Optional[str]:
        """Get the name of a hash element at given insertion order index."""
        names = self.get_names()
        if 0 <= index < len(names):
            return names[index]
        return None

    def set_name(self, index: int, name: Optional[str]) -> bool:
        """Set the name for a single hash element by insertion order index."""
        if not (0 <= index < len(self.key_to_node)):
            return False

        names = self.get_names()
        names[index] = name
        self.metadata.set("element_names", names)
        return True

    def has_names(self) -> bool:
        """Check if any hash elements have names."""
        names = self.get_names()
        return any(name is not None for name in names)

    def get_key_by_name(self, name: str) -> Optional[str]:
        """Get the actual key for a given name."""
        names = self.get_names()
        keys = list(self.key_to_node.keys())

        for i, element_name in enumerate(names):
            if element_name == name and i < len(keys):
                return keys[i]
        return None

    def get_value_by_name(self, name: str) -> Optional['GlangValue']:
        """Get value by name (returns None if name not found)."""
        key = self.get_key_by_name(name)
        if key:
            return self.get(key)
        return None

    def set_value_by_name(self, name: str, value: 'GlangValue') -> bool:
        """Set value by name (returns False if name not found)."""
        key = self.get_key_by_name(name)
        if key:
            self.set(key, value)
            return True
        return False