"""
Test the .node accessor for graph-aware element access.

This tests that individual elements can access their graph context through .node
"""

import pytest
import sys
import os

# Add src to path for imports
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'src'))

from glang.execution.graph_values import ListValue
from glang.execution.values import NumberValue, StringValue
from glang.execution.errors import GraphError


class TestNodeAccessor:
    """Test the .node property for accessing graph-aware nodes."""

    def test_simple_list_node_access(self):
        """Test basic .node access on list elements."""
        my_list = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Get an element
        element = my_list[1]  # This should be NumberValue(2)
        assert element.value == 2

        # Access its node
        node = element.node
        assert node is not None
        assert node.value == element  # Node contains the value

    def test_node_neighbors_in_list(self):
        """Test that nodes can access their neighbors in a list."""
        my_list = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Middle element should have two neighbors
        middle = my_list[1]
        node = middle.node
        neighbors = node.neighbors

        # Should have 2 neighbors (previous and next)
        assert len(neighbors) == 2

        # Check neighbor values
        neighbor_values = [n.value.value for n in neighbors]
        assert 1 in neighbor_values  # Previous element
        assert 3 in neighbor_values  # Next element

    def test_node_container_reference(self):
        """Test that nodes know their container."""
        my_list = ListValue([NumberValue(10), NumberValue(20)])

        element = my_list[0]
        node = element.node

        # Node should know its container
        assert node.container is not None
        assert node.container == my_list.graph  # Should be the underlying graph

    def test_node_unique_id(self):
        """Test that each node has a unique ID."""
        my_list = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # Get all node IDs
        ids = []
        for i in range(3):
            element = my_list[i]
            node = element.node
            ids.append(node.id)

        # All IDs should be unique
        assert len(ids) == 3
        assert len(set(ids)) == 3  # No duplicates

    def test_value_not_in_graph_error(self):
        """Test that accessing .node on a value not in a graph raises an error."""
        # Create a standalone value not in any graph
        standalone = NumberValue(42)

        # Should raise GraphError when accessing .node
        with pytest.raises(GraphError) as exc_info:
            _ = standalone.node

        assert "not part of a graph" in str(exc_info.value)

    def test_node_has_neighbor_check(self):
        """Test the has_neighbor method."""
        my_list = ListValue([StringValue("a"), StringValue("b"), StringValue("c")])

        first = my_list[0].node
        second = my_list[1].node
        third = my_list[2].node

        # First and second are neighbors
        assert first.has_neighbor(second)
        assert second.has_neighbor(first)

        # Second and third are neighbors
        assert second.has_neighbor(third)
        assert third.has_neighbor(second)

        # First and third are NOT neighbors (no direct connection)
        assert not first.has_neighbor(third)
        assert not third.has_neighbor(first)

    def test_node_path_finding(self):
        """Test finding paths between nodes."""
        my_list = ListValue([NumberValue(i) for i in range(5)])

        first = my_list[0].node
        last = my_list[4].node

        # There should be a path from first to last
        path = first.path_to(last)
        assert path is not None
        assert len(path) == 5  # Should go through all 5 nodes

        # Distance should be 4 (number of edges)
        distance = first.distance_to(last)
        assert distance == 4

    def test_first_and_last_neighbors(self):
        """Test edge cases for first and last elements."""
        my_list = ListValue([NumberValue(1), NumberValue(2), NumberValue(3)])

        # First element has only one neighbor (next)
        first = my_list[0].node
        assert len(first.neighbors) == 1
        assert first.neighbors[0].value.value == 2

        # Last element has only one neighbor (previous)
        last = my_list[2].node
        assert len(last.neighbors) == 1
        assert last.neighbors[0].value.value == 2


if __name__ == "__main__":
    pytest.main([__file__])