"""
Tests for the GraphType enumeration.
"""

import pytest
from glang.core import GraphType


class TestGraphType:
    """Test cases for the GraphType enumeration."""
    
    def test_graph_type_values(self):
        """Test that all expected graph types exist."""
        assert GraphType.LINEAR
        assert GraphType.TREE
        assert GraphType.CYCLIC
        assert GraphType.WEIGHTED
        assert GraphType.DIRECTED
        assert GraphType.UNDIRECTED
    
    def test_string_representation(self):
        """Test string representation of graph types."""
        assert str(GraphType.LINEAR) == "linear"
        assert str(GraphType.TREE) == "tree"
        assert str(GraphType.CYCLIC) == "cyclic"
        assert str(GraphType.WEIGHTED) == "weighted"
        assert str(GraphType.DIRECTED) == "directed"
        assert str(GraphType.UNDIRECTED) == "undirected"
    
    def test_from_string(self):
        """Test creating GraphType from string."""
        assert GraphType.from_string("linear") == GraphType.LINEAR
        assert GraphType.from_string("LINEAR") == GraphType.LINEAR
        assert GraphType.from_string("tree") == GraphType.TREE
        assert GraphType.from_string("CYCLIC") == GraphType.CYCLIC
        
        with pytest.raises(ValueError):
            GraphType.from_string("invalid")
    
    def test_type_checking_methods(self):
        """Test type checking methods."""
        # Linear type
        assert GraphType.LINEAR.is_linear() is True
        assert GraphType.TREE.is_linear() is False
        
        # Hierarchical type
        assert GraphType.TREE.is_hierarchical() is True
        assert GraphType.LINEAR.is_hierarchical() is False
        
        # Cycle allowance
        assert GraphType.CYCLIC.allows_cycles() is True
        assert GraphType.DIRECTED.allows_cycles() is True
        assert GraphType.UNDIRECTED.allows_cycles() is True
        assert GraphType.LINEAR.allows_cycles() is False
        assert GraphType.TREE.allows_cycles() is False
        
        # Weighted
        assert GraphType.WEIGHTED.is_weighted() is True
        assert GraphType.LINEAR.is_weighted() is False