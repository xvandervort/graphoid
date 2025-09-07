"""
Tests for universal reflection methods.

This module tests the reflection capabilities that work across all data types,
including type(), methods(), can(), inspect(), and size() methods.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.values import StringValue, NumberValue, BooleanValue, ListValue
from glang.execution.pipeline import ExecutionSession
from glang.ast.nodes import SourcePosition


class TestTypeMethod:
    """Test the type() reflection method on all data types."""
    
    def test_string_type(self):
        """Test type() method on string values."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.type()')
        assert result.success
        assert result.value.value == "string"
    
    def test_number_type(self):
        """Test type() method on number values."""
        session = ExecutionSession()
        
        result = session.execute_statement('num value = 42')
        assert result.success
        
        result = session.execute_statement('value.type()')
        assert result.success
        assert result.value.value == "num"
    
    def test_boolean_type(self):
        """Test type() method on boolean values."""
        session = ExecutionSession()
        
        result = session.execute_statement('bool flag = true')
        assert result.success
        
        result = session.execute_statement('flag.type()')
        assert result.success
        assert result.value.value == "bool"
    
    def test_list_type(self):
        """Test type() method on list values."""
        session = ExecutionSession()
        
        result = session.execute_statement('list data = [1, 2, 3]')
        assert result.success
        
        result = session.execute_statement('data.type()')
        assert result.success
        assert result.value.value == "list"
    
    def test_type_with_arguments_fails(self):
        """Test that type() with arguments raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.type("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()


class TestMethodsMethod:
    """Test the methods() reflection method on all data types."""
    
    def test_string_methods(self):
        """Test methods() on string returns correct method list."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.methods()')
        assert result.success
        assert result.value.get_type() == "list"
        
        methods = [elem.value for elem in result.value.elements]
        # Check that universal methods are included
        assert "type" in methods
        assert "methods" in methods
        assert "can" in methods
        assert "inspect" in methods
        assert "size" in methods
        # Check that string-specific methods are included
        assert "length" in methods
        assert "contains" in methods
        assert "reverse" in methods
    
    def test_number_methods(self):
        """Test methods() on number returns correct method list."""
        session = ExecutionSession()
        
        result = session.execute_statement('num value = 42')
        assert result.success
        
        result = session.execute_statement('value.methods()')
        assert result.success
        
        methods = [elem.value for elem in result.value.elements]
        # Check universal methods
        assert "type" in methods
        assert "methods" in methods
        assert "can" in methods
        assert "inspect" in methods
        assert "size" in methods
        # Check number-specific methods
        assert "to" in methods
    
    def test_boolean_methods(self):
        """Test methods() on boolean returns correct method list."""
        session = ExecutionSession()
        
        result = session.execute_statement('bool flag = true')
        assert result.success
        
        result = session.execute_statement('flag.methods()')
        assert result.success
        
        methods = [elem.value for elem in result.value.elements]
        # Check universal methods
        assert "type" in methods
        assert "methods" in methods
        assert "can" in methods
        assert "inspect" in methods
        assert "size" in methods
        # Check boolean-specific methods
        assert "flip" in methods
        assert "toggle" in methods
        assert "numify" in methods
        assert "toNum" in methods
    
    def test_list_methods(self):
        """Test methods() on list returns correct method list."""
        session = ExecutionSession()
        
        result = session.execute_statement('list data = [1, 2, 3]')
        assert result.success
        
        result = session.execute_statement('data.methods()')
        assert result.success
        
        methods = [elem.value for elem in result.value.elements]
        # Check universal methods
        assert "type" in methods
        assert "methods" in methods
        assert "can" in methods
        assert "inspect" in methods
        assert "size" in methods
        # Check list-specific methods
        assert "append" in methods
        assert "prepend" in methods
        assert "insert" in methods
        assert "reverse" in methods


