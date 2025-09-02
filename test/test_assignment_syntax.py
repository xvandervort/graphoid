"""Tests for assignment syntax functionality (Phase 4A)."""

import pytest
from glang.parser import SyntaxParser, InputType, IndexAssignment, SliceAccess, SliceAssignment
from glang.repl import REPL
from io import StringIO
import sys


class TestIndexAssignmentParsing:
    """Test parsing of index assignment syntax."""
    
    def setup_method(self):
        self.parser = SyntaxParser()
    
    def test_parse_simple_index_assignment(self):
        """Test parsing simple index assignments."""
        result = self.parser.parse_input("fruits[0] = mango")
        assert isinstance(result, IndexAssignment)
        assert result.variable_name == "fruits"
        assert result.indices == [0]
        assert result.value == "mango"
    
    def test_parse_negative_index_assignment(self):
        """Test parsing negative index assignments."""
        result = self.parser.parse_input("items[-1] = last")
        assert isinstance(result, IndexAssignment)
        assert result.variable_name == "items"
        assert result.indices == [-1]
        assert result.value == "last"
    
    def test_parse_multi_dimensional_assignment(self):
        """Test parsing multi-dimensional index assignments."""
        result = self.parser.parse_input("matrix[1][2] = 42")
        assert isinstance(result, IndexAssignment)
        assert result.variable_name == "matrix"
        assert result.indices == [1, 2]
        assert result.value == 42
    
    def test_parse_assignment_with_types(self):
        """Test assignment with different types."""
        # Number assignment
        result = self.parser.parse_input("data[0] = 999")
        assert result.value == 999
        assert isinstance(result.value, int)
        
        # Boolean assignment
        result = self.parser.parse_input("flags[1] = true")
        assert result.value is True
        
        # String assignment (quoted strings preserve quotes in this implementation)
        result = self.parser.parse_input("names[2] = 'John Doe'")
        assert result.value == "'John Doe'"
    
    def test_index_assignment_detection(self):
        """Test detection of index assignment patterns."""
        assert self.parser._is_index_assignment("fruits[0] = apple")
        assert self.parser._is_index_assignment("matrix[1][2] = 42")
        assert self.parser._is_index_assignment("items[-1] = last")
        assert not self.parser._is_index_assignment("fruits[0]")  # Just access
        assert not self.parser._is_index_assignment("fruits = ['a', 'b']")  # Variable declaration


class TestSliceParsing:
    """Test parsing of slice syntax."""
    
    def setup_method(self):
        self.parser = SyntaxParser()
    
    def test_parse_slice_access_patterns(self):
        """Test parsing various slice access patterns."""
        # Basic slice [start:stop]
        result = self.parser.parse_input("data[1:4]")
        assert isinstance(result, SliceAccess)
        assert result.variable_name == "data"
        assert result.start == 1
        assert result.stop == 4
        assert result.step is None
        
        # Slice with step [start:stop:step]
        result = self.parser.parse_input("numbers[::2]")
        assert result.variable_name == "numbers"
        assert result.start is None
        assert result.stop is None
        assert result.step == 2
        
        # Open-ended slice [start:]
        result = self.parser.parse_input("items[2:]")
        assert result.variable_name == "items"
        assert result.start == 2
        assert result.stop is None
        assert result.step is None
        
        # Reverse slice [:-1]
        result = self.parser.parse_input("text[:-1]")
        assert result.variable_name == "text"
        assert result.start is None
        assert result.stop == -1
        assert result.step is None
    
    def test_parse_slice_assignment(self):
        """Test parsing slice assignments."""
        result = self.parser.parse_input("fruits[1:3] = ['apple', 'banana']")
        assert isinstance(result, SliceAssignment)
        assert result.variable_name == "fruits"
        assert result.start == 1
        assert result.stop == 3
        assert result.step is None
        assert result.value == ["apple", "banana"]
    
    def test_slice_detection(self):
        """Test slice pattern detection."""
        assert self.parser._is_slice_access("data[1:3]")
        assert self.parser._is_slice_access("items[::2]")
        assert self.parser._is_slice_access("text[2:]")
        assert self.parser._is_slice_access("list[:-1]")
        assert not self.parser._is_slice_access("data[1]")  # Index access
        assert not self.parser._is_slice_access("data")     # Variable access
        
        assert self.parser._is_slice_assignment("data[1:3] = ['a', 'b']")
        assert self.parser._is_slice_assignment("items[::2] = [1, 3, 5]")
        assert not self.parser._is_slice_assignment("data[1] = x")  # Index assignment


