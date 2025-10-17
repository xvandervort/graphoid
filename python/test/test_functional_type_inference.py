"""Tests for type inference with functional operations."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestFunctionalTypeInference:
    """Test that functional operations work with type inference."""
    
    def test_filter_explicit_vs_inferred(self):
        """Test that explicit and inferred types work the same for filter."""
        session = ExecutionSession()
        session.execute_statement('numbers = [1, 2, 3, 4, 5]')
        
        # Explicit type declaration
        result1 = session.execute_statement('list explicit = numbers.filter("even")')
        assert result1.success
        
        # Inferred type declaration 
        result2 = session.execute_statement('inferred = numbers.filter("even")')
        assert result2.success
        
        # Both should have the same result
        result1_val = session.execute_statement('explicit')
        result2_val = session.execute_statement('inferred')
        
        assert result1_val.success and result2_val.success
        assert [elem.value for elem in result1_val.value.elements] == [elem.value for elem in result2_val.value.elements]
        assert result1_val.value.elements == result2_val.value.elements
    
    def test_map_explicit_vs_inferred(self):
        """Test that explicit and inferred types work the same for map."""
        session = ExecutionSession()
        session.execute_statement('numbers = [1, 2, 3]')
        
        # Explicit type declaration
        result1 = session.execute_statement('list explicit = numbers.map("double")')
        assert result1.success
        
        # Inferred type declaration
        result2 = session.execute_statement('inferred = numbers.map("double")')
        assert result2.success
        
        # Both should have the same result
        result1_val = session.execute_statement('explicit')
        result2_val = session.execute_statement('inferred')
        
        assert result1_val.success and result2_val.success
        assert [elem.value for elem in result1_val.value.elements] == [2, 4, 6]
        assert [elem.value for elem in result2_val.value.elements] == [2, 4, 6]
    
    def test_chaining_with_inference(self):
        """Test method chaining with type inference."""
        session = ExecutionSession()
        session.execute_statement('numbers = [1, 2, 3, 4, 5, 6]')
        
        # Chain with inferred type
        result = session.execute_statement('result = numbers.filter("positive").map("double").filter("even")')
        assert result.success
        
        # Check the result
        result_val = session.execute_statement('result')
        assert result_val.success
        assert [elem.value for elem in result_val.value.elements] == [2, 4, 6, 8, 10, 12]
    
    def test_each_inference(self):
        """Test that each() returns the original list type."""
        session = ExecutionSession()
        session.execute_statement('numbers = [1, 2, 3]')
        
        result = session.execute_statement('same = numbers.each("print")')
        assert result.success
        
        # Should return the original list
        result_val = session.execute_statement('same')
        assert result_val.success
        assert [elem.value for elem in result_val.value.elements] == [1, 2, 3]
    
    def test_string_method_inference(self):
        """Test string method return type inference."""
        session = ExecutionSession()
        session.execute_statement('name = "Alice"')
        
        # String method returning string
        result = session.execute_statement('upper = name.up()')
        assert result.success
        
        result_val = session.execute_statement('upper')
        assert result.success
        assert result_val.value.value == "ALICE"
        
        # String method returning list
        result = session.execute_statement('chars = name.chars()')
        assert result.success
        
        result_val = session.execute_statement('chars')
        assert result.success
        assert len(result_val.value.elements) == 5  # ['A', 'l', 'i', 'c', 'e']
    
    def test_hash_method_inference(self):
        """Test hash method return type inference."""
        session = ExecutionSession()
        session.execute_statement('config = {"host": "localhost", "port": 8080}')
        
        # Method returning list
        result = session.execute_statement('keys = config.keys()')
        assert result.success
        
        result_val = session.execute_statement('keys')
        assert result.success
        assert len(result_val.value.elements) == 2
        
        # Method returning boolean
        result = session.execute_statement('has_debug = config.has_key("debug")')
        assert result.success
        
        result_val = session.execute_statement('has_debug')
        assert result.success
        assert result_val.value.value is False