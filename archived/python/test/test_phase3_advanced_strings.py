"""
Tests for Phase 3: Advanced String Graph Operations + REPL Reflection.

This module tests the advanced string operations and reflection commands
implemented in Phase 3.
"""

import pytest
import sys
import os
from io import StringIO
from contextlib import redirect_stdout

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession


class TestAdvancedStringOperations:
    """Test advanced string operations from Phase 3."""
    
    def test_trim_method(self):
        """Test string trim() method."""
        session = ExecutionSession()
        
        # Test trimming whitespace
        result = session.execute_statement('string messy = "  Hello World  "')
        assert result.success
        
        result = session.execute_statement('string clean = messy.trim()')
        assert result.success
        
        # Check result
        result = session.execute_statement('clean')
        assert result.success
        assert result.value.value == "Hello World"
        
        # Check original unchanged (immutable)
        result = session.execute_statement('messy')
        assert result.success
        assert result.value.value == "  Hello World  "
        
        # Test empty string
        result = session.execute_statement('string empty = ""')
        assert result.success
        result = session.execute_statement('empty.trim()')
        assert result.success
        assert result.value.value == ""
        
        # Test no arguments
        result = session.execute_statement('messy.trim("invalid")')
        assert not result.success
    
    def test_join_method(self):
        """Test string join() method."""
        session = ExecutionSession()
        
        # Test joining string list
        result = session.execute_statement('list<string> words = ["apple", "banana", "cherry"]')
        assert result.success
        
        result = session.execute_statement('string separator = ", "')
        assert result.success
        
        result = session.execute_statement('string joined = separator.join(words)')
        assert result.success
        
        # Check result
        result = session.execute_statement('joined')
        assert result.success
        assert result.value.value == "apple, banana, cherry"
        
        # Test joining number list
        result = session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        assert result.success
        
        result = session.execute_statement('string dash = "-"')
        assert result.success
        
        result = session.execute_statement('string num_result = dash.join(numbers)')
        assert result.success
        
        result = session.execute_statement('num_result')
        assert result.success
        assert result.value.value == "1-2-3-4-5"
        
        # Test empty list
        result = session.execute_statement('list<string> empty_list = []')
        assert result.success
        result = session.execute_statement('separator.join(empty_list)')
        assert result.success
        assert result.value.value == ""
        
        # Test error cases
        result = session.execute_statement('separator.join("not_a_list")')
        assert not result.success
        
        result = session.execute_statement('separator.join()')
        assert not result.success
    
    def test_pattern_operations(self):
        """Test regex pattern operations."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "Hello World 123"')
        assert result.success
        
        # Test matches
        result = session.execute_statement('bool has_numbers = text.matches("[0-9]+")')
        assert result.success
        
        result = session.execute_statement('has_numbers')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('bool starts_hello = text.matches("^Hello")')
        assert result.success
        result = session.execute_statement('starts_hello')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('bool has_xyz = text.matches("xyz")')
        assert result.success
        result = session.execute_statement('has_xyz')
        assert result.success
        assert result.value.value == False
        
        # Test replace
        result = session.execute_statement('string no_numbers = text.replace("[0-9]+", "XXX")')
        assert result.success
        result = session.execute_statement('no_numbers')
        assert result.success
        assert result.value.value == "Hello World XXX"
        
        result = session.execute_statement('string lowercase_replaced = text.replace("[a-z]", "X")')
        assert result.success
        result = session.execute_statement('lowercase_replaced')
        assert result.success
        assert result.value.value == "HXXXX WXXXX 123"
        
        # Original unchanged
        result = session.execute_statement('text')
        assert result.success
        assert result.value.value == "Hello World 123"
        
        # Test findAll
        result = session.execute_statement('list<string> words = text.findAll("[A-Za-z]+")')
        assert result.success
        result = session.execute_statement('words')
        assert result.success
        word_values = [elem.value for elem in result.value.elements]
        assert word_values == ["Hello", "World"]
        
        result = session.execute_statement('list<string> numbers = text.findAll("[0-9]+")')
        assert result.success
        result = session.execute_statement('numbers')
        assert result.success
        number_values = [elem.value for elem in result.value.elements]
        assert number_values == ["123"]
        
        # Test no matches
        result = session.execute_statement('list<string> no_match = text.findAll("xyz")')
        assert result.success
        result = session.execute_statement('no_match')
        assert result.success
        assert len(result.value.elements) == 0
    
    def test_invalid_regex_patterns(self):
        """Test error handling for invalid regex patterns."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "Hello World"')
        assert result.success
        
        # Invalid regex pattern
        result = session.execute_statement('text.matches("[")')  # Unclosed bracket
        assert not result.success
        assert "Invalid regex pattern" in str(result.error)
        
        result = session.execute_statement('text.replace("[", "X")')
        assert not result.success
        assert "Invalid regex pattern" in str(result.error)
        
        result = session.execute_statement('text.findAll("[")')
        assert not result.success
        assert "Invalid regex pattern" in str(result.error)


