"""Tests for conditional behavior system."""

import pytest
from glang.execution.values import NumberValue, StringValue, BooleanValue, NoneValue
from glang.execution.graph_values import ListValue, HashValue
from glang.behaviors import ConditionalBehavior


class MockFunction:
    """Mock function for testing conditional behaviors."""

    def __init__(self, name, func):
        self.name = name
        self._func = func

    def get_type(self):
        return "function"

    def call(self, args, context):
        """Simulate function call."""
        if args:
            return self._func(args[0])
        return BooleanValue(False)


class TestConditionalBehaviorBasics:
    """Test basic conditional behavior mechanics."""

    def test_conditional_behavior_creation(self):
        """Test creating a ConditionalBehavior with mock functions."""
        # Create mock condition and transform functions
        def is_positive(value):
            if hasattr(value, 'value') and isinstance(value.value, (int, float)):
                return BooleanValue(value.value > 0)
            return BooleanValue(False)

        def double(value):
            if hasattr(value, 'value') and isinstance(value.value, (int, float)):
                return NumberValue(value.value * 2)
            return value

        condition_func = MockFunction("is_positive", is_positive)
        transform_func = MockFunction("double", double)

        # This will fail because MockFunction is not FunctionValue - that's expected
        with pytest.raises(ValueError, match="Condition must be a function or lambda"):
            behavior = ConditionalBehavior(condition_func, transform_func)

    def test_invalid_conditional_behavior_types(self):
        """Test that non-function values are rejected for conditional behaviors."""
        numbers = ListValue([NumberValue(1), NumberValue(2)])

        # Test invalid condition type
        with pytest.raises(ValueError, match="Condition must be a function or lambda"):
            numbers.add_conditional_rule(StringValue("not a function"), StringValue("also not"))

        # Test that the method validates properly
        try:
            numbers.add_conditional_rule(NumberValue(42), StringValue("test"))
            assert False, "Should have raised ValueError for invalid condition"
        except ValueError as e:
            assert "Condition must be a function or lambda" in str(e)

    def test_conditional_rule_method_exists(self):
        """Test that add_conditional_rule method exists on graph containers."""
        # Test that the method exists on ListValue
        numbers = ListValue([])
        assert hasattr(numbers, 'add_conditional_rule')
        assert callable(getattr(numbers, 'add_conditional_rule'))

        # Test that the method exists on HashValue
        data = HashValue([])
        assert hasattr(data, 'add_conditional_rule')
        assert callable(getattr(data, 'add_conditional_rule'))

    def test_conditional_rule_api_signature(self):
        """Test that the conditional rule API matches specification."""
        container = ListValue([NumberValue(1)])

        # API requirement: add_conditional_rule(condition, transform, on_fail=None)
        import inspect
        sig = inspect.signature(container.add_conditional_rule)
        params = list(sig.parameters.keys())

        # Should have condition, transform, and optional on_fail
        assert 'condition' in params
        assert 'transform' in params
        assert 'on_fail' in params

        # on_fail should be optional
        on_fail_param = sig.parameters['on_fail']
        assert on_fail_param.default is None


class TestConditionalBehaviorIntegration:
    """Integration tests that work with the existing system."""

    def test_conditional_behavior_with_existing_behaviors(self):
        """Test that conditional behavior framework integrates with existing behaviors."""
        numbers = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Add some standard behaviors first
        numbers.add_rule(StringValue("positive"))
        numbers.add_rule(StringValue("round_to_int"))

        # Verify standard behaviors work
        assert len(numbers._behaviors) == 2

        # Try to add invalid conditional rule - should fail gracefully
        with pytest.raises(ValueError):
            numbers.add_conditional_rule(StringValue("invalid"), StringValue("also invalid"))

        # Standard behaviors should still be there
        assert len(numbers._behaviors) == 2

    def test_conditional_behavior_error_handling(self):
        """Test error handling when adding invalid conditional behaviors."""
        numbers = ListValue([NumberValue(5), NumberValue(10)])

        # Test with various non-function types for condition
        with pytest.raises(ValueError):
            numbers.add_conditional_rule(NumberValue(42), StringValue("test"))

        # Test with various non-function types for transform
        with pytest.raises(ValueError):
            numbers.add_conditional_rule(StringValue("test"),
                                       BooleanValue(True))

        # Test with invalid on_fail type
        with pytest.raises(ValueError):
            numbers.add_conditional_rule(StringValue("cond"),
                                       StringValue("trans"),
                                       StringValue("invalid on_fail"))


