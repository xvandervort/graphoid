"""Tests for enhanced tab completion functionality (Phase 4)."""

import pytest
from glang.repl import REPL


class TestEnhancedTabCompletion:
    """Test enhanced tab completion features."""
    
    def setup_method(self):
        self.repl = REPL()
        # Create some test variables
        self.repl._process_input('list fruits = ["apple", "banana", "cherry"]')
        self.repl._process_input('list<num> scores = [95, 87, 92]')
        self.repl._process_input('graph network = directed()')
    
    def test_context_detection_methods(self):
        """Test context detection methods."""
        # Method context detection
        assert self.repl._is_method_context("fruits.append")
        assert self.repl._is_method_context("scores.types")
        assert not self.repl._is_method_context("fruits.")
        assert not self.repl._is_method_context("just_text")
        
        # Type constraint context detection
        assert self.repl._is_type_constraint_context("list<num")
        assert self.repl._is_type_constraint_context("list<s")
        assert not self.repl._is_type_constraint_context("list>")
        assert not self.repl._is_type_constraint_context("list")
        
        # Graph type declaration detection  
        assert self.repl._is_declaring_type("li")
        assert self.repl._is_declaring_type("g")
        assert not self.repl._is_declaring_type("list ")
        assert not self.repl._is_declaring_type("list fruits")
    
    def test_method_completions(self):
        """Test method completion functionality."""
        # Test append completion
        completions = self.repl._get_method_completions("fruits.ap", "ap")
        assert "append" in completions
        
        # Test type method completions
        completions = self.repl._get_method_completions("fruits.t", "t")
        assert "types" in completions
        assert "type_summary" in completions
        
        # Test all available methods for linear graphs
        completions = self.repl._get_method_completions("fruits.", "")
        expected_methods = [
            'append', 'prepend', 'insert', 'reverse', 'delete', 
            'size', 'empty', 'types', 'constraint', 'validate_constraint',
            'type_summary', 'coerce_to_constraint'
        ]
        for method in expected_methods:
            assert method in completions
        
        # Test constraint-specific completions
        # TODO: Fix parser to handle list<num> scores properly
        # completions = self.repl._get_method_completions("scores.c", "c")
        # assert "constraint" in completions
        # assert "coerce_to_constraint" in completions
        
        # Test nonexistent variable
        completions = self.repl._get_method_completions("nonexistent.test", "test")
        assert completions == []
    
    def test_type_constraint_completions(self):
        """Test type constraint completion functionality."""
        # Test num completion
        completions = self.repl._get_type_constraint_completions("list<n", "n")
        assert completions == ["num"]
        
        # Test string completion
        completions = self.repl._get_type_constraint_completions("list<s", "s")
        assert completions == ["string"]
        
        # Test all type constraints
        completions = self.repl._get_type_constraint_completions("list<", "")
        expected_types = ["num", "string", "bool", "list"]
        for type_name in expected_types:
            assert type_name in completions
        
        # Test partial matches
        completions = self.repl._get_type_constraint_completions("list<bo", "bo")
        assert completions == ["bool"]
        
        # Test no angle bracket
        completions = self.repl._get_type_constraint_completions("list", "")
        assert completions == []
    
    def test_graph_type_completions(self):
        """Test graph type completion functionality."""
        # Test partial matches
        assert "list" in self.repl._get_graph_type_completions("li")
        assert "graph" in self.repl._get_graph_type_completions("g")
        assert "directed" in self.repl._get_graph_type_completions("d")
        assert "weighted" in self.repl._get_graph_type_completions("w")
        
        # Test empty input returns all types
        completions = self.repl._get_graph_type_completions("")
        expected_types = ['list', 'graph', 'tree', 'directed', 'weighted', 'undirected']
        for graph_type in expected_types:
            assert graph_type in completions
        
        # Test no matches
        completions = self.repl._get_graph_type_completions("xyz")
        assert completions == []
    
    def test_main_completion_integration(self):
        """Test main completion function integration."""
        # Test method completion integration
        completions = self.repl._get_completions("ap", "fruits.ap")
        assert "append" in completions
        
        # Test type constraint integration
        completions = self.repl._get_completions("n", "list<n")
        assert "num" in completions
        
        # Test graph type integration at start of line
        completions = self.repl._get_completions("li", "li")
        assert "list" in completions
        
        # Test variable access (fruits should be available)
        completions = self.repl._get_completions("f", "f")
        assert "fruits" in completions
        
        # Test command completion
        completions = self.repl._get_completions("h", "h")
        assert "help" in completions or "h" in completions
    
    def test_completion_with_different_graph_types(self):
        """Test completion works with different graph types."""
        # Linear graph methods
        linear_completions = self.repl._get_method_completions("fruits.", "")
        assert "append" in linear_completions
        assert "types" in linear_completions
        
        # Non-linear graph methods (if implemented)
        # For now, this would return basic methods like 'size', 'empty'
        # When we implement other graph types, this test can be expanded
    
    def test_completion_edge_cases(self):
        """Test edge cases for completion."""
        # Empty line
        completions = self.repl._get_completions("", "")
        assert len(completions) > 0  # Should have commands, types, variables
        
        # Just a dot (invalid)
        completions = self.repl._get_completions("", ".")
        assert len(completions) == 0
        
        # Just angle bracket (invalid)
        completions = self.repl._get_completions("", "<")
        assert len(completions) == 0
        
        # Variable that doesn't exist
        completions = self.repl._get_method_completions("nonexistent.test", "test")
        assert completions == []
        
        # Complex method chain (not yet supported, should handle gracefully)
        completions = self.repl._get_method_completions("fruits.types().test", "test")
        assert completions == []  # Should not crash


