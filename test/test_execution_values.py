"""Tests for the glang execution value system."""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '../src'))

from glang.execution.values import *
from glang.ast.nodes import SourcePosition


class TestGlangValues:
    """Test the basic GlangValue types."""
    
    def test_string_value(self):
        """Test StringValue functionality."""
        pos = SourcePosition(1, 5)
        value = StringValue("hello", pos)
        
        assert value.to_python() == "hello"
        assert value.get_type() == "string"
        assert value.to_display_string() == "hello"
        assert value.position == pos
        
        # Test equality
        other = StringValue("hello")
        assert value == other
        
        different = StringValue("world")
        assert value != different
    
    def test_number_value(self):
        """Test NumberValue functionality."""
        # Integer
        int_val = NumberValue(42)
        assert int_val.to_python() == 42
        assert int_val.get_type() == "num"
        assert int_val.to_display_string() == "42"
        
        # Float
        float_val = NumberValue(3.14)
        assert float_val.to_python() == 3.14
        assert float_val.get_type() == "num"
        assert float_val.to_display_string() == "3.14"
        
        # Equality
        assert NumberValue(42) == NumberValue(42)
        assert NumberValue(42) != NumberValue(43)
    
    def test_boolean_value(self):
        """Test BooleanValue functionality."""
        true_val = BooleanValue(True)
        false_val = BooleanValue(False)
        
        assert true_val.to_python() is True
        assert false_val.to_python() is False
        
        assert true_val.get_type() == "bool"
        assert false_val.get_type() == "bool"
        
        # Display uses lowercase
        assert true_val.to_display_string() == "true"
        assert false_val.to_display_string() == "false"
        
        # Equality
        assert BooleanValue(True) == BooleanValue(True)
        assert BooleanValue(True) != BooleanValue(False)
    
    def test_list_value_basic(self):
        """Test basic ListValue functionality."""
        elements = [
            NumberValue(1),
            NumberValue(2),
            NumberValue(3)
        ]
        
        list_val = ListValue(elements)
        
        assert list_val.to_python() == [1, 2, 3]
        assert list_val.get_type() == "list"
        assert list_val.to_display_string() == "[1, 2, 3]"
        assert len(list_val) == 3
        
        # Test element access
        assert list_val.get_element(0) == NumberValue(1)
        assert list_val.get_element(-1) == NumberValue(3)
        
        # Test bounds checking
        with pytest.raises(Exception):  # Should be RuntimeError
            list_val.get_element(5)
    
    def test_list_value_constraints(self):
        """Test ListValue type constraints."""
        # Create constrained list
        elements = [NumberValue(1), NumberValue(2)]
        constrained_list = ListValue(elements, "num")
        
        # Valid constraint validation
        assert constrained_list.validate_constraint(NumberValue(3)) is True
        assert constrained_list.validate_constraint(StringValue("hello")) is False
        
        # Test append with constraint
        constrained_list.append(NumberValue(4))
        assert len(constrained_list) == 3
        
        # Try to append wrong type (should raise error)
        with pytest.raises(Exception):  # Should be TypeConstraintError
            constrained_list.append(StringValue("invalid"))
    
    def test_list_value_modification(self):
        """Test ListValue modification operations."""
        elements = [StringValue("a"), StringValue("b")]
        list_val = ListValue(elements)
        
        # Test append
        list_val.append(StringValue("c"))
        assert len(list_val) == 3
        assert list_val.get_element(2) == StringValue("c")
        
        # Test set_element
        list_val.set_element(1, StringValue("modified"))
        assert list_val.get_element(1) == StringValue("modified")
        
        # Test bounds checking for set_element
        with pytest.raises(Exception):  # Should be RuntimeError
            list_val.set_element(10, StringValue("out_of_bounds"))


class TestValueConversion:
    """Test conversion between Python and GlangValue types."""
    
    def test_python_to_glang_value(self):
        """Test converting Python values to GlangValues."""
        # String
        glang_str = python_to_glang_value("hello")
        assert isinstance(glang_str, StringValue)
        assert glang_str.value == "hello"
        
        # Integer
        glang_int = python_to_glang_value(42)
        assert isinstance(glang_int, NumberValue)
        assert glang_int.value == 42
        
        # Float
        glang_float = python_to_glang_value(3.14)
        assert isinstance(glang_float, NumberValue)
        assert glang_float.value == 3.14
        
        # Boolean
        glang_bool = python_to_glang_value(True)
        assert isinstance(glang_bool, BooleanValue)
        assert glang_bool.value is True
        
        # List
        glang_list = python_to_glang_value([1, "hello", True])
        assert isinstance(glang_list, ListValue)
        assert len(glang_list) == 3
        assert isinstance(glang_list.elements[0], NumberValue)
        assert isinstance(glang_list.elements[1], StringValue)
        assert isinstance(glang_list.elements[2], BooleanValue)
        
        # Fallback (unknown type)
        class CustomObject:
            def __str__(self):
                return "custom"
        
        custom_val = python_to_glang_value(CustomObject())
        assert isinstance(custom_val, StringValue)
        assert custom_val.value == "custom"
    
    def test_glang_value_to_python(self):
        """Test converting GlangValues to Python values."""
        # String
        str_val = StringValue("hello")
        assert glang_value_to_python(str_val) == "hello"
        
        # Number
        num_val = NumberValue(42)
        assert glang_value_to_python(num_val) == 42
        
        # Boolean
        bool_val = BooleanValue(True)
        assert glang_value_to_python(bool_val) is True
        
        # List
        list_val = ListValue([
            NumberValue(1),
            StringValue("hello"),
            BooleanValue(False)
        ])
        python_list = glang_value_to_python(list_val)
        assert python_list == [1, "hello", False]
    
    def test_round_trip_conversion(self):
        """Test that values survive round-trip conversion."""
        original_values = [
            "hello",
            42,
            3.14,
            True,
            False,
            [1, 2, "three", True]
        ]
        
        for original in original_values:
            # Python -> Glang -> Python
            glang_val = python_to_glang_value(original)
            converted_back = glang_value_to_python(glang_val)
            assert converted_back == original


class TestValueStringRepresentation:
    """Test string representations of values."""
    
    def test_value_str_methods(self):
        """Test __str__ and __repr__ methods."""
        str_val = StringValue("hello")
        assert str(str_val) == "hello"
        assert repr(str_val) == "StringValue('hello')"
        
        num_val = NumberValue(42)
        assert str(num_val) == "42"
        assert repr(num_val) == "NumberValue(42)"
        
        bool_val = BooleanValue(True)
        assert str(bool_val) == "true"  # lowercase for display
        assert repr(bool_val) == "BooleanValue(True)"
        
        list_val = ListValue([NumberValue(1), StringValue("hello")])
        assert str(list_val) == "[1, hello]"
        assert "ListValue" in repr(list_val)
    
    def test_nested_list_display(self):
        """Test display of nested lists."""
        inner_list = ListValue([NumberValue(1), NumberValue(2)])
        outer_list = ListValue([inner_list, StringValue("hello")])
        
        display = outer_list.to_display_string()
        assert "[1, 2]" in display
        assert "hello" in display


if __name__ == '__main__':
    pytest.main([__file__])