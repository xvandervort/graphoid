"""Test intrinsic behavior system for graph containers (lists, hashes)."""

import pytest
from glang.execution.values import (
    NumberValue, StringValue,
    BooleanValue, NoneValue, SymbolValue
)
from glang.execution.graph_values import ListValue, HashValue
from glang.behaviors import registry


class TestIntrinsicListBehaviors:
    """Test behavior support built into ListValue."""

    def test_list_add_rule_with_string(self):
        """Test adding behavior rule with string name."""
        # Create list with some nil values
        lst = ListValue([NumberValue(5), NoneValue(), NumberValue(10)])

        # Add nil_to_zero behavior
        lst.add_rule(StringValue("nil_to_zero"))

        # Existing elements should be transformed
        assert lst.elements[0].value == 5
        assert lst.elements[1].value == 0  # nil became 0
        assert lst.elements[2].value == 10

        # New elements should be transformed on append
        lst.append(NoneValue())
        assert lst.elements[3].value == 0

    def test_list_add_rule_with_symbol(self):
        """Test adding behavior rule with symbol."""
        lst = ListValue([NumberValue(5), NoneValue()])

        # Add behavior using symbol
        lst.add_rule(SymbolValue("nil_to_zero"))

        # Should work same as string
        assert lst.elements[1].value == 0

        # Verify it's the same behavior
        assert lst.has_rule(StringValue("nil_to_zero")).value == True
        assert lst.has_rule(SymbolValue("nil_to_zero")).value == True

    def test_list_validate_range_behavior(self):
        """Test range validation behavior with parameters."""
        lst = ListValue([NumberValue(50), NumberValue(150), NumberValue(-10)])

        # Add range validation 0-100
        lst.add_rule(StringValue("validate_range"), NumberValue(0), NumberValue(100))

        # Values should be clamped
        assert lst.elements[0].value == 50   # unchanged
        assert lst.elements[1].value == 100  # clamped from 150
        assert lst.elements[2].value == 0    # clamped from -10

        # New values should be clamped on append
        lst.append(NumberValue(200))
        assert lst.elements[3].value == 100

    def test_list_multiple_behaviors(self):
        """Test multiple behaviors applied in order."""
        lst = ListValue([NoneValue(), NumberValue(-50)])

        # Add two behaviors
        lst.add_rule(StringValue("nil_to_zero"))
        lst.add_rule(StringValue("positive"))

        # nil -> 0 -> 0 (already positive)
        assert lst.elements[0].value == 0
        # -50 -> 50 (made positive)
        assert lst.elements[1].value == 50

    def test_list_remove_rule(self):
        """Test removing a behavior rule."""
        lst = ListValue([NoneValue()])

        # Add and then remove behavior
        lst.add_rule(StringValue("nil_to_zero"))
        assert lst.elements[0].value == 0

        removed = lst.remove_rule(StringValue("nil_to_zero"))
        assert removed.value == True

        # New nils should not be transformed
        lst.append(NoneValue())
        assert isinstance(lst.elements[1], NoneValue)

    def test_list_get_rules(self):
        """Test getting list of active rules."""
        lst = ListValue([])

        # Initially no rules
        rules = lst.get_rules()
        assert len(rules.elements) == 0

        # Add some rules
        lst.add_rule(StringValue("nil_to_zero"))
        lst.add_rule(StringValue("positive"))

        rules = lst.get_rules()
        assert len(rules.elements) == 2
        # Rules should be sorted alphabetically
        assert rules.elements[0].value == "nil_to_zero"
        assert rules.elements[1].value == "positive"

    def test_list_clear_rules(self):
        """Test clearing all behavior rules."""
        lst = ListValue([NumberValue(-5)])

        lst.add_rule(StringValue("positive"))
        assert lst.elements[0].value == 5

        lst.clear_rules()

        # New negatives should not be transformed
        lst.append(NumberValue(-10))
        assert lst.elements[1].value == -10

    def test_list_duplicate_rule_ignored(self):
        """Test that duplicate rules are silently ignored."""
        lst = ListValue([])

        lst.add_rule(StringValue("nil_to_zero"))
        lst.add_rule(StringValue("nil_to_zero"))  # Duplicate

        rules = lst.get_rules()
        assert len(rules.elements) == 1  # Only one instance

    def test_list_string_behaviors(self):
        """Test string transformation behaviors."""
        lst = ListValue([StringValue("hello"), StringValue("WORLD")])

        # Add uppercase behavior
        lst.add_rule(StringValue("uppercase"))

        assert lst.elements[0].value == "HELLO"
        assert lst.elements[1].value == "WORLD"

        # New strings should be uppercased
        lst.append(StringValue("test"))
        assert lst.elements[2].value == "TEST"


