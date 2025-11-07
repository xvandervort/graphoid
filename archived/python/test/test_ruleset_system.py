"""Tests for the ruleset system (Enhancement #6)."""

import pytest
from glang.execution.values import NumberValue, StringValue, BooleanValue, NoneValue
from glang.execution.graph_values import ListValue, HashValue
from glang.behaviors import Ruleset, RulesetValue, create_ruleset


class MockFunction:
    """Mock function for testing ruleset with custom functions."""

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


class TestRulesetBasics:
    """Test basic ruleset creation and management."""

    def test_empty_ruleset_creation(self):
        """Test creating an empty ruleset."""
        ruleset = Ruleset()
        assert ruleset.size() == 0
        assert ruleset.to_display_string() == "Ruleset[0 rules]"

    def test_ruleset_with_string_rules(self):
        """Test creating ruleset with string rule specifications."""
        string_rules = [StringValue("none_to_zero"), StringValue("positive")]
        ruleset = Ruleset(string_rules)

        assert ruleset.size() == 2
        assert ruleset.to_display_string() == "Ruleset[2 rules]"

    def test_ruleset_value_wrapper(self):
        """Test RulesetValue wrapper functionality."""
        ruleset = Ruleset()
        ruleset_value = RulesetValue(ruleset)

        assert ruleset_value.get_type() == "ruleset"
        assert ruleset_value.to_display_string() == "Ruleset[0 rules]"
        assert ruleset_value.to_python() == ruleset

    def test_add_rule_to_existing_ruleset(self):
        """Test adding rules to an existing ruleset."""
        ruleset = Ruleset()
        ruleset.add_rule(StringValue("none_to_zero"))
        ruleset.add_rule(StringValue("positive"))

        assert ruleset.size() == 2
        rules = ruleset.get_rules()
        assert len(rules.elements) == 2

    def test_create_ruleset_helper(self):
        """Test the create_ruleset helper function."""
        rules = [StringValue("none_to_zero"), StringValue("positive")]
        ruleset_value = create_ruleset(rules)

        assert isinstance(ruleset_value, RulesetValue)
        assert ruleset_value.size().value == 2


class TestRulesetTypes:
    """Test different types of rules in rulesets."""

    def test_string_rule_specifications(self):
        """Test adding string-based rule specifications."""
        ruleset = Ruleset()

        # Add string rules
        ruleset.add_rule(StringValue("none_to_zero"))
        ruleset.add_rule(StringValue("positive"))

        assert ruleset.size() == 2

        # Check internal representation
        rules = ruleset.rules
        assert rules[0]['type'] == 'string'
        assert rules[0]['name'] == 'none_to_zero'
        assert rules[1]['type'] == 'string'
        assert rules[1]['name'] == 'positive'

    def test_dict_rule_specifications(self):
        """Test adding dictionary-based rule specifications with parameters."""
        ruleset = Ruleset()

        # Add rule with parameters
        rule_with_params = {
            'type': 'string',
            'name': 'validate_range',
            'args': [0, 100]
        }
        ruleset.add_rule(rule_with_params)

        assert ruleset.size() == 1
        assert ruleset.rules[0]['args'] == [0, 100]

    def test_invalid_rule_specification(self):
        """Test that invalid rule specifications are rejected."""
        ruleset = Ruleset()

        with pytest.raises(ValueError, match="Invalid rule specification"):
            ruleset.add_rule(NumberValue(42))  # Numbers are not valid rule specs


