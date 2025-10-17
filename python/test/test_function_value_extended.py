"""Additional tests for function value types."""

import pytest
from src.glang.execution.function_value import BuiltinFunctionValue
from src.glang.execution.values import StringValue, NumberValue, BooleanValue
from src.glang.execution.errors import RuntimeError
from src.glang.ast.nodes import SourcePosition


class TestBuiltinFunctionValue:
    """Test BuiltinFunctionValue class."""
    
    def test_basic_construction(self):
        """Test basic construction of builtin function."""
        def dummy_func(position=None):
            return StringValue("result", position)
        
        func_value = BuiltinFunctionValue("test_func", dummy_func)
        
        assert func_value.name == "test_func"
        assert func_value.func == dummy_func
        assert func_value.position is None
    
    def test_construction_with_position(self):
        """Test construction with position information."""
        position = SourcePosition(1, 5)
        def dummy_func(position=None):
            return StringValue("result", position)
        
        func_value = BuiltinFunctionValue("positioned_func", dummy_func, position)
        
        assert func_value.position == position
        assert func_value.name == "positioned_func"
    
    def test_to_python(self):
        """Test to_python method returns underlying function."""
        def original_func(position=None):
            return NumberValue(42, position)
        
        func_value = BuiltinFunctionValue("wrapper", original_func)
        result = func_value.to_python()
        
        assert result is original_func
    
    def test_get_type(self):
        """Test get_type returns correct type string."""
        def dummy_func(position=None):
            return BooleanValue(True, position)
        
        func_value = BuiltinFunctionValue("type_test", dummy_func)
        
        assert func_value.get_type() == "builtin_function"
    
    def test_to_display_string(self):
        """Test display string representation."""
        def dummy_func(position=None):
            return StringValue("test", position)
        
        func_value = BuiltinFunctionValue("display_test", dummy_func)
        display = func_value.to_display_string()
        
        assert display == "<builtin function display_test>"
    
    def test_universal_size(self):
        """Test universal_size returns 1."""
        def dummy_func(position=None):
            return StringValue("test", position)
        
        func_value = BuiltinFunctionValue("size_test", dummy_func)
        size = func_value.universal_size()
        
        assert isinstance(size, NumberValue)
        assert size.glang_number.to_python_int() == 1
    
    def test_universal_inspect(self):
        """Test universal_inspect returns function info."""
        position = SourcePosition(2, 8)
        def dummy_func(position=None):
            return StringValue("test", position)
        
        func_value = BuiltinFunctionValue("inspect_test", dummy_func, position)
        inspect_result = func_value.universal_inspect()
        
        assert isinstance(inspect_result, StringValue)
        assert inspect_result.value == "<builtin function inspect_test>"
        assert inspect_result.position == position


class TestBuiltinFunctionValueCalls:
    """Test calling builtin function values."""
    
    def test_call_no_arguments(self):
        """Test calling function with no arguments."""
        def no_args_func(position=None):
            return StringValue("no args called", position)
        
        func_value = BuiltinFunctionValue("no_args", no_args_func)
        result = func_value.call([])
        
        assert isinstance(result, StringValue)
        assert result.value == "no args called"
    
    def test_call_single_argument(self):
        """Test calling function with single argument."""
        def single_arg_func(arg, position=None):
            return StringValue(f"arg: {arg.value}", position)
        
        func_value = BuiltinFunctionValue("single_arg", single_arg_func)
        arg = StringValue("test_input")
        result = func_value.call([arg])
        
        assert isinstance(result, StringValue)
        assert result.value == "arg: test_input"
    
    def test_call_two_arguments(self):
        """Test calling function with two arguments."""
        def two_args_func(arg1, arg2, position=None):
            return StringValue(f"{arg1.value} + {arg2.value}", position)
        
        func_value = BuiltinFunctionValue("two_args", two_args_func)
        arg1 = StringValue("first")
        arg2 = StringValue("second")
        result = func_value.call([arg1, arg2])
        
        assert isinstance(result, StringValue)
        assert result.value == "first + second"
    
    def test_call_three_arguments(self):
        """Test calling function with three arguments."""
        def three_args_func(arg1, arg2, arg3, position=None):
            total = arg1.glang_number.to_python_int() + arg2.glang_number.to_python_int() + arg3.glang_number.to_python_int()
            return NumberValue(total, position)
        
        func_value = BuiltinFunctionValue("three_args", three_args_func)
        args = [NumberValue(1), NumberValue(2), NumberValue(3)]
        result = func_value.call(args)
        
        assert isinstance(result, NumberValue)
        assert result.glang_number.to_python_int() == 6
    
    def test_call_many_arguments(self):
        """Test calling function with more than three arguments."""
        def many_args_func(*args, position=None):
            count = len(args) - 1 if args[-1] is None else len(args)  # Handle position parameter
            return NumberValue(count, position)
        
        func_value = BuiltinFunctionValue("many_args", many_args_func)
        args = [NumberValue(i) for i in range(5)]  # 5 arguments
        result = func_value.call(args)
        
        assert isinstance(result, NumberValue)
        assert result.glang_number.to_python_int() == 5
    
    def test_call_with_position(self):
        """Test calling function with position information."""
        def position_aware_func(arg, position=None):
            return StringValue("position aware", position)
        
        call_position = SourcePosition(3, 12)
        func_value = BuiltinFunctionValue("pos_aware", position_aware_func)
        arg = StringValue("test")
        result = func_value.call([arg], call_position)
        
        assert isinstance(result, StringValue)
        assert result.position == call_position
    
    def test_call_type_error_handling(self):
        """Test handling of TypeError during function call."""
        def problematic_func(position=None):
            # This will raise TypeError because it expects an argument but gets none
            raise TypeError("Expected argument")
        
        func_value = BuiltinFunctionValue("problematic", problematic_func)
        
        with pytest.raises(RuntimeError) as exc_info:
            func_value.call([])
        
        assert "Error calling problematic" in str(exc_info.value)
        assert "Expected argument" in str(exc_info.value)
    
    def test_call_type_error_with_position(self):
        """Test TypeError handling includes position information."""
        def failing_func(arg, position=None):
            raise TypeError("Function failed")
        
        error_position = SourcePosition(5, 20)
        func_value = BuiltinFunctionValue("failing", failing_func)
        
        with pytest.raises(RuntimeError) as exc_info:
            func_value.call([StringValue("test")], error_position)
        
        runtime_error = exc_info.value
        assert runtime_error.position == error_position
        assert "Error calling failing" in str(runtime_error)


