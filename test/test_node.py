"""
Tests for the Node class.
"""

import pytest
from glang.core import Node, Edge


class TestNode:
    """Test cases for the Node class."""
    
    def test_node_initialization(self):
        """Test node initialization with default and custom parameters."""
        # Default initialization
        node1 = Node()
        assert node1.data is None
        assert node1.id is not None
        assert len(node1.id) > 0
        assert node1.out_degree == 0
        assert node1.in_degree == 0
        
        # With data
        node2 = Node("test_data")
        assert node2.data == "test_data"
        
        # With custom ID
        node3 = Node("data", "custom_id")
        assert node3.id == "custom_id"
        assert node3.data == "data"
    
    def test_node_unique_ids(self):
        """Test that nodes get unique IDs by default."""
        node1 = Node()
        node2 = Node()
        assert node1.id != node2.id
    
    def test_node_equality(self):
        """Test node equality based on ID."""
        node1 = Node("data", "same_id")
        node2 = Node("different_data", "same_id")
        node3 = Node("data", "different_id")
        
        assert node1 == node2  # Same ID
        assert node1 != node3  # Different ID
        assert hash(node1) == hash(node2)  # Same hash for same ID
    
    def test_edge_management(self):
        """Test adding and removing edges."""
        node1 = Node("A")
        node2 = Node("B")
        node3 = Node("C")
        
        # Create edges
        edge1 = Edge(node1, node2)
        edge2 = Edge(node2, node3)
        edge3 = Edge(node3, node1)
        
        # Check outgoing edges
        assert edge1 in node1.outgoing_edges
        assert edge2 in node2.outgoing_edges
        assert edge3 in node3.outgoing_edges
        
        # Check incoming edges
        assert edge1 in node2.incoming_edges
        assert edge2 in node3.incoming_edges
        assert edge3 in node1.incoming_edges
        
        # Check degrees
        assert node1.out_degree == 1
        assert node1.in_degree == 1
        assert node1.degree == 2
        
        # Remove edge
        node1.remove_outgoing_edge(edge1)
        node2.remove_incoming_edge(edge1)
        
        assert edge1 not in node1.outgoing_edges
        assert edge1 not in node2.incoming_edges
        assert node1.out_degree == 0
        assert node2.in_degree == 0
    
    def test_neighbors_and_connections(self):
        """Test neighbor and connection methods."""
        node1 = Node("A")
        node2 = Node("B")
        node3 = Node("C")
        
        edge1 = Edge(node1, node2)
        edge2 = Edge(node3, node1)
        
        # Test successors (nodes this node points to)
        successors = node1.get_successors()
        assert node2 in successors
        assert len(successors) == 1
        
        # Test predecessors (nodes that point to this node)
        predecessors = node1.get_predecessors()
        assert node3 in predecessors
        assert len(predecessors) == 1
        
        # Test all neighbors
        neighbors = node1.get_neighbors()
        assert node2 in neighbors
        assert node3 in neighbors
        assert len(neighbors) == 2
    
    def test_string_representations(self):
        """Test string representations."""
        node = Node("test_data", "test_id")
        
        str_repr = str(node)
        assert "test_id" in str_repr
        assert "test_data" in str_repr
        
        repr_str = repr(node)
        assert "test_id" in repr_str
        assert "test_data" in repr_str
        assert "out_degree" in repr_str
        assert "in_degree" in repr_str