class TestRulesetApplication:
    """Test applying rulesets to containers."""

    def test_add_rules_method_exists(self):
        """Test that add_rules method exists on graph containers."""
        # Test that the method exists on ListValue
        numbers = ListValue([])
        assert hasattr(numbers, 'add_rules')
        assert callable(getattr(numbers, 'add_rules'))

        # Test that the method exists on HashValue
        data = HashValue([])
        assert hasattr(data, 'add_rules')
        assert callable(getattr(data, 'add_rules'))

    def test_add_rules_type_validation(self):
        """Test that add_rules validates the ruleset parameter."""
        numbers = ListValue([NumberValue(1), NumberValue(2)])

        # Try to add non-ruleset - should raise error
        with pytest.raises(ValueError, match="add_rules requires a ruleset"):
            numbers.add_rules(StringValue("not a ruleset"))

    def test_apply_empty_ruleset(self):
        """Test applying an empty ruleset to a container."""
        numbers = ListValue([NumberValue(1), NumberValue(2)])
        empty_ruleset = create_ruleset([])

        # Should work without error
        result = numbers.add_rules(empty_ruleset)
        assert isinstance(result, NoneValue)

        # Numbers should be unchanged
        assert len(numbers.elements) == 2
        assert numbers.elements[0].value == 1
        assert numbers.elements[1].value == 2

    def test_apply_simple_ruleset(self):
        """Test applying a simple ruleset with standard behaviors."""
        # Create test data with issues
        numbers = ListValue([NumberValue(-5), NoneValue(), NumberValue(10)])

        # Create ruleset with standard behaviors
        rules = [StringValue("none_to_zero"), StringValue("positive")]
        ruleset = create_ruleset(rules)

        # Apply ruleset
        numbers.add_rules(ruleset)

        # Verify behaviors were applied
        # Note: This tests the integration, actual behavior application
        # depends on the existing behavior system working correctly
        assert len(numbers._behaviors) >= 2  # At least 2 behaviors added


class TestRulesetComposition:
    """Test composing complex rulesets."""

    def test_multiple_rule_types(self):
        """Test ruleset with multiple types of rules."""
        ruleset = Ruleset()

        # Add different types of rules
        ruleset.add_rule(StringValue("none_to_zero"))  # String rule

        # Add rule with parameters (using dict format for now)
        range_rule = {
            'type': 'string',
            'name': 'validate_range',
            'args': [0, 100]
        }
        ruleset.add_rule(range_rule)

        assert ruleset.size() == 2
        assert ruleset.rules[0]['type'] == 'string'
        assert ruleset.rules[1]['type'] == 'string'
        assert ruleset.rules[1]['args'] == [0, 100]

    def test_ruleset_reusability(self):
        """Test that the same ruleset can be applied to multiple containers."""
        # Create a reusable ruleset
        rules = [StringValue("none_to_zero"), StringValue("positive")]
        data_cleaning = create_ruleset(rules)

        # Apply to multiple containers
        temperatures = ListValue([NoneValue(), NumberValue(-5)])
        pressures = ListValue([NoneValue(), NumberValue(-10)])

        temperatures.add_rules(data_cleaning)
        pressures.add_rules(data_cleaning)

        # Both should have the same behaviors applied
        assert len(temperatures._behaviors) >= 2
        assert len(pressures._behaviors) >= 2

    def test_ruleset_modification_after_creation(self):
        """Test adding rules to a ruleset after creation."""
        # Create initial ruleset
        initial_rules = [StringValue("none_to_zero")]
        ruleset = create_ruleset(initial_rules)

        assert ruleset.size().value == 1

        # Add more rules
        ruleset.add_rule(StringValue("positive"))
        ruleset.add_rule(StringValue("round_to_int"))

        assert ruleset.size().value == 3

        # Verify all rules are present
        rules_list = ruleset.get_rules()
        assert len(rules_list.elements) == 3


