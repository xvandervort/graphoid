"""Tests for the enhanced method system."""

import pytest
from glang.core.graph import Graph
from glang.methods import (
    MethodDispatcher,
    LinearGraphMethods, 
    GraphMethods,
    ConversionMethods
)
from glang.repl.graph_manager import GraphManager


class TestLinearGraphMethods:
    """Test methods specific to linear graphs."""
    
    def setup_method(self):
        """Set up for each test."""
        self.graph = Graph.from_list(['apple', 'banana', 'cherry'])
        self.empty_graph = Graph.from_list([])
    
    def test_append_method(self):
        """Test append method."""
        result = LinearGraphMethods.append(self.graph, 'fruits', ['orange'])
        assert result == "Appended 'orange' to fruits"
        assert self.graph.to_list() == ['apple', 'banana', 'cherry', 'orange']
    
    def test_append_multiword(self):
        """Test append with multi-word value."""
        result = LinearGraphMethods.append(self.graph, 'fruits', ['golden', 'delicious'])
        assert result == "Appended 'golden delicious' to fruits"
        assert self.graph.to_list()[-1] == 'golden delicious'
    
    def test_prepend_method(self):
        """Test prepend method."""
        result = LinearGraphMethods.prepend(self.graph, 'fruits', ['mango'])
        assert result == "Prepended 'mango' to fruits"
        assert self.graph.to_list() == ['mango', 'apple', 'banana', 'cherry']
    
    def test_insert_method(self):
        """Test insert method."""
        result = LinearGraphMethods.insert(self.graph, 'fruits', ['1', 'grape'])
        assert result == "Inserted 'grape' at index 1 in fruits"
        assert self.graph.to_list() == ['apple', 'grape', 'banana', 'cherry']
    
    def test_insert_invalid_index(self):
        """Test insert with invalid index."""
        result = LinearGraphMethods.insert(self.graph, 'fruits', ['abc', 'grape'])
        assert "Error: First argument to insert must be a number" in result
    
    def test_reverse_method(self):
        """Test reverse method."""
        result = LinearGraphMethods.reverse(self.graph, 'fruits', [])
        assert result == "Reversed fruits"
        assert self.graph.to_list() == ['cherry', 'banana', 'apple']
    
    def test_delete_at_method(self):
        """Test delete_at method."""
        result = LinearGraphMethods.delete_at(self.graph, 'fruits', ['1'])
        assert "Deleted 'banana' from index 1 in fruits" in result
        assert self.graph.to_list() == ['apple', 'cherry']
    
    def test_get_method(self):
        """Test get method."""
        result = LinearGraphMethods.get(self.graph, 'fruits', ['0'])
        assert result == 'apple'
        
        result = LinearGraphMethods.get(self.graph, 'fruits', ['2'])
        assert result == 'cherry'
        
        result = LinearGraphMethods.get(self.graph, 'fruits', ['10'])
        assert "Error: Index 10 out of range" in result
    
    def test_set_method(self):
        """Test set method."""
        result = LinearGraphMethods.set(self.graph, 'fruits', ['1', 'mango'])
        assert result == "Set index 1 to 'mango' in fruits"
        assert self.graph.to_list() == ['apple', 'mango', 'cherry']
    
    def test_find_method(self):
        """Test find method."""
        result = LinearGraphMethods.find(self.graph, 'fruits', ['banana'])
        assert result == '1'
        
        result = LinearGraphMethods.find(self.graph, 'fruits', ['not_found'])
        assert "'not_found' not found in fruits" in result
    
    def test_find_all_method(self):
        """Test find_all method."""
        # Add duplicate
        self.graph.append('apple')
        result = LinearGraphMethods.find_all(self.graph, 'fruits', ['apple'])
        assert "Found 'apple' at indices: 0, 3" in result
    
    def test_count_method(self):
        """Test count method."""
        result = LinearGraphMethods.count(self.graph, 'fruits', ['apple'])
        assert result == "'apple' appears 1 time(s) in fruits"
    
    def test_slice_method(self):
        """Test slice method."""
        result = LinearGraphMethods.slice(self.graph, 'fruits', ['1', '3'])
        assert "Slice result: ['banana', 'cherry']" in result
    
    def test_error_handling(self):
        """Test error handling for invalid arguments."""
        # Missing arguments
        result = LinearGraphMethods.append(self.graph, 'fruits', [])
        assert "Error: append requires a value" in result
        
        result = LinearGraphMethods.insert(self.graph, 'fruits', ['1'])
        assert "Error: insert requires an index and a value" in result


