"""
Tests for the Graph class.
"""

import pytest
from glang.core import Node, Edge, Graph, GraphType


class TestGraph:
    """Test cases for the Graph class."""
    
    def test_graph_initialization(self):
        """Test graph initialization."""
        # Default graph
        graph = Graph()
        assert graph.graph_type == GraphType.DIRECTED
        assert graph.is_empty()
        assert graph.size == 0
        assert graph.edge_count == 0
        
        # Linear graph
        linear_graph = Graph(GraphType.LINEAR)
        assert linear_graph.graph_type == GraphType.LINEAR
    
    def test_node_operations(self):
        """Test node addition and removal."""
        graph = Graph()
        
        # Create and add nodes
        node1 = Node("A")
        node2 = graph.create_node("B")
        
        graph.add_node(node1)
        
        assert graph.size == 2
        assert graph.has_node(node1)
        assert graph.has_node(node2)
        assert not graph.is_empty()
        
        # Remove node
        graph.remove_node(node1)
        assert graph.size == 1
        assert not graph.has_node(node1)
        assert graph.has_node(node2)
    
    def test_edge_operations(self):
        """Test edge addition and removal."""
        graph = Graph()
        node1 = graph.create_node("A")
        node2 = graph.create_node("B")
        
        # Add edge
        edge = graph.add_edge(node1, node2, weight=2.0)
        assert graph.edge_count == 1
        assert graph.has_edge(node1, node2)
        assert graph.get_edge(node1, node2) == edge
        
        # Remove edge
        graph.remove_edge(edge)
        assert graph.edge_count == 0
        assert not graph.has_edge(node1, node2)
        assert graph.get_edge(node1, node2) is None
    
    def test_node_lookup(self):
        """Test node lookup by ID."""
        graph = Graph()
        node = graph.create_node("test_data", "test_id")
        
        found_node = graph.get_node_by_id("test_id")
        assert found_node == node
        
        not_found = graph.get_node_by_id("nonexistent")
        assert not_found is None
    
    def test_clear_graph(self):
        """Test clearing the graph."""
        graph = Graph()
        node1 = graph.create_node("A")
        node2 = graph.create_node("B")
        graph.add_edge(node1, node2)
        
        assert graph.size == 2
        assert graph.edge_count == 1
        
        graph.clear()
        
        assert graph.size == 0
        assert graph.edge_count == 0
        assert graph.is_empty()