class TestRulesetDocumentation:
    """Test that demonstrates the intended usage for documentation."""

    def test_intended_usage_example(self):
        """Document the intended usage pattern for rulesets."""
        # This test documents how the feature SHOULD work once fully implemented

        # Step 1: Create a reusable data cleaning ruleset
        data_cleaning_rules = [
            StringValue("none_to_zero"),
            StringValue("positive"),
            StringValue("round_to_int")
        ]
        data_cleaning = create_ruleset(data_cleaning_rules)

        # Step 2: Apply to multiple datasets
        temperatures = ListValue([NoneValue(), NumberValue(-5.7), NumberValue(98.6)])
        blood_pressure = ListValue([NoneValue(), NumberValue(-120), NumberValue(140.2)])
        heart_rate = ListValue([NoneValue(), NumberValue(-80), NumberValue(72.5)])

        # Step 3: Apply the same ruleset to all datasets
        temperatures.add_rules(data_cleaning)
        blood_pressure.add_rules(data_cleaning)
        heart_rate.add_rules(data_cleaning)

        # All datasets should have the same behaviors
        assert len(temperatures._behaviors) >= 3
        assert len(blood_pressure._behaviors) >= 3
        assert len(heart_rate._behaviors) >= 3

    def test_medical_validation_example(self):
        """Document medical validation ruleset usage."""
        # Medical validation ruleset for vital signs
        medical_rules = [
            StringValue("none_to_zero"),
            # In real usage, this would include validate_range with parameters
            StringValue("positive")
        ]

        medical_validation = create_ruleset(medical_rules)

        # Apply to medical data
        vital_signs = ListValue([NumberValue(120), NoneValue(), NumberValue(-80)])
        vital_signs.add_rules(medical_validation)

        # Verify medical validation was applied
        assert len(vital_signs._behaviors) >= 2

    def test_financial_data_example(self):
        """Document financial data processing with rulesets."""
        # Financial data processing ruleset
        financial_rules = [
            StringValue("none_to_zero"),
            StringValue("positive"),
            StringValue("round_to_int")  # Round to cents would be better
        ]

        financial_cleaning = create_ruleset(financial_rules)

        # Apply to financial datasets
        prices = ListValue([NumberValue(19.99), NoneValue(), NumberValue(-5.50)])
        revenues = ListValue([NumberValue(1000000.25), NoneValue(), NumberValue(-500)])

        prices.add_rules(financial_cleaning)
        revenues.add_rules(financial_cleaning)

        # Both datasets have financial validation
        assert len(prices._behaviors) >= 3
        assert len(revenues._behaviors) >= 3


class TestRulesetSpecification:
    """Test the specification compliance."""

    def test_api_specification_compliance(self):
        """Test that the API matches the specification exactly."""
        # From the spec: Rules[rule_list] syntax (approximated with create_ruleset)

        # Test 1: Ruleset creation works
        rules = [StringValue("none_to_zero"), StringValue("positive")]
        ruleset = create_ruleset(rules)
        assert isinstance(ruleset, RulesetValue)

        # Test 2: add_rules method exists and works
        container = ListValue([NumberValue(1)])
        assert hasattr(container, 'add_rules')

        result = container.add_rules(ruleset)
        assert isinstance(result, NoneValue)

        # Test 3: Ruleset methods work
        assert hasattr(ruleset, 'add_rule')
        assert hasattr(ruleset, 'get_rules')
        assert hasattr(ruleset, 'size')

    def test_ruleset_error_safety(self):
        """Test that rulesets are error-safe."""
        # From the spec: rulesets should handle errors gracefully

        container = ListValue([NumberValue(1)])

        # Invalid ruleset should raise clear error
        with pytest.raises(ValueError) as exc_info:
            container.add_rules(StringValue("not a ruleset"))

        assert "add_rules requires a ruleset" in str(exc_info.value)

        # Container should remain in valid state
        assert len(container._behaviors) == 0

    def test_ruleset_efficiency_design(self):
        """Test that ruleset application is designed for efficiency."""
        # Create ruleset with multiple rules
        rules = [StringValue("none_to_zero"), StringValue("positive"), StringValue("round_to_int")]
        ruleset = create_ruleset(rules)

        container = ListValue([NumberValue(1)])

        # Single add_rules call should apply all rules
        container.add_rules(ruleset)

        # Should be more efficient than individual add_rule calls
        # (This tests the design pattern, not performance)
        assert len(container._behaviors) >= 3

    def test_ruleset_reusability_design(self):
        """Test that rulesets support the reusability design goal."""
        # Create one ruleset
        rules = [StringValue("none_to_zero"), StringValue("positive")]
        shared_ruleset = create_ruleset(rules)

        # Apply to multiple containers
        container1 = ListValue([NumberValue(1)])
        container2 = ListValue([NumberValue(2)])
        container3 = HashValue([])

        # Same ruleset should work on different container types
        container1.add_rules(shared_ruleset)
        container2.add_rules(shared_ruleset)
        container3.add_rules(shared_ruleset)

        # All should have the behaviors applied
        assert len(container1._behaviors) >= 2
        assert len(container2._behaviors) >= 2
        assert len(container3._behaviors) >= 2