class TestGraphMethods:
    """Test methods that work on all graph types."""
    
    def setup_method(self):
        """Set up for each test."""
        self.graph = Graph.from_list(['a', 'b', 'c'])
        self.empty_graph = Graph.from_list([])
    
    def test_size_method(self):
        """Test size method."""
        result = GraphMethods.size(self.graph, 'test', [])
        assert result == '3'
        
        result = GraphMethods.size(self.empty_graph, 'empty', [])
        assert result == '0'
    
    def test_empty_method(self):
        """Test empty method."""
        result = GraphMethods.empty(self.graph, 'test', [])
        assert result == 'false'
        
        result = GraphMethods.empty(self.empty_graph, 'empty', [])
        assert result == 'true'
    
    def test_edges_method(self):
        """Test edges method."""
        result = GraphMethods.edges(self.graph, 'test', [])
        assert result == '2'  # Linear graph with 3 nodes has 2 edges
    
    def test_type_info_method(self):
        """Test type_info method."""
        result = GraphMethods.type_info(self.graph, 'test', [])
        assert 'Type: LINEAR' in result
        assert 'Linear: Yes' in result
        assert 'Directed: Yes' in result
    
    def test_to_list_method(self):
        """Test to_list method."""
        result = GraphMethods.to_list(self.graph, 'test', [])
        assert result == "['a', 'b', 'c']"
    
    def test_traverse_method(self):
        """Test traverse method."""
        result = GraphMethods.traverse(self.graph, 'test', [])
        assert "Traversal: ['a', 'b', 'c']" in result
    
    def test_stats_method(self):
        """Test stats method."""
        result = GraphMethods.stats(self.graph, 'test', [])
        assert 'Variable: test' in result
        assert 'Type: LINEAR' in result
        assert 'Nodes: 3' in result
        assert 'Edges: 2' in result
        assert 'Length: 3' in result
    
    def test_type_checking_methods(self):
        """Test type checking methods."""
        assert GraphMethods.is_linear(self.graph, 'test', []) == 'true'
        assert GraphMethods.is_directed(self.graph, 'test', []) == 'true'
        assert GraphMethods.is_weighted(self.graph, 'test', []) == 'false'
    
    def test_clear_method(self):
        """Test clear method."""
        result = GraphMethods.clear(self.graph, 'test', [])
        assert result == 'Cleared test'
        assert len(self.graph.nodes) == 0


class TestConversionMethods:
    """Test conversion methods."""
    
    def setup_method(self):
        """Set up for each test."""
        self.graph = Graph.from_list(['a', 'b', 'c'])
    
    def test_to_directed_method(self):
        """Test to_directed method."""
        result = ConversionMethods.to_directed(self.graph, 'test', [])
        # Since LINEAR is already directed, should indicate that
        assert 'already' in result.lower()
    
    def test_conversion_methods_exist(self):
        """Test that all conversion methods exist."""
        methods = ['to_directed', 'to_undirected', 'to_tree', 'to_linear']
        for method_name in methods:
            method = getattr(ConversionMethods, method_name)
            result = method(self.graph, 'test', [])
            assert isinstance(result, str)
            assert len(result) > 0