class TestLinearGraph:
    """Test cases for linear graph operations."""
    
    def test_from_list(self):
        """Test creating graph from list."""
        data = [1, 2, 3, 4]
        graph = Graph.from_list(data)
        
        assert graph.graph_type == GraphType.LINEAR
        assert graph.size == 4
        assert graph.to_list() == data
    
    def test_append_prepend(self):
        """Test append and prepend operations."""
        graph = Graph(GraphType.LINEAR)
        
        # Append
        node1 = graph.append(1)
        node2 = graph.append(2)
        node3 = graph.append(3)
        
        assert graph.to_list() == [1, 2, 3]
        
        # Prepend
        node0 = graph.prepend(0)
        
        assert graph.to_list() == [0, 1, 2, 3]
    
    def test_insert_operations(self):
        """Test insert operations at various positions."""
        graph = Graph.from_list([1, 2, 4, 5])
        
        # Insert in middle
        graph.insert(2, 3)
        assert graph.to_list() == [1, 2, 3, 4, 5]
        
        # Insert at beginning
        graph.insert(0, 0)
        assert graph.to_list() == [0, 1, 2, 3, 4, 5]
        
        # Insert at end
        graph.insert(100, 6)  # Large index should append
        assert graph.to_list() == [0, 1, 2, 3, 4, 5, 6]
        
        # Insert with negative index
        graph.insert(-1, 99)  # Should insert at the end (append behavior)
        assert graph.to_list() == [0, 1, 2, 3, 4, 5, 6, 99]
    
    def test_delete_operations(self):
        """Test delete operations."""
        graph = Graph.from_list([1, 2, 3, 4, 5])
        
        # Delete from middle
        deleted = graph.delete(2)
        assert deleted == 3
        assert graph.to_list() == [1, 2, 4, 5]
        
        # Delete from beginning
        deleted = graph.delete(0)
        assert deleted == 1
        assert graph.to_list() == [2, 4, 5]
        
        # Delete from end
        deleted = graph.delete(-1)
        assert deleted == 5
        assert graph.to_list() == [2, 4]
        
        # Delete invalid index
        deleted = graph.delete(100)
        assert deleted is None
        assert graph.to_list() == [2, 4]
    
    def test_get_set_operations(self):
        """Test get and set operations."""
        graph = Graph.from_list([1, 2, 3, 4, 5])
        
        # Get values
        assert graph.get(0) == 1
        assert graph.get(2) == 3
        assert graph.get(-1) == 5
        assert graph.get(100) is None
        
        # Set values
        assert graph.set(1, 99) is True
        assert graph.get(1) == 99
        assert graph.to_list() == [1, 99, 3, 4, 5]
        
        # Set invalid index
        assert graph.set(100, 42) is False
    
    def test_traversal_operations(self):
        """Test traversal and iteration."""
        data = [1, 2, 3, 4, 5]
        graph = Graph.from_list(data)
        
        # Test iteration
        node_data = [node.data for node in graph]
        assert node_data == data
        
        # Test traverse method
        assert graph.traverse() == data
        
        # Test to_list method
        assert graph.to_list() == data
    
    def test_reverse_operation(self):
        """Test reverse operation."""
        graph = Graph.from_list([1, 2, 3, 4, 5])
        graph.reverse()
        assert graph.to_list() == [5, 4, 3, 2, 1]
        
        # Test with single element
        single_graph = Graph.from_list([42])
        single_graph.reverse()
        assert single_graph.to_list() == [42]
        
        # Test with empty graph
        empty_graph = Graph(GraphType.LINEAR)
        empty_graph.reverse()
        assert empty_graph.to_list() == []
    
    def test_slice_operations(self):
        """Test slice operations."""
        graph = Graph.from_list([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        
        # Basic slice
        slice1 = graph.slice(2, 7)
        assert slice1.to_list() == [2, 3, 4, 5, 6]
        
        # Slice with step
        slice2 = graph.slice(0, 10, 2)
        assert slice2.to_list() == [0, 2, 4, 6, 8]
        
        # Slice from beginning
        slice3 = graph.slice(0, 5)
        assert slice3.to_list() == [0, 1, 2, 3, 4]
        
        # Slice to end
        slice4 = graph.slice(5)
        assert slice4.to_list() == [5, 6, 7, 8, 9]
        
        # Negative indices
        slice5 = graph.slice(-5, -1)
        assert slice5.to_list() == [5, 6, 7, 8]
    
    def test_find_operations(self):
        """Test find operations."""
        graph = Graph.from_list([1, 2, 3, 2, 4, 2, 5])
        
        # Find first occurrence
        assert graph.find(2) == 1
        assert graph.find(5) == 6
        assert graph.find(99) is None
        
        # Find all occurrences
        assert graph.find_all(2) == [1, 3, 5]
        assert graph.find_all(1) == [0]
        assert graph.find_all(99) == []
        
        # Count occurrences
        assert graph.count(2) == 3
        assert graph.count(1) == 1
        assert graph.count(99) == 0
    
    def test_linear_graph_restrictions(self):
        """Test that linear operations are restricted to linear graphs."""
        directed_graph = Graph(GraphType.DIRECTED)
        
        with pytest.raises(ValueError):
            directed_graph.append(1)
        
        with pytest.raises(ValueError):
            directed_graph.prepend(1)
        
        with pytest.raises(ValueError):
            directed_graph.insert(0, 1)
        
        with pytest.raises(ValueError):
            directed_graph.to_list()


class TestGraphContainment:
    """Test graph containment and membership operations."""
    
    def test_len_and_contains(self):
        """Test __len__ and __contains__ methods."""
        graph = Graph()
        node1 = graph.create_node("A")
        node2 = Node("B")
        
        assert len(graph) == 1
        assert node1 in graph
        assert node2 not in graph
        
        graph.add_node(node2)
        assert len(graph) == 2
        assert node2 in graph
    
    def test_string_representations(self):
        """Test string representations of graphs."""
        graph = Graph(GraphType.LINEAR)
        graph.append(1)
        graph.append(2)
        
        str_repr = str(graph)
        assert "linear" in str_repr
        assert "nodes=2" in str_repr
        assert "edges=1" in str_repr
        
        repr_str = repr(graph)
        assert "linear" in repr_str