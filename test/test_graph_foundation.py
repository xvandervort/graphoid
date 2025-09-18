"""Tests for graph foundation and graph-based values."""

import pytest
from glang.execution.graph_foundation import (
    GraphNode, GraphStructure, SequentialGraph, KeyedGraph,
    EdgeType, EdgeMetadata
)
from glang.execution.graph_values import ListValue, HashValue
from glang.execution.values import NumberValue, StringValue, BooleanValue


class TestGraphFoundation:
    """Test the core graph structures."""

    def test_graph_node_creation(self):
        """Test creating graph nodes."""
        value = NumberValue(42)
        node = GraphNode(value)

        assert node.value == value
        assert node.node_id is not None
        assert len(node._outgoing) == 0
        assert len(node._incoming) == 0

    def test_graph_node_edges(self):
        """Test adding edges between nodes."""
        node1 = GraphNode(NumberValue(1))
        node2 = GraphNode(NumberValue(2))

        metadata = EdgeMetadata(EdgeType.NAMED, key="connects_to")
        node1.add_edge_to(node2, metadata)

        assert node1.has_edge_to(node2)
        assert not node2.has_edge_to(node1)  # One-way
        assert len(node1.get_neighbors()) == 1
        assert node1.get_neighbors()[0] == node2

    def test_sequential_graph(self):
        """Test sequential graph structure (like lists)."""
        values = [NumberValue(1), NumberValue(2), NumberValue(3)]
        graph = SequentialGraph(values)

        assert len(graph) == 3
        assert graph.get_at_index(0).value == 1
        assert graph.get_at_index(2).value == 3

        # Test sequential connections
        first_node = graph.sequence_order[0]
        second_node = graph.sequence_order[1]
        assert first_node.has_edge_to(second_node)

    def test_keyed_graph(self):
        """Test keyed graph structure (like hashes)."""
        pairs = [("name", StringValue("Alice")), ("age", NumberValue(25))]
        graph = KeyedGraph(pairs)

        assert len(graph) == 2
        assert graph.get("name").value == "Alice"
        assert graph.get("age").value == 25
        assert graph.has_key("name")
        assert not graph.has_key("missing")


class TestListValue:
    """Test graph-based list implementation."""

    def test_list_creation(self):
        """Test creating graph lists."""
        elements = [NumberValue(1), NumberValue(2), NumberValue(3)]
        graph_list = ListValue(elements)

        assert len(graph_list) == 3
        assert graph_list[0].value == 1
        assert graph_list[2].value == 3
        assert graph_list.get_type() == "list"

    def test_list_append(self):
        """Test appending to graph lists."""
        graph_list = ListValue([NumberValue(1), NumberValue(2)])
        graph_list.append(NumberValue(3))

        assert len(graph_list) == 3
        assert graph_list[2].value == 3

    def test_list_indexing(self):
        """Test indexing operations."""
        graph_list = ListValue([NumberValue(10), NumberValue(20), NumberValue(30)])

        # Get by index
        assert graph_list[1].value == 20

        # Set by index
        graph_list[1] = NumberValue(25)
        assert graph_list[1].value == 25

    def test_list_slice(self):
        """Test list slicing."""
        graph_list = ListValue([NumberValue(i) for i in range(5)])
        sliced = graph_list.slice(1, 4)

        assert len(sliced) == 3
        assert sliced[0].value == 1
        assert sliced[2].value == 3

    def test_list_graph_features(self):
        """Test graph-specific features of lists."""
        graph_list = ListValue([NumberValue(i) for i in range(4)])

        # Add custom edges
        graph_list.add_edge(0, 3, "related")
        connected = graph_list.get_connected_to(0, "related")
        assert 3 in connected

    def test_list_to_graph_patterns(self):
        """Test converting lists to different graph patterns."""
        graph_list = ListValue([NumberValue(i) for i in range(4)])

        # Star pattern
        star_graph = graph_list.to_graph("star")
        assert len(star_graph.get_connected_to(0, "star")) == 3

    def test_list_backward_compatibility(self):
        """Test that graph lists work like old lists."""
        graph_list = ListValue([NumberValue(1), NumberValue(2)])

        # Test iteration
        values = [item.value for item in graph_list]
        assert values == [1, 2]

        # Test length
        assert len(graph_list) == 2

        # Test contains-like behavior
        assert graph_list.contains(NumberValue(1))
        assert not graph_list.contains(NumberValue(99))


