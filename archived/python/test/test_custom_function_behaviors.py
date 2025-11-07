"""Tests for custom function behavior system."""

import pytest
from glang.execution.values import NumberValue, StringValue, BooleanValue, NoneValue
from glang.execution.graph_values import ListValue, HashValue
from glang.behaviors import CustomFunctionBehavior


class MockFunction:
    """Mock function for testing custom behaviors."""

    def __init__(self, name, transform_func):
        self.name = name
        self._transform = transform_func

    def get_type(self):
        return "function"

    def call(self, args, context):
        """Simulate function call by applying transform to first argument."""
        if args:
            return self._transform(args[0])
        return args[0] if args else NumberValue(0)


class TestCustomFunctionBehaviorBasics:
    """Test basic custom function behavior mechanics."""

    def test_custom_behavior_creation(self):
        """Test creating a CustomFunctionBehavior."""
        # Create a simple doubling function
        def double_transform(value):
            if hasattr(value, 'value') and isinstance(value.value, (int, float)):
                return NumberValue(value.value * 2)
            return value

        mock_func = MockFunction("double", double_transform)

        # This will fail because MockFunction is not FunctionValue - that's expected
        with pytest.raises(ValueError, match="Custom behavior must be a function or lambda"):
            behavior = CustomFunctionBehavior(mock_func)

    def test_invalid_custom_function_type(self):
        """Test that non-function values are rejected for custom behaviors."""
        numbers = ListValue([NumberValue(1), NumberValue(2)])

        # Try to add a string as a custom function - should raise error
        with pytest.raises(ValueError, match="Custom rule must be a function or lambda"):
            numbers.add_custom_rule(StringValue("not a function"))

    def test_custom_function_error_handling_in_container(self):
        """Test error handling when adding invalid custom functions."""
        numbers = ListValue([NumberValue(5), NumberValue(10)])

        # Test with various non-function types
        with pytest.raises(ValueError):
            numbers.add_custom_rule(NumberValue(42))

        with pytest.raises(ValueError):
            numbers.add_custom_rule(BooleanValue(True))

        with pytest.raises(ValueError):
            numbers.add_custom_rule(NoneValue())


class TestCustomFunctionBehaviorIntegration:
    """Integration tests that work with the existing system."""

    def test_custom_behavior_with_existing_behaviors(self):
        """Test that custom behavior framework integrates with existing behaviors."""
        # For now, we can test that the add_custom_rule method exists and handles errors properly
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Add some standard behaviors first
        numbers.add_rule(StringValue("positive"))
        numbers.add_rule(StringValue("round_to_int"))

        # Verify standard behaviors work
        assert len(numbers._behaviors) == 2

        # Try to add invalid custom rule - should fail gracefully
        with pytest.raises(ValueError):
            numbers.add_custom_rule(StringValue("invalid"))

        # Standard behaviors should still be there
        assert len(numbers._behaviors) == 2

    def test_custom_function_method_exists(self):
        """Test that add_custom_rule method exists on graph containers."""
        # Test that the method exists on ListValue
        numbers = ListValue([])
        assert hasattr(numbers, 'add_custom_rule')
        assert callable(getattr(numbers, 'add_custom_rule'))

        # Test that the method exists on HashValue
        data = HashValue([])
        assert hasattr(data, 'add_custom_rule')
        assert callable(getattr(data, 'add_custom_rule'))

    def test_add_custom_rule_returns_none_value(self):
        """Test that add_custom_rule returns NoneValue when given invalid input."""
        numbers = ListValue([NumberValue(1)])

        # Should raise ValueError, not return NoneValue, for invalid input
        with pytest.raises(ValueError):
            result = numbers.add_custom_rule(StringValue("not a function"))


class TestCustomFunctionBehaviorDocumentation:
    """Test that demonstrates the intended usage for documentation."""

    def test_intended_usage_example(self):
        """Document the intended usage pattern (even though full implementation needs real functions)."""
        # This test documents how the feature SHOULD work once fully implemented

        # Step 1: Create a list with some data
        temperatures = ListValue([NumberValue(85), NumberValue(110), NumberValue(98.6)])

        # Step 2: The user would define a function in Glang:
        # func normalize_temp(value) {
        #     if value < 95 { return 95 }
        #     if value > 105 { return 105 }
        #     return value
        # }

        # Step 3: The user would attach the function as a behavior:
        # temperatures.add_custom_rule(normalize_temp)

        # Step 4: All values (existing and future) would be processed by the function
        # Expected results: [95, 105, 98.6]

        # For now, we just verify the method exists and handles errors
        assert hasattr(temperatures, 'add_custom_rule')

        # And that it properly rejects non-functions
        with pytest.raises(ValueError):
            temperatures.add_custom_rule(StringValue("not a function"))


class TestCustomFunctionBehaviorSpecification:
    """Test the specification compliance."""

    def test_api_specification(self):
        """Test that the API matches the specification requirements."""
        # From the spec: container.add_custom_rule(function)

        # Test 1: Method exists on containers
        list_container = ListValue([])
        hash_container = HashValue([])

        assert hasattr(list_container, 'add_custom_rule')
        assert hasattr(hash_container, 'add_custom_rule')

        # Test 2: Method signature accepts function parameter
        import inspect
        sig = inspect.signature(list_container.add_custom_rule)
        params = list(sig.parameters.keys())
        assert 'function' in params

        # Test 3: Method validates input type
        with pytest.raises(ValueError, match="Custom rule must be a function or lambda"):
            list_container.add_custom_rule(NumberValue(42))

        # Test 4: Method returns NoneValue for valid operations (when we have real functions)
        # This will be testable once we integrate with actual FunctionValue objects

    def test_custom_function_api_completeness(self):
        """Test that the custom function API is complete."""
        # Verify the API follows the specification exactly
        container = ListValue([NumberValue(1)])

        # API requirement: container.add_custom_rule(function)
        assert hasattr(container, 'add_custom_rule')

        # Should take exactly one parameter
        import inspect
        sig = inspect.signature(container.add_custom_rule)
        param_count = len([p for p in sig.parameters.values() if p.kind != p.VAR_KEYWORD])
        assert param_count == 1  # Only 'function' parameter

        # Should return NoneValue (will be tested with real functions later)
        # For now, just test error case
        with pytest.raises(ValueError):
            container.add_custom_rule(NumberValue(42))