class TestMethodDispatcher:
    """Test the main method dispatcher."""
    
    def setup_method(self):
        """Set up for each test."""
        self.graph_manager = GraphManager()
        self.dispatcher = MethodDispatcher(self.graph_manager)
        
        # Create test graphs
        self.graph_manager.create_from_list('fruits', ['apple', 'banana'])
        self.graph_manager.create_from_list('empty', [])
    
    def test_dispatch_mutating_method(self):
        """Test dispatching mutating methods."""
        result = self.dispatcher.dispatch_method('fruits', 'append', ['cherry'])
        assert "Appended 'cherry' to fruits" in result
        
        # Verify the graph was modified
        graph = self.graph_manager.get_variable('fruits')
        assert graph.to_list() == ['apple', 'banana', 'cherry']
    
    def test_dispatch_query_method(self):
        """Test dispatching query methods."""
        result = self.dispatcher.dispatch_method('fruits', 'size', [])
        assert result == '2'
        
        result = self.dispatcher.dispatch_method('fruits', 'find', ['banana'])
        assert result == '1'
    
    def test_dispatch_conversion_method(self):
        """Test dispatching conversion methods."""
        result = self.dispatcher.dispatch_method('fruits', 'to_directed', [])
        assert 'already' in result.lower()  # LINEAR is already directed
    
    def test_method_not_found(self):
        """Test handling of unknown methods."""
        result = self.dispatcher.dispatch_method('fruits', 'unknown_method', [])
        assert 'Error: Method \'unknown_method\' not supported' in result
    
    def test_similar_method_suggestions(self):
        """Test similar method suggestions."""
        result = self.dispatcher.dispatch_method('fruits', 'appnd', [])  # Typo
        assert 'Similar methods:' in result
        assert 'append' in result
    
    def test_variable_not_found(self):
        """Test handling of unknown variables."""
        result = self.dispatcher.dispatch_method('unknown_var', 'size', [])
        assert "Error: Variable 'unknown_var' not found" in result
    
    def test_type_compatibility_validation(self):
        """Test method type compatibility validation."""
        # Test with empty graph validation - make sure 'empty' graph exists
        # Debug: check if empty graph actually exists
        empty_graph = self.graph_manager.get_variable('empty')
        assert empty_graph is not None, "Empty graph should exist"
        assert len(empty_graph.nodes) == 0, "Empty graph should have 0 nodes"
        
        # Debug: check if dispatcher can find the graph
        dispatcher_graph = self.dispatcher.graph_manager.get_variable('empty')
        assert dispatcher_graph is not None, "Dispatcher should also find empty graph"
        assert self.graph_manager is self.dispatcher.graph_manager, "Should be same manager"
        
        result = self.dispatcher.dispatch_method('empty', 'reverse', [])
        assert 'requires a non-empty graph' in result
    
    def test_linear_method_on_linear_graph(self):
        """Test linear-specific methods work on linear graphs."""
        result = self.dispatcher.dispatch_method('fruits', 'get', ['0'])
        assert result == 'apple'
    
    def test_method_categories(self):
        """Test that methods are properly categorized."""
        # Test mutating methods
        assert 'append' in self.dispatcher.mutating_methods
        assert 'clear' in self.dispatcher.mutating_methods
        
        # Test query methods
        assert 'size' in self.dispatcher.query_methods
        assert 'find' in self.dispatcher.query_methods
        
        # Test conversion methods
        assert 'to_directed' in self.dispatcher.conversion_methods
        assert 'copy' in self.dispatcher.conversion_methods
    
    def test_all_methods_accessible(self):
        """Test that all methods are accessible through the dispatcher."""
        expected_count = (
            len(self.dispatcher.mutating_methods) + 
            len(self.dispatcher.query_methods) + 
            len(self.dispatcher.conversion_methods)
        )
        assert len(self.dispatcher.all_methods) == expected_count
    
    def test_list_available_methods(self):
        """Test listing available methods."""
        methods_list = self.dispatcher.list_available_methods()
        assert 'Mutating methods' in methods_list
        assert 'Query methods' in methods_list
        assert 'Conversion methods' in methods_list
        assert 'append' in methods_list
        assert 'size' in methods_list
    
    def test_method_help(self):
        """Test getting help for methods."""
        help_text = self.dispatcher.get_method_help('append')
        assert 'mutating' in help_text
        
        help_text = self.dispatcher.get_method_help('size')
        assert 'query' in help_text
        
        help_text = self.dispatcher.get_method_help('to_directed')
        assert 'conversion' in help_text
        
        help_text = self.dispatcher.get_method_help('nonexistent')
        assert 'not found' in help_text


class TestMethodIntegration:
    """Test method system integration with REPL."""
    
    def setup_method(self):
        """Set up integration tests."""
        self.graph_manager = GraphManager()
        self.dispatcher = MethodDispatcher(self.graph_manager)
    
    def test_end_to_end_workflow(self):
        """Test complete workflow with multiple operations."""
        # Create a graph
        self.graph_manager.create_from_list('numbers', ['1', '2', '3'])
        
        # Test various operations
        operations = [
            ('append', ['4'], 'Appended'),
            ('size', [], '4'),
            ('find', ['2'], '1'),
            ('reverse', [], 'Reversed'),
            ('get', ['0'], '4'),  # After reverse, first element is '4'
        ]
        
        for method, args, expected in operations:
            result = self.dispatcher.dispatch_method('numbers', method, args)
            assert expected in result, f"Failed on {method}: {result}"
    
    def test_error_recovery(self):
        """Test system behavior with errors."""
        self.graph_manager.create_from_list('test', ['a'])
        
        # Test various error conditions
        error_tests = [
            ('get', ['10'], 'out of range'),
            ('delete', ['5'], 'out of range'),
            ('insert', ['abc'], 'must be a number'),
            ('find', [], 'requires a value'),
        ]
        
        for method, args, expected_error in error_tests:
            result = self.dispatcher.dispatch_method('test', method, args)
            assert 'Error' in result
            # Graph should still be intact after errors
            size_result = self.dispatcher.dispatch_method('test', 'size', [])
            assert size_result == '1'
    
    def test_method_chaining_simulation(self):
        """Test simulation of method chaining effects."""
        self.graph_manager.create_from_list('chain', ['a', 'b', 'c'])
        
        # Simulate: chain.append('d').reverse().get(0)
        self.dispatcher.dispatch_method('chain', 'append', ['d'])
        self.dispatcher.dispatch_method('chain', 'reverse', [])
        result = self.dispatcher.dispatch_method('chain', 'get', ['0'])
        
        # After append and reverse, first element should be 'd'
        assert result == 'd'