class TestAdvancedIndexingSlicing:
    """Test advanced indexing and slicing features."""
    
    def test_negative_indexing(self):
        """Test negative indexing for strings and lists."""
        session = ExecutionSession()
        
        # String negative indexing
        result = session.execute_statement('string text = "Hello"')
        assert result.success
        
        result = session.execute_statement('text[-1]')
        assert result.success
        assert result.value.value == "o"
        
        result = session.execute_statement('text[-2]')
        assert result.success
        assert result.value.value == "l"
        
        # List negative indexing
        result = session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        assert result.success
        
        result = session.execute_statement('numbers[-1]')
        assert result.success
        assert result.value.value == 5
        
        result = session.execute_statement('numbers[-2]')
        assert result.success
        assert result.value.value == 4
    
    def test_step_slicing(self):
        """Test step slicing for strings and lists."""
        session = ExecutionSession()
        
        # String step slicing
        result = session.execute_statement('string text = "Hello World"')
        assert result.success
        
        result = session.execute_statement('text[::2]')
        assert result.success
        assert result.value.value == "HloWrd"
        
        result = session.execute_statement('text[::-1]')
        assert result.success
        assert result.value.value == "dlroW olleH"
        
        result = session.execute_statement('text[1::2]')
        assert result.success
        assert result.value.value == "el ol"
        
        # List step slicing
        result = session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]')
        assert result.success
        
        result = session.execute_statement('numbers[::2]')
        assert result.success
        values = [elem.value for elem in result.value.elements]
        assert values == [1, 3, 5, 7, 9]
        
        result = session.execute_statement('numbers[::3]')
        assert result.success
        values = [elem.value for elem in result.value.elements]
        assert values == [1, 4, 7, 10]
        
        result = session.execute_statement('numbers[::-1]')
        assert result.success
        values = [elem.value for elem in result.value.elements]
        assert values == [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]


class TestPhase3Integration:
    """Test integration of all Phase 3 features."""
    
    def test_comprehensive_string_workflow(self):
        """Test a comprehensive workflow using all Phase 3 string features."""
        session = ExecutionSession()
        
        # Setup
        result = session.execute_statement('string raw_data = "  apple,banana,cherry  "')
        assert result.success
        
        # Clean and split
        result = session.execute_statement('string clean_data = raw_data.trim()')
        assert result.success
        
        result = session.execute_statement('list<string> fruits = clean_data.split(",")')
        assert result.success
        
        # Verify fruits
        result = session.execute_statement('fruits')
        assert result.success
        fruit_values = [elem.value for elem in result.value.elements]
        assert fruit_values == ["apple", "banana", "cherry"]
        
        # Join with different separator
        result = session.execute_statement('string result = " | ".join(fruits)')
        assert result.success
        result = session.execute_statement('result')
        assert result.success
        assert result.value.value == "apple | banana | cherry"
        
        # Pattern matching
        result = session.execute_statement('bool has_apple = result.matches("apple")')
        assert result.success
        result = session.execute_statement('has_apple')
        assert result.success
        assert result.value.value == True
        
        # Replace
        result = session.execute_statement('string no_apple = result.replace("apple", "orange")')
        assert result.success
        result = session.execute_statement('no_apple')
        assert result.success
        assert result.value.value == "orange | banana | cherry"
        
        # Find all fruits (assuming they're alphabetic words)
        result = session.execute_statement('list<string> found_fruits = no_apple.findAll("[a-z]+")')
        assert result.success
        result = session.execute_statement('found_fruits')
        assert result.success
        found_values = [elem.value for elem in result.value.elements]
        assert found_values == ["orange", "banana", "cherry"]


class TestNewMethodDiscoverability:
    """Test that new methods appear in reflection system."""
    
    def test_string_methods_in_reflection(self):
        """Test that new string methods appear in methods() list."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "Hello"')
        assert result.success
        
        result = session.execute_statement('text.methods()')
        assert result.success
        
        methods = [elem.value for elem in result.value.elements]
        
        # Check that new Phase 3 methods are present
        assert "trim" in methods
        assert "join" in methods
        assert "matches" in methods
        assert "replace" in methods
        assert "findAll" in methods
        
        # Check that existing methods are still present
        assert "length" in methods
        assert "contains" in methods
        assert "up" in methods
        assert "split" in methods
    
    def test_can_method_recognizes_new_methods(self):
        """Test that can() method recognizes new string methods."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "Hello"')
        assert result.success
        
        # Test new methods
        result = session.execute_statement('text.can("trim")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.can("join")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.can("matches")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.can("replace")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.can("findAll")')
        assert result.success
        assert result.value.value == True
        
        # Test invalid method
        result = session.execute_statement('text.can("nonexistent")')
        assert result.success
        assert result.value.value == False


if __name__ == "__main__":
    pytest.main([__file__, "-v"])