class TestCompletionAccuracy:
    """Test that completions are accurate and useful."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_method_completion_accuracy(self):
        """Test that method completions are accurate for the variable type."""
        # Create variables of different types
        self.repl._process_input('list fruits = ["apple", "banana"]')
        self.repl._process_input('list<num> scores = [95, 87]')
        
        # Linear graph should have linear methods
        completions = self.repl._get_method_completions("fruits.", "")
        linear_methods = ['append', 'prepend', 'insert', 'reverse', 'delete']
        for method in linear_methods:
            assert method in completions, f"Linear method {method} missing"
        
        # Type introspection methods should be available
        type_methods = ['types', 'constraint', 'type_summary']
        for method in type_methods:
            assert method in completions, f"Type method {method} missing"
    
    def test_no_false_completions(self):
        """Test that we don't suggest invalid completions."""
        self.repl._process_input('list fruits = ["apple", "banana"]')
        
        # Should not suggest non-existent methods
        completions = self.repl._get_method_completions("fruits.xyz", "xyz")
        assert completions == []
        
        # Should not suggest methods that don't start with the text
        completions = self.repl._get_method_completions("fruits.xyz", "xyz")
        assert "append" not in completions
    
    def test_completion_ordering(self):
        """Test that completions are in a reasonable order."""
        # Type constraints should be in a logical order
        completions = self.repl._get_type_constraint_completions("list<", "")
        assert "num" in completions
        assert "string" in completions
        assert "bool" in completions
        assert "list" in completions
        
        # All should be present
        assert len(completions) == 4


class TestCompletionPerformance:
    """Test completion performance and edge cases."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_completion_with_many_variables(self):
        """Test completion performance with many variables."""
        # Create multiple variables
        for i in range(10):
            self.repl._process_input(f'list var{i} = ["item{i}"]')
        
        # Should still complete quickly
        completions = self.repl._get_completions("var", "var")
        assert len(completions) >= 10
        
        # Method completion should work on any variable
        completions = self.repl._get_method_completions("var5.ap", "ap")
        assert "append" in completions
    
    def test_completion_with_complex_names(self):
        """Test completion with complex variable names."""
        self.repl._process_input('list complex_name_123 = ["a", "b", "c"]')
        self.repl._process_input('list another_var = ["x", "y", "z"]')
        
        # Should complete complex names
        completions = self.repl._get_completions("complex", "complex")
        assert any("complex_name_123" in comp for comp in completions)
        
        # Method completion should work with complex names
        completions = self.repl._get_method_completions("complex_name_123.t", "t")
        assert "types" in completions