class TestCanMethod:
    """Test the can() reflection method for checking method availability."""
    
    def test_string_can_valid_methods(self):
        """Test can() returns true for valid string methods."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        # Test universal methods
        result = session.execute_statement('text.can("type")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.can("methods")')
        assert result.success
        assert result.value.value == True
        
        # Test string-specific methods
        result = session.execute_statement('text.can("length")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('text.can("reverse")')
        assert result.success
        assert result.value.value == True
    
    def test_string_can_invalid_methods(self):
        """Test can() returns false for invalid string methods."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.can("append")')
        assert result.success
        assert result.value.value == False
        
        result = session.execute_statement('text.can("nonexistent")')
        assert result.success
        assert result.value.value == False
    
    def test_number_can_methods(self):
        """Test can() on number values."""
        session = ExecutionSession()
        
        result = session.execute_statement('num value = 42')
        assert result.success
        
        # Valid number method
        result = session.execute_statement('value.can("to")')
        assert result.success
        assert result.value.value == True
        
        # Invalid for numbers
        result = session.execute_statement('value.can("append")')
        assert result.success
        assert result.value.value == False
    
    def test_list_can_methods(self):
        """Test can() on list values."""
        session = ExecutionSession()
        
        result = session.execute_statement('list data = [1, 2, 3]')
        assert result.success
        
        # Valid list method
        result = session.execute_statement('data.can("append")')
        assert result.success
        assert result.value.value == True
        
        # Invalid for lists (string method)
        result = session.execute_statement('data.can("length")')
        assert result.success
        assert result.value.value == False
    
    def test_can_with_non_string_argument(self):
        """Test that can() with non-string argument raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.can(42)')
        assert not result.success
        assert "argument must be string" in str(result.error).lower()
    
    def test_can_with_wrong_argument_count(self):
        """Test that can() with wrong argument count raises error."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.can()')
        assert not result.success
        assert "takes 1 argument" in str(result.error).lower()
        
        result = session.execute_statement('text.can("method", "extra")')
        assert not result.success
        assert "takes 1 argument" in str(result.error).lower()


class TestInspectMethod:
    """Test the inspect() reflection method for detailed representation."""
    
    def test_string_inspect(self):
        """Test inspect() on string values."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert '"hello"' in inspection
        assert "string" in inspection
        assert "5 chars" in inspection
    
    def test_string_inspect_empty(self):
        """Test inspect() on empty string."""
        session = ExecutionSession()
        
        result = session.execute_statement('string empty = ""')
        assert result.success
        
        result = session.execute_statement('empty.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert '""' in inspection
        assert "0 chars" in inspection
    
    def test_number_inspect(self):
        """Test inspect() on number values."""
        session = ExecutionSession()
        
        result = session.execute_statement('num value = 42.5')
        assert result.success
        
        result = session.execute_statement('value.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert "42.5" in inspection
        assert "(num)" in inspection
    
    def test_boolean_inspect(self):
        """Test inspect() on boolean values."""
        session = ExecutionSession()
        
        result = session.execute_statement('bool flag_true = true')
        assert result.success
        
        result = session.execute_statement('flag_true.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert "true" in inspection
        assert "(bool)" in inspection
        
        # Test false value
        result = session.execute_statement('bool flag_false = false')
        assert result.success
        
        result = session.execute_statement('flag_false.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert "false" in inspection
        assert "(bool)" in inspection
    
    def test_list_inspect(self):
        """Test inspect() on list values."""
        session = ExecutionSession()
        
        result = session.execute_statement('list data = [1, 2, 3]')
        assert result.success
        
        result = session.execute_statement('data.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert "list" in inspection
        assert "3 elements" in inspection
    
    def test_constrained_list_inspect(self):
        """Test inspect() on constrained list values."""
        session = ExecutionSession()
        
        result = session.execute_statement('list<string> names = ["Alice", "Bob"]')
        assert result.success
        
        result = session.execute_statement('names.inspect()')
        assert result.success
        
        inspection = result.value.value
        assert "list<string>" in inspection
        assert "2 elements" in inspection


class TestSizeMethod:
    """Test the size() reflection method for graph node counts."""
    
    def test_string_size(self):
        """Test size() on string values returns character count."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        result = session.execute_statement('text.size()')
        assert result.success
        assert result.value.value == 5
        
        # Test empty string
        result = session.execute_statement('string empty = ""')
        assert result.success
        
        result = session.execute_statement('empty.size()')
        assert result.success
        assert result.value.value == 0
    
    def test_string_size_unicode(self):
        """Test size() on string with Unicode characters."""
        session = ExecutionSession()
        
        result = session.execute_statement('string emoji = "🎉👍"')
        assert result.success
        
        result = session.execute_statement('emoji.size()')
        assert result.success
        assert result.value.value == 2  # Two Unicode characters
    
    def test_number_size(self):
        """Test size() on number values returns 1 (atomic)."""
        session = ExecutionSession()
        
        result = session.execute_statement('num value = 42.5')
        assert result.success
        
        result = session.execute_statement('value.size()')
        assert result.success
        assert result.value.value == 1  # Atomic value
    
    def test_boolean_size(self):
        """Test size() on boolean values returns 1 (atomic)."""
        session = ExecutionSession()
        
        result = session.execute_statement('bool flag = true')
        assert result.success
        
        result = session.execute_statement('flag.size()')
        assert result.success
        assert result.value.value == 1  # Atomic value
    
    def test_list_size(self):
        """Test size() on list values returns element count."""
        session = ExecutionSession()
        
        result = session.execute_statement('list data = [1, 2, 3, 4, 5]')
        assert result.success
        
        result = session.execute_statement('data.size()')
        assert result.success
        assert result.value.value == 5
        
        # Test empty list
        result = session.execute_statement('list empty = []')
        assert result.success
        
        result = session.execute_statement('empty.size()')
        assert result.success
        assert result.value.value == 0
    
    def test_nested_list_size(self):
        """Test size() on nested list counts top-level elements."""
        session = ExecutionSession()
        
        result = session.execute_statement('list nested = [[1, 2], [3, 4], [5]]')
        assert result.success
        
        result = session.execute_statement('nested.size()')
        assert result.success
        assert result.value.value == 3  # Three top-level elements (sublists)