class TestIntrinsicHashBehaviors:
    """Test behavior support built into HashValue."""

    def test_hash_add_rule_nil_to_zero(self):
        """Test adding nil_to_zero behavior to hash."""
        # Create hash with nil value
        hash_val = HashValue([("count", NoneValue()), ("total", NumberValue(100))])

        # Add behavior
        hash_val.add_rule(StringValue("nil_to_zero"))

        # Existing nil should be transformed
        assert hash_val.get("count").value == 0
        assert hash_val.get("total").value == 100

        # New nils should be transformed
        hash_val.set("missing", NoneValue())
        assert hash_val.get("missing").value == 0

    def test_hash_validate_range_behavior(self):
        """Test range validation on hash values."""
        hash_val = HashValue([("temp", NumberValue(150)), ("humidity", NumberValue(-20))])

        # Add range validation 0-100
        hash_val.add_rule(StringValue("validate_range"), NumberValue(0), NumberValue(100))

        # Values should be clamped
        assert hash_val.get("temp").value == 100
        assert hash_val.get("humidity").value == 0

        # New values should be clamped
        hash_val.set("pressure", NumberValue(200))
        assert hash_val.get("pressure").value == 100

    def test_hash_behavior_with_symbol(self):
        """Test hash behaviors using symbols."""
        hash_val = HashValue([("name", StringValue("alice"))])

        # Add behavior using symbol
        hash_val.add_rule(SymbolValue("uppercase"))

        assert hash_val.get("name").value == "ALICE"

        # Check rule exists
        assert hash_val.has_rule(SymbolValue("uppercase")).value == True

    def test_hash_get_and_clear_rules(self):
        """Test getting and clearing hash rules."""
        hash_val = HashValue([])

        # Add multiple rules
        hash_val.add_rule(StringValue("nil_to_zero"))
        hash_val.add_rule(StringValue("positive"))

        rules = hash_val.get_rules()
        assert len(rules.elements) == 2

        # Clear all rules
        hash_val.clear_rules()

        rules = hash_val.get_rules()
        assert len(rules.elements) == 0


class TestBehaviorInteraction:
    """Test interaction between behaviors and type constraints."""

    def test_behavior_respects_type_constraint(self):
        """Test that behaviors work with type constraints."""
        # List constrained to numbers
        lst = ListValue([], constraint="num")

        # Add nil_to_zero behavior
        lst.add_rule(StringValue("nil_to_zero"))

        # nil -> 0 satisfies num constraint
        lst.append(NoneValue())
        assert lst.elements[0].value == 0

        # String should still fail constraint
        with pytest.raises(Exception) as exc_info:
            lst.append(StringValue("test"))
        assert "Cannot append string to list<num>" in str(exc_info.value)

    def test_behavior_can_change_type(self):
        """Test behaviors that change value types."""
        lst = ListValue([StringValue("red"), StringValue("blue")])

        # Add color mapping behavior (string -> number)
        lst.add_rule(StringValue("map_colors"))

        # Colors should be mapped to numbers
        assert lst.elements[0].value == 1  # red -> 1
        assert lst.elements[1].value == 3  # blue -> 3

        # Unknown colors remain strings
        lst.append(StringValue("purple"))
        assert isinstance(lst.elements[2], StringValue)
        assert lst.elements[2].value == "purple"