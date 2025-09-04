"""Tests for method call variable resolution functionality."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.repl import REPL
from glang.core import AtomicValue


class TestMethodVariableResolution:
    """Test suite for method call variable resolution functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.repl = REPL()
    
    def test_append_list_variable(self):
        """Test appending a list variable: d.append a where a = [1, 2]"""
        # Create source list
        parsed = self.repl.syntax_parser.parse_input('list a = [1, 2]')
        self.repl._handle_variable_declaration(parsed)
        
        # Create empty destination list
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append the list variable
        parsed = self.repl.syntax_parser.parse_input('d.append a')
        result = self.repl._handle_method_call(parsed)
        
        # Check the result
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 2
        assert d.to_list() == [1, 2]
    
    def test_append_scalar_variable(self):
        """Test appending a scalar variable: d.append x where x = 'hello'"""
        # Create scalar variable
        parsed = self.repl.syntax_parser.parse_input('string val = "hello"')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination list
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append the scalar variable
        parsed = self.repl.syntax_parser.parse_input('d.append val')
        result = self.repl._handle_method_call(parsed)
        
        # Check the result
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 1
        assert d.to_list() == ['hello']
    
    def test_append_index_expression(self):
        """Test appending with index expression: d.append matrix[0]"""
        # Create matrix
        parsed = self.repl.syntax_parser.parse_input('list matrix = [[1, 2], [3, 4]]')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination list
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append indexed value
        parsed = self.repl.syntax_parser.parse_input('d.append matrix[0]')
        result = self.repl._handle_method_call(parsed)
        
        # Check the result - matrix[0] should extend d with [1, 2]
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 2
        assert d.to_list() == [1, 2]
    
    def test_append_chained_index(self):
        """Test appending with chained index: d.append matrix[0][1]"""
        # Create matrix
        parsed = self.repl.syntax_parser.parse_input('list matrix = [[1, 2], [3, 4]]')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination list
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append chained indexed value
        parsed = self.repl.syntax_parser.parse_input('d.append matrix[0][1]')
        result = self.repl._handle_method_call(parsed)
        
        # Check the result - matrix[0][1] = 2 should be appended as single value
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 1
        assert d.to_list() == [2]
    
    def test_append_multiple_variables(self):
        """Test multiple appends with different variable types"""
        # Create variables
        parsed = self.repl.syntax_parser.parse_input('list a = [1, 2]')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('list b = [3, 4]')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('string c = "hello"')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Multiple appends
        parsed = self.repl.syntax_parser.parse_input('d.append a')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append b')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append c')
        self.repl._handle_method_call(parsed)
        
        # Check final result
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 5  # [1, 2] + [3, 4] + ["hello"] = 5 elements
        assert d.to_list() == [1, 2, 3, 4, 'hello']
    
    def test_append_literal_vs_variable(self):
        """Test difference between literal values and variable references"""
        # Create a variable named 'test'
        parsed = self.repl.syntax_parser.parse_input('string test = "variable_value"')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append literal string
        parsed = self.repl.syntax_parser.parse_input('d.append "literal"')
        self.repl._handle_method_call(parsed)
        
        # Append variable reference
        parsed = self.repl.syntax_parser.parse_input('d.append test')
        self.repl._handle_method_call(parsed)
        
        # Check results
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 2
        assert d.to_list() == ['literal', 'variable_value']
    
    def test_append_numeric_variables(self):
        """Test appending numeric variables"""
        # Create numeric variables
        parsed = self.repl.syntax_parser.parse_input('num x = 42')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('num y = 3.14')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append numeric variables
        parsed = self.repl.syntax_parser.parse_input('d.append x')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append y')
        self.repl._handle_method_call(parsed)
        
        # Check results
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 2
        assert d.to_list() == [42, 3.14]
    
    def test_append_boolean_variables(self):
        """Test appending boolean variables"""
        # Create boolean variables
        parsed = self.repl.syntax_parser.parse_input('bool flag1 = true')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('bool flag2 = false')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append boolean variables
        parsed = self.repl.syntax_parser.parse_input('d.append flag1')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append flag2')
        self.repl._handle_method_call(parsed)
        
        # Check results
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 2
        assert d.to_list() == [True, False]
    
    def test_append_mixed_types(self):
        """Test appending mixed variable types together"""
        # Create mixed variables
        parsed = self.repl.syntax_parser.parse_input('list nums = [1, 2, 3]')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('string text = "hello"')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('bool flag = true')
        self.repl._handle_variable_declaration(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('num count = 42')
        self.repl._handle_variable_declaration(parsed)
        
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append all variables
        parsed = self.repl.syntax_parser.parse_input('d.append nums')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append text')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append flag')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append count')
        self.repl._handle_method_call(parsed)
        
        # Check results
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 6  # [1,2,3] + "hello" + true + 42 = 6 elements
        assert d.to_list() == [1, 2, 3, 'hello', True, 42]
    
    def test_error_handling_nonexistent_variable(self):
        """Test fallback behavior when referencing non-existent variables"""
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Try to append non-existent variable
        parsed = self.repl.syntax_parser.parse_input('d.append nonexistent')
        result = self.repl._handle_method_call(parsed)
        
        # Current behavior: falls back to treating as literal string
        # This could be considered reasonable behavior - unrecognized identifiers become strings
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 1  # 'nonexistent' added as literal string
        assert d.to_list() == ['nonexistent']
    
    def test_append_preserves_original_functionality(self):
        """Test that literal append functionality is preserved"""
        # Create destination
        parsed = self.repl.syntax_parser.parse_input('list d = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Append literal values (traditional functionality)
        parsed = self.repl.syntax_parser.parse_input('d.append 42')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append "hello"')
        self.repl._handle_method_call(parsed)
        
        parsed = self.repl.syntax_parser.parse_input('d.append true')
        self.repl._handle_method_call(parsed)
        
        # Check results
        d = self.repl.graph_manager.get_variable('d')
        assert d is not None
        assert len(d) == 3
        assert d.to_list() == [42, 'hello', True]
    
    def test_type_constraints_with_variables(self):
        """Test type constraints work with variable references"""
        # Create constrained list
        parsed = self.repl.syntax_parser.parse_input('list<num> numbers = []')
        self.repl._handle_variable_declaration(parsed)
        
        # Create compatible variable
        parsed = self.repl.syntax_parser.parse_input('num x = 42')
        self.repl._handle_variable_declaration(parsed)
        
        # Create incompatible variable
        parsed = self.repl.syntax_parser.parse_input('string text = "hello"')
        self.repl._handle_variable_declaration(parsed)
        
        # Append compatible variable (should work)
        parsed = self.repl.syntax_parser.parse_input('numbers.append x')
        result = self.repl._handle_method_call(parsed)
        
        # Check it was added
        numbers = self.repl.graph_manager.get_variable('numbers')
        assert len(numbers) == 1
        assert numbers.to_list() == [42]
        
        # Try to append incompatible variable (should fail)
        parsed = self.repl.syntax_parser.parse_input('numbers.append text')
        result = self.repl._handle_method_call(parsed)
        
        # Should still have only one element
        assert len(numbers) == 1
        assert numbers.to_list() == [42]


if __name__ == '__main__':
    pytest.main([__file__])