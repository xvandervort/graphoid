"""
Tests for the Edge class.
"""

import pytest
from glang.core import Node, Edge


class TestEdge:
    """Test cases for the Edge class."""
    
    def test_edge_initialization(self):
        """Test edge initialization."""
        node1 = Node("A")
        node2 = Node("B")
        
        # Basic edge
        edge = Edge(node1, node2)
        assert edge.from_node == node1
        assert edge.to_node == node2
        assert edge.weight == 1.0
        assert edge.metadata == {}
        assert edge.id is not None
        
        # Edge with weight and metadata
        metadata = {"type": "connection"}
        edge2 = Edge(node1, node2, weight=2.5, metadata=metadata)
        assert edge2.weight == 2.5
        assert edge2.metadata == metadata
        
        # Edge with custom ID
        edge3 = Edge(node1, node2, edge_id="custom_id")
        assert edge3.id == "custom_id"
    
    def test_edge_registration_with_nodes(self):
        """Test that edges are automatically registered with nodes."""
        node1 = Node("A")
        node2 = Node("B")
        
        edge = Edge(node1, node2)
        
        # Check that edge is registered with both nodes
        assert edge in node1.outgoing_edges
        assert edge in node2.incoming_edges
        assert node1.out_degree == 1
        assert node2.in_degree == 1
    
    def test_edge_removal_from_nodes(self):
        """Test removing edge from nodes."""
        node1 = Node("A")
        node2 = Node("B")
        
        edge = Edge(node1, node2)
        
        # Remove edge from nodes
        edge.remove_from_nodes()
        
        assert edge not in node1.outgoing_edges
        assert edge not in node2.incoming_edges
        assert node1.out_degree == 0
        assert node2.in_degree == 0
    
    def test_metadata_management(self):
        """Test metadata operations."""
        node1 = Node("A")
        node2 = Node("B")
        edge = Edge(node1, node2)
        
        # Set metadata
        edge.set_metadata("type", "connection")
        edge.set_metadata("weight_type", "distance")
        
        # Get metadata
        assert edge.get_metadata("type") == "connection"
        assert edge.get_metadata("weight_type") == "distance"
        assert edge.get_metadata("nonexistent") is None
        assert edge.get_metadata("nonexistent", "default") == "default"
        
        # Check existence
        assert edge.has_metadata("type") is True
        assert edge.has_metadata("nonexistent") is False
    
    def test_edge_reverse(self):
        """Test creating reversed edges."""
        node1 = Node("A")
        node2 = Node("B")
        
        original = Edge(node1, node2, weight=2.0, metadata={"type": "forward"})
        reversed_edge = original.reverse()
        
        assert reversed_edge.from_node == node2
        assert reversed_edge.to_node == node1
        assert reversed_edge.weight == 2.0
        assert reversed_edge.metadata == {"type": "forward"}
        assert reversed_edge.id != original.id
    
    def test_self_loop_detection(self):
        """Test self-loop detection."""
        node1 = Node("A")
        node2 = Node("B")
        
        regular_edge = Edge(node1, node2)
        self_loop = Edge(node1, node1)
        
        assert regular_edge.is_self_loop() is False
        assert self_loop.is_self_loop() is True
    
    def test_edge_equality(self):
        """Test edge equality based on ID."""
        node1 = Node("A")
        node2 = Node("B")
        
        edge1 = Edge(node1, node2, edge_id="same_id")
        edge2 = Edge(node2, node1, edge_id="same_id")  # Different direction, same ID
        edge3 = Edge(node1, node2, edge_id="different_id")
        
        assert edge1 == edge2  # Same ID
        assert edge1 != edge3  # Different ID
        assert hash(edge1) == hash(edge2)  # Same hash for same ID
    
    def test_string_representations(self):
        """Test string representations."""
        node1 = Node("A", "node1_id")
        node2 = Node("B", "node2_id")
        
        edge = Edge(node1, node2, weight=2.5, edge_id="test_edge")
        
        str_repr = str(edge)
        assert "node1_id" in str_repr
        assert "node2_id" in str_repr
        assert "2.5" in str_repr
        
        repr_str = repr(edge)
        assert "test_edge" in repr_str
        assert "node1_id" in repr_str
        assert "node2_id" in repr_str
        assert "2.5" in repr_str