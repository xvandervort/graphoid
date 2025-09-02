"""Tests for nested list functionality."""

import pytest
from glang.parser.tokenizer import Tokenizer
from glang.repl import REPL
from io import StringIO
import sys


class TestNestedListParsing:
    """Test parsing of nested list structures."""
    
    def setup_method(self):
        self.tokenizer = Tokenizer()
    
    def test_simple_nested_list(self):
        """Test parsing simple nested lists."""
        result = self.tokenizer.parse_list_literal_with_types("[[1, 2], [3, 4]]")
        expected = [[1, 2], [3, 4]]
        
        assert result == expected
        assert len(result) == 2
        assert isinstance(result[0], list)
        assert isinstance(result[1], list)
    
    def test_mixed_nested_content(self):
        """Test nested lists with mixed types."""
        result = self.tokenizer.parse_list_literal_with_types("[['a', 'b'], [1, 2], [true, false]]")
        expected = [['a', 'b'], [1, 2], [True, False]]
        
        assert result == expected
        assert isinstance(result[0][0], str)
        assert isinstance(result[1][0], int)
        assert isinstance(result[2][0], bool)
    
    def test_irregular_nested_structure(self):
        """Test nested lists with different sizes."""
        result = self.tokenizer.parse_list_literal_with_types("[[1], [2, 3, 4], [5, 6]]")
        expected = [[1], [2, 3, 4], [5, 6]]
        
        assert result == expected
        assert len(result[0]) == 1
        assert len(result[1]) == 3
        assert len(result[2]) == 2
    
    def test_three_level_nesting(self):
        """Test three levels of nesting."""
        result = self.tokenizer.parse_list_literal_with_types("[[[1, 2], [3, 4]], [[5, 6], [7, 8]]]")
        expected = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
        
        assert result == expected
        assert result[0][0][0] == 1
        assert result[0][0][1] == 2
        assert result[1][1][1] == 8
    
    def test_nested_with_quotes(self):
        """Test nested lists with quoted strings."""
        result = self.tokenizer.parse_list_literal_with_types("[['hello world', 'test'], ['quoted', test]]")
        expected = [['hello world', 'test'], ['quoted', 'test']]
        
        assert result == expected
        assert result[0][0] == 'hello world'  # quoted
        assert result[0][1] == 'test'         # quoted
        assert result[1][0] == 'quoted'       # quoted
        assert result[1][1] == 'test'         # unquoted
    
    def test_empty_nested_lists(self):
        """Test nested lists with empty sublists."""
        result = self.tokenizer.parse_list_literal_with_types("[[], [1, 2], []]")
        expected = [[], [1, 2], []]
        
        assert result == expected
        assert len(result[0]) == 0
        assert len(result[1]) == 2
        assert len(result[2]) == 0


class TestNestedListREPLIntegration:
    """Test nested list integration with REPL."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_create_and_display_nested_list(self):
        """Test creating and displaying nested lists."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list matrix = [[1, 2, 3], [4, 5, 6]]')
            self.repl._process_input('matrix')
            
            output = captured_output.getvalue().strip()
            assert "[[1, 2, 3], [4, 5, 6]]" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_single_level_indexing(self):
        """Test single-level indexing into nested lists."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list data = [[a, b], [c, d]]')
            self.repl._process_input('data[0]')
            self.repl._process_input('data[1]')
            
            output_lines = captured_output.getvalue().strip().split('\n')
            
            # Look for the list outputs
            list_outputs = [line for line in output_lines if '[' in line and ']' in line]
            assert len([line for line in list_outputs if "'a', 'b'" in line or "a, b" in line]) >= 1
            assert len([line for line in list_outputs if "'c', 'd'" in line or "c, d" in line]) >= 1
        finally:
            sys.stdout = sys.__stdout__
    
    def test_multi_dimensional_indexing(self):
        """Test multi-dimensional indexing."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list grid = [[1, 2], [3, 4]]')
            self.repl._process_input('grid[0][0]')  # Should be 1
            self.repl._process_input('grid[0][1]')  # Should be 2
            self.repl._process_input('grid[1][0]')  # Should be 3  
            self.repl._process_input('grid[1][1]')  # Should be 4
            
            output_lines = captured_output.getvalue().strip().split('\n')
            
            # Find lines that are just numbers
            number_lines = [line.strip() for line in output_lines if line.strip().isdigit()]
            
            assert '1' in number_lines
            assert '2' in number_lines
            assert '3' in number_lines
            assert '4' in number_lines
        finally:
            sys.stdout = sys.__stdout__
    
    def test_negative_indexing_nested(self):
        """Test negative indexing with nested lists."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list table = [[a, b, c], [x, y, z]]')
            self.repl._process_input('table[-1]')      # Last row: [x, y, z]
            self.repl._process_input('table[-1][-1]')  # Last element: z
            
            output = captured_output.getvalue().strip()
            assert 'z' in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_type_system_with_nested_lists(self):
        """Test type system integration with nested lists."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list nested = [[1, 2], [hello, world]]')
            self.repl._process_input('nested.types()')
            
            output = captured_output.getvalue().strip()
            # Top level should show list types
            assert 'list' in output
        finally:
            sys.stdout = sys.__stdout__


class TestNestedListErrorHandling:
    """Test error handling for nested list operations."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_index_into_non_list(self):
        """Test attempting to index into non-list elements."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list simple = [hello, world]')
            self.repl._process_input('simple[0][1]')  # hello[1] - should fail
            
            output = captured_output.getvalue().strip()
            assert "cannot index into non-list" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_out_of_bounds_nested(self):
        """Test out-of-bounds errors in nested context."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list small = [[1, 2]]')
            self.repl._process_input('small[0][5]')  # Index 5 in [1, 2] - should fail
            
            output = captured_output.getvalue().strip()
            assert "out of range" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_nested_bounds_checking(self):
        """Test bounds checking at different nesting levels."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list data = [[1, 2], [3, 4]]')
            
            # Test first level out of bounds
            self.repl._process_input('data[5][0]')
            output = captured_output.getvalue().strip()
            assert "out of range" in output.lower()
            
            # Test second level out of bounds
            self.repl._process_input('data[0][5]')
            output = captured_output.getvalue().strip()
            assert "out of range" in output.lower()
        finally:
            sys.stdout = sys.__stdout__


class TestComplexNestedStructures:
    """Test complex nested list scenarios."""
    
    def setup_method(self):
        self.tokenizer = Tokenizer()
    
    def test_complex_nested_with_mixed_types(self):
        """Test complex structures with mixed types at different levels."""
        result = self.tokenizer.parse_list_literal_with_types(
            "[[1, 'text', true], [3.14, false, 'more text'], [42]]"
        )
        
        assert len(result) == 3
        assert result[0] == [1, 'text', True]
        assert result[1] == [3.14, False, 'more text']
        assert result[2] == [42]
        
        # Check types
        assert isinstance(result[0][0], int)
        assert isinstance(result[0][1], str)
        assert isinstance(result[0][2], bool)
        assert isinstance(result[1][0], float)
        assert isinstance(result[1][1], bool)
        assert isinstance(result[1][2], str)
    
    def test_deeply_nested_structure(self):
        """Test deeply nested structures."""
        result = self.tokenizer.parse_list_literal_with_types(
            "[[[1, 2], [3]], [[4, 5, 6]], [[[7]]]]"
        )
        
        assert result[0][0] == [1, 2]
        assert result[0][1] == [3]
        assert result[1][0] == [4, 5, 6]  
        assert result[2][0] == [[7]]
        assert result[2][0][0] == [7]
        assert result[2][0][0][0] == 7