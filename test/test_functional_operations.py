"""Tests for functional programming operations (map, filter, each)."""

import pytest
from glang.execution.pipeline import ExecutionSession
from glang.execution.values import ListValue, NumberValue, StringValue, BooleanValue


class TestListMapMethod:
    """Test list.map() method with built-in transformations."""
    
    def test_map_double(self):
        """Test mapping double transformation over numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        
        result = session.execute_statement('numbers.map("double")')
        assert result.success
        assert isinstance(result.value, ListValue)
        assert len(result.value.elements) == 5
        assert [elem.value for elem in result.value.elements] == [2, 4, 6, 8, 10]
    
    def test_map_square(self):
        """Test mapping square transformation over numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [2, 3, 4]')
        
        result = session.execute_statement('numbers.map("square")')
        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [4, 9, 16]
    
    def test_map_negate(self):
        """Test mapping negate transformation over numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, -2, 3]')
        
        result = session.execute_statement('numbers.map("negate")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [-1, 2, -3]
    
    def test_map_upper_strings(self):
        """Test mapping upper transformation over strings."""
        session = ExecutionSession()
        session.execute_statement('list<string> names = ["alice", "bob", "charlie"]')
        
        result = session.execute_statement('names.map("upper")')
        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == ["ALICE", "BOB", "CHARLIE"]
    
    def test_map_lower_strings(self):
        """Test mapping lower transformation over strings."""
        session = ExecutionSession()
        session.execute_statement('list<string> names = ["ALICE", "Bob", "CHARLIE"]')
        
        result = session.execute_statement('names.map("lower")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == ["alice", "bob", "charlie"]
    
    def test_map_to_string(self):
        """Test mapping to_string transformation."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.map("to_string")')
        assert result.success
        assert result.value.constraint == "string"  # Type should change
        assert [elem.value for elem in result.value.elements] == ["1", "2", "3"]
    
    def test_map_to_num(self):
        """Test mapping to_num transformation."""
        session = ExecutionSession()
        session.execute_statement('list<string> numbers = ["1", "2", "3"]')
        
        result = session.execute_statement('numbers.map("to_num")')
        assert result.success
        assert result.value.constraint == "num"  # Type should change
        assert [elem.value for elem in result.value.elements] == [1, 2, 3]
    
    def test_map_chaining(self):
        """Test chaining map operations."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.map("double").map("square")')
        assert result.success
        # (1*2)^2=4, (2*2)^2=16, (3*2)^2=36
        assert [elem.value for elem in result.value.elements] == [4, 16, 36]
    
    def test_map_invalid_transformation(self):
        """Test map with invalid transformation name."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.map("invalid")')
        assert not result.success
        assert "Unknown transformation" in str(result.error)
    
    def test_map_incompatible_transformation(self):
        """Test map with incompatible transformation."""
        session = ExecutionSession()
        session.execute_statement('list<string> names = ["alice", "bob"]')
        
        result = session.execute_statement('names.map("double")')
        assert not result.success
        assert "Cannot double string" in str(result.error)


