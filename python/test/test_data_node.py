"""Test suite for data node functionality in glang.

Data nodes are now internal representations accessed via map.node() method.
Users work with maps and extract data nodes when needed.
"""

import pytest
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession


class TestDataNodeAccess:
    """Test accessing data nodes through maps."""

    def test_data_node_from_map(self):
        """Test extracting data node from map using node() method."""
        session = ExecutionSession()

        # Create a map and extract a data node
        session.execute_statement('map<string> config = { "host": "localhost", "port": "8080" }')
        result = session.execute_statement('config.node("host")')

        assert result.success
        assert result.value.get_type() == "data"

    def test_data_node_key_access(self):
        """Test accessing key from data node."""
        session = ExecutionSession()

        session.execute_statement('map<string> config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')
        result = session.execute_statement('node.key()')

        assert result.success
        assert str(result.value) == "host"

    def test_data_node_value_access(self):
        """Test accessing value from data node."""
        session = ExecutionSession()

        session.execute_statement('map<string> config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')
        result = session.execute_statement('node.value()')

        assert result.success
        assert str(result.value) == "localhost"

    def test_data_node_with_number_value(self):
        """Test data node containing number value."""
        session = ExecutionSession()

        session.execute_statement('map settings = { "timeout": 30, "retries": 3 }')
        session.execute_statement('data timeout_node = settings.node("timeout")')

        result = session.execute_statement('timeout_node.key()')
        assert result.success
        assert str(result.value) == "timeout"

        result = session.execute_statement('timeout_node.value()')
        assert result.success
        assert result.value.value == 30


class TestDataNodeMethods:
    """Test data node methods."""

    def test_data_node_universal_methods(self):
        """Test that data nodes support universal methods."""
        session = ExecutionSession()

        session.execute_statement('map config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')

        # type()
        result = session.execute_statement('node.type()')
        assert result.success
        assert str(result.value) == "data"

        # inspect()
        result = session.execute_statement('node.inspect()')
        assert result.success
        assert "data" in str(result.value).lower()

    def test_data_node_inspect(self):
        """Test data node inspect method shows key-value info."""
        session = ExecutionSession()

        session.execute_statement('map config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')
        result = session.execute_statement('node.inspect()')

        assert result.success
        inspect_str = str(result.value)
        assert "data" in inspect_str
        assert "host" in inspect_str
        # The inspect shows type information, not the actual value


class TestDataNodeConstraints:
    """Test data node behavior with map type constraints."""

    def test_constrained_map_data_nodes(self):
        """Test that data nodes from constrained maps maintain type info."""
        session = ExecutionSession()

        # Create constrained map and extract data node
        session.execute_statement('map<string> config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')

        # The data node should contain the string value
        result = session.execute_statement('node.value()')
        assert result.success
        assert str(result.value) == "localhost"

    def test_data_node_from_mixed_map(self):
        """Test data nodes from maps with mixed value types."""
        session = ExecutionSession()

        session.execute_statement('map settings = { "host": "localhost", "port": 8080, "debug": true }')

        # Extract different typed values
        session.execute_statement('data host_node = settings.node("host")')
        result = session.execute_statement('host_node.value()')
        assert result.success
        assert str(result.value) == "localhost"

        session.execute_statement('data port_node = settings.node("port")')
        result = session.execute_statement('port_node.value()')
        assert result.success
        assert result.value.value == 8080

        session.execute_statement('data debug_node = settings.node("debug")')
        result = session.execute_statement('debug_node.value()')
        assert result.success
        assert result.value.value is True


class TestDataNodeEdgeCases:
    """Test edge cases and error conditions."""

    def test_nonexistent_key_raises_error(self):
        """Test that accessing nonexistent key raises appropriate error."""
        session = ExecutionSession()

        session.execute_statement('map config = { "host": "localhost" }')
        result = session.execute_statement('config.node("missing_key")')

        assert not result.success
        assert "not found" in str(result.error).lower()

    def test_data_node_key_immutability(self):
        """Test that data node keys cannot be modified."""
        session = ExecutionSession()

        session.execute_statement('map config = { "host": "localhost" }')
        session.execute_statement('data node = config.node("host")')

        # Attempting to call key() as setter should not exist
        result = session.execute_statement('node.key("newkey")')
        assert not result.success  # key() method takes no arguments