"""Tests for type system functionality."""

import pytest
from glang.parser.tokenizer import Tokenizer
from glang.repl import REPL
from io import StringIO
import sys


class TestTypeInference:
    """Test automatic type inference."""
    
    def setup_method(self):
        self.tokenizer = Tokenizer()
    
    def test_infer_integers(self):
        """Test integer type inference."""
        assert self.tokenizer.infer_value_type("42") == 42
        assert self.tokenizer.infer_value_type("-10") == -10
        assert self.tokenizer.infer_value_type("0") == 0
        assert isinstance(self.tokenizer.infer_value_type("42"), int)
    
    def test_infer_floats(self):
        """Test float type inference."""
        assert self.tokenizer.infer_value_type("3.14") == 3.14
        assert self.tokenizer.infer_value_type("-2.5") == -2.5
        assert self.tokenizer.infer_value_type("0.0") == 0.0
        assert isinstance(self.tokenizer.infer_value_type("3.14"), float)
    
    def test_infer_booleans(self):
        """Test boolean type inference."""
        assert self.tokenizer.infer_value_type("true") is True
        assert self.tokenizer.infer_value_type("false") is False
        assert self.tokenizer.infer_value_type("TRUE") is True
        assert self.tokenizer.infer_value_type("False") is False
        assert isinstance(self.tokenizer.infer_value_type("true"), bool)
    
    def test_infer_strings(self):
        """Test string type inference."""
        assert self.tokenizer.infer_value_type("hello") == "hello"
        assert self.tokenizer.infer_value_type("world123") == "world123"
        assert self.tokenizer.infer_value_type("") == ""
        assert isinstance(self.tokenizer.infer_value_type("hello"), str)
    
    def test_parse_mixed_list(self):
        """Test parsing list with mixed types."""
        result = self.tokenizer.parse_list_literal_with_types("[42, hello, true, 3.14]")
        expected = [42, "hello", True, 3.14]
        
        assert result == expected
        assert isinstance(result[0], int)
        assert isinstance(result[1], str)
        assert isinstance(result[2], bool)
        assert isinstance(result[3], float)
    
    def test_parse_quoted_strings_in_list(self):
        """Test parsing quoted strings in lists."""
        result = self.tokenizer.parse_list_literal_with_types("['hello world', \"quoted\", unquoted]")
        expected = ["hello world", "quoted", "unquoted"]
        
        assert result == expected
        assert all(isinstance(item, str) for item in result)
    
    def test_empty_list_parsing(self):
        """Test parsing empty lists."""
        result = self.tokenizer.parse_list_literal_with_types("[]")
        assert result == []
    
    def test_complex_mixed_list(self):
        """Test parsing complex mixed-type list."""
        result = self.tokenizer.parse_list_literal_with_types("[0, 'start', true, 3.14, false, -42, 'end']")
        expected = [0, "start", True, 3.14, False, -42, "end"]
        
        assert result == expected
        assert [type(item).__name__ for item in result] == ['int', 'str', 'bool', 'float', 'bool', 'int', 'str']


