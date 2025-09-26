"""Tests for generic mapping behavior system."""

import pytest
from glang.execution.values import NumberValue, StringValue, BooleanValue, NoneValue
from glang.execution.graph_values import ListValue, HashValue
from glang.behaviors import MappingBehavior


class TestMappingBehavior:
    """Test the MappingBehavior class."""

    def test_basic_string_to_number_mapping(self):
        """Test basic string to number mapping."""
        mapping = HashValue([
            ("red", NumberValue(1)),
            ("green", NumberValue(2)),
            ("blue", NumberValue(3))
        ])

        behavior = MappingBehavior(mapping)

        assert behavior.apply(StringValue("red")).value == 1
        assert behavior.apply(StringValue("green")).value == 2
        assert behavior.apply(StringValue("blue")).value == 3

    def test_unmapped_value_without_default(self):
        """Test that unmapped values pass through unchanged."""
        mapping = HashValue([("red", NumberValue(1))])
        behavior = MappingBehavior(mapping)

        purple = StringValue("purple")
        result = behavior.apply(purple)
        assert isinstance(result, StringValue)
        assert result.value == "purple"

    def test_unmapped_value_with_default(self):
        """Test that unmapped values use default."""
        mapping = HashValue([("red", NumberValue(1))])
        default = NumberValue(0)
        behavior = MappingBehavior(mapping, default)

        result = behavior.apply(StringValue("purple"))
        assert isinstance(result, NumberValue)
        assert result.value == 0

    def test_string_to_string_mapping(self):
        """Test string to string mapping (department names)."""
        mapping = HashValue([
            ("walter", StringValue("HR")),
            ("james", StringValue("IT")),
            ("emily", StringValue("Admin"))
        ])

        behavior = MappingBehavior(mapping, StringValue("Unknown"))

        assert behavior.apply(StringValue("walter")).value == "HR"
        assert behavior.apply(StringValue("james")).value == "IT"
        assert behavior.apply(StringValue("unknown_person")).value == "Unknown"

    def test_number_to_string_mapping(self):
        """Test number to string mapping (status codes)."""
        mapping = HashValue([
            ("200", StringValue("OK")),
            ("404", StringValue("Not Found")),
            ("500", StringValue("Server Error"))
        ])

        behavior = MappingBehavior(mapping)

        assert behavior.apply(NumberValue(200)).value == "OK"
        assert behavior.apply(NumberValue(404)).value == "Not Found"

    def test_case_sensitive_mapping(self):
        """Test that mapping is case-sensitive."""
        mapping = HashValue([
            ("red", NumberValue(1)),
            ("RED", NumberValue(10))
        ])

        behavior = MappingBehavior(mapping)

        assert behavior.apply(StringValue("red")).value == 1
        assert behavior.apply(StringValue("RED")).value == 10


class TestListMappingRule:
    """Test add_mapping_rule() on ListValue."""

    def test_add_mapping_rule_to_list(self):
        """Test adding a mapping rule to a list."""
        colors = ListValue([StringValue("red"), StringValue("green"), StringValue("blue")])

        mapping = HashValue([
            ("red", NumberValue(1)),
            ("green", NumberValue(2)),
            ("blue", NumberValue(3))
        ])

        colors.add_mapping_rule(mapping)

        assert colors.elements[0].value == 1
        assert colors.elements[1].value == 2
        assert colors.elements[2].value == 3

    def test_mapping_rule_applies_to_new_elements(self):
        """Test that mapping applies to newly appended elements."""
        colors = ListValue([])

        mapping = HashValue([
            ("red", NumberValue(1)),
            ("green", NumberValue(2))
        ])

        colors.add_mapping_rule(mapping)

        colors.append(StringValue("red"))
        colors.append(StringValue("green"))

        assert colors.elements[0].value == 1
        assert colors.elements[1].value == 2

    def test_mapping_with_default_value(self):
        """Test mapping with default value for unmapped keys."""
        colors = ListValue([
            StringValue("red"),
            StringValue("purple"),
            StringValue("green")
        ])

        mapping = HashValue([
            ("red", NumberValue(1)),
            ("green", NumberValue(2))
        ])

        colors.add_mapping_rule(mapping, NumberValue(0))

        assert colors.elements[0].value == 1
        assert colors.elements[1].value == 0
        assert colors.elements[2].value == 2

    def test_employee_to_department_mapping(self):
        """Test practical example: employee to department mapping."""
        employees = ListValue([
            StringValue("walter"),
            StringValue("james"),
            StringValue("unknown"),
            StringValue("emily")
        ])

        dept_mapping = HashValue([
            ("walter", StringValue("HR")),
            ("james", StringValue("IT")),
            ("emily", StringValue("Admin"))
        ])

        employees.add_mapping_rule(dept_mapping, StringValue("Unknown"))

        assert employees.elements[0].value == "HR"
        assert employees.elements[1].value == "IT"
        assert employees.elements[2].value == "Unknown"
        assert employees.elements[3].value == "Admin"

    def test_multiple_mappings_in_sequence(self):
        """Test multiple mapping behaviors applied in sequence."""
        values = ListValue([StringValue("a"), StringValue("b")])

        first_map = HashValue([
            ("a", StringValue("alpha")),
            ("b", StringValue("beta"))
        ])

        second_map = HashValue([
            ("alpha", NumberValue(1)),
            ("beta", NumberValue(2))
        ])

        values.add_mapping_rule(first_map)
        values.add_mapping_rule(second_map)

        assert values.elements[0].value == 1
        assert values.elements[1].value == 2


class TestHashMappingRule:
    """Test add_mapping_rule() on HashValue."""

    def test_add_mapping_rule_to_hash(self):
        """Test adding a mapping rule to a hash."""
        config = HashValue([
            ("env", StringValue("dev")),
            ("region", StringValue("us-west"))
        ])

        env_mapping = HashValue([
            ("dev", StringValue("development")),
            ("prod", StringValue("production")),
            ("staging", StringValue("staging"))
        ])

        config.add_mapping_rule(env_mapping)

        assert config.get("env").value == "development"
        assert config.get("region").value == "us-west"

    def test_hash_mapping_with_default(self):
        """Test hash mapping with default value."""
        data = HashValue([
            ("status", StringValue("active")),
            ("unknown", StringValue("weird"))
        ])

        status_mapping = HashValue([
            ("active", NumberValue(1)),
            ("inactive", NumberValue(0))
        ])

        data.add_mapping_rule(status_mapping, NumberValue(-1))

        assert data.get("status").value == 1
        assert data.get("unknown").value == -1


class TestMappingWithOtherBehaviors:
    """Test mapping behaviors combined with other behaviors."""

    def test_mapping_combined_with_none_to_zero(self):
        """Test mapping combined with none_to_zero behavior."""
        values = ListValue([
            StringValue("red"),
            NoneValue(),
            StringValue("green")
        ])

        values.add_rule(StringValue("none_to_zero"))

        color_map = HashValue([
            ("red", NumberValue(1)),
            ("green", NumberValue(2)),
            ("0", NumberValue(99))
        ])
        values.add_mapping_rule(color_map)

        assert values.elements[0].value == 1
        assert values.elements[1].value == 99
        assert values.elements[2].value == 2

    def test_behavior_ordering_matters(self):
        """Test that behavior application order matters."""
        values = ListValue([StringValue("RED")])

        color_map = HashValue([("red", NumberValue(1))])

        values.add_rule(StringValue("lowercase"))
        values.add_mapping_rule(color_map)

        assert values.elements[0].value == 1