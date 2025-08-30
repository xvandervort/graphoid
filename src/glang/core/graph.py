"""
Graph implementation for Glang.
"""

from typing import Any, List, Set, Optional, Iterator, Union
from .node import Node
from .edge import Edge
from .graph_types import GraphType


class Graph:
    """
    A graph data structure for Glang.
    
    Supports various graph types and provides a unified interface
    for graph operations. Designed for extensibility and performance.
    """
    
    def __init__(self, graph_type: GraphType = GraphType.DIRECTED) -> None:
        """
        Initialize a new graph.
        
        Args:
            graph_type: The type of graph to create
        """
        self.graph_type = graph_type
        self._nodes: Set[Node] = set()
        self._edges: Set[Edge] = set()
        self._head: Optional[Node] = None  # For linear graphs
        self._tail: Optional[Node] = None  # For linear graphs
        self._size = 0
    
    def add_node(self, node: Node) -> None:
        """Add a node to the graph."""
        if node not in self._nodes:
            self._nodes.add(node)
            self._size += 1
    
    def create_node(self, data: Any = None, node_id: Optional[str] = None) -> Node:
        """Create and add a new node to the graph."""
        node = Node(data=data, node_id=node_id)
        self.add_node(node)
        return node
    
    def remove_node(self, node: Node) -> None:
        """Remove a node and all its edges from the graph."""
        if node not in self._nodes:
            return
        
        # Remove all edges connected to this node
        edges_to_remove = list(node.outgoing_edges | node.incoming_edges)
        for edge in edges_to_remove:
            self.remove_edge(edge)
        
        self._nodes.remove(node)
        self._size -= 1
        
        # Update head/tail pointers for linear graphs
        if self.graph_type.is_linear():
            if self._head == node:
                self._head = None
            if self._tail == node:
                self._tail = None
    
    def add_edge(self, from_node: Node, to_node: Node, **kwargs) -> Edge:
        """Create and add a new edge between two nodes."""
        # Ensure both nodes are in the graph
        self.add_node(from_node)
        self.add_node(to_node)
        
        edge = Edge(from_node=from_node, to_node=to_node, **kwargs)
        self._edges.add(edge)
        return edge
    
    def remove_edge(self, edge: Edge) -> None:
        """Remove an edge from the graph."""
        if edge in self._edges:
            edge.remove_from_nodes()
            self._edges.remove(edge)
    
    @property
    def nodes(self) -> Set[Node]:
        """Get all nodes in the graph."""
        return self._nodes.copy()
    
    @property
    def edges(self) -> Set[Edge]:
        """Get all edges in the graph."""
        return self._edges.copy()
    
    @property
    def size(self) -> int:
        """Number of nodes in the graph."""
        return self._size
    
    @property
    def edge_count(self) -> int:
        """Number of edges in the graph."""
        return len(self._edges)
    
    def is_empty(self) -> bool:
        """Check if the graph has no nodes."""
        return self._size == 0
    
    def get_node_by_id(self, node_id: str) -> Optional[Node]:
        """Find a node by its ID."""
        for node in self._nodes:
            if node.id == node_id:
                return node
        return None
    
    def has_node(self, node: Node) -> bool:
        """Check if a node exists in the graph."""
        return node in self._nodes
    
    def has_edge(self, from_node: Node, to_node: Node) -> bool:
        """Check if an edge exists between two nodes."""
        for edge in from_node.outgoing_edges:
            if edge.to_node == to_node:
                return True
        return False
    
    def get_edge(self, from_node: Node, to_node: Node) -> Optional[Edge]:
        """Get the edge between two nodes (first one found)."""
        for edge in from_node.outgoing_edges:
            if edge.to_node == to_node:
                return edge
        return None
    
    def clear(self) -> None:
        """Remove all nodes and edges from the graph."""
        # Clear all edge references from nodes
        for edge in list(self._edges):
            self.remove_edge(edge)
        
        self._nodes.clear()
        self._edges.clear()
        self._head = None
        self._tail = None
        self._size = 0
    
    def __len__(self) -> int:
        """Number of nodes in the graph."""
        return self._size
    
    def __contains__(self, node: Node) -> bool:
        """Check if a node is in the graph."""
        return node in self._nodes
    
    def __str__(self) -> str:
        """String representation of the graph."""
        return f"Graph(type={self.graph_type}, nodes={self._size}, edges={len(self._edges)})"
    
    def __repr__(self) -> str:
        """Detailed string representation of the graph."""
        return (
            f"Graph(graph_type={self.graph_type}, size={self._size}, "
            f"edge_count={len(self._edges)}, head={self._head}, tail={self._tail})"
        )
    
    # Linear list operations (for LINEAR graph type)
    
    @classmethod
    def from_list(cls, data_list: List[Any]) -> 'Graph':
        """Create a linear graph from a Python list."""
        graph = cls(GraphType.LINEAR)
        for item in data_list:
            graph.append(item)
        return graph
    
    def append(self, data: Any) -> Node:
        """Add a new node to the end of the linear graph."""
        if not self.graph_type.is_linear():
            raise ValueError("append() only supported for LINEAR graphs")
        
        new_node = self.create_node(data)
        
        if self._tail is not None:
            # Connect the current tail to the new node
            self.add_edge(self._tail, new_node)
        else:
            # This is the first node
            self._head = new_node
        
        self._tail = new_node
        return new_node
    
    def prepend(self, data: Any) -> Node:
        """Add a new node to the beginning of the linear graph."""
        if not self.graph_type.is_linear():
            raise ValueError("prepend() only supported for LINEAR graphs")
        
        new_node = self.create_node(data)
        
        if self._head is not None:
            # Connect the new node to the current head
            self.add_edge(new_node, self._head)
        else:
            # This is the first node
            self._tail = new_node
        
        self._head = new_node
        return new_node
    
    def insert(self, index: int, data: Any) -> Node:
        """Insert a new node at the specified index."""
        if not self.graph_type.is_linear():
            raise ValueError("insert() only supported for LINEAR graphs")
        
        if index < 0:
            index = max(0, self._size + index + 1)
        
        if index == 0:
            return self.prepend(data)
        
        if index >= self._size:
            return self.append(data)
        
        # Find the node at index-1
        current = self._head
        for _ in range(index - 1):
            if current is None:
                break
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        if current is None:
            return self.append(data)
        
        # Get the next node
        successors = current.get_successors()
        next_node = next(iter(successors)) if successors else None
        
        # Create new node
        new_node = self.create_node(data)
        
        # Remove edge from current to next (if exists)
        if next_node:
            edge = self.get_edge(current, next_node)
            if edge:
                self.remove_edge(edge)
            # Connect new node to next
            self.add_edge(new_node, next_node)
        else:
            # Inserting at the end
            self._tail = new_node
        
        # Connect current to new node
        self.add_edge(current, new_node)
        
        return new_node
    
    def delete(self, index: int) -> Optional[Any]:
        """Delete the node at the specified index and return its data."""
        if not self.graph_type.is_linear():
            raise ValueError("delete() only supported for LINEAR graphs")
        
        if index < 0:
            index = self._size + index
        
        if index < 0 or index >= self._size:
            return None
        
        # Find the node at the specified index
        current = self._head
        prev = None
        
        for i in range(index):
            if current is None:
                return None
            prev = current
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        if current is None:
            return None
        
        data = current.data
        
        # Get the next node
        successors = current.get_successors()
        next_node = next(iter(successors)) if successors else None
        
        # Update connections
        if prev is not None:
            # Remove edge from prev to current
            edge = self.get_edge(prev, current)
            if edge:
                self.remove_edge(edge)
            
            # Connect prev to next (if exists)
            if next_node:
                self.add_edge(prev, next_node)
        else:
            # Removing the head
            self._head = next_node
        
        if next_node is None:
            # Removing the tail
            self._tail = prev
        
        # Remove the node
        self.remove_node(current)
        
        return data
    
    def get(self, index: int) -> Optional[Any]:
        """Get the data at the specified index."""
        if not self.graph_type.is_linear():
            raise ValueError("get() only supported for LINEAR graphs")
        
        if index < 0:
            index = self._size + index
        
        if index < 0 or index >= self._size:
            return None
        
        current = self._head
        for _ in range(index):
            if current is None:
                return None
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        return current.data if current else None
    
    def set(self, index: int, data: Any) -> bool:
        """Set the data at the specified index."""
        if not self.graph_type.is_linear():
            raise ValueError("set() only supported for LINEAR graphs")
        
        if index < 0:
            index = self._size + index
        
        if index < 0 or index >= self._size:
            return False
        
        current = self._head
        for _ in range(index):
            if current is None:
                return False
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
        
        if current is not None:
            current.data = data
            return True
        
        return False
    
    # Traversal and conversion methods
    
    def __iter__(self) -> Iterator[Node]:
        """Iterate over nodes in the graph."""
        if self.graph_type.is_linear():
            # Linear traversal from head to tail
            current = self._head
            while current is not None:
                yield current
                successors = current.get_successors()
                current = next(iter(successors)) if successors else None
        else:
            # For non-linear graphs, just iterate over all nodes
            yield from self._nodes
    
    def traverse(self) -> List[Any]:
        """Get a list of data values in traversal order."""
        return [node.data for node in self]
    
    def to_list(self) -> List[Any]:
        """Convert the graph to a Python list (for linear graphs)."""
        if not self.graph_type.is_linear():
            raise ValueError("to_list() only supported for LINEAR graphs")
        return self.traverse()
    
    def reverse(self) -> None:
        """Reverse the order of a linear graph."""
        if not self.graph_type.is_linear():
            raise ValueError("reverse() only supported for LINEAR graphs")
        
        if self._size <= 1:
            return
        
        # Collect all nodes in current order
        nodes = list(self)
        
        # Clear existing edges
        for edge in list(self._edges):
            self.remove_edge(edge)
        
        # Rebuild in reverse order
        for i in range(len(nodes) - 1):
            self.add_edge(nodes[i + 1], nodes[i])
        
        # Swap head and tail
        self._head, self._tail = self._tail, self._head
    
    def slice(self, start: int, stop: Optional[int] = None, step: int = 1) -> 'Graph':
        """Create a new graph containing a slice of this linear graph."""
        if not self.graph_type.is_linear():
            raise ValueError("slice() only supported for LINEAR graphs")
        
        if stop is None:
            stop = self._size
        
        # Convert negative indices
        if start < 0:
            start = self._size + start
        if stop < 0:
            stop = self._size + stop
        
        # Clamp to valid range
        start = max(0, min(start, self._size))
        stop = max(0, min(stop, self._size))
        
        # Create new graph with sliced data
        sliced_data = []
        current = self._head
        index = 0
        
        while current is not None and index < stop:
            if index >= start and (index - start) % step == 0:
                sliced_data.append(current.data)
            
            successors = current.get_successors()
            current = next(iter(successors)) if successors else None
            index += 1
        
        return Graph.from_list(sliced_data)
    
    def find(self, data: Any) -> Optional[int]:
        """Find the first index of the specified data in a linear graph."""
        if not self.graph_type.is_linear():
            raise ValueError("find() only supported for LINEAR graphs")
        
        for index, node in enumerate(self):
            if node.data == data:
                return index
        return None
    
    def find_all(self, data: Any) -> List[int]:
        """Find all indices of the specified data in a linear graph."""
        if not self.graph_type.is_linear():
            raise ValueError("find_all() only supported for LINEAR graphs")
        
        indices = []
        for index, node in enumerate(self):
            if node.data == data:
                indices.append(index)
        return indices
    
    def count(self, data: Any) -> int:
        """Count occurrences of the specified data."""
        return len(self.find_all(data)) if self.graph_type.is_linear() else sum(1 for node in self._nodes if node.data == data)