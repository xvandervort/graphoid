"""Tests for Phase 2: Proper Variable Reference and Assignment functionality."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.repl.graph_manager import GraphManager
from glang.parser import SyntaxParser, ExpressionEvaluator, InputType
from glang.core import AtomicValue


class TestPhase2Assignments:
    """Test suite for Phase 2 assignment functionality."""
    
    def setup_method(self):
        """Set up test fixtures."""
        self.graph_manager = GraphManager()
        self.parser = SyntaxParser()
        self.evaluator = ExpressionEvaluator(self.graph_manager)
    
    def test_basic_index_assignment(self):
        """Test basic index assignment: string b = a[0]"""
        # Create source list
        self.graph_manager.create_from_list('a', ['one', 'two', 'three'])
        
        # Evaluate index expression
        result = self.evaluator.evaluate_expression('a[0]')
        assert result == 'one'
        
        result = self.evaluator.evaluate_expression('a[1]')
        assert result == 'two'
        
        result = self.evaluator.evaluate_expression('a[2]')
        assert result == 'three'
    
    def test_negative_indexing(self):
        """Test negative indexing: string b = a[-1]"""
        # Create source list
        self.graph_manager.create_from_list('nums', [10, 20, 30])
        
        # Test negative indexing
        result = self.evaluator.evaluate_expression('nums[-1]')
        assert result == 30
        
        result = self.evaluator.evaluate_expression('nums[-2]')
        assert result == 20
        
        result = self.evaluator.evaluate_expression('nums[-3]')
        assert result == 10
    
    def test_chained_indexing(self):
        """Test chained indexing: string val = matrix[0][1]"""
        # Create nested list structure
        matrix_data = [['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', 'i']]
        self.graph_manager.create_from_list('matrix', matrix_data)
        
        # Test chained indexing
        result = self.evaluator.evaluate_expression('matrix[0][1]')
        assert result == 'b'
        
        result = self.evaluator.evaluate_expression('matrix[1][2]')
        assert result == 'f'
        
        result = self.evaluator.evaluate_expression('matrix[2][0]')
        assert result == 'g'
    
    def test_complex_chained_indexing(self):
        """Test more complex chained indexing patterns."""
        # Create 3D-like nested structure
        data = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]]
        self.graph_manager.create_from_list('data3d', data)
        
        # Test deep indexing
        result = self.evaluator.evaluate_expression('data3d[0][1][1]')
        assert result == 4
        
        result = self.evaluator.evaluate_expression('data3d[1][0][0]')
        assert result == 5
    
    def test_simple_variable_reference(self):
        """Test simple variable reference in assignment."""
        # Create atomic value
        self.graph_manager.create_atomic_value('name', 'Alice', 'string')
        
        # Test variable reference
        result = self.evaluator.evaluate_expression('name')
        assert result == 'Alice'
        
        # Create single-element graph
        self.graph_manager.create_from_list('single', ['hello'])
        result = self.evaluator.evaluate_expression('single')
        assert result == 'hello'
    
    def test_error_handling_out_of_bounds(self):
        """Test error handling for out-of-bounds access."""
        # Create small list
        self.graph_manager.create_from_list('small', ['a', 'b'])
        
        # Test positive out-of-bounds
        with pytest.raises(ValueError, match="Index 2 out of range"):
            self.evaluator.evaluate_expression('small[2]')
        
        with pytest.raises(ValueError, match="Index 5 out of range"):
            self.evaluator.evaluate_expression('small[5]')
        
        # Test negative out-of-bounds
        with pytest.raises(ValueError, match="Index -3 out of range"):
            self.evaluator.evaluate_expression('small[-3]')
    
    def test_error_handling_nonexistent_variable(self):
        """Test error handling for non-existent variables."""
        with pytest.raises(ValueError, match="Variable 'nonexistent' not found"):
            self.evaluator.evaluate_expression('nonexistent[0]')
        
        with pytest.raises(ValueError, match="Variable 'missing' not found"):
            self.evaluator.evaluate_expression('missing')
    
    def test_error_handling_atomic_indexing(self):
        """Test error handling when trying to index atomic values."""
        # Create atomic value
        self.graph_manager.create_atomic_value('atomic_str', 'hello', 'string')
        
        # Try to index it (should fail)
        with pytest.raises(ValueError, match="Cannot index into atomic value"):
            self.evaluator.evaluate_expression('atomic_str[0]')
    
    def test_error_handling_non_list_chaining(self):
        """Test error handling when trying to chain index non-list values."""
        # Create list with non-list elements
        self.graph_manager.create_from_list('mixed', ['hello', 42, True])
        
        # Try to chain index a string (should fail)
        with pytest.raises(ValueError, match="Cannot index into non-list type"):
            self.evaluator.evaluate_expression('mixed[0][1]')
        
        # Try to chain index a number (should fail)
        with pytest.raises(ValueError, match="Cannot index into non-list type"):
            self.evaluator.evaluate_expression('mixed[1][0]')
    
    def test_error_handling_multi_element_to_scalar(self):
        """Test error handling when trying to assign multi-element graph to scalar."""
        # Create multi-element graph
        self.graph_manager.create_from_list('multi', ['a', 'b', 'c'])
        
        # Try to assign whole graph to scalar (should fail)
        with pytest.raises(ValueError, match="Cannot assign multi-element graph"):
            self.evaluator.evaluate_expression('multi')
    
    def test_expression_type_detection(self):
        """Test expression type detection methods."""
        # Test index expression detection
        assert self.evaluator._is_index_expression('var[0]') == True
        assert self.evaluator._is_index_expression('matrix[1][2]') == True
        assert self.evaluator._is_index_expression('data[0][1][2]') == True
        assert self.evaluator._is_index_expression('var[-1]') == True
        
        # Test non-index expressions
        assert self.evaluator._is_index_expression('var') == False
        assert self.evaluator._is_index_expression('hello') == False
        assert self.evaluator._is_index_expression('var[') == False
        assert self.evaluator._is_index_expression('var]') == False
        
        # Test simple variable detection
        assert self.evaluator._is_simple_variable('var') == True
        assert self.evaluator._is_simple_variable('hello_world') == True
        assert self.evaluator._is_simple_variable('var123') == True
        assert self.evaluator._is_simple_variable('_private') == True
        
        # Test non-simple variables
        assert self.evaluator._is_simple_variable('var[0]') == False
        assert self.evaluator._is_simple_variable('123var') == False
        assert self.evaluator._is_simple_variable('var.method') == False
    
    def test_mixed_type_indexing(self):
        """Test indexing with mixed data types."""
        # Create list with mixed types
        mixed_data = [42, 'hello', True, [1, 2, 3]]
        self.graph_manager.create_from_list('mixed', mixed_data)
        
        # Test accessing different types
        result = self.evaluator.evaluate_expression('mixed[0]')
        assert result == 42
        
        result = self.evaluator.evaluate_expression('mixed[1]')
        assert result == 'hello'
        
        result = self.evaluator.evaluate_expression('mixed[2]')
        assert result == True
        
        # Test chained access into nested list
        result = self.evaluator.evaluate_expression('mixed[3][1]')
        assert result == 2
    
    def test_deep_nesting(self):
        """Test very deep nesting scenarios."""
        # Create deeply nested structure
        deep = [[[[['deep_value']]]]]
        self.graph_manager.create_from_list('deep', deep)
        
        # Test accessing deeply nested value
        # Note: This tests the limit of our current implementation
        # We can access the first level through graph, then subsequent levels through list indexing
        result = self.evaluator.evaluate_expression('deep[0][0][0][0][0]')
        assert result == 'deep_value'


class TestIntegrationWithREPL:
    """Integration tests that simulate REPL behavior."""
    
    def setup_method(self):
        """Set up test fixtures."""
        from glang.repl import REPL
        self.repl = REPL()
    
    def test_end_to_end_scalar_assignment(self):
        """Test end-to-end scalar assignment through REPL parsing."""
        # Create source data
        parsed = self.repl.syntax_parser.parse_input('list source = ["hello", "world"]')
        assert parsed.input_type == InputType.VARIABLE_DECLARATION
        self.repl._handle_variable_declaration(parsed)
        
        # Verify source was created
        source = self.repl.graph_manager.get_variable('source')
        assert source is not None
        assert len(source) == 2
        
        # Test scalar assignment with indexing
        parsed = self.repl.syntax_parser.parse_input('string result = source[1]')
        assert parsed.input_type == InputType.VARIABLE_DECLARATION
        self.repl._handle_variable_declaration(parsed)
        
        # Verify result was created correctly
        result = self.repl.graph_manager.get_variable('result')
        assert isinstance(result, AtomicValue)
        assert result.value == 'world'
        assert result.atomic_type == 'string'
    
    def test_end_to_end_chained_assignment(self):
        """Test end-to-end chained assignment."""
        # Create matrix data
        parsed = self.repl.syntax_parser.parse_input('list matrix = [["a", "b"], ["c", "d"]]')
        self.repl._handle_variable_declaration(parsed)
        
        # Test chained assignment
        parsed = self.repl.syntax_parser.parse_input('string cell = matrix[1][0]')
        self.repl._handle_variable_declaration(parsed)
        
        # Verify result
        cell = self.repl.graph_manager.get_variable('cell')
        assert isinstance(cell, AtomicValue)
        assert cell.value == 'c'
    
    def test_end_to_end_numeric_assignment(self):
        """Test end-to-end numeric assignment."""
        # Create numeric data
        parsed = self.repl.syntax_parser.parse_input('list numbers = [100, 200, 300]')
        self.repl._handle_variable_declaration(parsed)
        
        # Test numeric assignment
        parsed = self.repl.syntax_parser.parse_input('num value = numbers[2]')
        self.repl._handle_variable_declaration(parsed)
        
        # Verify result
        value = self.repl.graph_manager.get_variable('value')
        assert isinstance(value, AtomicValue)
        assert value.value == 300
        assert value.atomic_type == 'num'
    
    def test_end_to_end_boolean_assignment(self):
        """Test end-to-end boolean assignment."""
        # Create boolean data
        parsed = self.repl.syntax_parser.parse_input('list flags = [true, false, true]')
        self.repl._handle_variable_declaration(parsed)
        
        # Test boolean assignment
        parsed = self.repl.syntax_parser.parse_input('bool flag = flags[1]')
        self.repl._handle_variable_declaration(parsed)
        
        # Verify result
        flag = self.repl.graph_manager.get_variable('flag')
        assert isinstance(flag, AtomicValue)
        assert flag.value == False
        assert flag.atomic_type == 'bool'


if __name__ == '__main__':
    pytest.main([__file__])