class TestReflectionMethodEdgeCases:
    """Test edge cases and error conditions for reflection methods."""
    
    def test_reflection_methods_with_arguments(self):
        """Test that reflection methods that take no arguments fail with arguments."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        # type() should fail with arguments
        result = session.execute_statement('text.type("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
        
        # methods() should fail with arguments
        result = session.execute_statement('text.methods("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
        
        # inspect() should fail with arguments
        result = session.execute_statement('text.inspect("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
        
        # size() should fail with arguments
        result = session.execute_statement('text.size("invalid")')
        assert not result.success
        assert "takes no arguments" in str(result.error).lower()
    
    def test_reflection_on_method_results(self):
        """Test using reflection on the results of other methods."""
        session = ExecutionSession()
        
        result = session.execute_statement('string text = "hello"')
        assert result.success
        
        # Get type of the result of type() method
        result = session.execute_statement('text.type().type()')
        assert result.success
        assert result.value.value == "string"
        
        # Get size of the result of methods() 
        result = session.execute_statement('text.methods().size()')
        assert result.success
        # Should be the number of available methods
        assert isinstance(result.value.value, int)
        assert result.value.value > 0
    
    def test_cross_type_consistency(self):
        """Test that reflection methods are consistent across types."""
        session = ExecutionSession()
        
        # Create values of different types
        result = session.execute_statement('string s = "test"')
        assert result.success
        
        result = session.execute_statement('num n = 42')
        assert result.success
        
        result = session.execute_statement('bool b = true')
        assert result.success
        
        result = session.execute_statement('list l = [1, 2]')
        assert result.success
        
        # All should have type() method that returns their type name
        result = session.execute_statement('s.can("type")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('n.can("type")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('b.can("type")')
        assert result.success
        assert result.value.value == True
        
        result = session.execute_statement('l.can("type")')
        assert result.success
        assert result.value.value == True


if __name__ == "__main__":
    pytest.main([__file__])