class TestTypeMethods:
    """Test type introspection methods."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_types_method(self):
        """Test the types() method on mixed data."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list mixed = [42, hello, true, 3.14, false]')
            self.repl._process_input('mixed.types()')
            output_lines = captured_output.getvalue().strip().split('\n')
            
            # Find the line with type information
            types_line = None
            for line in output_lines:
                if '[' in line and 'num' in line:
                    types_line = line
                    break
            
            assert types_line is not None
            assert 'num' in types_line
            assert 'string' in types_line
            assert 'bool' in types_line
        finally:
            sys.stdout = sys.__stdout__
    
    def test_typeof_method(self):
        """Test the typeof method on specific indices."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list data = [100, test, true]')
            self.repl._process_input('data.typeof 0')
            self.repl._process_input('data.typeof 1')
            self.repl._process_input('data.typeof 2')
            
            output_lines = captured_output.getvalue().strip().split('\n')
            
            # Find type responses (skip creation message)
            type_responses = [line for line in output_lines if line.strip() in ['num', 'string', 'bool']]
            
            assert len(type_responses) >= 3
            assert 'num' in type_responses
            assert 'string' in type_responses
            assert 'bool' in type_responses
        finally:
            sys.stdout = sys.__stdout__
    
    def test_typeof_with_negative_index(self):
        """Test typeof with negative indexing."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list items = [42, test]')
            self.repl._process_input('items.typeof -1')  # Should be 'string' (test)
            self.repl._process_input('items.typeof -2')  # Should be 'num' (42)
            
            output_lines = captured_output.getvalue().strip().split('\n')
            type_responses = [line for line in output_lines if line.strip() in ['num', 'string', 'bool']]
            
            assert len(type_responses) >= 2
        finally:
            sys.stdout = sys.__stdout__
    
    def test_typeof_out_of_bounds(self):
        """Test typeof with out-of-bounds index."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list small = [42]')
            self.repl._process_input('small.typeof 10')  # Out of bounds
            
            output = captured_output.getvalue().strip()
            assert "out of range" in output.lower()
        finally:
            sys.stdout = sys.__stdout__


class TestTypeSystemIntegration:
    """Test integration of type system with other features."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_mixed_types_with_indexing(self):
        """Test that indexing works correctly with different types."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list mixed = [42, hello, true, 3.14]')
            self.repl._process_input('mixed[0]')  # Should show 42
            self.repl._process_input('mixed[1]')  # Should show hello
            self.repl._process_input('mixed[2]')  # Should show True
            self.repl._process_input('mixed[3]')  # Should show 3.14
            
            output_lines = captured_output.getvalue().strip().split('\n')
            
            # Check that we can find the expected values
            assert '42' in output_lines
            assert 'hello' in output_lines
            assert 'True' in output_lines  # Python boolean repr
            assert '3.14' in output_lines
        finally:
            sys.stdout = sys.__stdout__
    
    def test_type_preservation_through_methods(self):
        """Test that types are preserved when using graph methods."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list nums = [1, 2, 3]')
            self.repl._process_input('nums.append 4')     # Add integer
            self.repl._process_input('nums.append true')  # Add boolean
            self.repl._process_input('nums.types()')      # Check types
            
            output = captured_output.getvalue().strip()
            
            # Should contain type information showing preservation
            assert 'num' in output
            assert 'bool' in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_legacy_syntax_type_support(self):
        """Test that legacy create syntax also supports type inference."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('create legacy [42, test, true]')
            self.repl._process_input('legacy.types()')
            
            output = captured_output.getvalue().strip()
            assert 'num' in output
            assert 'string' in output
            assert 'bool' in output
        finally:
            sys.stdout = sys.__stdout__


class TestTypeEdgeCases:
    """Test edge cases for type system."""
    
    def setup_method(self):
        self.tokenizer = Tokenizer()
    
    def test_edge_case_numbers(self):
        """Test edge cases for number parsing."""
        # These should be parsed as numbers
        assert isinstance(self.tokenizer.infer_value_type("0"), int)
        assert isinstance(self.tokenizer.infer_value_type("-0"), int)
        assert isinstance(self.tokenizer.infer_value_type("00"), int)
        
        # These should be parsed as floats
        assert isinstance(self.tokenizer.infer_value_type("0.0"), float)
        assert isinstance(self.tokenizer.infer_value_type("-0.0"), float)
    
    def test_numeric_strings(self):
        """Test strings that look like numbers but aren't."""
        # These should remain as strings because of quotes
        result = self.tokenizer.parse_list_literal_with_types("['42', \"3.14\", 'true']")
        assert result == ["42", "3.14", "true"]
        assert all(isinstance(item, str) for item in result)
    
    def test_boolean_case_variations(self):
        """Test different case variations of booleans."""
        variations = ["true", "TRUE", "True", "false", "FALSE", "False"]
        results = [self.tokenizer.infer_value_type(v) for v in variations]
        
        expected = [True, True, True, False, False, False]
        assert results == expected
        assert all(isinstance(r, bool) for r in results)