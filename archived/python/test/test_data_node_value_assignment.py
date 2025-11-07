"""Test data node value updates through map modifications.

Since data nodes are internal representations, values are updated by
modifying the source map and extracting fresh data nodes.
"""

import pytest
from glang.execution.pipeline import ExecutionSession


class TestDataNodeValueUpdates:
    """Test that data node values reflect map updates."""

    def test_map_update_reflects_in_data_node(self):
        """Test that updating map reflects in extracted data nodes."""
        session = ExecutionSession()

        # Create map
        session.execute_statement('map config = { "name": "Alice", "age": 25 }')

        # Extract initial data node
        session.execute_statement('data name_node = config.node("name")')
        result = session.execute_statement('name_node.value()')
        assert result.success
        assert str(result.value) == "Alice"

        # Update the map
        result = session.execute_statement('config["name"] = "Bob"')
        assert result.success

        # Extract fresh data node - should reflect the update
        session.execute_statement('data updated_name_node = config.node("name")')
        result = session.execute_statement('updated_name_node.value()')
        assert result.success
        assert str(result.value) == "Bob"

    def test_different_value_types(self):
        """Test updating map values with different types."""
        session = ExecutionSession()

        session.execute_statement('map settings = { "debug": true, "timeout": 30 }')

        # Check initial boolean value
        session.execute_statement('data debug_node = settings.node("debug")')
        result = session.execute_statement('debug_node.value()')
        assert result.success
        assert result.value.value is True

        # Update to false
        session.execute_statement('settings["debug"] = false')
        session.execute_statement('data updated_debug = settings.node("debug")')
        result = session.execute_statement('updated_debug.value()')
        assert result.success
        assert result.value.value is False

        # Check number value update
        session.execute_statement('settings["timeout"] = 60')
        session.execute_statement('data timeout_node = settings.node("timeout")')
        result = session.execute_statement('timeout_node.value()')
        assert result.success
        assert result.value.value == 60

    def test_key_immutability_in_data_nodes(self):
        """Test that data node keys cannot be changed."""
        session = ExecutionSession()

        session.execute_statement('map config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')

        # Key should be immutable - key() method takes no arguments
        result = session.execute_statement('node.key("newkey")')
        assert not result.success
        assert "argument" in str(result.error).lower()

    def test_constrained_map_value_updates(self):
        """Test value updates in type-constrained maps."""
        session = ExecutionSession()

        # Create constrained map
        session.execute_statement('map<string> config = { "host": "localhost", "port": "8080" }')

        # Update with valid string value
        result = session.execute_statement('config["host"] = "127.0.0.1"')
        assert result.success

        # Verify update in data node
        session.execute_statement('data host_node = config.node("host")')
        result = session.execute_statement('host_node.value()')
        assert result.success
        assert str(result.value) == "127.0.0.1"

    def test_multiple_data_nodes_independence(self):
        """Test that multiple data nodes from same map are independent snapshots."""
        session = ExecutionSession()

        session.execute_statement('map users = { "admin": "Alice", "user": "Bob" }')

        # Extract both data nodes
        session.execute_statement('data admin_node = users.node("admin")')
        session.execute_statement('data user_node = users.node("user")')

        # Verify initial values
        result = session.execute_statement('admin_node.value()')
        assert result.success
        assert str(result.value) == "Alice"

        result = session.execute_statement('user_node.value()')
        assert result.success
        assert str(result.value) == "Bob"

        # Update map
        session.execute_statement('users["admin"] = "Charlie"')

        # Old data node should still have old value (snapshot behavior)
        result = session.execute_statement('admin_node.value()')
        assert result.success
        assert str(result.value) == "Alice"  # Unchanged snapshot

        # But new extraction gets updated value
        session.execute_statement('data new_admin_node = users.node("admin")')
        result = session.execute_statement('new_admin_node.value()')
        assert result.success
        assert str(result.value) == "Charlie"

    def test_data_node_with_complex_values(self):
        """Test data nodes containing complex values like lists."""
        session = ExecutionSession()

        session.execute_statement('map data = { "scores": [95, 87, 92], "tags": ["important", "urgent"] }')

        # Extract data node with list value
        session.execute_statement('data scores_node = data.node("scores")')
        result = session.execute_statement('scores_node.value()')
        assert result.success
        assert result.value.get_type() == "list"

        # Update the list in the map
        session.execute_statement('data["scores"] = [100, 95, 90]')

        # Extract fresh data node
        session.execute_statement('data updated_scores = data.node("scores")')
        result = session.execute_statement('updated_scores.value()')
        assert result.success
        # The list should have new values
        assert len(result.value.elements) == 3
        assert result.value.elements[0].value == 100