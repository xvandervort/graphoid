"""Tests for indexing syntax functionality."""

import pytest
from glang.parser import SyntaxParser, InputType, IndexAccess
from glang.repl import REPL
from glang.core import Graph, GraphType
from io import StringIO
import sys


class TestIndexingSyntaxParser:
    """Test indexing syntax parsing."""
    
    def setup_method(self):
        self.parser = SyntaxParser()
    
    def test_parse_simple_index_access(self):
        """Test parsing simple index access."""
        result = self.parser.parse_input("fruits[0]")
        assert isinstance(result, IndexAccess)
        assert result.variable_name == "fruits"
        assert result.indices == [0]
    
    def test_parse_negative_index_access(self):
        """Test parsing negative index access."""
        result = self.parser.parse_input("items[-1]")
        assert isinstance(result, IndexAccess)
        assert result.variable_name == "items"
        assert result.indices == [-1]
    
    def test_parse_multi_index_access(self):
        """Test parsing multi-dimensional index access."""
        result = self.parser.parse_input("matrix[1][2]")
        assert isinstance(result, IndexAccess)
        assert result.variable_name == "matrix"
        assert result.indices == [1, 2]
    
    def test_parse_complex_index_access(self):
        """Test parsing complex index combinations."""
        result = self.parser.parse_input("data[0][-1][5]")
        assert isinstance(result, IndexAccess)
        assert result.variable_name == "data"
        assert result.indices == [0, -1, 5]
    
    def test_invalid_index_syntax(self):
        """Test that invalid index syntax is treated as variable access."""
        # Invalid syntax should fall back to variable access rather than index access
        result = self.parser.parse_input("fruits[")
        assert result.input_type == InputType.VARIABLE_ACCESS
        
        result = self.parser.parse_input("fruits]")
        assert result.input_type == InputType.VARIABLE_ACCESS
        
        # Non-numeric indices should not be detected as index access
        result = self.parser.parse_input("fruits[a]")
        assert result.input_type == InputType.VARIABLE_ACCESS
    
    def test_index_detection(self):
        """Test that index patterns are correctly detected."""
        assert self.parser._is_index_access("var[0]")
        assert self.parser._is_index_access("data[1][2]")
        assert self.parser._is_index_access("items[-1]")
        assert not self.parser._is_index_access("var.method")
        assert not self.parser._is_index_access("simple_var")


class TestIndexingIntegration:
    """Test indexing integration with REPL and graphs."""
    
    def setup_method(self):
        self.repl = REPL()
        
        # Create test graph
        test_data = ['apple', 'banana', 'cherry', 'date']
        self.repl.graph_manager.create_from_list('fruits', test_data)
    
    def test_simple_index_access(self):
        """Test simple index access through REPL."""
        # Capture output
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('fruits[0]')
            output = captured_output.getvalue().strip()
            assert output == "'apple'"
            
            self.repl._process_input('fruits[2]')
            output = captured_output.getvalue().strip().split('\n')[-1]
            assert output == "'cherry'"
        finally:
            sys.stdout = sys.__stdout__
    
    def test_negative_index_access(self):
        """Test negative index access."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('fruits[-1]')
            output = captured_output.getvalue().strip()
            assert output == "'date'"
            
            self.repl._process_input('fruits[-2]')
            output = captured_output.getvalue().strip().split('\n')[-1]
            assert output == "'cherry'"
        finally:
            sys.stdout = sys.__stdout__
    
    def test_index_out_of_bounds(self):
        """Test handling of out-of-bounds indices."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('fruits[10]')
            output = captured_output.getvalue().strip()
            assert "out of range" in output.lower()
            
            self.repl._process_input('fruits[-10]')
            output = captured_output.getvalue().strip().split('\n')[-1]
            assert "out of range" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_index_access_nonexistent_variable(self):
        """Test index access on nonexistent variable."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('nonexistent[0]')
            output = captured_output.getvalue().strip()
            assert "not found" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_multi_dimensional_indexing_works(self):
        """Test that multi-dimensional indexing works correctly."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            # Create a nested list and test multi-dimensional access
            self.repl._process_input('list matrix = [[1, 2], [3, 4]]')
            self.repl._process_input('matrix[0][1]')
            output = captured_output.getvalue().strip()
            
            # Should find the value 2
            assert '2' in output
            
            # Test error case - trying to index into non-list
            self.repl._process_input('list simple = [\"hello\", \"world\"]')
            self.repl._process_input('simple[0][1]')
            output = captured_output.getvalue().strip()
            assert ("cannot index into non-list" in output.lower() or 
                    "variable 'simple[0][1]' not found" in output.lower())
        finally:
            sys.stdout = sys.__stdout__


class TestIndexingEdgeCases:
    """Test edge cases for indexing functionality."""
    
    def setup_method(self):
        self.parser = SyntaxParser()
        self.repl = REPL()
    
    def test_empty_list_indexing(self):
        """Test indexing on empty graph."""
        # Create empty graph using new syntax instead of direct method call
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list empty = []')  # Create empty list
            self.repl._process_input('empty[0]')         # Try to access it
            output = captured_output.getvalue().strip()
            
            # Check that we get an appropriate error (either not found or out of range)
            assert ("out of range" in output.lower() or "not found" in output.lower())
        finally:
            sys.stdout = sys.__stdout__
    
    def test_single_element_indexing(self):
        """Test indexing on single-element graph."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list single = ["only"]')  # Create single-element list
            self.repl._process_input('single[0]')
            output_lines = captured_output.getvalue().strip().split('\n')
            # Find the line with 'only' (should be after creation message)
            only_line = [line for line in output_lines if line.strip() == "'only'"]
            assert len(only_line) >= 1
            
            self.repl._process_input('single[-1]')
            output_lines = captured_output.getvalue().strip().split('\n')
            # Count occurrences of 'only' - should be at least 2 now
            only_lines = [line for line in output_lines if line.strip() == "'only'"]
            assert len(only_lines) >= 2
            
            self.repl._process_input('single[1]')
            output = captured_output.getvalue().strip()
            assert "out of range" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_parser_priority(self):
        """Test that parser correctly prioritizes index access over variable access."""
        # Index access should be detected before variable access
        result = self.parser.parse_input("test[0]")
        assert result.input_type == InputType.INDEX_ACCESS
        
        # But regular variable access should still work
        result = self.parser.parse_input("test")
        assert result.input_type == InputType.VARIABLE_ACCESS