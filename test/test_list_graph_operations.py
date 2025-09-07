"""
Tests for list graph operations (analysis, transformation, and combinations).

This module tests the list operations that treat lists as ordered graph collections,
including analysis methods, transformations, and graph combinations.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.values import NumberValue, StringValue, BooleanValue, ListValue
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import SourcePosition


class TestListAnalysisMethods:
    """Test list analysis methods that examine graph properties."""
    
    def test_indexOf_basic(self):
        """Test indexOf finds correct index of elements."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [10, 20, 30, 20]')
        assert result.success
        
        # Find first occurrence
        result = session.execute_statement('numbers.indexOf(20)')
        assert result.success
        assert result.value.value == 1
        
        # Find different element
        result = session.execute_statement('numbers.indexOf(30)')
        assert result.success
        assert result.value.value == 2
    
    def test_indexOf_not_found(self):
        """Test indexOf returns -1 when element not found."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [1, 2, 3]')
        assert result.success
        
        result = session.execute_statement('numbers.indexOf(99)')
        assert result.success
        assert result.value.value == -1
    
    def test_indexOf_string_list(self):
        """Test indexOf works with string lists."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<string> words = ["apple", "banana", "cherry"]')
        assert result.success
        
        result = session.execute_statement('words.indexOf("banana")')
        assert result.success
        assert result.value.value == 1
    
    def test_count_elements(self):
        """Test count method counts occurrences correctly."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [1, 2, 2, 3, 2, 4]')
        assert result.success
        
        # Count multiple occurrences
        result = session.execute_statement('numbers.count(2)')
        assert result.success
        assert result.value.value == 3
        
        # Count single occurrence
        result = session.execute_statement('numbers.count(1)')
        assert result.success
        assert result.value.value == 1
        
        # Count non-existent element
        result = session.execute_statement('numbers.count(99)')
        assert result.success
        assert result.value.value == 0
    
    def test_min_numbers(self):
        """Test min method finds minimum value."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [5, 1, 9, 3, 7]')
        assert result.success
        
        result = session.execute_statement('numbers.min()')
        assert result.success
        assert result.value.value == 1
    
    def test_max_numbers(self):
        """Test max method finds maximum value."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [5, 1, 9, 3, 7]')
        assert result.success
        
        result = session.execute_statement('numbers.max()')
        assert result.success
        assert result.value.value == 9
    
    def test_sum_numbers(self):
        """Test sum method calculates total."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        assert result.success
        
        result = session.execute_statement('numbers.sum()')
        assert result.success
        assert result.value.value == 15
    
    def test_sum_empty_list(self):
        """Test sum of empty list returns 0."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> empty = []')
        assert result.success
        
        result = session.execute_statement('empty.sum()')
        assert result.success
        assert result.value.value == 0
    
    def test_min_max_empty_list_fails(self):
        """Test min/max on empty list raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> empty = []')
        assert result.success
        
        result = session.execute_statement('empty.min()')
        assert not result.success
        assert "empty list" in str(result.error).lower()
        
        result = session.execute_statement('empty.max()')
        assert not result.success
        assert "empty list" in str(result.error).lower()
    
    def test_analysis_methods_require_numbers(self):
        """Test that min/max/sum require numeric elements."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<string> words = ["hello", "world"]')
        assert result.success
        
        result = session.execute_statement('words.min()')
        assert not result.success
        assert "numbers" in str(result.error).lower()
        
        result = session.execute_statement('words.max()')
        assert not result.success
        assert "numbers" in str(result.error).lower()
        
        result = session.execute_statement('words.sum()')
        assert not result.success
        assert "numbers" in str(result.error).lower()


class TestListTransformationMethods:
    """Test list transformation methods that modify graph structure."""
    
    def test_sort_numbers(self):
        """Test sorting numeric lists."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [5, 1, 9, 3, 7]')
        assert result.success
        
        result = session.execute_statement('list<num> sorted_numbers = numbers.sort()')
        assert result.success
        
        # Check that original list is unchanged (immutable behavior)
        result = session.execute_statement('numbers')
        assert result.success
        original_values = [elem.value for elem in result.value.elements]
        assert original_values == [5, 1, 9, 3, 7]
        
        # Check that sorted list has correct order
        result = session.execute_statement('sorted_numbers')
        assert result.success
        expected_order = [1, 3, 5, 7, 9]
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_order
    
    def test_sort_strings(self):
        """Test sorting string lists alphabetically."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<string> words = ["zebra", "apple", "banana"]')
        assert result.success
        
        result = session.execute_statement('list<string> sorted_words = words.sort()')
        assert result.success
        
        # Check original list is unchanged
        result = session.execute_statement('words')
        assert result.success
        original_values = [elem.value for elem in result.value.elements]
        assert original_values == ["zebra", "apple", "banana"]
        
        # Check alphabetical order in sorted list
        result = session.execute_statement('sorted_words')
        assert result.success
        expected_order = ["apple", "banana", "zebra"]
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_order
    
    def test_sort_booleans(self):
        """Test sorting boolean lists (false < true)."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<bool> flags = [true, false, true, false]')
        assert result.success
        
        result = session.execute_statement('list<bool> sorted_flags = flags.sort()')
        assert result.success
        
        # Check original list is unchanged
        result = session.execute_statement('flags')
        assert result.success
        original_values = [elem.value for elem in result.value.elements]
        assert original_values == [True, False, True, False]
        
        # Check boolean order (false < true) in sorted list
        result = session.execute_statement('sorted_flags')
        assert result.success
        expected_order = [False, False, True, True]
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_order
    
    def test_sort_empty_list(self):
        """Test sorting empty list."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> empty = []')
        assert result.success
        
        result = session.execute_statement('list<num> sorted_empty = empty.sort()')
        assert result.success
        
        # Check that both lists are empty
        result = session.execute_statement('empty')
        assert result.success
        assert len(result.value.elements) == 0
        
        result = session.execute_statement('sorted_empty')
        assert result.success
        assert len(result.value.elements) == 0
    
    def test_sort_requires_same_type(self):
        """Test that sort requires all elements to be same type."""
        # Note: This test would require mixed-type lists, which may not be 
        # supported in current constrained list system. Skip for now.
        pass


class TestListCombinationOperations:
    """Test list combination operations (union, intersection, difference)."""
    
    def test_list_union_addition(self):
        """Test list union using + operator."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> list1 = [1, 2, 3]')
        assert result.success
        
        result = session.execute_statement('list<num> list2 = [4, 5, 6]')
        assert result.success
        
        result = session.execute_statement('list1 + list2')
        assert result.success
        
        expected_elements = [1, 2, 3, 4, 5, 6]
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_elements
    
    def test_list_union_preserves_duplicates(self):
        """Test that union preserves all elements including duplicates."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> list1 = [1, 2, 2]')
        assert result.success
        
        result = session.execute_statement('list<num> list2 = [2, 3, 3]')
        assert result.success
        
        result = session.execute_statement('list1 + list2')
        assert result.success
        
        expected_elements = [1, 2, 2, 2, 3, 3]
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_elements
    
    def test_list_difference_subtraction(self):
        """Test list difference using - operator."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> list1 = [1, 2, 3, 4, 5]')
        assert result.success
        
        result = session.execute_statement('list<num> list2 = [3, 4, 6]')
        assert result.success
        
        result = session.execute_statement('list1 - list2')
        assert result.success
        
        expected_elements = [1, 2, 5]  # Elements in list1 but not in list2
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_elements
    
    def test_list_intersection_ampersand(self):
        """Test list intersection using & operator."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> list1 = [1, 2, 3, 4, 5]')
        assert result.success
        
        result = session.execute_statement('list<num> list2 = [3, 4, 5, 6, 7]')
        assert result.success
        
        result = session.execute_statement('list1 & list2')
        assert result.success
        
        expected_elements = [3, 4, 5]  # Elements in both lists
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_elements
    
    def test_intersection_removes_duplicates(self):
        """Test that intersection removes duplicate elements."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> list1 = [1, 2, 2, 3, 3, 3]')
        assert result.success
        
        result = session.execute_statement('list<num> list2 = [2, 3, 4]')
        assert result.success
        
        result = session.execute_statement('list1 & list2')
        assert result.success
        
        expected_elements = [2, 3]  # Duplicates removed in intersection
        actual_values = [elem.value for elem in result.value.elements]
        assert actual_values == expected_elements
    
    def test_string_list_combinations(self):
        """Test list operations work with string lists."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<string> words1 = ["apple", "banana"]')
        assert result.success
        
        result = session.execute_statement('list<string> words2 = ["banana", "cherry"]')
        assert result.success
        
        # Union
        result = session.execute_statement('words1 + words2')
        assert result.success
        expected = ["apple", "banana", "banana", "cherry"]
        actual = [elem.value for elem in result.value.elements]
        assert actual == expected
        
        # Intersection
        result = session.execute_statement('words1 & words2')
        assert result.success
        expected = ["banana"]
        actual = [elem.value for elem in result.value.elements]
        assert actual == expected
        
        # Difference
        result = session.execute_statement('words1 - words2')
        assert result.success
        expected = ["apple"]
        actual = [elem.value for elem in result.value.elements]
        assert actual == expected


class TestListMethodValidation:
    """Test error handling and argument validation for list methods."""
    
    def test_analysis_methods_argument_validation(self):
        """Test that analysis methods validate argument counts."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [1, 2, 3]')
        assert result.success
        
        # indexOf requires exactly 1 argument
        result = session.execute_statement('numbers.indexOf()')
        assert not result.success
        assert "takes 1 argument" in str(result.error).lower()
        
        result = session.execute_statement('numbers.indexOf(1, 2)')
        assert not result.success
        assert "takes 1 argument" in str(result.error).lower()
        
        # count requires exactly 1 argument  
        result = session.execute_statement('numbers.count()')
        assert not result.success
        assert "takes 1 argument" in str(result.error).lower()
        
        # min, max, sum take no arguments
        result = session.execute_statement('numbers.min(1)')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
        
        result = session.execute_statement('numbers.max("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
        
        result = session.execute_statement('numbers.sum(true)')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
    
    def test_sort_argument_validation(self):
        """Test that sort validates arguments."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [3, 1, 2]')
        assert result.success
        
        result = session.execute_statement('numbers.sort("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()


class TestMethodDiscoverability:
    """Test that new list methods appear in reflection system."""
    
    def test_new_methods_in_methods_list(self):
        """Test that new methods appear in methods() reflection."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [1, 2, 3]')
        assert result.success
        
        result = session.execute_statement('numbers.methods()')
        assert result.success
        
        methods = [elem.value for elem in result.value.elements]
        
        # Check that all new analysis methods are present
        assert "indexOf" in methods
        assert "count" in methods
        assert "min" in methods
        assert "max" in methods
        assert "sum" in methods
        assert "sort" in methods
        
        # Check that universal methods are still present
        assert "type" in methods
        assert "size" in methods
        assert "methods" in methods
    
    def test_can_method_recognizes_new_methods(self):
        """Test that can() method recognizes new list methods."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<num> numbers = [1, 2, 3]')
        assert result.success
        
        # Test new analysis methods
        result = session.execute_statement('numbers.can("indexOf")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('numbers.can("count")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('numbers.can("min")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('numbers.can("sort")')
        assert result.success
        assert result.value.value == True
        
        # Test invalid method
        result = session.execute_statement('numbers.can("nonexistent")')
        assert result.success
        assert result.value.value == False


if __name__ == "__main__":
    pytest.main([__file__, "-v"])