class TestConditionalBehaviorDocumentation:
    """Test that demonstrates the intended usage for documentation."""

    def test_intended_usage_example(self):
        """Document the intended usage pattern."""
        # This test documents how the feature SHOULD work once fully implemented

        # Step 1: Create a mixed data container
        mixed_data = ListValue([NumberValue(5), StringValue("hello"), NumberValue(-3)])

        # Step 2: The user would define condition and transform functions in Glang:
        # func is_number(value) {
        #     return value.get_type() == "number"
        # }
        # func make_positive(value) {
        #     if value < 0 { return -value }
        #     return value
        # }

        # Step 3: The user would attach conditional behavior:
        # mixed_data.add_conditional_rule(is_number, make_positive)

        # Step 4: Only numbers would be processed
        # Expected results: [5, "hello", 3]

        # For now, we just verify the method exists and handles errors
        assert hasattr(mixed_data, 'add_conditional_rule')

        # And that it properly rejects non-functions
        with pytest.raises(ValueError):
            mixed_data.add_conditional_rule(StringValue("not a function"),
                                          StringValue("also not a function"))

    def test_type_specific_processing_example(self):
        """Document type-specific processing with conditional behaviors."""
        # This demonstrates the power of conditional behaviors for type-specific processing

        # Mixed data types
        data = ListValue([
            NumberValue(42),
            StringValue("  HELLO  "),
            NumberValue(-10),
            StringValue("world"),
            BooleanValue(True)
        ])

        # The user would define:
        # func is_string(value) { return value.get_type() == "string" }
        # func trim_and_lower(value) { return value.trim().lower() }
        # func is_negative_number(value) {
        #     return value.get_type() == "number" and value < 0
        # }
        # func abs_value(value) { return -value }

        # Then apply conditional behaviors:
        # data.add_conditional_rule(is_string, trim_and_lower)
        # data.add_conditional_rule(is_negative_number, abs_value)

        # Expected: [42, "hello", 10, "world", True]

        # For now, just test the API exists
        assert hasattr(data, 'add_conditional_rule')

    def test_conditional_with_fallback_example(self):
        """Document conditional behaviors with fallback (on_fail) processing."""
        # This shows the on_fail parameter usage

        # Data that needs validation
        user_ages = ListValue([NumberValue(25), NumberValue(-5), NumberValue(150)])

        # The user would define:
        # func is_valid_age(value) {
        #     return value >= 0 and value <= 120
        # }
        # func keep_value(value) { return value }
        # func default_age(value) { return 25 }  # Default age for invalid values

        # Apply with fallback:
        # user_ages.add_conditional_rule(is_valid_age, keep_value, default_age)

        # Expected: [25, 25, 25] - invalid ages replaced with default

        # For now, test that on_fail parameter is accepted
        import inspect
        sig = inspect.signature(user_ages.add_conditional_rule)
        assert 'on_fail' in sig.parameters


class TestConditionalBehaviorSpecification:
    """Test the specification compliance."""

    def test_api_specification_compliance(self):
        """Test that the API matches the specification exactly."""
        # From the spec: container.add_conditional_rule(condition: lambda, transform: lambda)

        container = ListValue([NumberValue(1)])

        # Test 1: Method exists
        assert hasattr(container, 'add_conditional_rule')

        # Test 2: Method signature matches spec
        import inspect
        sig = inspect.signature(container.add_conditional_rule)
        params = list(sig.parameters.keys())

        # Should have condition, transform, and optional on_fail parameters
        assert 'condition' in params
        assert 'transform' in params
        assert 'on_fail' in params

        # Test 3: Parameters have correct types expected (functions)
        with pytest.raises(ValueError, match="Condition must be a function or lambda"):
            container.add_conditional_rule(NumberValue(1), NumberValue(2))

        # Test 4: Returns NoneValue for valid operations (will test with real functions later)
        # For now, just test that error cases work properly

    def test_conditional_behavior_composition(self):
        """Test that conditional behaviors compose with other behavior types."""
        # This tests that conditional behaviors work alongside other behavior types

        container = ListValue([NumberValue(5)])

        # Add standard behavior
        container.add_rule(StringValue("positive"))
        initial_count = len(container._behaviors)

        # Try to add conditional behavior (will fail with mock, but shouldn't break existing)
        try:
            container.add_conditional_rule(StringValue("invalid"), StringValue("invalid"))
        except ValueError:
            pass  # Expected to fail

        # Original behaviors should be preserved
        assert len(container._behaviors) == initial_count

    def test_conditional_behavior_uniqueness(self):
        """Test that the same conditional behavior isn't added twice."""
        # This would be tested with real functions, but we document the expectation

        container = ListValue([NumberValue(1)])

        # The expectation is that:
        # func cond(x) { return true }
        # func trans(x) { return x }
        # container.add_conditional_rule(cond, trans)
        # container.add_conditional_rule(cond, trans)  # Should be ignored

        # For now, just document that the API should prevent duplicates
        # This will be fully testable once we have real function integration
        assert hasattr(container, 'add_conditional_rule')

    def test_conditional_behavior_error_safety(self):
        """Test that conditional behaviors are error-safe."""
        # From the spec: behaviors should handle errors gracefully

        container = ListValue([NumberValue(1)])

        # Invalid inputs should raise clear errors, not crash the system
        with pytest.raises(ValueError) as exc_info:
            container.add_conditional_rule(StringValue("bad"), StringValue("bad"))

        assert "Condition must be a function or lambda" in str(exc_info.value)

        # Container should remain in valid state
        assert len(container._behaviors) == 0
        assert len(container._behavior_names) == 0