class TestAssignmentIntegration:
    """Test assignment integration with REPL."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_basic_index_assignment(self):
        """Test basic index assignment functionality."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list fruits = [\"apple\", \"banana\", \"cherry\"]')
            self.repl._process_input('fruits[0] = mango')
            self.repl._process_input('fruits')
            
            output = captured_output.getvalue()
            assert "Set fruits[0] = mango" in output
            assert "['mango', 'banana', 'cherry']" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_negative_index_assignment(self):
        """Test negative index assignment."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list data = [1, 2, 3, 4, 5]')
            self.repl._process_input('data[-1] = 999')
            self.repl._process_input('data')
            
            output = captured_output.getvalue()
            assert "Set data[-1] = 999" in output
            assert "[1, 2, 3, 4, 999]" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_multi_dimensional_assignment(self):
        """Test multi-dimensional assignment."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list matrix = [[1, 2], [3, 4]]')
            self.repl._process_input('matrix[0][1] = 99')
            self.repl._process_input('matrix')
            
            output = captured_output.getvalue()
            assert "Set matrix[0][1] = 99" in output
            assert "[[1, 99], [3, 4]]" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_slice_access(self):
        """Test slice access functionality."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list numbers = [1, 2, 3, 4, 5, 6, 7, 8]')
            self.repl._process_input('numbers[1:4]')
            self.repl._process_input('numbers[::2]')
            self.repl._process_input('numbers[2:]')
            
            output = captured_output.getvalue()
            assert "[2, 3, 4]" in output
            assert "[1, 3, 5, 7]" in output
            assert "[3, 4, 5, 6, 7, 8]" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_slice_assignment(self):
        """Test slice assignment functionality."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list data = [\"a\", \"b\", \"c\", \"d\", \"e\"]')
            self.repl._process_input('data[1:3] = [\"X\", \"Y\"]')
            self.repl._process_input('data')
            
            output = captured_output.getvalue()
            assert "Set data[1:3] = ['X', 'Y']" in output
            assert "['a', 'X', 'Y', 'd', 'e']" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_step_slice_assignment(self):
        """Test step slice assignment."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list numbers = [1, 2, 3, 4, 5, 6]')
            self.repl._process_input('numbers[::2] = [10, 30, 50]')
            self.repl._process_input('numbers')
            
            output = captured_output.getvalue()
            assert "Set numbers[::2] = [10, 30, 50]" in output
            assert "[10, 2, 30, 4, 50, 6]" in output
        finally:
            sys.stdout = sys.__stdout__


class TestAssignmentErrorHandling:
    """Test error handling for assignment operations."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_index_out_of_bounds_assignment(self):
        """Test assignment to out-of-bounds indices."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list small = [\"a\", \"b\", \"c\"]')
            self.repl._process_input('small[10] = x')
            
            output = captured_output.getvalue()
            assert "out of range" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_assignment_to_nonexistent_variable(self):
        """Test assignment to nonexistent variable."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('nonexistent[0] = value')
            
            output = captured_output.getvalue()
            assert "not found" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_multi_dim_assignment_to_non_list(self):
        """Test multi-dimensional assignment to non-list element."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list simple = [\"hello\", \"world\"]')
            self.repl._process_input('simple[0][1] = x')
            
            output = captured_output.getvalue()
            assert "cannot assign" in output.lower() or "non-list" in output.lower()
        finally:
            sys.stdout = sys.__stdout__
    
    def test_slice_on_empty_list(self):
        """Test slicing on empty list."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list empty = []')  # Create empty list
            self.repl._process_input('empty[1:3]')  # Slice the empty list
            
            output = captured_output.getvalue()
            # Empty slice should return empty list, not error
            assert "[]" in output
        finally:
            sys.stdout = sys.__stdout__


class TestAssignmentTypeSystem:
    """Test assignment integration with type system."""
    
    def setup_method(self):
        self.repl = REPL()
    
    def test_assignment_preserves_types(self):
        """Test that assignments preserve type inference."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list mixed = [1, \"hello\", true]')
            self.repl._process_input('mixed[0] = 999')
            self.repl._process_input('mixed[1] = world')  
            self.repl._process_input('mixed[2] = false')
            self.repl._process_input('mixed.types()')
            
            output = captured_output.getvalue()
            assert "num" in output
            assert "string" in output
            assert "bool" in output
        finally:
            sys.stdout = sys.__stdout__
    
    def test_slice_assignment_with_mixed_types(self):
        """Test slice assignment with mixed types."""
        captured_output = StringIO()
        sys.stdout = captured_output
        
        try:
            self.repl._process_input('list data = [\"a\", \"b\", \"c\", \"d\", \"e\"]')
            self.repl._process_input('data[1:4] = [42, true, 3.14]')
            self.repl._process_input('data')
            self.repl._process_input('data.types()')
            
            output = captured_output.getvalue()
            assert "['a', 42, true, 3.14, 'e']" in output
            assert "string" in output
            assert "num" in output
            assert "bool" in output
        finally:
            sys.stdout = sys.__stdout__


class TestAssignmentEdgeCases:
    """Test edge cases for assignment functionality."""
    
    def setup_method(self):
        self.parser = SyntaxParser()
        self.repl = REPL()
    
    def test_complex_slice_patterns(self):
        """Test complex slice patterns."""
        # Reverse slice with step
        result = self.parser.parse_input("data[::-1]")
        assert isinstance(result, SliceAccess)
        assert result.start is None
        assert result.stop is None
        assert result.step == -1
        
        # Complex slice
        result = self.parser.parse_input("items[-5:-1:2]")
        assert result.start == -5
        assert result.stop == -1
        assert result.step == 2
    
    def test_whitespace_handling(self):
        """Test that assignments handle whitespace correctly."""
        result = self.parser.parse_input("  fruits[0]   =   apple  ")
        assert isinstance(result, IndexAssignment)
        assert result.variable_name == "fruits"
        assert result.value == "apple"
        
        result = self.parser.parse_input("  data[ 1 : 3 ]  =  ['x', 'y']  ")
        assert isinstance(result, SliceAssignment)
        assert result.variable_name == "data"
        assert result.start == 1
        assert result.stop == 3
    
    def test_assignment_priority_over_access(self):
        """Test that assignment is detected over access when both patterns match."""
        # This should be detected as assignment, not access
        result = self.parser.parse_input("fruits[0] = apple")
        assert result.input_type == InputType.INDEX_ASSIGNMENT
        
        result = self.parser.parse_input("data[1:3] = ['a', 'b']")
        assert result.input_type == InputType.SLICE_ASSIGNMENT
        
        # These should be detected as access
        result = self.parser.parse_input("fruits[0]")
        assert result.input_type == InputType.INDEX_ACCESS
        
        result = self.parser.parse_input("data[1:3]")
        assert result.input_type == InputType.SLICE_ACCESS