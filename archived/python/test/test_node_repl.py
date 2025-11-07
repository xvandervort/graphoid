"""
Test the .node accessor through the REPL execution pipeline.
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.pipeline import ExecutionSession


class TestNodeAccessorInREPL:
    """Test the .node accessor through the execution pipeline."""

    def test_node_access_basic(self):
        """Test basic .node access in REPL."""
        session = ExecutionSession()

        # Create a list
        result = session.execute_statement('a = [1, 2, 3]')
        assert result.success

        # Access a node
        result = session.execute_statement('a[1].node')
        assert result.success
        assert result.value is not None
        assert result.value.get_type() == "node"

    def test_node_neighbors(self):
        """Test accessing neighbors through node."""
        session = ExecutionSession()

        # Create a list
        result = session.execute_statement('a = [10, 20, 30]')
        assert result.success

        # Get neighbors of middle element
        result = session.execute_statement('a[1].node.neighbors')
        assert result.success
        assert result.value.get_type() == "list"
        # Should have 2 neighbors (previous and next)
        assert len(result.value.elements) == 2

    def test_node_value(self):
        """Test accessing original value through node."""
        session = ExecutionSession()

        # Create a list
        result = session.execute_statement('a = ["hello", "world"]')
        assert result.success

        # Get value of node
        result = session.execute_statement('a[0].node.value')
        assert result.success
        assert result.value.value == "hello"

    def test_node_id(self):
        """Test that node has unique ID."""
        session = ExecutionSession()

        # Create a list
        result = session.execute_statement('a = [1, 2]')
        assert result.success

        # Get node IDs
        result1 = session.execute_statement('a[0].node.id')
        assert result1.success
        id1 = result1.value.value

        result2 = session.execute_statement('a[1].node.id')
        assert result2.success
        id2 = result2.value.value

        # IDs should be different
        assert id1 != id2

    def test_standalone_value_node_error(self):
        """Test that standalone values raise error on .node access."""
        session = ExecutionSession()

        # Create standalone value
        result = session.execute_statement('x = 42')
        assert result.success

        # Should error when accessing .node
        result = session.execute_statement('x.node')
        assert not result.success
        assert "not part of a graph" in str(result.error)


if __name__ == "__main__":
    pytest.main([__file__])