class TestListFilterMethod:
    """Test list.filter() method with built-in predicates."""
    
    def test_filter_positive(self):
        """Test filtering positive numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [-2, -1, 0, 1, 2, 3]')
        
        result = session.execute_statement('numbers.filter("positive")')
        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [1, 2, 3]
    
    def test_filter_negative(self):
        """Test filtering negative numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [-2, -1, 0, 1, 2, 3]')
        
        result = session.execute_statement('numbers.filter("negative")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [-2, -1]
    
    def test_filter_even(self):
        """Test filtering even numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5, 6]')
        
        result = session.execute_statement('numbers.filter("even")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [2, 4, 6]
    
    def test_filter_odd(self):
        """Test filtering odd numbers."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5, 6]')
        
        result = session.execute_statement('numbers.filter("odd")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [1, 3, 5]
    
    def test_filter_empty_strings(self):
        """Test filtering empty strings."""
        session = ExecutionSession()
        session.execute_statement('list<string> strings = ["hello", "", "world", "", "test"]')
        
        result = session.execute_statement('strings.filter("empty")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == ["", ""]
    
    def test_filter_non_empty_strings(self):
        """Test filtering non-empty strings."""
        session = ExecutionSession()
        session.execute_statement('list<string> strings = ["hello", "", "world", "", "test"]')
        
        result = session.execute_statement('strings.filter("non_empty")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == ["hello", "world", "test"]
    
    def test_filter_truthy(self):
        """Test filtering truthy values."""
        session = ExecutionSession()
        session.execute_statement('list values = [0, 1, "", "hello", true, false]')
        
        result = session.execute_statement('values.filter("truthy")')
        assert result.success
        # 1, "hello", and true are truthy
        assert len(result.value.elements) == 3
    
    def test_filter_chaining(self):
        """Test chaining filter operations."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [-3, -2, -1, 0, 1, 2, 3, 4, 5, 6]')
        
        result = session.execute_statement('numbers.filter("positive").filter("even")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [2, 4, 6]
    
    def test_select_alias(self):
        """Test select as alias for filter."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        
        result = session.execute_statement('numbers.select("even")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [2, 4]
    
    def test_reject_method(self):
        """Test reject (opposite of filter)."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        
        result = session.execute_statement('numbers.reject("even")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [1, 3, 5]
    
    def test_filter_invalid_predicate(self):
        """Test filter with invalid predicate name."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.filter("invalid")')
        assert not result.success
        assert "Unknown predicate" in str(result.error)


class TestListEachMethod:
    """Test list.each() method."""
    
    def test_each_print(self, capsys):
        """Test each with print action."""
        session = ExecutionSession()
        session.execute_statement('list<string> names = ["alice", "bob", "charlie"]')
        
        result = session.execute_statement('names.each("print")')
        assert result.success
        
        # Check printed output
        captured = capsys.readouterr()
        assert "alice\nbob\ncharlie\n" in captured.out
    
    def test_each_returns_original(self):
        """Test that each returns the original list for chaining."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.each("print")')
        assert result.success
        assert isinstance(result.value, ListValue)
        assert [elem.value for elem in result.value.elements] == [1, 2, 3]
    
    def test_each_chaining(self, capsys):
        """Test chaining after each."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.each("print").map("double")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [2, 4, 6]
        
        # Check that each still printed
        captured = capsys.readouterr()
        assert "1\n2\n3\n" in captured.out
    
    def test_each_invalid_action(self):
        """Test each with invalid action name."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.each("invalid")')
        assert not result.success
        assert "Unknown action" in str(result.error)


class TestCombinedFunctionalOperations:
    """Test combining map, filter, and each operations."""
    
    def test_filter_then_map(self):
        """Test filtering then mapping."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5, 6]')
        
        # Get even numbers then double them
        result = session.execute_statement('numbers.filter("even").map("double")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [4, 8, 12]
    
    def test_map_then_filter(self):
        """Test mapping then filtering."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3, 4, 5]')
        
        # Double all numbers then keep only those > 5
        result = session.execute_statement('numbers.map("double").filter("positive")')
        assert result.success
        # All doubled numbers are positive, so all should remain
        assert [elem.value for elem in result.value.elements] == [2, 4, 6, 8, 10]
    
    def test_complex_chain(self):
        """Test complex chaining of operations."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]')
        
        # Filter positive, double them, then keep only even results
        result = session.execute_statement('numbers.filter("positive").map("double").filter("even")')
        assert result.success
        # positive: [1,2,3,4,5] -> doubled: [2,4,6,8,10] -> all are even
        assert [elem.value for elem in result.value.elements] == [2, 4, 6, 8, 10]
    
    def test_type_transformation_chain(self):
        """Test chaining with type transformations."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        # Convert to strings, uppercase (which does nothing for digit strings), then back to numbers
        result = session.execute_statement('numbers.map("to_string").map("to_num")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [1, 2, 3]
        assert result.value.constraint == "num"


class TestTransformationAliases:
    """Test transformation and predicate aliases."""
    
    def test_increment_alias(self):
        """Test inc as alias for increment."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.map("inc")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [2, 3, 4]
    
    def test_decrement_alias(self):
        """Test dec as alias for decrement."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2, 3]')
        
        result = session.execute_statement('numbers.map("dec")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == [0, 1, 2]
    
    def test_string_case_aliases(self):
        """Test up/down as aliases for upper/lower."""
        session = ExecutionSession()
        session.execute_statement('list<string> names = ["alice", "BOB"]')
        
        result = session.execute_statement('names.map("up")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == ["ALICE", "BOB"]
        
        result = session.execute_statement('names.map("down")')
        assert result.success
        assert [elem.value for elem in result.value.elements] == ["alice", "bob"]
    
    def test_type_conversion_aliases(self):
        """Test str/num/bool as aliases."""
        session = ExecutionSession()
        session.execute_statement('list<num> numbers = [1, 2]')
        
        result = session.execute_statement('numbers.map("str")')
        assert result.success
        assert all(isinstance(elem, StringValue) for elem in result.value.elements)
        
        session.execute_statement('list<string> strings = ["1", "2"]')
        result = session.execute_statement('strings.map("num")')
        assert result.success
        assert all(isinstance(elem, NumberValue) for elem in result.value.elements)