"""Tests for universal graph methods (first() and last())."""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestUniversalGraphMethods:
    """Test universal first() and last() methods across all graph types."""

    def setup_method(self):
        """Set up test environment."""
        self.session = ExecutionSession()

    def test_list_first_and_last(self):
        """Test first() and last() methods on lists."""
        # Test with non-empty list
        result = self.session.execute_statement('items = [10, 20, 30, 40, 50]')
        assert result.success

        # Test first()
        result = self.session.execute_statement('first_item = items.first()')
        assert result.success

        result = self.session.execute_statement('first_item')
        assert result.success
        assert result.value.value == 10

        # Test last()
        result = self.session.execute_statement('last_item = items.last()')
        assert result.success

        result = self.session.execute_statement('last_item')
        assert result.success
        assert result.value.value == 50

    def test_list_first_and_last_empty(self):
        """Test first() and last() methods on empty lists."""
        result = self.session.execute_statement('empty_list = []')
        assert result.success

        # Test first() on empty list
        result = self.session.execute_statement('first_empty = empty_list.first()')
        assert result.success

        result = self.session.execute_statement('first_empty')
        assert result.success
        assert result.value.get_type() == "none"

        # Test last() on empty list
        result = self.session.execute_statement('last_empty = empty_list.last()')
        assert result.success

        result = self.session.execute_statement('last_empty')
        assert result.success
        assert result.value.get_type() == "none"

    def test_list_first_and_last_single_element(self):
        """Test first() and last() methods on single-element lists."""
        result = self.session.execute_statement('single = ["only"]')
        assert result.success

        # Test first()
        result = self.session.execute_statement('first_single = single.first()')
        assert result.success

        result = self.session.execute_statement('first_single')
        assert result.success
        assert result.value.value == "only"

        # Test last()
        result = self.session.execute_statement('last_single = single.last()')
        assert result.success

        result = self.session.execute_statement('last_single')
        assert result.success
        assert result.value.value == "only"

    def test_map_first_and_last(self):
        """Test first() and last() methods on maps (insertion order)."""
        # Test with non-empty map
        result = self.session.execute_statement('config = {"host": "localhost", "port": 8080, "debug": true}')
        assert result.success

        # Test first() - should return first inserted key-value as map
        result = self.session.execute_statement('first_map = config.first()')
        assert result.success

        result = self.session.execute_statement('first_map')
        assert result.success
        assert result.value.get_type() == "map"
        # Should contain only the first key-value pair
        first_keys = result.value.keys()
        assert len(first_keys) == 1
        assert first_keys[0] == "host"
        assert result.value.get("host").value == "localhost"

        # Test last() - should return last inserted key-value as map
        result = self.session.execute_statement('last_map = config.last()')
        assert result.success

        result = self.session.execute_statement('last_map')
        assert result.success
        assert result.value.get_type() == "map"
        # Should contain only the last key-value pair
        last_keys = result.value.keys()
        assert len(last_keys) == 1
        assert last_keys[0] == "debug"
        assert result.value.get("debug").value is True

    def test_map_first_and_last_empty(self):
        """Test first() and last() methods on empty maps."""
        result = self.session.execute_statement('empty_map = {}')
        assert result.success

        # Test first() on empty map
        result = self.session.execute_statement('first_empty_map = empty_map.first()')
        assert result.success

        result = self.session.execute_statement('first_empty_map')
        assert result.success
        assert result.value.get_type() == "none"

        # Test last() on empty map
        result = self.session.execute_statement('last_empty_map = empty_map.last()')
        assert result.success

        result = self.session.execute_statement('last_empty_map')
        assert result.success
        assert result.value.get_type() == "none"

    def test_map_first_and_last_single_element(self):
        """Test first() and last() methods on single-element maps."""
        result = self.session.execute_statement('single_map = {"key": "value"}')
        assert result.success

        # Test first()
        result = self.session.execute_statement('first_map = single_map.first()')
        assert result.success

        result = self.session.execute_statement('first_map')
        assert result.success
        assert result.value.get_type() == "map"
        first_keys = result.value.keys()
        assert len(first_keys) == 1
        assert first_keys[0] == "key"
        assert result.value.get("key").value == "value"

        # Test last() - should be same as first for single element
        result = self.session.execute_statement('last_map = single_map.last()')
        assert result.success

        result = self.session.execute_statement('last_map')
        assert result.success
        assert result.value.get_type() == "map"
        last_keys = result.value.keys()
        assert len(last_keys) == 1
        assert last_keys[0] == "key"
        assert result.value.get("key").value == "value"

    # TODO: Add tree tests when tree{} literal syntax is implemented

    def test_universal_methods_with_parentheses_optional(self):
        """Test that first() and last() work with and without parentheses."""
        # Set up data
        result = self.session.execute_statement('numbers = [1, 2, 3]')
        assert result.success

        # With parentheses
        result = self.session.execute_statement('with_parens = numbers.first()')
        assert result.success

        result = self.session.execute_statement('with_parens')
        assert result.success
        first_with_parens = result.value.value

        # Without parentheses
        result = self.session.execute_statement('without_parens = numbers.first')
        assert result.success

        result = self.session.execute_statement('without_parens')
        assert result.success
        first_without_parens = result.value.value

        # Should be the same
        assert first_with_parens == first_without_parens == 1

    def test_map_insertion_order_preserved(self):
        """Test that maps preserve insertion order for first() and last()."""
        # Build map step by step to test insertion order
        result = self.session.execute_statement('ordered_map = {}')
        assert result.success

        result = self.session.execute_statement('ordered_map["first"] = "alpha"')
        assert result.success

        result = self.session.execute_statement('ordered_map["middle"] = "beta"')
        assert result.success

        result = self.session.execute_statement('ordered_map["last"] = "gamma"')
        assert result.success

        # Test first() - should return first inserted key-value pair
        result = self.session.execute_statement('first_ordered = ordered_map.first()')
        assert result.success

        result = self.session.execute_statement('first_ordered')
        assert result.success
        assert result.value.get_type() == "map"
        first_keys = result.value.keys()
        assert len(first_keys) == 1
        assert first_keys[0] == "first"
        assert result.value.get("first").value == "alpha"

        # Test last() - should return last inserted key-value pair
        result = self.session.execute_statement('last_ordered = ordered_map.last()')
        assert result.success

        result = self.session.execute_statement('last_ordered')
        assert result.success
        assert result.value.get_type() == "map"
        last_keys = result.value.keys()
        assert len(last_keys) == 1
        assert last_keys[0] == "last"
        assert result.value.get("last").value == "gamma"

    def test_consistency_across_graph_types(self):
        """Test that all graph types consistently support first() and last() methods."""
        # Test that all currently implemented graph types have these methods
        graph_types = [
            ('my_list = [1, 2, 3]', 'my_list'),
            ('my_map = {"a": 1, "b": 2}', 'my_map'),
            # TODO: Add tree when tree{} syntax is implemented
        ]

        for setup_stmt, var_name in graph_types:
            # Create the data structure
            result = self.session.execute_statement(setup_stmt)
            assert result.success, f"Failed to create {var_name}"

            # Test first() method exists and is callable
            result = self.session.execute_statement(f'{var_name}.first()')
            assert result.success, f"first() method failed for {var_name}"

            # Test last() method exists and is callable
            result = self.session.execute_statement(f'{var_name}.last()')
            assert result.success, f"last() method failed for {var_name}"

            # Clear for next iteration
            result = self.session.execute_statement(f'{var_name} = none')
            assert result.success