class TestHashValue:
    """Test graph-based hash implementation."""

    def test_hash_creation(self):
        """Test creating graph hashes."""
        pairs = [("name", StringValue("Bob")), ("age", NumberValue(30))]
        graph_hash = HashValue(pairs)

        assert len(graph_hash) == 2
        assert graph_hash["name"].value == "Bob"
        assert graph_hash["age"].value == 30
        assert graph_hash.get_type() == "hash"

    def test_hash_operations(self):
        """Test hash operations."""
        graph_hash = HashValue([])

        # Set values
        graph_hash["key1"] = StringValue("value1")
        graph_hash["key2"] = NumberValue(42)

        assert len(graph_hash) == 2
        assert graph_hash["key1"].value == "value1"
        assert "key1" in graph_hash

    def test_hash_keys_values_items(self):
        """Test keys, values, and items methods."""
        pairs = [("a", NumberValue(1)), ("b", NumberValue(2))]
        graph_hash = HashValue(pairs)

        keys = graph_hash.keys()
        values = graph_hash.values()
        items = graph_hash.items()

        assert set(keys) == {"a", "b"}
        assert len(values) == 2
        assert len(items) == 2

    def test_hash_graph_features(self):
        """Test graph-specific features of hashes."""
        pairs = [("user1", StringValue("Alice")), ("user2", StringValue("Bob"))]
        graph_hash = HashValue(pairs)

        # Add edge between values
        graph_hash.add_value_edge("user1", "user2", "friend")
        connected = graph_hash.get_connected_keys("user1", "friend")
        assert "user2" in connected

    def test_hash_merge(self):
        """Test hash merging."""
        hash1 = HashValue([("a", NumberValue(1)), ("b", NumberValue(2))])
        hash2 = HashValue([("b", NumberValue(3)), ("c", NumberValue(4))])

        merged = hash1.merge(hash2)
        assert len(merged) == 3
        assert merged["b"].value == 3  # Later value wins
        assert merged["c"].value == 4

    def test_hash_backward_compatibility(self):
        """Test that graph hashes work like old hashes."""
        graph_hash = HashValue([("key", StringValue("value"))])

        # Test contains
        assert "key" in graph_hash
        assert "missing" not in graph_hash

        # Test get/set
        graph_hash["new_key"] = NumberValue(123)
        assert graph_hash.get("new_key").value == 123


class TestGraphMigration:
    """Test migration from old values to graph values."""

    def test_migration_functions(self):
        """Test that migration functions work."""
        # Import here to test the actual migration
        from glang.execution.graph_migration import (
            validate_graph_migration, enable_graph_values
        )

        # Enable graph values
        enable_graph_values()

        # Run validation
        assert validate_graph_migration() == True

    def test_python_to_glang_conversion(self):
        """Test converting Python values to graph values."""
        from glang.execution.values import python_to_glang_value

        # Test list conversion
        python_list = [1, 2, 3]
        glang_list = python_to_glang_value(python_list)

        # Should be a graph list when graph values are enabled
        assert hasattr(glang_list, 'graph')  # Graph-based implementation
        assert len(glang_list) == 3
        assert glang_list[0].value == 1

        # Test dict conversion
        python_dict = {"name": "Charlie", "age": 35}
        glang_hash = python_to_glang_value(python_dict)

        # Should be a graph hash when graph values are enabled
        assert hasattr(glang_hash, 'graph')  # Graph-based implementation
        assert len(glang_hash) == 2
        assert glang_hash["name"].value == "Charlie"


class TestGraphCompatibility:
    """Test that graph values maintain full compatibility."""

    def test_type_constraints(self):
        """Test that type constraints work with graph values."""
        # Constrained list
        constrained_list = ListValue([], constraint="num")
        constrained_list.append(NumberValue(42))

        with pytest.raises(Exception):  # Should fail constraint
            constrained_list.append(StringValue("not a number"))

    def test_graph_display(self):
        """Test that graph values display correctly."""
        graph_list = ListValue([NumberValue(1), NumberValue(2)])
        display = graph_list.to_display_string()
        assert "1" in display and "2" in display

        graph_hash = HashValue([("key", StringValue("value"))])
        display = graph_hash.to_display_string()
        assert "key" in display and "value" in display

    def test_equality_comparison(self):
        """Test that graph values compare correctly."""
        list1 = ListValue([NumberValue(1), NumberValue(2)])
        list2 = ListValue([NumberValue(1), NumberValue(2)])
        list3 = ListValue([NumberValue(1), NumberValue(3)])

        assert list1 == list2
        assert list1 != list3

        # Test comparison with Python lists
        assert list1 == [NumberValue(1), NumberValue(2)]
        assert list1 != [NumberValue(1), NumberValue(3)]