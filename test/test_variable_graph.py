"""
Tests for the VariableGraph system.
"""

import pytest
from glang.core import Graph, GraphType, VariableGraph, VariableNode


class TestVariableNode:
    """Test cases for VariableNode."""
    
    def test_variable_node_creation(self):
        """Test creating variable nodes."""
        name_node = VariableNode("test_var", "name", "var_id")
        assert name_node.data == "test_var"
        assert name_node.node_type == "name"
        assert name_node.is_name_node()
        assert not name_node.is_value_node()
        
        test_graph = Graph.from_list([1, 2, 3])
        value_node = VariableNode(test_graph, "value")
        assert value_node.data == test_graph
        assert value_node.node_type == "value"
        assert value_node.is_value_node()
        assert not value_node.is_name_node()
    
    def test_variable_node_string_representation(self):
        """Test string representations of variable nodes."""
        name_node = VariableNode("test", "name")
        value_node = VariableNode("data", "value")
        
        name_str = str(name_node)
        value_str = str(value_node)
        
        assert "📛" in name_str
        assert "📊" in value_str
        assert "test" in name_str
        assert "data" in value_str


class TestVariableGraph:
    """Test cases for VariableGraph."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.var_graph = VariableGraph()
    
    def test_variable_assignment(self):
        """Test assigning variables."""
        test_graph = Graph.from_list([1, 2, 3])
        
        self.var_graph.assign_variable("test", test_graph)
        
        assert self.var_graph.has_variable("test")
        retrieved = self.var_graph.get_variable("test")
        assert retrieved is test_graph
        assert self.var_graph.get_variable_count() == 1
    
    def test_variable_reassignment(self):
        """Test reassigning a variable to a new value."""
        graph1 = Graph.from_list([1, 2, 3])
        graph2 = Graph.from_list([4, 5, 6])
        
        # Initial assignment
        self.var_graph.assign_variable("test", graph1)
        assert self.var_graph.get_variable("test") is graph1
        
        # Reassignment
        self.var_graph.assign_variable("test", graph2)
        assert self.var_graph.get_variable("test") is graph2
        assert self.var_graph.get_variable_count() == 1  # Still only one variable
    
    def test_variable_deletion(self):
        """Test deleting variables."""
        test_graph = Graph.from_list([1, 2, 3])
        
        self.var_graph.assign_variable("test", test_graph)
        assert self.var_graph.has_variable("test")
        
        success = self.var_graph.delete_variable("test")
        assert success
        assert not self.var_graph.has_variable("test")
        assert self.var_graph.get_variable("test") is None
        assert self.var_graph.get_variable_count() == 0
        
        # Try to delete non-existent variable
        success = self.var_graph.delete_variable("nonexistent")
        assert not success
    
    def test_multiple_variables(self):
        """Test managing multiple variables."""
        graph1 = Graph.from_list([1, 2, 3])
        graph2 = Graph.from_list(["a", "b", "c"])
        graph3 = Graph(GraphType.DIRECTED)
        
        self.var_graph.assign_variable("nums", graph1)
        self.var_graph.assign_variable("letters", graph2)
        self.var_graph.assign_variable("directed", graph3)
        
        assert self.var_graph.get_variable_count() == 3
        
        var_list = self.var_graph.list_variables()
        assert "nums" in var_list
        assert "letters" in var_list
        assert "directed" in var_list
        
        assert self.var_graph.get_variable("nums") is graph1
        assert self.var_graph.get_variable("letters") is graph2
        assert self.var_graph.get_variable("directed") is graph3
    
    def test_variable_info(self):
        """Test getting variable information."""
        test_graph = Graph.from_list([1, 2, 3, 4, 5])
        self.var_graph.assign_variable("test", test_graph)
        
        info = self.var_graph.get_variable_info("test")
        assert info is not None
        assert info["name"] == "test"
        assert info["type"] == "linear"
        assert info["size"] == 5
        assert info["edges"] == 4
        assert info["value"] is test_graph
        
        # Test non-existent variable
        info = self.var_graph.get_variable_info("nonexistent")
        assert info is None
    
    def test_namespace_visualization(self):
        """Test namespace visualization."""
        # Empty namespace
        viz = self.var_graph.visualize_namespace()
        assert "No variables defined" in viz
        
        # With variables
        graph1 = Graph.from_list([1, 2, 3])
        graph2 = Graph(GraphType.TREE)
        
        self.var_graph.assign_variable("linear", graph1)
        self.var_graph.assign_variable("tree", graph2)
        
        viz = self.var_graph.visualize_namespace()
        assert "Variable Namespace Graph:" in viz
        assert "Variables: 2" in viz
        assert "📛 linear -> 📊 linear graph" in viz
        assert "📛 tree -> 📊 tree graph" in viz
    
    def test_graph_structure_properties(self):
        """Test that VariableGraph maintains graph structure properties."""
        graph1 = Graph.from_list([1, 2, 3])
        graph2 = Graph.from_list([4, 5])
        
        self.var_graph.assign_variable("first", graph1)
        self.var_graph.assign_variable("second", graph2)
        
        # Check that the variable graph has the expected structure
        assert self.var_graph.size == 4  # 2 name nodes + 2 value nodes
        assert self.var_graph.edge_count == 2  # 2 assignment edges
        
        # Check that assignment edges have correct metadata
        assignment_edges = [e for e in self.var_graph.edges if e.get_metadata("assignment")]
        assert len(assignment_edges) == 2
        
        for edge in assignment_edges:
            assert edge.get_metadata("assignment") is True
            var_name = edge.get_metadata("variable_name")
            assert var_name in ["first", "second"]


class TestVariableGraphIntegration:
    """Integration tests for variable graph system."""
    
    def test_assignment_edge_metadata(self):
        """Test that assignment edges have proper metadata."""
        var_graph = VariableGraph()
        test_graph = Graph.from_list([1, 2, 3])
        
        var_graph.assign_variable("test", test_graph)
        
        # Find the assignment edge
        assignment_edges = [e for e in var_graph.edges if e.get_metadata("assignment")]
        assert len(assignment_edges) == 1
        
        edge = assignment_edges[0]
        assert edge.get_metadata("assignment") is True
        assert edge.get_metadata("variable_name") == "test"
        
        # Verify the edge connects name node to value node
        assert edge.from_node.is_name_node()
        assert edge.to_node.is_value_node()
        assert edge.from_node.data == "test"
        assert edge.to_node.data is test_graph
    
    def test_namespace_as_meta_graph(self):
        """Test that the variable namespace truly functions as a meta-graph."""
        var_graph = VariableGraph()
        
        # Create several graphs with different structures
        linear = Graph.from_list([1, 2, 3])
        tree = Graph(GraphType.TREE)
        directed = Graph(GraphType.DIRECTED)
        
        # Assign them to variables
        var_graph.assign_variable("my_list", linear)
        var_graph.assign_variable("my_tree", tree)
        var_graph.assign_variable("my_directed", directed)
        
        # The variable graph should now contain:
        # - 3 name nodes (for variable names)
        # - 3 value nodes (for graph values)
        # - 3 assignment edges (connecting names to values)
        assert var_graph.size == 6
        assert var_graph.edge_count == 3
        
        # All nodes should be VariableNode instances
        for node in var_graph.nodes:
            assert isinstance(node, VariableNode)
            assert node.node_type in ["name", "value"]
        
        # All edges should be assignment edges
        for edge in var_graph.edges:
            assert edge.get_metadata("assignment") is True
            assert edge.has_metadata("variable_name")
    
    def test_variable_dependencies_placeholder(self):
        """Test the placeholder dependency analysis."""
        var_graph = VariableGraph()
        graph1 = Graph.from_list([1, 2, 3])
        
        var_graph.assign_variable("test", graph1)
        
        # This is currently a placeholder that returns empty dependencies
        deps = var_graph.get_variable_dependencies()
        assert "test" in deps
        assert deps["test"] == []