class TestBuiltinFunctionValueEdgeCases:
    """Test edge cases for builtin function values."""
    
    def test_function_with_return_value_types(self):
        """Test function that can return different types."""
        def flexible_func(type_arg, position=None):
            if type_arg.value == "string":
                return StringValue("string result", position)
            elif type_arg.value == "number":
                return NumberValue(42, position)
            else:
                return BooleanValue(True, position)
        
        func_value = BuiltinFunctionValue("flexible", flexible_func)
        
        # Test string return
        string_result = func_value.call([StringValue("string")])
        assert isinstance(string_result, StringValue)
        assert string_result.value == "string result"
        
        # Test number return
        number_result = func_value.call([StringValue("number")])
        assert isinstance(number_result, NumberValue)
        assert number_result.glang_number.to_python_int() == 42
        
        # Test boolean return
        bool_result = func_value.call([StringValue("other")])
        assert isinstance(bool_result, BooleanValue)
        assert bool_result.value is True
    
    def test_function_name_edge_cases(self):
        """Test function names with special characters."""
        def dummy_func(position=None):
            return StringValue("test", position)
        
        # Test with underscores
        func_underscore = BuiltinFunctionValue("test_func_name", dummy_func)
        assert "test_func_name" in func_underscore.to_display_string()
        
        # Test with numbers
        func_numbers = BuiltinFunctionValue("func123", dummy_func)
        assert "func123" in func_numbers.to_display_string()
        
        # Test empty name (edge case)
        func_empty = BuiltinFunctionValue("", dummy_func)
        assert "<builtin function >" in func_empty.to_display_string()
    
    def test_position_propagation(self):
        """Test that position is properly propagated through calls."""
        def position_checking_func(position=None):
            # Return a value with the passed position
            return StringValue("position_test", position)
        
        original_position = SourcePosition(7, 25)
        call_position = SourcePosition(10, 30)
        
        func_value = BuiltinFunctionValue("pos_check", position_checking_func, original_position)
        result = func_value.call([], call_position)
        
        # The result should have the call position, not the function definition position
        assert result.position == call_position
        
        # But the function itself should retain its original position
        assert func_value.position == original_position
    
    def test_inspect_with_position(self):
        """Test inspect method preserves position."""
        position = SourcePosition(4, 15)
        def dummy_func(position=None):
            return StringValue("test", position)
        
        func_value = BuiltinFunctionValue("inspect_pos_test", dummy_func, position)
        inspect_result = func_value.universal_inspect()
        
        assert inspect_result.position == position
    
    def test_size_with_position(self):
        """Test size method preserves position."""
        position = SourcePosition(6, 18)
        def dummy_func(position=None):
            return StringValue("test", position)
        
        func_value = BuiltinFunctionValue("size_pos_test", dummy_func, position)
        size_result = func_value.universal_size()
        
        assert size_result.position == position